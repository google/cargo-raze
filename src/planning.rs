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
use cargo::core::Dependency;
use cargo::core::Package as CargoPackage;
use cargo::core::PackageId;
use cargo::core::PackageSet;
use cargo::core::Resolve;
use cargo::core::SourceId;
use cargo::core::Workspace;
use cargo::core::dependency::Kind;
use cargo::ops;
use cargo::ops::Packages;
use cargo::util::CargoResult;
use cargo::util::Cfg;
use cargo::util::Config;
use cargo::util::ToUrl;
use context::BuildDependency;
use context::BuildTarget;
use context::CrateContext;
use context::LicenseData;
use context::WorkspaceContext;
use license;
use settings::GenMode;
use settings::RazeSettings;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::str;
use util;

pub struct PlannedBuild {
  pub workspace_context: WorkspaceContext,
  pub crate_contexts: Vec<CrateContext>,
}

pub struct BuildPlanner<'a> {
  settings: RazeSettings,
  cargo_config: &'a Config,
  platform_attrs: Vec<Cfg>,
  registry: Option<SourceId>,
}

impl<'a> BuildPlanner<'a> {
  pub fn new(settings: RazeSettings, cargo_config: &'a Config) -> CargoResult<BuildPlanner<'a>> {
    Ok(BuildPlanner {
      platform_attrs: try!(util::fetch_attrs(&settings.target)),
      cargo_config: cargo_config,
      registry: None,
      settings: settings,
    })
  }

  pub fn set_registry_from_url(&mut self, host: String) -> CargoResult<()> {
    match host.to_url().map(|url| SourceId::for_registry(&url)) {
      Ok(registry_id) => {
        self.registry = Some(registry_id);
        Ok(())
      },
      Err(value) => Err(CargoError::from(value)),
    }
  }

  pub fn plan_build(&self) -> CargoResult<PlannedBuild> {
    let ResolvedPlan {
      root_name,
      packages,
      resolve,
    } = try!(ResolvedPlan::resolve_from_files(&self.cargo_config));

    let root_package_id = try!(
      resolve
        .iter()
        .filter(|dep| dep.name() == root_name)
        .next()
        .ok_or(CargoError::from("root crate should be in cargo resolve"))
    );

    // TODO:(get the root package here)
    let root_direct_deps = resolve.deps(&root_package_id).cloned().collect::<HashSet<_>>();

    let mut crate_contexts = Vec::new();

    let source_id = match self.registry.clone() {
      Some(v) => v,
      None => try!(SourceId::crates_io(&self.cargo_config)),
    };

    for id in try!(find_all_package_ids(source_id, &resolve)) {
      let package = packages.get(&id).unwrap().clone();
      let mut features = resolve.features(&id).clone().into_iter().collect::<Vec<_>>();
      features.sort();
      let full_name = format!("{}-{}", id.name(), id.version());
      let path = format!("./vendor/{}-{}/", id.name(), id.version());

      // Verify that package is really vendored
      if self.settings.genmode == GenMode::Vendored {
        try!(fs::metadata(&path).map_err(|_| CargoError::from(format!(
          "failed to find {}. Either switch to \"Remote\" genmode, or run `cargo vendor -x` first.",
          &path
        ))));
      }

      // Identify all possible dependencies
      let PlannedDeps {
        mut build_deps,
        mut dev_deps,
        mut normal_deps,
      } = PlannedDeps::find_all_deps(
        &id,
        &package,
        &resolve,
        &self.settings.target,
        &self.platform_attrs,
      );
      build_deps.sort();
      dev_deps.sort();
      normal_deps.sort();

      let mut targets = try!(identify_targets(&full_name, &package));
      targets.sort();

      let possible_crate_settings =
        self.settings.crates.get(id.name()).and_then(|c| c.get(&id.version().to_string()));

      let should_gen_buildrs =
        possible_crate_settings.map(|s| s.gen_buildrs.clone()).unwrap_or(false);
      let build_script_target = if should_gen_buildrs {
        targets.iter().find(|t| t.kind.deref() == "custom-build").cloned()
      } else {
        None
      };

      let targets_sans_build_script =
        targets.into_iter().filter(|t| t.kind.deref() != "custom-build").collect::<Vec<_>>();

      let additional_deps =
        possible_crate_settings.map(|s| s.additional_deps.clone()).unwrap_or(Vec::new());

      let additional_flags =
        possible_crate_settings.map(|s| s.additional_flags.clone()).unwrap_or(Vec::new());

      let extra_aliased_targets =
        possible_crate_settings.map(|s| s.extra_aliased_targets.clone()).unwrap_or(Vec::new());

      // Skip generated dependencies explicitly designated to be skipped (potentially due to
      // being replaced or customized as part of additional_deps)
      let non_skipped_normal_deps = if let Some(s) = possible_crate_settings {
        normal_deps
          .into_iter()
          .filter(|d| !s.skipped_deps.contains(&format!("{}-{}", d.name, d.version)))
          .collect::<Vec<_>>()
      } else {
        normal_deps
      };

      let cargo_license_string = if let Some(ref l) = package.manifest().metadata().license {
        l.clone()
      } else {
        String::new()
      };

      let licenses = license::get_available_licenses(&cargo_license_string)
        .into_iter()
        .map(|(name, rating)| LicenseData {
          name: name,
          rating: rating.to_string(),
        })
        .collect();

      crate_contexts.push(CrateContext {
        pkg_name: id.name().to_owned(),
        pkg_version: id.version().to_string(),
        licenses: licenses,
        features: features,
        is_root_dependency: root_direct_deps.contains(&id),
        metadeps: Vec::new(), /* TODO(acmcarther) */
        dependencies: non_skipped_normal_deps,
        build_dependencies: build_deps,
        dev_dependencies: dev_deps,
        path: path,
        build_script_target: build_script_target,
        targets: targets_sans_build_script,
        platform_triple: self.settings.target.to_owned(),
        additional_deps: additional_deps,
        additional_flags: additional_flags,
        extra_aliased_targets: extra_aliased_targets,
      })
    }

    let workspace_context = WorkspaceContext {
      workspace_path: self.settings.workspace_path.clone(),
      platform_triple: self.settings.target.clone(),
      gen_workspace_prefix: self.settings.gen_workspace_prefix.clone(),
    };

    crate_contexts.sort_by_key(|context| format!("{}-{}", context.pkg_name, context.pkg_version));

    Ok(PlannedBuild {
      workspace_context: workspace_context,
      crate_contexts: crate_contexts,
    })
  }
}

/** The set of all included dependencies for Cargo's dependency categories. */
pub struct PlannedDeps {
  pub build_deps: Vec<BuildDependency>,
  pub dev_deps: Vec<BuildDependency>,
  pub normal_deps: Vec<BuildDependency>,
}

impl PlannedDeps {
  /**
   * Identifies the full set of cargo dependencies for the provided package id using cargo's
   * resolution details.
   */
  pub fn find_all_deps(
    id: &PackageId,
    package: &CargoPackage,
    resolve: &Resolve,
    platform_triple: &str,
    platform_attrs: &Vec<Cfg>,
  ) -> PlannedDeps {
    let platform_deps = package
      .dependencies()
      .iter()
      .filter(|dep| {
        dep.platform().map(|p| p.matches(&platform_triple, Some(&platform_attrs))).unwrap_or(true)
      })
      .cloned()
      .collect::<Vec<Dependency>>();
    let build_deps = util::take_kinded_dep_names(&platform_deps, Kind::Build);
    let dev_deps = util::take_kinded_dep_names(&platform_deps, Kind::Development);
    let normal_deps = util::take_kinded_dep_names(&platform_deps, Kind::Normal);
    let resolved_deps = resolve
      .deps(&id)
      .into_iter()
      .map(|dep| BuildDependency {
        name: dep.name().to_owned(),
        version: dep.version().to_string(),
      })
      .collect::<Vec<BuildDependency>>();

    PlannedDeps {
      normal_deps: resolved_deps
        .iter()
        .filter(|d| normal_deps.contains(&d.name))
        .cloned()
        .collect(),
      build_deps: resolved_deps.iter().filter(|d| build_deps.contains(&d.name)).cloned().collect(),
      dev_deps: resolved_deps.into_iter().filter(|d| dev_deps.contains(&d.name)).collect(),
    }
  }
}

/** A synthesized Cargo dependency resolution. */
pub struct ResolvedPlan<'a> {
  pub root_name: String,
  pub packages: PackageSet<'a>,
  pub resolve: Resolve,
}

