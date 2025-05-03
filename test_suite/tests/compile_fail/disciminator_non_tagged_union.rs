use ronky::Exported;

#[derive(Exported)]
#[arri(discriminator = "species")]
enum DiscriminatorsCantBeUsedHere {
    WellOfCourseNot,
    ButLetsStillAddThisTest,
    SomethingLikeThis,
}

fn main() {}
