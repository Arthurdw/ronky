use ronky::Exported;

#[derive(Exported)]
#[arri(tag_key = "kind")]
enum Shape {
    Circle { radius: f64 },
    Label(String),
}

fn main() {}
