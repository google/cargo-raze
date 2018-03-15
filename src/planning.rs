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
use cargo::core::Workspace;
use cargo::core::dependency::Kind;
use cargo::core::dependency::Platform;
use cargo::ops;
use cargo::ops::Packages;
use cargo::util::CargoResult;
use cargo::util::Cfg;
use cargo::util::CfgExpr;
use cargo::util::Config;
use context::BuildDependency;
use context::BuildTarget;
use context::CrateContext;
use context::LicenseData;
use context::WorkspaceContext;
use license;
use metadata::Dependency;
use metadata::Metadata;
use metadata::Package;
use metadata::PackageId;
use metadata::Resolve;
use metadata::ResolveNode;
use metadata::Target;
use metadata::testing as metadata_testing;
use serde_json;
use settings::CrateSettings;
use settings::GenMode;
use settings::RazeSettings;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::str::FromStr;
use tempdir::TempDir;
use util;

/**
 * An entity that can retrive deserialized metadata for a Cargo Workspace.
 *
 * The "CargoInternalsMetadataFetcher" is probably the one you want.
 *
 * Usage of ..Subcommand.. is waiting on a cargo release containing
 * https://github.com/rust-lang/cargo/pull/5122
 */
pub trait MetadataFetcher {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata>;
}

pub trait BuildPlanner {
  fn plan_build(
    &mut self,
    settings: &RazeSettings,
    files: CargoWorkspaceFiles,
  ) -> CargoResult<PlannedBuild>;
}

pub struct CargoWorkspaceFiles {
  pub toml_path: PathBuf,
  pub lock_path_opt: Option<PathBuf>,
}

pub struct BuildPlannerImpl<'fetcher> {
  metadata_fetcher: &'fetcher mut MetadataFetcher,
}

pub struct PlannedBuild {
  pub workspace_context: WorkspaceContext,
  pub crate_contexts: Vec<CrateContext>,
}

pub struct CargoSubcommandMetadataFetcher;

pub struct CargoInternalsMetadataFetcher<'config> {
  cargo_config: &'config Config,
}

impl<'fetcher> BuildPlanner for BuildPlannerImpl<'fetcher> {
  fn plan_build(
    &mut self,
    settings: &RazeSettings,
    files: CargoWorkspaceFiles,
  ) -> CargoResult<PlannedBuild> {
    let metadata = try!(self.metadata_fetcher.fetch_metadata(files));
    if settings.genmode == GenMode::Vendored {
      self.assert_crates_vendored(&metadata);
    }
    let workspace_context = WorkspaceContext {
      workspace_path: settings.workspace_path.clone(),
      platform_triple: settings.target.clone(),
      gen_workspace_prefix: settings.gen_workspace_prefix.clone(),
    };

    let crate_contexts = try!(self.produce_crate_contexts(&settings, &metadata));

    Ok(PlannedBuild {
      crate_contexts: crate_contexts,
      workspace_context: workspace_context,
    })
  }
}

