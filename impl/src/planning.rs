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

mod checks;
mod crate_catalog;
mod license;
mod subplanners;

use anyhow::Result;

use cargo_lock::Lockfile;

use crate::{
  context::{CrateContext, WorkspaceContext},
  metadata::RazeMetadata,
  settings::RazeSettings,
  util::PlatformDetails,
};

use crate_catalog::CrateCatalog;
use subplanners::WorkspaceSubplanner;

/** A ready-to-be-rendered build, containing renderable context for each crate. */
#[derive(Debug)]
pub struct PlannedBuild {
  pub workspace_context: WorkspaceContext,
  pub crate_contexts: Vec<CrateContext>,
  pub lockfile: Option<Lockfile>,
}

/** An entity that can produce an organized, planned build ready to be rendered. */
pub trait BuildPlanner {
  /**
   * A function that returns a completely planned build using internally generated metadata, along
   * with settings, platform specifications, and critical file locations.
   */
  fn plan_build(&self, platform_details: Option<PlatformDetails>) -> Result<PlannedBuild>;
}

/** The default implementation of a `BuildPlanner`. */
pub struct BuildPlannerImpl {
  pub metadata: RazeMetadata,
  pub settings: RazeSettings,
}

impl BuildPlanner for BuildPlannerImpl {
  /** Retrieves metadata for local workspace and produces a build plan. */
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
    Self {
      metadata,
      settings,
    }
  }
}

#[cfg(test)]
mod tests {
  use std::{collections::HashMap, path::PathBuf};

  use crate::{
    metadata::{
      tests::{dummy_raze_metadata, dummy_raze_metadata_fetcher},
      MetadataFetcher,
    },
    settings::{tests::*, GenMode},
    testing::*,
  };

  use super::*;
  use cargo_metadata::{MetadataCommand, PackageId};
  use indoc::indoc;
  use semver::{Version, VersionReq};

  fn dummy_resolve_dropping_metadata() -> RazeMetadata {
    let raze_metadata = dummy_raze_metadata();
    let mut metadata = raze_metadata.metadata.clone();
    assert!(metadata.resolve.is_some());
    metadata.resolve = None;
    RazeMetadata::new(metadata, HashMap::new(), PathBuf::from("/some/crate"), None)
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

  fn dummy_package_dropping_metadata() -> RazeMetadata {
    let raze_metadata = dummy_raze_metadata();
    let mut metadata = raze_metadata.metadata.clone();
    metadata.packages.clear();
    RazeMetadata::new(metadata, HashMap::new(), PathBuf::from("/some/crate"), None)
  }

  #[test]
  fn test_plan_build_missing_package_in_metadata() {
    let planner = BuildPlannerImpl::new(dummy_package_dropping_metadata(), dummy_raze_settings());
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));

    assert!(planned_build_res.is_err());
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

  fn dummy_injecting_metadata() -> RazeMetadata {
    let raze_metadata = dummy_raze_metadata();
    let mut metadata = raze_metadata.metadata.clone();

    // Phase 1: Add a dummy dependency to the dependency graph.

    let mut resolve = metadata.resolve.take().unwrap();
    let mut new_node = resolve.nodes[0].clone();
    let name = "test_dep";
    let name_id = "test_dep_id";

    // Add the new dependency.
    let id = PackageId {
      repr: name_id.to_string(),
    };
    resolve.nodes[0].dependencies.push(id.clone());

    // Add the new node representing the dependency.
    new_node.id = id;
    new_node.deps = Vec::new();
    new_node.dependencies = Vec::new();
    new_node.features = Vec::new();
    resolve.nodes.push(new_node);
    metadata.resolve = Some(resolve);

    // Phase 2: Add the dummy dependency to the package list.

    let mut new_package = metadata.packages[0].clone();
    new_package.name = name.to_string();
    new_package.id = PackageId {
      repr: name_id.to_string(),
    };
    new_package.version = Version::new(0, 0, 1);
    metadata.packages.push(new_package);

    RazeMetadata::new(metadata, HashMap::new(), PathBuf::from("/some/crate"), None)
  }

  #[test]
  fn test_plan_build_minimum_workspace_dependency() {
    let planned_build_res =
      BuildPlannerImpl::new(dummy_injecting_metadata(), dummy_raze_settings()).plan_build(Some(
        PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
      ));

    let planned_build = planned_build_res.unwrap();
    assert_eq!(planned_build.crate_contexts.len(), 1);
    let dep = planned_build.crate_contexts.get(0).unwrap();
    assert_eq!(dep.pkg_name, "test_dep");
    assert!(!dep.workspace_member_dependents.is_empty());
    assert!(
      !dep.workspace_path_to_crate.contains("."),
      "{} should be sanitized",
      dep.workspace_path_to_crate
    );
    assert!(
      !dep.workspace_path_to_crate.contains("-"),
      "{} should be sanitized",
      dep.workspace_path_to_crate
    );
  }

