use syn::LitStr;

use super::{parse_flag, parse_required_string};

fn validate_rename(value: &LitStr) -> syn::Result<()> {
    let new_name = value.value();

    if new_name.is_empty() {
        Err(syn::Error::new(value.span(), "A rename cannot be empty"))
    } else if new_name.contains(' ') {
        Err(syn::Error::new(
            value.span(),
            "A rename cannot contain spaces",
        ))
    } else if new_name.starts_with(|c: char| c.is_numeric()) {
        Err(syn::Error::new(
            value.span(),
            "A rename cannot start with a number",
        ))
    } else if new_name
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && c != '_')
    {
        Err(syn::Error::new(
            value.span(),
            "A rename can only contain a-z, A-Z and 0-9 or _",
        ))
    } else {
        Ok(())
    }
}

define_arri_attrs! {
    /// Represents parsed attributes for struct fields.
    pub(crate) struct FieldArguments {
        /// Optional rename value for the field.
        pub(crate) rename: Option<String>,
        /// Indicates whether the type is nullable.
        pub(crate) is_nullable: Option<bool>,
    }

    parse(args, input) {
        "rename" => {
            let value = parse_required_string(input, "rename")?;
            validate_rename(&value)?;
            args.rename = Some(value.value());
        }
        "nullable" => {
            args.is_nullable = Some(parse_flag(input)?);
        }
    }
}
