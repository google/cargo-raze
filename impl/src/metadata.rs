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

use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
  string::String,
};

use anyhow::{anyhow, Result};

use cargo_metadata::MetadataCommand;
pub use cargo_metadata::{DependencyKind, Metadata, Node, Package, PackageId};

use tempfile::TempDir;

use rustc_serialize::hex::ToHex;

const SYSTEM_CARGO_BIN_PATH: &str = "cargo";

/**
 * An entity that can retrive deserialized metadata for a Cargo Workspace.
 *
 * Usage of ..Subcommand.. is waiting on a cargo release containing
 * <https://github.com/rust-lang/cargo/pull/5122>
 */
pub trait MetadataFetcher {
  fn fetch_metadata(&self, files: &CargoWorkspaceFiles) -> Result<Metadata>;
}

/** The local Cargo workspace files to be used for build planning .*/
#[derive(Debug, Clone)]
pub struct CargoWorkspaceFiles {
  pub toml_path: PathBuf,
  pub lock_path_opt: Option<PathBuf>,
}

/** A workspace metadata fetcher that uses the Cargo Metadata subcommand. */
pub struct CargoMetadataFetcher {
  cargo_bin_path: PathBuf,
  use_tempdir: bool,
}

impl CargoMetadataFetcher {
  pub fn new<P: Into<PathBuf>>(cargo_bin_path: P, use_tempdir: bool) -> CargoMetadataFetcher {
    CargoMetadataFetcher {
      cargo_bin_path: cargo_bin_path.into(),
      use_tempdir,
    }
  }

  fn clone_crate_src(
    &self,
    registry_url: &str,
    dir: &Path,
    name: &str,
    version: &str,
  ) -> Result<PathBuf> {
    let mut url = url::Url::parse(registry_url)?;
    url.set_path("");

    log::debug!("Cloning binary dependency: {}", &name);
    let mut cloner = cargo_clone::Cloner::new();
    cloner
      .set_registry_url(url.to_string().trim_end_matches("/"))
      .set_out_dir(dir);

    cloner.clone(
      cargo_clone::CloneMethodKind::Crate,
      name,
      Some(version),
      &Vec::new(),
    )?;

    let crate_dir = dir.join(format!("{}-{}", name, version));
    if !crate_dir.exists() {
      return Err(anyhow!("Directory does not exist"));
    }

    Ok(crate_dir)
  }

  pub fn fetch_crate_src(
    &self,
    registry_url: &str,
    dir: &Path,
    lockfile_dir: &Path,
    name: &str,
    version: &str,
  ) -> Result<(PathBuf, CargoWorkspaceFiles)> {
    let src_dir = self.clone_crate_src(registry_url, dir, name, version)?;

    let toml_path: PathBuf = src_dir.join("Cargo.toml");
    let lock_path_opt: PathBuf =
      cargo_generate_lockfile(src_dir.as_path(), lockfile_dir, name, version)?;

    match toml_path.exists() && lock_path_opt.exists() {
      true => Ok((
        src_dir,
        CargoWorkspaceFiles {
          toml_path,
          lock_path_opt: Some(lock_path_opt),
        },
      )),
      false => Err(anyhow!(
        "Cargo files could not be found for '{}' ({})",
        name,
        version
      )),
    }
  }
}

impl Default for CargoMetadataFetcher {
  fn default() -> CargoMetadataFetcher {
    CargoMetadataFetcher::new(SYSTEM_CARGO_BIN_PATH, true)
  }
}

impl MetadataFetcher for CargoMetadataFetcher {
  fn fetch_metadata(&self, files: &CargoWorkspaceFiles) -> Result<Metadata> {
    assert!(files.toml_path.is_file());
    assert!(files.lock_path_opt.as_ref().map_or(true, |p| p.is_file()));

    let tempdir = TempDir::new()?;

    // UNWRAP: Guarded by function assertion
    let cargo_dir = match self.use_tempdir {
      false => files.toml_path.as_path().parent().unwrap(),
      true => {
        let dir_path = tempdir.path();
        let new_toml_path = dir_path.join(files.toml_path.file_name().unwrap());
        fs::copy(files.toml_path.as_path(), new_toml_path)?;
        if let Some(lock_path) = files.lock_path_opt.as_ref() {
          let new_lock_path = dir_path.join(lock_path.file_name().unwrap());
          fs::copy(lock_path.as_path(), new_lock_path)?;
        }

        dir_path
      },
    };

    MetadataCommand::new()
      .cargo_path(&self.cargo_bin_path)
      .current_dir(cargo_dir)
      .exec()
      .map_err(|e| e.into())
  }
}

