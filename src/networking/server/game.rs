use crate::{
    networking::proto::game::{
        PlayerEvent, PlayerEventFromServer, PlayerState, PlayerStateFromServer,
    },
    prelude::*,
};

use super::{MessageTarget, NetServer};

pub struct ServerGamePlugin;

impl Plugin for ServerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            handle_client_messages
                .run_if_resource_exists::<NetServer>()
                .run_in_state(GameState::ServerInGame),
        );
    }
}

fn handle_client_messages(mut server: ResMut<NetServer>) {
    while let Some(incomming) = server.recv_reliable::<PlayerEvent>() {
        server.send_reliable_to(
            &PlayerEventFromServer {
                player_idx: incomming.client_idx.try_into().unwrap(),
                kind: incomming.message,
            },
            MessageTarget::AllExcept(incomming.client_idx),
        )
    }
    while let Some(incomming) = server.recv_unreliable::<PlayerState>() {
        server.send_unreliable_to(
            &PlayerStateFromServer {
                player_idx: incomming.client_idx.try_into().unwrap(),
                state: incomming.message,
            },
            MessageTarget::AllExcept(incomming.client_idx),
        )
    }
}