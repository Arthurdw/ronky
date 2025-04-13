use proc_macro::TokenStream;
use syn::Attribute;

use super::parse_arri_attrs;

#[derive(Debug, Default)]
pub(crate) struct PropertiesArguments {
    pub(crate) strict: bool,
}

pub(crate) fn extract(attrs: &[Attribute]) -> Result<PropertiesArguments, TokenStream> {
    let mut args = PropertiesArguments::default();
    parse_arri_attrs(attrs, &["strict"], |_, _| {
        args.strict = true;
        Ok(())
    })?;
    Ok(args)
}
