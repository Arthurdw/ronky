use ronky::Exported;

#[derive(Exported)]
#[arri(tag_key = "kind", value_key = "kind")]
enum Shape {
    Circle { radius: f64 },
}

fn main() {}