/**
 * Ensures a lockfile is generated for a crate on disk
 *
 * If a lockfile for the current crate exists in the `raze` lockfiles output directory, that
 * lockfile will instead be installed into the crate's directory. If no lockfile exists, one
 * will be generated.
 */
fn cargo_generate_lockfile(
  crate_dir: &Path,
  lockfile_dir: &Path,
  name: &str,
  version: &str,
) -> Result<PathBuf> {
  let target_lockfile = lockfile_dir.join(format!("Cargo.{}-{}.lock", name, version));
  let bundled_lockfile = crate_dir.join("Cargo.lock");

  // If a lockfile exists from a previous run, use that instead
  if target_lockfile.exists() {
    fs::copy(&target_lockfile, &bundled_lockfile)?;
    return Ok(bundled_lockfile);
  }

  // Generate a new lockfile if one wasn't included in the bundle
  if !bundled_lockfile.exists() {
    std::process::Command::new(SYSTEM_CARGO_BIN_PATH)
      .arg("generate-lockfile")
      .current_dir(&crate_dir)
      .output()?;
  }

  Ok(bundled_lockfile)
}

pub fn fetch_crate_checksum(index_url: &str, name: &str, version: &str) -> Result<String> {
  let index_path_is_url = url::Url::parse(index_url).is_ok();
  let crate_index_path = if index_path_is_url {
    crates_index::BareIndex::from_url(index_url)?
      .open_or_clone()?
      .crate_(name)
      .ok_or(anyhow!("Failed to find crate '{}' in index", name))?
  } else {
    crates_index::Index::new(index_url)
      .crate_(name)
      .ok_or(anyhow!("Failed to find crate '{}' in index", name))?
  };

  let (_index, crate_version) = crate_index_path
    .versions()
    .iter()
    .enumerate()
    .find(|(_, ver)| ver.version() == version)
    .ok_or(anyhow!(
      "Failed to find version {} for crate {}",
      version,
      name
    ))?;

  Ok(crate_version.checksum()[..].to_hex())
}

/** A struct containing information about a binary dependency */
pub struct BinaryDependencyInfo {
  /** A collection of metadata for each binary dependency */
  pub metadata: Vec<Metadata>,

  /** A mapping of names of binary dependencies to their Cargo files */
  pub files: HashMap<String, CargoWorkspaceFiles>,
}

/** Gather catalog and crate file info about binary dependencies */
pub fn gather_binary_dep_info(
  binary_deps: &HashMap<String, cargo_toml::Dependency>,
  registry_url: &str,
  lockfiles_dir: &Path,
  binary_deps_dir: &Path,
) -> Result<BinaryDependencyInfo> {
  let metadata_fetcher = CargoMetadataFetcher::new(SYSTEM_CARGO_BIN_PATH, false);

  // Determine the lockfiles dir so we can either reuse existing lockfiles or output new ones
  let mut bin_crate_files: HashMap<String, CargoWorkspaceFiles> = HashMap::new();
  let mut bin_metadatas: Vec<Metadata> = Vec::new();

  for (package, info) in (binary_deps).into_iter() {
    // Install the package into a temp dir so we can run get it's metadata
    let (_src_dir, bin_workspace_files) = metadata_fetcher.fetch_crate_src(
      registry_url,
      &PathBuf::from(binary_deps_dir),
      &lockfiles_dir,
      &package,
      &info.req().to_string(),
    )?;

    bin_metadatas.push(metadata_fetcher.fetch_metadata(&bin_workspace_files)?);
    bin_crate_files.insert(
      format!("{}-{}", &package, &info.req().to_string()),
      bin_workspace_files,
    );
  }

  Ok(BinaryDependencyInfo {
    metadata: bin_metadatas,
    files: bin_crate_files,
  })
}

