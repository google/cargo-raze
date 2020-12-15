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

use crate::{
  error::RazeError,
  metadata::{
    MetadataFetcher, DEFAULT_CRATE_INDEX_URL, DEFAULT_CRATE_REGISTRY_URL, SYSTEM_CARGO_BIN_PATH,
  },
};
use anyhow::{anyhow, Context, Result};
use cargo_metadata::{Metadata, MetadataCommand, Package};
use semver::VersionReq;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  hash::Hash,
  path::{Path, PathBuf},
};

pub type CrateSettingsPerVersion = HashMap<VersionReq, CrateSettings>;

/** The configuration settings for `cargo-raze`, included in a projects Cargo metadata */
#[derive(Debug, Clone, Deserialize)]
pub struct RazeSettings {
  /**
   * The path to the Cargo.toml working directory.
   */
  pub workspace_path: String,

  /**
   * The relative path within each workspace member directory where aliases the member's dependencies should be rendered.
   *
   * By default, a new directory will be created next to the `Cargo.toml` file named `cargo` for users to refer to them
   * as. For example, the toml file `//my/package:Cargo.toml`  will have aliases rendered as something like
   * `//my/package/cargo:dependency`. Note that setting this value to `"."` will cause the BUILD file in the same package
   * as the Cargo.toml file to be overwritten.
   */
  #[serde(default = "default_package_aliases_dir")]
  pub package_aliases_dir: String,

  /**
   * If true, will force the `workspace_path` setting will be treated as a Bazel label.
   *
   * When false, the `workspace_path` setting describes a path relative to the `Cargo.toml` file
   * where raze will output content. Setting this field to true will instead interpret `workspace_path`
   * to be a Bazel label and raze will generate content in the described Bazel package location.
   */
  #[serde(default = "incompatible_relative_workspace_path")]
  pub incompatible_relative_workspace_path: bool,

  /**
   * The platform target to generate BUILD rules for.
   *
   * This comes in the form of a "triple", such as "x86_64-unknown-linux-gnu"
   */
  #[serde(default)]
  pub target: Option<String>,

  /**
   * A list of targets to generate BUILD rules for.
   *
   * Each item comes in the form of a "triple", such as "x86_64-unknown-linux-gnu"
   */
  #[serde(default)]
  pub targets: Option<Vec<String>>,

  /**
   * A list of binary dependencies.
   */
  #[serde(default)]
  pub binary_deps: HashMap<String, cargo_toml::Dependency>,

  /** Any crate-specific configuration. See CrateSettings for details. */
  #[serde(default)]
  pub crates: HashMap<String, CrateSettingsPerVersion>,

  /**
   * Prefix for generated Bazel workspaces (from workspace_rules)
   *
   * This is only useful with remote genmode. It prefixes the names of the workspaces for
   * dependencies (@PREFIX_crateName_crateVersion) as well as the name of the repository function
   * generated in crates.bzl (PREFIX_fetch_remote_crates()).
   *
   * TODO(acmcarther): Does this have a non-bazel analogue?
   */
  #[serde(default = "default_raze_settings_field_gen_workspace_prefix")]
  pub gen_workspace_prefix: String,

  /** How to generate the dependencies. See GenMode for details. */
  #[serde(default = "default_raze_settings_field_genmode")]
  pub genmode: GenMode,

  /**
   * The name of the output BUILD files when `genmode == "Vendored"`
   * Default: BUILD.bazel
   */
  #[serde(default = "default_raze_settings_field_output_buildfile_suffix")]
  pub output_buildfile_suffix: String,

  /**
   * Default value for per-crate gen_buildrs setting if it's not explicitly for a crate.
   *
   * See that setting for more information.
   */
  #[serde(default = "default_raze_settings_field_gen_buildrs")]
  pub default_gen_buildrs: bool,

  /**
   * The default crates registry.
   *
   * The patterns `{crate}` and `{version}` will be used to fill
   * in the package's name (eg: rand) and version (eg: 0.7.1).
   * See https://doc.rust-lang.org/cargo/reference/registries.html#index-format
   */
  #[serde(default = "default_raze_settings_registry")]
  pub registry: String,

  /**
   * The index url to use for Binary dependencies
   */
  #[serde(default = "default_raze_settings_index_url")]
  pub index_url: String,

