#[test]
fn main() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/ui/compile_fail_*.rs");
    test_cases.pass("tests/ui/pass_*.rs");
}
