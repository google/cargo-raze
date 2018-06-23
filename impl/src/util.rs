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
use slug;
use std::process::Command;
use std::str;
use std::str::FromStr;
use std::iter::Iterator;
use std::fmt;

pub struct LimitedResults<T> {
  pub items: Vec<T>,
  pub count_extras: usize,
}

impl <T> LimitedResults<T> {
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl <T: fmt::Debug> fmt::Debug for LimitedResults<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.count_extras > 0 {
      write!(f, "{:?} and {} others", &self.items, self.count_extras)
    } else {
      write!(f, "{:?}", &self.items)
    }
  }
}

pub fn collect_up_to<T, U: Iterator<Item=T>>(max: usize, iter: U) -> LimitedResults<T> {
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
    items: items,
    count_extras: count_extras
  }
}

pub fn sanitize_ident(ident: &str) -> String {
  slug::slugify(&ident).replace("-", "_")
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

  let output = try!(Command::new("rustc").args(&args).output().map_err(|_| CargoError::from(
    format!("could not run rustc to fetch attrs for target {}", target)
  )));

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
      .map(|cfg| cfg.expect("attrs from rustc should be parsable into Cargo Cfg"))
      .collect(),
  )
}