  /**
   * The name of the [rules_rust](https://github.com/bazelbuild/rules_rust) repository
   * used in the generated workspace.
   */
  #[serde(default = "default_raze_settings_rust_rules_workspace_name")]
  pub rust_rules_workspace_name: String,

  /**
   * The expected path relative to the `Cargo.toml` file where vendored sources can
   * be found. This should match the path passed to the `cargo vendor` command. eg:
   * `cargo vendor -q --versioned-dirs "cargo/vendor"
   */
  #[serde(default = "default_raze_settings_vendor_dir")]
  pub vendor_dir: String,
}

/** Override settings for individual crates (as part of `RazeSettings`). */
#[derive(Debug, Clone, Deserialize, Serialize)]
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
   * Dependencies to be removed from a crate, in the form "{dep-name}-{dep-version}"
   *
   * This is applied during Cargo analysis, so it uses Cargo-style labeling
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

  /** Environment variables to be added to the crate compilation process. */
  #[serde(default)]
  pub additional_env: HashMap<String, String>,

  /**
   * Whether or not to generate the build script that goes with this crate.
   *
   * Many build scripts will not function, as they will still be built hermetically. However, build
   * scripts that merely generate files into OUT_DIR may be fully functional.
   */
  #[serde(default = "default_crate_settings_field_gen_buildrs")]
  pub gen_buildrs: Option<bool>,

  /**
   * The verbatim `data` clause to be included for the generated build targets.
   *
   * N.B. Build scripts are always provided all crate files for their `data` attr.
   */
  #[serde(default = "default_crate_settings_field_data_attr")]
  pub data_attr: Option<String>,

  /**
   * The verbatim `compile_data` clause to be included for the generated build targets.
   */
  #[serde(default)]
  pub compile_data_attr: Option<String>,

  /**
   * Additional environment variables to add when running the build script.
   */
  #[serde(default)]
  pub buildrs_additional_environment_variables: HashMap<String, String>,

  /**
   * The arguments given to the patch tool.
   *
   * Defaults to `-p0`, however `-p1` will usually be needed for patches generated by git.
   *
   * If multiple `-p` arguments are specified, the last one will take effect.
   * If arguments other than `-p` are specified, Bazel will fall back to use patch command line
   * tool instead of the Bazel-native patch implementation.
   *
   * When falling back to `patch` command line tool and `patch_tool` attribute is not specified,
   * `patch` will be used.
   */
  #[serde(default)]
  pub patch_args: Vec<String>,

  /**
   * Sequence of Bash commands to be applied on Linux/Macos after patches are applied.
   */
  #[serde(default)]
  pub patch_cmds: Vec<String>,

  /**
   * Sequence of Powershell commands to be applied on Windows after patches are applied.
   *
   * If this attribute is not set, patch_cmds will be executed on Windows, which requires Bash
   * binary to exist.
   */
  #[serde(default)]
  pub patch_cmds_win: Vec<String>,

  /**
   * The `patch(1)` utility to use.
   *
   * If this is specified, Bazel will use the specifed patch tool instead of the Bazel-native patch
   * implementation.
   */
  #[serde(default)]
  pub patch_tool: Option<String>,

  /**
   * A list of files that are to be applied as patches after extracting the archive.
   *
   * By default, it uses the Bazel-native patch implementation which doesn't support fuzz match and
   * binary patch, but Bazel will fall back to use patch command line tool if `patch_tool`
   * attribute is specified or there are arguments other than `-p` in `patch_args` attribute.
   */
  #[serde(default)]
  pub patches: Vec<String>,

  /**
   * Path to a file to be included as part of the generated BUILD file.
   *
   * For example, some crates include non-Rust code typically built through a build.rs script. They
   * can be made compatible by manually writing appropriate Bazel targets, and including them into
   * the crate through a combination of additional_build_file and additional_deps.
   *
   * Note: This field should be a path to a file relative to the Cargo workspace root. For more
   * context, see https://doc.rust-lang.org/cargo/reference/workspaces.html#root-package
   */
  #[serde(default)]
  pub additional_build_file: Option<PathBuf>,
}

/**
 * Describes how dependencies should be managed in tree. Options are {Remote, Vendored}.
 *
 * Remote:
 * This mode assumes that files are not locally vendored, and generates a workspace-level
 * function that can bring them in.
 *
 * Vendored:
 * This mode assumes that files are vendored (into vendor/), and generates BUILD files
 * accordingly
 */
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum GenMode {
  Vendored,
  Remote,
  Unspecified,
}

