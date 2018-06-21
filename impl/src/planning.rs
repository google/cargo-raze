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
use cargo::core::dependency::Platform;
use cargo::core::SourceId;
use cargo::util::CargoResult;
use context::BuildDependency;
use context::BuildTarget;
use context::CrateContext;
use context::LicenseData;
use context::GitRepo;
use context::WorkspaceContext;
use license;
use metadata::CargoWorkspaceFiles;
use metadata::Metadata;
use metadata::MetadataFetcher;
use metadata::Package;
use metadata::PackageId;
use metadata::ResolveNode;
use settings::CrateSettings;
use settings::GenMode;
use settings::RazeSettings;
use slug;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::str;
use std::str::FromStr;
use util;

/** An entity that can produce an organized, planned build ready to be rendered. */
pub trait BuildPlanner {
  fn plan_build(
    &mut self,
    settings: &RazeSettings,
    files: CargoWorkspaceFiles,
  ) -> CargoResult<PlannedBuild>;
}

/** The default implementation of a BuildPlanner. */
pub struct BuildPlannerImpl<'fetcher> {
  metadata_fetcher: &'fetcher mut MetadataFetcher,
}

/** A ready-to-be-rendered build, containing renderable context for each crate. */
#[derive(Debug)]
pub struct PlannedBuild {
  pub workspace_context: WorkspaceContext,
  pub crate_contexts: Vec<CrateContext>,
}

