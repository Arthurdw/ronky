//! # 🐱 RONKY 🐱
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/ronky)](https://crates.io/crates/ronky)
//! [![Purrs Per Minute](https://img.shields.io/badge/purrs-over%209000-orange)](https://github.com/modiimedia/arri)
//! [![Cat Approved](https://img.shields.io/badge/cat-approved-brightgreen)](https://github.com/modiimedia/arri)
//! [![Rustacean Friendly](https://img.shields.io/badge/rustacean-friendly-blue)](https://github.com/modiimedia/arri)
//!
//! _"Converting Rust types shouldn't be this purr-fect, but here we are..."_
//!
//! ## 😺 What in the Whiskers is Ronky?
//!
//! Imagine if your Rust types could speak other languages without learning a single foreign word.
//! That's Ronky – your code's personal polyglot translator that speaks fluent
//! [Arri](https://github.com/modiimedia/arri), turning your carefully crafted Rust types into
//! schemas that even JavaScript developers can understand.
//!
//! Born from the frustration of manual schema creation (and named after a particularly vocal cat),
//! Ronky does the tedious work so you can focus on the important stuff – like deciding whether your
//! next variable should be called `data` or `info` (we both know you'll pick `data`).
//!
//! > 🚧 **Paws at Work**: Like a cat that's not quite finished knocking everything off your desk,
//! > Ronky is still under construction. Object serialization and deserialization are coming soon,
//! > probably right after this catnap. 🚧
//!
//! ## ✨ Features That Make You Go "Meow!"
//!
//! Ronky doesn't just toss your types over the fence to Arri-land. It crafts them with the same
//! attention to detail that a cat gives to knocking your most precious possessions off shelves:
//!
//! ```text
//! ┌───────────────────────────────────────────────────────────────────────┐
//! │                          RONKY'S REPERTOIRE                           │
//! ├───────────────────────────┬───────────────────────────────────────────┤
//! │ 🧬 Type Wizardry          │ - Transforms primitives with grace        │
//! │                           │ - Handles generic types without whining   │
//! │                           │ - Makes associated types feel welcome     │
//! ├───────────────────────────┼───────────────────────────────────────────┤
//! │ 🧩 Collection Conjuring   │ - Vectors become elegant "elements"       │
//! │                           │ - Maps manifest as "values" schemas       │
//! │                           │ - Optional types know when to disappear   │
//! ├───────────────────────────┼───────────────────────────────────────────┤
//! │ 🛡️ Guardian Features      │ - Strict mode keeps schemas pristine      │
//! │                           │ - Discriminators tag unions properly      │
//! │                           │ - Circular refs handled without dizziness │
//! ├───────────────────────────┼───────────────────────────────────────────┤
//! │ 🔄 Transformation Magic   │ - Case transformations (snake → UPPER)    │
//! │                           │ - Field renaming for multilingual joy     │
//! │                           │ - Nullable marking for optional presence  │
//! ├───────────────────────────┼───────────────────────────────────────────┤
//! │ 📝 Documentation Delight  │ - Comments become documentation           │
//! │                           │ - Deprecation warnings that don't nag     │
//! │                           │ - Metadata that brings joy to readers     │
//! └───────────────────────────┴───────────────────────────────────────────┘
//! ```
//!
//! With Ronky, your type schema generation is both:
//! 1. **Compile-time verified** - Errors at compile time, not at 3 AM when you're deploying
//! 2. **Automatically generated** - Because life's too short to manually update schemas
//!
//! > 💡 **Pro Tip**: Ronky's powers grow with your documentation. The more doc comments you add,
//! > the more magnificent your schemas become. It's like feeding treats to a cat – the rewards
//! > are well worth it.
//!
//! ## 🤝 The Cool Cats Club (Compatible Crates)
//!
//! Ronky has an extensive social network. Think of these crates as the neighborhood cats that
//! regularly visit your backyard – they're all welcome and get special treatment:
//!
//! ```text
//! TEMPORAL FRIENDS         🕰️  chrono, time
//! IDENTITY SPECIALISTS     🪪  uuid
//! BIG NUMBER EXPERTS       🔢  bigdecimal, num-bigint, num-bigfloat
//! PRECISION MASTERS        💰  rust_decimal, decimal
//! WEB-SAVVY NAVIGATORS     🌐  url
//! DATA-HANDLING WIZARDS    📊  bytes
//! CONCURRENT COMPANIONS    🧵  dashmap
//! ORDERLY ORGANIZERS       📋  indexmap
//! OPTIMIZED PERFORMERS     ⚡  smallvec
//! ```
//!
//! Each of these crates gets the VIP (Very Important Purring) treatment from Ronky. Their types
//! are handled with the care and respect they deserve.
//!
//! > 🐈 **Missing your favorite companion?** Open an issue to suggest a new addition to Ronky's
//! > compatible crates collection. The more the merrier!
//!
//! ## 👨‍💻 The Story Behind Ronky
//!
//! Let me share the real origin story – Ronky wasn't born from late-night caffeine-fueled frustration
//! (though there's been plenty of that in my coding career). It all started when a friend dropped a
//! Discord message with a link to the [Arri project](https://github.com/modiimedia/arri). One look at
//! what Arri was doing with type conversions and I was hooked.
//!
//! As a passionate Rustacean, I immediately thought: "This is brilliant! But it needs Rust support."
//! And thus Ronky was born – creating a bridge between the elegant type system of Rust and the universal
//! schema language of Arri.
//!
//! I named it after my cat not just because he was adorable (though he absolutely was), but because
//! he embodied what good libraries should be – helpful, reliable, and occasionally surprising you
//! with something delightful. Like Ronky the cat finding innovative ways to get treats without
//! moving, Ronky the library finds ways to update your API schemas without you lifting a finger.
//!
//! My hope is that this library saves you time, prevents those pesky "the API contract changed but
//! the docs didn't" bugs, and maybe – just maybe – frees up enough of your day to spend time with
//! your own four-legged friend (or fish, or rubber duck - whatever brings you joy).
//!
//! ## 📚 The Illustrated Guide to Ronky
//!
//! ### 🔄 The Basic Transformation
//!
//! ```rust
//! use ronky::{Exportable, Exported, SCHEMA_VERSION};
//! use serde_json::{Value, from_str, to_string_pretty};
//!
//! // Just add water (and a derive macro)
//! #[derive(Exported)]
//! #[arri(transform = "uppercase")] // LOUD NOISES
//! enum Result<T: Exportable, E: Exportable> {
//!     /// When things go right (rarely, if you're me)
//!     Ok(T),
//!     /// When things go wrong (my default state)
//!     Err(E),
//! }
//!
//!
//! // Announce our intentions to the world
//! println!("🧪 Creating an Arri {} schema and hoping for the best...", SCHEMA_VERSION);
//!
//! // The cat-alchemy happens here
//! let schema_json = Result::<String, ()>::export()
//!     .serialize()
//!     .expect("this to work (please, I have deadlines)");
//!
//! // Humans like pretty things
//! let pretty_json = to_string_pretty(&from_str::<Value>(&schema_json).unwrap()).unwrap();
//!
//! // Admire our handiwork
//! println!("{}", pretty_json);
//!
//! // Now go make a cup of tea, you've earned it
//! ```
//!
//! ### 🧩 The Advanced Cat-egory: Building Complex Types
//!
//! ```rust
//! use ronky::{Exportable, Exported, SCHEMA_VERSION};
//!
//! /// Metadata about things (and sometimes other things)
//! #[derive(Exported)]
//! struct About<T: Exportable> {
//!     /// What we called it before marketing got involved
//!     #[deprecated(since = "1.0.0", note = "Use `firstName` and `lastName` instead")]
//!     name: String,
//!     
//!     /// The name that appears on your coffee cup at Starbucks
//!     first_name: String,
//!     
//!     /// The name your parents use when you're in trouble
//!     last_name: Option<String>,
//!     
//!     /// The number that makes you sigh at birthday parties
//!     age: u32,
//!     
//!     /// The subject of our obsession
//!     of: T,
//! }
//!
//! /// A creature that creates Rust crates, ironically
//! #[derive(Exported)]
//! #[arri(strict)] // No surprises allowed! Like a cat with a cucumber
//! struct Human {
//!     /// Fellow code-monkeys who review your PRs
//!     friends: Vec<Human>, // Recursive types? No problem!
//!     
//!     /// The real owners of your home
//!     pets: Vec<About<Pet>>,
//! }
//!
//! /// Fashion choices for the discerning feline
//! #[derive(Exported)]
//! #[arri(transform = ["snake_case", "uppercase"])] // MULTI_STYLE_TRANSFORMATION
//! enum CatColor {
//!     /// Like my coffee and my humor
//!     Black,
//!     
//!     /// Like my documentation standards and error handling
//!     White,
//!     
//!     /// Like my moral compass when it comes to optimization
//!     Gray,
//!     
//!     /// Like my commit history after a weekend hackathon
//!     MixedGrayWhite,
//! }
//!
//! /// Entities that interrupt your Zoom calls at the worst possible moment
//! #[derive(Exported)]
//! #[arri(transform = "uppercase", discriminator = "species")]
//! enum Pet {
//!     Dog {
//!         /// The word you'll repeat 37 times at the dog park
//!         name: String,
//!         
//!         /// What you'll forget when the vet asks
//!         #[arri(nullable)]
//!         breed: Option<String>,
//!     },
//!     
//!     #[arri(rename = "cat")] // All hail the cat overlords!
//!     Lion {
//!         /// A suggestion they might consider responding to someday
//!         name: String,
//!         
//!         /// Their royal garment
//!         #[arri(nullable)]
//!         color: Option<CatColor>,
//!     },
//! }
//! ```
//!
//! > 🔥 **Hot Tip**: These examples aren't just decorative – they're functional!
//! > Copy, paste, and experience the magic of Ronky firsthand. Your future self
//! > will thank you when your API documentation is automatically up-to-date.
//!
//! ## 🐈 The Ronky Memorial Section
//!
//! ```text
//!      /\_/\  
//!     ( o.o )
//!      > ^ <
//!     /  O  \  "Meow meow, transform types meow."
//!                                     - Ronky (2010-2024)
//! ```
//!
//! This library immortalizes a magnificent cat named Ronky, whose thunderous purrs
//! (or "ronks" in Dutch) could wake the neighbors. For 14 remarkable years, this
//! whiskered genius supervised everything that happened in the house.
//!
//! Despite battling acromegaly, Ronky maintained a proud dignity and an uncanny ability
//! to walk across keyboards. His legacy continues in this library!
//!
//! Ronky taught me important programming principles:
//!
//! - **Persistence**: If at first you don't succeed, meow louder until someone fixes it for you
//! - **Efficiency**: Why do something yourself when you can delegate?
//! - **Work-Life Balance**: No bug is so important that it can't wait until after a nap
//! - **Code Reviews**: Sometimes the best feedback is just silently judging from a distance
//!
//! This library aims to embody these principles by automating the tedious parts of
//! API development so you can focus on the parts that matter – like figuring out why
//! your application works in development but not production (it's always CORS).
//!
//! ## 📋 Quick Reference
//!
//! ### The Basics
//!
//! 1. Add `ronky` to your `Cargo.toml`:
//!    ```toml
//!    [dependencies]
//!    ronky = "1.0.0"  # Check crates.io for the latest version
//!    ```
//!
//! 2. Import the essentials:
//!    ```rust
//!    use ronky::{Exported, SCHEMA_VERSION};
//!    ```
//!
//! 3. Decorate your types:
//!    ```rust
//!    #[derive(Exported)]
//!    struct MyType { /* fields */ }
//!    ```
//!
//! 4. Export and serialize:
//!    ```rust
//!    let schema = MyType::export().serialize().unwrap();
//!    ```
//!
//! 5. Profit! (This step is not automated by Ronky, sorry)
//!
//! ### Attribute Options
//!
//! - `#[arri(strict)]` - No extra properties allowed
//! - `#[arri(transform = "snake_case")]` - Transform enum variant names
//! - `#[arri(discriminator = "type")]` - Set discriminator field name
//! - `#[arri(rename = "newName")]` - Rename a field or variant
//! - `#[arri(nullable)]` - Mark a field as nullable
//!
//! ## 🌟 Final Thought
//!
//! Remember: Type conversion should be like a cat's nap – automatic, elegant, and requiring
//! no effort on your part. Let Ronky handle the tedious work while you focus on building
//! something amazing.
//!
//! Now go pet your cat (or dog, or rubber duck) – they've been waiting patiently while you
//! read this documentation. ❤️

#[cfg(feature = "derive")]
extern crate ronky_derive;

#[cfg(feature = "derive")]
pub use ronky_derive::Exported;

extern crate arri_repr;
pub use arri_repr::*;

pub static SCHEMA_VERSION: &str = "v0.0.8";

// TODO: implement conversion from ATD to Rust types
// | ATD Type | Rust Type |
// |---|---|
// | string | String |
// | boolean | bool |
// | timestamp | DateTime |
// | float32 | f32 |
// | float64 | f64 |
// | int8 | i8 |
// | uint8 | u8 |
// | int16 | i16 |
// | uint16 | u16 |
// | int32 | i32 |
// | uint32 | u32 |
// | int64 | i64 |
// | uint64 | u64 |