impl Default for CrateSettings {
  fn default() -> Self {
    Self {
      additional_deps: Vec::new(),
      skipped_deps: Vec::new(),
      extra_aliased_targets: Vec::new(),
      additional_flags: Vec::new(),
      additional_env: HashMap::new(),
      gen_buildrs: default_crate_settings_field_gen_buildrs(),
      data_attr: default_crate_settings_field_data_attr(),
      compile_data_attr: None,
      buildrs_additional_environment_variables: HashMap::new(),
      patch_args: Vec::new(),
      patch_cmds: Vec::new(),
      patch_cmds_win: Vec::new(),
      patch_tool: None,
      patches: Vec::new(),
      additional_build_file: None,
    }
  }
}

fn default_raze_settings_field_gen_workspace_prefix() -> String {
  "raze".to_owned()
}

fn default_raze_settings_field_genmode() -> GenMode {
  GenMode::Unspecified
}

fn default_raze_settings_field_output_buildfile_suffix() -> String {
  "BUILD.bazel".to_owned()
}

fn default_raze_settings_field_gen_buildrs() -> bool {
  false
}

fn default_raze_settings_registry() -> String {
  format!(
    "{}/{}",
    DEFAULT_CRATE_REGISTRY_URL, "api/v1/crates/{crate}/{version}/download"
  )
}

fn default_raze_settings_index_url() -> String {
  DEFAULT_CRATE_INDEX_URL.to_string()
}

fn default_raze_settings_rust_rules_workspace_name() -> String {
  "io_bazel_rules_rust".to_owned()
}

fn default_raze_settings_vendor_dir() -> String {
  "vendor".to_owned()
}

fn default_crate_settings_field_gen_buildrs() -> Option<bool> {
  None
}

fn default_crate_settings_field_data_attr() -> Option<String> {
  None
}

fn default_package_aliases_dir() -> String {
  ".".to_owned()
}

fn incompatible_relative_workspace_path() -> bool {
  true
}

/** Formats a registry url to include the name and version fo the target package */
pub fn format_registry_url(registry_url: &str, name: &str, version: &str) -> String {
  registry_url
    .replace("{crate}", name)
    .replace("{version}", version)
}

/** Check that the the `additional_build_file` represents a path to a file from the cargo workspace root */
fn validate_crate_setting_additional_build_file(
  additional_build_file: &Path,
  cargo_workspace_root: &Path,
) -> Result<()> {
  let additional_build_file = cargo_workspace_root.join(&additional_build_file);
  if !additional_build_file.exists() {
    return Err(anyhow!(
      "File not found. `{}` should be a relative path from the cargo workspace root: {}",
      additional_build_file.display(),
      cargo_workspace_root.display()
    ));
  }

  Ok(())
}

/** Ensures crate settings associatd with the parsed [RazeSettings](crate::settings::RazeSettings) have valid crate settings */
fn validate_crate_settings(
  settings: &RazeSettings,
  cargo_workspace_root: &Path,
) -> Result<(), RazeError> {
  let mut errors = Vec::new();

  for (crate_name, crate_settings) in settings.crates.iter() {
    for (version, crate_settings) in crate_settings.iter() {
      if crate_settings.additional_build_file.is_none() {
        continue;
      }

      let result = validate_crate_setting_additional_build_file(
        // UNWRAP: Safe due to check above
        crate_settings.additional_build_file.as_ref().unwrap(),
        cargo_workspace_root,
      );

      if let Some(err) = result.err() {
        errors.push(RazeError::Config {
          field_path_opt: Some(format!(
            "raze.crates.{}.{}.additional_build_file",
            crate_name,
            version.to_string()
          )),
          message: err.to_string(),
        });
      }
    }
  }

  // Surface all errors
  if !errors.is_empty() {
    return Err(RazeError::Config {
      field_path_opt: None,
      message: format!("{:?}", errors),
    });
  }

  Ok(())
}

