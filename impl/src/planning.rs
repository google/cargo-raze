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

mod crate_catalog;
mod license;
mod subplanners;

use std::{
  collections::{BTreeMap, BTreeSet},
  fmt::Debug,
};

use anyhow::Result;
use cargo_lock::Lockfile;

use crate::{
  context::{CrateContext, DependencyAlias, WorkspaceContext},
  metadata::RazeMetadata,
  settings::RazeSettings,
  util::PlatformDetails,
};

use crate_catalog::CrateCatalog;
use subplanners::WorkspaceSubplanner;

/// A ready-to-be-rendered build, containing renderable context for each crate.
#[derive(Debug)]
pub struct PlannedBuild {
  /// The overall context for this workspace
  pub workspace_context: WorkspaceContext,
  /// The creates to build for
  pub crate_contexts: Vec<CrateContext>,
  /// And aliases that are defined at the workspace root
  pub workspace_aliases: Vec<DependencyAlias>,
  /// The version lock used if present
  pub lockfile: Option<Lockfile>,
}

/// An entity that can produce an organized, planned build ready to be rendered.
pub trait BuildPlanner {
  /// A function that returns a completely planned build using internally generated metadata, along
  /// with settings, platform specifications, and critical file locations.
  fn plan_build(&self, platform_details: Option<PlatformDetails>) -> Result<PlannedBuild>;
}

/// The default implementation of a `BuildPlanner`.
pub struct BuildPlannerImpl {
  metadata: RazeMetadata,
  settings: RazeSettings,
}

impl BuildPlanner for BuildPlannerImpl {
  /// Retrieves metadata for local workspace and produces a build plan.
  fn plan_build(&self, platform_details: Option<PlatformDetails>) -> Result<PlannedBuild> {
    // Create one combined metadata object which includes all dependencies and binaries
    let crate_catalog = CrateCatalog::new(&self.metadata.metadata)?;

    // Generate additional PlatformDetails
    let workspace_subplanner = WorkspaceSubplanner {
      crate_catalog: &crate_catalog,
      settings: &self.settings,
      platform_details: &platform_details,
      metadata: &self.metadata,
    };

    workspace_subplanner.produce_planned_build()
  }
}

impl BuildPlannerImpl {
  pub fn new(metadata: RazeMetadata, settings: RazeSettings) -> Self {
    Self { metadata, settings }
  }
}

/// Items that need to be rendered with platform select blocks.
pub trait PlatformCrateAttribute<T> {
  fn new(platforms: Vec<String>, attrs: Vec<T>) -> Self;
}

/// Group PlatformCrateAttribute items into concise select blocks, with no duplicates.
pub fn consolidate_platform_attributes<
  AttrType: Ord + Clone + Debug,
  T: PlatformCrateAttribute<AttrType>,
>(
  platform_attributes: BTreeMap<String, BTreeSet<AttrType>>,
) -> Vec<T> {
  let mut platform_map: BTreeMap<AttrType, Vec<String>> = BTreeMap::new();
  for (platform, pfs) in platform_attributes {
    for attr in pfs {
      platform_map
        .entry(attr)
        .and_modify(|e| e.push(platform.clone()))
        .or_insert_with(|| vec![platform.clone()]);
    }
  }

  let mut platforms_to_attrs: BTreeMap<Vec<String>, Vec<AttrType>> = BTreeMap::new();
  for (attr, platforms) in platform_map {
    platforms_to_attrs
      .entry(platforms.clone())
      .and_modify(|e| e.push(attr.clone()))
      .or_insert_with(|| vec![attr.clone()]);
  }

  platforms_to_attrs
    .iter()
    .map(|(platforms, attrs)| T::new(platforms.to_vec(), attrs.to_vec()))
    .collect()
}

#[cfg(test)]
pub mod tests {
  use std::{collections::BTreeMap, collections::HashMap, collections::HashSet};

  use crate::{
    metadata::tests::{
      dummy_raze_metadata, dummy_raze_metadata_fetcher, DummyCargoMetadataFetcher,
    },
    settings::{tests::*, GenMode},
    testing::*,
  };

