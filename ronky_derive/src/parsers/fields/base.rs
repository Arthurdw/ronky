use quote::quote;
use syn::Field;

use crate::parsers::{
    attributes::typeschema,
    types::{is_option_type, parse_type},
};

use super::{FieldParser, ParsedField};

pub struct BaseParser;

impl FieldParser for BaseParser {
    fn parse<'a>(field: &'a Field) -> ParsedField<'a> {
        let typeschema: proc_macro2::TokenStream = parse_type(&field.ty).into();
        let attrs = match typeschema::extract(&field.attrs) {
            Ok(Some(attrs)) => {
                let nullable = attrs.nullable;

                Some(quote! {
                    ty.set_nullable(#nullable);
                })
            }
            Ok(None) => None,
            Err(stream) => Some(stream.into()),
        };

        let typeschema = quote! {
            {
                let mut ty = #typeschema;
                #attrs
                ty
            }
        }
        .into();

        if is_option_type(&field.ty) {
            return ParsedField::Optional(field, typeschema);
        }
        ParsedField::Required(field, typeschema)
    }
}
