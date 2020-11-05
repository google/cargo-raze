// Copyright 2020 Google Inc.
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
  io, iter,
  path::{Path, PathBuf},
  str::FromStr,
};

use anyhow::Result;

use cargo_lock::{lockfile::Lockfile, SourceId, Version};
use cargo_platform::Platform;

use itertools::Itertools;

use crate::{
  context::{
    BuildableDependency, BuildableTarget, CrateContext, CrateDependencyContext,
    CrateTargetedDepContext, DependencyAlias, GitRepo, LicenseData, SourceDetails,
    WorkspaceContext,
  },
  error::{RazeError, PLEASE_FILE_A_BUG},
  metadata::{fetch_crate_checksum, CargoWorkspaceFiles, DependencyKind, Node, Package},
  planning::license,
  settings::{format_registry_url, CrateSettings, GenMode, RazeSettings},
  util::{
    self, filter_bazel_triples, generate_bazel_conditions, get_matching_bazel_triples,
    is_bazel_supported_platform, PlatformDetails,
  },
};

use super::{
  checks,
  crate_catalog::{CrateCatalog, CrateCatalogEntry},
  PlannedBuild,
};

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

/** A set of dependencies that a crate has for a specific target/cfg */
struct TargetedDependencySet {
  target: String,
  dependencies: DependencySet,
}

/** An internal working planner for generating context for an individual crate. */
struct CrateSubplanner<'planner> {
  // Workspace-Wide details
  settings: &'planner RazeSettings,
  platform_details: &'planner Option<PlatformDetails>,
  crate_catalog: &'planner CrateCatalog,
  // Crate specific content
  crate_catalog_entry: &'planner CrateCatalogEntry,
  source_id: &'planner Option<SourceId>,
  node: &'planner Node,
  crate_settings: Option<&'planner CrateSettings>,
  sha256: &'planner Option<String>,
}

/** An internal working planner for generating context for a whole workspace. */
pub struct WorkspaceSubplanner<'planner> {
  pub(super) settings: &'planner RazeSettings,
  pub(super) platform_details: &'planner Option<PlatformDetails>,
  pub(super) crate_catalog: &'planner CrateCatalog,
  pub(super) files: &'planner CargoWorkspaceFiles,
  pub(super) binary_dependencies: &'planner Vec<CrateCatalog>,
  pub(super) binary_deps_files: &'planner HashMap<String, CargoWorkspaceFiles>,
}

impl<'planner> WorkspaceSubplanner<'planner> {
  /** Produces a planned build using internal state. */
  pub fn produce_planned_build(&self) -> Result<PlannedBuild> {
    checks::check_resolve_matches_packages(&self.crate_catalog.metadata)?;

    let mut packages: Vec<&Package> = self.crate_catalog.metadata.packages.iter().collect();

    match self.settings.genmode {
      GenMode::Remote => {
        for bin_dep in self.binary_dependencies {
          checks::check_resolve_matches_packages(&bin_dep.metadata)?;
          packages.extend(bin_dep.metadata.packages.iter());
        }
      },
      GenMode::Vendored => {
        checks::check_all_vendored(self.crate_catalog.entries(), &self.settings.workspace_path)?;
      },
      _ => { /* No checks to perform */ },
    }

    checks::warn_unused_settings(&self.settings.crates, &packages);

    let crate_contexts = self.produce_crate_contexts()?;

    Ok(PlannedBuild {
      workspace_context: self.produce_workspace_context(),
      crate_contexts,
      binary_crate_files: self.binary_deps_files.clone(),
    })
  }

  /** Constructs a workspace context from settings. */
  fn produce_workspace_context(&self) -> WorkspaceContext {
    WorkspaceContext {
      workspace_path: self.settings.workspace_path.clone(),
      gen_workspace_prefix: self.settings.gen_workspace_prefix.clone(),
      output_buildfile_suffix: self.settings.output_buildfile_suffix.clone(),
    }
  }

