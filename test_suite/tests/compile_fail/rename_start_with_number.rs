use ronky::Exported;

#[derive(Exported)]
struct Test {
    #[arri(rename = "123invalid")]
    example: u8,
}

fn main() {}
