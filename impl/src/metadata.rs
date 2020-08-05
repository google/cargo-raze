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

use std::{borrow::Cow, env, fmt, fs, io::Read, path::{Path, PathBuf}};

use anyhow::{Error, Result};

use cargo_metadata::MetadataCommand;
pub use cargo_metadata::{DependencyKind, Metadata, Node, Package, PackageId};

use tempdir::TempDir;

use crate::settings::CargoToml;

const SYSTEM_CARGO_BIN_PATH: &str = "cargo";

/**
 * An entity that can retrive deserialized metadata for a Cargo Workspace.
 *
 * Usage of ..Subcommand.. is waiting on a cargo release containing
 * <https://github.com/rust-lang/cargo/pull/5122>
 */
pub trait MetadataFetcher {
  fn fetch_metadata(&mut self, files: &CargoWorkspaceFiles) -> Result<Metadata>;
}

#[derive(Debug)]
enum MetadataError {
  RootToml(Error, PathBuf),
  MemberTomls(Error, Vec<PathBuf>),
  CargoLock(Error, PathBuf),
}

impl std::error::Error for MetadataError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      MetadataError::RootToml(e, _) => Some(e.as_ref()),
      MetadataError::MemberTomls(e, _) => Some(e.as_ref()),
      MetadataError::CargoLock(e, _) => Some(e.as_ref()),
    }
  }
}
impl fmt::Display for MetadataError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MetadataError::RootToml(_, path) => write!(f, "couldn't open Cargo.toml at: \"{}\"", path.display()),
      MetadataError::MemberTomls(_, members) => {
        let s = members
          .iter()
          .map(|m| m.to_string_lossy())
          .collect::<Vec<Cow<'_, str>>>()
          .join(", ");
        write!(f, "couldn't open Cargo.tomls for the following members: \"{}\"", s)
      },
      MetadataError::CargoLock(_, path) => write!(f, "couldn't open Cargo.lock at: \"{}\"", path.display()),
    }
  }
}

/** The local Cargo workspace files to be used for build planning .*/
#[derive(Debug)]
pub struct CargoWorkspaceFiles {
  pub root_toml_path: PathBuf,
  pub member_toml_paths: Vec<PathBuf>,
  pub lock_path_opt: Option<PathBuf>,
}

impl CargoWorkspaceFiles {
  pub fn new<P: Into<PathBuf>>(root_toml_path: P, infer_lock_file: bool) -> Result<Self> {
    let root_toml_path = root_toml_path.into();
    // Find abs path to toml
    // TODO: make nicer
    let abs_root_toml_path = if root_toml_path.is_absolute() {
      root_toml_path.canonicalize()?
    } else {
      env::current_dir()?
        .join(root_toml_path)
        .canonicalize()?
    };

    // Verify that toml file exists
    let mut root_toml = fs::File::open(&abs_root_toml_path)
      .map_err(|e| MetadataError::RootToml(e.into(), abs_root_toml_path.clone()))?;
    let mut toml_contents = String::new();
    root_toml.read_to_string(&mut toml_contents)?;
    let cargo_toml = toml::from_str::<CargoToml>(&toml_contents)?;

    let globs = cargo_toml.workspace
      .map(|w| w.members)
      .unwrap_or_default()
      .iter()
      .map(|member_glob| {
        let dir_glob = abs_root_toml_path
          .parent()
          .unwrap() // CHECKED: Toml must live in a dir
          .join(member_glob)
          .join(".");
          // .join("Cargo.toml");
        let tomls = glob::glob(dir_glob.to_str().unwrap())
          .unwrap()
          .filter_map(Result::ok)
          .map(|d| d.join("Cargo.toml"))
          .collect::<Vec<PathBuf>>();
        if tomls.is_empty() {
          Err(MetadataError::RootToml(Error::msg("Glob didn't match anything"), PathBuf::from(member_glob)).into())
        } else {
          Ok(tomls)
        }
      })
      .collect::<Result<Vec<Vec<PathBuf>>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<PathBuf>>();

    let member_toml_paths = globs
      .iter()
      .map(|m| {
        m.canonicalize()
          .map_err(Error::from)
          .and_then(|p| p.strip_prefix(abs_root_toml_path.parent().unwrap()).map(PathBuf::from).map_err(Error::from))
          .map_err(|e| MetadataError::RootToml(e.into(), PathBuf::from(m)).into())
      })
      .collect::<Result<Vec<PathBuf>>>()?;

    // Try to find an associated lock file
    let mut abs_lock_path_opt = None;
    if infer_lock_file {
      let expected_abs_lock_path = abs_root_toml_path
        .parent()
        .unwrap() // CHECKED: Toml must live in a dir
        .join("Cargo.lock");

      if fs::File::open(&expected_abs_lock_path).is_ok() {
        abs_lock_path_opt = Some(expected_abs_lock_path);
      }
    }

    Ok(CargoWorkspaceFiles {
      root_toml_path: abs_root_toml_path,
      member_toml_paths: member_toml_paths,
      lock_path_opt: abs_lock_path_opt,
    })
  }
}

