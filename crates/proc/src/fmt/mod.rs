mod base;
mod vec;

use base::BaseFormatter;
use proc_macro2::TokenStream;
use syn::Field;
use vec::VecFormatter;

use crate::formatter::FieldFormatter;

/// Process a field and return a JSON representation of it.
pub fn format_field(field: &Field) -> TokenStream {
    let path = match &field.ty {
        syn::Type::Path(path) => path.path.clone(),
        _ => panic!("Received vector type that is not a path"),
    };

    let formatter: Box<dyn FieldFormatter> = match path.get_ident() {
        Some(ident) => match ident.to_string().to_lowercase().as_ref() {
            // TODO: add support for generics
            "vec" => Box::new(VecFormatter()),
            _ => Box::new(BaseFormatter()),
        },
        None => Box::new(BaseFormatter()),
    };

    formatter.format_field(field)
}
