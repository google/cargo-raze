// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cargo::CargoError;
use cargo::core::Workspace;
use cargo::core::summary::FeatureValue as CargoFeatureValue;
use cargo::core::dependency::Kind as CargoKind;
use cargo::ops::Packages;
use cargo::ops;
use cargo::util::CargoResult;
use cargo::util::Config;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempdir::TempDir;
use util::RazeError;
use util;

pub type PackageId = String;
pub type Kind = String;
pub type TargetSpec = String;

/**
 * An entity that can retrive deserialized metadata for a Cargo Workspace.
 *
 * The "CargoInternalsMetadataFetcher" is probably the one you want.
 *
 * Usage of ..Subcommand.. is waiting on a cargo release containing
 * https://github.com/rust-lang/cargo/pull/5122
 */
pub trait MetadataFetcher {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata>;
}

/** The local Cargo workspace files to be used for build planning .*/
pub struct CargoWorkspaceFiles {
  pub toml_path: PathBuf,
  pub lock_path_opt: Option<PathBuf>,
}

/**
 * The metadata for a whole Cargo workspace.
 *
 * WARNING: Cargo-raze does not control the definition of this struct. This struct mirrors Cargo's
 * own "ExportInfo":
 * https://github.com/rust-lang/cargo/blob/9c78c3a17ac4bc0c8b3b837095f60aa84d09c426/src/cargo/ops/cargo_output_metadata.rs#L78-L85
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metadata {
  pub packages: Vec<Package>,
  pub resolve: Resolve,
  pub workspace_members: Vec<PackageId>,
  pub target_directory: String,
  pub version: i64,
}

/**
 * The metadata for an individual Cargo crate.
 *
 * WARNING: Cargo-raze does not control the definition of this struct. This struct mirrors Cargo's
 * own "SerializedPackage":
 * https://github.com/rust-lang/cargo/blob/9c78c3a17ac4bc0c8b3b837095f60aa84d09c426/src/cargo/core/package.rs#L32-L50
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
  pub features: HashMap<String, Vec<String>>,
  pub manifest_path: String,
  pub edition: String,
  pub sha256: Option<String>,
}

/**
 * The metadata for a dependency (a reference connecting a crate to another crate).
 *
 * WARNING: Cargo-raze does not control the definition of this struct. This struct mirrors Cargo's
 * own "SerializedDependency":
 * https://github.com/rust-lang/cargo/blob/75ec2d3a8d045f90792b3ce5d7050cad43bfb3bf/src/cargo/core/dependency.rs#L49-L60
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
  pub name: String,
  pub source: String,
  pub req: String,
  pub kind: Option<Kind>,
  #[serde(default = "default_dependency_field_optional")] pub optional: bool,
  #[serde(default = "default_dependency_field_uses_default_features")]
  pub uses_default_features: bool,
  pub features: Vec<String>,
  pub target: Option<TargetSpec>,
}

/**
 * The metadata for a compileable target.
 *
 * WARNING: Cargo-raze does not control the definition of this struct. This struct mirrors Cargo's
 * own "SerializedTarget":
 * https://github.com/rust-lang/cargo/blob/c24a09772c2c1cb315970dbc721f2a42d4515f21/src/cargo/core/manifest.rs#L188-L197
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Target {
  pub name: String,
  pub kind: Vec<String>,
  pub crate_types: Vec<String>,
  pub src_path: String,
  pub edition: String,
}

/**
 * The metadata for a fully resolved dependency tree.
 *
 * WARNING: Cargo-raze does not control the definition of this struct. This struct mirrors Cargo's
 * own "MetadataResolve":
 * https://github.com/rust-lang/cargo/blob/9c78c3a17ac4bc0c8b3b837095f60aa84d09c426/src/cargo/ops/cargo_output_metadata.rs#L91-L95
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolve {
  pub nodes: Vec<ResolveNode>,
  pub root: PackageId,
}

/**
 * The metadata for a single resolved entry in the full dependency tree.
 *
 * WARNING: Cargo-raze does not control the definition of this struct. This struct mirrors Cargo's
 * own "Node":
 * https://github.com/rust-lang/cargo/blob/9c78c3a17ac4bc0c8b3b837095f60aa84d09c426/src/cargo/ops/cargo_output_metadata.rs#L102-L106
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolveNode {
  pub id: PackageId,
  pub dependencies: Vec<PackageId>,
  // Optional due to recent feature addition in Cargo.
  pub features: Option<Vec<String>>,
}

/** A workspace metadata fetcher that uses the Cargo Metadata subcommand. */
#[allow(dead_code)]
pub struct CargoSubcommandMetadataFetcher;

/**
 * A workspace metadata fetcher that uses Cargo's internals.
 *
 * !DANGER DANGER!
 * This struct is very hard to test as it uses Cargo's stateful internals, please take care when
 * changing it.
 * !DANGER DANGER!
 */
pub struct CargoInternalsMetadataFetcher<'config> {
  cargo_config: &'config Config,
}

