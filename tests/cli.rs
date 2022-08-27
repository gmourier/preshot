use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

const BINARY_NAME: &str = "blitz";
const MASTER_KEY: &str = "masterKey";

#[test]
fn discover_keys_missing_master_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("discover-keys");

    cmd.assert().failure();

    Ok(())
}

#[test]
#[ignore] //TODO: Add a control check to throw an error if the UIDS arg is missing
fn discover_keys_missing_uids() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("discover-keys").arg(MASTER_KEY);

    cmd.assert().failure();

    Ok(())
}

#[test]
fn discover_keys_single() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("discover-keys")
        .arg(MASTER_KEY)
        .arg("d7d30ffe-ec60-484f-84f8-1c8b7d0ac352");

    let expected = " uid                                  | 🔑 key \n d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 | 623359df9ea4d4a6c676c329c793191601ce7dd15541c2394277eae26aeedf1e \n";

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn discover_keys_multiple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("discover-keys")
        .arg(MASTER_KEY)
        .arg("d7d30ffe-ec60-484f-84f8-1c8b7d0ac352")
        .arg("c5a18797-621c-42b5-81bd-23fbf0202364");

    let expected = " uid                                  | 🔑 key \n d7d30ffe-ec60-484f-84f8-1c8b7d0ac352 | 623359df9ea4d4a6c676c329c793191601ce7dd15541c2394277eae26aeedf1e \n c5a18797-621c-42b5-81bd-23fbf0202364 | 9decc7baffbed2fa9b9cfa599c3d72ecf8db3fad02b65941caa378e824299482 \n";

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn generate_keys_single() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("generate-keys").arg(MASTER_KEY);

    cmd.assert().success();

    Ok(())
}

#[test]
fn generate_keys_multiple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("generate-keys").arg(MASTER_KEY).arg("10");

    cmd.assert().success();

    Ok(())
}

#[test]
fn generate_keys_multiple_misstyped_count_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("generate-keys").arg(MASTER_KEY).arg("dummy");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn generate_uuids() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("generate-uuids");

    cmd.assert().success();

    Ok(())
}

#[test]
fn generate_uuids_multiple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("generate-uuids").arg("10");

    cmd.assert().success();

    Ok(())
}

#[test]
fn generate_uuids_misstyped_count_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("generate-uuids").arg("dummy");

    cmd.assert().failure();

    Ok(())
}
