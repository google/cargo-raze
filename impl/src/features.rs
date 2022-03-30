// Copyright 2022 AgileBits Inc.
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

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::RazeError;
use crate::settings::RazeSettings;
use crate::util::cargo_bin_path;
use anyhow::{Error, Result};
use cargo_metadata::{Package, PackageId, Version};
use serde::{Deserialize, Serialize};

type UnconsolidatedFeatures = BTreeMap<PackageId, BTreeMap<String, BTreeSet<String>>>;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Features {
  pub features: Vec<String>,
  pub targeted_features: Vec<TargetedFeatures>,
}

impl Features {
  pub fn empty() -> Features {
    Features {
      features: Vec::new(),
      targeted_features: vec![],
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TargetedFeatures {
  pub platforms: Vec<String>,
  pub features: Vec<String>,
}

// A function that runs `cargo-tree` to analyze per-platform features.
// This step should not need to be separate from cargo-metadata, but cargo-metadata's
// output is incomplete in this respect.
//
// See: https://github.com/rust-lang/cargo/issues/9863
// and: https://github.com/illicitonion/cargo-metadata-pathologies/tree/main/platform-specific-features
//
// There is no plan for Cargo to expose platform data in `cargo-metadata`:
// See https://github.com/rust-lang/cargo/pull/9982#issuecomment-965423533
//
// The Rust documentation says `cargo-tree` may stabilize its `--unit-graph` output, which
// would make this function faster by avoiding mulitple invocations of `cargo-tree`.
// https://rust-lang.github.io/rfcs/2957-cargo-features2.html#cargo-metadata
//
pub fn get_per_platform_features(
  cargo_dir: &Path,
  settings: &RazeSettings,
  packages: &[Package],
) -> Result<BTreeMap<PackageId, Features>> {
  let mut triples: BTreeSet<String> = BTreeSet::new();
  if let Some(target) = settings.target.clone() {
    triples.insert(target);
  }
  if let Some(targets) = settings.targets.clone() {
    triples.extend(targets);
  }

  let mut triple_map = BTreeMap::new();
  for triple in triples {
    triple_map.insert(
      triple.clone(),
      // TODO: This part is slow, since it runs cargo-tree per-platform.
      make_package_map(run_cargo_tree(cargo_dir, triple.as_str())?, packages)?,
    );
  }

  let features: BTreeMap<PackageId, Features> = transpose_keys(triple_map)
    .into_iter()
    .map(consolidate_features)
    .collect();
  Ok(features)
}

// Runs `cargo-tree` with a very specific format argument that makes it easier
// to extract per-platform targets.
fn run_cargo_tree(cargo_dir: &Path, triple: &str) -> Result<Vec<String>> {
  let cargo_bin: PathBuf = cargo_bin_path();
  let mut cargo_tree = Command::new(cargo_bin);
  cargo_tree.current_dir(cargo_dir);
  cargo_tree
    .arg("tree")
    .arg("--prefix=none")
    .arg("--frozen")
    .arg(format!("--target={}", triple))
    .arg("--format={p}|{f}|"); // The format to print output with

  let tree_output = cargo_tree.output()?;
  assert!(tree_output.status.success());

  let text = String::from_utf8(tree_output.stdout)?;
  let mut crates: BTreeSet<String> = BTreeSet::new();
  for line in text.lines().filter(|line| {
    // remove dedupe lines     // remove lines with no features
    !(line.ends_with("(*)") || line.ends_with("||") || line.is_empty())
  }) {
    crates.insert(line.to_string());
  }
  Ok(crates.iter().map(|s| s.to_string()).collect())
}

fn make_package_map(
  crates: Vec<String>,
  packages: &[Package],
) -> Result<BTreeMap<PackageId, BTreeSet<String>>> {
  let mut package_map: BTreeMap<PackageId, BTreeSet<String>> = BTreeMap::new();
  for c in &crates {
    let (name, version, features) = process_line(c)?;
    let id = find_package_id(name, version, packages)?;

    // TODO: this should not be necessary
    match package_map.get_mut(&id) {
      Some(existing_features) => {
        let f = existing_features.union(&features).cloned().collect();
        package_map.insert(id, f);
      }
      None => {
        package_map.insert(id, features);
      }
    }
  }
  Ok(package_map)
}

// Process the output of cargo-tree to discover features that are only targeted
// per-platform. The input format is specified by the format string in `run_cargo_tree`.
//
// This function does some basic text processing, and ignores
// bogus and/or repetitive lines that cargo-tree inserts.
fn process_line(s: &str) -> Result<(String, Version, BTreeSet<String>)> {
  match (s.find(' '), s.find('|')) {
    (Some(space), Some(pipe)) => {
      let (package, features) = s.split_at(pipe);
      let features_trimmed = features.replace('|', "");
      let feature_set: BTreeSet<String> = features_trimmed
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
      let (name, mut version_str) = package.split_at(space);
      version_str = version_str.trim_start_matches(|c| c == ' ' || c == 'v');
      let version_end = match version_str.find(' ') {
        Some(index) => index,
        None => version_str.chars().count(),
      };
      let version = Version::parse(&version_str[..version_end])?;
      Ok((name.trim().to_string(), version, feature_set))
    }
    _ => Err(Error::new(RazeError::Generic(
      "Failed to process cargo tree line.".into(),
    ))),
  }
}

fn find_package_id(name: String, version: Version, packages: &[Package]) -> Result<PackageId> {
  packages
    .iter()
    .find(|package| package.name == name && package.version == version)
    .map(|package| package.id.clone())
    .ok_or_else(|| Error::new(RazeError::Generic("Failed to find package.".into())))
}

fn transpose_keys(
  triples: BTreeMap<String, BTreeMap<PackageId, BTreeSet<String>>>,
) -> UnconsolidatedFeatures {
  let mut package_map: UnconsolidatedFeatures = BTreeMap::new();
  for (triple, packages) in triples {
    for (pkg, features) in packages {
      match package_map.get_mut(&pkg) {
        Some(triple_map) => {
          triple_map.insert(triple.clone(), features);
        }
        None => {
          let mut m = BTreeMap::new();
          m.insert(triple.clone(), features);
          package_map.insert(pkg.clone(), m);
        }
      }
    }
  }
  package_map
}

fn consolidate_features(
  pkg: (PackageId, BTreeMap<String, BTreeSet<String>>),
) -> (PackageId, Features) {
  let (id, features) = pkg;

  // Find the features common to all targets
  let sets: Vec<&BTreeSet<String>> = features.values().collect();
  let common_features = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
    acc.intersection(hs).cloned().collect()
  });

  // Partition the platform features
  let mut platform_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
  for (platform, pfs) in features {
    for feature in pfs {
      if !common_features.contains(&feature) {
        match platform_map.get_mut(&feature) {
          Some(platforms) => {
            platforms.push(platform.clone());
          }
          None => {
            platform_map.insert(feature, vec![platform.clone()]);
          }
        }
      }
    }
  }

  let mut platforms_to_features: BTreeMap<Vec<String>, Vec<String>> = BTreeMap::new();
  for (feature, platforms) in platform_map {
    let key = platforms.clone();
    match platforms_to_features.get_mut(&key) {
      Some(features) => {
        features.push(feature);
      }
      None => {
        platforms_to_features.insert(key, vec![feature]);
      }
    }
  }

  let mut common_vec: Vec<String> = common_features.iter().cloned().collect();
  common_vec.sort();

  let targeted_features: Vec<TargetedFeatures> = platforms_to_features
    .iter()
    .map(|ptf| {
      let (platforms, features) = ptf;
      TargetedFeatures {
        platforms: platforms.to_vec(),
        features: features.to_vec(),
      }
    })
    .collect();

  (
    id,
    Features {
      features: common_vec,
      targeted_features,
    },
  )
}
