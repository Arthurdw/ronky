use sonic_rs::Error as SonicError;

/// Trait for types that can be serialized to JSON
pub trait ExportedSerialize {
    fn to_json(&self) -> Result<String, SonicError>;
}

/// Trait for types that can be deserialized from JSON
pub trait ExportedDeserialize: Sized {
    fn from_json(json: &str) -> Result<Self, SonicError>;
}

// Blanket implementation for all types that implement serde::Serialize
impl<T> ExportedSerialize for T
where
    T: serde::Serialize,
{
    fn to_json(&self) -> Result<String, SonicError> {
        sonic_rs::to_string(self)
    }
}

// Blanket implementation for all types that implement serde::Deserialize
impl<T> ExportedDeserialize for T
where
    T: for<'de> serde::Deserialize<'de>,
{
    fn from_json(json: &str) -> Result<Self, SonicError> {
        sonic_rs::from_str(json)
    }
}
