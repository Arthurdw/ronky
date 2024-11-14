# Ronky - A simple way to export Rust definitions to other languages

[![Crates.io Version](https://img.shields.io/crates/v/ronky)](https://crates.io/crates/ronky)

## Example

There is still a lot of work to be done, but here is a simple example of what I
have in mind.

The following code and it's output:

```rs
#[derive(Export)]
struct Human {
    name: String,
    age: u32,
    friends: Vec<Human>,
    pets: Vec<Pet>,
}

#[derive(Export)]
struct Pet {
    name: String,
    species: String,
}
```

Which results in the following JSON:

```json
{
  "types": [
    {
      "name": "Human",
      "fields": [
        {
          "name": "name",
          "type": "String"
        },
        {
          "name": "age",
          "type": "u32"
        },
        {
          "name": "friends",
          "type": "list"
          "of": ["Human"],
        },
        {
          "name": "pets",
          "type": "list"
          "of": ["Pet"],
        }
      ]
    },
    {
      "name": "Pet",
      "fields": [
        {
          "name": "name",
          "type": "String"
        },
        {
          "name": "species",
          "type": "String"
        }
      ]
    }
  ]
}
```

That can then be converted to the following typescript (any supported language,
or create your own):

```ts
interface Human {
  name: string;
  age: number;
  friends: Human[];
  pets: Pet[];
}

interface Pet {
  name: string;
  species: string;
}
```

## In memory of Ronky

In loving memory of my dear cat Ronky, named for his unique habit of spinning
with a loud sound _(to "ronk" in Dutch)_. Ronky lived to the age of 14 and bravely
endured acromegaly. This condition resulted in the abnormal growth of his tissues
and bones.

He passed away peacefully, surrounded by those who loved him. He will be deeply missed.

![Beautiful picture of Ronky](./.readme/assets/ronky.jpg)

Photo by [Startshot](https://www.instagram.com/_startshot_/)
