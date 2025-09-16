/// Enum representing various string transformation types.
///
/// These transformations can be applied to strings to convert them
/// into different cases, such as uppercase, lowercase, snake case,
/// camel case, or pascal case.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnumTransformation {
    /// Converts the string to uppercase.
    Uppercase,
    /// Converts the string to lowercase.
    Lowercase,
    /// Converts the string to snake case.
    Snakecase,
    /// Converts the string to camel case.
    Camelcase,
    /// Converts the string to pascal case.
    Pascalcase,
}

impl EnumTransformation {
    /// Applies the transformation to the given string.
    ///
    /// # Arguments
    ///
    /// * `value` - The input string to transform.
    ///
    /// # Returns
    ///
    /// A new string with the transformation applied.
    pub fn apply(&self, value: &str) -> String {
        use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};

        match self {
            Self::Uppercase => value.to_uppercase(),
            Self::Lowercase => value.to_lowercase(),
            Self::Snakecase => value.to_snake_case(),
            Self::Camelcase => value.to_lower_camel_case(),
            Self::Pascalcase => value.to_pascal_case(),
        }
    }
}

impl TryFrom<String> for EnumTransformation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for EnumTransformation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized = value.replace([' ', '-'], "_").to_lowercase();
        Ok(match normalized.as_str() {
            "uppercase" => Self::Uppercase,
            "lowercase" => Self::Lowercase,
            "snake_case" | "snakecase" => Self::Snakecase,
            "camelcase" => Self::Camelcase,
            "pascalcase" => Self::Pascalcase,
            _ => return Err(format!("Unknown transformation: {}", value)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
