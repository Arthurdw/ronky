mod elements;
mod r#enum;
mod enum_transformation;
mod exportable;
mod metadata;
mod properties;
mod r#ref;
mod serializable;
mod serializer;
mod tagged_union;
mod r#type;

pub use r#enum::EnumSchema;
pub use enum_transformation::EnumTransformation;
pub use exportable::Exportable;
pub use metadata::MetadataSchema;
pub use properties::PropertiesSchema;
pub use r#ref::RefSchema;
pub use serializable::Serializable;
pub use tagged_union::TaggedUnionSchema;
pub use r#type::{TypeSchema, Types};

pub mod type_utils {
    pub fn get_type_name_from(repr: impl ToString) -> String {
        repr.to_string()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|word| !word.is_empty() && word.chars().next().unwrap().is_uppercase())
            .collect::<String>()
    }

    pub fn get_type_name<T>() -> String
    where
        T: ?Sized,
    {
        get_type_name_from(std::any::type_name::<T>())
    }
}
