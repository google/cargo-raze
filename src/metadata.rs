use std::collections::HashMap;

pub type PackageId = String;
pub type FeatureOrDependency = String;
pub type Kind = String;
pub type TargetSpec = String;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Metadata {
  pub packages: Vec<Package>,
  pub resolve: Resolve,
  pub workspace_members: Vec<PackageId>,
  pub target_directory: String,
  pub version: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Package {
  pub name: String,
  pub version: String,
  pub id: PackageId,
  pub license: Option<String>,
  pub license_file: Option<String>,
  pub description: Option<String>,
  pub source: Option<String>,
  pub dependencies: Vec<Dependency>,
  pub targets: Vec<Target>,
  pub features: HashMap<String, Vec<FeatureOrDependency>>,
  pub manifest_path: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Dependency {
  pub name: String,
  pub source: String,
  pub req: String,
  pub kind: Option<Kind>,
  #[serde(default = "default_dependency_field_optional")]
  pub optional: bool,
  #[serde(default = "default_dependency_field_use_default_features")]
  pub use_default_features: bool,
  pub features: Vec<FeatureOrDependency>,
  pub target: Option<TargetSpec>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Target {
  pub name: String,
  pub kind: Vec<String>,
  pub crate_types: Vec<String>,
  pub src_path: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Resolve {
  pub nodes: Vec<ResolveNode>,
  pub root: PackageId,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ResolveNode {
  pub id: PackageId,
  pub dependencies: Vec<PackageId>,
  // Optional due to recent feature addition in Cargo.
  pub features: Option<Vec<String>>,
}

fn default_dependency_field_optional() -> bool {
  // Dependencies are implicitly required.
  // TODO(acmcarther): Citation?
  false
}

fn default_dependency_field_use_default_features() -> bool {
  // Default features are used by default
  // Citation: https://doc.rust-lang.org/cargo/reference/manifest.html#rules
  true
}

pub mod testing {
  use super::*;

  pub fn dummy_metadata() -> Metadata {
    Metadata {
      packages: Vec::new(),
      resolve: dummy_resolve(),
      workspace_members: Vec::new(),
      target_directory: String::new(),
      version: 1,
    }
  }

  pub fn dummy_resolve() -> Resolve {
    Resolve {
      nodes: Vec::new(),
      root: String::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json;

  #[test]
  fn test_metadata_deserializes_correctly() {
    let metadata_file_contents = include_str!("../test_fixtures/metadata.txt");
    let metadata = serde_json::from_str::<Metadata>(metadata_file_contents).unwrap();
  }
}