/** Verifies that the provided settings make sense. */
fn validate_settings(
  settings: &mut RazeSettings,
  cargo_workspace_path: &Path,
) -> Result<(), RazeError> {
  if !settings.workspace_path.starts_with("//") {
    return Err(
      RazeError::Config {
        field_path_opt: Some("raze.workspace_path".to_owned()),
        message: concat!(
          "Path must start with \"//\". Paths into local repositories (such as ",
          "@local//path) are currently unsupported."
        )
        .to_owned(),
      }
      .into(),
    );
  }

  if settings.workspace_path != "//" && settings.workspace_path.ends_with('/') {
    settings.workspace_path.pop();
  }

  if settings.genmode == GenMode::Unspecified {
    eprintln!(
      "WARNING: The [raze] setting `genmode` is unspecified. Not specifying `genmode` is \
       deprecated. Please explicitly set it to either \"Remote\" or \"Vendored\""
    );
    settings.genmode = GenMode::Vendored;
  }

  validate_crate_settings(settings, cargo_workspace_path)?;

  Ok(())
}

/** The intermediate configuration settings for `cargo-raze`, included in a project's Cargo metadata
 *
 * Note that this struct should contain only `Option` and match all public fields of
 * [`RazeSettings`](crate::settings::RazeSettings)
 */
#[derive(Debug, Clone, Deserialize)]
struct RawRazeSettings {
  #[serde(default)]
  pub workspace_path: Option<String>,
  #[serde(default)]
  pub package_aliases_dir: Option<String>,
  #[serde(default)]
  pub target: Option<String>,
  #[serde(default)]
  pub targets: Option<Vec<String>>,
  #[serde(default)]
  pub binary_deps: HashMap<String, cargo_toml::Dependency>,
  #[serde(default)]
  pub crates: HashMap<String, CrateSettingsPerVersion>,
  #[serde(default)]
  pub gen_workspace_prefix: Option<String>,
  #[serde(default)]
  pub genmode: Option<GenMode>,
  #[serde(default)]
  pub output_buildfile_suffix: Option<String>,
  #[serde(default)]
  pub default_gen_buildrs: Option<bool>,
  #[serde(default)]
  pub registry: Option<String>,
  #[serde(default)]
  pub index_url: Option<String>,
  #[serde(default)]
  pub incompatible_relative_workspace_path: Option<bool>,
  #[serde(default)]
  pub rust_rules_workspace_name: Option<String>,
  #[serde(default)]
  pub vendor_dir: Option<String>,
}

impl RawRazeSettings {
  /** Checks whether or not the settings have non-package specific settings specified */
  fn contains_primary_options(&self) -> bool {
    self.workspace_path.is_some()
      || self.package_aliases_dir.is_some()
      || self.target.is_some()
      || self.targets.is_some()
      || self.gen_workspace_prefix.is_some()
      || self.genmode.is_some()
      || self.output_buildfile_suffix.is_some()
      || self.default_gen_buildrs.is_some()
      || self.registry.is_some()
      || self.index_url.is_some()
      || self.incompatible_relative_workspace_path.is_some()
      || self.rust_rules_workspace_name.is_some()
      || self.vendor_dir.is_some()
  }

  fn print_notices_and_warnings(&self) {
    if self.default_gen_buildrs.is_none() {
      eprintln!(
        "NOTICE: The default of `[*.raze.default_gen_buildrs]` will soon be set to `true`. Please \
         explicitly set this flag to prevent a change in behavior."
      );
    }

    if self.incompatible_relative_workspace_path.unwrap_or(false) {
      eprintln!(
        "WARNING: `[*.raze.incompatible_relative_workspace_path]` will is deprecated. Please set \
         this flag to true and make sure `workspace_path` is the label where the Cargo-raze is \
         expected to write it's output."
      );
    }

    if self.target.is_some() {
      eprintln!(
        "WARNING: `[*.raze.target]` will soon be deprecated. Please update your project to use \
         `[*.raze.targets]`."
      );
    }

    if self.rust_rules_workspace_name.is_none() {
      eprintln!(
        "WARNING: The default of `[*.raze.rust_rules_workspace_name]` will soon be set to \
         `\"rules_rust\"`. Please explicitly set this flag to prevent a change in behavior or \
         upgrade your code to use the latest version of `rules_rust` and change references of \
         `io_bazel_rules_rust` to `rules_rust` in your project."
      );
    }

    if self.package_aliases_dir.is_none() {
      eprintln!(
        "WARNING: The default of `[*.raze.package_aliases_dir]` will soon be set to `\"cargo\"`. \
         Please explicitly set this flag to `\".\"` to prevent a change in behavior or update \
         your project structure to account for this change."
      );
    }
  }
}

