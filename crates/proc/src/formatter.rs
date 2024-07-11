use proc_macro2::TokenStream;
use syn::Field;

pub trait FieldFormatter {
    fn format_field(&self, field: &Field) -> TokenStream;
}
