use ronky::Exported;

#[derive(Exported)]
#[arri(discriminator = "type", tag_key = "kind")]
enum Shape {
    Circle { radius: f64 },
}

fn main() {}
