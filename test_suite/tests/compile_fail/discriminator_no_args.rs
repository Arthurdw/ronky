use ronky::Exported;

#[derive(Exported)]
#[arri(discriminator)]
enum WellWhatToUse {
    IDontKnow { doYou: Option<bool> },
}

fn main() {}