impl MetadataFetcher for CargoSubcommandMetadataFetcher {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata> {
    assert!(files.toml_path.is_file());
    assert!(
      files
        .lock_path_opt
        .as_ref()
        .map(|p| p.is_file())
        .unwrap_or(true)
    );

    // Copy files into a temp directory
    // UNWRAP: Guarded by function assertion
    let cargo_tempdir = {
      let dir = try!(
        TempDir::new("cargo_raze_metadata_dir").map_err(CargoError::from)
      );
      {
        let dir_path = dir.path();
        let new_toml_path = dir_path.join(files.toml_path.file_name().unwrap());
        try!(fs::copy(files.toml_path, new_toml_path).map_err(CargoError::from));
        if let Some(lock_path) = files.lock_path_opt {
          let new_lock_path = dir_path.join(lock_path.file_name().unwrap());
          try!(fs::copy(lock_path, new_lock_path).map_err(CargoError::from));
        }
      }
      dir
    };

    // Shell out to cargo
    let exec_output = try!(
      Command::new("cargo")
        .current_dir(cargo_tempdir.path())
        .args(&["metadata", "--format-version", "1"])
        .output()
        .map_err(CargoError::from)
    );

    // Handle command errs
    let stdout_str =
      String::from_utf8(exec_output.stdout).unwrap_or("[unparsable bytes]".to_owned());
    if !exec_output.status.success() {
      let stderr_str =
        String::from_utf8(exec_output.stderr).unwrap_or("[unparsable bytes]".to_owned());
      println!("`cargo metadata` failed. Inspect Cargo.toml for issues!");
      println!("stdout: {}", stdout_str);
      println!("stderr: {}", stderr_str);
      return Err(CargoError::from(RazeError::Generic("Failed to run `cargo metadata`".to_owned())))
    }

    // Parse and yield metadata
    serde_json::from_str::<Metadata>(&stdout_str)
      .map_err(CargoError::from)
  }
}

impl<'config> MetadataFetcher for CargoInternalsMetadataFetcher<'config> {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata> {
    let manifest = if files.toml_path.is_relative() {
      env::current_dir().unwrap().join(&files.toml_path)
    } else {
      files.toml_path
    };
    let ws = try!(Workspace::new(&manifest, &self.cargo_config));
    let specs = Packages::All.to_package_id_specs(&ws)?;
    let root_name = specs.iter().next().unwrap().name().to_owned();

    let (resolved_packages, cargo_resolve) =
      ops::resolve_ws_precisely(&ws, None, &[], false, false, &specs)?;

    let root_package_id = try!(
      cargo_resolve
        .iter()
        .filter(|dep| dep.name().as_str() == root_name)
        .next()
        .ok_or(CargoError::from(RazeError::Internal("root crate should be in cargo resolve".to_owned())))
    ).to_string();

    let mut packages = Vec::new();
    let mut resolve = Resolve {
      nodes: Vec::new(),
      root: root_package_id,
    };

    for id in cargo_resolve.iter() {
      let dependencies = cargo_resolve.deps(id)
        .map(|(p, _)| p.to_string())
        .collect();
      let features = cargo_resolve
        .features_sorted(id)
        .iter()
        .map(|s| s.to_string())
        .collect();
      resolve.nodes.push(ResolveNode {
        id: id.to_string(),
        dependencies: dependencies,
        features: Some(features),
      })
    }

    for package_id in resolved_packages.package_ids() {
      // TODO(acmcarther): Justify this unwrap
      let package = resolved_packages.get_one(&package_id).unwrap().clone();
      let manifest_metadata = package.manifest().metadata();

      let mut dependencies = Vec::new();
      for dependency in package.dependencies().iter() {
        dependencies.push(Dependency {
          name: dependency.package_name().to_string(),
          // UNWRAP: It's cargo's responsibility to ensure a serializable source_id
          source: serde_json::to_string(dependency.source_id()).unwrap(),
          req: dependency.version_req().to_string(),
          kind: match dependency.kind() {
            CargoKind::Normal => None,
            CargoKind::Development => Some("dev".to_owned()),
            CargoKind::Build => Some("build".to_owned()),
          },
          optional: dependency.is_optional(),
          uses_default_features: dependency.uses_default_features(),
          features: dependency.features().iter().map(|s| s.to_string()).collect(),
          target: dependency.platform().map(|p| p.to_string()),
        });
      }

      let mut targets = Vec::new();
      for target in package.targets().iter() {
        let crate_types = target
          .rustc_crate_types()
          .iter()
          .map(|t| t.to_string())
          .collect();
        targets.push(Target {
          name: target.name().to_owned(),
          kind: util::kind_to_kinds(target.kind()),
          crate_types: crate_types,
          src_path: target.src_path().path().display().to_string(),
          edition: target.edition().to_string()
        });
      }

      let mut features = HashMap::new();
      for (feature, feature_values) in package.summary().features().iter() {
        let mut our_feature_values = Vec::new();
        for feature_value in feature_values {
          let our_feature_value = {
            match feature_value {
              CargoFeatureValue::Feature(feature_name) =>
                feature_name.to_string(),
              CargoFeatureValue::Crate(crate_name) =>
                crate_name.to_string(),
              // This matches the current Serialize impl for CargoFeatureValue
              CargoFeatureValue::CrateFeature(crate_name, feature_name) =>
                format!("{}/{}", crate_name.as_str(), feature_name.as_str()),
            }
          };
          our_feature_values.push(our_feature_value);
        }
        features.insert(feature.to_string(),
                        our_feature_values);
      }

      // UNWRAP: It's cargo's responsibility to ensure a serializable source_id
      let pkg_source = serde_json::to_string(package_id.source_id()).unwrap();

      // Cargo use SHA256 for checksum so we can use them directly
      let sha256 = package
        .manifest()
        .summary()
        .checksum()
        .map(ToString::to_string);

      packages.push(Package {
        name: package.name().to_string(),
        version: package.version().to_string(),
        id: package_id.to_string(),
        license: manifest_metadata.license.clone(),
        license_file: manifest_metadata.license_file.clone(),
        description: manifest_metadata.description.clone(),
        source: Some(pkg_source),
        dependencies: dependencies,
        targets: targets,
        features: features,
        manifest_path: package.manifest_path().display().to_string(),
        edition: package.manifest().edition().to_string(),
        sha256: sha256,
      });
    }

