window.SIDEBAR_ITEMS = {"attr":[["reflect_trait","A macro that automatically generates type data for traits, which their implementors can then register."]],"derive":[["FromReflect","Derives the `FromReflect` trait."],["Reflect","The main derive macro used by `bevy_reflect` for deriving its `Reflect` trait."],["TypeUuid",""]],"enum":[["DynamicVariant","A dynamic representation of an enum variant."],["ReflectMut","A mutable enumeration of “kinds” of reflected type."],["ReflectOwned","An owned enumeration of “kinds” of reflected type."],["ReflectPathError","An error returned from a failed path string query."],["ReflectRef","An immutable enumeration of “kinds” of reflected type."],["TypeInfo","Compile-time type information for various reflected types."],["VariantField",""],["VariantInfo","A container for compile-time enum variant info."],["VariantType","Describes the form of an enum variant."]],"fn":[["array_apply","Applies the reflected array data to the given array."],["array_debug","The default debug formatter for [`Array`] types."],["array_hash","Returns the `u64` hash of the given array."],["array_partial_eq","Compares two arrays (one concrete and one reflected) to see if they are equal."],["enum_debug","The default debug formatter for [`Enum`] types."],["enum_hash","Returns the `u64` hash of the given enum."],["enum_partial_eq","Compares an [`Enum`] with a [`Reflect`] value."],["list_apply","Applies the elements of `b` to the corresponding elements of `a`."],["list_debug","The default debug formatter for [`List`] types."],["list_hash","Returns the `u64` hash of the given list."],["list_partial_eq","Compares a [`List`] with a [`Reflect`] value."],["map_apply","Applies the elements of reflected map `b` to the corresponding elements of map `a`."],["map_debug","The default debug formatter for [`Map`] types."],["map_partial_eq","Compares a [`Map`] with a [`Reflect`] value."],["struct_debug","The default debug formatter for [`Struct`] types."],["struct_partial_eq","Compares a [`Struct`] with a [`Reflect`] value."],["tuple_apply","Applies the elements of `b` to the corresponding elements of `a`."],["tuple_debug","The default debug formatter for [`Tuple`] types."],["tuple_partial_eq","Compares a [`Tuple`] with a [`Reflect`] value."],["tuple_struct_debug","The default debug formatter for [`TupleStruct`] types."],["tuple_struct_partial_eq","Compares a [`TupleStruct`] with a [`Reflect`] value."]],"macro":[["impl_from_reflect_value","A macro used to generate a `FromReflect` trait implementation for the given type."],["impl_reflect_struct","A replacement for `#[derive(Reflect)]` to be used with foreign types which the definitions of cannot be altered."],["impl_reflect_value","A macro used to generate reflection trait implementations for the given type."],["impl_type_uuid","Derives `TypeUuid` for the given type. This is used internally to implement `TypeUuid` on foreign types, such as those in the std. This macro should be used in the format of `<[Generic Params]> [Type (Path)], [Uuid (String Literal)]`."]],"mod":[["erased_serde","github crates-io docs-rs"],["prelude",""],["serde",""],["std_traits",""],["utility","Helpers for working with Bevy reflection."]],"struct":[["ArrayInfo","A container for compile-time array info."],["ArrayIter","An iterator over an [`Array`]."],["DynamicArray","A fixed-size list of reflected values."],["DynamicEnum","A dynamic representation of an enum."],["DynamicInfo","A container for compile-time info related to Bevy’s dynamic types, including primitives."],["DynamicList","A list of reflected values."],["DynamicMap","An ordered mapping between reflected values."],["DynamicStruct","A struct type which allows fields to be added at runtime."],["DynamicTuple","A tuple which allows fields to be added at runtime."],["DynamicTupleStruct","A tuple struct which allows fields to be added at runtime."],["EnumInfo","A container for compile-time enum info, used by `TypeInfo`."],["FieldIter","An iterator over the field values of a struct."],["ListInfo","A container for compile-time list info."],["ListIter","An iterator over an [`List`]."],["MapInfo","A container for compile-time map info."],["MapIter","An iterator over the key-value pairs of a [`Map`]."],["NamedField","The named field of a reflected struct."],["ParsedPath","A pre-parsed path to an element within a type."],["ReflectDeserialize","A struct used to deserialize reflected instances of a type."],["ReflectFromPtr","[`Reflect`] values are commonly used in situations where the actual types of values are not known at runtime. In such situations you might have access to a `*const ()` pointer that you know implements [`Reflect`], but have no way of turning it into a `&dyn Reflect`."],["ReflectFromReflect","Type data that represents the [`FromReflect`] trait and allows it to be used dynamically."],["ReflectSerialize","A struct used to serialize reflected instances of a type."],["StructInfo","A container for compile-time named struct info."],["StructVariantInfo","Type info for struct variants."],["TupleFieldIter","An iterator over the field values of a tuple."],["TupleInfo","A container for compile-time tuple info."],["TupleStructFieldIter","An iterator over the field values of a tuple struct."],["TupleStructInfo","A container for compile-time tuple struct info."],["TupleVariantInfo","Type info for tuple variants."],["TypeRegistration","Runtime storage for type metadata, registered into the [`TypeRegistry`]."],["TypeRegistry","A synchronized wrapper around a [`TypeRegistry`]."],["TypeRegistryArc","A synchronized wrapper around a [`TypeRegistry`]."],["TypeRegistryInternal","A registry of reflected types."],["UnitVariantInfo","Type info for unit variants."],["UnnamedField","The unnamed field of a reflected tuple or tuple struct."],["Uuid","A Universally Unique Identifier (UUID)."],["ValueInfo","A container for compile-time info related to general value types, including primitives."],["VariantFieldIter","An iterator over the fields in the current enum variant."]],"trait":[["Array","A trait used to power array-like operations via reflection."],["Enum","A trait used to power enum-like operations via reflection."],["FromReflect","A trait that enables types to be dynamically constructed from reflected data."],["FromType","Trait used to generate [`TypeData`] for trait reflection."],["GetField","A convenience trait which combines fetching and downcasting of struct fields."],["GetPath","A trait which allows nested [`Reflect`] values to be retrieved with path strings."],["GetTupleField","A convenience trait which combines fetching and downcasting of tuple fields."],["GetTupleStructField","A convenience trait which combines fetching and downcasting of tuple struct fields."],["GetTypeRegistration","A trait which allows a type to generate its [`TypeRegistration`] for registration into the [`TypeRegistry`]."],["List","A trait used to power list-like operations via reflection."],["Map","A trait used to power map-like operations via reflection."],["Reflect","The core trait of `bevy_reflect`, used for accessing and modifying data dynamically."],["Struct","A trait used to power struct-like operations via reflection."],["Tuple","A trait used to power tuple-like operations via reflection."],["TupleStruct","A trait used to power tuple struct-like operations via reflection."],["TypeData","A trait used to type-erase type metadata."],["TypeUuid","A trait for types with a statically associated UUID."],["TypeUuidDynamic","A trait for types with an associated UUID."],["Typed","A static accessor to compile-time type information."]]};