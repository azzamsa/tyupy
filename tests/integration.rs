use std::{error::Error, process::Command};

use assert_cmd::{crate_name, prelude::*};
use predicates::prelude::*;

#[test]
fn help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Transform URLs into any format"));

    Ok(())
}
