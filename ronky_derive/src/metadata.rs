// TODO: Refactor move this (or applicable contents to the parsers(attributes) module)

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Expr, ExprLit, Field, Lit, LitStr, Meta, MetaNameValue};

/// Extracts documentation comments from the given attributes and converts them into a `TokenStream`.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to extract documentation from.
///
/// # Returns
///
/// Returns an `Option<TokenStream>` containing the extracted documentation, or `None` if no documentation is found.
fn extract_docs(attrs: &[Attribute]) -> Option<TokenStream> {
    let docs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(line),
                        ..
                    }),
                ..
            }) => line.value().trim().to_string(),
            _ => unreachable!(),
        })
        .collect::<Vec<String>>()
        .join("\n");

    match docs.is_empty() {
        true => None,
        false => Some(
            quote! {
                metadata.set_description(#docs);
            }
            .into(),
        ),
    }
}

/// Extracts the `deprecated` attribute from the given attributes and converts it into a `TokenStream`.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to extract the `deprecated` attribute from.
///
/// # Returns
///
/// Returns an `Option<TokenStream>` containing the extracted `deprecated` attribute, or `None` if not found.
fn extract_deprecated(attrs: &[Attribute]) -> Option<TokenStream> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("deprecated"))
        .map(|attr| {
            let mut stream: TokenStream2 = quote! {
                metadata.set_deprecated(true);
            };

            let _ = attr.parse_nested_meta(|meta| {
                match meta
                    .path
                    .get_ident()
                    .map(|ident| ident.to_string())
                    .as_deref()
                {
                    Some("since") => {
                        let since: LitStr = meta.value()?.parse()?;
                        stream.extend(quote! {
                            metadata.set_deprecated_since(#since);
                        });
                        Ok(())
                    }
                    Some("note") => {
                        let note: LitStr = meta.value()?.parse()?;
                        stream.extend(quote! {
                            metadata.set_deprecated_message(#note);
                        });
                        Ok(())
                    }
                    _ => unreachable!(),
                }
            });

            stream.into()
        })
}

/// Combines the extracted documentation and `deprecated` attributes into a single `TokenStream`.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to extract attributes from.
///
/// # Returns
///
/// Returns an `Option<TokenStream>` containing the combined attributes, or `None` if no attributes are found.
pub fn extract_attrs(attrs: &[Attribute]) -> Option<TokenStream> {
    let docs: Option<TokenStream2> = extract_docs(attrs).map(Into::into);
    let deprecated: Option<TokenStream2> = extract_deprecated(attrs).map(Into::into);

    if docs.is_none() && deprecated.is_none() {
        return None;
    }

    Some(
        quote! {
            {
                let mut metadata = ronky::MetadataSchema::new();
                #deprecated;
                #docs
                metadata
            }
        }
        .into(),
    )
}

/// Extracts metadata from the given attributes and constructs a `MetadataSchema` object.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to extract metadata from.
///
/// # Returns
///
/// Returns a `TokenStream` representing the constructed `MetadataSchema` object.
pub fn extract(attrs: &[Attribute]) -> TokenStream {
    let base: proc_macro2::TokenStream =
        extract_attrs(attrs).map_or(quote!(ronky::MetadataSchema::new()), Into::into);

    quote! {
        {
            let mut metadata = #base;
            metadata.set_id(ronky::type_utils::get_type_name_from(Self::get_type_name()));
            metadata
        }
    }
    .into()
}

/// Extracts metadata from the attributes of a given field.
///
/// # Arguments
///
/// * `field` - A reference to a `Field` object to extract metadata from.
///
/// # Returns
///
/// Returns an `Option<TokenStream>` containing the extracted metadata, or `None` if no attributes are present.
pub fn extract_from_field(field: &Field) -> Option<TokenStream> {
    if field.attrs.is_empty() {
        return None;
    }

    extract_attrs(&field.attrs)
}
