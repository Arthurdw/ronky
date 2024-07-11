use exported::Exported;
use proc::Export;
use serde_json::json;

#[allow(dead_code)]
#[derive(Export)]
struct Human {
    name: String,
    age: u32,
    friends: Vec<Human>,
    pets: Vec<Pet>,
}

#[allow(dead_code)]
#[derive(Export)]
struct Pet {
    name: String,
    species: String,
}

pub fn main() {
    // todo!("make this properly exported and integrated with extensions")
    let human = Human::export();
    let pet = Pet::export();

    let types = json!({
        "types": [human, pet]
    });

    println!("{:#}", types);
}
