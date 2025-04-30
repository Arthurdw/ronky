use crate::{Serializable, serializer::Serializer};

/// Represents a reference schema in an Arri schema.
///
/// This struct is used to define a reference to another schema.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct RefSchema {
    /// The reference string pointing to another schema.
    pub r#ref: String,
}

impl RefSchema {
    /// Creates a new `RefSchema` instance.
    ///
    /// # Arguments
    ///
    /// * `r#ref` - A value that can be converted to a string, representing the reference.
    ///
    /// # Returns
    ///
    /// A new `RefSchema` instance.
    pub fn new(r#ref: impl ToString) -> Self {
        Self {
            r#ref: r#ref.to_string(),
        }
    }
}

impl Serializable for RefSchema {
    /// Serializes the `RefSchema` into a string representation.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the serialized schema, or `None` if serialization fails.
    fn serialize(&self) -> Option<String> {
        Serializer::builder().set("ref", &self.r#ref).build().into()
    }
}
