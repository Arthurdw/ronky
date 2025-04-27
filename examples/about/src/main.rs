use ronky::{Exportable, Exported};

/// General representation of something
#[allow(dead_code)]
#[derive(Exported)]
struct About<T: Exportable> {
    /// The full name of the entity
    #[deprecated(since = "1.0.0", note = "Use `firstName` and `lastName` instead")]
    name: String,

    /// The first name of the entity.
    first_name: String,

    /// The last name of the entity.
    last_name: Option<String>,

    /// The age of the entity in years.
    age: u32,

    /// Of what this about is
    of: T,
}

/// Represents a human being with personal details and relationships.
#[allow(dead_code)]
#[derive(Exported)]
struct Human {
    /// A list of the human's friends, represented as other `Human` instances.
    friends: Vec<Human>,

    /// A list of the human's pets.
    pets: Vec<About<Pet>>,
}

/// Available cat colors
#[allow(dead_code)]
#[derive(Exported)]
#[arri(transform = ["snake_case", "uppercase"])]
enum CatColor {
    /// The color of the cat is black.
    Black,
    /// The color of the cat is white.
    White,
    /// The color of the cat is gray.
    Gray,
    /// The color of the cat is mixed gray and white.
    MixedGrayWhite,
}

/// Represents a pet with a name and species.
#[allow(dead_code)]
#[derive(Exported)]
#[arri(transform = "uppercase", discriminator = "species")]
enum Pet {
    Dog {
        /// The name of the pet.
        name: String,

        /// The breed of the pet.
        #[arri(nullable)]
        breed: Option<String>,
    },
    Cat {
        /// The name of the pet.
        name: String,

        /// The color of the pet.
        #[arri(nullable)]
        color: Option<CatColor>,
    },
}

fn main() {
    let serialized = About::<Human>::export().serialize().unwrap();
    println!("{}", serialized);
    // Format the serialized JSON string and write it to out.json
    let formatted = serde_json::to_string_pretty(
        &serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
    )
    .unwrap();
    std::fs::write("out.json", formatted).expect("Unable to write to file");
}
