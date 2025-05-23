use std::fmt::Display;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

use anyhow::{Context, Result, bail};

mod crystal;
mod dart;
mod go;
mod javascript;
mod php;
mod python;
mod ruby;
mod rust;
use crystal::parse_shard_lock;
use dart::parse_pubspec_lock;
use go::parse_go_sum;
use javascript::{parse_npm_lock, parse_yarn_lock};
use php::parse_composer_lock;
use python::parse_requirements_txt;
use ruby::parse_gemfile_lock;
use serde::Deserialize;

#[derive(PartialEq, Eq, Debug, Deserialize)]
// This is the struct we want to use for the final display
// of the lock contents.
// It only contains the name and version of the package.
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

#[derive(PartialEq, Eq, Debug, Deserialize)]
// Sometimes we need a struct to represent package metadata while parsing
// locks because we already got the name. In this case the struct
// only contains the package version.
struct PackageMetadata {
    version: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.version)
    }
}

#[derive(Deserialize, Debug)]
// poetry, uv and Cargo uses exact same format for the lock file
struct TomlLock {
    #[serde(rename = "package")]
    packages: Vec<Package>,
}

impl TomlLock {
    fn packages(self) -> Vec<Package> {
        self.packages.into_iter().collect()
    }
}

fn parse_toml_lock(contents: &str) -> Result<Vec<Package>> {
    let lock: TomlLock = toml::from_str(contents)?;
    Ok(lock.packages())
}

fn parse_lock(name: &str, contents: &str) -> Result<Vec<Package>> {
    fn is_python_requirements(name: &str) -> bool {
        name.contains("requirements") && name.ends_with(".txt")
    }

    match name {
        "Cargo.lock" | "poetry.lock" | "uv.lock" => parse_toml_lock(contents),
        "Gemfile.lock" => parse_gemfile_lock(contents),
        "composer.lock" => parse_composer_lock(contents),
        "go.sum" => parse_go_sum(contents),
        "package-lock.json" => parse_npm_lock(contents),
        "pubspec.lock" => parse_pubspec_lock(contents),
        "shard.lock" => parse_shard_lock(contents),
        "yarn.lock" => parse_yarn_lock(contents),
        name if is_python_requirements(name) => parse_requirements_txt(contents),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toml_lock() {
        let contents = r#"
# This file is automatically @generated by Poetry 2.0.1 and should not be changed by hand.

[[package]]
name = "anyio"
version = "4.8.0"
description = "High level compatibility layer for multiple asynchronous event loop implementations"
files = [
]

[package.dependencies]
idna = ">=2.8"

[package.extras]
doc = ["Sphinx (>=7.4,<8.0)", "packaging", "sphinx-autodoc-typehints (>=1.2.0)", "sphinx_rtd_theme"]

[[package]]
name = "asgiref"
version = "3.8.1"

[metadata]
lock-version = "2.1"
python-versions = ">= 3.13"
"#;
        let packages = parse_toml_lock(contents).unwrap();
        assert_eq!(
            &packages,
            &[
                Package::new("anyio", "4.8.0"),
                Package::new("asgiref", "3.8.1")
            ]
        );
    }
}
