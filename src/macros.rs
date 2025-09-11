#[macro_export]
macro_rules! pub_fields {
    {
        $(#[$struct_attr:meta])*
        struct $name:ident {
            $( $(#[$field_attr:meta])* $field:ident: $t:ty),* $(,)?
        }
    } => {
        $(#[$struct_attr])*
        pub struct $name {
            $( $(#[$field_attr])* pub $field: $t),*
        }
    };
}

