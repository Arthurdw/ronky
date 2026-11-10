use arri_common::EnumTransformation;
use proc_macro2::Span;
use quote::quote;
use syn::parse::ParseStream;

use super::{parse_required_string, parse_string_or_list};

/// Macro to match an `EnumTransformation` variant and generate corresponding tokens.
macro_rules! enum_transformation_match {
    ($transform:expr => $($variant:ident),*) => {
        match $transform {
            $(
                EnumTransformation::$variant => quote! { ronky::EnumTransformation::$variant },
            )*
        }
    };
}

/// Converts an `EnumTransformation` into a token stream representation.
pub fn enum_transformation_to_tokens(transform: &EnumTransformation) -> proc_macro2::TokenStream {
    enum_transformation_match!(
        transform => Uppercase, Lowercase, Snakecase, Camelcase, Pascalcase,
                     Kebabcase, Screamingkebabcase, Screamingsnakecase
    )
}

/// A `tag_key` / `value_key` of a union, along with where it was defined.
#[derive(Debug, Clone)]
pub(crate) struct UnionKey {
    pub(crate) value: String,
    pub(crate) span: Span,
}

/// Parses a `tag_key` / `value_key`, ensuring it is a non-empty JSON key.
fn parse_union_key(input: ParseStream, key_name: &str) -> syn::Result<UnionKey> {
    let lit = parse_required_string(input, key_name)?;
    let value = lit.value();
    if value.is_empty() {
        return Err(syn::Error::new(
            lit.span(),
            format!("`{}` cannot be empty", key_name),
        ));
    }
    Ok(UnionKey {
        value,
        span: lit.span(),
    })
}

define_arri_attrs! {
    /// Represents parsed attributes for enum variants.
    pub(crate) struct EnumVariants {
        /// List of transformations to apply to the enum variants.
        pub(crate) transform: Vec<EnumTransformation>,
        /// Optional discriminator value for the enum variants.
        /// Produces the deprecated "discriminator" schema form.
        pub(crate) discriminator: Option<String>,
        /// Optional JSON key holding the variant's tag (internally / adjacently tagged unions).
        pub(crate) tag_key: Option<UnionKey>,
        /// Optional JSON key holding the variant's value (adjacently tagged unions).
        pub(crate) value_key: Option<UnionKey>,
    }

    parse(args, input) {
        "transform" => {
            let transforms = parse_string_or_list(input, "transform")?;
            let transforms = transforms
                .into_iter()
                .map(|s| EnumTransformation::try_from(s).map_err(|e| input.error(e)))
                .collect::<Result<Vec<_>, _>>()?;
            args.transform.extend(transforms);
        }
        "discriminator" => {
            let value = parse_required_string(input, "discriminator")?;
            args.discriminator = Some(value.value());
        }
        "tag_key" => {
            args.tag_key = Some(parse_union_key(input, "tag_key")?);
        }
        "value_key" => {
            args.value_key = Some(parse_union_key(input, "value_key")?);
        }
    }
}
