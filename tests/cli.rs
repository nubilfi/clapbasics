use anyhow::Result as HowResult;
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Filesystem fixtures and assertions for testing.
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> HowResult<()> {
    let mut cmd = Command::cargo_bin("clapbasics")?;

    cmd.arg("foobar").arg("not.exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error opening not.exist file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> HowResult<()> {
    let file = assert_fs::NamedTempFile::new("cp.sample.txt")?;
    file.write_str("Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")?;

    let mut cmd = Command::cargo_bin("clapbasics")?;
    cmd.arg("consectetur").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2: consectetur"))
        .stdout(predicate::str::contains(format!("{}:\n", file.path().display())).count(0));

    Ok(())
}

#[test]
fn find_content_in_multiple_files() -> HowResult<()> {
    let f1 = assert_fs::NamedTempFile::new("cp.sample1.txt")?;
    let f2 = assert_fs::NamedTempFile::new("cp.sample2.txt")?;

    f1.write_str("Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")?;
    f2.write_str("This file contain consectetur")?;

    let mut cmd = Command::cargo_bin("clapbasics")?;
    cmd.arg("consectetur").arg(f1.path()).arg(f2.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{}:\n",
            f1.path().display()
        )))
        .stdout(predicate::str::contains("2: elit"))
        .stdout(predicate::str::contains(format!(
            "{}:\n",
            f2.path().display()
        )))
        .stdout(predicate::str::contains("1: This file contain elit word"));

    Ok(())
}
