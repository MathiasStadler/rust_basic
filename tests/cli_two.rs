// FROM HERE
// https://stackoverflow.com/questions/71534013/how-do-you-run-the-main-binary-and-then-run-tests-based-on-it-in-rust

// call a prg from example folder
use assert_cmd::prelude::*; // Add methods on commands
                            // use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(r#"examples/main"#)?;

    // cmd.arg("foobar").arg("test/file/doesnt/exist");

    cmd.assert().success();

    Ok(())
}

#[test]
fn test_main_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(r#"examples/main_args"#)?;

    cmd.arg("one_args").arg("two_args");

    cmd.assert().success();

    Ok(())
}

// cargo test -- --nocapture --test cli_two
// cargo test --test cli_two
