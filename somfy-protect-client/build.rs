use std::{env, process::Command};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let cwd = env::var("CARGO_MANIFEST_DIR")?;

    // Build the somfy protect api mock image in the repository
    let output = Command::new("docker")
        .arg("build")
        .arg("--file")
        .arg(&format!("{cwd}/../mock/Dockerfile"))
        .arg("--tag")
        .arg("somfy-protect-api-mock:latest")
        .arg(&format!("{cwd}/../mock"))
        .output()?;
    if !output.status.success() {
        eprintln!("stderr: {}", String::from_utf8(output.stderr)?);
        bail!("unable to build somfy-protect-api-mock:latest:latest");
    }
    eprintln!("Built somfy-protect-api-mock:latest");

    // trigger recompilation when dockerfiles are modified
    println!("cargo:rerun-if-changed=../mock");

    Ok(())
}