/** Grows a list with duplicate keys between two maps */
fn extend_duplicates<K: Hash + Eq + Clone, V>(
  extended_list: &mut Vec<K>,
  main_map: &HashMap<K, V>,
  input_map: &HashMap<K, V>,
) {
  extended_list.extend(
    input_map
      .iter()
      .filter_map(|(key, _value)| {
        // Log the key if it exists in both the main and input maps
        if main_map.contains_key(key) {
          Some(key)
        } else {
          None
        }
      })
      .cloned()
      .collect::<Vec<K>>(),
  );
}

/** Parse [`RazeSettings`](crate::settings::RazeSettings) from workspace metadata */
fn parse_raze_settings_workspace(
  metadata_value: &serde_json::value::Value,
  metadata: &Metadata,
) -> Result<RazeSettings> {
  RawRazeSettings::deserialize(metadata_value)?.print_notices_and_warnings();
  let mut settings = RazeSettings::deserialize(metadata_value)?;

  let workspace_packages: Vec<&Package> = metadata
    .packages
    .iter()
    .filter(|pkg| metadata.workspace_members.contains(&pkg.id))
    .collect();

  let mut duplicate_binary_deps = Vec::new();
  let mut duplicate_crate_settings = Vec::new();

  for package in workspace_packages.iter() {
    if let Some(pkg_value) = package.metadata.get("raze") {
      let pkg_settings = RawRazeSettings::deserialize(pkg_value)?;
      if pkg_settings.contains_primary_options() {
        return Err(anyhow!(
          "The package '{}' contains Primary raze settings, please move these to the \
           `[workspace.metadata.raze]`",
          package.name
        ));
      }

      // Log duplicate binary dependencies
      extend_duplicates(
        &mut duplicate_binary_deps,
        &settings.binary_deps,
        &pkg_settings.binary_deps,
      );

      settings
        .binary_deps
        .extend(pkg_settings.binary_deps.into_iter());

      // Log duplicate crate settings
      extend_duplicates(
        &mut duplicate_crate_settings,
        &settings.crates,
        &pkg_settings.crates,
      );

      settings.crates.extend(pkg_settings.crates.into_iter());
    }
  }

  // Check for duplication errors
  if duplicate_binary_deps.len() > 0 {
    return Err(anyhow!(
      "Duplicate `raze.binary_deps` values detected accross various crates: {:?}",
      duplicate_binary_deps
    ));
  }
  if duplicate_crate_settings.len() > 0 {
    return Err(anyhow!(
      "Duplicate `raze.crates.*` values detected accross various crates: {:?}",
      duplicate_crate_settings
    ));
  }

  Ok(settings)
}

/** Parse [`RazeSettings`](crate::settings::RazeSettings) from a project's root package's metadata */
fn parse_raze_settings_root_package(
  metadata_value: &serde_json::value::Value,
  root_package: &Package,
) -> Result<RazeSettings> {
  RawRazeSettings::deserialize(metadata_value)?.print_notices_and_warnings();
  return RazeSettings::deserialize(metadata_value).with_context(|| {
    format!(
      "Failed to load raze settings from root package: {}",
      root_package.name,
    )
  });
}

/** Parse [`RazeSettings`](crate::settings::RazeSettings) from any workspace member's metadata */
fn parse_raze_settings_any_package(metadata: &Metadata) -> Result<RazeSettings> {
  let mut settings_packages = Vec::new();

  for package in metadata.packages.iter() {
    if let Some(pkg_value) = package.metadata.get("raze") {
      let pkg_settings = RawRazeSettings::deserialize(pkg_value)?;
      if pkg_settings.contains_primary_options() {
        settings_packages.push(package);
      }
    }
  }

  // There should only be one package with raze
  if settings_packages.len() > 1 {
    return Err(anyhow!(
      "Multiple packages contain primary raze settings: {:?}",
      settings_packages
        .iter()
        .map(|pkg| &pkg.name)
        .collect::<Vec<&String>>()
    ));
  }

  // UNWRAP: Safe due to checks above
  let settings_value = settings_packages[0].metadata.get("raze").unwrap();
  RawRazeSettings::deserialize(settings_value)?.print_notices_and_warnings();
  RazeSettings::deserialize(settings_value)
    .with_context(|| format!("Failed to deserialize raze settings: {:?}", settings_value))
}

/** A struct only to deserialize a Cargo.toml to in search of the legacy syntax for [`RazeSettings`](crate::settings::RazeSettings) */
#[derive(Debug, Clone, Deserialize)]
pub struct LegacyCargoToml {
  pub raze: RazeSettings,
}

