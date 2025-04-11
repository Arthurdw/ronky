use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Exported)]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    quote! {
        fn exported() -> bool {
            true
        }
    }
    .into()
}