  use super::*;
  use camino::Utf8PathBuf;
  use cargo_metadata::PackageId;
  use indoc::indoc;
  use itertools::Itertools;
  use semver::{Version, VersionReq};

  fn dummy_resolve_dropping_metadata() -> RazeMetadata {
    let raze_metadata = dummy_raze_metadata();
    let mut metadata = raze_metadata.metadata;
    assert!(metadata.resolve.is_some());
    metadata.resolve = None;
    RazeMetadata {
      metadata,
      cargo_workspace_root: Utf8PathBuf::from("/some/crate"),
      lockfile: None,
      checksums: HashMap::new(),
      features: BTreeMap::new(),
    }
  }

  #[test]
  fn test_plan_build_missing_resolve_returns_error() {
    let planner = BuildPlannerImpl::new(dummy_resolve_dropping_metadata(), dummy_raze_settings());
    let res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));
    assert!(res.is_err());
  }

  #[test]
  fn test_plan_build_minimum_workspace() {
    let planner = BuildPlannerImpl::new(dummy_raze_metadata(), dummy_raze_settings());
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));

    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  #[test]
  fn test_plan_build_minimum_workspace_dependency() {
    let planned_build_res = BuildPlannerImpl::new(
      template_raze_metadata(templates::DUMMY_MODIFIED_METADATA),
      dummy_raze_settings(),
    )
    .plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));

    let planned_build = planned_build_res.unwrap();
    assert_eq!(planned_build.crate_contexts.len(), 1);
    let dep = planned_build.crate_contexts.get(0).unwrap();
    assert_eq!(dep.pkg_name, "test_dep");
    assert!(!dep.workspace_member_dependents.is_empty());
    assert!(
      !dep.workspace_path_to_crate.contains('.'),
      "{} should be sanitized",
      dep.workspace_path_to_crate
    );
    assert!(
      !dep.workspace_path_to_crate.contains('-'),
      "{} should be sanitized",
      dep.workspace_path_to_crate
    );
  }

  pub fn dummy_workspace_crate_metadata(metadata_template: &str) -> RazeMetadata {
    let dir = make_basic_workspace();
    let (mut fetcher, _server, _index_dir) = dummy_raze_metadata_fetcher();

    // Ensure we render the given template
    fetcher.set_metadata_fetcher(Box::new(DummyCargoMetadataFetcher {
      metadata_template: Some(metadata_template.to_string()),
    }));

    let raze_metadata = fetcher
      .fetch_metadata(utf8_path(dir.as_ref()), None, None)
      .unwrap();
    let mut metadata = raze_metadata.metadata;

    // Phase 1: Create a workspace package, add it to the packages list.

    let name = "ws_crate_dep";
    let name_id = "ws_crate_dep_id";
    let id = PackageId {
      repr: name_id.to_string(),
    };

    let mut new_package = metadata.packages[0].clone();
    new_package.name = name.to_string();
    new_package.id = id.clone();
    new_package.version = Version::new(0, 0, 1);
    metadata.packages.push(new_package);

    // Phase 2: Add the workspace packages to the workspace members.

    metadata.workspace_members.push(id);

    RazeMetadata {
      metadata,
      cargo_workspace_root: Utf8PathBuf::from("/some/crate"),
      lockfile: None,
      checksums: HashMap::new(),
      features: BTreeMap::new(),
    }
  }

  #[test]
  fn test_plan_build_ignores_workspace_crates() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Vendored;

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::BASIC_METADATA),
      settings,
    );
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  #[test]
  fn test_plan_build_produces_aliased_dependencies() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::PLAN_BUILD_PRODUCES_ALIASED_DEPENDENCIES),
      settings,
    );
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));

    // Retrieve the crates that have aliased dependencies from the planned build
    let crates_with_aliased_deps: Vec<CrateContext> = planned_build_res
      .unwrap()
      .crate_contexts
      .into_iter()
      .filter(|krate| !krate.default_deps.aliased_dependencies.is_empty())
      .collect();

    // Vec length should be 1 as only cargo-raze-alias-test should have aliased dependencies
    assert_eq!(
      crates_with_aliased_deps.len(),
      1,
      "Crates with aliased dependencies is not 1"
    );

    // Find and verify that the cargo-raze-alias-test crate is in the Vec
    let krate_position = crates_with_aliased_deps
      .iter()
      .position(|krate| krate.pkg_name == "cargo-raze-alias-test");
    assert!(krate_position.is_some());

    // Get crate context using computed position
    let crate_ctx = crates_with_aliased_deps[krate_position.unwrap()].clone();

    // There are two default dependencies for cargo-raze-alias-test, log^0.4 and log^0.3
    // However, log^0.3 is aliased to old_log while log^0.4 isn't aliased. Therefore, we
    // should only see one aliased dependency (log^0.3 -> old_log) which shows that the
    // name and semver matching for aliased dependencies is working correctly
    let (name, dep) = crate_ctx
      .default_deps
      .aliased_dependencies
      .iter()
      .exactly_one()
      .unwrap();
    assert_eq!(name, &dep.target);
    assert_eq!(dep.target, "@raze_test__log__0_3_9//:log");
    assert_eq!(dep.alias, "old_log");
  }

  #[test]
  fn test_plan_build_produces_proc_macro_dependencies() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::PLAN_BUILD_PRODUCES_PROC_MACRO_DEPENDENCIES),
      settings,
    );
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    let serde = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "serde")
      .unwrap();

    assert_eq!(
      serde
        .default_deps
        .proc_macro_dependencies
        .iter()
        .filter(|dep| dep.name == "serde_derive")
        .count(),
      1
    );

    assert_eq!(
      serde
        .default_deps
        .dependencies
        .iter()
        .filter(|dep| dep.name == "serde_derive")
        .count(),
      0
    );
  }

  #[test]
  fn test_plan_build_produces_build_proc_macro_dependencies() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::PLAN_BUILD_PRODUCES_BUILD_PROC_MACRO_DEPENDENCIES),
      settings,
    );
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    let markup = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "markup5ever")
      .unwrap();

    assert_eq!(
      markup
        .default_deps
        .proc_macro_dependencies
        .iter()
        .filter(|dep| dep.name == "serde_derive")
        .count(),
      0
    );

    assert_eq!(
      markup
        .default_deps
        .build_proc_macro_dependencies
        .iter()
        .filter(|dep| dep.name == "serde_derive")
        .count(),
      1
    );
  }

  #[test]
  fn test_subplan_produces_crate_root_with_forward_slash() {
    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::SUBPLAN_PRODUCES_CRATE_ROOT_WITH_FORWARD_SLASH),
      dummy_raze_settings(),
    );
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    assert_eq!(
      planned_build.crate_contexts[0].targets[0].path,
      "src/lib.rs"
    );
  }

  #[test]
  // Tests the fix for https://github.com/google/cargo-raze/issues/389
  // as implemented in https://github.com/google/cargo-raze/pull/437
  fn test_plan_build_deduplicates_target_dependencies() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;
    let mut triples = HashSet::new();
    triples.insert("wasm32-unknown-unknown".to_string());
    triples.insert("x86_64-unknown-linux-gnu".to_string());
    settings.targets = Some(triples);

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(
        templates::SUBPLAN_OMITS_PLATFORM_DEPS_ALREADY_IN_DEFAULT_DEPS,
      ),
      settings,
    );
    let planned_build = planner.plan_build(None).unwrap();

    let flate2 = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "flate2")
      .unwrap();

    let miniz_oxide = flate2
      .default_deps
      .dependencies
      .iter()
      .find(|dep| dep.name == "miniz_oxide");
    assert!(miniz_oxide.is_some());
    assert_eq!(flate2.targeted_deps[0].deps.dependencies.len(), 0);
  }

  #[test]
  fn test_plan_includes_all_not_unknown_targets() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;
    let mut triples = HashSet::new();
    triples.insert("aarch64-apple-darwin".to_string());
    triples.insert("aarch64-unknown-linux-gnu".to_string());
    triples.insert("i686-apple-darwin".to_string());
    triples.insert("i686-pc-windows-msvc".to_string());
    triples.insert("i686-unknown-linux-gnu".to_string());
    triples.insert("x86_64-apple-darwin".to_string());
    triples.insert("x86_64-pc-windows-msvc".to_string());
    triples.insert("x86_64-unknown-linux-gnu".to_string());
    triples.insert("wasm32-wasi".to_string());
    triples.insert("wasm32-unknown-unknown".to_string());
    settings.targets = Some(triples);

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::TARGET_OS_IS_NOT_UNKNOWN),
      settings,
    );
    let planned_build = planner.plan_build(None).unwrap();
    let async_std = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "async-std")
      .unwrap();

    // The wasm-targeted deps should have both wasm platforms
    let wasm_targeted_dep_context = async_std
      .targeted_deps
      .iter()
      .find(|dep_context| {
        dep_context
          .deps
          .dependencies
          .iter()
          .find(|dep| dep.name == "wasm-bindgen-futures")
          .is_some()
      })
      .unwrap();
    assert_eq!(
      wasm_targeted_dep_context.platform_targets,
      vec!["wasm32-unknown-unknown", "wasm32-wasi",]
    );

    // The wasm-targeted deps should have both wasm platforms
    let os_targeted_dep_context = async_std
      .targeted_deps
      .iter()
      .find(|dep_context| {
        dep_context
          .deps
          .dependencies
          .iter()
          .find(|dep| dep.name == "async-io")
          .is_some()
      })
      .unwrap();
    assert_eq!(
      os_targeted_dep_context.platform_targets,
      vec![
        "aarch64-apple-darwin",
        "aarch64-unknown-linux-gnu",
        "i686-apple-darwin",
        "i686-pc-windows-msvc",
        "i686-unknown-linux-gnu",
        "wasm32-wasi",
        "x86_64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu"
      ]
    );
  }

  #[test]
  // Tests the fix for https://github.com/google/cargo-raze/issues/451.
  // Bazel errors out if mutually exclusive select branches contain the
  // same dependency. rust-errno is a real world example of this problem,
  // where libc is listed under 'cfg(unix)' and 'cfg(target_os="wasi")'.
  fn test_plan_build_consolidates_targets_across_platforms() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;
    let mut triples = HashSet::new();
    triples.insert("wasm32-wasi".to_string());
    triples.insert("x86_64-unknown-linux-gnu".to_string());
    settings.target = None;
    settings.targets = Some(triples);

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::SUBPLAN_CONSOLIDATES_TARGETED_DEPS),
      settings,
    );
    let planned_build = planner.plan_build(None).unwrap();

    let errno = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "errno")
      .unwrap();

    assert_eq!(errno.targeted_deps.len(), 1);
    assert_eq!(
      errno.targeted_deps[0].platform_targets,
      vec!["wasm32-wasi", "x86_64-unknown-linux-gnu",]
    );
  }

  fn dummy_binary_dependency_metadata(is_remote_genmode: bool) -> (RazeMetadata, RazeSettings) {
    let (mut fetcher, server, index_dir) = dummy_raze_metadata_fetcher();

    // Only in cases where `RazeSettings::genmode == "Remote"` do we exepct the metadata to be anything different
    // than the standard metadata. So we use a generated template to represent that state.
    let dummy_metadata_fetcher = DummyCargoMetadataFetcher {
      metadata_template: if is_remote_genmode {
        Some(templates::DUMMY_BINARY_DEPENDENCY_REMOTE.to_string())
      } else {
        Some(templates::BASIC_METADATA.to_string())
      },
    };
    fetcher.set_metadata_fetcher(Box::new(dummy_metadata_fetcher));

    let mock = mock_remote_crate("some-binary-crate", "3.3.3", &server);
    let dir = mock_crate_index(
      &to_index_crates_map(vec![("some-binary-crate", "3.3.3")]),
      Some(utf8_path(index_dir.path())),
    );
    assert!(dir.is_none());

    let mut settings = dummy_raze_settings();
    settings.binary_deps.insert(
      "some-binary-crate".to_string(),
      cargo_toml::Dependency::Simple("3.3.3".to_string()),
    );

    let dir = make_basic_workspace();
    let raze_metadata = fetcher
      .fetch_metadata(utf8_path(dir.as_ref()), Some(&settings.binary_deps), None)
      .unwrap();

    for mock in mock.endpoints.iter() {
      mock.assert();
    }

    (raze_metadata, settings)
  }

  #[test]
  fn test_binary_dependencies_remote_genmode() {
    let (raze_metadata, mut settings) =
      dummy_binary_dependency_metadata(/*is_remote_genmode=*/ true);
    settings.genmode = GenMode::Remote;

    // Make sure the dummy settings contain the information we expect
    let version = Version::parse("3.3.3").unwrap();
    assert!(settings.binary_deps.contains_key("some-binary-crate"));
    assert_eq!(
      settings.binary_deps.get("some-binary-crate").unwrap().req(),
      version.to_string()
    );

    let planner = BuildPlannerImpl::new(raze_metadata, settings);
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    // We expect to have a crate context for the binary dependency
    let context = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "some-binary-crate" && ctx.pkg_version == version)
      .unwrap();

    // It's also expected to have a checksum
    assert!(context.sha256.is_some());

    assert_eq!(
      context.source_details.download_url,
      Some(
        "https://crates.io/api/v1/crates/some-binary-crate/3.3.3/download"
          .parse()
          .unwrap()
      )
    );
  }

  #[test]
  fn test_binary_dependencies_vendored_genmode() {
    let (raze_metadata, mut settings) =
      dummy_binary_dependency_metadata(/*is_remote_genmode=*/ false);
    settings.genmode = GenMode::Vendored;

    // Make sure the dummy settings contain the information we expect
    let version = Version::parse("3.3.3").unwrap();
    assert!(settings.binary_deps.contains_key("some-binary-crate"));
    assert_eq!(
      settings.binary_deps.get("some-binary-crate").unwrap().req(),
      version.to_string()
    );

    let planner = BuildPlannerImpl::new(raze_metadata, settings);
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    // Vendored builds do not use binary dependencies and should not alter the outputs
    assert!(!planned_build
      .crate_contexts
      .iter()
      .any(|ctx| ctx.pkg_name == "some-binary-crate"));
  }

  #[test]
  fn test_workspace_context_contains_no_binary_dependencies() {
    let (raze_metadata, mut settings) =
      dummy_binary_dependency_metadata(/*is_remote_genmode=*/ true);
    settings.genmode = GenMode::Remote;

    // Make sure the dummy settings contain the information we expect
    let version = Version::parse("3.3.3").unwrap();
    assert!(settings.binary_deps.contains_key("some-binary-crate"));
    assert_eq!(
      settings.binary_deps.get("some-binary-crate").unwrap().req(),
      version.to_string()
    );

    let planner = BuildPlannerImpl::new(raze_metadata, settings);
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    // Ensure binary dependencies are not considered workspace members after planning
    let binary_dep_name = format!("some-binary-crate-{}", version.to_string());
    assert_eq!(
      planned_build
        .workspace_context
        .workspace_members
        .iter()
        .any(|member| match member.file_name() {
          Some(file_name) => file_name == binary_dep_name.as_str(),
          None => false,
        }),
      false
    )
  }

  #[test]
  fn test_semver_matching() {
    let toml_file = indoc! { r#"
    [package]
    name = "semver_toml"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    # This has no settings
    anyhow = "1.0"

    openssl-sys = "=0.9.24"
    openssl = "=0.10.2"
    unicase = "=2.1"
    bindgen = "=0.32"
    clang-sys = "=0.21.1"

    # The following are negative tests aka test they dont match
    lexical-core = "0.7.4"

    [package.metadata.raze]
    workspace_path = "//cargo"
    genmode = "Remote"

    # All these examples are basically from the readme and "handling unusual crates:
    # They are adapted to handle the variety of semver patterns
    # In reality, you probably want to express many patterns more generally

    # Test bare versions
    # AKA: `==0.9.24`
    [package.metadata.raze.crates.openssl-sys.'0.9.24']
    additional_flags = [
      # Vendored openssl is 1.0.2m
      "--cfg=ossl102",
      "--cfg=version=102",
    ]
    additional_deps = [
      "@//third_party/openssl:crypto",
      "@//third_party/openssl:ssl",
    ]

    # Test `^` range
    # AKA: `>=0.10.0 < 0.11.0-0`
    [package.metadata.raze.crates.openssl.'^0.10']
    additional_flags = [
      # Vendored openssl is 1.0.2m
      "--cfg=ossl102",
      "--cfg=version=102",
      "--cfg=ossl10x",
    ]

    # Test `*` or globs
    # AKA: `>=0.21.0 < 0.22.0-0`
    [package.metadata.raze.crates.clang-sys.'0.21.*']
    gen_buildrs = true

    # Test `~` range
    # AKA: `>=2.0.0 < 3.0.0-0`
    [package.metadata.raze.crates.unicase.'~2']
    additional_flags = [
      # Rustc is 1.15, enable all optional settings
      "--cfg=__unicase__iter_cmp",
      "--cfg=__unicase__defauler_hasher",
    ]

    # Test `*` full glob
    # AKA: Get out of my way raze and just give me this for everything
    [package.metadata.raze.crates.bindgen.'*']
    gen_buildrs = true # needed to build bindgen
    extra_aliased_targets = [
        "cargo_bin_bindgen"
    ]

    # This should not match unicase, and should not error
    [package.metadata.raze.crates.unicase.'2.6.0']
    additional_flags = [
        "--cfg=SHOULD_NOT_MATCH"
    ]

    [package.metadata.raze.crates.lexical-core.'~0.6']
    additional_flags = [
        "--cfg=SHOULD_NOT_MATCH"
    ]

    [package.metadata.raze.crates.lexical-core.'^0.6']
    additional_flags = [
        "--cfg=SHOULD_NOT_MATCH"
    ]
    "#};

    let settings = {
      let temp_dir = make_workspace(toml_file, None);
      let manifest_path = Utf8PathBuf::from_path_buf(temp_dir.as_ref().join("Cargo.toml")).unwrap();
      crate::settings::load_settings_from_manifest(&manifest_path, None).unwrap()
    };

    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(templates::SEMVER_MATCHING),
      settings,
    );

    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(),
    )));

    let crates: Vec<CrateContext> = planned_build_res
      .unwrap()
      .crate_contexts
      .into_iter()
      .collect();

    let dep = |name: &str, ver_req: &str| {
      let ver_req = VersionReq::parse(ver_req).unwrap();
      &crates
        .iter()
        .find(|dep| dep.pkg_name == name && ver_req.matches(&dep.pkg_version))
        .unwrap_or_else(|| panic!("{} not found", name))
        .raze_settings
    };

    let assert_dep_not_match = |name: &str, ver_req: &str| {
      // Didnt match anything so should not have any settings
      let test_dep = dep(name, ver_req);
      assert!(test_dep.additional_flags.is_empty());
      assert!(test_dep.additional_deps.is_empty());
      assert!(test_dep.gen_buildrs.is_none());
      assert!(test_dep.extra_aliased_targets.is_empty());
      assert!(test_dep.patches.is_empty());
      assert!(test_dep.patch_cmds.is_empty());
      assert!(test_dep.patch_tool.is_none());
      assert!(test_dep.patch_cmds_win.is_empty());
      assert!(test_dep.skipped_deps.is_empty());
      assert!(test_dep.additional_build_file.is_none());
      assert!(test_dep.data_attr.is_none());
      assert!(test_dep.compile_data_attr.is_none());
    };

    assert_dep_not_match("anyhow", "*");
    assert_dep_not_match("lexical-core", "^0.7");

    assert_eq! {
      dep("openssl-sys", "0.9.24").additional_deps,
      vec![
        "@//third_party/openssl:crypto",
        "@//third_party/openssl:ssl"
      ]
    };
    assert_eq! {
      dep("openssl-sys", "0.9.24").additional_flags,
      vec!["--cfg=ossl102", "--cfg=version=102"]
    };

    assert_eq! {
      dep("openssl", "0.10.*").additional_flags,
      vec!["--cfg=ossl102", "--cfg=version=102", "--cfg=ossl10x"],
    };

    assert!(dep("clang-sys", "0.21").gen_buildrs.unwrap_or_default());

    assert_eq! {
      dep("unicase", "2.1").additional_flags,
      vec! [
        "--cfg=__unicase__iter_cmp",
        "--cfg=__unicase__defauler_hasher",
      ]
    };

    assert!(dep("bindgen", "*").gen_buildrs.unwrap_or_default());
    assert_eq! {
        dep("bindgen", "*").extra_aliased_targets,
        vec!["cargo_bin_bindgen"]
    };
  }

  fn dummy_workspace_member_toml_contents(name: &str, dep_version: &str) -> String {
    assert!(
      dep_version == "0.2.1" || dep_version == "0.1.0",
      "The `dummy_workspace_members_metadata` template was generated with these versions, if \
       something else is passed, that file will need to be regenerated"
    );
    indoc::formatdoc! { r#"
      [package]
      name = "{name}"
      version = "0.0.1"

      [lib]
      path = "src/lib.rs"

      [dependencies]
      unicode-xid = "{dep_version}"
    "#, name = name, dep_version = dep_version }
  }

  fn dummy_workspace_members_metadata() -> RazeMetadata {
    let (mut fetcher, _server, _index_dir) = dummy_raze_metadata_fetcher();
    fetcher.set_metadata_fetcher(Box::new(DummyCargoMetadataFetcher {
      metadata_template: Some("dummy_workspace_members_metadata.json.template".to_string()),
    }));

    let workspace_toml = indoc! { r#"
      [workspace]
      members = [
          "lib_a",
          "lib_b",
      ]
     "# };

    let workspace_lock = indoc! { r#"
      [[package]]
      name = "lib_a"
      version = "0.0.1"
      dependencies = [
          "unicode-xid 0.2.1",
      ]
      
      [[package]]
      name = "lib_b"
      version = "0.0.1"
      dependencies = [
          "unicode-xid 0.1.0",
      ]
    "# };

    let crate_dir = make_workspace(workspace_toml, Some(workspace_lock));

    for (member, dep_version) in vec![("lib_a", "0.2.1"), ("lib_b", "0.1.0")].iter() {
      let member_dir = crate_dir.as_ref().join(&member);
      std::fs::create_dir_all(&member_dir).unwrap();
      std::fs::write(
        member_dir.join("Cargo.toml"),
        dummy_workspace_member_toml_contents(member, dep_version),
      )
      .unwrap();
    }

    fetcher
      .fetch_metadata(utf8_path(crate_dir.as_ref()), None, None)
      .unwrap()
  }

  #[test]
  fn test_workspace_members_share_dependency_of_different_versions() {
    let raze_metadata = dummy_workspace_members_metadata();

    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(raze_metadata, settings);
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(),
      )))
      .unwrap();

    // Ensure both versions of `unicode-xib` are available
    assert!(planned_build
      .crate_contexts
      .iter()
      .any(|ctx| ctx.pkg_name == "unicode-xid" && ctx.pkg_version == Version::new(0, 1, 0)));

    assert!(planned_build
      .crate_contexts
      .iter()
      .any(|ctx| ctx.pkg_name == "unicode-xid" && ctx.pkg_version == Version::new(0, 2, 1)));
  }
  // TODO(acmcarther): Add tests:
  // TODO(acmcarther): Extra flags work
  // TODO(acmcarther): Extra deps work
  // TODO(acmcarther): Buildrs works
  // TODO(acmcarther): Extra aliases work
  // TODO(acmcarther): Skipped deps work
}