  fn create_crate_context(
    &self,
    node: &Node,
    package_to_checksum: &HashMap<(String, Version), String>,
    catalog: &CrateCatalog,
  ) -> Option<Result<CrateContext>> {
    let own_crate_catalog_entry = catalog.entry_for_package_id(&node.id)?;
    let own_package = own_crate_catalog_entry.package();

    let checksum_opt =
      package_to_checksum.get(&(own_package.name.clone(), own_package.version.clone()));

    let is_binary_dep = self
      .settings
      .binary_deps
      .keys()
      .any(|key| key == &own_package.name);

    // Skip the root package (which is probably a junk package, by convention) unless it's a binary dir
    if !is_binary_dep && own_crate_catalog_entry.is_root() {
      return None;
    }

    // Skip workspace crates, since we haven't yet decided how they should be handled.
    //
    // Except in the case of binary dependencies. They can handle these no problem
    //
    // Hey you, reader! If you have opinions about this please comment on the below bug, or file
    // another bug.
    // See Also: https://github.com/google/cargo-raze/issues/111
    if !is_binary_dep && own_crate_catalog_entry.is_workspace_crate() {
      return None;
    }

    // UNWRAP: Safe given unwrap during serialize step of metadata
    let own_source_id = own_package
      .source
      .as_ref()
      .map(|s| SourceId::from_url(&s.to_string()).unwrap());

    let crate_settings = self.crate_settings(&own_package).ok()?;

    let crate_subplanner = CrateSubplanner {
      crate_catalog: &catalog,
      settings: self.settings,
      platform_details: self.platform_details,
      crate_catalog_entry: &own_crate_catalog_entry,
      source_id: &own_source_id,
      node: &node,
      crate_settings: crate_settings,
      sha256: &checksum_opt.map(|c| c.to_owned()),
    };

    Some(crate_subplanner.produce_context())
  }

  fn crate_settings(&self, package: &Package) -> Result<Option<&CrateSettings>> {
    self
      .settings
      .crates
      .get(&package.name)
      .map_or(Ok(None), |settings| {
        let mut versions = settings
          .iter()
          .filter(|(ver_req, _)| ver_req.matches(&package.version))
          .peekable();

        match versions.next() {
          // This is possible if the crate does not have any version overrides to match against
          None => Ok(None),
          Some((_, settings)) if versions.peek().is_none() => Ok(Some(settings)),
          Some(current) => Err(RazeError::Config {
            field_path_opt: None,
            message: format!(
              "Multiple potential semver matches `[{}]` found for `{}`",
              iter::once(current).chain(versions).map(|x| x.0).join(", "),
              &package.name
            ),
          }),
        }
      })
      .map_err(|e| e.into())
  }