#[cfg(test)]
mod tests {
  use httpmock::MockServer;
  use indoc::indoc;

  use super::*;
  use crate::testing::*;

  use std::{fs::File, io::Write};

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_without_lock() {
    let dir = TempDir::new().unwrap();
    let toml_path = dir.path().join("Cargo.toml");
    let mut toml = File::create(&toml_path).unwrap();
    toml.write_all(basic_toml().as_bytes()).unwrap();
    let files = CargoWorkspaceFiles {
      lock_path_opt: None,
      toml_path,
    };

    CargoMetadataFetcher::default()
      .fetch_metadata(&files)
      .unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_with_lock() {
    let dir = TempDir::new().unwrap();
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

    CargoMetadataFetcher::default()
      .fetch_metadata(&files)
      .unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_handles_bad_files() {
    let dir = TempDir::new().unwrap();
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

    let fetcher = CargoMetadataFetcher::default();
    assert!(fetcher.fetch_metadata(&files).is_err());
  }

  #[test]
  fn test_fetching_src() {
    let mock_server = MockServer::start();
    let dir = mock_remote_crate("fake-crate", "3.3.3", &mock_server);
    let lockfile_dir = TempDir::new().unwrap();
    let (path, _files) = CargoMetadataFetcher::default()
      .fetch_crate_src(
        &mock_server.url(""),
        dir.as_ref(),
        lockfile_dir.as_ref(),
        &"fake-crate".to_string(),
        &"3.3.3".to_string(),
      )
      .unwrap();
    assert!(path.exists());

    // Ensure the name follows a consistent pattern: `{name}-{version}`
    assert_eq!(
      dir.into_path().join("fake-crate-3.3.3").as_path(),
      path.as_path()
    );
    assert!(path.join("Cargo.toml").exists());
    assert!(path.join("Cargo.lock").exists());
    assert!(path.join("test").exists());
  }

  #[test]
  fn generate_lockfile_use_previously_generated() {
    let (crate_dir, _files) = make_basic_workspace();
    let lock_dir = TempDir::new().unwrap();

    fs::write(
      lock_dir.as_ref().join("Cargo.test-0.0.1.lock"),
      "# test_generate_lockfile",
    )
    .unwrap();

    // Returns the lockfile from lock_dir
    let lockfile =
      cargo_generate_lockfile(crate_dir.as_ref(), lock_dir.as_ref(), "test", "0.0.1").unwrap();

    assert!(lockfile.exists());
    assert_eq!(
      fs::read_to_string(lockfile).unwrap(),
      "# test_generate_lockfile"
    );
  }

  #[test]
  fn test_cargo_generate_lockfile_use_bundled() {
    let (crate_dir, _files) = make_basic_workspace();

    // Returns the bundled Cargo.lock file
    let lockfile = cargo_generate_lockfile(
      crate_dir.as_ref(),
      TempDir::new().unwrap().as_ref(),
      "test",
      "0.0.1",
    )
    .unwrap();

    assert!(lockfile.exists());
    assert_eq!(fs::read_to_string(lockfile).unwrap(), basic_lock());
  }

  #[test]
  fn cargo_generate_lockfile_new_file() {
    let (crate_dir, files) = make_basic_workspace();

    fs::remove_file(files.lock_path_opt.unwrap()).unwrap();

    // Generates a new lockfile
    let lockfile = cargo_generate_lockfile(
      crate_dir.as_ref(),
      TempDir::new().unwrap().as_ref(),
      "test",
      "0.0.1",
    )
    .unwrap();

    assert!(lockfile.exists());
    assert_eq!(
      fs::read_to_string(lockfile).unwrap(),
      indoc! { r#"
        # This file is automatically @generated by Cargo.
        # It is not intended for manual editing.
        [[package]]
        name = "test"
        version = "0.0.1"
      "# }
    );
  }
}
