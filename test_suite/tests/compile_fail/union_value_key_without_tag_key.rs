use ronky::Exported;

#[derive(Exported)]
#[arri(value_key = "data")]
enum Shape {
    Circle { radius: f64 },
}

fn main() {}
