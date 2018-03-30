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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildDependency {
  pub name: String,
  pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildTarget {
  pub name: String,
  pub kind: String,
  pub path: String,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GitData {
  pub remote: String,
  pub commit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: String,
  pub licenses: Vec<LicenseData>,
  pub features: Vec<String>,
  pub path: String,
  pub dependencies: Vec<BuildDependency>,
  pub build_dependencies: Vec<BuildDependency>,
  pub dev_dependencies: Vec<BuildDependency>,
  pub is_root_dependency: bool,
  pub metadeps: Vec<Metadep>,
  pub platform_triple: String,
  pub targets: Vec<BuildTarget>,
  pub build_script_target: Option<BuildTarget>,
  pub additional_deps: Vec<String>,
  pub additional_flags: Vec<String>,
  pub extra_aliased_targets: Vec<String>,
  pub data_attr: Option<String>,
  pub git_data: Option<GitData>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  /**
   * The bazel path prefix to the vendor directory
   */
  pub workspace_path: String,

  /**
   * The compilation target triple.
   */
  pub platform_triple: String,

  /**
   * The generated new_http_library Bazel workspace prefix.
   *
   * This has no effect unless the GenMode setting is Remote.
   */
  pub gen_workspace_prefix: String,
}
