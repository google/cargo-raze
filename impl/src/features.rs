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

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::process::Command;

use crate::error::RazeError;
use crate::settings::RazeSettings;
use crate::util::cargo_bin_path;
use anyhow::{Error, Result};
use camino::Utf8PathBuf;
use cargo_metadata::{Package, PackageId, Version};
use itertools::Itertools;
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
      targeted_features: Vec::new(),
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
  get_per_platform_features_with_command(cargo_dir, settings, packages, run_cargo_tree)
}

pub fn get_per_platform_features_with_command(
  cargo_dir: &Path,
  settings: &RazeSettings,
  packages: &[Package],
  command: fn(&Path, &str) -> Result<String>,
) -> Result<BTreeMap<PackageId, Features>> {
  let triples = settings.enabled_targets();

  // Map of PackageIds using the keys that cargo-tree provides
  let mut package_map: HashMap<(String, Version), PackageId> = HashMap::new();
  for package in packages.iter().cloned() {
    package_map
      .entry((package.name, package.version))
      .or_insert(package.id);
  }

  let mut triple_map = BTreeMap::new();
  for triple in triples {
    triple_map.insert(
      triple.clone(),
      // TODO: This part is slow, since it runs cargo-tree per-platform.
      packages_by_platform(
        clean_cargo_tree_output(&command(cargo_dir, triple.as_str()).map_err(|_err| {
          Error::new(RazeError::Generic(
            "Failed to process cargo-tree output.".into(),
          ))
        })?),
        &package_map,
      )
      .map_err(|_err| {
        Error::new(RazeError::Generic(
          "Failed to segment packages by platform.".into(),
        ))
      })?,
    );
  }

  Ok(
    transpose_keys(triple_map)
      .into_iter()
      .map(consolidate_features)
      .collect(),
  )
}

fn clean_cargo_tree_output(cargo_tree_output: &str) -> Vec<String> {
  let mut crates = Vec::new();
  for line in cargo_tree_output.lines().filter(|line| {
    // remove dedupe lines     // remove lines with no features
    !(line.ends_with("(*)") || line.ends_with("||") || line.is_empty())
  }) {
    crates.push(line.to_string());
  }
  crates
}

// Runs `cargo-tree` with a very specific format argument that makes it easier
// to extract per-platform targets.
fn run_cargo_tree(cargo_dir: &Path, triple: &str) -> Result<String> {
  let cargo_bin: Utf8PathBuf = cargo_bin_path();
  let mut cargo_tree = Command::new(&cargo_bin);
  cargo_tree.current_dir(cargo_dir);
  let args = [
    "tree".to_string(),
    "--prefix=none".to_string(),
    "--frozen".to_string(),
    "--workspace".to_string(),
    format!("--target={}", triple),
    "--format={p}|{f}|".to_string(), // The format to print output with
  ];
  cargo_tree.args(args.iter());

  let tree_output = cargo_tree
    .output()
    .map_err(|_err| Error::new(RazeError::Generic("Failed to run cargo-tree.".into())))?;
  if !tree_output.status.success() {
    eprintln!(
      "Running `{:?} {}` in {:?} failed, output follows:",
      cargo_bin,
      args.iter().join(" "),
      cargo_dir
    );
    eprintln!("{:?}", std::str::from_utf8(&tree_output.stderr));
    panic!("cargo-tree ran and returned failure, see stderr for details");
  }

  String::from_utf8(tree_output.stdout).map_err(|_err| {
    Error::new(RazeError::Generic(
      "Failed to convert cargo-tree output to UTF-8.".into(),
    ))
  })
}

fn packages_by_platform(
  crates: Vec<String>,
  packages: &HashMap<(String, Version), PackageId>,
) -> Result<BTreeMap<PackageId, BTreeSet<String>>> {
  let mut package_map: BTreeMap<PackageId, BTreeSet<String>> = BTreeMap::new();
  for c in &crates {
    let (name, version, features) = process_line(c).map_err(|_err| {
      Error::new(RazeError::Generic(
        "Failed to parse cargo-tree line.".into(),
      ))
    })?;
    let id = packages
      .get(&(name, version))
      .ok_or_else(|| Error::new(RazeError::Generic("No PackageId found.".into())))?
      .clone();

    package_map
      .entry(id)
      .and_modify(|e| {
        *e = e.union(&features).cloned().collect();
      })
      .or_insert(features);
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
      let version = Version::parse(&version_str[..version_end]).map_err(|_err| {
        Error::new(RazeError::Generic(
          "Failed to parse cargo-tree version.".into(),
        ))
      })?;
      Ok((name.trim().to_string(), version, feature_set))
    }
    _ => Err(Error::new(RazeError::Generic(
      "Failed to process cargo tree line.".into(),
    ))),
  }
}

