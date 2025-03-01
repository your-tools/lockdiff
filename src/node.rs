use crate::{Package, PackageMetadata};
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct NpmLock {
    packages: BTreeMap<String, PackageMetadata>,
}

impl NpmLock {
    pub(crate) fn packages(self) -> Vec<Package> {
        self.packages
            .into_iter()
            .filter(|(k, _)| !k.is_empty())
            .map(|(k, v)| Package::new(&k.replace("node_modules/", ""), &v.version))
            .collect()
    }
}

pub(crate) fn parse_npm_lock(contents: &str) -> Result<Vec<Package>> {
    let npm_lock: NpmLock = serde_json::from_str(contents)?;
    Ok(npm_lock.packages())
}

pub(crate) fn parse_yarn_lock(contents: &str) -> Result<Vec<Package>> {
    let lock = yarn_lock_parser::parse_str(contents)?;
    Ok(lock
        .entries
        .iter()
        .map(|entry| Package::new(entry.name, entry.version))
        .collect())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_npm_lock() {
        let contents = r#"
{
  "name": "my-proj",
  "version": "0.0.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "name": "my-proj",
      "version": "0.0.0",
      "dependencies": {
        "@eslint": "^2.1.2"
      }
    },
    "node_modules/@eslint/eslintrc": {
      "version": "2.1.2",
       "resolved": "https://registry.npmjs.org/@eslint/eslintrc/-/eslintrc-2.1.2.tgz"
    }
  }
}
"#;

        let packages = parse_npm_lock(contents).unwrap();
        assert_eq!(&packages, &[Package::new("@eslint/eslintrc", "2.1.2")]);
    }
}