/** Parse [`RazeSettings`](crate::settings::RazeSettings) from a Cargo.toml file using the legacy syntax `[raze]` */
fn parse_raze_settings_legacy(metadata: &Metadata) -> Result<RazeSettings> {
  let root_toml = metadata.workspace_root.join("Cargo.toml");
  let toml_contents = std::fs::read_to_string(&root_toml)?;
  let data = toml::from_str::<LegacyCargoToml>(&toml_contents).with_context(|| {
    format!(
      "Failed to read `[raze]` settings from {}",
      root_toml.display()
    )
  })?;
  Ok(data.raze)
}

/** Parses raze settings from the contents of a `Cargo.toml` file */
fn parse_raze_settings(metadata: Metadata) -> Result<RazeSettings> {
  // Workspace takes precedence
  let workspace_level_settigns = metadata.workspace_metadata.get("raze");
  if let Some(value) = workspace_level_settigns {
    return parse_raze_settings_workspace(value, &metadata);
  }

  // Root packages are the next priority
  if let Some(root_package) = metadata.root_package() {
    if let Some(value) = root_package.metadata.get("raze") {
      return parse_raze_settings_root_package(value, root_package);
    }
  }

  // Attempt to load legacy settings but do not allow failures to propogate
  if let Ok(settings) = parse_raze_settings_legacy(&metadata) {
    eprintln!(
      "WARNING: The top-level `[raze]` key is deprecated. Please set `[workspace.metadata.raze]` \
       or `[package.metadata.raze]` instead."
    );
    return Ok(settings);
  }

  // Finally check any package for settings
  parse_raze_settings_any_package(&metadata)
}

/** A cargo command wrapper for gathering cargo metadata used to parse [RazeSettings](crate::settings::RazeSettings) */
struct SettingsMetadataFetcher {
  pub cargo_bin_path: PathBuf,
}

impl MetadataFetcher for SettingsMetadataFetcher {
  fn fetch_metadata(&self, working_dir: &Path, _include_deps: bool) -> Result<Metadata> {
    // This fetch does not require network access.
    MetadataCommand::new()
      .cargo_path(&self.cargo_bin_path)
      .no_deps()
      .current_dir(working_dir)
      .other_options(vec!["--offline".to_owned()])
      .exec()
      .with_context(|| {
        format!(
          "Failed to fetch Metadata with `{}` from `{}`",
          &self.cargo_bin_path.display(),
          working_dir.display()
        )
      })
  }
}

/** Load settings used to configure the functionality of Cargo Raze */
pub fn load_settings<T: AsRef<Path>>(
  cargo_toml_path: T,
  cargo_bin_path: Option<String>,
) -> Result<RazeSettings, RazeError> {
  // Create a MetadataFetcher from either an optional Cargo binary path
  // or a fallback expected to be found on the system.
  let fetcher = SettingsMetadataFetcher {
    cargo_bin_path: cargo_bin_path
      .unwrap_or(SYSTEM_CARGO_BIN_PATH.to_string())
      .into(),
  };

  let cargo_toml_dir = cargo_toml_path
    .as_ref()
    .parent()
    .ok_or(RazeError::Generic(format!(
      "Failed to find parent directory for cargo toml file: {:?}",
      cargo_toml_path.as_ref().display(),
    )))?;
  let metadata = {
    let result = fetcher.fetch_metadata(cargo_toml_dir, false);
    if result.is_err() {
      return Err(RazeError::Generic(result.err().unwrap().to_string()));
    }
    // UNWRAP: safe due to check above
    result.unwrap()
  };

  let mut settings = {
    let result = parse_raze_settings(metadata);
    if result.is_err() {
      return Err(RazeError::Generic(result.err().unwrap().to_string()));
    }
    // UNWRAP: safe due to check above
    result.unwrap()
  };

  validate_settings(&mut settings, cargo_toml_dir)?;

  Ok(settings)
}

#[cfg(test)]
pub mod tests {
  use crate::testing::{make_workspace, named_toml_contents};

  use super::*;
  use indoc::{formatdoc, indoc};
  use tempfile::TempDir;

