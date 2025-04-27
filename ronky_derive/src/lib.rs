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

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract the type parameters from generics
    let type_params = generics
        .type_params()
        .map(|param| param.ident.clone())
        .collect::<Vec<_>>();

    // Only generate the specialized get_type_name implementation if there are generic arguments
    let get_type_name_impl = if !type_params.is_empty() {
        quote! {
            fn get_type_name() -> String {
                format!(
                    "::ronky::--virtual--::external::{}",
                    vec![stringify!(#struct_name).to_string(), #(#type_params::get_type_name()),*].join("")
                )
            }
        }
    } else {
        // For non-generic types, use the default implementation
        quote! {}
    };

    quote! {
        impl #impl_generics ronky::Exportable for #struct_name #ty_generics #where_clause {
            fn export_internal() -> impl ronky::Serializable {
                #export
            }
            #get_type_name_impl
        }
    }
    .into()
}
