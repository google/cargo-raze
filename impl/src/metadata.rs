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

use cargo_metadata::MetadataCommand;
pub use cargo_metadata::{Dependency, DependencyKind, Metadata, Node, PackageId, Package, Resolve, Target};

use std::{fs, path::PathBuf};

use tempdir::TempDir;

const SYSTEM_CARGO_BIN_PATH: &str = "cargo";

pub type Kind = String;

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

    MetadataCommand::new()
      .cargo_path(&self.cargo_bin_path)
      .current_dir(cargo_tempdir.path())
      .other_options(&["--format-version".to_owned(), "1".to_owned()])
      .exec()
      .map_err(|e| e.into())
  }
}

/*
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
*/