  #[test]
  fn test_plan_build_verifies_vendored_state() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Vendored;
    let planner = BuildPlannerImpl::new(dummy_injecting_metadata(), settings);
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));

    assert!(planned_build_res.is_err());
  }

  fn dummy_workspace_crate_metadata(toml: Option<&str>) -> RazeMetadata {
    let mut metadata = match toml {
      Some(toml) => {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.as_ref().join("Cargo.toml"), toml).unwrap();
        MetadataCommand::new()
          .current_dir(dir.as_ref())
          .exec()
          .unwrap()
      },
      None => dummy_raze_metadata().metadata.clone(),
    };

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

    RazeMetadata::new(metadata, HashMap::new(), PathBuf::from("/some/crate"), None)
  }

  #[test]
  fn test_plan_build_ignores_workspace_crates() {
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Vendored;

    let planner = BuildPlannerImpl::new(dummy_workspace_crate_metadata(None), settings);
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  #[test]
  fn test_plan_build_produces_aliased_dependencies() {
    let toml_file = indoc! { r#"
    [package]
    name = "advanced_toml"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    actix-web = "2.0.0"
    actix-rt = "1.0.0"
    "# };

    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(dummy_workspace_crate_metadata(Some(toml_file)), settings);
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(Some(PlatformDetails::new(
      "some_target_triple".to_owned(),
      Vec::new(), /* attrs */
    )));

    let crates_with_aliased_deps: Vec<CrateContext> = planned_build_res
      .unwrap()
      .crate_contexts
      .into_iter()
      .filter(|krate| krate.default_deps.aliased_dependencies.len() != 0)
      .collect();

    // Vec length shouldn't be 0
    assert!(
      crates_with_aliased_deps.len() != 0,
      "Crates with aliased dependencies is 0"
    );

    // Find the actix-web crate
    let actix_web_position = crates_with_aliased_deps
      .iter()
      .position(|krate| krate.pkg_name == "actix-http");
    assert!(actix_web_position.is_some());

    // Get crate context using computed position
    let actix_http_context = crates_with_aliased_deps[actix_web_position.unwrap()].clone();

    assert!(actix_http_context.default_deps.aliased_dependencies.len() == 1);
    assert!(
      actix_http_context.default_deps.aliased_dependencies[0].target
        == "@raze_test__failure__0_1_8//:failure"
    );
    assert!(actix_http_context.default_deps.aliased_dependencies[0].alias == "fail_ure");
  }

  #[test]
  fn test_plan_build_produces_proc_macro_dependencies() {
    let toml_file = indoc! { r#"
    [package]
    name = "advanced_toml"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    serde = { version = "=1.0.112", features = ["derive"] }
    "# };
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(dummy_workspace_crate_metadata(Some(toml_file)), settings);
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

    let serde_derive_proc_macro_deps: Vec<_> = serde
      .default_deps
      .proc_macro_dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(serde_derive_proc_macro_deps.len(), 1);

    let serde_derive_normal_deps: Vec<_> = serde
      .default_deps
      .dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(serde_derive_normal_deps.len(), 0);
  }

  #[test]
  fn test_plan_build_produces_build_proc_macro_dependencies() {
    let toml_file = indoc! { r#"
    [package]
    name = "advanced_toml"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    markup5ever = "=0.10.0"
    "# };
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(dummy_workspace_crate_metadata(Some(toml_file)), settings);
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

    let markup_proc_macro_deps: Vec<_> = markup
      .default_deps
      .proc_macro_dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(markup_proc_macro_deps.len(), 0);

    let markup_build_proc_macro_deps: Vec<_> = markup
      .default_deps
      .build_proc_macro_dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(markup_build_proc_macro_deps.len(), 1);
  }

  #[test]
  fn test_subplan_produces_crate_root_with_forward_slash() {
    let toml_file = indoc! { r#"
    [package]
    name = "advanced_toml"
    version = "0.1.0"

    [lib]
    path = "not_a_file.rs"

    [dependencies]
    markup5ever = "=0.10.0"
    "# };
    let planner = BuildPlannerImpl::new(
      dummy_workspace_crate_metadata(Some(toml_file)),
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

  fn dummy_binary_dependency_metadata() -> (RazeMetadata, RazeSettings) {
    let (fetcher, server, index_dir) = dummy_raze_metadata_fetcher();
    let mock = mock_remote_crate("some-binary-crate", "3.3.3", &server);
    let dir = mock_crate_index(
      &to_index_crates_map(vec![("some-binary-crate", "3.3.3")]),
      Some(index_dir.as_ref()),
    );
    assert!(dir.is_none());

    let mut settings = dummy_raze_settings();
    settings.binary_deps.insert(
      "some-binary-crate".to_string(),
      cargo_toml::Dependency::Simple("3.3.3".to_string()),
    );

    let (_dir, files) = make_basic_workspace();
    let raze_metadata = fetcher
      .fetch_metadata(&files, Some(&settings.binary_deps), None)
      .unwrap();

    for mock in mock.endpoints.iter() {
      mock.assert();
    }

    (raze_metadata, settings)
  }

  #[test]
  fn test_binary_dependencies_remote_genmode() {
    let (raze_metadata, mut settings) = dummy_binary_dependency_metadata();
    settings.genmode = GenMode::Remote;

    let planner = BuildPlannerImpl::new(raze_metadata, settings);
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    let version = Version::parse("3.3.3").unwrap();

    // We expect to have a crate context for the binary dependency
    let context = planned_build
      .crate_contexts
      .iter()
      .inspect(|x| println!("{}{}", x.pkg_name, x.pkg_version))
      .find(|ctx| ctx.pkg_name == "some-binary-crate" && ctx.pkg_version == version)
      .unwrap();

    // It's also expected to have a checksum
    assert!(context.sha256.is_some());
  }

  #[test]
  fn test_binary_dependencies_vendored_genmode() {
    let (raze_metadata, mut settings) = dummy_binary_dependency_metadata();
    settings.genmode = GenMode::Vendored;

    let planner = BuildPlannerImpl::new(raze_metadata, settings);
    let planned_build = planner
      .plan_build(Some(PlatformDetails::new(
        "some_target_triple".to_owned(),
        Vec::new(), /* attrs */
      )))
      .unwrap();

    let wasm_version = Version::parse("0.2.68").unwrap();

    // Vendored builds do not use binary dependencies and should not alter the outputs
    assert!(planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "wasm-bindgen-cli" && ctx.pkg_version == wasm_version)
      .is_none());
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

    [raze]
    workspace_path = "//cargo"
    genmode = "Remote"

    # All these examples are basically from the readme and "handling unusual crates:
    # They are adapted to handle the variety of semver patterns
    # In reality, you probably want to express many patterns more generally

    # Test bare versions
    # AKA: `==0.9.24`
    [raze.crates.openssl-sys.'0.9.24']
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
    [raze.crates.openssl.'^0.10']
    additional_flags = [
      # Vendored openssl is 1.0.2m
      "--cfg=ossl102",
      "--cfg=version=102",
      "--cfg=ossl10x",
    ]

    # Test `*` or globs
    # AKA: `>=0.21.0 < 0.22.0-0`
    [raze.crates.clang-sys.'0.21.*']
    gen_buildrs = true

    # Test `~` range
    # AKA: `>=2.0.0 < 3.0.0-0`
    [raze.crates.unicase.'~2']
    additional_flags = [
      # Rustc is 1.15, enable all optional settings
      "--cfg=__unicase__iter_cmp",
      "--cfg=__unicase__defauler_hasher",
    ]

    # Test `*` full glob
    # AKA: Get out of my way raze and just give me this for everything
    [raze.crates.bindgen.'*']
    gen_buildrs = true # needed to build bindgen
    extra_aliased_targets = [
        "cargo_bin_bindgen"
    ]

    # This should not match unicase, and should not error
    [raze.crates.unicase.'2.6.0']
    additional_flags = [
        "--cfg=SHOULD_NOT_MATCH"
    ]

    [raze.crates.lexical-core.'~0.6']
    additional_flags = [
        "--cfg=SHOULD_NOT_MATCH"
    ]

    [raze.crates.lexical-core.'^0.6']
    additional_flags = [
        "--cfg=SHOULD_NOT_MATCH"
    ]
    "#};

    let settings = {
      let (_temp_dir, files) = make_workspace(toml_file, None);
      crate::settings::load_settings(&files.toml_path).unwrap()
    };

    let planner = BuildPlannerImpl::new(dummy_workspace_crate_metadata(Some(toml_file)), settings);

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
        .expect(&format!("{} not found", name))
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
    let (fetcher, _server, _index_dir) = dummy_raze_metadata_fetcher();

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

    let (crate_dir, files) = make_workspace(&workspace_toml, Some(&workspace_lock));

    for (member, dep_version) in vec![("lib_a", "0.2.1"), ("lib_b", "0.1.0")].iter() {
      let member_dir = crate_dir.as_ref().join(&member);
      std::fs::create_dir_all(&member_dir).unwrap();
      std::fs::write(
        member_dir.join("Cargo.toml"),
        dummy_workspace_member_toml_contents(member, dep_version),
      )
      .unwrap();
    }

    fetcher.fetch_metadata(&files, None, None).unwrap()
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
      .find(|ctx| ctx.pkg_name == "unicode-xid" && ctx.pkg_version == Version::from((0, 1, 0)))
      .is_some());

    assert!(planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "unicode-xid" && ctx.pkg_version == Version::from((0, 2, 1)))
      .is_some());
  }
  // TODO(acmcarther): Add tests:
  // TODO(acmcarther): Extra flags work
  // TODO(acmcarther): Extra deps work
  // TODO(acmcarther): Buildrs works
  // TODO(acmcarther): Extra aliases work
  // TODO(acmcarther): Skipped deps work
}
