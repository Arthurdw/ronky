mod base;
mod vec;

use base::{get_path, BaseFormatter};
use proc_macro2::TokenStream;
use syn::Field;
use vec::VecFormatter;

use crate::formatter::FieldFormatter;

/// Process a field and return a JSON representation of it.
pub fn format_field(field: &Field) -> TokenStream {
    let path = get_path(field);

    let formatter: Box<dyn FieldFormatter> = 'formatter: {
        if path.get_ident().is_some() || path.segments.is_empty() {
            break 'formatter Box::new(BaseFormatter());
        }

        let segment = path.segments.first().unwrap();
        let ident = segment.ident.to_string();

        match ident.as_str() {
            "Vec" => break 'formatter Box::new(VecFormatter()),
            _ => panic!("Unknown type: {}", ident),
        };
    };

    formatter.format_field(field)
}
