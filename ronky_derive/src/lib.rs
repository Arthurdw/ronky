mod r#enum;
mod metadata;
mod named_struct;
mod parsers;

use r#enum::export_enum;
use named_struct::export_named_struct;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, parse_macro_input, spanned::Spanned};

#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(ref fields), .. }) => export_named_struct(&input, &fields.named),
        Data::Enum(DataEnum { ref variants, .. }) => export_enum(&input, variants),
        _ => quote_spanned!(input.span() => compile_error!("Only named structs or enums are exportable for now")).into()
    }
}

#[proc_macro_derive(Exported, attributes(arri))]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let export: proc_macro2::TokenStream = export_stream(input.clone()).into();

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    quote! {
        impl ronky::Exportable for #struct_name {
            fn export_internal() -> impl ronky::Serializable {
                #export
            }
        }
    }
    .into()
}
