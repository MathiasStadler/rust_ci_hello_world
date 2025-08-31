// tests/integration_test.rs
use assert_cmd::Command;


// cargo add  assert_cmd
#[test]
fn test_main_output() {
    Command::cargo_bin("rust_ci_hello_world")
        .unwrap()
        .assert()
        .success()
        .stdout("Hello, world!\n");
}