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
  collections::{HashMap, HashSet},
  path::PathBuf,
  str::{self},
};

use anyhow::{anyhow, Result};

use crate::{
  error::RazeError,
  metadata::{Metadata, Package, PackageId},
  settings::{GenMode, RazeSettings},
  util::{self, find_bazel_workspace_root},
};

pub const VENDOR_DIR: &str = "vendor/";
/** An entry in the Crate catalog for a single crate. */
pub struct CrateCatalogEntry {
  // The package metadata for the crate
  pub package: Package,
  // The name of the package sanitized for use within Bazel
  pub sanitized_name: String,
  // The version of the package sanitized for use within Bazel
  pub sanitized_version: String,
  // A unique identifier for the package derived from Cargo usage of the form {name}-{version}
  pub package_ident: String,
  // Is this the root crate in the whole catalog?
  pub is_root: bool,
  // Is this a dependency of the catalog root crate?
  pub is_root_dep: bool,
  // Is this a member of the root crate workspace?
  pub is_workspace_crate: bool,
}

impl CrateCatalogEntry {
  pub fn new(
    package: &Package,
    is_root: bool,
    is_root_dep: bool,
    is_workspace_crate: bool,
  ) -> Self {
    let sanitized_name = package.name.replace("-", "_");
    let sanitized_version = util::sanitize_ident(&package.version.clone().to_string());

    Self {
      package: package.clone(),
      package_ident: format!("{}-{}", &package.name, &package.version),
      sanitized_name,
      sanitized_version,
      is_root,
      is_root_dep,
      is_workspace_crate,
    }
  }

  /** Yields the name of the default target for this crate (sanitized). */
  #[allow(dead_code)]
  pub fn default_build_target_name(&self) -> &str {
    &self.sanitized_name
  }

  /** Returns a reference to the contained package. */
  pub fn package(&self) -> &Package {
    &self.package
  }

  /** Returns whether or not this is the root crate in the workspace. */
  pub fn is_root(&self) -> bool {
    self.is_root
  }

  /** Returns whether or not this is a member of the root workspace. */
  pub fn is_workspace_crate(&self) -> bool {
    self.is_workspace_crate
  }

  /** Returns whether or not this is a dependency of the workspace root crate.*/
  pub fn is_root_dep(&self) -> bool {
    self.is_root_dep
  }

  /**
   * Returns the packages expected path during current execution.
   *
   * Not for use except during planning as path is local to run location.
   */
  pub fn expected_vendored_path(&self, workspace_path: &str) -> String {
    let mut dir = find_bazel_workspace_root().unwrap_or(PathBuf::from("."));

    // Trim the absolute label identifier from the start of the workspace path
    dir.push(workspace_path.trim_start_matches('/'));

    dir.push(VENDOR_DIR);
    dir.push(&self.package_ident);

    return dir.display().to_string();
  }

  /** Yields the expected location of the build file (relative to execution path). */
  pub fn local_build_path(&self, settings: &RazeSettings) -> Result<String> {
    match settings.genmode {
      GenMode::Remote => Ok(format!("remote/BUILD.{}.bazel", &self.package_ident,)),
      GenMode::Vendored => Ok(format!(
        "vendor/{}/{}",
        &self.package_ident, settings.output_buildfile_suffix,
      )),
      // Settings should always have `genmode` set to one of the above fields
      GenMode::Unspecified => Err(anyhow!(
        "Unable to determine local build path. GenMode should not be Unspecified"
      )),
    }
  }

  /** Yields the precise path to this dependency for the provided settings. */
  pub fn workspace_path(&self, settings: &RazeSettings) -> Result<String> {
    match settings.genmode {
      GenMode::Remote => Ok(format!(
        "@{}__{}__{}//",
        &settings.gen_workspace_prefix, &self.sanitized_name, &self.sanitized_version
      )),
      GenMode::Vendored => {
        // Convert "settings.workspace_path" to dir. Workspace roots are special cased, no need to append /
        if settings.workspace_path.ends_with("//") {
          Ok(format!(
            "{}vendor/{}",
            settings.workspace_path, &self.package_ident
          ))
        } else {
          Ok(format!(
            "{}/vendor/{}",
            settings.workspace_path, &self.package_ident
          ))
        }
      },
      GenMode::Unspecified => Err(anyhow!(
        "Unable to determine workspace path for GenMode::Unspecified"
      )),
    }
  }

