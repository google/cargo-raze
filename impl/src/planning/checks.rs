// Copyright 2020 Google Inc.
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
  env, fs,
};

use anyhow::Result;

use crate::{
  error::RazeError,
  metadata::{Metadata, Package, PackageId},
  planning::{CrateCatalogEntry, VENDOR_DIR},
  settings::CrateSettingsPerVersion,
  util::collect_up_to,
};

// TODO(acmcarther): Consider including a switch to disable limiting
const MAX_DISPLAYED_MISSING_VENDORED_CRATES: usize = 5;
const MAX_DISPLAYED_MISSING_RESOLVE_PACKAGES: usize = 5;

// Verifies that all provided packages are vendored (in VENDOR_DIR relative to CWD)
pub fn check_all_vendored(
  crate_catalog_entries: &[CrateCatalogEntry],
  workspace_path: &str,
) -> Result<()> {
  let missing_package_ident_iter = crate_catalog_entries
    .iter()
    // Root does not need to be vendored -- usually it is a wrapper package.
    .filter(|p| !p.is_root())
    .filter(|p| !p.is_workspace_crate())
    .filter(|p| fs::metadata(p.expected_vendored_path(workspace_path)).is_err())
    .map(|p| p.package_ident.clone());

  let limited_missing_crates = collect_up_to(
    MAX_DISPLAYED_MISSING_VENDORED_CRATES,
    missing_package_ident_iter,
  );

  if limited_missing_crates.is_empty() {
    return Ok(());
  }

  // Oops, missing some crates. Yield a nice message
  let expected_full_path = env::current_dir()
    .unwrap()
    .join(format!("./{}", VENDOR_DIR));

  Err(
    RazeError::Planning {
      dependency_name_opt: None,
      message: format!(
        "Failed to find expected vendored crates in {:?}: {:?}. Did you forget to run cargo \
         vendor?",
        expected_full_path.to_str(),
        limited_missing_crates
      ),
    }
    .into(),
  )
}

pub fn check_resolve_matches_packages(metadata: &Metadata) -> Result<()> {
  let known_package_ids = metadata
    .packages
    .iter()
    .map(|p| p.id.clone())
    .collect::<HashSet<PackageId>>();

  let node_ids_missing_package_decl_iter = metadata
    .resolve
    .as_ref()
    .ok_or_else(|| RazeError::Generic("Missing resolve graph".into()))?
    .nodes
    .iter()
    .filter(|n| !known_package_ids.contains(&n.id))
    .map(|n| n.id.clone());
  let limited_missing_node_ids = collect_up_to(
    MAX_DISPLAYED_MISSING_RESOLVE_PACKAGES,
    node_ids_missing_package_decl_iter,
  );

  if limited_missing_node_ids.is_empty() {
    return Ok(());
  }

  // Oops, missing some package metadata. Yield a nice message
  Err(
    RazeError::Planning {
      dependency_name_opt: None,
      message: format!(
        "Failed to find metadata.packages which were expected from metadata.resolve {:?}. {}",
        limited_missing_node_ids,
        crate::error::PLEASE_FILE_A_BUG
      ),
    }
    .into(),
  )
}

pub fn warn_unused_settings(
  all_crate_settings: &HashMap<String, CrateSettingsPerVersion>,
  all_packages: &[&Package],
) {
  let mut known_versions_per_crate = HashMap::new();
  for &Package {
    ref name,
    ref version,
    ..
  } in all_packages
  {
    known_versions_per_crate
      .entry(name.clone())
      .or_insert_with(HashSet::new)
      .insert(version.clone());
  }

  for (name, settings_per_version) in all_crate_settings {
    if !known_versions_per_crate.contains_key(name) {
      eprintln!(
        "Found unused raze settings for all of {}-{:?}",
        name,
        settings_per_version.keys()
      );
      // No version introspection needed -- no known version of this crate
      continue;
    }

    // UNWRAP: Guarded above
    let all_known_versions = known_versions_per_crate.get(name).unwrap();

    for version in settings_per_version.keys() {
      if !all_known_versions.contains(version) {
        eprintln!(
          "Found unused raze settings for {}-{}, but {:?} were known",
          name, version, all_known_versions
        )
      }
    }
  }
}
