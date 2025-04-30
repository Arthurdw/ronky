/// Provides functionality for exporting types into Arri schemas.
///
/// This module defines the `Exportable` trait, which allows types to be
/// converted into serializable schemas. It also includes macros and utilities
/// for handling generic types, type schemas, and feature-specific schemas.
use crate::{
    EmptySchema, PropertiesSchema, RefSchema, TaggedUnionSchema, ValuesSchema, type_utils,
};
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
    /// Tracks types currently being exported to prevent infinite recursion.
    static RECURSION_TRACKER: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// A trait for types that can be exported into Arri schemas.
pub trait Exportable {
    /// Retrieves the type name of the implementing type.
    fn get_type_name() -> String {
        type_utils::get_type_name::<Self>()
    }

    /// Exports the type into a serializable schema.
    ///
    /// This method ensures that recursive types are handled correctly.
    fn export() -> Box<dyn Serializable> {
        Self::export_with_recursion_check()
    }

    /// Internal method for exporting the type.
    ///
    /// This method should be implemented by types to define their specific
    /// export logic.
    fn export_internal() -> impl Serializable;

    /// Exports the type with recursion tracking to prevent infinite loops.
    fn export_with_recursion_check() -> Box<dyn Serializable> {
        let type_name = Self::get_type_name();

        let is_recursive = RECURSION_TRACKER.with(|tracker| {
            let mut tracker = tracker.borrow_mut();
            let is_recursive = tracker.contains(&type_name);

            if !is_recursive {
                // Add the type to the recursion tracker.
                tracker.insert(type_name.clone());
            }

            is_recursive
        });

        if is_recursive {
            return Box::new(RefSchema::new(type_utils::get_type_name_from(type_name)));
        }

        let result = Self::export_internal();

        // Remove the type from the recursion tracker after exporting.
        RECURSION_TRACKER.with(|tracker| {
            let mut tracker = tracker.borrow_mut();
            tracker.remove(&type_name);
        });

        Box::new(result)
    }
}

/// A macro for defining exportable types and schemas.
///
/// This macro provides a convenient way to define type schemas, generic
/// implementations, and feature-specific schemas for types.
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

    (@parse_typeschema static $ty:ty => $to:expr, $($rest:tt)*) => {
        exportable!(@parse_typeschema $ty => { $to }, $($rest)*);
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

/// A type alias for slices of a given type.
///
/// This alias is used to simplify the interaction with slices in the macro.
type SliceOf<T> = [T];

exportable! {
    typeschema: {
        static () => EmptySchema::new(),
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
        Result<T, E> => {
            let mut schema = TaggedUnionSchema::new();
            let mut ok_props = PropertiesSchema::new();
            let mut err_props = PropertiesSchema::new();

            ok_props.set_property("value", Box::new(T::export()));
            err_props.set_property("value", Box::new(E::export()));

            schema.add_mapping("Ok", Box::new(ok_props));
            schema.add_mapping("Err", Box::new(err_props));
            schema
        },

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
        },
        "time" => {
            exportable! {
                typeschema: {
                    time::OffsetDateTime => Timestamp,
                    time::PrimitiveDateTime => Timestamp,
                    time::Date => Timestamp,
                    time::Time => Timestamp,
                    time::Duration => Int64,
                }
            }
        },
        "uuid" => {
            exportable! {
                typeschema: {
                    uuid::Uuid => String,
                }
            }
        },
        "bigdecimal" => {
            exportable! {
                typeschema: {
                    bigdecimal::BigDecimal => String,
                }
            }
        },
        "num-bigint" => {
            exportable! {
                typeschema: {
                    num_bigint::BigInt => String,
                }
            }
        },
        "num-bigfloat" => {
            exportable! {
                typeschema: {
                    num_bigfloat::BigFloat => String,
                }
            }
        },
        "rust_decimal" => {
            exportable! {
                typeschema: {
                    rust_decimal::Decimal => String,
                }
            }
        },
        "decimal" => {
            exportable! {
                typeschema: {
                    decimal::d128 => String,
                }
            }
        },
        "url" => {
            exportable! {
                typeschema: {
                    url::Url => String,
                }
            }
        },
        "bytes" => {
            exportable! {
                typeschema: {
                    bytes::Bytes => String,
                    bytes::BytesMut => String,
                }
            }
        },
        "dashmap" => {
            use dashmap::{DashMap, DashSet};

            exportable! {
                generic: {
                    DashMap<K: ToString, V> => ValuesSchema::new(Box::new(V::export())),
                    DashSet<T> => ElementsSchema::new(Box::new(T::export())),
                }
            }
        },
        "indexmap" => {
            use indexmap::{IndexMap, IndexSet};

            exportable! {
                generic: {
                    IndexMap<K: ToString, V> => ValuesSchema::new(Box::new(V::export())),
                    IndexSet<T> => ElementsSchema::new(Box::new(T::export())),
                }
            }
        },
        "smallvec" => {
            use smallvec::SmallVec;

            exportable! {
                generic: {
                    SmallVec<T: smallvec::Array> => ElementsSchema::new(Box::new(T::export())),
                }
            }
        }
    }
}
