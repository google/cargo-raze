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

use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum GenMode {
  /** Generate Vendored-style dependencies.
   *
   * This mode assumes that files are vendored (into vendor/), and generates BUILD files
   * accordingly
   */
  Vendored,

  /**
   * Generate Remote-style dependencies.
   *
   * This mode assumes that files are not locally vendored, and generates a workspace-level
   * function that can bring them in.
   */
  Remote,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CargoToml {
  /** The raze settings (the only part of the Cargo.toml we care about. */
  pub raze: RazeSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RazeSettings {
  /** The path to the Cargo.toml working directory. */
  pub workspace_path: String,

  /** The platform target to generate BUILD rules for.
   *
   * This comes in the form of a "triple", such as "x86_64-unknown-linux-gnu"
   */
  #[serde(default = "default_target")]
  pub target: String,

  /** Any crate-specific configuration. See CrateSetings for details. */
  #[serde(default)]
  pub crates: HashMap<String, HashMap<String, CrateSettings>>,

  /**
   * Prefix for generated Bazel workspaces (from workspace_rules)
   *
   * This is only useful with remote genmode. It prefixes the names of the workspaces for
   * dependencies (@PREFIX_crateName_crateVersion) as well as the name of the repository function
   * generated in crates.bzl (PREFIX_fetch_remote_crates()).
   *
   * TODO(acmcarther): Does this have a non-bazel analogue?
   */
  #[serde(default = "default_gen_workspace_prefix")]
  pub gen_workspace_prefix: String,

  /** How to generate the dependencies. See GenMode for details. */
  #[serde(default = "default_genmode")]
  pub genmode: GenMode,
}

fn default_target() -> String {
  "x86_64-unknown-linux-gnu".to_owned()
}

fn default_gen_workspace_prefix() -> String {
  "raze".to_owned()
}

fn default_genmode() -> GenMode {
  GenMode::Vendored
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrateSettings {
  /**
   * Dependencies to be added to a crate.
   *
   * Importantly, the format of dependency references depends on the genmode.
   * Remote: @{gen_workspace_prefix}__{dep_name}__{dep_version_sanitized}/:{dep_name}
   * Vendored: //{workspace_path}/vendor/{dep_name}-{dep_version}:{dep_name}
   *
   * In addition, the added deps must be accessible from a remote workspace under Remote GenMode.
   * Usually, this means they _also_ need to be remote, but a "local" build path prefixed with
   * "@", in the form "@//something_local" may work.
   */
  #[serde(default)]
  pub additional_deps: Vec<String>,

  /**
   * Dependencies to be removed from a crate, in the form "//etc".
   *
   * Importantly, the format of dependency references depends on the gen mode.
   * Remote: @{gen_workspace_prefix}__{dep_name}__{dep_version_sanitized}/:{dep_name}
   * Vendored: //{workspace_path}/vendor/{dep_name}-{dep_version}:{dep_name}
   */
  #[serde(default)]
  pub skipped_deps: Vec<String>,

  /**
   * Library targets that should be aliased in the root BUILD file.
   *
   * This is useful to facilitate using binary utility crates, such as bindgen, as part of genrules.
   */
  #[serde(default)]
  pub extra_aliased_targets: Vec<String>,

  /** Flags to be added to the crate compilation process, in the form "--flag". */
  #[serde(default)]
  pub additional_flags: Vec<String>,

  /**
   * Whether or not to generate the build script that goes with this crate.
   *
   * Many build scripts will not function, as they will still be built hermetically. However, build
   * scripts that merely generate files into OUT_DIR may be fully functional.
   */
  #[serde(default = "default_gen_buildrs")]
  pub gen_buildrs: bool,
}

fn default_gen_buildrs() -> bool {
  false
}