impl<'a> ResolvedPlan<'a> {
  /**
   * Performs Cargo's own build plan resolution, yielding the root crate, the set of packages, and
   * the resolution graph.
   */
  pub fn resolve_from_files(cargo_config: &Config) -> CargoResult<ResolvedPlan> {
    let lockfile = Path::new("Cargo.lock");
    let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
    let manifest = env::current_dir().unwrap().join(&manifest_path);
    let ws = try!(Workspace::new(&manifest, cargo_config));
    let specs = Packages::All.into_package_id_specs(&ws)?;
    let root_name = specs.iter().next().unwrap().name().to_owned();

    let (packages, resolve) = ops::resolve_ws_precisely(&ws, None, &[], false, false, &specs)?;

    Ok(ResolvedPlan {
      root_name: root_name,
      packages: packages,
      resolve: resolve,
    })
  }
}

/** Enumerates the set of all possibly relevant packages for the Cargo dependencies */
fn find_all_package_ids(registry_id: SourceId, resolve: &Resolve) -> CargoResult<Vec<PackageId>> {
  try!(fs::metadata("Cargo.lock").map_err(|_| CargoError::from(
    "failed to find Cargo.lock. Please run `cargo generate-lockfile` first."
  )));

  let mut package_ids =
    resolve.iter().filter(|id| *id.source_id() == registry_id).cloned().collect::<Vec<_>>();
  package_ids.sort_by_key(|id| id.name().to_owned());
  Ok(package_ids)
}

/** Derives target objects from Cargo's target information. */
fn identify_targets(full_name: &str, package: &CargoPackage) -> CargoResult<Vec<BuildTarget>> {
  let partial_path = format!("{}/", full_name);
  let partial_path_byte_length = partial_path.as_bytes().len();
  let mut targets = Vec::new();

  for target in package.targets().iter() {
    let target_path_str = try!(target.src_path().to_str().ok_or(CargoError::from(format!(
      "path for {}'s target {} wasn't unicode",
      &full_name,
      target.name()
    )))).to_owned();
    let crate_name_str_idx =
      try!(target_path_str.find(&partial_path).ok_or(CargoError::from(format!(
        "path for {}'s target {} should have been in vendor directory",
        &full_name,
        target.name()
      ))));
    let local_path_bytes = target_path_str
      .bytes()
      .skip(crate_name_str_idx + partial_path_byte_length)
      .collect::<Vec<_>>();
    let local_path_str = String::from_utf8(local_path_bytes).unwrap();
    for kind in util::kind_to_kinds(target.kind()) {
      targets.push(BuildTarget {
        name: target.name().to_owned(),
        path: local_path_str.clone(),
        kind: kind,
      });
    }
  }

  Ok(targets)
}
