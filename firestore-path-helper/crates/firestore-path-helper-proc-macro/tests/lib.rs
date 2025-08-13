#[test]
fn test() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/ui/fail_args_duplicate.rs");
    test_cases.compile_fail("tests/ui/fail_args_extra.rs");
    test_cases.compile_fail("tests/ui/fail_args_no_to_string.rs");
    test_cases.compile_fail("tests/ui/fail_format_empty.rs");
    test_cases.compile_fail("tests/ui/fail_format_invalid_collection_id.rs");
    test_cases.compile_fail("tests/ui/fail_format_invalid_document_id.rs");
    test_cases.compile_fail("tests/ui/fail_format_odd_segments.rs");
    test_cases.compile_fail("tests/ui/fail_format_unknown_document_id.rs");
    test_cases.pass("tests/ui/pass_custom_document_id.rs");
    test_cases.pass("tests/ui/pass_duplicate_document_id.rs");
    test_cases.pass("tests/ui/pass_no_args.rs");
    test_cases.pass("tests/ui/pass.rs");
}
