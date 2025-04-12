use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, ExprLit, Lit, LitStr, Meta, MetaNameValue};

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

fn extract_deprecated(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("deprecated"))
        .map(|attr| {
            let mut stream: proc_macro2::TokenStream = quote! {
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
                    _ => Err(meta.error("Unsupported deprecated attribute")),
                }
            });

            stream
        })
}

pub fn extract(input: &DeriveInput) -> TokenStream {
    let id = input.ident.to_string();
    // TODO: There aint no way we can't just into this?
    let docs: Option<proc_macro2::TokenStream> = match extract_docs(&input.attrs) {
        Some(stream) => Some(stream.into()),
        None => None,
    };
    let deprecated = extract_deprecated(&input.attrs);

    quote! {
        {
            let mut metadata = ronky::MetadataSchema::new();
            metadata.set_id(#id);
            #deprecated;
            #docs
            metadata
        }
    }
    .into()
}
