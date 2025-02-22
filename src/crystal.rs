use crate::{Package, PackageMetadata};
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
struct ShardLock {
    shards: BTreeMap<String, PackageMetadata>,
}

impl ShardLock {
    fn packages(self) -> Vec<Package> {
        self.shards
            .into_iter()
            .map(|(key, value)| Package::new(&key, &value.version))
            .collect()
    }
}

pub(crate) fn parse_shard_lock(contents: &str) -> Result<Vec<Package>> {
    let shard_lock: ShardLock = serde_yml::from_str(contents)?;
    Ok(shard_lock.packages())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_pubspec_lock() {
        let contents = r#"
version: 2.0
shards:
  db:
    git: https://github.com/crystal-lang/crystal-db.git
    version: 0.13.1

  pg:
    git: https://github.com/will/crystal-pg.git
    version: 0.29.0
"#;
        let packages = parse_shard_lock(contents).unwrap();
        assert_eq!(
            &packages,
            &[Package::new("db", "0.13.1"), Package::new("pg", "0.29.0")]
        )
    }
}
