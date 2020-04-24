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

use cargo::CargoResult;

use serde_json;
use std::{collections::HashMap, fs, path::PathBuf, process::Command};

use crate::util::RazeError;
use tempdir::TempDir;

use serde_derive::{Deserialize, Serialize};

const SYSTEM_CARGO_BIN_PATH: &str = "cargo";

pub type PackageId = String;
pub type Kind = String;
pub type TargetSpec = String;

/**
 * An entity that can retrive deserialized metadata for a Cargo Workspace.
 *
 * The `CargoSubcommandMetadataFetcher` is probably the one you want.
 *
 * Usage of ..Subcommand.. is waiting on a cargo release containing
 * <https://github.com/rust-lang/cargo/pull/5122>
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
 * WARNING: Cargo-raze does not control the definition of this struct.
 * This struct mirrors Cargo's own [`ExportInfo`](
 * https://github.com/rust-lang/cargo/blob/0.40.0/src/cargo/ops/cargo_output_metadata.rs#L78-L85)
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
 * WARNING: Cargo-raze does not control the definition of this struct.
 * This struct mirrors Cargo's own [`SerializedPackage`](
 * https://github.com/rust-lang/cargo/blob/0.40.0/src/cargo/core/package.rs#L32-L50)
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
 * WARNING: Cargo-raze does not control the definition of this struct.
 * This struct mirrors Cargo's own [`SerializedDependency`](
 * https://github.com/rust-lang/cargo/blob/0.40.0/src/cargo/core/dependency.rs#L49-L60)
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
  pub name: String,
  pub source: String,
  pub req: String,
  pub kind: Option<Kind>,
  #[serde(default = "default_dependency_field_optional")]
  pub optional: bool,
  #[serde(default = "default_dependency_field_uses_default_features")]
  pub uses_default_features: bool,
  pub features: Vec<String>,
  pub target: Option<TargetSpec>,
}

/**
 * The metadata for a compileable target.
 *
 * WARNING: Cargo-raze does not control the definition of this struct.
 * This struct mirrors Cargo's own [`SerializedTarget`](
 * https://github.com/rust-lang/cargo/blob/0.40.0/src/cargo/core/manifest.rs#L188-L197)
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
 * WARNING: Cargo-raze does not control the definition of this struct.
 * This struct mirrors Cargo's own [`MetadataResolve`](
 * https://github.com/rust-lang/cargo/blob/0.40.0/src/cargo/ops/cargo_output_metadata.rs#L91-L95)
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolve {
  pub nodes: Vec<ResolveNode>,
  pub root: PackageId,
}

/**
 * The metadata for a single resolved entry in the full dependency tree.
 *
 * WARNING: Cargo-raze does not control the definition of this struct.
 * This struct mirrors Cargo's own [`Node`](
 * https://github.com/rust-lang/cargo/blob/0.40.0/src/cargo/ops/cargo_output_metadata.rs#L102-L106)
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolveNode {
  pub id: PackageId,
  pub dependencies: Vec<PackageId>,
  // Optional due to recent feature addition in Cargo.
  pub features: Option<Vec<String>>,
}

/** A workspace metadata fetcher that uses the Cargo Metadata subcommand. */
pub struct CargoSubcommandMetadataFetcher {
  cargo_bin_path: PathBuf,
}

impl CargoSubcommandMetadataFetcher {
  pub fn new<P: Into<PathBuf>>(cargo_bin_path: P) -> CargoSubcommandMetadataFetcher {
    CargoSubcommandMetadataFetcher {
      cargo_bin_path: cargo_bin_path.into(),
    }
  }
}

impl Default for CargoSubcommandMetadataFetcher {
  fn default() -> CargoSubcommandMetadataFetcher {
    CargoSubcommandMetadataFetcher::new(SYSTEM_CARGO_BIN_PATH)
  }
}

impl MetadataFetcher for CargoSubcommandMetadataFetcher {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata> {
    assert!(files.toml_path.is_file());
    assert!(files.lock_path_opt.as_ref().map_or(true, |p| p.is_file()));

    // Copy files into a temp directory
    // UNWRAP: Guarded by function assertion
    let cargo_tempdir = {
      let dir = TempDir::new("cargo_raze_metadata_dir")?;

      let dir_path = dir.path();
      let new_toml_path = dir_path.join(files.toml_path.file_name().unwrap());
      fs::copy(files.toml_path, new_toml_path)?;
      if let Some(lock_path) = files.lock_path_opt {
        let new_lock_path = dir_path.join(lock_path.file_name().unwrap());
        fs::copy(lock_path, new_lock_path)?;
      }

      dir
    };

    // Shell out to cargo
    let exec_output = Command::new(&self.cargo_bin_path)
      .current_dir(cargo_tempdir.path())
      .args(&["metadata", "--format-version", "1"])
      .output()?;

    // Handle command errs
    let stdout_str =
      String::from_utf8(exec_output.stdout).unwrap_or_else(|_| "[unparsable bytes]".to_owned());

    if !exec_output.status.success() {
      let stderr_str =
        String::from_utf8(exec_output.stderr).unwrap_or_else(|_| "[unparsable bytes]".to_owned());
      println!("`cargo metadata` failed. Inspect Cargo.toml for issues!");
      println!("stdout: {}", stdout_str);
      println!("stderr: {}", stderr_str);
      return Err(RazeError::Generic("Failed to run `cargo metadata`".to_owned()).into());
    }

    // Parse and yield metadata
    serde_json::from_str::<Metadata>(&stdout_str).map_err(|e| e.into())
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
      StubMetadataFetcher { metadata }
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
      lock_path_opt: None,
      toml_path,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher::default();

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
      lock_path_opt: Some(lock_path),
      toml_path,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher::default();

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
      lock_path_opt: None,
      toml_path,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher::default();
    assert!(fetcher.fetch_metadata(files).is_err());
  }
}
