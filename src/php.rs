use anyhow::Result;
use serde::Deserialize;

use crate::Package;

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ComposerPackage {
    name: String,
    version: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
struct ComposerLock {
    packages: Vec<ComposerPackage>,
    packages_dev: Vec<ComposerPackage>,
}

impl ComposerLock {
    fn packages(self) -> Vec<Package> {
        self.packages
            .into_iter()
            .chain(self.packages_dev)
            .map(|p| Package::new(&p.name, &p.version))
            .collect()
    }
}

pub(crate) fn parse_composer(contents: &str) -> Result<Vec<Package>> {
    let composer_lock: ComposerLock = serde_json::from_str(contents)?;
    Ok(composer_lock.packages())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_composer_lock() {
        let contents = r#"
{
   "_readme" : [
      "This file locks the dependencies of your project to a known state"
   ],
   "content-hash" : "657b85759b97fae05fc2983bd4abadf7",
   "packages" : [
      {
         "name" : "acme.corp/stuff",
         "source" : {},
         "version" : "1.2.3"
      }
   ],
   "packages-dev" : [
      {
         "name" : "myclabs/deep-copy",
         "source" : {},
         "version" : "1.11.0"
      }
   ]
}
"#;
        let packages = parse_composer(contents).unwrap();
        assert_eq!(
            &packages,
            &[
                Package::new("acme.corp/stuff", "1.2.3"),
                Package::new("myclabs/deep-copy", "1.11.0")
            ]
        );
    }
}
