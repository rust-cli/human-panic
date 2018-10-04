extern crate assert_cli;

#[test]
fn release() {
  assert_cli::Assert::command(&["cargo", "run", "--release"])
    .stderr()
    .contains("custom-panic-test")
    .stderr()
    .contains("My Company Support")
    .stderr()
    .contains("support@mycompany.com")
    .stderr()
    .contains("support.mycompany.com")
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
