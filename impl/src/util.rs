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

use anyhow::Result;
use std::{collections::HashSet, env, fmt, iter::Iterator, process::Command, str::FromStr};

use camino::{Utf8Path, Utf8PathBuf};
use cargo_platform::Cfg;

use cfg_expr::{targets::get_builtin_target_by_triple, Expression, Predicate};
use pathdiff::diff_paths;

pub(crate) const SYSTEM_CARGO_BIN_PATH: &str = "cargo";
pub(crate) const RAZE_LOCKFILE_NAME: &str = "Cargo.raze.lock";

static SUPPORTED_PLATFORM_TRIPLES: &[&str] = &[
  // SUPPORTED_T1_PLATFORM_TRIPLES
  "i686-apple-darwin",
  "i686-pc-windows-msvc",
  "i686-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "x86_64-pc-windows-msvc",
  "x86_64-unknown-linux-gnu",
  // SUPPORTED_T2_PLATFORM_TRIPLES
  "aarch64-apple-darwin",
  "aarch64-apple-ios",
  "aarch64-apple-ios-sim",
  "aarch64-linux-android",
  "aarch64-unknown-linux-gnu",
  "arm-unknown-linux-gnueabi",
  "armv7-linux-androideabi",
  "i686-linux-android",
  "i686-unknown-freebsd",
  "powerpc-unknown-linux-gnu",
  "s390x-unknown-linux-gnu",
  "wasm32-unknown-unknown",
  "wasm32-wasi",
  "x86_64-apple-ios",
  "x86_64-linux-android",
  "x86_64-unknown-freebsd",
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BazelTargetSupport {
  /// The target specifically matches a target expression
  ///
  /// # Examples:
  /// * `cfg(unix)` There are supported platforms from the `unix` `target_family` but not all platforms are of the `unix` family.
  /// * `cfg(not(windows))` There are supported platforms in addition to those in the `windows` `target_family`.
  /// * `x86_64-apple-darwin` This is a supported target triple but obviously won't match with other triples.
  SpecificTargetMatches,

  /// The target matches broadly
  ///
  /// # Examples:
  /// * `cfg(not(fuchsia))` `fuchsia` would be considered a 'default' dependency since no supported target maps to it.
  AllTargetsMatch,

  /// This target cannot be supported, not as a broad or specifc match
  ///
  /// # Examples:
  /// * `unknown-unknown-unknown` This will not match any triple.
  /// * `cfg(foo)` `foo` is not a strongly defined cfg value.
  /// * `cfg(target_os = "redox")` `redox` is not a supported platform.
  Unsupported,
}

/// Determines if the target matches those supported by and defined in rules_rust
pub fn is_bazel_supported_platform(target: &str) -> BazelTargetSupport {
  // Ensure the target is represented as an expression
  let target_exp = match target.starts_with("cfg(") {
    true => target.to_owned(),
    false => format!("cfg(target = \"{}\")", target),
  };

  let expression = match Expression::parse(&target_exp) {
    Ok(exp) => exp,
    // If the target expression cannot be parsed it is not considered a Bazel platform
    Err(_) => return BazelTargetSupport::Unsupported,
  };

  let mut specific_match = false;
  let mut matches_all = true;

  // Attempt to match the expression
  for target_info in SUPPORTED_PLATFORM_TRIPLES
    .iter()
    .map(|x| get_builtin_target_by_triple(x).unwrap())
  {
    let target_matches = expression.eval(|pred| {
      match pred {
        Predicate::Target(tp) => tp.matches(target_info),
        Predicate::KeyValue { key, val } => {
          (*key == "target") && (*val == target_info.triple.as_str())
        }
        // For now there is no other kind of matching
        _ => false,
      }
    });

    if target_matches {
      specific_match = true;
    } else {
      matches_all = false;
    }
  }

  match (specific_match, matches_all) {
    (true, true) => BazelTargetSupport::AllTargetsMatch,
    (true, false) => BazelTargetSupport::SpecificTargetMatches,
    _ => BazelTargetSupport::Unsupported,
  }
}

/// Maps a Rust cfg or triple target to Bazel supported triples.
///
/// Note, the Bazel triples must be defined in:
/// https://github.com/bazelbuild/rules_rust/blob/master/rust/platform/platform.bzl
pub fn get_matching_bazel_triples<'a>(
  target: &str,
  allowlist: &'a Option<HashSet<String>>,
) -> Result<impl Iterator<Item = &'static str> + 'a> {
  let expression = match target.starts_with("cfg(") {
    true => Expression::parse(target),
    false => Expression::parse(&format!("cfg(target = \"{}\")", target)),
  }?;

  let triples = SUPPORTED_PLATFORM_TRIPLES
    .iter()
    .filter_map(move |triple| {
      let target_info = get_builtin_target_by_triple(triple).unwrap();
      let triple = target_info.triple.as_str();
      let res = expression.eval(|pred| match pred {
        Predicate::Target(tp) => tp.matches(target_info),
        Predicate::KeyValue { key, val } => *key == "target" && *val == triple,
        // For now there is no other kind of matching
        _ => false,
      });

      match res {
        true => Some(triple),
        false => None,
      }
    })
    .filter(move |x| {
      allowlist
        .as_ref()
        .map(|targets| targets.contains(*x))
        .unwrap_or(true)
    });

  Ok(triples)
}

