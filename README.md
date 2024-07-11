# Ronky - A simple way to export Rust definitions to other languages

## Example

There is still alot of work to be done, but here is a simple example of what I have in mind.

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

That can then be converted to the following typescript (or any supported language):

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
