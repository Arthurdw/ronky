use ronky::{Exported, ExportedDeserialize, ExportedSerialize};
use serde::{Deserialize, Serialize};

#[test]
fn test_simple_struct_serialization() {
    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        active: bool,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        active: true,
    };

    // Test serialization
    let json = person.to_json().unwrap();
    let expected_json = r#"{"name":"Alice","age":30,"active":true}"#;
    assert_eq!(json, expected_json);

    // Test deserialization
    let deserialized: Person = Person::from_json(expected_json).unwrap();
    assert_eq!(deserialized, person);
}

#[test]
fn test_nested_struct_serialization() {
    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct Address {
        street: String,
        city: String,
        zip: String,
    }

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct Employee {
        id: u64,
        name: String,
        address: Address,
        salary: Option<f64>,
    }

    let employee = Employee {
        id: 12345,
        name: "Bob".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            zip: "12345".to_string(),
        },
        salary: Some(75000.50),
    };

    // Test serialization
    let json = employee.to_json().unwrap();
    assert!(json.contains(r#""id":12345"#));
    assert!(json.contains(r#""name":"Bob""#));
    assert!(json.contains(r#""street":"123 Main St""#));
    assert!(json.contains(r#""salary":75000.5"#));

    // Test deserialization
    let deserialized: Employee = Employee::from_json(&json).unwrap();
    assert_eq!(deserialized, employee);
}

#[test]
fn test_vector_and_hashmap_serialization() {
    use std::collections::HashMap;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct Data {
        items: Vec<String>,
        metadata: HashMap<String, i32>,
    }

    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), 1);
    metadata.insert("count".to_string(), 42);

    let data = Data {
        items: vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ],
        metadata,
    };

    // Test serialization
    let json = data.to_json().unwrap();
    assert!(json.contains(r#""items":["item1","item2","item3"]"#));
    assert!(json.contains(r#""version":1"#));
    assert!(json.contains(r#""count":42"#));

    // Test deserialization
    let deserialized: Data = Data::from_json(&json).unwrap();
    assert_eq!(deserialized, data);
}

#[test]
fn test_enum_serialization() {
    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    #[arri(discriminator = "type")]
    enum Status {
        Active { dummy: Option<()> },
        Inactive { reason: String },
        Pending { id: u32, message: Option<String> },
    }

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct Task {
        title: String,
        status: Status,
    }

    // Test unit-like variant with optional field
    let task1 = Task {
        title: "Task 1".to_string(),
        status: Status::Active { dummy: None },
    };
    let json1 = task1.to_json().unwrap();
    assert!(json1.contains(r#""Active""#));
    let deserialized1: Task = Task::from_json(&json1).unwrap();
    assert_eq!(deserialized1, task1);

    // Test struct variant with fields
    let task2 = Task {
        title: "Task 2".to_string(),
        status: Status::Inactive {
            reason: "Maintenance".to_string(),
        },
    };
    let json2 = task2.to_json().unwrap();
    assert!(json2.contains(r#""Inactive""#) && json2.contains(r#""Maintenance""#));
    let deserialized2: Task = Task::from_json(&json2).unwrap();
    assert_eq!(deserialized2, task2);

    // Test struct variant with optional field
    let task3 = Task {
        title: "Task 3".to_string(),
        status: Status::Pending {
            id: 999,
            message: Some("Waiting for approval".to_string()),
        },
    };
    let json3 = task3.to_json().unwrap();
    assert!(json3.contains(r#""id":999"#));
    assert!(json3.contains(r#""message":"Waiting for approval""#));
    let deserialized3: Task = Task::from_json(&json3).unwrap();
    assert_eq!(deserialized3, task3);
}

#[test]
fn test_generic_struct_serialization() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Container<T> {
        value: T,
        count: usize,
    }

    let container = Container {
        value: "Hello".to_string(),
        count: 5,
    };

    let json = container.to_json().unwrap();
    assert_eq!(json, r#"{"value":"Hello","count":5}"#);

    let deserialized: Container<String> = Container::from_json(&json).unwrap();
    assert_eq!(deserialized, container);

    // Test with different type
    let int_container = Container {
        value: 42i32,
        count: 1,
    };

    let int_json = int_container.to_json().unwrap();
    assert_eq!(int_json, r#"{"value":42,"count":1}"#);

    let int_deserialized: Container<i32> = Container::from_json(&int_json).unwrap();
    assert_eq!(int_deserialized, int_container);
}
