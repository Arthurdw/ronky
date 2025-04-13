pub(crate) mod properties;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{Attribute, Meta, meta::ParseNestedMeta, spanned::Spanned};

fn concat_list(items: &[impl ToString]) -> String {
    items
        .iter()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn parse_arri_attrs<'a, P>(
    attrs: &'a [Attribute],
    available_args: &[impl ToString],
    mut parser: P,
) -> Result<(), TokenStream>
where
    P: FnMut(String, ParseNestedMeta<'_>) -> syn::Result<()>,
{
    let attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("arri"))
        .collect();

    for attr in attrs.iter() {
        // We will only accept meta lists
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.tokens.is_empty() {
                let available_args = concat_list(available_args);
                return Err(quote_spanned!(
                    meta_list.span() => compile_error!(concat!(
                        "No arguments were provided for this. Available argument(s) are: ",
                        #available_args
                )))
                .into());
            }

            if let Err(err) = meta_list.parse_nested_meta(|meta| {
                for arg in available_args {
                    if meta.path.is_ident(&arg.to_string()) {
                        return parser(arg.to_string(), meta);
                    }
                }

                Err(meta.error(format!(
                    "Unrecognized argument, available argument(s) are: {}",
                    concat_list(available_args)
                )))
            }) {
                return Err(err.into_compile_error().into());
            }
        } else {
            return Err(quote_spanned!(attr.span() => compile_error!("The only supported attribute format for arri is with a list of arguments. Expected usage: `#[arri(...)]`")).into());
        }
    }

    Ok(())
}
