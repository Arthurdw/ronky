mod arri_types;

use arri_types::ArriTypesParser;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Path, Type};

pub(crate) trait TypeParser {
    fn parse(field: &Path) -> TokenStream;
}

/// Retrieves the `Path` from the given `Field`.
///
/// # Arguments
///
/// * `field` - A reference to a `Field` from which the `Path` is extracted.
///
/// # Returns
///
/// A reference to the `Path` associated with the `Field`.
///
/// # Panics
///
/// This function will panic if the field type is not a `Type::Path`.
fn get_path<'a>(ty: &'a Type) -> &'a Path {
    match &ty {
        Type::Path(type_path) => &type_path.path,
        _ => panic!("Unsupported field type"),
    }
}

pub(crate) fn is_option_type(field: &Type) -> bool {
    if let Type::Path(type_path) = field {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

pub(crate) fn parse_type(ty: &Type) -> TokenStream {
    let path = get_path(ty);
    let arri_type = ArriTypesParser::parse(path);
    let stream: proc_macro2::TokenStream = arri_type.into();

    quote!(ronky::TypeSchema::new(#stream)).into()
}