impl<'fetcher> BuildPlannerImpl<'fetcher> {
  pub fn new(metadata_fetcher: &'fetcher mut MetadataFetcher) -> BuildPlannerImpl<'fetcher> {
    BuildPlannerImpl {
      metadata_fetcher: metadata_fetcher,
    }
  }

  fn assert_crates_vendored(&self, metadata: &Metadata) {
    for package in metadata.packages.iter() {
      // Don't expect the root crate to be vendored
      if package.id == metadata.resolve.root {
        continue;
      }

      let full_name = format!("{}-{}", package.name, package.version);
      let path = format!("./vendor/{}/", full_name);

      if fs::metadata(&path).is_err() {
        panic!(format!(
          "failed to find {}. Either switch to \"Remote\" genmode, or run `cargo vendor -x` first.",
          &path
        ));
      };
    }
  }

  fn get_root_deps(&self, metadata: &Metadata) -> CargoResult<Vec<PackageId>> {
    let root_resolve_node_opt = {
      let root_id = &metadata.resolve.root;
      metadata.resolve.nodes.iter().find(|node| &node.id == root_id)
    };
    let root_resolve_node = if root_resolve_node_opt.is_some() {
      // UNWRAP: Guarded above
      root_resolve_node_opt.unwrap()
    } else {
      return Err(CargoError::from("Finding root crate details"));
    };
    Ok(root_resolve_node.dependencies.clone())
  }

  fn produce_crate_contexts(
    &self,
    settings: &RazeSettings,
    metadata: &Metadata,
  ) -> CargoResult<Vec<CrateContext>> {
    let root_direct_deps = try!(self.get_root_deps(&metadata));
    let packages_by_id = metadata
      .packages
      .iter()
      .map(|p| (p.id.clone(), p.clone()))
      .collect::<HashMap<PackageId, Package>>();

    // Verify that all nodes are present in package list
    {
      let mut missing_nodes = Vec::new();
      for node in metadata.resolve.nodes.iter() {
        if !packages_by_id.contains_key(&node.id) {
          missing_nodes.push(&node.id);
        }
      }
      if !missing_nodes.is_empty() {
        // This implies that we either have a mistaken understanding of Cargo resolution, or that
        // it broke.
        return Err(CargoError::from(format!(
          "Metadata.packages list was missing keys: {:?}",
          missing_nodes
        )));
      }
    }

    let mut crate_contexts = Vec::new();
    // TODO(acmcarther): handle unwrap
    let platform_attrs = util::fetch_attrs(&settings.target).unwrap();
    let mut sorted_nodes: Vec<&ResolveNode> = metadata.resolve.nodes.iter().collect();
    sorted_nodes.sort_unstable_by_key(|n| &n.id);
    for node in sorted_nodes.into_iter() {
      let own_package = packages_by_id.get(&node.id).unwrap();
      let full_name = format!("{}-{}", own_package.name, own_package.version);
      let path = format!("./vendor/{}/", full_name);

      // Skip the root package (which is probably a junk package, by convention)
      if own_package.id == metadata.resolve.root {
        continue;
      }

      // Resolve dependencies into types
      let mut build_dep_names = Vec::new();
      let mut dev_dep_names = Vec::new();
      let mut normal_dep_names = Vec::new();
      for dep in own_package.dependencies.iter() {
        if dep.target.is_some() {
          // UNWRAP: Safe from above check
          let target_str = dep.target.as_ref().unwrap();
          println!("target_str: {}", target_str);
          let platform = try!(Platform::from_str(target_str));

          // Skip this dep if it doesn't match our platform attributes
          if !platform.matches(&settings.target, Some(&platform_attrs)) {
            continue;
          }
        }

        match dep.kind.as_ref().map(|v| v.as_str()) {
          None | Some("normal") => normal_dep_names.push(dep.name.clone()),
          Some("dev") => dev_dep_names.push(dep.name.clone()),
          Some("build") => build_dep_names.push(dep.name.clone()),
          something_else => panic!(
            "Unhandlable dependency type {:?} for {} on {} detected!",
            something_else, own_package.name, dep.name
          ),
        }
      }

      let mut build_deps = Vec::new();
      let mut dev_deps = Vec::new();
      let mut normal_deps = Vec::new();
      for dep_id in node.dependencies.iter() {
        // UNWRAP: Safe from verification of packages_by_id
        let dep_package = packages_by_id.get(dep_id.as_str()).unwrap();
        let build_dependency = BuildDependency {
          name: dep_package.name.clone(),
          version: dep_package.version.clone(),
        };
        if build_dep_names.contains(&dep_package.name) {
          build_deps.push(build_dependency.clone());
        }

        if dev_dep_names.contains(&dep_package.name) {
          dev_deps.push(build_dependency.clone());
        }

        if normal_dep_names.contains(&dep_package.name) {
          normal_deps.push(build_dependency);
        }
      }
      build_deps.sort();
      dev_deps.sort();
      normal_deps.sort();

      let mut targets = try!(self.produce_targets(&own_package));
      targets.sort();

      let possible_crate_settings =
        settings.crates.get(&own_package.name).and_then(|c| c.get(&own_package.version));

      let should_gen_buildrs =
        possible_crate_settings.map(|s| s.gen_buildrs.clone()).unwrap_or(false);
      let build_script_target = if should_gen_buildrs {
        targets.iter().find(|t| t.kind.as_str() == "custom-build").cloned()
      } else {
        None
      };

      let targets_sans_build_script =
        targets.into_iter().filter(|t| t.kind.as_str() != "custom-build").collect::<Vec<_>>();

      let additional_deps =
        possible_crate_settings.map(|s| s.additional_deps.clone()).unwrap_or(Vec::new());

      let additional_flags =
        possible_crate_settings.map(|s| s.additional_flags.clone()).unwrap_or(Vec::new());

      let extra_aliased_targets =
        possible_crate_settings.map(|s| s.extra_aliased_targets.clone()).unwrap_or(Vec::new());

      // Skip generated dependencies explicitly designated to be skipped (potentially due to
      // being replaced or customized as part of additional_deps)
      let non_skipped_normal_deps = possible_crate_settings
        .map(|s| prune_skipped_deps(&normal_deps, s))
        .unwrap_or_else(|| normal_deps);
      let non_skipped_build_deps = possible_crate_settings
        .map(|s| prune_skipped_deps(&build_deps, s))
        .unwrap_or_else(|| build_deps);

      let license_str = own_package.license.as_ref().map(|s| s.as_str()).unwrap_or("");
      let licenses = load_and_dedup_licenses(license_str);

      let data_attr = possible_crate_settings.and_then(|s| s.data_attr.clone());

      crate_contexts.push(CrateContext {
        pkg_name: own_package.name.clone(),
        pkg_version: own_package.version.clone(),
        licenses: licenses,
        features: node.features.clone(),
        is_root_dependency: root_direct_deps.contains(&node.id),
        metadeps: Vec::new(), /* TODO(acmcarther) */
        dependencies: non_skipped_normal_deps,
        build_dependencies: non_skipped_build_deps,
        dev_dependencies: dev_deps,
        path: path,
        build_script_target: build_script_target,
        targets: targets_sans_build_script,
        platform_triple: settings.target.to_owned(),
        additional_deps: additional_deps,
        additional_flags: additional_flags,
        extra_aliased_targets: extra_aliased_targets,
        data_attr: data_attr,
      })
    }

    Ok(crate_contexts)
  }

  fn produce_targets(&self, package: &Package) -> CargoResult<Vec<BuildTarget>> {
    let full_name = format!("{}-{}", package.name, package.version);
    let partial_path = format!("{}/", full_name);
    let partial_path_byte_length = partial_path.as_bytes().len();
    let mut targets = Vec::new();
    for target in package.targets.iter() {
      // N.B. This error is really weird, but it boils down to us not being able to find the crate's
      // name as part of the complete path to the crate root.
      // For example, "/some/long/path/crate-version/lib.rs" should contain crate-version in the path
      // for crate at some version.
      let crate_name_str_idx =
        try!(target.src_path.find(&partial_path).ok_or(CargoError::from(format!(
          "{} had a target {} whose crate root appeared to be outside of the crate.",
          &full_name, target.name
        ))));

      let local_path_bytes = target
        .src_path
        .bytes()
        .skip(crate_name_str_idx + partial_path_byte_length)
        .collect::<Vec<_>>();
      // UNWRAP: Sliced from a known unicode string -- conversion is safe
      let mut local_path_str = String::from_utf8(local_path_bytes).unwrap();
      if local_path_str.starts_with("./") {
        local_path_str = local_path_str.split_off(2);
      }

      for kind in target.kind.iter() {
        targets.push(BuildTarget {
          name: target.name.clone(),
          path: local_path_str.clone(),
          kind: kind.clone(),
        });
      }
    }

    Ok(targets)
  }
}

impl MetadataFetcher for CargoSubcommandMetadataFetcher {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata> {
    assert!(files.toml_path.is_file());
    assert!(files.lock_path_opt.as_ref().map(|p| p.is_file()).unwrap_or(true));

