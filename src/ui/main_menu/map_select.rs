use bevy_ggrs::RollbackIdProvider;
use jumpy_matchmaker_proto::TargetClient;

use crate::{
    metadata::MapMeta,
    networking::{
        client::NetClient,
        proto::{match_setup::MatchSetupMessage, ReliableGameMessageKind},
    },
    player::input::WantsGamePause,
    ui::pause_menu::PauseMenuPage,
};

use super::*;

#[derive(SystemParam)]
pub struct MapSelectMenu<'w, 's> {
    client: Option<Res<'w, NetClient>>,
    menu_input: Query<'w, 's, &'static mut ActionState<MenuAction>>,
    menu_page: ResMut<'w, MenuPage>,
    game: Res<'w, GameMeta>,
    game_state: Res<'w, CurrentState<GameState>>,
    pause_page: ResMut<'w, PauseMenuPage>,
    commands: Commands<'w, 's>,
    localization: Res<'w, Localization>,
    map_assets: Res<'w, Assets<MapMeta>>,
    rids: ResMut<'w, RollbackIdProvider>,
    reset_manager: ResetManager<'w, 's>,
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}

impl<'w, 's> WidgetSystem for MapSelectMenu<'w, 's> {
    type Args = bool;

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut egui::Ui,
        _id: WidgetId,
        is_waiting: Self::Args,
    ) {
        let mut params: MapSelectMenu = state.get_mut(world);

        handle_match_setup_messages(&mut params);

        let in_game = params.game_state.0 == GameState::InGame;

        if params.menu_input.single().just_pressed(MenuAction::Back) {
            // If we are on the main menu
            if params.game_state.0 == GameState::MainMenu {
                *params.menu_page = MenuPage::PlayerSelect;

            // If we're on a map selection in game, we must be in the pause menu
            } else if in_game {
                *params.pause_page = PauseMenuPage::Default;
            }
        }

        ui.vertical_centered_justified(|ui| {
            let bigger_text_style = &params.game.ui_theme.font_styles.bigger;
            let heading_text_style = &params.game.ui_theme.font_styles.heading;
            let small_button_style = &params.game.ui_theme.button_styles.small;

            if !in_game {
                ui.add_space(heading_text_style.size / 4.0);
                ui.themed_label(heading_text_style, &params.localization.get("local-game"));
                ui.themed_label(
                    bigger_text_style,
                    &params.localization.get("map-select-title"),
                );
                ui.add_space(small_button_style.font.size);
            }

            let available_size = ui.available_size();
            let menu_width = params.game.main_menu.menu_width;
            let x_margin = (available_size.x - menu_width) / 2.0;
            let outer_margin = egui::style::Margin::symmetric(x_margin, heading_text_style.size);

            if is_waiting {
                ui.themed_label(
                    bigger_text_style,
                    &params.localization.get("waiting-for-map"),
                );
            } else {
                BorderedFrame::new(&params.game.ui_theme.panel.border)
                    .margin(outer_margin)
                    .padding(params.game.ui_theme.panel.padding.into())
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        let mut first_button = true;

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (section_title, map_handles) in [
                                (
                                    params.localization.get("default-maps"),
                                    &params.game.stable_map_handles,
                                ),
                                (
                                    params.localization.get("experimental-maps"),
                                    &params.game.experimental_map_handles,
                                ),
                            ] {
                                ui.add_space(bigger_text_style.size / 2.0);
                                let label = ui.themed_label(bigger_text_style, &section_title);

                                // Clippy lint is a false alarm, necessary to avoid borrowing params
                                #[allow(clippy::unnecessary_to_owned)]
                                for map_handle in map_handles
                                    .iter()
                                    .map(|x| x.clone_weak())
                                    .collect::<Vec<_>>()
                                    .into_iter()
                                {
                                    let map_meta = params.map_assets.get(&map_handle).unwrap();
                                    ui.add_space(ui.spacing().item_spacing.y);

                                    let button =
                                        BorderedButton::themed(small_button_style, &map_meta.name)
                                            .show(ui);

                                    if first_button {
                                        first_button = false;
                                        // There's something weird where egui focuses on the first
                                        // thing in the scroll area, so we have to re-focus on the
                                        // button, instead of the label.
                                        if label.has_focus() {
                                            ui.ctx().memory().request_focus(button.id);
                                        }
                                    }

                                    if button.clicked() {
                                        info!("Selected map, starting game");
                                        *params.pause_page = PauseMenuPage::Default;
                                        *params.menu_page = MenuPage::Home;
                                        params.reset_manager.reset_world();
                                        params.commands.spawn().insert(map_handle.clone_weak());
                                        params.commands.insert_resource(WantsGamePause(false));
                                        params
                                            .commands
                                            .insert_resource(NextState(GameState::InGame));

                                        if let Some(client) = &mut params.client {
                                            client.send_reliable(
                                                MatchSetupMessage::SelectMap(map_handle),
                                                TargetClient::All,
                                            );
                                        }
                                    }
                                }
                            }
                        });
                    });
            }
        });
    }
}

fn handle_match_setup_messages(params: &mut MapSelectMenu) {
    if let Some(client) = &mut params.client {
        while let Some(message) = client.recv_reliable() {
            match message.kind {
                ReliableGameMessageKind::MatchSetup(setup) => match setup {
                    MatchSetupMessage::SelectMap(map_handle) => {
                        info!("Other player selected map, starting game");
                        *params.menu_page = MenuPage::Home;
                        params.reset_manager.reset_world();
                        params
                            .commands
                            .spawn()
                            .insert(map_handle)
                            .insert(Rollback::new(params.rids.next_id()));
                        params
                            .commands
                            .insert_resource(NextState(GameState::InGame));
                    }
                    other => warn!("Unexpected message: {other:?}"),
                },
            }
        }
    }
}