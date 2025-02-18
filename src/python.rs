use crate::Package;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PoetryLock {
    #[serde(rename = "package")]
    packages: Vec<PythonPackage>,
}

impl PoetryLock {
    fn packages(self) -> Vec<Package> {
        self.packages
            .into_iter()
            .map(|p| Package::new(&p.name, &p.version))
            .collect()
    }
}

#[derive(Deserialize, Debug)]
struct PythonPackage {
    name: String,
    version: String,
}

pub(crate) fn parse_poetry_lock(contents: &str) -> Result<Vec<Package>> {
    let lock: PoetryLock = toml::from_str(contents)?;
    Ok(lock.packages())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_poetry_lock() {
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
        let packages = parse_poetry_lock(contents).unwrap();
        assert_eq!(
            &packages,
            &[
                Package::new("anyio", "4.8.0"),
                Package::new("asgiref", "3.8.1")
            ]
        );
    }
}
