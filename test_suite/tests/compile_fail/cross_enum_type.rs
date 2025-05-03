use ronky::Exported;

#[derive(Exported)]
enum Mixed {
    Normal,
    Tagged { name: String },
}

fn main() {}
