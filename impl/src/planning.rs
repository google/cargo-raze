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
  collections::{HashMap, HashSet},
  path::PathBuf,
  str::{self, FromStr},
};

use anyhow::Result;

use cargo_lock::lockfile::Lockfile;
use cargo_lock::SourceId;
use cargo_platform::Platform;

use itertools::Itertools;

use crate::{
  context::{
    BuildableDependency, BuildableTarget, CrateContext, DependencyAlias, GitRepo, LicenseData,
    SourceDetails, WorkspaceContext,
  },
  license,
  metadata::{
    CargoWorkspaceFiles, DependencyKind, Metadata, MetadataFetcher, Node, Package, PackageId,
  },
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
  ) -> Result<PlannedBuild>;
}

/** A set of named dependencies (without version) derived from a package manifest. */
struct DependencyNames {
  // Dependencies that are required for all buildable targets of this crate
  normal_dep_names: Vec<String>,
  // Dependencies that are required for build script only
  build_dep_names: Vec<String>,
  // Dependencies that are required for tests
  dev_dep_names: Vec<String>,
  // Dependencies that have been renamed and need to be aliased in the build rule
  aliased_dep_names: HashMap<String, String>,
}

// TODO(acmcarther): Remove this struct -- move it into CrateContext.
/** A set of dependencies that a crate has broken down by type. */
struct DependencySet {
  // Dependencies that are required for all buildable targets of this crate
  normal_deps: Vec<BuildableDependency>,
  proc_macro_deps: Vec<BuildableDependency>,
  // Dependencies that are required for build script only
  build_deps: Vec<BuildableDependency>,
  // Dependencies that proc macros and are required for the build script only
  build_proc_macro_deps: Vec<BuildableDependency>,
  // Dependencies that are required for tests
  dev_deps: Vec<BuildableDependency>,
  // Dependencies that have been renamed and need to be aliased in the build rule
  aliased_deps: Vec<DependencyAlias>,
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
  package_id_to_entries_idx: HashMap<PackageId, usize>,
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
  files: &'planner CargoWorkspaceFiles,
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
  node: &'planner Node,
  crate_settings: &'planner CrateSettings,
  sha256: &'planner Option<String>,
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
    let sanitized_version = util::sanitize_ident(&package.version.clone().to_string());

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
  pub fn new(metadata: &Metadata) -> Result<Self> {
    let resolve = metadata
      .resolve
      .as_ref()
      .ok_or_else(|| RazeError::Generic("Missing resolve graph".into()))?;

    let root_resolve_node = {
      let root_id_opt = resolve
        .root
        .as_ref();

      if let Some(root_id) = root_id_opt {
        Some(resolve
          .nodes
          .iter()
          .find(|node| &node.id == root_id)
          .map(|n| &n.id)
          .ok_or_else(|| RazeError::Generic("Missing crate with root ID in resolve graph".into()))?)
      } else {
        None
      }
    };

    let workspace_crates = metadata
      .workspace_members
      .iter()
      .cloned()
      .collect::<HashSet<_>>();
    let root_direct_deps =
    resolve
      .nodes
      .iter()
      .filter(|n| workspace_crates.contains(&n.id))
      .map(|n| &n.dependencies)
      .flatten()
      .cloned()
      .collect::<HashSet<PackageId>>();

    // TODO: handle name collisions due to version differences
    let entries = metadata
      .packages
      .iter()
      .map(|package| {
        CrateCatalogEntry::new(
          package,
          root_resolve_node.contains(&&package.id),
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

    Ok(Self {
      entries,
      package_id_to_entries_idx,
    })
  }

  /** Yields the internally contained entry set. */
  pub fn entries(&self) -> &Vec<CrateCatalogEntry> {
    &self.entries
  }

  /** Finds and returns the catalog entry with the given package id if present. */
  pub fn entry_for_package_id(&self, package_id: &PackageId) -> Option<&CrateCatalogEntry> {
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
  ) -> Result<PlannedBuild> {
    let metadata = self.metadata_fetcher.fetch_metadata(&files)?;
    let crate_catalog = CrateCatalog::new(&metadata)?;

    let workspace_subplanner = WorkspaceSubplanner {
      crate_catalog: &crate_catalog,
      metadata: &metadata,
      settings: &settings,
      platform_details: &platform_details,
      files: &files,
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
  pub fn produce_planned_build(&self) -> Result<PlannedBuild> {
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
  fn produce_crate_contexts(&self) -> Result<Vec<CrateContext>> {
    // Gather the checksums for all packages in the lockfile
    // which have them.
    //
    // Store the representation of the package as a tuple
    // of (name, version) -> checksum.
    let mut package_to_checksum = HashMap::new();
    if let Some(lock_path) = self.files.lock_path_opt.as_ref() {
      let lockfile = Lockfile::load(lock_path.as_path())?;
      for package in lockfile.packages {
        if let Some(checksum) = package.checksum {
          package_to_checksum.insert(
            (package.name.to_string(), package.version),
            checksum.to_string(),
          );
        }
      }
    }

    self
      .metadata
      .resolve
      .as_ref()
      .ok_or_else(|| RazeError::Generic("Missing resolve graph".into()))?
      .nodes
      .iter()
      .sorted_by_key(|n| &n.id)
      .filter_map(|node| {
        // UNWRAP: Node packages guaranteed to exist by guard in `produce_planned_build`
        let own_crate_catalog_entry = self.crate_catalog.entry_for_package_id(&node.id).unwrap();
        let own_package = own_crate_catalog_entry.package();

        let checksum_opt =
          package_to_checksum.get(&(own_package.name.clone(), own_package.version.clone()));

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

        // UNWRAP: Safe given unwrap during serialize step of metadata
        let own_source_id = own_package
          .source
          .as_ref()
          .map(|s| SourceId::from_url(&s.to_string()).unwrap());

        let crate_subplanner = CrateSubplanner {
          crate_catalog: &self.crate_catalog,
          settings: self.settings,
          platform_details: self.platform_details,
          crate_catalog_entry: &own_crate_catalog_entry,
          source_id: &own_source_id,
          node: &node,
          crate_settings: &crate_settings,
          sha256: &checksum_opt.map(|c| c.to_owned()),
        };

        Some(crate_subplanner.produce_context())
      })
      .collect()
  }
}

impl<'planner> CrateSubplanner<'planner> {
  /** Builds a crate context from internal state. */
  fn produce_context(&self) -> Result<CrateContext> {
    let DependencySet {
      build_deps,
      build_proc_macro_deps,
      proc_macro_deps,
      dev_deps,
      normal_deps,
      aliased_deps,
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
      pkg_version: package.version.to_string(),
      edition: package.edition.clone(),
      license: self.produce_license(),
      features: self.node.features.clone(),
      is_root_dependency: self.crate_catalog_entry.is_root_dep(),
      dependencies: normal_deps,
      proc_macro_dependencies: proc_macro_deps,
      build_dependencies: build_deps,
      build_proc_macro_dependencies: build_proc_macro_deps,
      dev_dependencies: dev_deps,
      aliased_dependencies: aliased_deps,
      workspace_path_to_crate: self.crate_catalog_entry.workspace_path(&self.settings),
      build_script_target: build_script_target_opt,
      raze_settings: self.crate_settings.clone(),
      source_details: self.produce_source_details(),
      expected_build_path: self.crate_catalog_entry.local_build_path(&self.settings),
      sha256: self.sha256.clone(),
      lib_target_name,
      targets,
    })
  }

  /** Generates license data from internal crate details. */
  fn produce_license(&self) -> LicenseData {
    let licenses_str = self
      .crate_catalog_entry
      .package()
      .license
      .as_ref()
      .map_or("", String::as_str);

    license::get_license_from_str(licenses_str)
  }

  /** Generates the set of dependencies for the contained crate. */
  fn produce_deps(&self) -> Result<DependencySet> {
    let DependencyNames {
      build_dep_names,
      dev_dep_names,
      normal_dep_names,
      aliased_dep_names,
    } = self.identify_named_deps()?;

    let mut build_deps = Vec::new();
    let mut build_proc_macro_deps = Vec::new();
    let mut proc_macro_deps = Vec::new();
    let mut dev_deps = Vec::new();
    let mut normal_deps = Vec::new();
    let mut aliased_deps = Vec::new();

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

      // Implicitly dependencies are on the [lib] target from Cargo.toml (of which there is
      // guaranteed to be at most one).
      // In this function, we don't explicitly narrow to be considering only the [lib] Target - we
      // rely on the fact that only one [lib] is allowed in a Package, and so treat the Package
      // synonymously with the [lib] Target therein.
      // Only the [lib] target is allowed to be labelled as a proc-macro, so checking if "any"
      // target is a proc-macro is equivalent to checking if the [lib] target is a proc-macro (and
      // accordingly, whether we need to treat this dep like a proc-macro).
      let is_proc_macro = dep_package
        .targets
        .iter()
        .flat_map(|target| target.crate_types.iter())
        .any(|crate_type| crate_type.as_str() == "proc-macro");

      let buildable_dependency = BuildableDependency {
        name: dep_package.name.clone(),
        version: dep_package.version.to_string(),
        buildable_target: buildable_target.clone(),
        is_proc_macro,
      };

      if build_dep_names.contains(&dep_package.name) {
        if buildable_dependency.is_proc_macro {
          build_proc_macro_deps.push(buildable_dependency.clone());
        } else {
          build_deps.push(buildable_dependency.clone());
        }
      }

      if dev_dep_names.contains(&dep_package.name) {
        dev_deps.push(buildable_dependency.clone());
      }

      if normal_dep_names.contains(&dep_package.name) {
        // sys crates build files may generate DEP_* environment variables that
        // need to be visible in their direct dependency build files.
        if dep_package.name.ends_with("-sys") {
          build_deps.push(buildable_dependency.clone());
        }
        if buildable_dependency.is_proc_macro {
          proc_macro_deps.push(buildable_dependency);
        } else {
          normal_deps.push(buildable_dependency);
        }
        // Only add aliased normal deps to the Vec
        if let Some(alias) = aliased_dep_names.get(&dep_package.name) {
          aliased_deps.push(DependencyAlias {
            target: buildable_target.clone(),
            alias: util::sanitize_ident(alias),
          })
        }
      }
    }

    build_deps.sort();
    build_proc_macro_deps.sort();
    proc_macro_deps.sort();
    dev_deps.sort();
    normal_deps.sort();

    Ok(DependencySet {
      build_deps,
      build_proc_macro_deps,
      proc_macro_deps,
      dev_deps,
      normal_deps,
      aliased_deps,
    })
  }

  /** Yields the list of dependencies as described by the manifest (without version). */
  fn identify_named_deps(&self) -> Result<DependencyNames> {
    // Resolve dependencies into types
    let mut build_dep_names = Vec::new();
    let mut dev_dep_names = Vec::new();
    let mut normal_dep_names = Vec::new();

    // Store aliased dependencies in a HashMap
    let mut aliased_dep_names = HashMap::new();

    let platform_attrs = self.platform_details.attrs();
    let package = self.crate_catalog_entry.package();
    for dep in &package.dependencies {
      if dep.target.is_some() {
        // UNWRAP: Safe from above check
        let target_str = format!("{}", dep.target.as_ref().unwrap());
        let platform = Platform::from_str(&target_str)?;

        // Skip this dep if it doesn't match our platform attributes
        if !platform.matches(&self.settings.target, platform_attrs.as_ref()) {
          continue;
        }
      }

      match dep.kind {
        DependencyKind::Normal => normal_dep_names.push(dep.name.clone()),
        DependencyKind::Development => dev_dep_names.push(dep.name.clone()),
        DependencyKind::Build => build_dep_names.push(dep.name.clone()),
        _ => {
          return Err(
            RazeError::Planning {
              dependency_name_opt: Some(package.name.to_string()),
              message: format!(
                "Unhandlable dependency type {:?} on {} detected! {}",
                dep.kind, dep.name, PLEASE_FILE_A_BUG
              ),
            }
            .into(),
          )
        }
      }

      // Check if the dependency has been renamed
      if let Some(alias) = dep.rename.as_ref() {
        aliased_dep_names.insert(dep.name.clone(), alias.clone());
      }
    }

    Ok(DependencyNames {
      build_dep_names,
      dev_dep_names,
      normal_dep_names,
      aliased_dep_names,
    })
  }

  /** Generates source details for internal crate. */
  fn produce_source_details(&self) -> SourceDetails {
    SourceDetails {
      git_data: self
        .source_id
        .as_ref()
        .filter(|id| id.is_git())
        .map(|id| GitRepo {
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
    if !self
      .crate_settings
      .gen_buildrs
      .unwrap_or(self.settings.default_gen_buildrs)
    {
      return None;
    }

    all_targets
      .iter()
      .position(|t| t.kind == "custom-build")
      .map(|idx| all_targets.remove(idx))
  }

  /**
   * Produces the complete set of build targets specified by this crate.
   *
   * This function may access the file system. See #find_package_root_for_manifest for more
   * details.
   */
  fn produce_targets(&self) -> Result<Vec<BuildableTarget>> {
    let mut targets = Vec::new();
    let package = self.crate_catalog_entry.package();
    for target in &package.targets {
      let manifest_path = PathBuf::from(&package.manifest_path);
      assert!(manifest_path.is_absolute());

      let package_root_path = self.find_package_root_for_manifest(&manifest_path)?;

      // Trim the manifest_path parent dir from the target path (to give us the crate-local path)
      let mut package_root_path_str = target
        .src_path
        .to_string_lossy()
        .into_owned()
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
  fn find_package_root_for_manifest(&self, manifest_path: &PathBuf) -> Result<PathBuf> {
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

mod checks {
  use std::{
    collections::{HashMap, HashSet},
    env, fs,
  };

  use anyhow::Result;

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
  pub fn check_all_vendored(crate_catalog_entries: &[CrateCatalogEntry]) -> Result<()> {
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
    metadata::{CargoMetadataFetcher, Metadata, MetadataFetcher},
    planning::checks,
    settings::testing as settings_testing,
  };

  use super::*;
  use semver::Version;
  use std::fs::File;
  use std::io::Write;
  use tempdir::TempDir;

  fn basic_toml() -> &'static str {
    "
[package]
name = \"test\"
version = \"0.0.1\"

[lib]
path = \"not_a_file.rs\"
    "
  }

  fn basic_lock() -> &'static str {
    "
[[package]]
name = \"test\"
version = \"0.0.1\"
dependencies = [
]
    "
  }

  fn make_workspace(
    toml_file: &'static str,
    lock_file: Option<&'static str>,
  ) -> (TempDir, CargoWorkspaceFiles) {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let root_toml_path = {
      let path = dir.path().join("Cargo.toml");
      let mut toml = File::create(&path).unwrap();
      toml.write_all(toml_file.as_bytes()).unwrap();
      path
    };
    let lock_path = match lock_file {
      Some(lock_file) => {
        let path = dir.path().join("Cargo.lock");
        let mut lock = File::create(&path).unwrap();
        lock.write_all(lock_file.as_bytes()).unwrap();
        Some(path)
      }
      None => None,
    };
    let files = CargoWorkspaceFiles {
      lock_path_opt: lock_path,
      root_toml_path,
    };

    (dir, files)
  }

  fn make_basic_workspace() -> (TempDir, CargoWorkspaceFiles) {
    make_workspace(basic_toml(), Some(basic_lock()))
  }

  #[test]
  #[allow(non_snake_case)]
  fn test__checks__check_resolve_matches_packages_fails_correctly() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = CargoMetadataFetcher::default();
    let mut metadata = fetcher.fetch_metadata(&files).unwrap();

    // Invalidate the metadata, expect an error.
    metadata.packages = Vec::new();
    assert!(checks::check_resolve_matches_packages(&metadata).is_err());
  }

  #[test]
  #[allow(non_snake_case)]
  fn test__checks__check_resolve_matches_packages_works_correctly() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = CargoMetadataFetcher::default();
    let metadata = fetcher.fetch_metadata(&files).unwrap();

    // Should not panic with valid metadata.
    checks::check_resolve_matches_packages(&metadata).unwrap();
  }

  // A wrapper around a MetadataFetcher which drops the
  // resolved dependency graph from the acquired metadata.
  #[derive(Default)]
  struct ResolveDroppingMetadataFetcher {
    fetcher: CargoMetadataFetcher,
  }

  impl MetadataFetcher for ResolveDroppingMetadataFetcher {
    fn fetch_metadata(&mut self, files: &CargoWorkspaceFiles) -> Result<Metadata> {
      let mut metadata = self.fetcher.fetch_metadata(&files)?;
      assert!(metadata.resolve.is_some());
      metadata.resolve = None;
      Ok(metadata)
    }
  }

  #[test]
  fn test_plan_build_missing_resolve_returns_error() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = ResolveDroppingMetadataFetcher::default();
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );
    assert!(res.is_err());
  }

  // A wrapper around a MetadataFetcher which drops the
  // list of packages from the acquired metadata.
  #[derive(Default)]
  struct PackageDroppingMetadataFetcher {
    fetcher: CargoMetadataFetcher,
  }

  impl MetadataFetcher for PackageDroppingMetadataFetcher {
    fn fetch_metadata(&mut self, files: &CargoWorkspaceFiles) -> Result<Metadata> {
      let mut metadata = self.fetcher.fetch_metadata(&files)?;
      metadata.packages.clear();
      Ok(metadata)
    }
  }

  #[test]
  fn test_plan_build_missing_package_in_metadata() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = PackageDroppingMetadataFetcher::default();
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.is_err());
  }

  #[test]
  fn test_plan_build_minimum_workspace() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = CargoMetadataFetcher::default();
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  // A wrapper around a MetadataFetcher which injects a fake
  // dependency into the acquired metadata.
  #[derive(Default)]
  struct DependencyInjectingMetadataFetcher {
    fetcher: CargoMetadataFetcher,
  }

  impl MetadataFetcher for DependencyInjectingMetadataFetcher {
    fn fetch_metadata(&mut self, files: &CargoWorkspaceFiles) -> Result<Metadata> {
      let mut metadata = self.fetcher.fetch_metadata(&files)?;

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

      Ok(metadata)
    }
  }

  #[test]
  fn test_plan_build_minimum_root_dependency() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = DependencyInjectingMetadataFetcher::default();
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings_testing::dummy_raze_settings(),
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    let planned_build = planned_build_res.unwrap();
    assert_eq!(planned_build.crate_contexts.len(), 1);
    let dep = planned_build.crate_contexts.get(0).unwrap();
    assert_eq!(dep.pkg_name, "test_dep");
    assert_eq!(dep.is_root_dependency, true);
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
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = DependencyInjectingMetadataFetcher::default();

    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Vendored;
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res = planner.plan_build(
      &settings,
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.is_err());
  }

  // A wrapper around a MetadataFetcher which injects a fake
  // package into the workspace.
  #[derive(Default)]
  struct WorkspaceCrateMetadataFetcher {
    fetcher: CargoMetadataFetcher,
  }

  impl MetadataFetcher for WorkspaceCrateMetadataFetcher {
    fn fetch_metadata(&mut self, files: &CargoWorkspaceFiles) -> Result<Metadata> {
      let mut metadata = self.fetcher.fetch_metadata(&files)?;

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

      Ok(metadata)
    }
  }

  #[test]
  fn test_plan_build_ignores_workspace_crates() {
    let (_temp_dir, files) = make_basic_workspace();
    let mut fetcher = WorkspaceCrateMetadataFetcher::default();
    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Vendored;

    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(
      &settings,
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  #[test]
  fn test_plan_build_produces_aliased_dependencies() {
    let toml_file = "
    [package]
    name = \"advanced_toml\"
    version = \"0.1.0\"

    [lib]
    path = \"not_a_file.rs\"

    [dependencies]
    actix-web = \"2.0.0\"
    actix-rt = \"1.0.0\"
        ";
    let (_temp_dir, files) = make_workspace(toml_file, None);
    let mut fetcher = WorkspaceCrateMetadataFetcher::default();
    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    // N.B. This will fail if we don't correctly ignore workspace crates.
    let planned_build_res = planner.plan_build(
      &settings,
      files,
      PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
    );

    let crates_with_aliased_deps: Vec<CrateContext> = planned_build_res
      .unwrap()
      .crate_contexts
      .into_iter()
      .filter(|krate| krate.aliased_dependencies.len() != 0)
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

    assert!(actix_http_context.aliased_dependencies.len() == 1);
    assert!(
      actix_http_context.aliased_dependencies[0].target == "@raze_test__failure__0_1_8//:failure"
    );
    assert!(actix_http_context.aliased_dependencies[0].alias == "fail_ure");
  }

  #[test]
  fn test_plan_build_produces_proc_macro_dependencies() {
    let toml_file = "
    [package]
    name = \"advanced_toml\"
    version = \"0.1.0\"

    [lib]
    path = \"not_a_file.rs\"

    [dependencies]
    serde = { version = \"=1.0.112\", features = [\"derive\"] }
        ";
    let (_temp_dir, files) = make_workspace(toml_file, None);
    let mut fetcher = WorkspaceCrateMetadataFetcher::default();
    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build = planner
      .plan_build(
        &settings,
        files,
        PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
      )
      .unwrap();

    let serde = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "serde")
      .unwrap();

    let serde_derive_proc_macro_deps: Vec<_> = serde
      .proc_macro_dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(serde_derive_proc_macro_deps.len(), 1);

    let serde_derive_normal_deps: Vec<_> = serde
      .dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(serde_derive_normal_deps.len(), 0);
  }

  #[test]
  fn test_plan_build_produces_build_proc_macro_dependencies() {
    let toml_file = "
    [package]
    name = \"advanced_toml\"
    version = \"0.1.0\"

    [lib]
    path = \"not_a_file.rs\"

    [dependencies]
    markup5ever = \"=0.10.0\"
        ";
    let (_temp_dir, files) = make_workspace(toml_file, None);
    let mut fetcher = WorkspaceCrateMetadataFetcher::default();
    let mut settings = settings_testing::dummy_raze_settings();
    settings.genmode = GenMode::Remote;

    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build = planner
      .plan_build(
        &settings,
        files,
        PlatformDetails::new("some_target_triple".to_owned(), Vec::new() /* attrs */),
      )
      .unwrap();

    let markup = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "markup5ever")
      .unwrap();

    let markup_proc_macro_deps: Vec<_> = markup
      .proc_macro_dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(markup_proc_macro_deps.len(), 0);

    let markup_build_proc_macro_deps: Vec<_> = markup
      .build_proc_macro_dependencies
      .iter()
      .filter(|dep| dep.name == "serde_derive")
      .collect();
    assert_eq!(markup_build_proc_macro_deps.len(), 1);
  }
  // TODO(acmcarther): Add tests:
  // TODO(acmcarther): Extra flags work
  // TODO(acmcarther): Extra deps work
  // TODO(acmcarther): Buildrs works
  // TODO(acmcarther): Extra aliases work
  // TODO(acmcarther): Skipped deps work
}
