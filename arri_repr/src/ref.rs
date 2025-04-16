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

#[derive(Debug, PartialEq, Eq)]
pub enum BoxOption<T> {
    Boxed(T),
    Ref(RefSchema),
}

impl<T: Serializable> Serializable for BoxOption<T> {
    fn serialize(&self) -> Option<String> {
        match self {
            Self::Boxed(value) => value.serialize(),
            Self::Ref(value) => value.serialize(),
        }
    }
}
