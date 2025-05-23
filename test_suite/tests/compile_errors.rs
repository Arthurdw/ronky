use std::fs;

#[test]
fn compile_fail() {
    let failures = fs::read_dir("tests/compile_fail")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(String::from)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let t = trybuild::TestCases::new();
    for test in failures {
        t.compile_fail(format!("tests/compile_fail/{}.rs", test));
    }
}
