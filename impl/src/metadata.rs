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

use anyhow::{anyhow, Context, Result};
use cargo_lock::Lockfile;
use cargo_metadata::{Metadata, MetadataCommand};
use glob::glob;
use pathdiff::diff_paths;
use regex::Regex;
use rustc_serialize::hex::ToHex;
use tempfile::TempDir;
use url::Url;

const SYSTEM_CARGO_BIN_PATH: &str = "cargo";
pub(crate) const DEFAULT_CRATE_REGISTRY_URL: &str = "https://crates.io";
pub(crate) const DEFAULT_CRATE_INDEX_URL: &str = "https://github.com/rust-lang/crates.io-index";

/** A struct containing all metadata about a project with which to plan generated output files for */
#[derive(Debug, Clone)]
pub struct RazeMetadata {
  // `cargo metadata` output of the current project
  pub metadata: Metadata,

  // The absolute path to the current project's cargo workspace root
  pub workspace_root: PathBuf,

  // The metadata of a lockfile that was generated as a result of fetching metadata
  pub lockfile: Option<Lockfile>,

  // A map of all known crates with checksums
  checksums: HashMap<String, String>,
}

impl RazeMetadata {
  pub fn new(
    metadata: Metadata,
    checksums: HashMap<String, String>,
    workspace_root: PathBuf,
    lockfile: Option<Lockfile>,
  ) -> RazeMetadata {
    RazeMetadata {
      metadata,
      checksums,
      workspace_root,
      lockfile,
    }
  }

  /** Get the checksum of a crate using a unique formatter. */
  pub fn get_checksum(&self, name: &str, version: &str) -> Option<&String> {
    self.checksums.get(&format!("{}-{}", name, version))
  }
}

/** A struct containing information about a binary dependency */
pub struct BinaryDependencyInfo {
  pub name: String,
  pub info: cargo_toml::Dependency,
  pub lockfile: Option<PathBuf>,
}

/**
 * An entity that can retrive deserialized metadata for a Cargo Workspace.
 *
 * Usage of ..Subcommand.. is waiting on a cargo release containing
 * <https://github.com/rust-lang/cargo/pull/5122>
 */
pub trait MetadataFetcher {
  fn fetch_metadata(
    &self,
    files: &CargoWorkspaceFiles,
    binary_dep_info: Option<&HashMap<String, cargo_toml::Dependency>>,
    override_lockfile: Option<PathBuf>,
  ) -> Result<RazeMetadata>;
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
  registry_url: Url,
  index_url: String,
}

impl CargoMetadataFetcher {
  pub fn new<P: Into<PathBuf>>(
    cargo_bin_path: P,
    registry_url: Url,
    index_url: &str,
  ) -> CargoMetadataFetcher {
    CargoMetadataFetcher {
      cargo_bin_path: cargo_bin_path.into(),
      registry_url: registry_url.clone(),
      index_url: String::from(index_url),
    }
  }

  // Create a symlink file on unix systems
  #[cfg(target_family = "unix")]
  fn make_symlink(&self, src: &Path, dest: &Path) -> Result<()> {
    std::os::unix::fs::symlink(src, dest)
      .with_context(|| "Failed to create symlink for generating metadata")
  }

  // Create a symlink file on windows systems
  #[cfg(target_family = "windows")]
  fn make_symlink(&self, src: &Path, dest: &Path) -> Result<()> {
    std::os::windows::fs::symlink_file(dest, src)
      .with_context(|| "Failed to create symlink for generating metadata")
  }