impl<'fetcher> BuildPlanner for BuildPlannerImpl<'fetcher> {
  fn plan_build(
    &mut self,
    settings: &RazeSettings,
    files: CargoWorkspaceFiles,
  ) -> CargoResult<PlannedBuild> {
    let metadata = try!(self.metadata_fetcher.fetch_metadata(files));
    if settings.genmode == GenMode::Vendored {
      try!(self.check_crates_vendored(&metadata));
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

  fn check_crates_vendored(&self, metadata: &Metadata) -> CargoResult<()> {
    for package in metadata.packages.iter() {
      // Don't expect the root crate to be vendored
      if package.id == metadata.resolve.root {
        continue;
      }

      let full_name = format!("{}-{}", package.name, package.version);
      let path = format!("./vendor/{}/", full_name);

      if fs::metadata(&path).is_err() {
        return Err(CargoError::from(format!(
          "failed to find {}. Either switch to \"Remote\" genmode, or run `cargo vendor -x` first.",
          &path
        )));
      };
    }

    Ok(())
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
      eprintln!("Resolve was: {:#?}", metadata.resolve);
      eprintln!("root_id: {:?}", metadata.resolve.root);
      return Err(CargoError::from("Resolve did not contain root crate!"));
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
    let (package_id_to_build_path, package_id_to_default_build_target) = {
      let mut package_id_to_build_path = HashMap::new();
      let mut package_id_to_default_build_target = HashMap::new();

      for package in metadata.packages.iter() {
        let sanitized_name = slug::slugify(&package.name).replace("-", "_");
        let build_path = match settings.genmode {
          GenMode::Remote => {
            let sanitized_version = slug::slugify(&package.version).replace("-", "_");
            format!(
              "@{}__{}__{}//",
              settings.gen_workspace_prefix, sanitized_name, sanitized_version
            )
          },
          GenMode::Vendored => {
            format!("{}/vendor/{}-{}", settings.workspace_path, package.name, package.version)
          },
        };
        package_id_to_default_build_target
          .insert(package.id.clone(), format!("{}:{}", build_path, sanitized_name));
        package_id_to_build_path.insert(package.id.clone(), build_path);
      }

      (package_id_to_build_path, package_id_to_default_build_target)
    };

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

    // Verify that user settings are being used
    {
      let mut name_to_versions = HashMap::new();
      for package in metadata.packages.iter() {
        name_to_versions
          .entry(package.name.clone())
          .or_insert(HashSet::new())
          .insert(package.version.clone());
      }
      let empty_set = HashSet::new();
      for (name, versions) in settings.crates.iter() {
        for v in versions.keys() {
          let alternatives = name_to_versions.get(name).unwrap_or(&empty_set);
          if !alternatives.contains(v) {
            let help = if alternatives.is_empty() {
              "no alternatives found.".to_owned()
            } else {
              format!("did you mean one of {}-{:?}?", name, alternatives)
            };
            eprintln!("Found unused raze settings for {}-{}, {}", name, v, help);
          }
        }
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

      // UNWRAP: Safe given unwrap during serialize step of metadata
      let own_source_id = own_package.source.as_ref()
        .map(|s| serde_json::from_str::<SourceId>(&s).unwrap());

      // Resolve dependencies into types
      let mut build_dep_names = Vec::new();
      let mut dev_dep_names = Vec::new();
      let mut normal_dep_names = Vec::new();
      for dep in own_package.dependencies.iter() {
        if dep.target.is_some() {
          // UNWRAP: Safe from above check
          let target_str = dep.target.as_ref().unwrap();
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
        // UNWRAP(s): Safe from verification of packages_by_id
        let dep_package = packages_by_id.get(dep_id.as_str()).unwrap();
        let build_target = package_id_to_default_build_target.get(dep_id).unwrap();

        let build_dependency = BuildDependency {
          name: dep_package.name.clone(),
          version: dep_package.version.clone(),
          build_target: build_target.clone(),
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

      let is_git = own_source_id.as_ref().map_or(false, SourceId::is_git);
      let git_data = if is_git {
        // UNWRAP: is_git true implies own_source_id exists
        let s = own_source_id.as_ref().unwrap();
        Some(GitRepo {
          remote: s.url().to_string(),
          commit: s.precise().unwrap().to_owned(),
        })
      } else {
        None
      };

      let mut targets = try!(self.produce_targets(&own_package, &own_source_id, settings));
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
        features: node.features.clone().unwrap_or(Vec::new()),
        is_root_dependency: root_direct_deps.contains(&node.id),
        metadeps: Vec::new(), /* TODO(acmcarther) */
        dependencies: non_skipped_normal_deps,
        build_dependencies: non_skipped_build_deps,
        dev_dependencies: dev_deps,
        path: path,
        // UNWRAP: Safe -- struct derived from package set
        build_path: package_id_to_build_path.get(&own_package.id).unwrap().clone(),
        build_script_target: build_script_target,
        targets: targets_sans_build_script,
        platform_triple: settings.target.to_owned(),
        additional_deps: additional_deps,
        additional_flags: additional_flags,
        extra_aliased_targets: extra_aliased_targets,
        data_attr: data_attr,
        git_data: git_data,
        sha256: own_package.sha256.clone(),
      })
    }

    Ok(crate_contexts)
  }


  fn produce_targets(&self, package: &Package, source_id: &Option<SourceId>, settings: &RazeSettings) -> CargoResult<Vec<BuildTarget>> {
    let mut targets = Vec::new();
    for target in package.targets.iter() {
      let manifest_pathbuf = PathBuf::from(&package.manifest_path);
      assert!(manifest_pathbuf.is_absolute());

      let is_git = source_id.as_ref().map_or(false, SourceId::is_git);
      let local_root = match (settings.genmode.clone(), is_git) {
        // UNWRAP: We know from source_id that this is a git package, so it must have a repo root
        (GenMode::Remote, true) => try!(package_git_root(&manifest_pathbuf)).to_str().unwrap().to_owned(),
        _ => manifest_pathbuf.parent().unwrap().display().to_string(),
      };

      // Trim the manifest_path parent dir from the target path (to give us the crate-local path)
      let mut local_path_str = target
        .src_path
        .clone()
        .split_off(local_root.len() + 1);

      // Some crates have a weird prefix, trim that.
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

fn package_git_root(manifest: &PathBuf) -> CargoResult<PathBuf> {
  let cloned = manifest.clone();
  let mut check_path = cloned.as_path();
  while let Some(c) = check_path.parent() {
    let joined = c.join(".git");
    if joined.is_dir() {
      return Ok(c.to_path_buf());
    } else {
      check_path = c;
    }
  }

  // Reached filesystem root and did not find Git repo
  Err(CargoError::from(format!("Unable to locate git repository root for manifest {:?}", manifest)))
}

#[cfg(test)]
mod tests {
  use super::*;
  use metadata::Metadata;
  use metadata::ResolveNode;
  use metadata::testing as metadata_testing;
  use metadata::testing::StubMetadataFetcher;
  use settings::testing as settings_testing;

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
    metadata.resolve.nodes[ROOT_NODE_IDX].dependencies.push("test_dep_id".to_owned());
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
  fn test_plan_build_missing_resolve_fails() {
    let mut fetcher = StubMetadataFetcher::with_metadata(metadata_testing::dummy_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res =
      planner.plan_build(&settings_testing::dummy_raze_settings(), dummy_workspace_files());

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.is_err());
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
    let planned_build_res =
      planner.plan_build(&settings_testing::dummy_raze_settings(), dummy_workspace_files());

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.is_err());
  }

  #[test]
  fn test_plan_build_minimum_workspace() {
    let mut fetcher = StubMetadataFetcher::with_metadata(minimum_valid_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res =
      planner.plan_build(&settings_testing::dummy_raze_settings(), dummy_workspace_files());

    println!("{:#?}", planned_build_res);
    assert!(planned_build_res.unwrap().crate_contexts.is_empty());
  }

  #[test]
  fn test_plan_build_minimum_root_dependency() {
    let mut fetcher = StubMetadataFetcher::with_metadata(minimum_dependency_metadata());
    let mut planner = BuildPlannerImpl::new(&mut fetcher);
    let planned_build_res =
      planner.plan_build(&settings_testing::dummy_raze_settings(), dummy_workspace_files());

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
    let planned_build_res = planner.plan_build(&settings, dummy_workspace_files());

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
