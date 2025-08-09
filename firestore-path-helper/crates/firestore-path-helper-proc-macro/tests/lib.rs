#[test]
fn test() {
    let test_cases = trybuild::TestCases::new();
    test_cases.pass("tests/ui/pass.rs");
}
