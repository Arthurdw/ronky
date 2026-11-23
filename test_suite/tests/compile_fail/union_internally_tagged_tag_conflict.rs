use ronky::Exported;

#[derive(Exported)]
#[arri(tag_key = "kind")]
enum Shape {
    Circle {
        radius: f64,
        #[arri(rename = "kind")]
        circle_kind: String,
    },
}

fn main() {}