  /** Produces a crate context for each declared crate and dependency. */
  fn produce_crate_contexts(&self) -> Result<Vec<CrateContext>> {
    // Build a list of all catalogs we want the context for
    let mut catalogs = vec![self.crate_catalog];
    for catalog in self.binary_dependencies.iter() {
      catalogs.push(catalog);
    }

    // Build a list of all workspace files
    let mut files: Vec<&CargoWorkspaceFiles> = self
      .binary_deps_files
      .iter()
      .map(|(_key, val)| val)
      .collect();
    files.push(self.files);

    // Gather the checksums for all packages in the lockfile
    // which have them.
    //
    // Store the representation of the package as a tuple
    // of (name, version) -> checksum.
    let mut package_to_checksum = HashMap::new();
    for workspace_files in files.iter() {
      if let Some(lock_path) = workspace_files.lock_path_opt.as_ref() {
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
    }

    // Additionally, the binary dependencies need to have their checksums added as well in
    // Remote GenMode configurations. Vendored GenMode relies on the behavior of `cargo vendor`
    // and doesn't perform any special logic to fetch binary dependency crates.
    if self.settings.genmode == GenMode::Remote {
      for (pkg, info) in self.settings.binary_deps.iter() {
        let version = semver::Version::parse(info.req())?;
        package_to_checksum.insert(
          (pkg.clone(), version.clone()),
          fetch_crate_checksum(&self.settings.index_url, pkg, &version.to_string())?,
        );
      }
    }

    let contexts = catalogs
      .iter()
      .map(|catalog| {
        (*catalog)
          .metadata
          .resolve
          .as_ref()
          .ok_or_else(|| RazeError::Generic("Missing resolve graph".into()))?
          .nodes
          .iter()
          .sorted_by_key(|n| &n.id)
          .filter_map(|node| self.create_crate_context(node, &package_to_checksum, catalog))
          .collect::<Result<Vec<CrateContext>>>()
      })
      .collect::<Result<Vec<Vec<CrateContext>>>>()?;

    Ok(contexts.iter().flat_map(|i| i.clone()).collect())
  }
}

impl<'planner> CrateSubplanner<'planner> {
  /** Builds a crate context from internal state. */
  fn produce_context(&self) -> Result<CrateContext> {
    let (
      DependencySet {
        build_deps,
        build_proc_macro_deps,
        proc_macro_deps,
        dev_deps,
        normal_deps,
        aliased_deps,
      },
      targeted_deps,
    ) = self.produce_deps()?;

    let package = self.crate_catalog_entry.package();

    let manifest_path = PathBuf::from(&package.manifest_path);
    assert!(manifest_path.is_absolute());
    let package_root = self.find_package_root_for_manifest(&manifest_path)?;

    let mut targets = self.produce_targets(&package_root)?;
    let build_script_target_opt = self.take_build_script_target(&mut targets);

    let mut lib_target_name = None;
    {
      for target in &targets {
        if target.kind == "lib" || target.kind == "proc-macro" {
          lib_target_name = Some(target.name.clone());
          break;
        }
      }
    }

    // Build a list of dependencies while addression a potential whitelist of target triples
    let mut filtered_deps = Vec::new();
    for dep_set in targeted_deps.iter() {
      let mut target_triples = get_matching_bazel_triples(&dep_set.target)?;
      filter_bazel_triples(
        &mut target_triples,
        self
          .settings
          .targets
          .as_ref()
          .unwrap_or(&Vec::<String>::new()),
      );

      if target_triples.len() == 0 {
        continue;
      }

      filtered_deps.push(CrateTargetedDepContext {
        target: dep_set.target.clone(),
        deps: CrateDependencyContext {
          dependencies: dep_set.dependencies.normal_deps.clone(),
          proc_macro_dependencies: dep_set.dependencies.proc_macro_deps.clone(),
          build_dependencies: dep_set.dependencies.build_deps.clone(),
          build_proc_macro_dependencies: dep_set.dependencies.build_proc_macro_deps.clone(),
          dev_dependencies: dep_set.dependencies.dev_deps.clone(),
          aliased_dependencies: dep_set.dependencies.aliased_deps.clone(),
        },
        conditions: generate_bazel_conditions(&target_triples)?,
      });
    }

    filtered_deps.sort();

    let context = CrateContext {
      pkg_name: package.name.clone(),
      pkg_version: package.version.clone(),
      edition: package.edition.clone(),
      license: self.produce_license(),
      features: self.node.features.clone(),
      is_root_dependency: self.crate_catalog_entry.is_root_dep(),
      default_deps: CrateDependencyContext {
        dependencies: normal_deps,
        proc_macro_dependencies: proc_macro_deps,
        build_dependencies: build_deps,
        build_proc_macro_dependencies: build_proc_macro_deps,
        dev_dependencies: dev_deps,
        aliased_dependencies: aliased_deps,
      },
      targeted_deps: filtered_deps,
      workspace_path_to_crate: self.crate_catalog_entry.workspace_path(&self.settings)?,
      build_script_target: build_script_target_opt,
      links: package.links.clone(),
      raze_settings: self.crate_settings.cloned().unwrap_or_default(),
      source_details: self.produce_source_details(&package, &package_root),
      expected_build_path: self.crate_catalog_entry.local_build_path(&self.settings)?,
      sha256: self.sha256.clone(),
      registry_url: format_registry_url(
        &self.settings.registry,
        &package.name,
        &package.version.to_string(),
      ),
      lib_target_name,
      targets,
    };

    Ok(context)
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

  fn _produce_deps(&self, names: &DependencyNames) -> Result<DependencySet> {
    let build_dep_names = &names.build_dep_names;
    let dev_dep_names = &names.dev_dep_names;
    let normal_dep_names = &names.normal_dep_names;
    let aliased_dep_names = &names.aliased_dep_names;

    let mut dep_set = DependencySet {
      build_deps: Vec::new(),
      build_proc_macro_deps: Vec::new(),
      proc_macro_deps: Vec::new(),
      dev_deps: Vec::new(),
      normal_deps: Vec::new(),
      aliased_deps: Vec::new(),
    };

    let all_skipped_deps = self
      .crate_settings
      .iter()
      .flat_map(|pkg| pkg.skipped_deps.iter())
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
        .workspace_path_and_default_target(&self.settings)?;

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
        version: dep_package.version.clone(),
        buildable_target: buildable_target.clone(),
        is_proc_macro,
      };

      if build_dep_names.contains(&dep_package.name) {
        if buildable_dependency.is_proc_macro {
          dep_set
            .build_proc_macro_deps
            .push(buildable_dependency.clone());
        } else {
          dep_set.build_deps.push(buildable_dependency.clone());
        }
      }

      if dev_dep_names.contains(&dep_package.name) {
        dep_set.dev_deps.push(buildable_dependency.clone());
      }

      if normal_dep_names.contains(&dep_package.name) {
        // sys crates build files may generate DEP_* environment variables that
        // need to be visible in their direct dependency build files.
        if dep_package.name.ends_with("-sys") {
          dep_set.build_deps.push(buildable_dependency.clone());
        }
        if buildable_dependency.is_proc_macro {
          dep_set.proc_macro_deps.push(buildable_dependency);
        } else {
          dep_set.normal_deps.push(buildable_dependency);
        }
        // Only add aliased normal deps to the Vec
        if let Some(alias) = aliased_dep_names.get(&dep_package.name) {
          dep_set.aliased_deps.push(DependencyAlias {
            target: buildable_target.clone(),
            alias: util::sanitize_ident(alias),
          })
        }
      }
    }

