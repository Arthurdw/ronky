use ronky::Exported;

#[derive(Exported)]
struct Test {
    #[arri(rename)]
    example: u8,
}

fn main() {}
