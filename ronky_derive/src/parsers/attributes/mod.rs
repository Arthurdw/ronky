pub(crate) mod properties;
pub(crate) mod typeschema;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{
    Attribute, Meta, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

fn parse_arri_attrs<'a, T: Parse>(attrs: &'a [Attribute]) -> Result<Option<T>, TokenStream> {
    let attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("arri"))
        .collect();

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

pub(crate) fn goto_next(input: ParseStream) -> syn::Result<()> {
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    } else if !input.is_empty() {
        return Err(input.error("Expected ',' or end of input"));
    }

    Ok(())
}
