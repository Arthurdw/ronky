use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    GenericArgument, Path, PathArguments, PathSegment, Type,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, PathSep},
};

use super::TypeParser;

pub struct ArriTypesParser;

impl ArriTypesParser {
    fn convert(identifier: &proc_macro2::Ident) -> TokenStream {
        // TODO: I feel like we can find a way where we don't have to repeat the quote macro
        // constantly?
        match identifier.to_string().as_str() {
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
            _ => quote_spanned!(
                    identifier.span() => 
                    compile_error!(concat!(
                        "Ronky/Arri does not (yet) support the type: ",
                        stringify!(#identifier),
                        ".\nIf you think this should be added please create an issue @ <https://github.com/Arthurdw/ronky/issues>"
                ))).into(),
        }
    }

    fn parse_arguments(args: &Punctuated<GenericArgument, Comma>) -> TokenStream {
        if args.is_empty() {
            return quote_spanned!(
                args.span() =>
                compile_error!("Unable to parse empty arguments, please use the full path to the type. If you think this should be supported, please create an issue @ <https://github.com/Arthurdw/ronky/issues>")
            ).into();
        }

        if !args.len() == 1 {
            // TODO: support this
            return quote_spanned!(args.span() => compile_error!("More than one argument is not supported yet")).into();
        }

        let arg = args.first().unwrap();
        let repr = match arg {
            GenericArgument::Type(Type::Path(path)) => {
                if path.path.segments.len() != 1 {
                    // TODO: support this
                    return quote_spanned!(
                        path.path.segments.span() =>
                        compile_error!("For now only the first segment is supported, please use the full path to the type.")).into();
                }
                let segment = path.path.segments.last().unwrap();
                &segment.ident
            }
            arg => return quote_spanned!(
                arg.span() =>
                compile_error!("Unsupported argument type, please use a type as the first argument. If you believe this should be supported please create an issue @ <https://github.com/Arthurdw/ronky/issues>")).into(),
        };
        ArriTypesParser::convert(&repr)
    }

    fn parse_segments(segments: &Punctuated<PathSegment, PathSep>) -> TokenStream {
        if segments.is_empty() {
            return quote_spanned!(
                segments.span() =>
                compile_error!("Unable to parse empty segments, please use the full path to the type. If you think this should be supported, please create an issue @ <https://github.com/Arthurdw/ronky/issues>")
            ).into();
        }

        let segment = segments.first().unwrap();

        match segment.ident.to_string().as_str() {
            "Option" => {
                let args = match &segment.arguments {
                    PathArguments::AngleBracketed(generics) => generics,
                    // TODO: is this reachable?
                    _ => return quote_spanned!(
                        segment.ident.span() =>
                        compile_error!("Unsupported Option type, please make sure you are using the build in (Rust) Option type with the `<` and `>` as generic arguments.")
                    ).into(),
                };
                assert_eq!(args.args.len(), 1);
                return Self::parse_arguments(&args.args);
            }
            _ => ArriTypesParser::convert(&segment.ident),
        }
    }
}

impl TypeParser for ArriTypesParser {
    fn parse(path: &Path) -> TokenStream {
        Self::parse_segments(&path.segments)
    }
}