  /** Creates a copy workspace in a temporary directory for fetching the metadata of the current workspace */
  fn make_temp_workspace(&self, files: &CargoWorkspaceFiles) -> Result<(TempDir, PathBuf)> {
    let temp_dir = TempDir::new()?;
    assert!(files.toml_path.is_file());
    assert!(files.lock_path_opt.as_ref().map_or(true, |p| p.is_file()));

    // First gather metadata without downloading any dependencies so we can identify any path dependencies.
    let no_deps_metadata = MetadataCommand::new()
      .cargo_path(&self.cargo_bin_path)
      .current_dir(files.toml_path.parent().unwrap()) // UNWRAP: Guarded by function assertion
      .no_deps()
      .exec()?;

    // There should be a `Cargo.toml` file in the workspace root
    fs::copy(
      no_deps_metadata.workspace_root.join("Cargo.toml"),
      temp_dir.as_ref().join("Cargo.toml"),
    )?;

    // Optionally copy over the lock file
    if no_deps_metadata.workspace_root.join("Cargo.lock").exists() {
      fs::copy(
        no_deps_metadata.workspace_root.join("Cargo.lock"),
        temp_dir.as_ref().join("Cargo.lock"),
      )?;
    }

    // Copy over the Cargo.toml files of each workspace member
    let re = Regex::new(r".+\(path\+file://(.+)\)")?;
    for member in no_deps_metadata.workspace_members.iter() {
      // Get a path to the workspace member directory
      let path = {
        let capture = match re.captures(&member.repr) {
          Some(capture) => capture,
          None => continue,
        };

        let re_match = match capture.get(1) {
          Some(re_match) => re_match,
          None => continue,
        };

        PathBuf::from(re_match.as_str())
      };

      let toml_path = path.join("Cargo.toml");

      // If no Cargo.toml file is found, skip this
      if !toml_path.exists() {
        continue;
      }

      // Copy the Cargo.toml files into the temp directory to match the directory structure on disk
      let diff = diff_paths(&path, &no_deps_metadata.workspace_root).ok_or(anyhow!(
        "All workspace memebers are expected to be under the workspace root"
      ))?;
      let new_path = temp_dir.as_ref().join(diff);
      fs::create_dir_all(&new_path)?;
      fs::copy(path.join("Cargo.toml"), new_path.join("Cargo.toml"))?;

      // Additionally, symlink everything in some common source directories to ensure specified
      // library targets can be relied on and won't prevent fetching metadata
      for dir in vec!["bin", "src"].iter() {
        let glob_pattern = format!("{}/**/*.rs", path.join(dir).display());
        for entry in glob(glob_pattern.as_str()).expect("Failed to read glob pattern") {
          let path = entry?;

          // Determine the difference between the workspace root and the current file
          let diff = diff_paths(&path, &no_deps_metadata.workspace_root).ok_or(anyhow!(
            "All workspace memebers are expected to be under the workspace root"
          ))?;

          // Create a matching directory tree for the current file within the temp workspace
          let new_path = temp_dir.as_ref().join(diff);
          if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
          }

          self.make_symlink(&path, &new_path)?;
        }
      }
    }

