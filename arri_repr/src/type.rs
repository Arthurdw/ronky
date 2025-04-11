// TODO: docs
use crate::{Serializable, serializer::Serializer};

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSchema {
    r#type: Types,
}

impl TypeSchema {
    pub fn new(r#type: Types) -> Self {
        Self { r#type }
    }
}

impl Serializable for TypeSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("type", &self.r#type)
            .build()
            .into()
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
        .to_string()
        .into()
    }
}
