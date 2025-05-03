use ronky::Exported;

#[derive(Exported)]
struct Test {
    #[arri(nullable)]
    example: u8,
}

fn main() {}
