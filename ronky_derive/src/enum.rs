use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{
    DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Variant, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
};

use crate::{
    metadata,
    named_struct::export_struct_fields,
    parsers::{
        ParsedField,
        attributes::{
            enum_variants::{self, UnionKey, enum_transformation_to_tokens},
            fields,
        },
        parse_field,
    },
};

/// How an enum is represented in the exported schema.
enum Representation {
    /// All unit variants, exported using the "enum" schema form.
    Enum,
    /// Exported using the deprecated "discriminator" schema form.
    Discriminator(String),
    /// Exported using the "union" schema form without a tag key.
    ExternallyTagged,
    /// Exported using the "union" schema form with a tag key.
    InternallyTagged(UnionKey),
    /// Exported using the "union" schema form with both a tag and a value key.
    AdjacentlyTagged(UnionKey, UnionKey),
}

impl Representation {
    fn is_union(&self) -> bool {
        matches!(
            self,
            Self::ExternallyTagged | Self::InternallyTagged(_) | Self::AdjacentlyTagged(..)
        )
    }
}

/// Returns early with a `compile_error!` at the given span.
macro_rules! bail {
    ($span:expr => $msg:expr) => {
        return Err(quote_spanned!($span => compile_error!($msg);).into())
    };
}

/// Exports an enum as a `TokenStream` for use in schema generation.
///
/// Enums with only unit variants are exported as an "enum" schema. Enums with data are
/// exported as a "union" schema, unless the deprecated `discriminator` attribute is used.
///
/// # Arguments
///
/// * `input` - A reference to the `DeriveInput` representing the enum.
/// * `variants` - A reference to a `Punctuated` collection of `Variant` objects representing the enum's variants.
///
/// # Returns
///
/// Returns a `TokenStream` that defines the schema for the enum.
pub fn export_enum(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> TokenStream {
    let metadata: TokenStream2 = metadata::extract(&input.attrs).into();

    // Extract attributes for the enum
    let attrs = match enum_variants::extract(&input.attrs) {
        Ok(attrs) => attrs,
        Err(stream) => return stream,
    };
    let transform = attrs
        .iter()
        .flat_map(|attr| attr.transform.iter().map(enum_transformation_to_tokens))
        .collect::<Vec<TokenStream2>>();
    let discriminator = attrs.iter().find_map(|attr| attr.discriminator.clone());
    let tag_key = attrs.iter().find_map(|attr| attr.tag_key.clone());
    let value_key = attrs.iter().find_map(|attr| attr.value_key.clone());

    let representation = match resolve_representation(variants, discriminator, tag_key, value_key) {
        Ok(representation) => representation,
        Err(stream) => return stream,
    };

    let mut exported = Vec::new();
    for variant in variants.iter() {
        match export_variant(variant, &representation) {
            Ok(stream) => exported.push(stream),
            Err(stream) => return stream,
        }
    }

    let (schema, setup) = match &representation {
        Representation::Enum => (quote!(ronky::EnumSchema::new()), quote!()),
        Representation::Discriminator(discriminator) => (
            quote!(ronky::TaggedUnionSchema::new()),
            quote!(schema.set_discriminator(#discriminator);),
        ),
        Representation::ExternallyTagged => (quote!(ronky::UnionSchema::new()), quote!()),
        Representation::InternallyTagged(tag_key) => (quote!(ronky::UnionSchema::new()), {
            let tag_key = &tag_key.value;
            quote!(schema.set_tag_key(#tag_key);)
        }),
        Representation::AdjacentlyTagged(tag_key, value_key) => {
            (quote!(ronky::UnionSchema::new()), {
                let (tag_key, value_key) = (&tag_key.value, &value_key.value);
                quote! {
                    schema.set_tag_key(#tag_key);
                    schema.set_value_key(#value_key);
                }
            })
        }
    };

    // Generate the final schema
    quote! {
        use ronky::Serializable;
        let mut schema = #schema;
        schema.set_metadata(#metadata);
        schema.set_transforms(&[#(#transform),*]);
        #setup
        #(#exported)*

        schema
    }
    .into()
}

/// Validates the enum attributes against its variants and determines how it should be exported.
fn resolve_representation(
    variants: &Punctuated<Variant, Comma>,
    discriminator: Option<String>,
    tag_key: Option<UnionKey>,
    value_key: Option<UnionKey>,
) -> Result<Representation, TokenStream> {
    let has_data = variants.iter().any(|v| !v.fields.is_empty());

    if let Some(key) = tag_key.as_ref().or(value_key.as_ref())
        && discriminator.is_some()
    {
        return Err(quote_spanned!(key.span =>
            compile_error!("`discriminator` cannot be combined with `tag_key` or `value_key`. Use `tag_key` instead, the discriminator schema form is deprecated.");
        )
        .into());
    }

    match (discriminator, tag_key, value_key) {
        (Some(discriminator), ..) => {
            // The discriminator form requires every variant to be an object with data
            let mixed = variants
                .iter()
                .zip(variants.iter().skip(1))
                .find(|(prev, cur)| prev.fields.is_empty() != cur.fields.is_empty());
            if let Some((_, variant)) = mixed {
                return Err(quote_spanned!(variant.span() =>
                    compile_error!("Arri requires that enums using `discriminator` can only be all enum or all tagged union variants. This variant violates that rule. Remove `discriminator` to export a union, which supports unit variants.");
                )
                .into());
            }
            if !has_data {
                return Err(quote_spanned!(discriminator.span() =>
                    compile_error!("Discriminator can only be used with tagged enums.");
                )
                .into());
            }
            Ok(Representation::Discriminator(discriminator))
        }
        (None, None, Some(value_key)) => Err(quote_spanned!(value_key.span =>
            compile_error!("`value_key` can only be used alongside `tag_key`.");
        )
        .into()),
        (None, Some(tag_key), Some(value_key)) => {
            if tag_key.value == value_key.value {
                return Err(quote_spanned!(value_key.span =>
                    compile_error!("`value_key` must be different from `tag_key`.");
                )
                .into());
            }
            Ok(Representation::AdjacentlyTagged(tag_key, value_key))
        }
        (None, Some(tag_key), None) => {
            if let Some(variant) = variants
                .iter()
                .find(|v| matches!(v.fields, Fields::Unnamed(_)) && !v.fields.is_empty())
            {
                return Err(quote_spanned!(variant.span() =>
                    compile_error!("Internally tagged unions (`tag_key` without `value_key`) only support unit and struct variants. Add a `value_key` to use an adjacently tagged union instead.");
                )
                .into());
            }
            Ok(Representation::InternallyTagged(tag_key))
        }
        (None, None, None) => {
            // Enums with only unit variants become an "enum", anything else an externally tagged union
            if has_data {
                Ok(Representation::ExternallyTagged)
            } else {
                Ok(Representation::Enum)
            }
        }
    }
}

/// Generates the code that adds a single variant to the schema.
fn export_variant(
    variant: &Variant,
    representation: &Representation,
) -> Result<TokenStream2, TokenStream> {
    // Extract attributes for the variant
    let attrs = fields::extract(&variant.attrs)?;

    let variant_name = attrs
        .iter()
        .rev()
        .find(|a| a.rename.is_some())
        .and_then(|a| a.rename.as_ref())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // Strip the r# prefix if present (for raw identifiers like r#type)
            let name = variant.ident.to_string();
            name.strip_prefix("r#").unwrap_or(&name).to_string()
        });

    let variant_metadata: Option<TokenStream2> =
        metadata::extract_attrs(&variant.attrs).map(Into::into);
    let set_variant_metadata = |target: TokenStream2| {
        variant_metadata.as_ref().map(|ts| {
            quote! {
                use ronky::Serializable;
                #target.set_metadata(#ts);
            }
        })
    };

    let add = if representation.is_union() {
        quote!(add_variant)
    } else {
        quote!(add_mapping)
    };

    // `Unit`, `Unit {}` and `Unit()` are all treated as unit variants
    let stream = match (&variant.fields, representation) {
        (fields, Representation::Enum) if fields.is_empty() => {
            // TODO: followup on request of having a list of metadata as variants, as we can't provide
            // any docs or deprecation notices to a variant :/
            // See current state here: https://discord.com/channels/1272569268869005322/1272569269342965874/1363165596048429137
            quote! {
                schema.add_variant(#variant_name);
            }
        }
        (fields, _) if fields.is_empty() => {
            // By convention unit variants of a union are represented as an empty object:
            // - externally tagged: `{ "<variant>": {} }`
            // - internally tagged: `{ "<tag_key>": "<variant>" }`
            // - adjacently tagged: `{ "<tag_key>": "<variant>", "<value_key>": {} }`
            let metadata = set_variant_metadata(quote!(export));
            quote! {
                schema.#add(#variant_name, Box::new({
                    let mut export = ronky::PropertiesSchema::new();
                    #metadata
                    export
                }));
            }
        }
        (Fields::Named(fields), _) => {
            if let Representation::InternallyTagged(tag_key) = representation {
                check_no_tag_conflict(fields, tag_key)?;
            }

            let metadata = set_variant_metadata(quote!(export));
            let struct_export: TokenStream2 = export_struct_fields(&fields.named, &None).into();

            quote! {
                schema.#add(#variant_name, Box::new({
                    let mut export = {#struct_export};
                    #metadata
                    export
                }));
            }
        }
        (Fields::Unnamed(fields), Representation::Discriminator(_)) => {
            let (field_stream, field_metadata) = export_unnamed_field(variant, fields, false)?;
            let metadata = set_variant_metadata(quote!(export));

            quote! {
                schema.add_mapping(#variant_name, Box::new({
                    let mut export = ronky::PropertiesSchema::new();
                    #metadata
                    export.set_property("value", Box::new({
                        let mut ty = #field_stream;
                        #field_metadata
                        ty
                    }));
                    export
                }));
            }
        }
        (Fields::Unit, _) => unreachable!("unit variants are handled above"),
        (Fields::Unnamed(fields), _) => {
            // Newtype variants use the schema of their field directly
            let (field_stream, field_metadata) = export_unnamed_field(variant, fields, true)?;
            let metadata = set_variant_metadata(quote!(ty));

            quote! {
                schema.add_variant(#variant_name, Box::new({
                    let mut ty = #field_stream;
                    #field_metadata
                    #metadata
                    ty
                }));
            }
        }
    };

    Ok(stream)
}

