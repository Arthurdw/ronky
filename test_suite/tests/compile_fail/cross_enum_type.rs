use ronky::Exported;

#[derive(Exported)]
#[arri(discriminator = "type")]
enum Mixed {
    Normal,
    Tagged { name: String },
}

fn main() {}
