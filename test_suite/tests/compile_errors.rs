#[test]
fn compile_fail() {
    let failures = vec!["empty_arri_attribute", "cross_enum_type"];

    let t = trybuild::TestCases::new();
    for test in failures {
        t.compile_fail(format!("tests/fail/{}.rs", test));
    }
}
