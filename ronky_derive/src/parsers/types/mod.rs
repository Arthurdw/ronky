mod arri_types;

use arri_types::ArriTypesParser;
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{Path, Type, spanned::Spanned};

pub(crate) trait TypeParser {
    fn parse(ty: &Type, field: &Path) -> TokenStream;
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
/// # Compilation Errors
///
/// This function will not compile if the field type is not a `Type::Path`.
fn get_path<'a>(ty: &'a Type) -> Result<&'a Path, TokenStream> {
    match &ty {
        Type::Path(type_path) => Ok(&type_path.path),
        _ => Err(quote_spanned!(ty.span() => compile_error!("Unsupported field type!\nIf you believe this should be supported please create a issue @ <https://github.com/Arthurdw/ronky/issues>")).into()),
    }
}

pub(crate) fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

pub(crate) fn parse_type(ty: &Type) -> TokenStream {
    let path = match get_path(ty) {
        Ok(path) => path,
        Err(stream) => return stream,
    };
    ArriTypesParser::parse(ty, path)
}
