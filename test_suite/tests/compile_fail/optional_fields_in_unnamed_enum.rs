use ronky::Exported;

#[derive(Exported)]
#[arri(discriminator = "type")]
enum OptionalFields {
    ThisRequiesAValue(Option<String>),
}

fn main() {}
