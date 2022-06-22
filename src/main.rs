use std::fmt::Display;

use anyhow::{bail, Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Lock {
    package: Vec<Package>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Expected exactly one arg");
    }

    let lock_path = &args[1];
    let lock_contents = std::fs::read_to_string(&lock_path).context("Could not read lock file")?;
    let lock: Lock = toml::from_str(&lock_contents).context("Could not parse lock")?;
    for package in lock.package {
        println!("{package}");
    }

    Ok(())
}
