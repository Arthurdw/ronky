use syn::Type;

/// Checks if the given type is an `Option`.
///
/// # Arguments
///
/// * `ty` - A reference to a `Type` object to check.
///
/// # Returns
///
/// Returns `true` if the type is an `Option`, otherwise `false`.
pub(crate) fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Option"),
        _ => false,
    }
}