  pub fn dummy_raze_settings() -> RazeSettings {
    RazeSettings {
      workspace_path: "//cargo".to_owned(),
      package_aliases_dir: "cargo".to_owned(),
      target: Some("x86_64-unknown-linux-gnu".to_owned()),
      targets: None,
      crates: HashMap::new(),
      gen_workspace_prefix: "raze_test".to_owned(),
      genmode: GenMode::Remote,
      output_buildfile_suffix: "BUILD".to_owned(),
      default_gen_buildrs: default_raze_settings_field_gen_buildrs(),
      incompatible_relative_workspace_path: incompatible_relative_workspace_path(),
      binary_deps: HashMap::new(),
      registry: default_raze_settings_registry(),
      index_url: default_raze_settings_index_url(),
      rust_rules_workspace_name: default_raze_settings_rust_rules_workspace_name(),
      vendor_dir: default_raze_settings_vendor_dir(),
    }
  }

  #[test]
  fn test_loading_package_settings() {
    let toml_contents = indoc! { r#"
    [package]
    name = "load_settings_test"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    actix-web = "2.0.0"
    actix-rt = "1.0.0"

    [target.x86_64-apple-ios.dependencies]
    [target.x86_64-linux-android.dependencies]
    bitflags = "1.2.1"

    [package.metadata.raze]
    workspace_path = "//workspace_path/raze"
    genmode = "Remote"

    [package.metadata.raze.binary_deps]
    wasm-bindgen-cli = "0.2.68"
    "# };

    let temp_workspace_dir = TempDir::new()
      .ok()
      .expect("Failed to set up temporary directory");
    let cargo_toml_path = temp_workspace_dir.path().join("Cargo.toml");
    std::fs::write(&cargo_toml_path, &toml_contents).unwrap();

    let settings = load_settings(cargo_toml_path, None).unwrap();
    assert!(settings.binary_deps.len() > 0);
  }

  #[test]
  fn test_loading_settings_legacy() {
    let toml_contents = indoc! { r#"
    [package]
    name = "load_settings_test"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    actix-web = "2.0.0"
    actix-rt = "1.0.0"

    [target.x86_64-apple-ios.dependencies]
    [target.x86_64-linux-android.dependencies]
    bitflags = "1.2.1"

    [raze]
    workspace_path = "//workspace_path/raze"
    genmode = "Remote"

    [raze.binary_deps]
    wasm-bindgen-cli = "0.2.68"
    "# };

    let temp_workspace_dir = TempDir::new()
      .ok()
      .expect("Failed to set up temporary directory");
    let cargo_toml_path = temp_workspace_dir.path().join("Cargo.toml");
    std::fs::write(&cargo_toml_path, &toml_contents).unwrap();

    let settings = load_settings(cargo_toml_path, /*cargo_bin_path=*/ None).unwrap();
    assert!(settings.binary_deps.len() > 0);
  }

  #[test]
  fn test_loading_workspace_settings() {
    let toml_contents = indoc! { r#"
      [workspace]
      members = [
        "crate_a",
        "crate_b",
      ]

      [workspace.metadata.raze]
      workspace_path = "//workspace_path/raze"
      genmode = "Remote"
    "# };

    let (dir, files) = make_workspace(toml_contents, None);
    for member in vec!["crate_a", "crate_b"].iter() {
      let crate_toml = dir.as_ref().join(member).join("Cargo.toml");
      std::fs::create_dir_all(crate_toml.parent().unwrap()).unwrap();
      let toml_contents = formatdoc! { r#"
        {named_contents}

        [package.metadata.raze.crates.settings-test-{name}.'*']
        additional_flags = [
        "--cfg={name}"
        ]    
      "#, named_contents = named_toml_contents(member, "0.0.1"), name = member };
      std::fs::write(crate_toml, toml_contents).unwrap();
    }

    let settings = load_settings(files.toml_path, None).unwrap();
    assert_eq!(&settings.workspace_path, "//workspace_path/raze");
    assert_eq!(settings.genmode, GenMode::Remote);
    assert_eq!(settings.crates.len(), 2);
  }

  #[test]
  fn test_formatting_registry_url() {
    assert_eq!(
      format_registry_url(
        &default_raze_settings_registry(),
        &"foo".to_string(),
        &"0.0.1".to_string()
      ),
      "https://crates.io/api/v1/crates/foo/0.0.1/download"
    );

    assert_eq!(
      format_registry_url(
        &"https://registry.io/{crate}/{crate}/{version}/{version}".to_string(),
        &"foo".to_string(),
        &"0.0.1".to_string()
      ),
      "https://registry.io/foo/foo/0.0.1/0.0.1"
    );
  }
}
