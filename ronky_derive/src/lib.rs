pub(crate) mod parsers;
mod metadata;

use parsers::parse_field;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};


#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match &data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => panic!("Only named fields are supported for now"),
    };
    let metadata: proc_macro2::TokenStream = metadata::extract(&input).into();

    // TODO:: parsed fields
    let parsed_fields = fields.iter().map(parse_field);
    // TODO: find optional fields

    quote! {
        let mut schema = ronky::PropertiesSchema::new();
        schema.set_metadata(Box::new(#metadata));
        schema
    }
    .into()
}

#[proc_macro_derive(Exported)]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let result = export_stream(input.into_token_stream().into());
    let quotable_result: proc_macro2::TokenStream = result.into();

    quote! {
        impl ronky::Exportable for #struct_name {
            fn export() -> impl ronky::Serializable {
                #quotable_result
            }
        }
    }
    .into()
}
