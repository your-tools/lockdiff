use std::fmt::Display;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};

mod node;
mod php;
mod rust;
use node::{parse_npm, parse_yarn};
use php::parse_composer;
use rust::parse_cargo;

#[derive(PartialEq, Eq, Debug)]
/// Represents basic information of a package in a lock file
/// We keep only the name and the version
struct Package {
    name: String,
    version: String,
}

impl Package {
    fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

fn parse_lock(name: &str, contents: &str) -> Result<Vec<Package>> {
    match name {
        "Cargo.lock" | "poetry.lock" => parse_cargo(contents),
        "composer.lock" => parse_composer(contents),
        "package-lock.json" => parse_npm(contents),
        "yarn.lock" => parse_yarn(contents),
        _ => bail!("Unknown lock name: {name}"),
    }
}

pub fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Expected exactly one arg");
    }

    let lock_path = &args[1];
    let lock_path = PathBuf::from(lock_path);
    let name = lock_path
        .file_name()
        .ok_or_else(|| anyhow!("Lock path should have a file name"))?;
    let name = name
        .to_str()
        .ok_or_else(|| anyhow!("File name should be valid UTF-8"))?;
    let lock_contents = std::fs::read_to_string(&lock_path).context("Could not read lock file")?;
    let packages = parse_lock(name, &lock_contents)?;
    for package in packages {
        println!("{package}");
    }
    Ok(())
}
