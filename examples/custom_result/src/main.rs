use ronky::{Exportable, Exported, SCHEMA_VERSION};
use serde_json::{Value, from_str, to_string_pretty};

#[allow(dead_code)]
#[derive(Exported)]
#[arri(transform = "uppercase")]
enum Result<T: Exportable, E: Exportable> {
    /// Heck, you can even add a special note here!
    Ok(T),
    Err(E),
}

fn main() {
    println!(
        "Serializing struct to a Arri {} JSON schema.",
        SCHEMA_VERSION
    );

    let serialized = Result::<String, ()>::export()
        .serialize()
        .expect("there to be something");
    let formatted = to_string_pretty(&from_str::<Value>(&serialized).unwrap()).unwrap();
    println!("{}", formatted);
}
