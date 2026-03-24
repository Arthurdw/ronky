use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{
    Attribute, LitBool, LitStr, Meta, Token, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

/// Parses a required `= "value"` after an attribute key.
pub(crate) fn parse_required_string(input: ParseStream, key_name: &str) -> syn::Result<LitStr> {
    if !input.peek(syn::token::Eq) {
        return Err(input.error(format!("Expected '=' after '{}'", key_name)));
    }
    input.parse::<syn::token::Eq>()?;
    input.parse::<LitStr>()
}

/// Parses an optional `= true/false` flag, defaulting to `true` when bare.
pub(crate) fn parse_flag(input: ParseStream) -> syn::Result<bool> {
    if input.peek(syn::token::Eq) {
        input.parse::<syn::token::Eq>()?;
        let value: LitBool = input.parse()?;
        Ok(value.value())
    } else {
        Ok(true)
    }
}

/// Parses a required `= "value"` or `= ["value1", "value2"]`.
pub(crate) fn parse_string_or_list(input: ParseStream, key_name: &str) -> syn::Result<Vec<String>> {
    if !input.peek(syn::token::Eq) {
        return Err(input.error(format!("Expected '=' after '{}'", key_name)));
    }
    input.parse::<syn::token::Eq>()?;

    if input.peek(syn::token::Bracket) {
        let content;
        bracketed!(content in input);
        let list = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;
        Ok(list.into_iter().map(|lit| lit.value()).collect())
    } else {
        let lit: LitStr = input.parse()?;
        Ok(vec![lit.value()])
    }
}

/// Defines an attribute argument struct with consistent parsing.
///
/// Generates the struct, its `Parse` impl (with key-matching loop, unknown-attribute
/// error, and comma handling), and an `extract()` function.
///
/// The `parse(args, input)` block names the variables available in handler blocks:
/// - `args`: the struct being populated
/// - `input`: the `ParseStream`
macro_rules! define_arri_attrs {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident : $field_ty:ty
            ),* $(,)?
        }

        parse($args:ident, $input:ident) {
            $( $key:literal => $handler:block )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Default)]
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_vis $field : $field_ty,
            )*
        }

        impl ::syn::parse::Parse for $name {
            fn parse($input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                let mut $args = Self::default();

                while !$input.is_empty() {
                    let key: ::syn::Ident = $input.parse()?;
                    let key_str = key.to_string();

                    match key_str.as_str() {
                        $( $key => $handler, )*
                        _ => return Err($input.error(format!("Unknown property: {}", key_str))),
                    }

                    super::goto_next($input)?;
                }

                Ok($args)
            }
        }

        pub(crate) fn extract(
            attrs: &[::syn::Attribute],
        ) -> Result<Vec<$name>, ::proc_macro::TokenStream> {
            super::parse_arri_attrs(attrs)
        }
    };
}

pub(crate) mod enum_variants;
pub(crate) mod fields;
pub(crate) mod properties;

/// Parses attributes with the `#[arri(...)]` format and extracts their arguments.
pub(crate) fn parse_arri_attrs<T: Parse>(attrs: &[Attribute]) -> Result<Vec<T>, TokenStream> {
    let attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("arri"))
        .collect();

    let mut parsed_attributes = Vec::new();

    for attr in attrs.iter() {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.tokens.is_empty() {
                return Err(quote_spanned!(meta_list.span() => compile_error!("No arguments were provided for this.")).into());
            }

            match meta_list.parse_args_with(T::parse) {
                Ok(res) => parsed_attributes.push(res),
                Err(err) => return Err(err.into_compile_error().into()),
            };
        } else {
            return Err(quote_spanned!(attr.span() => compile_error!("The only supported attribute format for arri is with a list of arguments. Expected usage: `#[arri(...)]`")).into());
        }
    }

    Ok(parsed_attributes)
}

/// Advances the parse stream to the next token, ensuring proper syntax.
pub(crate) fn goto_next(input: ParseStream) -> syn::Result<()> {
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    } else if !input.is_empty() {
        return Err(input.error("Expected ',' or end of input"));
    }

    Ok(())
}
