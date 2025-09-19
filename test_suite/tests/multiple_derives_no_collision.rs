// Test that multiple SerializableDerive uses in the same module don't cause symbol collisions
use ronky::{Serializable, SerializableDerive};

#[derive(SerializableDerive)]
struct FirstStruct {
    name: String,
}

#[derive(SerializableDerive)]
struct SecondStruct {
    id: u32,
}

#[derive(SerializableDerive)]
struct ThirdStruct {
    value: f64,
}

#[test]
fn test_multiple_derives_no_collision() {
    let first = FirstStruct {
        name: "test".to_string(),
    };
    let second = SecondStruct { id: 42 };
    let third = ThirdStruct { value: 2.71 };

    // All should serialize without symbol collisions
    assert!(first.serialize().is_some());
    assert!(second.serialize().is_some());
    assert!(third.serialize().is_some());
}