/// Returns all of the supported targets, filtered by an allowlist.
pub fn get_enabled_targets<'a>(
  allowlist: &'a Option<HashSet<String>>,
) -> impl Iterator<Item = &'static str> + 'a {
  SUPPORTED_PLATFORM_TRIPLES
    .iter()
    .filter(move |x| {
      allowlist
        .as_ref()
        .map(|targets| targets.contains(**x))
        .unwrap_or(true)
    })
    .map(AsRef::as_ref)
}

/// Returns whether or not the given path is a Bazel workspace root
pub fn is_bazel_workspace_root(dir: &Utf8Path) -> bool {
  let workspace_files = [dir.join("WORKSPACE.bazel"), dir.join("WORKSPACE")];
  workspace_files.iter().any(|x| x.exists())
}

/// Returns a path to a Bazel workspace root based on the current working
/// directory, otherwise None if not workspace is detected.
pub fn find_bazel_workspace_root(manifest_path: &Utf8Path) -> Option<Utf8PathBuf> {
  let mut dir = if manifest_path.is_dir() {
    Some(manifest_path)
  } else {
    manifest_path.parent()
  };

  while let Some(current_dir) = dir {
    if is_bazel_workspace_root(current_dir) {
      return Some(Utf8PathBuf::from(current_dir));
    }

    dir = current_dir.parent();
  }

  None
}

pub struct PlatformDetails {
  target_triple: String,
  attrs: Vec<Cfg>,
}

pub struct LimitedResults<T> {
  pub items: Vec<T>,
  pub count_extras: usize,
}

impl PlatformDetails {
  pub fn new_using_rustc(target_triple: &str) -> Result<Self> {
    let attrs = fetch_attrs(target_triple)?;
    Ok(Self::new(target_triple.to_owned(), attrs))
  }

  pub fn new(target_triple: String, attrs: Vec<Cfg>) -> Self {
    Self {
      target_triple,
      attrs,
    }
  }

  #[allow(dead_code)]
  pub fn target_triple(&self) -> &str {
    &self.target_triple
  }

  pub fn attrs(&self) -> &Vec<Cfg> {
    &self.attrs
  }
}

impl<T> LimitedResults<T> {
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl<T: fmt::Debug> fmt::Debug for LimitedResults<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.count_extras > 0 {
      write!(f, "{:?} and {} others", &self.items, self.count_extras)
    } else {
      write!(f, "{:?}", &self.items)
    }
  }
}

pub fn collect_up_to<T, U: Iterator<Item = T>>(max: usize, iter: U) -> LimitedResults<T> {
  let mut items = Vec::new();
  let mut count_extras = 0;
  for item in iter {
    // Spill extra crates into a counter to avoid overflowing terminal
    if items.len() < max {
      items.push(item);
    } else {
      count_extras += 1;
    }
  }

  LimitedResults {
    items,
    count_extras,
  }
}

pub fn sanitize_ident(ident: &str) -> String {
  slug::slugify(&ident).replace('-', "_")
}

/// Gets the proper system attributes for the provided platform triple using rustc.
fn fetch_attrs(target: &str) -> Result<Vec<Cfg>> {
  let args = vec![format!("--target={}", target), "--print=cfg".to_owned()];

  let output = Command::new("rustc").args(&args).output()?;

  if !output.status.success() {
    panic!(
      "getting target attrs for {} failed with status: '{}' \nstdout: {}\nstderr: {}",
      target,
      output.status,
      String::from_utf8(output.stdout).unwrap_or_else(|_| "[unparseable bytes]".to_owned()),
      String::from_utf8(output.stderr).unwrap_or_else(|_| "[unparseable bytes]".to_owned())
    )
  }

  let attr_str =
    String::from_utf8(output.stdout).expect("successful run of rustc's output to be utf8");

  Ok(
    attr_str
      .lines()
      .map(Cfg::from_str)
      .map(|cfg| cfg.expect("attrs from rustc should be parsable into Cargo Cfg"))
      .collect(),
  )
}

pub fn get_workspace_member_path(
  manifest_path: &Utf8Path,
  workspace_root: &Utf8Path,
) -> Option<Utf8PathBuf> {
  assert!(manifest_path.ends_with("Cargo.toml"));
  // UNWRAP: A manifest path should always be a path to a 'Cargo.toml' file which should always have a parent directory
  Utf8PathBuf::from_path_buf(diff_paths(manifest_path.parent().unwrap(), workspace_root)?).ok()
}

