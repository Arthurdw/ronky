use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr};

pub fn extract(input: &DeriveInput) -> TokenStream {
    let id = input.ident.to_string();

    let deprecated = input
        .attrs
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
        });

    quote! {
        {
            let mut metadata = ronky::MetadataSchema::new();
            metadata.set_id(#id);
            #deprecated;
            metadata
        }
    }
    .into()
}
