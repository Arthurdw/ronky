use super::goto_next;
use proc_macro::TokenStream;
use syn::{
    Attribute, Ident, LitBool, Token,
    parse::{Parse, ParseStream},
};

use super::parse_arri_attrs;

// TODO: provide method which outputs all available properties
#[derive(Debug, Default)]
pub(crate) struct PropertiesArguments {
    pub(crate) strict: bool,
}

impl Parse for PropertiesArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = PropertiesArguments::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            match key_str.as_str() {
                "strict" => {
                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                        let value: LitBool = input.parse()?;
                        args.strict = value.value();
                    } else {
                        args.strict = true;
                    }
                }
                _ => Err(input.error(format!("Unknown property: {}", key_str)))?,
            }

            goto_next(input)?;
        }

        Ok(args)
    }
}

pub(crate) fn extract(attrs: &[Attribute]) -> Result<Option<PropertiesArguments>, TokenStream> {
    parse_arri_attrs(attrs)
}
