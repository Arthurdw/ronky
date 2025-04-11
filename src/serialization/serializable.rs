use std::fmt::Debug;

pub trait Serializable {
    fn serialize(&self) -> String;
}

impl Debug for dyn Serializable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

impl PartialEq for dyn Serializable {
    fn eq(&self, other: &Self) -> bool {
        self.serialize() == other.serialize()
    }
}

impl Eq for dyn Serializable {}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSerializable {
        value: String,
    }

    impl Serializable for MockSerializable {
        fn serialize(&self) -> String {
            self.value.clone()
        }
    }

    #[test]
    fn test_serialize() {
        let mock = MockSerializable {
            value: "test_value".to_string(),
        };
        assert_eq!(mock.serialize(), "test_value");
    }

    #[test]
    fn test_debug_trait() {
        let mock = MockSerializable {
            value: "debug_value".to_string(),
        };
        assert_eq!(format!("{:?}", &mock as &dyn Serializable), "debug_value");
    }

    #[test]
    fn test_partial_eq_trait() {
        let mock1 = MockSerializable {
            value: "value".to_string(),
        };
        let mock2 = MockSerializable {
            value: "value".to_string(),
        };
        assert_eq!(&mock1 as &dyn Serializable, &mock2 as &dyn Serializable);
    }
}
