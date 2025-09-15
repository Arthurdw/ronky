use ronky::{Exported, ExportedDeserialize, ExportedSerialize};
use serde::{Deserialize, Serialize};

#[test]
fn test_chrono_serialization() {
    use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct ChronoStruct {
        utc_time: DateTime<Utc>,
        local_time: DateTime<Local>,
        fixed_offset_time: DateTime<FixedOffset>,
        naive_date: NaiveDate,
        naive_time: NaiveTime,
        naive_datetime: NaiveDateTime,
        duration: chrono::Duration,
    }

    let _fixed_offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let test_data = ChronoStruct {
        utc_time: DateTime::parse_from_rfc3339("2023-12-25T10:30:00Z")
            .unwrap()
            .with_timezone(&Utc),
        local_time: DateTime::parse_from_rfc3339("2023-12-25T10:30:00Z")
            .unwrap()
            .with_timezone(&Local),
        fixed_offset_time: DateTime::parse_from_rfc3339("2023-12-25T10:30:00+05:00").unwrap(),
        naive_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
        naive_time: NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        naive_datetime: NaiveDateTime::parse_from_str("2023-12-25 10:30:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        duration: chrono::Duration::hours(2),
    };

    let json = test_data.to_json().unwrap();
    let deserialized: ChronoStruct = ChronoStruct::from_json(&json).unwrap();

    assert_eq!(deserialized.utc_time, test_data.utc_time);
    assert_eq!(deserialized.naive_date, test_data.naive_date);
    assert_eq!(deserialized.naive_time, test_data.naive_time);
    assert_eq!(deserialized.naive_datetime, test_data.naive_datetime);
    assert_eq!(deserialized.duration, test_data.duration);
}

#[test]
fn test_time_serialization() {
    use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time};

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct TimeStruct {
        offset_datetime: OffsetDateTime,
        primitive_datetime: PrimitiveDateTime,
        date: Date,
        time: Time,
        duration: Duration,
    }

    let test_data = TimeStruct {
        offset_datetime: OffsetDateTime::parse(
            "2023-12-25T10:30:00Z",
            &time::format_description::well_known::Iso8601::DEFAULT,
        )
        .unwrap(),
        primitive_datetime: PrimitiveDateTime::parse(
            "2023-12-25T10:30:00",
            &time::format_description::well_known::Iso8601::DEFAULT,
        )
        .unwrap(),
        date: Date::from_ordinal_date(2023, 359).unwrap(),
        time: Time::from_hms(10, 30, 0).unwrap(),
        duration: Duration::hours(2),
    };

    let json = test_data.to_json().unwrap();
    let deserialized: TimeStruct = TimeStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_uuid_serialization() {
    use uuid::Uuid;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct UuidStruct {
        id: Uuid,
        optional_id: Option<Uuid>,
        ids: Vec<Uuid>,
    }

    let test_data = UuidStruct {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        optional_id: Some(Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap()),
        ids: vec![
            Uuid::parse_str("6ba7b811-9dad-11d1-80b4-00c04fd430c8").unwrap(),
            Uuid::parse_str("6ba7b812-9dad-11d1-80b4-00c04fd430c8").unwrap(),
        ],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: UuidStruct = UuidStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_bigdecimal_serialization() {
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct BigDecimalStruct {
        price: BigDecimal,
        optional_price: Option<BigDecimal>,
        prices: Vec<BigDecimal>,
    }

    let test_data = BigDecimalStruct {
        price: BigDecimal::from_str("123.456789").unwrap(),
        optional_price: Some(BigDecimal::from_str("999.999").unwrap()),
        prices: vec![
            BigDecimal::from_str("1.23").unwrap(),
            BigDecimal::from_str("4.56").unwrap(),
        ],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: BigDecimalStruct = BigDecimalStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_num_bigint_serialization() {
    use num_bigint::BigInt;
    use std::str::FromStr;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct BigIntStruct {
        large_number: BigInt,
        optional_number: Option<BigInt>,
        numbers: Vec<BigInt>,
    }

    let test_data = BigIntStruct {
        large_number: BigInt::from_str("123456789012345678901234567890").unwrap(),
        optional_number: Some(BigInt::from_str("987654321").unwrap()),
        numbers: vec![
            BigInt::from_str("111").unwrap(),
            BigInt::from_str("222").unwrap(),
        ],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: BigIntStruct = BigIntStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_num_bigfloat_serialization() {
    use num_bigfloat::BigFloat;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct BigFloatStruct {
        precise_number: BigFloat,
        optional_number: Option<BigFloat>,
        numbers: Vec<BigFloat>,
    }

    let test_data = BigFloatStruct {
        precise_number: BigFloat::from_f64(123.456789),
        optional_number: Some(BigFloat::from_f64(999.999)),
        numbers: vec![BigFloat::from_f64(1.23), BigFloat::from_f64(4.56)],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: BigFloatStruct = BigFloatStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_rust_decimal_serialization() {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct RustDecimalStruct {
        price: Decimal,
        optional_price: Option<Decimal>,
        prices: Vec<Decimal>,
    }

    let test_data = RustDecimalStruct {
        price: Decimal::from_str("123.456789").unwrap(),
        optional_price: Some(Decimal::from_str("999.999").unwrap()),
        prices: vec![
            Decimal::from_str("1.23").unwrap(),
            Decimal::from_str("4.56").unwrap(),
        ],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: RustDecimalStruct = RustDecimalStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_decimal_serialization() {
    use decimal::d128;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct DecimalStruct {
        value: d128,
        optional_value: Option<d128>,
        values: Vec<d128>,
    }

    let test_data = DecimalStruct {
        value: d128!(123.456),
        optional_value: Some(d128!(999.999)),
        values: vec![d128!(1.23), d128!(4.56)],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: DecimalStruct = DecimalStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_url_serialization() {
    use url::Url;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct UrlStruct {
        website: Url,
        optional_url: Option<Url>,
        urls: Vec<Url>,
    }

    let test_data = UrlStruct {
        website: Url::parse("https://example.com/path?query=value").unwrap(),
        optional_url: Some(Url::parse("http://localhost:3000").unwrap()),
        urls: vec![
            Url::parse("https://github.com").unwrap(),
            Url::parse("https://crates.io").unwrap(),
        ],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: UrlStruct = UrlStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_bytes_serialization() {
    use bytes::{Bytes, BytesMut};

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct BytesStruct {
        data: Bytes,
        mutable_data: BytesMut,
        optional_data: Option<Bytes>,
        data_list: Vec<Bytes>,
    }

    let test_data = BytesStruct {
        data: Bytes::from("hello world"),
        mutable_data: BytesMut::from("mutable data"),
        optional_data: Some(Bytes::from("optional")),
        data_list: vec![Bytes::from("item1"), Bytes::from("item2")],
    };

    let json = test_data.to_json().unwrap();
    let deserialized: BytesStruct = BytesStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_dashmap_serialization() {
    use dashmap::{DashMap, DashSet};

    #[derive(Exported, Serialize, Deserialize, Debug)]
    struct DashMapStruct {
        map_data: DashMap<String, i32>,
        set_data: DashSet<String>,
    }

    let test_data = DashMapStruct {
        map_data: {
            let map = DashMap::new();
            map.insert("key1".to_string(), 42);
            map.insert("key2".to_string(), 84);
            map
        },
        set_data: {
            let set = DashSet::new();
            set.insert("item1".to_string());
            set.insert("item2".to_string());
            set
        },
    };

    let json = test_data.to_json().unwrap();
    let deserialized: DashMapStruct = DashMapStruct::from_json(&json).unwrap();

    // DashMap doesn't guarantee order, so we check individual elements
    assert_eq!(deserialized.map_data.len(), test_data.map_data.len());
    assert_eq!(deserialized.map_data.get("key1").map(|v| *v), Some(42));
    assert_eq!(deserialized.map_data.get("key2").map(|v| *v), Some(84));

    assert_eq!(deserialized.set_data.len(), test_data.set_data.len());
    assert!(deserialized.set_data.contains("item1"));
    assert!(deserialized.set_data.contains("item2"));
}

#[test]
fn test_indexmap_serialization() {
    use indexmap::{IndexMap, IndexSet};

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct IndexMapStruct {
        map_data: IndexMap<String, i32>,
        set_data: IndexSet<String>,
    }

    let mut map_data = IndexMap::new();
    map_data.insert("key1".to_string(), 42);
    map_data.insert("key2".to_string(), 84);

    let mut set_data = IndexSet::new();
    set_data.insert("item1".to_string());
    set_data.insert("item2".to_string());

    let test_data = IndexMapStruct { map_data, set_data };

    let json = test_data.to_json().unwrap();
    let deserialized: IndexMapStruct = IndexMapStruct::from_json(&json).unwrap();

    assert_eq!(deserialized, test_data);
}

#[test]
fn test_smallvec_serialization() {
    use smallvec::{SmallVec, smallvec};

    // Note: SmallVec arrays need to implement Exportable, but for serialization testing
    // we can test the functionality by just ensuring it works with serde
    let small_numbers: SmallVec<[i32; 4]> = smallvec![1, 2, 3, 4, 5];
    let json = serde_json::to_string(&small_numbers).unwrap();
    let deserialized: SmallVec<[i32; 4]> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, small_numbers);

    let small_strings: SmallVec<[String; 2]> = smallvec!["hello".to_string(), "world".to_string()];
    let json = serde_json::to_string(&small_strings).unwrap();
    let deserialized: SmallVec<[String; 2]> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, small_strings);
}

#[test]
fn test_combined_partnered_libs() {
    // Test multiple libraries together when available
    {
        use chrono::{DateTime, Utc};
        use url::Url;
        use uuid::Uuid;

        #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
        struct CombinedStruct {
            id: Uuid,
            created_at: DateTime<Utc>,
            website: Url,
        }

        let test_data = CombinedStruct {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            created_at: DateTime::parse_from_rfc3339("2023-12-25T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            website: Url::parse("https://example.com").unwrap(),
        };

        let json = test_data.to_json().unwrap();
        let deserialized: CombinedStruct = CombinedStruct::from_json(&json).unwrap();

        assert_eq!(deserialized, test_data);
    }
}
