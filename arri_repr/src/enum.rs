use crate::{MetadataSchema, Serializable, serializer::Serializer};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnumTransformation {
    Uppercase,
    Lowercase,
    Snakecase,
    Camelcase,
    Pascalcase,
}

impl EnumTransformation {
    pub fn apply(&self, value: &str) -> String {
        match self {
            Self::Uppercase => value.to_uppercase(),
            Self::Lowercase => value.to_lowercase(),
            Self::Snakecase => Self::to_snake_case(self, value),
            Self::Camelcase => Self::to_camel_case(self, value),
            Self::Pascalcase => Self::to_pascal_case(self, value),
        }
    }

    /// Converts a string to snake case.
    /// Examples:
    /// - "hello world" -> "hello_world"
    /// - "hello_world" -> "hello_world"
    /// - "helloWorld" -> "hello_world"
    /// - "HelloWorld" -> "hello_world"
    fn to_snake_case(&self, value: &str) -> String {
        value
            .replace(" ", "_")
            .replace("-", "_")
            .chars()
            .flat_map(|c| {
                if c.is_uppercase() {
                    vec!['_', c.to_ascii_lowercase()]
                } else {
                    vec![c]
                }
            })
            .collect::<String>()
            .trim_start_matches('_')
            .to_string()
    }

    /// Converts a string to camel case.
    /// Examples:
    /// - "hello world" -> "helloWorld"
    /// - "hello_world" -> "helloWorld"
    /// - "helloWorld" -> "helloWorld"
    /// - "HelloWorld" -> "helloWorld"
    fn to_camel_case(&self, value: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        if let Some(first_char) = value.chars().next() {
            result.push(first_char.to_ascii_lowercase());
        }

        for c in value.chars().skip(1) {
            if c == '_' || c == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Converts a string to pascal case.
    /// Examples:
    /// - "hello world" -> "HelloWorld"
    /// - "hello_world" -> "HelloWorld"
    /// - "helloWorld" -> "HelloWorld"
    /// - "HelloWorld" -> "HelloWorld"
    fn to_pascal_case(&self, value: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in value.chars() {
            if c == '_' || c == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

impl TryFrom<String> for EnumTransformation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "uppercase" => Self::Uppercase,
            "UPPERCASE" => Self::Uppercase,
            "lowercase" => Self::Lowercase,
            "snake_case" => Self::Snakecase,
            "snakecase" => Self::Snakecase,
            "camelCase" => Self::Camelcase,
            "camelcase" => Self::Camelcase,
            "PascalCase" => Self::Pascalcase,
            "pascalcase" => Self::Pascalcase,
            _ => return Err(format!("Unknown transformation: {}", value)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct EnumSchema {
    pub r#enum: Vec<String>,
    pub metadata: Option<MetadataSchema>,
    pub transformations: Vec<EnumTransformation>,
}

impl EnumSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_variant(&mut self, variant: impl ToString) {
        let transformed = self
            .transformations
            .iter()
            .fold(variant.to_string(), |acc, transform| transform.apply(&acc));

        self.r#enum.push(transformed);
    }

    pub fn set_transforms(&mut self, transformations: &[EnumTransformation]) {
        self.transformations = transformations.into();
    }
}

impl Serializable for EnumSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("enum", &self.r#enum)
            .set("metadata", &self.metadata)
            .build()
            .into()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(metadata);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_serialize() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1");
        enum_schema.add_variant("Variant2");
        let serialized: serde_json::Value =
            serde_json::from_str(&enum_schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({ "enum": ["Variant1", "Variant2"] })
        );
    }

    #[test]
    fn test_add_variant() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1".to_string());
        enum_schema.add_variant("Variant2".to_string());

        assert_eq!(
            enum_schema.r#enum,
            vec!["Variant1".to_string(), "Variant2".to_string()]
        );
    }

    #[test]
    fn test_set_metadata() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1".to_string());
        let metadata = MetadataSchema::default();
        enum_schema.set_metadata(metadata.clone());

        assert_eq!(enum_schema.metadata, Some(metadata));
    }

    #[test]
    fn test_serialize_with_metadata() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1".to_string());
        let metadata = MetadataSchema::default();
        enum_schema.set_metadata(metadata);

        let serialized: serde_json::Value =
            serde_json::from_str(&enum_schema.serialize().unwrap()).unwrap();

        assert!(serialized.get("metadata").is_some());
    }

    #[test]
    fn test_to_snake_case() {
        let helper = EnumTransformation::Snakecase;
        assert_eq!(helper.apply("hello world"), "hello_world");
        assert_eq!(helper.apply("helloWorld"), "hello_world");
        assert_eq!(helper.apply("HelloWorld"), "hello_world");
        assert_eq!(helper.apply("hello_world"), "hello_world");
    }

    #[test]
    fn test_to_camel_case() {
        let helper = EnumTransformation::Camelcase;
        assert_eq!(helper.apply("hello world"), "helloWorld");
        assert_eq!(helper.apply("hello_world"), "helloWorld");
        assert_eq!(helper.apply("helloWorld"), "helloWorld");
        assert_eq!(helper.apply("HelloWorld"), "helloWorld");
    }

    #[test]
    fn test_to_pascal_case() {
        let helper = EnumTransformation::Pascalcase;
        assert_eq!(helper.apply("hello world"), "HelloWorld");
        assert_eq!(helper.apply("hello_world"), "HelloWorld");
        assert_eq!(helper.apply("helloWorld"), "HelloWorld");
        assert_eq!(helper.apply("HelloWorld"), "HelloWorld");
    }

    #[test]
    fn test_to_uppercase() {
        let helper = EnumTransformation::Uppercase;
        assert_eq!(helper.apply("hello world"), "HELLO WORLD");
        assert_eq!(helper.apply("hello_world"), "HELLO_WORLD");
        assert_eq!(helper.apply("helloWorld"), "HELLOWORLD");
        assert_eq!(helper.apply("HelloWorld"), "HELLOWORLD");
    }

    #[test]
    fn test_to_lowercase() {
        let helper = EnumTransformation::Lowercase;
        assert_eq!(helper.apply("hello world"), "hello world");
        assert_eq!(helper.apply("hello_world"), "hello_world");
        assert_eq!(helper.apply("helloWorld"), "helloworld");
        assert_eq!(helper.apply("HelloWorld"), "helloworld");
    }
}
