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

use cargo::CargoError;
use cargo::core::SourceId;
use cargo::core::dependency::Platform;
use cargo::util::CargoResult;
use context::BuildableDependency;
use context::BuildableTarget;
use context::CrateContext;
use context::GitRepo;
use context::LicenseData;
use context::SourceDetails;
use context::WorkspaceContext;
use license;
use metadata::CargoWorkspaceFiles;
use metadata::Metadata;
use metadata::MetadataFetcher;
use metadata::Package;
use metadata::ResolveNode;
use serde_json;
use settings::CrateSettings;
use settings::GenMode;
use settings::RazeSettings;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::str;
use util::PlatformDetails;
use util::RazeError;
use util::PLEASE_FILE_A_BUG;
use util;

pub const VENDOR_DIR: &'static str = "vendor/";

/** An entity that can produce an organized, planned build ready to be rendered. */
pub trait BuildPlanner {
  /**
   * A function that returns a completely planned build using internally generated metadata, along
   * with settings, platform specifications, and critical file locations.
   */
  fn plan_build(
    &mut self,
    settings: &RazeSettings,
    files: CargoWorkspaceFiles,
    platform_details: PlatformDetails,
  ) -> CargoResult<PlannedBuild>;
}

/** A set of named dependencies (without version) derived from a package manifest. */
struct DependencyNames {
  // Dependencies that are required for all buildable targets of this crate
  normal_dep_names: Vec<String>,
  // Dependencies that are required for build script only
  build_dep_names: Vec<String>,
  // Dependencies that are required for tests
  dev_dep_names: Vec<String>,
}

// TODO(acmcarther): Remove this struct -- move it into CrateContext.
/** A set of dependencies that a crate has broken down by type. */
struct DependencySet {
  // Dependencies that are required for all buildable targets of this crate
  normal_deps: Vec<BuildableDependency>,
  // Dependencies that are required for build script only
  build_deps: Vec<BuildableDependency>,
  // Dependencies that are required for tests
  dev_deps: Vec<BuildableDependency>,
}

/** An entry in the Crate catalog for a single crate. */
pub struct CrateCatalogEntry {
  // The package metadata for the crate
  package: Package,
  // The name of the package sanitized for use within Bazel
  sanitized_name: String,
  // The version of the package sanitized for use within Bazel
  sanitized_version: String,
  // A unique identifier for the package derived from Cargo usage of the form {name}-{version}
  package_ident: String,
  // Is this the root crate in the whole catalog?
  is_root: bool,
  // Is this a dependency of the catalog root crate?
  is_root_dep: bool,
}

/** An intermediate structure that contains details about all crates in the workspace. */
pub struct CrateCatalog {
  entries: Vec<CrateCatalogEntry>,
  package_id_to_entries_idx: HashMap<String, usize>,
}

/** The default implementation of a BuildPlanner. */
pub struct BuildPlannerImpl<'fetcher> {
  metadata_fetcher: &'fetcher mut MetadataFetcher,
}

/** An internal working planner for generating context for a whole workspace. */
struct WorkspaceSubplanner<'planner> {
  metadata: &'planner Metadata,
  settings: &'planner RazeSettings,
  platform_details: &'planner PlatformDetails,
  crate_catalog: &'planner CrateCatalog,
}

/** An internal working planner for generating context for an individual crate. */
struct CrateSubplanner<'planner> {
  // Workspace-Wide details
  settings: &'planner RazeSettings,
  platform_details: &'planner PlatformDetails,
  crate_catalog: &'planner CrateCatalog,
  // Crate specific content
  crate_catalog_entry: &'planner CrateCatalogEntry,
  source_id: &'planner Option<SourceId>,
  node: &'planner ResolveNode,
  crate_settings: &'planner CrateSettings,
}

/** A ready-to-be-rendered build, containing renderable context for each crate. */
#[derive(Debug)]
pub struct PlannedBuild {
  pub workspace_context: WorkspaceContext,
  pub crate_contexts: Vec<CrateContext>,
}

impl CrateCatalogEntry {

