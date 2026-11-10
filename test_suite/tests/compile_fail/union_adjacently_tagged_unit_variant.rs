use ronky::Exported;

#[derive(Exported)]
#[arri(tag_key = "kind", value_key = "data")]
enum Shape {
    Circle { radius: f64 },
    Nothing,
}

fn main() {}
