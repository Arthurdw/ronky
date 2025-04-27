use crate::{RefSchema, ValuesSchema, type_utils};
use crate::{Serializable, TypeSchema, Types, elements::ElementsSchema};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{OsStr, OsString};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroUsize,
};
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32,
    AtomicU64,
};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};

thread_local! {
    // Track which types we've started exporting (even if not completed)
    static RECURSION_TRACKER: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub trait Exportable {
    fn get_type_name() -> String {
        type_utils::get_type_name::<Self>()
    }

    fn export() -> Box<dyn Serializable> {
        Self::export_with_recursion_check()
    }

    fn export_internal() -> impl Serializable;

    fn export_with_recursion_check() -> Box<dyn Serializable> {
        let type_name = Self::get_type_name();

        let is_recursive = RECURSION_TRACKER.with(|tracker| {
            let mut tracker = tracker.borrow_mut();
            let is_recursive = tracker.contains(&type_name);

            if !is_recursive {
                // Add our type to the set
                tracker.insert(type_name.clone());
            }

            is_recursive
        });

        if is_recursive {
            return Box::new(RefSchema::new(type_utils::get_type_name_from(type_name)));
        }

        let result = Self::export_internal();

        // Remove our type from the set when done
        RECURSION_TRACKER.with(|tracker| {
            let mut tracker = tracker.borrow_mut();
            tracker.remove(&type_name);
        });

        Box::new(result)
    }
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

    (
        typeschema: { $($ty:tt)* },
        generic: { $($gen:tt)* }
    ) => {
        exportable!(typeschema: { $($ty)* });
        exportable!(generic: { $($gen)* });
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
            fn export_internal() -> impl Serializable {
                $implementation
            }
        }
        exportable!(@parse_typeschema $($rest)*);
    };

    // Termination case for typeschema
    (@parse_typeschema) => {};

    // --- Generic implementation parsers ---
    // Generic implementation with expression - with trait bounds
    (@parse_impls $type:ident < $($type_param:ident $(: $trait_bound:path)?),* $(,)? > => $implementation:expr, $($rest:tt)*) => {
        impl<$($type_param: 'static + Exportable $(+ $trait_bound)?),*> Exportable for $type<$($type_param),*> {
            fn export_internal() -> impl Serializable {
                $implementation
            }
            fn get_type_name() -> String {
                format!(
                    "::ronky::--virtual--::generic::{}",
                    vec![$($type_param::get_type_name()),*].join("")
                )
            }
        }
        exportable!(@parse_impls $($rest)*);
    };

    // Generic implementation with block
    (@parse_impls $type:ident < $($type_param:ident $(: $trait_bound:path)?),* $(,)? > => $implementation:block, $($rest:tt)*) => {
        exportable!(@parse_impls $type < $($type_param $(: $trait_bound)?),* > => {
            $implementation
        }, $($rest)*);
    };

    // Termination case for generic implementations
    (@parse_impls) => {};
}

type SliceOf<T> = [T]; // This way of interacting with slices allows us to keep the same macro

