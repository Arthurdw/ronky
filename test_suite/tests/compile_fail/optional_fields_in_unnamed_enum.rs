use ronky::Exported;

#[derive(Exported)]
enum OptionalFields {
    ThisRequiesAValue(Option<String>),
}

fn main() {}
