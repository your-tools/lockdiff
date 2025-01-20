use crate::Package;
use anyhow::{bail, Result};

pub(crate) fn parse_gemfile_lock(contents: &str) -> Result<Vec<Package>> {
    // Inspired by:
    // https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler/lockfile_parser.rb
    let mut res = vec![];
    let blank4 = "    ";
    let blank6 = "      ";
    let mut in_gem_section = false;
    let mut in_spec_section = false;
    for line in contents.lines() {
        if line.starts_with("GEM") {
            in_gem_section = true;
            continue;
        }
        if !line.starts_with(' ') {
            in_gem_section = false;
            in_spec_section = false;
        }
        if in_gem_section {
            if line == "  specs:" {
                in_spec_section = true;
                continue;
            }
            if in_spec_section && line.starts_with(blank4) {
                if line.starts_with(blank6) {
                    continue;
                }
                let package = parse_line(&line[4..])?;
                res.push(package);
            } else {
                in_spec_section = false;
            }
        }
    }
    Ok(res)
}

fn parse_line(line: &str) -> Result<Package> {
    let parts: Vec<_> = line.splitn(2, ' ').collect();
    if parts.len() != 2 {
        bail!("Expected two words for spec line {line}");
    }
    let name = parts[0];
    let version_in_parenthesis = parts[1];
    let end = version_in_parenthesis.len() - 1;
    let version = &version_in_parenthesis[1..end];
    Ok(Package::new(name, version))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_gemfile_lock() {
        let contents = r#"
OTHER SECTION
  foo
    bar

GEM
  remote: https://rubygems.org/
  specs:
    diff-lcs (1.5.1)
    rspec (3.13.0)
      rspec-core (~> 3.13.0)
      rspec-expectations (~> 3.13.0)
      rspec-mocks (~> 3.13.0)
    rspec-core (3.13.2)
      rspec-support (~> 3.13.0)
  ignored:
      line1
      line2

PLATFORMS
  ruby
  x86_64-linux
        "#;

        let packages = parse_gemfile_lock(contents).unwrap();
        assert_eq!(
            packages,
            &[
                Package::new("diff-lcs", "1.5.1"),
                Package::new("rspec", "3.13.0"),
                Package::new("rspec-core", "3.13.2"),
            ]
        );
    }
}