  /** Emits a complete path to this dependency and default target using the given settings. */
  pub fn workspace_path_and_default_target(&self, settings: &RazeSettings) -> Result<String> {
    match settings.genmode {
      GenMode::Remote => Ok(format!(
        "@{}__{}__{}//:{}",
        &settings.gen_workspace_prefix,
        &self.sanitized_name,
        &self.sanitized_version,
        &self.sanitized_name
      )),
      GenMode::Vendored => {
        // Convert "settings.workspace_path" to dir. Workspace roots are special cased, no need to append /
        if settings.workspace_path.ends_with("//") {
          Ok(format!(
            "{}vendor/{}:{}",
            settings.workspace_path, &self.package_ident, &self.sanitized_name
          ))
        } else {
          Ok(format!(
            "{}/vendor/{}:{}",
            settings.workspace_path, &self.package_ident, &self.sanitized_name
          ))
        }
      },
      GenMode::Unspecified => Err(anyhow!(
        "Unable to determine workspace path for GenMode::Unspecified"
      )),
    }
  }
}

/** An intermediate structure that contains details about all crates in the workspace. */
pub struct CrateCatalog {
  pub metadata: Metadata,
  pub entries: Vec<CrateCatalogEntry>,
  pub package_id_to_entries_idx: HashMap<PackageId, usize>,
}

impl CrateCatalog {
  /** Produces a CrateCatalog using the package entries from a metadata blob.*/
  pub fn new(metadata: &Metadata) -> Result<Self> {
    let resolve = metadata
      .resolve
      .as_ref()
      .ok_or_else(|| RazeError::Generic("Missing resolve graph".into()))?;

    let root_resolve_node = {
      let root_id = resolve
        .root
        .as_ref()
        .ok_or_else(|| RazeError::Generic("Missing root in resolve graph".into()))?;

      resolve
        .nodes
        .iter()
        .find(|node| &node.id == root_id)
        .ok_or_else(|| RazeError::Generic("Missing crate with root ID in resolve graph".into()))?
    };

    let root_direct_deps = root_resolve_node
      .dependencies
      .iter()
      .cloned()
      .collect::<HashSet<_>>();
    let workspace_crates = metadata
      .workspace_members
      .iter()
      .cloned()
      .collect::<HashSet<_>>();

    let entries = metadata
      .packages
      .iter()
      .map(|package| {
        CrateCatalogEntry::new(
          package,
          root_resolve_node.id == package.id,
          root_direct_deps.contains(&package.id),
          workspace_crates.contains(&package.id),
        )
      })
      .collect::<Vec<_>>();

    let mut package_id_to_entries_idx = HashMap::new();

    // This loop also ensures there are no duplicates
    for (idx, entry) in entries.iter().enumerate() {
      let existing_value = package_id_to_entries_idx.insert(entry.package.id.clone(), idx);
      assert!(None == existing_value);
    }

    Ok(Self {
      metadata: metadata.clone(),
      entries,
      package_id_to_entries_idx,
    })
  }

  /** Yields the internally contained entry set. */
  pub fn entries(&self) -> &Vec<CrateCatalogEntry> {
    &self.entries
  }

  /** Finds and returns the catalog entry with the given package id if present. */
  pub fn entry_for_package_id(&self, package_id: &PackageId) -> Option<&CrateCatalogEntry> {
    self
      .package_id_to_entries_idx
      .get(package_id)
      // UNWRAP: Indexes guaranteed to be valid -- structure is immutable
      .map(|entry_idx| self.entries.get(*entry_idx).unwrap())
  }
}
