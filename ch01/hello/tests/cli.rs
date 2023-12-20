// use std::process::Command;
use assert_cmd::Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    // let res = cmd.output();
    // assert!(res.is_ok());
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
