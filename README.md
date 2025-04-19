# Ronky - A simple way to export Rust definitions

[![Crates.io Version](https://img.shields.io/crates/v/ronky)](https://crates.io/crates/ronky)

This library allows you to extract Rust types and serialize them into
[Arri](https://github.com/modiimedia/arri) types.

> 🚧 This is still under development, features such as object serialisation and
> de serialisation will be coming soon. 🚧

<!--toc:start-->

- [Ronky - A simple way to export Rust definitions](#ronky-a-simple-way-to-export-rust-definitions)
  - [Features (exporting only for now)](#features-exporting-only-for-now)
  - [Feature(s) that will never be implemented](#features-that-will-never-be-implemented)
  - [Example Usage](#example-usage)
  - [In memory of Ronky](#in-memory-of-ronky)
  <!--toc:end-->

## Features (exporting only for now)

- [x] Compile time errors and integration with Rust analyzer
- [x] Type schema form (and their associated types)
- [x] Enum schema form
- [x] Elements (vectors) schema form
- [x] Properties (structs) schema form (including optional properties)
- [x] Strict mode
- [x] Discriminator (tagged unions/enums with fields) schema form
- [x] Ref schema form (for circular references)
- [x] isNullable keyword
- [x] metadata keyword (includes: id, description, isDeprecated, deprecatedNote,
      deprecatedSince)
- [ ] Values schema form (for maps)
- [ ] See which Arri schema version is being used

## Feature(s) that will never be implemented

- Empty schema form, this is something that should never be used in Rust code
  anyway and is greatly a sign of bad code and a common source for bugs.

## Example Usage

This is how code could look with Ronky, as you can see there is nothing really a
lot that you have to add to make it work. Only the `Exported` derive should be
present. And if you want to use generics they should also be exportable (which
is implicitly done by the `Exportable` derive).

```rust
use ronky::{Exportable, Exported};

/// General representation of something
#[allow(dead_code)]
#[derive(Exported)]
struct About<T: Exportable> {
    /// The full name of the human
    #[deprecated(since = "1.0.0", note = "Use `firstName` and `lastName` instead")]
    name: String,

    /// The first name of the human.
    first_name: String,

    /// The last name of the human.
    last_name: String,

    /// The age of the human in years.
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
    pets: Vec<Pet>,
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
}
```

The output of this command would be this JSON (has been reformatted for
displaying purposes).

```json
{
  "properties": {
    "name": {
      "type": "string",
      "metadata": {
        "description": "The full name of the human",
        "isDeprecated": true,
        "deprecatedSince": "1.0.0",
        "deprecatedNote": "Use `firstName` and `lastName` instead"
      }
    },
    "first_name": {
      "type": "string",
      "metadata": { "description": "The first name of the human." }
    },
    "last_name": {
      "type": "string",
      "metadata": { "description": "The last name of the human." }
    },
    "age": {
      "type": "uint32",
      "metadata": { "description": "The age of the human in years." }
    },
    "of": {
      "properties": {
        "friends": {
          "elements": { "ref": "Human" },
          "metadata": {
            "description": "A list of the human's friends, represented as other
            `Human` instances."
          }
        },
        "pets": {
          "elements": {
            "discriminator": "species",
            "mapping": {
              "CAT": {
                "properties": {
                  "name": {
                    "type": "string",
                    "metadata": { "description": "The name of the pet." }
                  }
                },
                "optionalProperties": {
                  "color": {
                    "enum": ["BLACK", "WHITE", "GRAY", "MIXED_GRAY_WHITE"],
                    "metadata": { "description": "The color of the pet." },
                    "isNullable": true
                  }
                }
              },
              "DOG": {
                "properties": {
                  "name": {
                    "type": "string",
                    "metadata": { "description": "The name of the pet." }
                  }
                },
                "optionalProperties": {
                  "breed": {
                    "type": "string",
                    "metadata": { "description": "The breed of the pet." },
                    "isNullable": true
                  }
                }
              }
            },
            "metadata": {
              "id": "Pet",
              "description": "Represents a pet with a name and species."
            }
          },
          "metadata": { "description": "A list of the human's pets." }
        }
      },
      "optionalProperties": {},
      "metadata": { "description": "Of what this about is" }
    }
  },
  "optionalProperties": {},
  "metadata": {
    "id": "About",
    "description": "General representation of something"
  }
}
```

## In memory of Ronky

In loving memory of my dear cat Ronky, named for his unique habit of spinning
very loud _(to "ronk" in Dutch)_. Ronky lived to the age of 14 and bravely
endured acromegaly.

He passed away peacefully, surrounded by those who loved him. He will be deeply missed.

![A beautiful picture of Ronky](./.readme/assets/ronky.jpg)

Photo by [Startshot](https://www.instagram.com/_startshot_/)
