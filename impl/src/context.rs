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
  collections::{BTreeMap, BTreeSet},
  path::PathBuf,
};

use crate::{features::Features, settings::CrateSettings};
use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

/// A struct containing information about a crate's dependency that's buildable in Bazel
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BuildableDependency {
  // Note: Buildifier-compliant BUILD file generation depends on correct sorting of collections
  // of this struct by `buildable_target`. Do not add fields preceding this field.
  pub buildable_target: String,
  pub name: String,
  pub version: Version,
  pub is_proc_macro: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DependencyAlias {
  pub target: String,
  pub alias: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BuildableTarget {
  pub kind: String,
  pub name: String,

  /// The path in Bazel's format (i.e. with forward slashes) to the target's entry point.
  pub path: String,
  pub edition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Metadep {
  pub name: String,
  pub min_version: Version,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LicenseData {
  pub name: String,
  pub rating: String,
}

impl Default for LicenseData {
  fn default() -> Self {
    LicenseData {
      name: "no license".into(),
      rating: "restricted".into(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GitRepo {
  pub remote: String,
  pub commit: String,

  // Directory containing the crate's Cargo.toml file, relative to the git repo root.
  // Will be None iff the crate lives at the root of the git repo.
  pub path_to_crate_root: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SourceDetails {
  pub git_data: Option<GitRepo>,
  pub download_url: Option<Url>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateDependencyContext {
  pub dependencies: BTreeSet<BuildableDependency>,
  pub proc_macro_dependencies: BTreeSet<BuildableDependency>,
  // data_dependencies can only be set when using cargo-raze as a library at the moment.
  pub data_dependencies: BTreeSet<BuildableDependency>,
  pub build_dependencies: BTreeSet<BuildableDependency>,
  pub build_proc_macro_dependencies: BTreeSet<BuildableDependency>,
  // build_data_dependencies can only be set when using cargo-raze as a library at the moment.
  pub build_data_dependencies: BTreeSet<BuildableDependency>,
  pub build_tools_dependencies: BTreeSet<BuildableDependency>,
  pub dev_dependencies: BTreeSet<BuildableDependency>,
  /// Aliased dependencies, sorted/keyed by their `target` name in the `DependencyAlias` struct.
  pub aliased_dependencies: BTreeMap<String, DependencyAlias>,
}

impl CrateDependencyContext {
  pub fn contains(&self, name: &str, version: Version) -> bool {
    let condition = |dep: &BuildableDependency| dep.name.eq(&name) && dep.version.eq(&version);
    self.dependencies.iter().any(condition)
      || self.proc_macro_dependencies.iter().any(condition)
      || self.build_dependencies.iter().any(condition)
      || self.build_proc_macro_dependencies.iter().any(condition)
      || self.dev_dependencies.iter().any(condition)
  }

  pub fn subtract(&mut self, other: &CrateDependencyContext) {
    self.dependencies = self
      .dependencies
      .difference(&other.dependencies)
      .cloned()
      .collect();
    self.proc_macro_dependencies = self
      .proc_macro_dependencies
      .difference(&other.proc_macro_dependencies)
      .cloned()
      .collect();
    self.build_dependencies = self
      .build_dependencies
      .difference(&other.build_dependencies)
      .cloned()
      .collect();
    self.build_proc_macro_dependencies = self
      .build_proc_macro_dependencies
      .difference(&other.build_proc_macro_dependencies)
      .cloned()
      .collect();
    self.dev_dependencies = self
      .dev_dependencies
      .difference(&other.dev_dependencies)
      .cloned()
      .collect();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateTargetedDepContext {
  pub target: String,
  pub deps: CrateDependencyContext,
  pub platform_targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: Version,
  pub edition: String,
  pub raze_settings: CrateSettings,
  pub canonical_additional_build_file: Option<PathBuf>,
  pub default_deps: CrateDependencyContext,
  pub targeted_deps: Vec<CrateTargetedDepContext>,
  pub license: LicenseData,
  pub features: Features,
  pub workspace_path_to_crate: String,
  pub workspace_member_dependents: Vec<PathBuf>,
  pub workspace_member_dev_dependents: Vec<PathBuf>,
  pub workspace_member_build_dependents: Vec<PathBuf>,
  pub is_workspace_member_dependency: bool,
  pub is_binary_dependency: bool,
  pub targets: Vec<BuildableTarget>,
  pub build_script_target: Option<BuildableTarget>,
  pub links: Option<String>,
  pub source_details: SourceDetails,
  pub sha256: Option<String>,

  // TODO(acmcarther): This is used internally by renderer to know where to put the build file. It
  // probably should live somewhere else. Renderer params (separate from context) should live
  // somewhere more explicit.
  //
  // I'm punting on this now because this requires a more serious look at the renderer code.
  pub expected_build_path: String,

  // The name of the main lib target for this crate (if present).
  // Currently only one such lib can exist per crate.
  pub lib_target_name: Option<String>,
  // This field tracks whether or not the lib target of `lib_target_name`
  // is a proc_macro library or not.
  pub is_proc_macro: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  // The bazel path prefix to the vendor directory
  pub workspace_path: String,

  // The generated new_http_library Bazel workspace prefix.
  //
  // This has no effect unless the GenMode setting is Remote.
  pub gen_workspace_prefix: String,

  // The file extension of generated BUILD files.
  //
  // Bare files will just be named after this setting. Named files, such as those passed to
  // repository rules, will take the form of $prefix.$this_value.
  pub output_buildfile_suffix: String,

  // A list of relative paths from a Cargo workspace root to a Cargo package.
  pub workspace_members: Vec<PathBuf>,
}