    // Copy files into a temp directory
    // UNWRAP: Guarded by function assertion
    let cargo_tempdir = {
      let dir = try!(
        TempDir::new("cargo_raze_metadata_dir").map_err(|_| CargoError::from("creating tempdir"))
      );
      {
        let dir_path = dir.path();
        let new_toml_path = dir_path.join(files.toml_path.file_name().unwrap());
        try!(
          fs::copy(files.toml_path, new_toml_path)
            .map_err(|_| CargoError::from("copying cargo toml"))
        );
        if let Some(lock_path) = files.lock_path_opt {
          let new_lock_path = dir_path.join(lock_path.file_name().unwrap());
          try!(
            fs::copy(lock_path, new_lock_path).map_err(|_| CargoError::from("copying cargo lock"))
          );
        }
      }
      dir
    };

    // Shell out to cargo
    let exec_output = try!(
      Command::new("cargo")
        .current_dir(cargo_tempdir.path())
        .args(&["metadata", "--format-version", "1"])
        .output()
        .map_err(|_| CargoError::from("running `cargo metadata`"))
    );

    // Handle command errs
    let stdout_str =
      String::from_utf8(exec_output.stdout).unwrap_or("[unparsable bytes]".to_owned());
    if !exec_output.status.success() {
      let stderr_str =
        String::from_utf8(exec_output.stderr).unwrap_or("[unparsable bytes]".to_owned());
      println!("`cargo metadata` failed. Inspect Cargo.toml for issues!");
      println!("stdout: {}", stdout_str);
      println!("stderr: {}", stderr_str);
      return Err(CargoError::from("running `cargo metadata`"));
    }

