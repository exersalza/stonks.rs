#[macro_export]
macro_rules! pub_fields {
    {
        $(#[derive($($macros:tt)*)])*
        struct $name:ident {
            $($field:ident: $t:ty,)*
        }
    } => {
        $(#[derive($($macros)*)])*
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}
