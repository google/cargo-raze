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
  collections::{BTreeMap, HashMap, HashSet},
  path::PathBuf,
  str::{self, FromStr},
};

use cargo::{
  core::SourceId,
  CargoResult,
};
use cargo_platform::Platform;

use itertools::Itertools;

use crate::{
  context::{
    BuildableDependency, BuildableTarget, CrateContext, GitRepo, LicenseData, SourceDetails,
    WorkspaceContext,
  },
  license,
  metadata::{CargoWorkspaceFiles, Metadata, MetadataFetcher, Package, ResolveNode},
  settings::{CrateSettings, GenMode, RazeSettings},
  util::{self, PlatformDetails, RazeError, PLEASE_FILE_A_BUG},
};

pub const VENDOR_DIR: &str = "vendor/";

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
  // Is this a member of the root crate workspace?
  is_workspace_crate: bool,
}

/** An intermediate structure that contains details about all crates in the workspace. */
pub struct CrateCatalog {
  entries: Vec<CrateCatalogEntry>,
  package_id_to_entries_idx: HashMap<String, usize>,
}

/** The default implementation of a `BuildPlanner`. */
pub struct BuildPlannerImpl<'fetcher> {
  metadata_fetcher: &'fetcher mut dyn MetadataFetcher,
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
  pub fn new(
    package: &Package,
    is_root: bool,
    is_root_dep: bool,
    is_workspace_crate: bool,
  ) -> Self {
    let sanitized_name = package.name.replace("-", "_");
    let sanitized_version = util::sanitize_ident(&package.version);

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
  pub fn expected_vendored_path(&self) -> String {
    format!("./{}{}", VENDOR_DIR, &self.package_ident)
  }

  /** Yields the expected location of the build file (relative to execution path). */
  pub fn local_build_path(&self, settings: &RazeSettings) -> String {
    match settings.genmode {
      GenMode::Remote => format!(
        "remote/{}.{}",
        &self.package_ident, settings.output_buildfile_suffix,
      ),
      GenMode::Vendored => format!(
        "vendor/{}/{}",
        &self.package_ident, settings.output_buildfile_suffix,
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
      GenMode::Vendored => {
        // Convert "settings.workspace_path" to dir. Workspace roots are special cased, no need to append /
        if settings.workspace_path.ends_with("//") {
          format!("{}vendor/{}", settings.workspace_path, &self.package_ident)
        } else {
          format!("{}/vendor/{}", settings.workspace_path, &self.package_ident)
        }
      }
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
      GenMode::Vendored => {
        // Convert "settings.workspace_path" to dir. Workspace roots are special cased, no need to append /
        if settings.workspace_path.ends_with("//") {
          format!(
            "{}vendor/{}:{}",
            settings.workspace_path, &self.package_ident, &self.sanitized_name
          )
        } else {
          format!(
            "{}/vendor/{}:{}",
            settings.workspace_path, &self.package_ident, &self.sanitized_name
          )
        }
      }
    }
  }
}

impl CrateCatalog {
  /** Produces a CrateCatalog using the package entries from a metadata blob.*/
  pub fn new(metadata: &Metadata) -> Self {
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

    for (idx, entry) in entries.iter().enumerate() {
      let existing_value = package_id_to_entries_idx.insert(entry.package.id.clone(), idx);
      assert!(None == existing_value);
    }

    Self {
      entries,
      package_id_to_entries_idx,
    }
  }

  /** Yields the internally contained entry set. */
  pub fn entries(&self) -> &Vec<CrateCatalogEntry> {
    &self.entries
  }

  /** Finds and returns the catalog entry with the given package id if present. */
  pub fn entry_for_package_id(&self, package_id: &str) -> Option<&CrateCatalogEntry> {
    self
      .package_id_to_entries_idx
      .get(package_id)
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
    let metadata = self.metadata_fetcher.fetch_metadata(files)?;
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
  pub fn new(metadata_fetcher: &'fetcher mut dyn MetadataFetcher) -> Self {
    Self { metadata_fetcher }
  }
}

impl<'planner> WorkspaceSubplanner<'planner> {
  /** Produces a planned build using internal state. */
  pub fn produce_planned_build(&self) -> CargoResult<PlannedBuild> {
    checks::check_resolve_matches_packages(&self.metadata)?;

    if self.settings.genmode != GenMode::Remote {
      checks::check_all_vendored(self.crate_catalog.entries())?;
    }

    checks::warn_unused_settings(&self.settings.crates, &self.metadata.packages);

    let crate_contexts = self.produce_crate_contexts()?;

    Ok(PlannedBuild {
      workspace_context: self.produce_workspace_context(),
      crate_contexts,
    })
  }

  /** Constructs a workspace context from settings. */
  fn produce_workspace_context(&self) -> WorkspaceContext {
    WorkspaceContext {
      workspace_path: self.settings.workspace_path.clone(),
      platform_triple: self.settings.target.clone(),
      gen_workspace_prefix: self.settings.gen_workspace_prefix.clone(),
      output_buildfile_suffix: self.settings.output_buildfile_suffix.clone(),
    }
  }

  /** Produces a crate context for each declared crate and dependency. */
  fn produce_crate_contexts(&self) -> CargoResult<Vec<CrateContext>> {
    self
      .metadata
      .resolve
      .nodes
      .iter()
      .sorted_by_key(|n| &n.id)
      .filter_map(|node| {
        // UNWRAP: Node packages guaranteed to exist by guard in `produce_planned_build`
        let own_crate_catalog_entry = self.crate_catalog.entry_for_package_id(&node.id).unwrap();
        let own_package = own_crate_catalog_entry.package();

        // Skip the root package (which is probably a junk package, by convention)
        if own_crate_catalog_entry.is_root() {
          return None;
        }

        // Skip workspace crates, since we haven't yet decided how they should be handled.
        //
        // Hey you, reader! If you have opinions about this please comment on the below bug, or file
        // another bug.
        // See Also: https://github.com/google/cargo-raze/issues/111
        if own_crate_catalog_entry.is_workspace_crate() {
          return None;
        }

        let crate_settings = self
          .settings
          .crates
          .get(&own_package.name)
          .and_then(|c| c.get(&own_package.version))
          .cloned()
          .unwrap_or_else(CrateSettings::default);

        println!("DEBUG: {:?}", own_package.source);

        // UNWRAP: Safe given unwrap during serialize step of metadata
        let own_source_id = own_package
          .source
          .as_ref()
          .map(|s| SourceId::from_url(&s).unwrap());

        let crate_subplanner = CrateSubplanner {
          crate_catalog: &self.crate_catalog,
          settings: self.settings,
          platform_details: self.platform_details,
          crate_catalog_entry: &own_crate_catalog_entry,
          source_id: &own_source_id,
          node: &node,
          crate_settings: &crate_settings,
        };

        Some(crate_subplanner.produce_context())
      })
      .collect()
  }
}

impl<'planner> CrateSubplanner<'planner> {
  /** Builds a crate context from internal state. */
  fn produce_context(&self) -> CargoResult<CrateContext> {
    let DependencySet {
      build_deps,
      dev_deps,
      normal_deps,
    } = self.produce_deps()?;

    let mut targets = self.produce_targets()?;
    let build_script_target_opt = self.take_build_script_target(&mut targets);

    let package = self.crate_catalog_entry.package();
    let mut lib_target_name = None;
    {
      for target in &targets {
        if target.kind == "lib" || target.kind == "proc-macro" {
          lib_target_name = Some(target.name.clone());
          break;
        }
      }
    }
    Ok(CrateContext {
      pkg_name: package.name.clone(),
      pkg_version: package.version.clone(),
      edition: package.edition.clone(),
      licenses: self.produce_licenses(),
      features: self.node.features.clone().unwrap_or_default(),
      is_root_dependency: self.crate_catalog_entry.is_root_dep(),
      dependencies: normal_deps,
      build_dependencies: build_deps,
      dev_dependencies: dev_deps,
      workspace_path_to_crate: self.crate_catalog_entry.workspace_path(&self.settings),
      build_script_target: build_script_target_opt,
      raze_settings: self.crate_settings.clone(),
      source_details: self.produce_source_details(),
      expected_build_path: self.crate_catalog_entry.local_build_path(&self.settings),
      sha256: package.sha256.clone(),
      lib_target_name,
      targets,
    })
  }

  /** Generates license data from internal crate details. */
  fn produce_licenses(&self) -> Vec<LicenseData> {
    let licenses_str = self
      .crate_catalog_entry
      .package()
      .license
      .as_ref()
      .map_or("", String::as_str);
    load_and_dedup_licenses(licenses_str)
  }

  /** Generates the set of dependencies for the contained crate. */
  fn produce_deps(&self) -> CargoResult<DependencySet> {
    let DependencyNames {
      build_dep_names,
      dev_dep_names,
      normal_dep_names,
    } = self.identify_named_deps()?;

    let mut build_deps = Vec::new();
    let mut dev_deps = Vec::new();
    let mut normal_deps = Vec::new();
    let all_skipped_deps = self
      .crate_settings
      .skipped_deps
      .iter()
      .cloned()
      .collect::<HashSet<_>>();

    for dep_id in &self.node.dependencies {
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
        buildable_target,
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
      build_deps,
      dev_deps,
      normal_deps,
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
    for dep in &package.dependencies {
      if dep.target.is_some() {
        // UNWRAP: Safe from above check
        let target_str = dep.target.as_ref().unwrap();
        let platform = Platform::from_str(target_str)?;

        // Skip this dep if it doesn't match our platform attributes
        if !platform.matches(&self.settings.target, platform_attrs.as_ref()) {
          continue;
        }
      }

      match dep.kind.as_ref().map(|v| v.as_str()) {
        None | Some("normal") => normal_dep_names.push(dep.name.clone()),
        Some("dev") => dev_dep_names.push(dep.name.clone()),
        Some("build") => build_dep_names.push(dep.name.clone()),
        something_else => {
          return Err(
            RazeError::Planning {
              dependency_name_opt: Some(package.name.to_string()),
              message: format!(
                "Unhandlable dependency type {:?} on {} detected! {}",
                something_else, dep.name, PLEASE_FILE_A_BUG
              ),
            }
            .into(),
          )
        }
      }
    }

    Ok(DependencyNames {
      build_dep_names,
      dev_dep_names,
      normal_dep_names,
    })
  }

  /** Generates source details for internal crate. */
  fn produce_source_details(&self) -> SourceDetails {
    SourceDetails {
      git_data: self.source_id.filter(|id| id.is_git()).map(|id| GitRepo {
        remote: id.url().to_string(),
        commit: id.precise().unwrap().to_owned(),
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

    let build_script_target_idx_opt = all_targets.iter().position(|t| t.kind == "custom-build");
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
    for target in &package.targets {
      let manifest_path = PathBuf::from(&package.manifest_path);
      assert!(manifest_path.is_absolute());

      let package_root_path = self.find_package_root_for_manifest(&manifest_path)?;

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

      for kind in &target.kind {
        targets.push(BuildableTarget {
          name: target.name.clone(),
          path: package_root_path_str.clone(),
          kind: kind.clone(),
          edition: target.edition.clone(),
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
  fn find_package_root_for_manifest(&self, manifest_path: &PathBuf) -> CargoResult<PathBuf> {
    let has_git_repo_root = {
      let is_git = self.source_id.map_or(false, SourceId::is_git);
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
      Err(
        RazeError::Generic(format!(
          "Unable to locate git repository root for manifest at {:?}. {}",
          manifest_path, PLEASE_FILE_A_BUG
        ))
        .into(),
      )
    }
  }
}

fn load_and_dedup_licenses(licenses: &str) -> Vec<LicenseData> {
  let mut rating_to_license_name = BTreeMap::new();
  for (license_name, license_type) in license::get_available_licenses(licenses) {
    let rating = license_type.to_bazel_rating();

    rating_to_license_name
      .entry(rating.to_string())
      .and_modify(|license_names: &mut String| {
        license_names.push_str(",");
        license_names.push_str(&license_name);
      })
      .or_insert_with(|| license_name.to_owned());
  }

  rating_to_license_name
    .into_iter()
    .map(|(rating, name)| LicenseData { rating, name })
    .collect::<Vec<_>>()
}

mod checks {
  use std::{
    collections::{HashMap, HashSet},
    env, fs,
  };

  use cargo::util::CargoResult;

  use crate::{
    metadata::{Metadata, Package, PackageId},
    planning::{CrateCatalogEntry, VENDOR_DIR},
    settings::CrateSettingsPerVersion,
    util::{collect_up_to, RazeError},
  };

  // TODO(acmcarther): Consider including a switch to disable limiting
  const MAX_DISPLAYED_MISSING_VENDORED_CRATES: usize = 5;
  const MAX_DISPLAYED_MISSING_RESOLVE_PACKAGES: usize = 5;

  // Verifies that all provided packages are vendored (in VENDOR_DIR relative to CWD)
  pub fn check_all_vendored(crate_catalog_entries: &[CrateCatalogEntry]) -> CargoResult<()> {
    let missing_package_ident_iter = crate_catalog_entries
      .iter()
      // Root does not need to be vendored -- usually it is a wrapper package.
      .filter(|p| !p.is_root())
      .filter(|p| !p.is_workspace_crate())
      .filter(|p| fs::metadata(p.expected_vendored_path()).is_err())
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

    Err(RazeError::Planning {
      dependency_name_opt: None,
      message: format!(
        "Failed to find expected vendored crates in {:?}: {:?}. Did you forget to run cargo vendor?",
        expected_full_path.to_str(),
        limited_missing_crates)
      }.into())
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
    Err(
      RazeError::Planning {
        dependency_name_opt: None,
        message: format!(
          "Failed to find metadata.packages which were expected from metadata.resolve {:?}. {}",
          limited_missing_node_ids,
          crate::util::PLEASE_FILE_A_BUG
        ),
      }
      .into(),
    )
  }

  pub fn warn_unused_settings(
    all_crate_settings: &HashMap<String, CrateSettingsPerVersion>,
    all_packages: &[Package],
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
}

#[cfg(test)]
mod tests {
  use crate::{
    metadata::{testing as metadata_testing, testing::StubMetadataFetcher, Metadata, ResolveNode},
    planning::checks,
    settings::testing as settings_testing,
  };

  use super::*;

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
    let no_license_data = vec![LicenseData {
      name: "no license".to_owned(),
      rating: "restricted".to_owned(),
    }];

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

  #[test]
  fn test_plan_build_ignores_workspace_crates() {
    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Vendored;

    let workspace_crates_metadata = {
      let mut base_metadata = minimum_valid_metadata();
      let mut workspace_crate_dep = metadata_testing::dummy_package();
      workspace_crate_dep.name = "ws_crate_dep".to_owned();
      workspace_crate_dep.id = "ws_crate_dep_id".to_owned();
      workspace_crate_dep.version = "ws_crate_version".to_owned();
      base_metadata.packages.push(workspace_crate_dep);
      base_metadata
        .workspace_members
        .push("ws_crate_dep_id".to_owned());
      base_metadata
    };

    let mut fetcher = StubMetadataFetcher::with_metadata(workspace_crates_metadata);
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(
      &settings,
      dummy_workspace_files(),
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  // TODO(acmcarther): Add tests:
  // TODO(acmcarther): Extra flags work
  // TODO(acmcarther): Extra deps work
  // TODO(acmcarther): Buildrs works
  // TODO(acmcarther): Extra aliases work
  // TODO(acmcarther): Skipped deps work
}