    Ok((temp_dir, no_deps_metadata.workspace_root))
  }

  /** Download a crate's source code from the current registry url */
  fn fetch_crate_src(&self, dir: &Path, name: &str, version: &str) -> Result<PathBuf> {
    // The registry url should only be the host URL with ports. No path
    let registry_url = {
      let mut r_url = self.registry_url.clone();
      r_url.set_path("");
      r_url.to_string()
    };

    let mut url = url::Url::parse(&registry_url)?;
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

  /** Add binary dependencies as workspace members to the given workspace root Cargo.toml file */
  fn inject_binaries_into_workspace(
    &self,
    binary_deps: Vec<String>,
    root_toml: &Path,
  ) -> Result<()> {
    // Read the current manifest
    let mut manifest = {
      let content = fs::read_to_string(root_toml)?;
      cargo_toml::Manifest::from_str(content.as_str())?
    };

    // Parse the current `workspace` section of the manifest if one exists
    let mut workspace = match manifest.workspace {
      Some(workspace) => workspace,
      None => cargo_toml::Workspace::default(),
    };

    // Add the binary dependencies as workspace members to the `workspace` metadata
    for dep in binary_deps.iter() {
      workspace.members.push(dep.to_string());
    }

    // Replace the workspace metadata with the modified metadata
    manifest.workspace = Some(workspace);

    // Write the metadata back to disk.
    // cargo_toml::Manifest cannot be serialized direcly.
    // see: https://gitlab.com/crates.rs/cargo_toml/-/issues/3
    let value = toml::Value::try_from(&manifest)?;
    std::fs::write(root_toml, toml::to_string(&value)?).with_context(|| {
      format!(
        "Failed to inject workspace metadata to {}",
        root_toml.display()
      )
    })
  }

  /** Look up a crate in a specified crate index to determine it's checksum */
  fn fetch_crate_checksum(&self, name: &str, version: &str) -> Result<String> {
    let index_path_is_url = url::Url::parse(&self.index_url).is_ok();
    let crate_index_path = if index_path_is_url {
      crates_index::BareIndex::from_url(&self.index_url)?
        .open_or_clone()?
        .crate_(name)
        .ok_or(anyhow!("Failed to find crate '{}' in index", name))?
    } else {
      crates_index::Index::new(&self.index_url)
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

  /**
   * Ensures a lockfile is generated for a crate on disk
   *
   * If a lockfile for the current crate exists in the `raze` lockfiles output directory, that
   * lockfile will instead be installed into the crate's directory. If no lockfile exists, one
   * will be generated.
   */
  fn cargo_generate_lockfile(
    &self,
    override_lockfile: &Option<PathBuf>,
    cargo_dir: &Path,
  ) -> Result<Lockfile> {
    let lockfile_path = cargo_dir.join("Cargo.lock");

    if let Some(override_lockfile) = override_lockfile {
      fs::copy(&override_lockfile, &lockfile_path)?;
    } else {
      std::process::Command::new(SYSTEM_CARGO_BIN_PATH)
        .arg("generate-lockfile")
        .current_dir(&cargo_dir)
        .output()?;
    }

    Lockfile::load(&lockfile_path)
      .with_context(|| format!("Failed to load lockfile: {}", lockfile_path.display()))
  }
}

impl Default for CargoMetadataFetcher {
  fn default() -> CargoMetadataFetcher {
    CargoMetadataFetcher::new(
      SYSTEM_CARGO_BIN_PATH,
      // UNWRAP: The default is covered by testing and should never return err
      Url::parse(DEFAULT_CRATE_REGISTRY_URL).unwrap(),
      DEFAULT_CRATE_INDEX_URL,
    )
  }
}

impl MetadataFetcher for CargoMetadataFetcher {
  fn fetch_metadata(
    &self,
    files: &CargoWorkspaceFiles,
    binary_dep_info: Option<&HashMap<String, cargo_toml::Dependency>>,
    override_lockfile: Option<PathBuf>,
  ) -> Result<RazeMetadata> {
    let (cargo_dir, workspace_root) = self.make_temp_workspace(files)?;
    let cargo_root_toml = cargo_dir.as_ref().join("Cargo.toml");

    let mut checksums: HashMap<String, String> = HashMap::new();
    let mut output_lockfile = None;

    // Gather new lockfile data if any binary dependencies were provided
    if let Some(binary_dep_info) = binary_dep_info {
      if !binary_dep_info.is_empty() {
        let mut src_dirnames: Vec<String> = Vec::new();

        for (name, info) in binary_dep_info.iter() {
          let version = info.req().to_string();
          let src_dir = self.fetch_crate_src(cargo_dir.as_ref(), &name, &version)?;
          checksums.insert(
            format!("{}-{}", &name, &version),
            self.fetch_crate_checksum(&name, &version)?,
          );
          if let Some(dirname) = src_dir.file_name() {
            if let Some(dirname_str) = dirname.to_str() {
              src_dirnames.push(dirname_str.to_string());
            }
          }
        }

        self.inject_binaries_into_workspace(src_dirnames, &cargo_root_toml)?;

        // Potentially generate a new lockfile
        output_lockfile =
          Some(self.cargo_generate_lockfile(&override_lockfile, cargo_dir.as_ref())?);
      }
    }

    // Load checksums from the lockfile
    let workspace_toml_lock = cargo_dir.as_ref().join("Cargo.lock");
    if workspace_toml_lock.exists() {
      let lockfile = Lockfile::load(workspace_toml_lock)?;
      for package in &lockfile.packages {
        if let Some(checksum) = &package.checksum {
          checksums.insert(
            format!(
              "{}-{}",
              package.name.to_string(),
              package.version.to_string()
            ),
            checksum.to_string(),
          );
        }
      }
    }

    let metadata = MetadataCommand::new()
      .cargo_path(&self.cargo_bin_path)
      .current_dir(cargo_dir.as_ref())
      .exec()?;

    Ok(RazeMetadata {
      metadata,
      checksums,
      workspace_root,
      lockfile: output_lockfile,
    })
  }
}

#[cfg(test)]
pub mod tests {
  use httpmock::MockServer;

  use super::*;
  use crate::testing::*;

  use std::{fs::File, io::Write};

  pub fn dummy_raze_metadata_fetcher() -> (CargoMetadataFetcher, MockServer, TempDir) {
    let tempdir = TempDir::new().unwrap();
    let mock_server = MockServer::start();
    (
      CargoMetadataFetcher::new(
        SYSTEM_CARGO_BIN_PATH,
        Url::parse(&mock_server.base_url()).unwrap(),
        tempdir.as_ref().display().to_string().as_str(),
      ),
      mock_server,
      tempdir,
    )
  }

  pub fn dummy_raze_metadata() -> RazeMetadata {
    let (_dir, files) = make_basic_workspace();
    let (fetcher, _server, _index_dir) = dummy_raze_metadata_fetcher();
    fetcher.fetch_metadata(&files, None, None).unwrap()
  }

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
      .fetch_metadata(&files, None, None)
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
      .fetch_metadata(&files, None, None)
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
    assert!(fetcher.fetch_metadata(&files, None, None).is_err());
  }

  #[test]
  fn test_fetching_src() {
    let (fetcher, mock_server, _index_url) = dummy_raze_metadata_fetcher();
    let mock = mock_remote_crate("fake-crate", "3.3.3", &mock_server);

    let path = fetcher
      .fetch_crate_src(mock.data_dir.as_ref(), "fake-crate", "3.3.3")
      .unwrap();

    for mock in mock.endpoints.iter() {
      mock.assert();
    }

    assert!(path.exists());

    // Ensure the name follows a consistent pattern: `{name}-{version}`
    assert_eq!(
      mock.data_dir.into_path().join("fake-crate-3.3.3").as_path(),
      path.as_path()
    );
    assert!(path.join("Cargo.toml").exists());
    assert!(path.join("Cargo.lock").exists());
    assert!(path.join("test").exists());
  }

  #[test]
  fn test_inject_dependency_to_workspace() {
    let (fetcher, _mock_server, _index_url) = dummy_raze_metadata_fetcher();

    let (crate_dir, files) = make_workspace_with_dependency();
    let mut manifest =
      cargo_toml::Manifest::from_str(fs::read_to_string(&files.toml_path).unwrap().as_str())
        .unwrap();

    let basic_dep_toml = crate_dir.as_ref().join("basic_dep/Cargo.toml");
    fs::create_dir_all(basic_dep_toml.parent().unwrap()).unwrap();
    fs::write(&basic_dep_toml, named_toml_contents("basic_dep", "0.0.1")).unwrap();
    assert!(basic_dep_toml.exists());

    manifest.workspace = Some({
      let mut workspace = cargo_toml::Workspace::default();
      workspace.members.push("test".to_string());
      workspace
    });

    // Ensure the manifest only includes the new workspace member after the injection
    assert_ne!(
      cargo_toml::Manifest::from_str(fs::read_to_string(&files.toml_path).unwrap().as_str())
        .unwrap(),
      manifest
    );

    // Fetch metadata
    fetcher
      .inject_binaries_into_workspace(vec!["test".to_string()], &files.toml_path)
      .unwrap();

    // Ensure workspace now has the new member
    assert_eq!(
      cargo_toml::Manifest::from_str(fs::read_to_string(&files.toml_path).unwrap().as_str())
        .unwrap(),
      manifest
    );
  }

  #[test]
  fn test_generate_lockfile_use_previously_generated() {
    let (fetcher, _mock_server, _index_url) = dummy_raze_metadata_fetcher();

    let (crate_dir, files) = make_workspace_with_dependency();
    let override_lockfile = crate_dir.as_ref().join("locks_test/Cargo.raze.lock");

    fs::create_dir_all(override_lockfile.parent().unwrap()).unwrap();
    fs::write(&override_lockfile, "# test_generate_lockfile").unwrap();

    // Returns the built in lockfile
    assert_eq!(
      fetcher
        .cargo_generate_lockfile(&Some(override_lockfile), crate_dir.as_ref(),)
        .unwrap(),
      cargo_lock::Lockfile::load(files.lock_path_opt.unwrap()).unwrap(),
    );
  }

  #[test]
  fn test_cargo_generate_lockfile_new_file() {
    let (fetcher, _mock_server, _index_url) = dummy_raze_metadata_fetcher();

    let (crate_dir, _files) = make_workspace(advanced_toml(), None);
    let expected_lockfile = crate_dir.as_ref().join("expected/Cargo.expected.lock");

    fs::create_dir_all(expected_lockfile.parent().unwrap()).unwrap();
    fs::write(&expected_lockfile, advanced_lock()).unwrap();

    // Returns a newly generated lockfile
    assert_eq!(
      fetcher
        .cargo_generate_lockfile(&None, crate_dir.as_ref(),)
        .unwrap(),
      cargo_lock::Lockfile::load(&expected_lockfile).unwrap()
    );
  }
}
