use crate::{Serializable, TypeSchema, Types, elements::ElementSchema};

pub trait Exportable {
    fn export() -> impl Serializable;
}

// TODO: document this lol
macro_rules! exportable {
    // --- Main entry points ---
    // Handle individual blocks
    (generic: { $($gen:tt)* }) => {
        exportable!(@parse_impls $($gen)*);
    };
    (typeschema: { $($ty:tt)* }) => {
        exportable!(@parse_typeschema $($ty)*);
    };
    (features: { $($feat:tt)* }) => {
        exportable!(@parse_features $($feat)*);
    };

    // Handle combination of blocks
    (
        typeschema: { $($ty:tt)* },
        generic: { $($gen:tt)* },
        features: { $($feat:tt)* }
    ) => {
        exportable!(typeschema: { $($ty)* });
        exportable!(generic: { $($gen)* });
        exportable!(features: { $($feat)* });
    };

    // --- Parse feature blocks ---
    (@parse_features $feature:literal => { $($body:item)* }, $($rest:tt)*) => {
        exportable!(@parse_features $feature => { $($body)* });
        exportable!(@parse_features $($rest)*);
    };
    (@parse_features $feature:literal => { $($body:item)* }) => {
        $(
            #[cfg(feature = $feature)]
            $body
        )*
    };
    (@parse_features) => {};

    // --- TypeSchema implementation parsers ---
    // TypeSchema with identifier
    (@parse_typeschema $ty:ty => $to:ident, $($rest:tt)*) => {
        exportable!(@parse_typeschema $ty => {
            TypeSchema::new(Types::$to)
        }, $($rest)*);
    };

    // TypeSchema with block
    (@parse_typeschema $ty:ty => $implementation:block, $($rest:tt)*) => {
        impl Exportable for $ty {
            fn export() -> impl Serializable {
                $implementation
            }
        }
        exportable!(@parse_typeschema $($rest)*);
    };

    // Termination case for typeschema
    (@parse_typeschema) => {};

    // --- Generic implementation parsers ---
    // Generic implementation with expression
    (@parse_impls $type:ident < $($type_params:ident),* > => $implementation:expr, $($rest:tt)*) => {
        impl<$($type_params: 'static + Exportable),*> Exportable for $type<$($type_params),*> {
            fn export() -> impl Serializable {
                $implementation
            }
        }
        exportable!(@parse_impls $($rest)*);
    };

    // Generic implementation with block
    (@parse_impls $type:ident < $($type_params:ident),* > => $implementation:block, $($rest:tt)*) => {
        exportable!(@parse_impls $type < $($type_params),* > => {
            $implementation
        }, $($rest)*);
    };

    // Termination case for generic implementations
    (@parse_impls) => {};
}

exportable! {
    typeschema: {
        String => String,
        &str => String,
        bool => Boolean,
        f32 => Float32,
        f64 => Float64,
        i8 => Int8,
        u8 => Uint8,
        i16 => Int16,
        u16 => Uint16,
        i32 => Int32,
        u32 => Uint32,
        i64 => Int64,
        u64 => Uint64,
    },
    generic: {
        Option<T> =>  T::export(),
        Vec<T> => ElementSchema::new(Box::new(T::export())),
        Box<T> =>  {
            // TODO: find way to compare current name with parent
            // std::any::type_name::<T>().split("::").last().unwrap().to_string()
            T::export()
        },
    },
    features: {
        "chrono" => {
            use chrono::{DateTime, FixedOffset, Utc};

            exportable! {
                typeschema: {
                    DateTime<FixedOffset> => Timestamp,
                    DateTime<Utc> => Timestamp,
                }
            }

            #[test]
            fn test_serialize_datetime() {
                let fixed_offset = DateTime::<FixedOffset>::export().serialize();
                let utc = DateTime::<Utc>::export().serialize();
                let expected = TypeSchema::new(Types::Timestamp).serialize();

                assert_eq!(fixed_offset, expected);
                assert_eq!(utc, expected);
            }
        }
    }
}
