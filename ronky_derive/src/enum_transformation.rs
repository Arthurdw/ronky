/// Enum representing various string transformation types.
///
/// These transformations can be applied to strings to convert them
/// into different cases, such as uppercase, lowercase, snake case,
/// camel case, or pascal case.
///
/// Note: This is a copy of the EnumTransformation from arri_repr,
/// included here to break the circular dependency. The transformation
/// methods are not needed in ronky_derive and are implemented in arri_repr.
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
