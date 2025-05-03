use ronky::Exported;

#[derive(Exported)]
struct Test {
    #[arri(rename = "this should not contain spaces")]
    example: u8,
}

fn main() {}
