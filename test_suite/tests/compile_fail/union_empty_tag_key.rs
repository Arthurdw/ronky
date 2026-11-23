use ronky::Exported;

#[derive(Exported)]
#[arri(tag_key = "")]
enum Shape {
    Circle { radius: f64 },
}

fn main() {}