    let workspace_members = ws.members()
      .map(|pkg| pkg.package_id().to_string())
      .collect();

    Ok(Metadata {
      packages: packages,
      resolve: resolve,
      workspace_members: workspace_members,
      target_directory: ws.target_dir().display().to_string(),
      version: 0, /* not generated via subcomand */
    })
  }
}

impl<'config> CargoInternalsMetadataFetcher<'config> {
  pub fn new(cargo_config: &'config Config) -> CargoInternalsMetadataFetcher<'config> {
    CargoInternalsMetadataFetcher {
      cargo_config: cargo_config,
    }
  }
}

fn default_dependency_field_optional() -> bool {
  // Dependencies are implicitly required.
  // TODO(acmcarther): Citation?
  false
}

fn default_dependency_field_uses_default_features() -> bool {
  // Default features are used by default
  // Citation: https://doc.rust-lang.org/cargo/reference/manifest.html#rules
  true
}

#[cfg(test)]
pub mod testing {
  use super::*;

  pub struct StubMetadataFetcher {
    metadata: Metadata,
  }

  impl MetadataFetcher for StubMetadataFetcher {
    fn fetch_metadata(&mut self, _: CargoWorkspaceFiles) -> CargoResult<Metadata> {
      Ok(self.metadata.clone())
    }
  }

  impl StubMetadataFetcher {
    pub fn with_metadata(metadata: Metadata) -> StubMetadataFetcher {
      StubMetadataFetcher { metadata: metadata }
    }
  }

  pub fn dummy_package() -> Package {
    Package {
      name: String::new(),
      version: String::new(),
      id: String::new(),
      license: None,
      license_file: None,
      description: None,
      source: None,
      dependencies: Vec::new(),
      targets: Vec::new(),
      features: HashMap::new(),
      manifest_path: String::new(),
      edition: String::new(),
      sha256: None,
    }
  }

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
  use std::fs::File;
  use std::io::Write;

  fn basic_toml() -> &'static str {
    "
[package]
name = \"test\"
version = \"0.0.1\"

[lib]
path = \"not_a_file.rs\"
    "
  }

  fn basic_lock() -> &'static str {
    "
[[package]]
name = \"test\"
version = \"0.0.1\"
dependencies = [
]
    "
  }

  #[test]
  fn test_metadata_deserializes_correctly() {
    let metadata_file_contents = include_str!("../test_fixtures/metadata.txt");
    serde_json::from_str::<Metadata>(metadata_file_contents).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_without_lock() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let toml_path = dir.path().join("Cargo.toml");
    let mut toml = File::create(&toml_path).unwrap();
    toml.write_all(basic_toml().as_bytes()).unwrap();
    let files = CargoWorkspaceFiles {
      toml_path: toml_path,
      lock_path_opt: None,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher;

    fetcher.fetch_metadata(files).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_with_lock() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let toml_path = {
      let path = dir.path().join("Cargo.toml");
      let mut toml = File::create(&path).unwrap();
      toml.write_all(basic_toml().as_bytes()).unwrap();
      path
    };
    let lock_path = {
      let path = dir.path().join("Cargo.lock");
      let mut lock = File::create(&path).unwrap();
      lock.write_all(basic_lock().as_bytes()).unwrap();
      path
    };
    let files = CargoWorkspaceFiles {
      toml_path: toml_path,
      lock_path_opt: Some(lock_path),
    };

    let mut fetcher = CargoSubcommandMetadataFetcher;

    fetcher.fetch_metadata(files).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_handles_bad_files() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let toml_path = {
      let path = dir.path().join("Cargo.toml");
      let mut toml = File::create(&path).unwrap();
      toml.write_all(b"hello").unwrap();
      path
    };
    let files = CargoWorkspaceFiles {
      toml_path: toml_path,
      lock_path_opt: None,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher;
    assert!(fetcher.fetch_metadata(files).is_err());
  }
}
