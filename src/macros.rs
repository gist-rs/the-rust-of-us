use std::any::TypeId;
use std::fmt;

#[derive(Debug)]
pub struct TypeIdWrapper(pub TypeId);

impl fmt::Display for TypeIdWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl PartialEq for TypeIdWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[macro_export]
macro_rules! get_type_id {
    ($t:ty) => {
        $crate::macros::TypeIdWrapper(std::any::TypeId::of::<$t>())
    };
}

#[macro_export]
macro_rules! impl_character_info {
    ($struct_name:ident {
        $($field:ident: $field_type:ty,)*
    }) => {
        impl CharacterInfo for $struct_name {
            $(
                fn $field(&self) -> &$field_type {
                    &self.$field
                }
            )*

            fn get_clone(&self) -> Self {
                self.clone()
            }
        }
    };
}
