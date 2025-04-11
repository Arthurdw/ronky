// Note: we should be able to parse and serialize the type
//
// Type schema form:
// ```json
// { "type": "boolean" }
// ```

use crate::serialization::Types;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSchema {
    r#type: Types,
}