/// Generates the schema and metadata code for the single field of an unnamed (newtype) variant.
///
/// When `allow_optional` is set, `Option<T>` fields are exported as a nullable `T`.
fn export_unnamed_field(
    variant: &Variant,
    fields: &FieldsUnnamed,
    allow_optional: bool,
) -> Result<(TokenStream2, Option<TokenStream2>), TokenStream> {
    if fields.unnamed.len() != 1 {
        bail!(variant.span() => "Unamed enums variants must have exactly one field.");
    }

    let (field, stream, args, is_optional) = match parse_field(fields.unnamed.first().unwrap())? {
        ParsedField::Required(field, stream, args) => (field, stream, args, false),
        ParsedField::Optional(field, stream, args) => (field, stream, args, true),
    };

    if is_optional && !allow_optional {
        bail!(variant.span() => "Optional fields are not supported in unnamed enums.");
    }

    let mut stream: TokenStream2 = stream.into();
    // An explicit `#[arri(nullable = ...)]` is already handled by the field parser.
    if is_optional && args.iter().all(|a| a.is_nullable.is_none()) {
        stream = quote! {{
            use ronky::Serializable;
            let mut ty = #stream;
            ty.set_nullable(true);
            ty
        }};
    }

    let field_metadata = metadata::extract_from_field(field).map(|ts| {
        let ts: TokenStream2 = ts.into();
        quote! {
            use ronky::Serializable;
            ty.set_metadata(#ts);
        }
    });

    Ok((stream, field_metadata))
}

/// Ensures none of the fields of an internally tagged variant collide with the tag key.
fn check_no_tag_conflict(fields: &FieldsNamed, tag_key: &UnionKey) -> Result<(), TokenStream> {
    let tag_key = &tag_key.value;

    for field in fields.named.iter() {
        let renamed = fields::extract(&field.attrs)?
            .into_iter()
            .rev()
            .find_map(|a| a.rename);
        let ident = field.ident.as_ref().unwrap();
        let name = renamed.unwrap_or_else(|| {
            let name = ident.to_string();
            name.strip_prefix("r#").unwrap_or(&name).to_string()
        });

        if &name == tag_key {
            bail!(ident.span() => "This field has the same name as the union's `tag_key`, which is not allowed for internally tagged unions.");
        }
    }

    Ok(())
}
