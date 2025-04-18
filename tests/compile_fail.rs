/// Ensure that inappropriate types don't implement the two message traits.
/// Not testing the main message trait, because all messages should implement that.
#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/**/*.rs");
}
