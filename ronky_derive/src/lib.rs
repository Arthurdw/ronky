pub(crate) mod parsers;

use parsers::parse_field;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse};

#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(input).unwrap();

    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match &data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => panic!("Only named fields are supported for now"),
    };

    let id = input.ident.to_string();
    // let parsed_fields = fields.iter().map(parse_field);

    quote! {
        use ronky::{MetadataSchema, PropertiesSchema};

        let mut schema = PropertiesSchema::new();
        let mut metadata = MetadataSchema::new();
        metadata.set_id(#id);

        schema.set_metadata(Box::new(metadata));
        schema
    }
    .into()
}

#[proc_macro_derive(Exported)]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(input).unwrap();
    let struct_name = input.ident.clone();
    let result = export_stream(input.into_token_stream().into());
    let quotable_result: proc_macro2::TokenStream = result.into();

    quote! {
        use ronky::{Exportable, Serializable};

        impl Exportable for #struct_name {
            fn export() -> impl Serializable {
                #quotable_result
            }
        }
    }
    .into()
}
