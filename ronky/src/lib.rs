#[cfg(feature = "derive")]
extern crate ronky_derive;

#[cfg(feature = "derive")]
pub use ronky_derive::Exported;

extern crate arri_repr;
pub use arri_repr::*;

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
