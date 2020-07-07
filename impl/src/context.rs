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

use crate::settings::CrateSettings;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildableDependency {
  pub name: String,
  pub version: String,
  pub buildable_target: String,
  pub is_proc_macro: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct DependencyAlias {
  pub target: String,
  pub alias: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildableTarget {
  pub name: String,
  pub kind: String,
  pub path: String,
  pub edition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Metadep {
  pub name: String,
  pub min_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GitRepo {
  pub remote: String,
  pub commit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceDetails {
  pub git_data: Option<GitRepo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: String,
  pub edition: String,
  pub raze_settings: CrateSettings,
  pub license: LicenseData,
  pub features: Vec<String>,
  pub workspace_path_to_crate: String,
  pub dependencies: Vec<BuildableDependency>,
  pub proc_macro_dependencies: Vec<BuildableDependency>,
  pub build_dependencies: Vec<BuildableDependency>,
  pub build_proc_macro_dependencies: Vec<BuildableDependency>,
  pub dev_dependencies: Vec<BuildableDependency>,
  pub aliased_dependencies: Vec<DependencyAlias>,
  pub is_root_dependency: bool,
  pub targets: Vec<BuildableTarget>,
  pub build_script_target: Option<BuildableTarget>,
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  // The bazel path prefix to the vendor directory
  pub workspace_path: String,

  // The compilation target triple.
  pub platform_triple: String,

  // The generated new_http_library Bazel workspace prefix.
  //
  // This has no effect unless the GenMode setting is Remote.
  pub gen_workspace_prefix: String,

  // The file extension of generated BUILD files.
  //
  // Bare files will just be named after this setting. Named files, such as those passed to
  // repository rules, will take the form of $prefix.$this_value.
  pub output_buildfile_suffix: String,
}