    dep_set.build_deps.sort();
    dep_set.build_proc_macro_deps.sort();
    dep_set.proc_macro_deps.sort();
    dep_set.dev_deps.sort();
    dep_set.normal_deps.sort();

    Ok(dep_set)
  }

  /** Generates the set of dependencies for the contained crate. */
  fn produce_deps(&self) -> Result<(DependencySet, Vec<TargetedDependencySet>)> {
    let (default_deps, targeted_deps) = self.identify_named_deps()?;

    let targeted_set = targeted_deps
      .iter()
      .map(|(target, deps)| TargetedDependencySet {
        target: target.clone(),
        dependencies: self._produce_deps(deps).unwrap(),
      })
      .collect::<Vec<TargetedDependencySet>>();

    Ok((self._produce_deps(&default_deps)?, targeted_set))
  }

  /** Yields the list of dependencies as described by the manifest (without version). */
  fn identify_named_deps(&self) -> Result<(DependencyNames, HashMap<String, DependencyNames>)> {
    // Resolve dependencies into types
    let mut default_dep_names = DependencyNames {
      build_dep_names: Vec::new(),
      dev_dep_names: Vec::new(),
      normal_dep_names: Vec::new(),
      aliased_dep_names: HashMap::new(),
    };

    let mut targeted_dep_names: HashMap<String, DependencyNames> = HashMap::new();

    let package = self.crate_catalog_entry.package();
    for dep in &package.dependencies {
      // This shadow allow for dependencies with target restrictions to override where
      // to write data about itself.
      let mut dep_names = &mut default_dep_names;

      if dep.target.is_some() {
        // UNWRAP: Safe from above check
        let target_str = format!("{}", dep.target.as_ref().unwrap());

        // Legacy behavior
        if let Some(platform_details) = &self.platform_details {
          if let Some(settings_target) = &self.settings.target {
            let platform = Platform::from_str(&target_str)?;

            // Skip this dep if it doesn't match our platform attributes
            if !platform.matches(settings_target, platform_details.attrs().as_ref()) {
              continue;
            }
          }
        }

        let (is_bazel_platform, matches_all_platforms) = is_bazel_supported_platform(&target_str);
        // If the target is not supported by Bazel, we ignore it
        if !is_bazel_platform {
          continue;
        }

        // In cases where the cfg target matches all platforms, we consider it a default dependency
        if !matches_all_platforms {
          // Ensure an entry is created for the 'conditional' dependency
          dep_names = match targeted_dep_names.get_mut(&target_str) {
            Some(targeted) => targeted,
            None => {
              // Create a new entry if one was not found
              targeted_dep_names.insert(
                target_str.clone(),
                DependencyNames {
                  normal_dep_names: Vec::new(),
                  build_dep_names: Vec::new(),
                  dev_dep_names: Vec::new(),
                  aliased_dep_names: HashMap::new(),
                },
              );
              // This unwrap should be safe given the insert above
              targeted_dep_names.get_mut(&target_str).unwrap()
            },
          };
        }
      }
      match dep.kind {
        DependencyKind::Normal => dep_names.normal_dep_names.push(dep.name.clone()),
        DependencyKind::Development => dep_names.dev_dep_names.push(dep.name.clone()),
        DependencyKind::Build => dep_names.build_dep_names.push(dep.name.clone()),
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
        },
      }

      // Check if the dependency has been renamed
      if let Some(alias) = dep.rename.as_ref() {
        dep_names
          .aliased_dep_names
          .insert(dep.name.clone(), alias.clone());
      }
    }

