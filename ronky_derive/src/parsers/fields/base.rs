use proc_macro::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{Field, spanned::Spanned};

use crate::parsers::{
    attributes::typeschema,
    types::{is_option_type, parse_type},
};

use super::{FieldParser, ParsedField};

pub struct BaseParser;

impl FieldParser for BaseParser {
    fn parse<'a>(field: &'a Field) -> Result<ParsedField<'a>, TokenStream> {
        let typeschema: proc_macro2::TokenStream = parse_type(&field.ty).into();
        let is_optional = is_option_type(&field.ty);
        let attrs = match typeschema::extract(&field.attrs) {
            Ok(Some(attrs)) => {
                let is_nullable = attrs.is_nullable;

                if !is_optional && is_nullable {
                    let type_name = field.ty.to_token_stream().to_string();
                    return Err(quote_spanned!(field.ty.span() => compile_error!(concat!("Only an optional type can be nullable. Use Option<", #type_name, "> instead of ", #type_name))).into());
                }

                Some(quote! {
                    ty.set_nullable(#is_nullable);
                })
            }
            Ok(None) => None,
            Err(stream) => return Err(stream.into()),
        };

        let typeschema = quote! {
            {
                let mut ty = #typeschema;
                #attrs
                ty
            }
        }
        .into();

        if is_optional {
            return Ok(ParsedField::Optional(field, typeschema));
        }
        Ok(ParsedField::Required(field, typeschema))
    }
}
