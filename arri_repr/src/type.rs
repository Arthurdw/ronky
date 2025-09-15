use crate::{MetadataSchema, Serializable};
use ronky_derive::Serializable as SerializableDerive;

/// Represents a schema for a type in an Arri schema.
///
/// This struct defines the type, optional metadata, and nullability
/// associated with the schema.
#[derive(Debug, PartialEq, Eq, SerializableDerive)]
#[arri_disable(nullable)]
pub struct TypeSchema {
    /// The type of the schema.
    r#type: Types,

    /// Optional metadata associated with the schema.
    metadata: Option<MetadataSchema>,

    /// Indicates whether the schema allows null values. If `Some(true)`,
    /// null values are permitted.
    is_nullable: Option<bool>,
}

impl TypeSchema {
    /// Creates a new `TypeSchema` instance with the specified type.
    ///
    /// # Arguments
    ///
    /// * `r#type` - The type of the schema.
    ///
    /// # Returns
    ///
    /// A new `TypeSchema` instance with default values for metadata and nullability.
    pub fn new(r#type: Types) -> Self {
        Self {
            r#type,
            metadata: None,
            is_nullable: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Types {
    String,
    Boolean,
    Timestamp,
    Float32,
    Float64,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
}

impl Serializable for Types {
    fn serialize(&self) -> Option<String> {
        (match self {
            Self::String => "string",
            Self::Boolean => "boolean",
            Self::Timestamp => "timestamp",
            Self::Float32 => "float32",
            Self::Float64 => "float64",
            Self::Int8 => "int8",
            Self::Uint8 => "uint8",
            Self::Int16 => "int16",
            Self::Uint16 => "uint16",
            Self::Int32 => "int32",
            Self::Uint32 => "uint32",
            Self::Int64 => "int64",
            Self::Uint64 => "uint64",
        })
        .serialize()
    }
}