/** A workspace metadata fetcher that uses the Cargo Metadata subcommand. */
pub struct CargoMetadataFetcher {
  cargo_bin_path: PathBuf,
}

impl CargoMetadataFetcher {
  pub fn new<P: Into<PathBuf>>(cargo_bin_path: P) -> CargoMetadataFetcher {
    CargoMetadataFetcher {
      cargo_bin_path: cargo_bin_path.into(),
    }
  }
}

impl Default for CargoMetadataFetcher {
  fn default() -> CargoMetadataFetcher {
    CargoMetadataFetcher::new(SYSTEM_CARGO_BIN_PATH)
  }
}

impl MetadataFetcher for CargoMetadataFetcher {
  fn fetch_metadata(&mut self, files: &CargoWorkspaceFiles) -> Result<Metadata> {
    assert!(files.root_toml_path.is_file());
    assert!(files.lock_path_opt.as_ref().map_or(true, |p| p.is_file()));

    // Copy files into a temp directory
    // UNWRAP: Guarded by function assertion
    // TODO: Figure out why this is needed
    let cargo_tempdir = {
      let dir = TempDir::new("cargo_raze_metadata_dir")?;

      let dir_path = dir.path();
      let new_root_toml_path = dir_path.join(files.root_toml_path.file_name().unwrap());
      fs::copy(files.root_toml_path.as_path(), new_root_toml_path)?;
      for member in &files.member_toml_paths {
        let abs_member_toml_path = files.root_toml_path.parent().unwrap().join(member);
        let new_member_toml_path = dir_path.join(member);
        println!("Creating dirs for {:?}", new_member_toml_path);
        fs::create_dir_all(new_member_toml_path.parent().unwrap())?;
        fs::copy(abs_member_toml_path, new_member_toml_path)?;
      }
      if let Some(lock_path) = files.lock_path_opt.as_ref() {
        let new_lock_path = dir_path.join(lock_path.file_name().unwrap());
        fs::copy(lock_path.as_path(), new_lock_path)?;
      }

      dir
    };

    MetadataCommand::new()
      .cargo_path(&self.cargo_bin_path)
      // .current_dir(cargo_tempdir.path())
      .current_dir(files.root_toml_path.parent().unwrap())
      .exec()
      .map_err(|e| e.into())
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
    let root_toml_path = dir.path().join("Cargo.toml");
    let mut toml = File::create(&root_toml_path).unwrap();
    toml.write_all(basic_toml().as_bytes()).unwrap();
    let files = CargoWorkspaceFiles {
      lock_path_opt: None,
      root_toml_path,
      member_toml_paths: vec![],
    };

    let mut fetcher = CargoMetadataFetcher::default();

    fetcher.fetch_metadata(&files).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_with_lock() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let root_toml_path = {
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
      root_toml_path,
      member_toml_paths: vec![],
    };

    let mut fetcher = CargoMetadataFetcher::default();

    fetcher.fetch_metadata(&files).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_handles_bad_files() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let root_toml_path = {
      let path = dir.path().join("Cargo.toml");
      let mut toml = File::create(&path).unwrap();
      toml.write_all(b"hello").unwrap();
      path
    };
    let files = CargoWorkspaceFiles {
      lock_path_opt: None,
      root_toml_path,
      member_toml_paths: vec![],
    };

    let mut fetcher = CargoMetadataFetcher::default();
    assert!(fetcher.fetch_metadata(&files).is_err());
  }
}
