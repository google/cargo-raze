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
use cargo::core::TargetKind;
use cargo::util::CargoResult;
use cargo::util::Cfg;
use std::process::Command;
use std::collections::HashSet;
use std::str;
use std::str::FromStr;
use std::hash::Hash;

pub fn unique_targets_ordered(opt_item: &Option<String>, items: &Vec<String>) -> Vec<String> {
  let mut items_set = HashSet::new();
  for item in items.iter() {
    items_set.insert(item.clone());
  }
  if opt_item.is_some() {
    items_set.insert(opt_item.as_ref().unwrap().clone());
  }
  let mut items = items_set.into_iter().collect::<Vec<_>>();
  items.sort();
  items
}

/**
 * Extracts consistently named Strings for the provided TargetKind.
 *
 * TODO(acmcarther): Remove this shim borrowed from Cargo when Cargo is upgraded
 */
pub fn kind_to_kinds(kind: &TargetKind) -> Vec<String> {
  match kind {
    &TargetKind::Lib(ref kinds) => kinds.iter().map(|k| k.crate_type().to_owned()).collect(),
    &TargetKind::Bin => vec!["bin".to_owned()],
    &TargetKind::ExampleBin | &TargetKind::ExampleLib(_) => vec!["example".to_owned()],
    &TargetKind::Test => vec!["test".to_owned()],
    &TargetKind::CustomBuild => vec!["custom-build".to_owned()],
    &TargetKind::Bench => vec!["bench".to_owned()],
  }
}

/** Gets the proper system attributes for the provided platform triple using rustc. */
pub fn fetch_attrs(target: &str) -> CargoResult<Vec<Cfg>> {
  let args = vec![format!("--target={}", target), "--print=cfg".to_owned()];

  let output = try!(Command::new("rustc").args(&args).output().map_err(|_| {
    CargoError::from(format!(
      "could not run rustc to fetch attrs for target {}",
      target
    ))
  }));

  if !output.status.success() {
    panic!(format!(
      "getting target attrs for {} failed with status: '{}' \nstdout: {}\nstderr: {}",
      target,
      output.status,
      String::from_utf8(output.stdout).unwrap_or("[unparseable bytes]".to_owned()),
      String::from_utf8(output.stderr).unwrap_or("[unparseable bytes]".to_owned())
    ))
  }

  let attr_str =
    String::from_utf8(output.stdout).expect("successful run of rustc's output to be utf8");

  Ok(
    attr_str
      .lines()
      .map(Cfg::from_str)
      .map(|cfg| {
        cfg.expect("attrs from rustc should be parsable into Cargo Cfg")
      })
      .collect(),
  )
}
