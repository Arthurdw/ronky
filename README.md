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
- [x] Values schema form (for maps)
- [x] See which Arri schema version is being used

### Popular compatible crates

Missing a popular crate? Check if a issue for this exists, and if not create one!

- [x] `chrono`
- [x] `time`
- [x] `uuid`
- [x] `bigdecimal`
- [x] `num-bigint`
- [x] `num-bigfloat`
- [x] `rust_decimal`
- [x] `decimal`
- [x] `url`
- [x] `bytes`
- [x] `dashmap`
- [x] `indexmap`
- [x] `smallvec`

## Feature(s) that will never be implemented

- Empty schema form, this is something that should never be used in Rust code
  anyway and is greatly a sign of bad code and a common source for bugs.

## Example Usage

You can find examples in the `./examples/` directory.

## In memory of Ronky

In loving memory of my dear cat Ronky, named for his unique habit of spinning
very loud _(to "ronk" in Dutch)_. Ronky lived to the age of 14 and bravely
endured acromegaly.

He passed away peacefully, surrounded by those who loved him. He will be deeply missed.

![A beautiful picture of Ronky](./.readme/assets/ronky.jpg)

Photo by [Startshot](https://www.instagram.com/_startshot_/)
