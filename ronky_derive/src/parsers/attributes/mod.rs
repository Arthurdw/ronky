pub(crate) mod enum_variants;
pub(crate) mod fields;
pub(crate) mod properties;
pub(crate) mod typeschema;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{
    Attribute, Meta, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

/// Parses attributes with the `#[arri(...)]` format and extracts their arguments.
///
/// # Type Parameters
///
/// * `T` - A type that implements the `Parse` trait, representing the expected structure of the arguments.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to parse.
///
/// # Returns
///
/// A `Result` containing an optional parsed value of type `T` or a `TokenStream` error.
fn parse_arri_attrs<T: Parse>(attrs: &[Attribute]) -> Result<Option<T>, TokenStream> {
    let attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("arri"))
        .collect();

    // TODO: we are never iterating, actually go over all
    for attr in attrs.iter() {
        // We will only accept meta lists
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.tokens.is_empty() {
                return Err(quote_spanned!(meta_list.span() => compile_error!("No arguments were provided for this.")).into());
            }

            return match meta_list.parse_args_with(T::parse) {
                Ok(res) => Ok(Some(res)),
                Err(err) => Err(err.into_compile_error().into()),
            };
        } else {
            return Err(quote_spanned!(attr.span() => compile_error!("The only supported attribute format for arri is with a list of arguments. Expected usage: `#[arri(...)]`")).into());
        }
    }

    Ok(None)
}

/// Advances the parse stream to the next token, ensuring proper syntax.
///
/// # Arguments
///
/// * `input` - The input parse stream.
///
/// # Returns
///
/// A `syn::Result` indicating success or failure.
///
/// # Errors
///
/// Returns an error if the input does not contain a valid delimiter or is not empty.
pub(crate) fn goto_next(input: ParseStream) -> syn::Result<()> {
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    } else if !input.is_empty() {
        return Err(input.error("Expected ',' or end of input"));
    }

    Ok(())
}