    Ok((default_dep_names, targeted_dep_names))
  }

  /** Generates source details for internal crate. */
  fn produce_source_details(&self, package: &Package, package_root: &Path) -> SourceDetails {
    SourceDetails {
      git_data: self.source_id.as_ref().filter(|id| id.is_git()).map(|id| {
        let manifest_parent = package.manifest_path.parent().unwrap();
        let path_to_crate_root = manifest_parent.strip_prefix(package_root).unwrap();
        let path_to_crate_root = if path_to_crate_root.components().next().is_some() {
          Some(path_to_crate_root.to_string_lossy().to_string())
        } else {
          None
        };
        GitRepo {
          remote: id.url().to_string(),
          commit: id.precise().unwrap().to_owned(),
          path_to_crate_root,
        }
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
      .and_then(|x| x.gen_buildrs)
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
  fn produce_targets(&self, package_root_path: &Path) -> Result<Vec<BuildableTarget>> {
    let mut targets = Vec::new();
    let package = self.crate_catalog_entry.package();
    for target in &package.targets {
      // Bazel uses / as a path delimiter, but / is not the path delimiter on all
      // operating systems (like Mac OS 9, or something people actually use like Windows).
      // Strip off the package root, decompose the path into parts and rejoin
      // them with '/'.
      let package_root_path_str = target
        .src_path
        .strip_prefix(package_root_path)
        .unwrap_or(&target.src_path)
        .components()
        .map(|c| c.as_os_str().to_str())
        .try_fold("".to_owned(), |res, v| Some(format!("{}/{}", res, v?)))
        .ok_or(io::Error::new(
          io::ErrorKind::InvalidData,
          format!(
            "{:?} contains non UTF-8 characters and is not a legal path in Bazel",
            &target.src_path
          ),
        ))?
        .trim_start_matches("/")
        .to_owned();

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
