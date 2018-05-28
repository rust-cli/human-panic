extern crate assert_cli;

#[test]
fn integration() {
  assert_cli::Assert::main_binary()
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
