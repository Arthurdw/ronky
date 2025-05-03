use ronky::Exported;

#[derive(Exported)]
struct Test {
    #[arri(rename = "what-if-we-did-*-and-got-paid-$-for-it?")]
    example: u8,
}

fn main() {}
