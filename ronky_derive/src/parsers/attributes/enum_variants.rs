use arri_common::EnumTransformation;
use quote::quote;

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

define_arri_attrs! {
    /// Represents parsed attributes for enum variants.
    pub(crate) struct EnumVariants {
        /// List of transformations to apply to the enum variants.
        pub(crate) transform: Vec<EnumTransformation>,
        /// Optional discriminator value for the enum variants.
        pub(crate) discriminator: Option<String>,
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
    }
}