  pub fn new(package: &Package, is_root: bool, is_root_dep: bool) -> CrateCatalogEntry {
    let sanitized_name = util::sanitize_ident(&package.name);
    let sanitized_version = util::sanitize_ident(&package.version);

    CrateCatalogEntry {
      package: package.clone(),
      sanitized_name: sanitized_name,
      sanitized_version: sanitized_version,
      package_ident: format!("{}-{}", &package.name, &package.version),
      is_root: is_root,
      is_root_dep: is_root_dep,
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

  /** Returns whether or not this is a dependency of the workspace root crate.*/
  pub fn is_root_dep(&self) -> bool {
    self.is_root_dep
  }

  /**
   * Returns the packages expected path during current execution.
   *
   * Not for use except during planning as path is local to run location.
   */
  pub fn expected_vendored_path(&self) -> String {
    format!("./{}{}", VENDOR_DIR, &self.package_ident)
  }

  /** Yields the expected location of the build file (relative to execution path). */
  pub fn local_build_path(&self, settings: &RazeSettings) -> String {
    match settings.genmode {
      GenMode::Remote => format!(
        "remote/{}.BUILD",
        &self.package_ident
      ),
      GenMode::Vendored => format!(
        "vendor/{}/BUILD",
        &self.package_ident
      ),
    }
  }

  /** Yields the precise path to this dependency for the provided settings. */
  #[allow(dead_code)]
  pub fn workspace_path(&self, settings: &RazeSettings) -> String {
    match settings.genmode {
      GenMode::Remote => format!(
        "@{}__{}__{}//",
        &settings.gen_workspace_prefix, &self.sanitized_name, &self.sanitized_version
      ),
      GenMode::Vendored => format!(
        "{}/vendor/{}",
        &settings.workspace_path, &self.package_ident
      ),
    }
  }

  /** Emits a complete path to this dependency and default target using the given settings. */
  pub fn workspace_path_and_default_target(&self, settings: &RazeSettings) -> String {
    match settings.genmode {
      GenMode::Remote => format!(
        "@{}__{}__{}//:{}",
        &settings.gen_workspace_prefix,
        &self.sanitized_name,
        &self.sanitized_version,
        &self.sanitized_name
      ),
      GenMode::Vendored => format!(
        "{}/vendor/{}:{}",
        &settings.workspace_path, &self.package_ident, &self.sanitized_name
      ),
    }
  }
}

impl CrateCatalog {
  /** Produces a CrateCatalog using the package entries from a metadata blob.*/
  pub fn new(metadata: &Metadata) -> CrateCatalog {
    let root_resolve_node = {
      let root_resolve_node_opt = {
        let root_id = &metadata.resolve.root;
        metadata
          .resolve
          .nodes
          .iter()
          .find(|node| &node.id == root_id)
      };

      if root_resolve_node_opt.is_none() {
        eprintln!("Resolve was: {:#?}", metadata.resolve);
        eprintln!("root_id: {:?}", metadata.resolve.root);
        panic!("Resolve did not contain root crate!");
      }

      // UNWRAP: Guarded above
      root_resolve_node_opt.unwrap()
    };
    let root_direct_deps =
      root_resolve_node.dependencies.iter().cloned().collect::<HashSet<_>>();

    let crate_catalog_entries = metadata
      .packages
      .iter()
      .map(|package| CrateCatalogEntry::new(package,
                                            root_resolve_node.id == package.id,
                                            root_direct_deps.contains(&package.id)))
      .collect::<Vec<_>>();

    let mut package_id_to_entries_idx = HashMap::new();

    for (idx, crate_catalog_entry) in crate_catalog_entries.iter().enumerate() {
      let existing_value = package_id_to_entries_idx.insert(crate_catalog_entry.package.id.clone(), idx);
      assert!(None == existing_value);
    }

    CrateCatalog {
      entries: crate_catalog_entries,
      package_id_to_entries_idx: package_id_to_entries_idx,
    }
  }

  /** Yields the internally contained entry set. */
  pub fn entries(&self) -> &Vec<CrateCatalogEntry> {
    &self.entries
  }

  /** Finds and returns the catalog entry with the given package id if present. */
  pub fn entry_for_package_id(&self, package_id: &String) -> Option<&CrateCatalogEntry> {
    self.package_id_to_entries_idx.get(package_id)
      // UNWRAP: Indexes guaranteed to be valid -- structure is immutable
      .map(|entry_idx| self.entries.get(*entry_idx).unwrap())
  }
}

impl<'fetcher> BuildPlanner for BuildPlannerImpl<'fetcher> {
  /** Retrieves metadata for local workspace and produces a build plan. */
  fn plan_build(
    &mut self,
    settings: &RazeSettings,
    files: CargoWorkspaceFiles,
    platform_details: PlatformDetails,
  ) -> CargoResult<PlannedBuild> {
    let metadata = try!(self.metadata_fetcher.fetch_metadata(files));
    let crate_catalog = CrateCatalog::new(&metadata);

    let workspace_subplanner = WorkspaceSubplanner {
      crate_catalog: &crate_catalog,
      metadata: &metadata,
      settings: &settings,
      platform_details: &platform_details,
    };

    workspace_subplanner.produce_planned_build()
  }
}

impl<'fetcher> BuildPlannerImpl<'fetcher> {
  pub fn new(metadata_fetcher: &'fetcher mut MetadataFetcher) -> BuildPlannerImpl<'fetcher> {
    BuildPlannerImpl {
      metadata_fetcher: metadata_fetcher,
    }
  }
}

impl<'planner> WorkspaceSubplanner<'planner> {
  /** Produces a planned build using internal state. */
  pub fn produce_planned_build(&self) -> CargoResult<PlannedBuild> {
    try!(checks::check_resolve_matches_packages(&self.metadata));
    if self.settings.genmode == GenMode::Vendored {
      try!(checks::check_all_vendored(self.crate_catalog.entries()));
    }
    checks::warn_unused_settings(&self.settings.crates, &self.metadata.packages);

    let crate_contexts = try!(self.produce_crate_contexts());
    Ok(PlannedBuild {
      workspace_context: self.produce_workspace_context(),
      crate_contexts: crate_contexts,
    })
  }

  /** Constructs a workspace context from settings. */
  fn produce_workspace_context(&self) -> WorkspaceContext {
    WorkspaceContext {
      workspace_path: self.settings.workspace_path.clone(),
      platform_triple: self.settings.target.clone(),
      gen_workspace_prefix: self.settings.gen_workspace_prefix.clone(),
    }
  }

  /** Produces a crate context for each declared crate and dependency. */
  fn produce_crate_contexts(&self) -> CargoResult<Vec<CrateContext>> {
    let mut sorted_nodes: Vec<&ResolveNode> = self.metadata.resolve.nodes.iter().collect();
    sorted_nodes.sort_unstable_by_key(|n| &n.id);

    let mut crate_contexts = Vec::new();
    for node in sorted_nodes.into_iter() {
      // UNWRAP: Node packages guaranteed to exist by guard in `produce_planned_build`
      let own_crate_catalog_entry = self.crate_catalog.entry_for_package_id(&node.id).unwrap();
      let own_package = own_crate_catalog_entry.package();

      // Skip the root package (which is probably a junk package, by convention)
      if own_package.id == self.metadata.resolve.root {
        continue;
      }

      let crate_settings = self
        .settings
        .crates
        .get(&own_package.name)
        .and_then(|c| c.get(&own_package.version))
        .cloned()
        .unwrap_or_else(CrateSettings::default);

      // UNWRAP: Safe given unwrap during serialize step of metadata
      let own_source_id = own_package
        .source
        .as_ref()
        .map(|s| serde_json::from_str::<SourceId>(&s).unwrap());

      let crate_subplanner = CrateSubplanner {
        crate_catalog: &self.crate_catalog,
        settings: self.settings,
        platform_details: self.platform_details,
        crate_catalog_entry: &own_crate_catalog_entry,
        source_id: &own_source_id,
        node: &node,
        crate_settings: &crate_settings,
      };

      let crate_context = try!(crate_subplanner.produce_context());
      crate_contexts.push(crate_context);
    }

    Ok(crate_contexts)
  }
}

impl<'planner> CrateSubplanner<'planner> {
  /** Builds a crate context from internal state. */
  fn produce_context(&self) -> CargoResult<CrateContext> {
    let DependencySet {
      build_deps,
      dev_deps,
      normal_deps,
    } = try!(self.produce_deps());

    let mut targets = try!(self.produce_targets());
    let build_script_target_opt = self.take_build_script_target(&mut targets);

    let package = self.crate_catalog_entry.package();
    Ok(CrateContext {
      pkg_name: package.name.clone(),
      pkg_version: package.version.clone(),
      licenses: self.produce_licenses(),
      features: self.node.features.clone().unwrap_or(Vec::new()),
      is_root_dependency: self.crate_catalog_entry.is_root_dep(),
      dependencies: normal_deps,
      build_dependencies: build_deps,
      dev_dependencies: dev_deps,
      workspace_path_to_crate: self
        .crate_catalog_entry
        .workspace_path(&self.settings),
      build_script_target: build_script_target_opt,
      targets: targets,
      raze_settings: self.crate_settings.clone(),
      source_details: self.produce_source_details(),
      expected_build_path: self
        .crate_catalog_entry
        .local_build_path(&self.settings),
      sha256: package.sha256.clone(),
    })
  }

  /** Generates license data from internal crate details. */
  fn produce_licenses(&self) -> Vec<LicenseData> {
    let licenses_str = self
      .crate_catalog_entry
      .package()
      .license
      .as_ref()
      .map(String::as_str)
      .unwrap_or("");
    load_and_dedup_licenses(licenses_str)
  }

  /** Generates the set of dependencies for the contained crate. */
  fn produce_deps(&self) -> CargoResult<DependencySet> {
    let DependencyNames {
      build_dep_names,
      dev_dep_names,
      normal_dep_names,
    } = try!(self.identify_named_deps());

    let mut build_deps = Vec::new();
    let mut dev_deps = Vec::new();
    let mut normal_deps = Vec::new();
    let all_skipped_deps = self
      .crate_settings
      .skipped_deps
      .iter()
      .cloned()
      .collect::<HashSet<_>>();
    for dep_id in self.node.dependencies.iter() {
      // UNWRAP(s): Safe from verification of packages_by_id
      let dep_package = self
        .crate_catalog
        .entry_for_package_id(&dep_id)
        .unwrap()
        .package();

      // Skip settings-indicated deps to skip
      if all_skipped_deps.contains(&format!("{}-{}", dep_package.name, dep_package.version)) {
        continue;
      }

      // UNWRAP: Guaranteed to exist by checks in WorkspaceSubplanner#produce_build_plan
      let buildable_target = self
        .crate_catalog
        .entry_for_package_id(dep_id)
        .unwrap()
        .workspace_path_and_default_target(&self.settings);

      let buildable_dependency = BuildableDependency {
        name: dep_package.name.clone(),
        version: dep_package.version.clone(),
        buildable_target: buildable_target,
      };
      if build_dep_names.contains(&dep_package.name) {
        build_deps.push(buildable_dependency.clone());
      }

      if dev_dep_names.contains(&dep_package.name) {
        dev_deps.push(buildable_dependency.clone());
      }

      if normal_dep_names.contains(&dep_package.name) {
        normal_deps.push(buildable_dependency);
      }
    }

    build_deps.sort();
    dev_deps.sort();
    normal_deps.sort();

    Ok(DependencySet {
      build_deps: build_deps,
      dev_deps: dev_deps,
      normal_deps: normal_deps,
    })
  }

  /** Yields the list of dependencies as described by the manifest (without version). */
  fn identify_named_deps(&self) -> CargoResult<DependencyNames> {
    // Resolve dependencies into types
    let mut build_dep_names = Vec::new();
    let mut dev_dep_names = Vec::new();
    let mut normal_dep_names = Vec::new();

    let platform_attrs = self.platform_details.attrs();
    let package = self.crate_catalog_entry.package();
    for dep in package.dependencies.iter() {
      if dep.target.is_some() {
        // UNWRAP: Safe from above check
        let target_str = dep.target.as_ref().unwrap();
        let platform = try!(Platform::from_str(target_str));

        // Skip this dep if it doesn't match our platform attributes
        if !platform.matches(&self.settings.target, Some(&platform_attrs)) {
          continue;
        }
      }

      match dep.kind.as_ref().map(|v| v.as_str()) {
        None | Some("normal") => normal_dep_names.push(dep.name.clone()),
        Some("dev") => dev_dep_names.push(dep.name.clone()),
        Some("build") => build_dep_names.push(dep.name.clone()),
        something_else => {
          return Err(CargoError::from(RazeError::Planning {
            dependency_name_opt: Some(package.name.to_string()),
            message: format!(
              "Unhandlable dependency type {:?} on {} detected! {}",
              something_else,
              dep.name,
              PLEASE_FILE_A_BUG)
          }))
        }
      }
    }

    Ok(DependencyNames {
      build_dep_names: build_dep_names,
      dev_dep_names: dev_dep_names,
      normal_dep_names: normal_dep_names,
    })
  }

  /** Generates source details for internal crate. */
  fn produce_source_details(&self) -> SourceDetails {
    if self.source_id.is_none() {
      return SourceDetails { git_data: None };
    }

    // UNWRAP: Guarded above
    let source_id = self.source_id.as_ref().unwrap();
    if !source_id.is_git() {
      return SourceDetails { git_data: None };
    }

    SourceDetails {
      git_data: Some(GitRepo {
        remote: source_id.url().to_string(),
        commit: source_id.precise().unwrap().to_owned(),
      }),
    }
  }

  /**
   * Extracts the (one and only) build script target from the provided set of build targets.
   *
   * This function mutates the provided list of build arguments. It removes the first (and usually,
   * only) found build script target.
   */
  fn take_build_script_target(
    &self,
    all_targets: &mut Vec<BuildableTarget>,
  ) -> Option<BuildableTarget> {
    if !self.crate_settings.gen_buildrs {
      return None;
    }

    let build_script_target_idx_opt = all_targets
      .iter()
      .enumerate()
      .find(|&(_idx, t)| t.kind.as_str() == "custom-build")
      .map(|(idx, _t)| idx);

    let build_script_target_opt = build_script_target_idx_opt.map(|idx| all_targets.remove(idx));

    if !self.crate_settings.gen_buildrs {
      return None;
    }

    build_script_target_opt
  }

  /**
   * Produces the complete set of build targets specified by this crate.
   *
   * This function may access the file system. See #find_package_root_for_manifest for more
   * details.
   */
  fn produce_targets(&self) -> CargoResult<Vec<BuildableTarget>> {
    let mut targets = Vec::new();
    let package = self.crate_catalog_entry.package();
    for target in package.targets.iter() {
      let manifest_path = PathBuf::from(&package.manifest_path);
      assert!(manifest_path.is_absolute());

      let package_root_path = try!(self.find_package_root_for_manifest(manifest_path));

      // Trim the manifest_path parent dir from the target path (to give us the crate-local path)j
      let mut package_root_path_str = target
        .src_path
        .clone()
        // TODO(acmcarther): Is this even guaranteed to work? I don't think the `display` output
        // can be guaranteed....
        .split_off(package_root_path.display().to_string().len() + 1);

      // Some crates have a weird prefix, trim that.
      if package_root_path_str.starts_with("./") {
        package_root_path_str = package_root_path_str.split_off(2);
      }

      for kind in target.kind.iter() {
        targets.push(BuildableTarget {
          name: target.name.clone(),
          path: package_root_path_str.clone(),
          kind: kind.clone(),
        });
      }
    }

    targets.sort();
    Ok(targets)
  }

  /**
   * Finds the root of a contained git package.
   *
   * This function needs to access the file system if the dependency is a git dependency in order
   * to find the true filesystem root of the dependency. The root cause is that git dependencies
   * often aren't solely the crate of interest, but rather a repository that contains the crate of
   * interest among others.
   */
  fn find_package_root_for_manifest(&self, manifest_path: PathBuf) -> CargoResult<PathBuf> {
    let has_git_repo_root = {
      let is_git = self.source_id.as_ref().map_or(false, SourceId::is_git);
      is_git && self.settings.genmode == GenMode::Remote
    };

    // Return manifest path itself if not git
    if !has_git_repo_root {
      // TODO(acmcarther): How do we know parent is valid here?
      // UNWRAP: Pathbuf guaranteed to succeed from Path
      return Ok(PathBuf::from(manifest_path.parent().unwrap()));
    }

    // If package is git package it may be nested under a parent repository. We need to find the
    // package root.
    {
      let mut check_path = manifest_path.as_path();
      while let Some(c) = check_path.parent() {
        let joined = c.join(".git");
        if joined.is_dir() {
          // UNWRAP: Pathbuf guaranteed to succeed from Path
          return Ok(PathBuf::from(c));
        } else {
          check_path = c;
        }
      }

      // Reached filesystem root and did not find Git repo
      Err(CargoError::from(RazeError::Generic(format!(
        "Unable to locate git repository root for manifest at {:?}. {}",
        manifest_path, PLEASE_FILE_A_BUG
      ))))
    }
  }
}

fn load_and_dedup_licenses(licenses: &str) -> Vec<LicenseData> {
  let mut rating_to_license_name = HashMap::new();
  for (license_name, license_type) in license::get_available_licenses(licenses) {
    let rating = license_type.to_bazel_rating();

    if rating_to_license_name.contains_key(&rating) {
      let mut license_names_str: &mut String = rating_to_license_name.get_mut(&rating).unwrap();
      license_names_str.push_str(",");
      license_names_str.push_str(&license_name);
    } else {
      rating_to_license_name.insert(rating, license_name.to_owned());
    }
  }

  let mut license_data_list = rating_to_license_name
    .into_iter()
    .map(|(rating, name)| LicenseData {
      name: name,
      rating: rating.to_owned(),
    })
    .collect::<Vec<_>>();

  // Make output deterministic
  license_data_list.sort_by_key(|d| d.rating.clone());

  license_data_list
}

mod checks {
  use cargo::CargoError;
  use cargo::util::CargoResult;
  use metadata::Metadata;
  use metadata::Package;
  use metadata::PackageId;
  use planning::CrateCatalogEntry;
  use planning::VENDOR_DIR;
  use settings::CrateSettingsPerVersion;
  use std::collections::HashMap;
  use std::collections::HashSet;
  use std::env;
  use std::fs;
  use util::collect_up_to;
  use util::RazeError;

  // TODO(acmcarther): Consider including a switch to disable limiting
  const MAX_DISPLAYED_MISSING_VENDORED_CRATES: usize = 5;
  const MAX_DISPLAYED_MISSING_RESOLVE_PACKAGES: usize = 5;

  // Verifies that all provided packages are vendored (in VENDOR_DIR relative to CWD)
  pub fn check_all_vendored(crate_catalog_entries: &Vec<CrateCatalogEntry>) -> CargoResult<()> {
    let missing_package_ident_iter = crate_catalog_entries
      .iter()
      // Root does not need to be vendored -- usually it is a wrapper package.
      .filter(|p| !p.is_root())
      .filter(|p| !fs::metadata(p.expected_vendored_path()).is_ok())
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
    return Err(CargoError::from(RazeError::Planning {
          dependency_name_opt: None,
          message: format!(
            "Failed to find expected vendored crates in {:?}: {:?}. Did you forget to run cargo-vendor?",
            expected_full_path.to_str(),
            limited_missing_crates)
    }));
  }

  pub fn check_resolve_matches_packages(metadata: &Metadata) -> CargoResult<()> {
    let known_package_ids = metadata
      .packages
      .iter()
      .map(|p| p.id.clone())
      .collect::<HashSet<PackageId>>();

    let node_ids_missing_package_decl_iter = metadata
      .resolve
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
    return Err(CargoError::from(RazeError::Planning {
      dependency_name_opt: None,
      message: format!(
        "Failed to find metadata.packages which were expected from metadata.resolve {:?}. {}",
        limited_missing_node_ids,
        ::util::PLEASE_FILE_A_BUG)
    }));
  }

  pub fn warn_unused_settings(
    all_crate_settings: &HashMap<String, CrateSettingsPerVersion>,
    all_packages: &Vec<Package>,
  ) {
    let mut known_versions_per_crate = HashMap::new();
    for &Package {
      ref name,
      ref version,
      ..
    } in all_packages.iter()
    {
      known_versions_per_crate
        .entry(name.clone())
        .or_insert(HashSet::new())
        .insert(version.clone());
    }

    for (name, settings_per_version) in all_crate_settings.iter() {
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use metadata::Metadata;
  use metadata::ResolveNode;
  use metadata::testing as metadata_testing;
  use metadata::testing::StubMetadataFetcher;
  use settings::testing as settings_testing;
  use planning::checks;

  const ROOT_NODE_IDX: usize = 0;

  fn dummy_workspace_files() -> CargoWorkspaceFiles {
    CargoWorkspaceFiles {
      toml_path: PathBuf::from("/tmp/Cargo.toml"),
      lock_path_opt: None,
    }
  }

  fn minimum_valid_metadata() -> Metadata {
    let mut metadata = metadata_testing::dummy_metadata();
    metadata.resolve.root = "test_root_id".to_owned();
    metadata.resolve.nodes.push(ResolveNode {
      id: "test_root_id".to_owned(),
      dependencies: Vec::new(),
      features: None,
    });
    let mut test_package = metadata_testing::dummy_package();
    test_package.name = "test_root".to_owned();
    test_package.id = "test_root_id".to_owned();
    metadata.packages.push(test_package);

    metadata
  }

  fn minimum_dependency_metadata() -> Metadata {
    let mut metadata = minimum_valid_metadata();
    metadata.resolve.nodes[ROOT_NODE_IDX]
      .dependencies
      .push("test_dep_id".to_owned());
    metadata.resolve.nodes.push(ResolveNode {
      id: "test_dep_id".to_owned(),
      dependencies: Vec::new(),
      features: None,
    });
    let mut test_dep = metadata_testing::dummy_package();
    test_dep.name = "test_dep".to_owned();
    test_dep.id = "test_dep_id".to_owned();
    test_dep.version = "test_version".to_owned();
    metadata.packages.push(test_dep);
    metadata
  }

  #[test]
  #[allow(non_snake_case)]
  fn test__checks__check_resolve_matches_packages_fails_correctly() {
    let mut mangled_metadata = minimum_valid_metadata();
    mangled_metadata.packages = Vec::new();
    assert!(checks::check_resolve_matches_packages(&mangled_metadata).is_err());
  }

  #[test]
  #[allow(non_snake_case)]
  fn test__checks__check_resolve_matches_packages_works_correctly() {
    // Should not panic
    checks::check_resolve_matches_packages(&minimum_valid_metadata()).unwrap();
  }

  #[test]
  fn test_license_loading_works_with_no_license() {
    let no_license_data = vec![
      LicenseData {
        name: "no license".to_owned(),
        rating: "restricted".to_owned(),
      },
    ];

    assert_eq!(load_and_dedup_licenses(""), no_license_data);
    assert_eq!(load_and_dedup_licenses("///"), no_license_data);
  }

  #[test]
  fn test_license_loading_dedupes_equivalent_licenses() {
    // WTFPL is "disallowed",but we map that down to the same thing as GPL
    assert_eq!(
      load_and_dedup_licenses("Unlicense/ WTFPL /GPL-3.0"),
      vec![
        LicenseData {
          name: "GPL-3.0,WTFPL".to_owned(),
          rating: "restricted".to_owned(),
        },
        LicenseData {
          name: "Unlicense".to_owned(),
          rating: "unencumbered".to_owned(),
        },
      ]
    );
  }

  #[test]
  #[should_panic]
  fn test_plan_build_missing_resolve_panics() {
    let mut fetcher = StubMetadataFetcher::with_metadata(metadata_testing::dummy_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let _ = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      dummy_workspace_files(),
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );
  }

  #[test]
  fn test_plan_build_missing_package_in_metadata() {
    let mut metadata = metadata_testing::dummy_metadata();
    metadata.resolve.root = "test_root".to_owned();
    metadata.resolve.nodes.push(ResolveNode {
      id: "test_root".to_owned(),
      dependencies: Vec::new(),
      features: None,
    });

    let mut fetcher = StubMetadataFetcher::with_metadata(metadata);
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      dummy_workspace_files(),
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.is_err());
  }

  #[test]
  fn test_plan_build_minimum_workspace() {
    let mut fetcher = StubMetadataFetcher::with_metadata(minimum_valid_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      dummy_workspace_files(),
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  #[test]
  fn test_plan_build_minimum_root_dependency() {
    let mut fetcher = StubMetadataFetcher::with_metadata(minimum_dependency_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      dummy_workspace_files(),
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    let planned_build = planned_build_res.unwrap();
    assert_eq!(planned_build.crate_contexts.len(), 1);
    let dep = planned_build.crate_contexts.get(0).unwrap();
    assert_eq!(dep.pkg_name, "test_dep");
    assert_eq!(dep.is_root_dependency, true);
  }

  #[test]
  fn test_plan_build_verifies_vendored_state() {
    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Vendored;

    let mut fetcher = StubMetadataFetcher::with_metadata(minimum_dependency_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings,
      dummy_workspace_files(),
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.is_err());
  }

  // TODO(acmcarther): Add tests:
  // TODO(acmcarther): Extra flags work
  // TODO(acmcarther): Extra deps work
  // TODO(acmcarther): Buildrs works
  // TODO(acmcarther): Extra aliases work
  // TODO(acmcarther): Skipped deps work
}
