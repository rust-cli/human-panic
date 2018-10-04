extern crate assert_cli;

#[test]
fn release() {
  assert_cli::Assert::command(&["cargo", "run", "--release"])
    .stderr()
    .contains("single-panic-test")
    .stderr()
    .contains("Human Panic Authors")
    .stderr()
    .contains("human-panic-crate@example.com")
    .fails_with(101)
    .unwrap();
}

#[test]
fn debug() {
  assert_cli::Assert::command(&["cargo", "run"])
    .stderr()
    .contains("OMG EVERYTHING IS ON FIRE!!!")
    .fails_with(101)
    .unwrap();
}
