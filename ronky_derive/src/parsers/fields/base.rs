use proc_macro::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{Field, spanned::Spanned};

use crate::parsers::{
    attributes::{
        fields,
        typeschema::{self, TypeSchemaArguments},
    },
    types::is_option_type,
};

use super::{FieldParser, ParsedField};

/// A parser for processing fields in a struct or enum.
///
/// The `BaseParser` is responsible for extracting metadata from fields,
/// such as type information, attributes, and optionality, and converting
/// them into a `ParsedField` representation.
pub struct BaseParser;

impl FieldParser for BaseParser {
    /// Parses a single field and returns a `ParsedField` representation.
    ///
    /// # Arguments
    ///
    /// * `field` - A reference to the `Field` to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(ParsedField)` - If the field is successfully parsed.
    /// * `Err(TokenStream)` - If there is an error during parsing, such as
    ///   invalid attributes or type mismatches.
    fn parse(field: &Field) -> Result<ParsedField<'_>, TokenStream> {
        let ty = &field.ty;

        // Generate the exportable type representation.
        let export = quote!(<#ty as ronky::Exportable>::export());

        // Check if the field type is an `Option`.
        let is_optional = is_option_type(&field.ty);

        // Extract and process the `typeschema` attributes.
        let attrs = match typeschema::extract(&field.attrs) {
            Ok(attrs) => {
                let mut actual = TypeSchemaArguments::default();

                for attr in attrs {
                    if let Some(is_nullable) = attr.is_nullable {
                        // Ensure only optional types can be nullable.
                        if !is_optional && is_nullable {
                            let type_name = field.ty.to_token_stream().to_string();
                            return Err(quote_spanned!(field.ty.span() =>
                                compile_error!(concat!(
                                    "Only an optional type can be nullable. Use Option<",
                                    #type_name,
                                    "> instead of ",
                                    #type_name
                                ))
                            )
                            .into());
                        }

                        actual.is_nullable = Some(is_nullable);
                    }
                }

                // Generate nullable attribute code.
                actual.is_nullable.map(|is_nullable| {
                    quote! {
                        use ronky::Serializable;
                        ty.set_nullable(#is_nullable);
                    }
                })
            }
            Err(stream) => return Err(stream),
        };

        // Generate the type schema representation.
        let typeschema = quote! {
            {
                let mut ty = #export;
                #attrs
                ty
            }
        }
        .into();

        // Extract field-specific attributes.
        let field_attrs = fields::extract(&field.attrs)?;

        // Return the parsed field based on its optionality.
        if is_optional {
            return Ok(ParsedField::Optional(field, typeschema, field_attrs));
        }
        Ok(ParsedField::Required(field, typeschema, field_attrs))
    }
}
