use assert_cmd::Command;
use rstest::rstest;

fn command() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[rstest]
fn count() {
    command()
        .arg("count")
        .arg("10")
        .assert()
        .success()
        .stdout("6\n");
}

#[rstest]
fn count_stdin() {
    command()
        .arg("count")
        .arg("-")
        .write_stdin("10")
        .assert()
        .success()
        .stdout("6\n");
}

#[rstest]
fn enumerated_sequence() {
    command()
        .arg("sequence")
        .arg("--enumerate")
        .arg("4")
        .assert()
        .success()
        .stdout("0: 4\n1: 2\n2: 1\n");
}

#[rstest]
fn sequence() {
    command()
        .arg("sequence")
        .arg("4")
        .assert()
        .success()
        .stdout("4\n2\n1\n");
}

#[rstest]
fn sequence_stdin() {
    command()
        .arg("sequence")
        .arg("-")
        .write_stdin("4")
        .assert()
        .success()
        .stdout("4\n2\n1\n");
}