pub fn package_ident(package_name: &str, package_version: &str) -> String {
  format!("{}-{}", package_name, package_version)
}

/// Locates a lockfile for the associated crate. A `Cargo.raze.lock` file in the
/// [RazeSettings::workspace_path](crate::settings::RazeSettings::workspace_path)
/// directory will take precidence over a standard `Cargo.lock` file.
pub fn find_lockfile(
  cargo_workspace_root: &Utf8Path,
  raze_output_dir: &Utf8Path,
) -> Option<Utf8PathBuf> {
  // The custom raze lockfile will always take precidence
  let raze_lockfile = raze_output_dir.join(RAZE_LOCKFILE_NAME);
  if raze_lockfile.exists() {
    return Some(raze_lockfile);
  }

  // If there is an existing standard lockfile, use it.
  let cargo_lockfile = cargo_workspace_root.join("Cargo.lock");
  if cargo_lockfile.exists() {
    return Some(cargo_lockfile);
  }

  // No lockfile is available.
  None
}

/// Locates a cargo binary form either an evironment variable or PATH
pub fn cargo_bin_path() -> Utf8PathBuf {
  Utf8PathBuf::from(env::var("CARGO").unwrap_or_else(|_| SYSTEM_CARGO_BIN_PATH.to_string()))
}

#[cfg(test)]
mod tests {
  use std::fs::File;

  use tempfile::TempDir;

  use super::*;

  #[test]
  fn test_collect_up_to_works_for_zero() {
    let test_items: Vec<u32> = Vec::new();
    let results = collect_up_to(10, test_items.iter());
    assert!(results.is_empty());
  }

  #[test]
  fn test_collect_up_to_works_for_one() {
    let test_items = vec![1];
    let results = collect_up_to(10, test_items.iter());
    assert_eq!(results.items, vec![&1]);
    assert!(!results.is_empty());
  }

  #[test]
  fn test_collect_up_to_works_for_others_and_bounds_correctly() {
    let test_items = vec![1, 2, 3];
    let results = collect_up_to(2, test_items.iter());
    assert_eq!(results.items, vec![&1, &2]);
    assert_eq!(results.count_extras, 1);
    assert!(!results.is_empty());
  }

  #[test]
  fn detecting_workspace_root() {
    let bazel_root = TempDir::new().unwrap();
    let manifest = Utf8PathBuf::from_path_buf(bazel_root.as_ref().join("Cargo.toml")).unwrap();

    // Starting within the temp directory, we'll find that there are no WORKSPACE.bazel files
    // and thus return None to indicate a Bazel workspace root could not be found.
    assert_eq!(find_bazel_workspace_root(&manifest), None);

    // After creating a WORKSPACE.bazel file in that directory, we expect to find to be
    // returned a path to the temporary directory
    File::create(bazel_root.path().join("WORKSPACE.bazel")).unwrap();
    assert_eq!(
      find_bazel_workspace_root(&manifest)
        .unwrap()
        .canonicalize()
        .unwrap(),
      bazel_root.into_path().canonicalize().unwrap()
    );
  }

  #[test]
  fn detect_bazel_platforms() {
    assert_eq!(
      is_bazel_supported_platform("cfg(not(fuchsia))"),
      BazelTargetSupport::AllTargetsMatch
    );
    assert_eq!(
      is_bazel_supported_platform("cfg(not(target_os = \"redox\"))"),
      BazelTargetSupport::AllTargetsMatch
    );
    assert_eq!(
      is_bazel_supported_platform("cfg(unix)"),
      BazelTargetSupport::SpecificTargetMatches
    );
    assert_eq!(
      is_bazel_supported_platform("cfg(not(windows))"),
      BazelTargetSupport::SpecificTargetMatches
    );
    assert_eq!(
      is_bazel_supported_platform("cfg(target = \"x86_64-apple-darwin\")"),
      BazelTargetSupport::SpecificTargetMatches
    );
    assert_eq!(
      is_bazel_supported_platform("x86_64-apple-darwin"),
      BazelTargetSupport::SpecificTargetMatches
    );
    assert_eq!(
      is_bazel_supported_platform("unknown-unknown-unknown"),
      BazelTargetSupport::Unsupported
    );
    assert_eq!(
      is_bazel_supported_platform("cfg(foo)"),
      BazelTargetSupport::Unsupported
    );
    assert_eq!(
      is_bazel_supported_platform("cfg(target_os = \"redox\")"),
      BazelTargetSupport::Unsupported
    );
  }

  #[test]
  fn all_supported_platform_triples_unwrap() {
    for triple in SUPPORTED_PLATFORM_TRIPLES.iter() {
      get_builtin_target_by_triple(triple).unwrap();
    }
  }
}
