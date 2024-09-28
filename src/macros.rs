#[macro_export]
macro_rules! char_type {
    ($t:ty) => {
        std::any::TypeId::of::<$t>()
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
