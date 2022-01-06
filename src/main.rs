mod struct_accessors;

use modular_bitfield::prelude::*;
use struct_accessors::make_bitfield_serde;

macro_rules! make_bitfield_serdex {(
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

make_bitfield_serde! {
        #[bitfield(bits = 16)]
        #[repr(u16)]
        #[derive(Copy, Clone, Debug)]
        pub struct DirectoryAdditionalInfo {
                pub max_size: B16 : pub get u16 : pub set u16,
        }
}


fn main() {
    println!("Hello, world!");
}
