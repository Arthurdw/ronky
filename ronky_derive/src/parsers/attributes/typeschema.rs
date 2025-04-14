use super::goto_next;
use proc_macro::TokenStream;
use syn::{
    Attribute, Ident, LitBool, Token,
    parse::{Parse, ParseStream},
};

use super::parse_arri_attrs;

#[derive(Debug, Default)]
pub(crate) struct TypeSchemaArguments {
    pub(crate) is_nullable: bool,
}

impl Parse for TypeSchemaArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = TypeSchemaArguments::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            match key_str.as_str() {
                "nullable" => {
                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                        let value: LitBool = input.parse()?;
                        args.is_nullable = value.value();
                    } else {
                        args.is_nullable = true;
                    }
                }
                _ => Err(input.error(format!("Unknown property: {}", key_str)))?,
            }

            goto_next(input)?;
        }

        Ok(args)
    }
}

pub(crate) fn extract(attrs: &[Attribute]) -> Result<Option<TypeSchemaArguments>, TokenStream> {
    parse_arri_attrs(attrs)
}