fn transpose_keys(
  triples: BTreeMap<String, BTreeMap<PackageId, BTreeSet<String>>>,
) -> UnconsolidatedFeatures {
  let mut package_map: UnconsolidatedFeatures = BTreeMap::new();
  for (triple, packages) in triples {
    for (pkg, features) in packages {
      package_map
        .entry(pkg)
        .and_modify(|e| {
          e.insert(triple.clone(), features.clone());
        })
        .or_insert_with(|| {
          let mut m = BTreeMap::new();
          m.insert(triple.clone(), features);
          m
        });
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
        platform_map
          .entry(feature)
          .and_modify(|e| e.push(platform.clone()))
          .or_insert_with(|| vec![platform.clone()]);
      }
    }
  }

  let mut platforms_to_features: BTreeMap<Vec<String>, Vec<String>> = BTreeMap::new();
  for (feature, platforms) in platform_map {
    let key = platforms.clone();
    platforms_to_features
      .entry(key)
      .and_modify(|e| e.push(feature.clone()))
      .or_insert_with(|| vec![feature]);
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    planning::BuildPlannerImpl,
    planning::{tests::dummy_workspace_crate_metadata, BuildPlanner},
    settings::{tests::*, GenMode},
    testing::*,
  };
  use cargo_metadata::Version;
  use semver::{BuildMetadata, Prerelease};
  use std::collections::{BTreeSet, HashSet};
  use std::fs::read_to_string;

  #[test]
  fn test_process_line() {
    let process_line_tests: Vec<(&str, (&str, (u64,u64,u64), Vec<&str>, Option<BuildMetadata>))> = vec![
          ("lalrpop v0.19.5|default,lexer,pico-args|", ("lalrpop", (0,19,5), vec!["default", "lexer", "pico-args"], None)),
          ("lalrpop-util v0.19.5|default,std|", ("lalrpop-util", (0,19,5), vec!["default", "std"], None)),
          ("hashbrown v0.9.1|raw|", ("hashbrown", (0,9,1), vec!["raw"], None)),
          ("regex-syntax v0.6.23|default,unicode,unicode-age,unicode-bool,unicode-case,unicode-gencat,unicode-perl,unicode-script,unicode-segment|", ("regex-syntax", (0,6,23), vec!["default", "unicode", "unicode-age", "unicode-bool", "unicode-case", "unicode-gencat", "unicode-perl", "unicode-script", "unicode-segment"], None)),
          ("crunchy v0.2.2|default,limit_128|", ("crunchy", (0,2,2), vec!["default", "limit_128"], None)),
          ("isahc v1.2.0|default,encoding_rs,http2,mime,static-curl,text-decoding|", ("isahc", (1,2,0), vec!["default", "encoding_rs", "http2", "mime", "static-curl", "text-decoding"], None)),
          ("curl v0.4.35|default,http2,openssl-probe,openssl-sys,ssl,static-curl|", ("curl", (0,4,35), vec!["default", "http2", "openssl-probe", "openssl-sys", "ssl", "static-curl"], None)),
          ("curl-sys v0.4.41+curl-7.75.0|default,http2,libnghttp2-sys,openssl-sys,ssl,static-curl|", ("curl-sys", (0,4,41), vec!["default", "http2", "libnghttp2-sys", "openssl-sys", "ssl", "static-curl"], Some(BuildMetadata::new("curl-7.75.0").unwrap()))),
        ];

    for line_test in process_line_tests {
      let (input, (name, version_tuple, features, build_metadata)) = line_test;

      let bm = if let Some(b) = build_metadata {
        b
      } else {
        BuildMetadata::EMPTY
      };

      let version = Version {
        major: version_tuple.0,
        minor: version_tuple.1,
        patch: version_tuple.2,
        pre: Prerelease::EMPTY,
        build: bm,
      };
      let mut feature_set = BTreeSet::new();
      for feature in features {
        feature_set.insert(feature.to_string());
      }

      let result = process_line(input).unwrap();
      assert_eq!(name, result.0);
      assert_eq!(version, result.1);
      assert_eq!(feature_set, result.2);
    }
  }

  #[test]
  fn test_clean_cargo() {
    let input = r###"
cargo-raze v0.15.0 (/tmp/.tmpyVIlry)||
anyhow v1.0.40|default,std|
cargo-clone-crate v0.1.6||
anyhow v1.0.40|default,std|
flate2 v1.0.20|default,miniz_oxide,rust_backend|
cfg-if v1.0.0||
crc32fast v1.2.1|default,std|
cfg-if v1.0.0||
cfg-if v1.0.0||
value-bag v1.0.0-alpha.6||
ctor v0.1.20 (proc-macro)||

quote v1.0.9|default,proc-macro|
proc-macro2 v1.0.26|default,proc-macro|
unicode-xid v0.2.1|default|
syn v1.0.68|clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut|
proc-macro2 v1.0.26|default,proc-macro| (*)
"###;

    let result = clean_cargo_tree_output(input);
    assert_eq!(8, result.len());
    assert_eq!(result[0], "anyhow v1.0.40|default,std|");
    assert_eq!(result[1], "anyhow v1.0.40|default,std|");
    assert_eq!(
      result[2],
      "flate2 v1.0.20|default,miniz_oxide,rust_backend|"
    );
    assert_eq!(result[3], "crc32fast v1.2.1|default,std|");
    assert_eq!(result[4], "quote v1.0.9|default,proc-macro|");
    assert_eq!(result[5], "proc-macro2 v1.0.26|default,proc-macro|");
    assert_eq!(result[6], "unicode-xid v0.2.1|default|");
    assert_eq!(result[7], "syn v1.0.68|clone-impls,default,derive,extra-traits,full,parsing,printing,proc-macro,quote,visit,visit-mut|");
  }

  fn mock_cargo_tree_command(_cargo_dir: &Path, triple: &str) -> Result<String> {
    let cargo_tree_template_dir = Utf8PathBuf::from(std::file!())
      .parent()
      .unwrap()
      .join("testing/cargo_tree")
      .canonicalize()
      .unwrap();

    let cargo_tree_content = cargo_tree_template_dir.join(format!("{}{}", triple, ".txt"));
    Ok(read_to_string(cargo_tree_content).unwrap())
  }

  #[test]
  fn test_per_platform_plan() {
    let platforms = vec![
      "aarch64-apple-darwin",
      "aarch64-unknown-linux-gnu",
      "x86_64-apple-darwin",
      "x86_64-pc-windows-msvc",
      "x86_64-unknown-linux-gnu",
      "wasm32-unknown-unknown",
    ];

    let temp_dir = make_basic_workspace();
    let mut settings = dummy_raze_settings();
    settings.genmode = GenMode::Vendored;
    settings.target = None;
    let mut targets: HashSet<String> = HashSet::new();
    for target in platforms {
      targets.insert(target.to_string());
    }
    settings.targets = Some(targets);

    let mut metadata = dummy_workspace_crate_metadata(templates::CARGO_TREE);
    metadata.features = get_per_platform_features_with_command(
      temp_dir.path(),
      &settings,
      &metadata.metadata.packages,
      mock_cargo_tree_command,
    )
    .unwrap();

    let planner = BuildPlannerImpl::new(metadata, settings);

    let planned_build = planner.plan_build(None).unwrap();
    let tokio = planned_build
      .crate_contexts
      .iter()
      .find(|ctx| ctx.pkg_name == "tokio")
      .unwrap();

    let targeted_features = &tokio.features.targeted_features;
    assert_eq!(targeted_features.len(), 5);
    assert_eq!(
      targeted_features[0].platforms,
      vec!["aarch64-apple-darwin", "aarch64-unknown-linux-gnu"]
    );
    assert_eq!(targeted_features[0].features, vec!["time"]);
    assert_eq!(
      targeted_features[1].platforms,
      vec![
        "aarch64-apple-darwin",
        "aarch64-unknown-linux-gnu",
        "wasm32-unknown-unknown",
        "x86_64-apple-darwin",
      ]
    );
    assert_eq!(
      targeted_features[1].features,
      vec!["libc", "mio", "net", "socket2",]
    );
    assert_eq!(
      targeted_features[2].platforms,
      vec![
        "aarch64-apple-darwin",
        "aarch64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "x86_64-unknown-linux-gnu",
      ]
    );
    assert_eq!(targeted_features[2].features, vec!["fs",]);
    assert_eq!(
      targeted_features[3].platforms,
      vec!["aarch64-apple-darwin", "x86_64-apple-darwin",]
    );
    assert_eq!(targeted_features[3].features, vec!["stats",]);
    assert_eq!(
      targeted_features[4].platforms,
      vec!["x86_64-pc-windows-msvc"]
    );
    assert_eq!(targeted_features[4].features, vec!["winapi",]);
  }
}
