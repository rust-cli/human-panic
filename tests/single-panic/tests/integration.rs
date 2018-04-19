extern crate assert_cli;

#[test]
fn integration() {
  assert_cli::Assert::main_binary()
    .stderr()
    .contains("single-panic-test")
    .stderr()
    .contains("Human Panic Authors")
    .stderr()
    .contains("human-panic-crate@example.com")
    .fails_with(101)
    .unwrap();
}
