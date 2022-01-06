macro_rules! make_bitfield_serde {(
        $(#[$struct_meta:meta])*
        $struct_vis:vis
        struct $StructName:ident {
                $(
                        $(#[$field_meta:meta])*
                        $field_vis:vis
                        $field_name:ident : $field_ty:ty $(: $getter_vis:vis get $field_user_ty:ty $(: $setter_vis:vis set $field_setter_user_ty:ty)?)?
                ),* $(,)?
        }
) => {
        $(#[$struct_meta])*
        $struct_vis
        struct $StructName {
                $(
                        $(#[$field_meta])*
                        $field_vis
                        $field_name : $field_ty,
                )*
        }

        paste::paste! {
                #[derive(serde::Deserialize, serde::Serialize)]
                //#[serde(remote = "" $StructName)]
                pub(crate) struct [<Serde $StructName>] {
                        $(
                                $(
                                        pub(crate) $field_name : $field_user_ty,
                                )?
                        )*
                }
        }
}}
pub(crate) use make_bitfield_serde;