    // Parse and yield metadata
    serde_json::from_str::<Metadata>(&stdout_str)
      .map_err(|_| CargoError::from("parsing `cargo metadata` output"))
  }
}

impl<'config> MetadataFetcher for CargoInternalsMetadataFetcher<'config> {
  fn fetch_metadata(&mut self, files: CargoWorkspaceFiles) -> CargoResult<Metadata> {
    let manifest = env::current_dir().unwrap().join(&files.toml_path);
    let ws = try!(Workspace::new(&manifest, &self.cargo_config));
    let specs = Packages::All.into_package_id_specs(&ws)?;
    let root_name = specs.iter().next().unwrap().name().to_owned();

    let (resolved_packages, cargo_resolve) =
      ops::resolve_ws_precisely(&ws, None, &[], false, false, &specs)?;

    let root_package_id = try!(
      cargo_resolve
        .iter()
        .filter(|dep| dep.name() == root_name)
        .next()
        .ok_or(CargoError::from("root crate should be in cargo resolve"))
    ).to_string();

    let mut packages = Vec::new();
    let mut resolve = Resolve {
      nodes: Vec::new(),
      root: root_package_id,
    };

    for id in cargo_resolve.iter() {
      let dependencies = cargo_resolve.deps(id).map(|p| p.to_string()).collect();
      let features = cargo_resolve.features_sorted(id).iter().map(|s| s.to_string()).collect();
      resolve.nodes.push(ResolveNode {
        id: id.to_string(),
        dependencies: dependencies,
        features: features,
      })
    }

    for package_id in resolved_packages.package_ids() {
      // TODO(acmcarther): Justify this unwrap
      let package = resolved_packages.get(&package_id).unwrap().clone();
      let manifest_metadata = package.manifest().metadata();

      let mut dependencies = Vec::new();
      for dependency in package.dependencies().iter() {
        dependencies.push(Dependency {
          name: dependency.name().to_string(),
          source: dependency.source_id().to_string(),
          req: dependency.version_req().to_string(),
          kind: match dependency.kind() {
            Kind::Normal => None,
            Kind::Development => Some("dev".to_owned()),
            Kind::Build => Some("build".to_owned()),
          },
          optional: dependency.is_optional(),
          use_default_features: dependency.uses_default_features(),
          features: dependency.features().iter().cloned().collect(),
          target: dependency.platform().map(|p| p.to_string()),
        });
      }

      let mut targets = Vec::new();
      for target in package.targets().iter() {
        let crate_types = target.rustc_crate_types().iter().map(|t| t.to_string()).collect();
        targets.push(Target {
          name: target.name().to_owned(),
          kind: util::kind_to_kinds(target.kind()),
          crate_types: crate_types,
          src_path: target.src_path().display().to_string(),
        });
      }

      let mut features = HashMap::new();
      for (feature, features_or_dependencies) in package.summary().features().iter() {
        features.insert(feature.clone(), features_or_dependencies.clone());
      }

      packages.push(Package {
        name: package.name().to_string(),
        version: package.version().to_string(),
        id: package_id.to_string(),
        license: manifest_metadata.license.clone(),
        license_file: manifest_metadata.license_file.clone(),
        description: manifest_metadata.description.clone(),
        source: Some(package_id.source_id().to_string()),
        dependencies: dependencies,
        targets: targets,
        features: features,
        manifest_path: package.manifest_path().display().to_string(),
      });
    }

    let workspace_members = ws.members().map(|pkg| pkg.package_id().to_string()).collect();

    Ok(Metadata {
      packages: packages,
      resolve: resolve,
      workspace_members: workspace_members,
      target_directory: ws.target_dir().display().to_string(),
      version: 0, /* not generated via subcomand */
    })
  }
}

impl<'config> CargoInternalsMetadataFetcher<'config> {
  pub fn new(cargo_config: &'config Config) -> CargoInternalsMetadataFetcher<'config> {
    CargoInternalsMetadataFetcher {
      cargo_config: cargo_config,
    }
  }
}

fn prune_skipped_deps(
  deps: &Vec<BuildDependency>,
  crate_settings: &CrateSettings,
) -> Vec<BuildDependency> {
  deps
    .iter()
    .filter(|d| !crate_settings.skipped_deps.contains(&format!("{}-{}", d.name, d.version)))
    .map(|dep| dep.clone())
    .collect::<Vec<_>>()
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::Write;

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

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_without_lock() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let toml_path = dir.path().join("Cargo.toml");
    let mut toml = File::create(&toml_path).unwrap();
    toml.write_all(basic_toml().as_bytes()).unwrap();
    let files = CargoWorkspaceFiles {
      toml_path: toml_path,
      lock_path_opt: None,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher;

    fetcher.fetch_metadata(files).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_works_with_lock() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let toml_path = {
      let path = dir.path().join("Cargo.toml");
      let mut toml = File::create(&path).unwrap();
      toml.write_all(basic_toml().as_bytes()).unwrap();
      path
    };
    let lock_path = {
      let path = dir.path().join("Cargo.lock");
      let mut lock = File::create(&path).unwrap();
      lock.write_all(basic_lock().as_bytes()).unwrap();
      path
    };
    let files = CargoWorkspaceFiles {
      toml_path: toml_path,
      lock_path_opt: Some(lock_path),
    };

    let mut fetcher = CargoSubcommandMetadataFetcher;

    fetcher.fetch_metadata(files).unwrap();
  }

  #[test]
  fn test_cargo_subcommand_metadata_fetcher_handles_bad_files() {
    let dir = TempDir::new("test_cargo_raze_metadata_dir").unwrap();
    let toml_path = {
      let path = dir.path().join("Cargo.toml");
      let mut toml = File::create(&path).unwrap();
      toml.write_all(b"hello").unwrap();
      path
    };
    let files = CargoWorkspaceFiles {
      toml_path: toml_path,
      lock_path_opt: None,
    };

    let mut fetcher = CargoSubcommandMetadataFetcher;
    assert!(fetcher.fetch_metadata(files).is_err());
  }
}
