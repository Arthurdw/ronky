use crate::{Serializable, serializer::Serializer};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct RefSchema {
    pub r#ref: String,
}

impl RefSchema {
    pub fn new(r#ref: impl ToString) -> Self {
        Self {
            r#ref: r#ref.to_string(),
        }
    }
}

impl Serializable for RefSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder().set("ref", &self.r#ref).build().into()
    }
}
