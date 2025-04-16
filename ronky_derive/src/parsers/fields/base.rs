use proc_macro::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{Field, Type, spanned::Spanned};

use crate::parsers::{attributes::typeschema, types::is_option_type};

use super::{FieldParser, ParsedField};

pub struct BaseParser;

impl BaseParser {
    fn contains_recursive_type(parent: &str, ty: &Type) -> bool {
        match ty {
            Type::Path(type_path) if !type_path.path.segments.is_empty() => {
                // Check if the type path's first segment matches the parent name exactly
                let segment = &type_path.path.segments[0];
                if segment.ident == parent {
                    return true;
                }

                // Check arguments of generic types like Option<Parent>
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner_type) = arg {
                            if Self::contains_recursive_type(parent, inner_type) {
                                return true;
                            }
                        }
                    }
                }

                false
            }
            // For other complex types, examine their inner types
            Type::Array(array) => Self::contains_recursive_type(parent, &array.elem),
            Type::Slice(slice) => Self::contains_recursive_type(parent, &slice.elem),
            Type::Reference(reference) => Self::contains_recursive_type(parent, &reference.elem),
            Type::Ptr(ptr) => Self::contains_recursive_type(parent, &ptr.elem),
            Type::Tuple(tuple) => tuple
                .elems
                .iter()
                .any(|elem| Self::contains_recursive_type(parent, elem)),
            Type::Group(group) => Self::contains_recursive_type(parent, &group.elem),
            Type::Paren(paren) => Self::contains_recursive_type(parent, &paren.elem),
            _ => false,
        }
    }
}

impl FieldParser for BaseParser {
    // TODO: remove parent field
    fn parse<'a>(_: &str, field: &'a Field) -> Result<ParsedField<'a>, TokenStream> {
        let ty = &field.ty;
        let export = quote!(<#ty as ronky::Exportable>::export());
        let is_optional = is_option_type(&field.ty);
        let attrs = match typeschema::extract(&field.attrs) {
            Ok(Some(attrs)) => {
                let is_nullable = attrs.is_nullable;

                if !is_optional && is_nullable {
                    let type_name = field.ty.to_token_stream().to_string();
                    return Err(quote_spanned!(field.ty.span() => compile_error!(concat!("Only an optional type can be nullable. Use Option<", #type_name, "> instead of ", #type_name))).into());
                }

                Some(quote! {
                    use ronky::Serializable;
                    ty.set_nullable(#is_nullable);
                })
            }
            Ok(None) => None,
            Err(stream) => return Err(stream),
        };

        let typeschema = quote! {
            {
                let mut ty = #export;
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
