use std::fmt::Display;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

use anyhow::{bail, Context, Result};

mod dart;
mod go;
mod node;
mod php;
mod ruby;
mod rust;
use dart::parse_pubspec_lock;
use go::parse_go_sum;
use node::{parse_npm_lock, parse_yarn_lock};
use php::parse_composer_lock;
use ruby::parse_gemfile_lock;
use rust::parse_cargo_lock;

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
        write!(f, "{} ({})", self.name, self.version)
    }
}

fn parse_lock(name: &str, contents: &str) -> Result<Vec<Package>> {
    match name {
        "Cargo.lock" | "poetry.lock" => parse_cargo_lock(contents),
        "Gemfile.lock" => parse_gemfile_lock(contents),
        "composer.lock" => parse_composer_lock(contents),
        "go.sum" => parse_go_sum(contents),
        "package-lock.json" => parse_npm_lock(contents),
        "pubspec.lock" => parse_pubspec_lock(contents),
        "yarn.lock" => parse_yarn_lock(contents),
        _ => bail!("Unknown lock name: {name}"),
    }
}

pub fn run() -> Result<()> {
    // Main entry point
    //
    // Note that if anything goes wrong, we would rather print the original contents
    // of the lock file rather than just an error message
    //
    // So, first make sure we have *something* to print
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Expected exactly one arg");
    }
    let lock_path = &args[1];
    let lock_path = PathBuf::from(lock_path);
    let file_name = lock_path
        .file_name()
        .context("|| lock path should have a file name")?;
    let lock_contents = std::fs::read_to_string(&lock_path).context("Could not read lock file")?;

    // Then, try and convert the lock, and if it fails, just print the contents followed
    // by an error message
    if let Err(e) = print_lock(file_name, &lock_path, &lock_contents) {
        println!("{lock_contents}");
        eprintln!("Note: {e:#}");
    }
    Ok(())
}

fn print_lock(file_name: &OsStr, lock_path: &Path, contents: &str) -> Result<()> {
    let file_name = file_name
        .to_str()
        .context("File name {file_name:?} should be UTF-8")?;
    let packages = parse_lock(file_name, contents)
        .context(format!("Could not parse {}", lock_path.display()))?;
    for package in packages {
        println!("{package}");
    }
    Ok(())
}
