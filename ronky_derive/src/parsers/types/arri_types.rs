use proc_macro::TokenStream;
use quote::quote;
use syn::{
    GenericArgument, Path, PathArguments, PathSegment, Type,
    punctuated::Punctuated,
    token::{Comma, PathSep},
};

use super::TypeParser;

pub struct ArriTypesParser;

impl ArriTypesParser {
    fn convert(repr: &str) -> TokenStream {
        // TODO: I feel like we can find a way where we don't have to repeat the quote macro
        // constantly?
        match repr {
            "String" => quote!(ronky::Types::String).into(),
            "bool" => quote!(ronky::Types::Boolean).into(),
            "f32" => quote!(ronky::Types::Float32).into(),
            "f64" => quote!(ronky::Types::Float64).into(),
            "i8" => quote!(ronky::Types::Int8).into(),
            "u8" => quote!(ronky::Types::Uint8).into(),
            "i16" => quote!(ronky::Types::Int16).into(),
            "u16" => quote!(ronky::Types::Uint16).into(),
            "i32" => quote!(ronky::Types::Int32).into(),
            "u32" => quote!(ronky::Types::Uint32).into(),
            "i64" => quote!(ronky::Types::Int64).into(),
            "u64" => quote!(ronky::Types::Uint64).into(),
            _ => panic!(
                "Ronky/Arri does not (yet) support the type: {}. If you think this should be added please create an issue @ <https://github.com/Arthurdw/ronky/issues>",
                repr
            ),
        }
    }

    fn parse_arguments(args: &Punctuated<GenericArgument, Comma>) -> TokenStream {
        if args.is_empty() {
            panic!("Cannot parse empty arguments");
        }

        if !args.len() == 1 {
            // TODO: support this
            panic!("More than one argument is not supported yet");
        }

        let arg = args.first().unwrap();
        let repr = match arg {
            GenericArgument::Type(Type::Path(path)) => {
                if path.path.segments.len() != 1 {
                    panic!("Unsupported type");
                }
                let segment = path.path.segments.last().unwrap();
                segment.ident.to_string()
            }
            _ => panic!("Unsupported argument type"),
        };
        ArriTypesParser::convert(&repr)
    }

    fn parse_segments(segments: &Punctuated<PathSegment, PathSep>) -> TokenStream {
        if segments.is_empty() {
            panic!("Cannot parse empty segments");
        }

        let segment = segments.first().unwrap();

        match segment.ident.to_string().as_str() {
            "Option" => {
                let args = match &segment.arguments {
                    PathArguments::AngleBracketed(generics) => generics,
                    _ => panic!("Unsupported Option type"),
                };
                assert_eq!(args.args.len(), 1);
                return Self::parse_arguments(&args.args);
            }
            repr => ArriTypesParser::convert(repr),
        }
    }
}

impl TypeParser for ArriTypesParser {
    fn parse(path: &Path) -> TokenStream {
        Self::parse_segments(&path.segments)
    }
}
