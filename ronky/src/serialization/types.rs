use super::Serializable;

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
    fn serialize(&self) -> String {
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
    }
}

// TODO: implement conversion from ATD to Rust types
// | ATD Type | Rust Type |
// |---|---|
// | string | String |
// | boolean | bool |
// | timestamp | DateTime |
// | float32 | f32 |
// | float64 | f64 |
// | int8 | i8 |
// | uint8 | u8 |
// | int16 | i16 |
// | uint16 | u16 |
// | int32 | i32 |
// | uint32 | u32 |
// | int64 | i64 |
// | uint64 | u64 |