exportable! {
    typeschema: {
        char => String,
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
        AtomicBool => Boolean,
        AtomicI8 => Int8,
        AtomicU8 => Uint8,
        AtomicI16 => Int16,
        AtomicU16 => Uint16,
        AtomicI32 => Int32,
        AtomicU32 => Uint32,
        AtomicI64 => Int64,
        AtomicU64 => Uint64,
        NonZeroI8 => Int8,
        NonZeroU8 => Uint8,
        NonZeroI16 => Int16,
        NonZeroU16 => Uint16,
        NonZeroI32 => Int32,
        NonZeroU32 => Uint32,
        NonZeroI64 => Int64,
        NonZeroU64 => Uint64,
        NonZeroIsize => Int64,
        NonZeroUsize => Uint64,
        OsStr => String,
        OsString => String,
        PathBuf => String,
        Path => String,
        IpAddr => String,
        Ipv4Addr => String,
        Ipv6Addr => String,
        SocketAddr => String,
        Duration => Int64,
        Instant => Int64,
        SystemTime => Int64,
    },
    generic: {
        // Ignored types
        Option<T> => T::export(), // Option is a special case, this gets handled in the proc macro
        Rc<T> => T::export(),
        Arc<T> => T::export(),
        Cell<T> => T::export(),
        RefCell<T> => T::export(),
        Mutex<T> => T::export(),
        RwLock<T> => T::export(),
        NonNull<T> => T::export(),

        // General exports
        Box<T> => T::export_with_recursion_check(),

        // Element Schema's
        SliceOf<T> => ElementsSchema::new(Box::new(T::export())),
        Vec<T> => ElementsSchema::new(Box::new(T::export())),
        VecDeque<T> => ElementsSchema::new(Box::new(T::export())),
        LinkedList<T> => ElementsSchema::new(Box::new(T::export())),
        HashSet<T> => ElementsSchema::new(Box::new(T::export())),
        BTreeSet<T> => ElementsSchema::new(Box::new(T::export())),
        BinaryHeap<T> => ElementsSchema::new(Box::new(T::export())),

        // Values Schema's
        HashMap<K: ToString, V> => ValuesSchema::new(Box::new(V::export())),
        BTreeMap<K: ToString, V> => ValuesSchema::new(Box::new(V::export())),
    },
    features: {
        "chrono" => {
            use chrono::{DateTime, FixedOffset, Utc, Local, NaiveTime, NaiveDate, NaiveDateTime, TimeZone};

            exportable! {
                typeschema: {
                    DateTime<Utc> => Timestamp,
                    DateTime<Local> => Timestamp,
                    DateTime<FixedOffset> => Timestamp,
                    NaiveDate => Timestamp,
                    NaiveTime => Timestamp,
                    NaiveDateTime => Timestamp,
                    chrono::Duration => Int64,
                },
                generic: {
                    DateTime<Tz: TimeZone> => TypeSchema::new(Types::Timestamp),
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
        },
        "uuid" => {
            exportable! {
                typeschema: {
                    uuid::Uuid => String,
                }
            }

            #[test]
            fn test_serialize_uuid() {
                let uuid = uuid::Uuid::export().serialize();
                let expected = TypeSchema::new(Types::String).serialize();

                assert_eq!(uuid, expected);
            }
        },
        "bigdecimal" => {
            exportable! {
                typeschema: {
                    bigdecimal::BigDecimal => String,
                }
            }

            #[test]
            fn test_serialize_decimal() {
                let decimal = bigdecimal::BigDecimal::export().serialize();
                let expected = TypeSchema::new(Types::String).serialize();

                assert_eq!(decimal, expected);
            }
        },
        "num-bigint" => {
            exportable! {
                typeschema: {
                    num_bigint::BigInt => String,
                }
            }

            #[test]
            fn test_serialize_bigint() {
                let bigint = num_bigint::BigInt::export().serialize();
                let expected = TypeSchema::new(Types::String).serialize();

                assert_eq!(bigint, expected);
            }
        },
        "num-bigfloat" => {
            exportable! {
                typeschema: {
                    num_bigfloat::BigFloat => String,
                }
            }

            #[test]
            fn test_serialize_bigfloat() {
                let bigfloat = num_bigfloat::BigFloat::export().serialize();
                let expected = TypeSchema::new(Types::String).serialize();

                assert_eq!(bigfloat, expected);
            }
        },
        "rust_decimal" => {
            exportable! {
                typeschema: {
                    rust_decimal::Decimal => String,
                }
            }

            #[test]
            fn test_serialize_rust_decimal() {
                let decimal = rust_decimal::Decimal::export().serialize();
                let expected = TypeSchema::new(Types::String).serialize();

                assert_eq!(decimal, expected);
            }
        }
    }
}
