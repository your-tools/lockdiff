use crate::Package;
use anyhow::{Result, bail};
use std::collections::BTreeMap;

pub(crate) fn parse_go_sum(contents: &str) -> Result<Vec<Package>> {
    // For some reason go.mod contains several versions for the same package,
    // so we keep the latest one
    let mut version_map = BTreeMap::<&str, &str>::new();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let words: Vec<_> = line.split(' ').collect();
        if words.len() != 3 {
            bail!("Expected line {line} to contains exactly 3 words");
        }
        let name = &words[0];
        let version = &words[1];
        let version = version.split('/').next().expect("version cannot be empty");
        version_map.insert(name, version);
    }
    Ok(version_map
        .into_iter()
        .map(|(name, version)| Package::new(name, version))
        .collect())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_go_sum() {
        let contents = r#"
cloud.google.com/go v0.26.0/go.mod h1:hash1
cloud.google.com/go v0.34.0/go.mod h1:hash1
cloud.google.com/go/storage v1.14.0/go.mod h1:hash2
"#;
        let packages = parse_go_sum(contents).unwrap();
        assert_eq!(
            &packages,
            &[
                Package::new("cloud.google.com/go", "v0.34.0"),
                Package::new("cloud.google.com/go/storage", "v1.14.0")
            ]
        );
    }
}
