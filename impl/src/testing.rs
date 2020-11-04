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

use flate2::Compression;
use httpmock::{Method::GET, MockServer};
use serde_json::json;
use std::{
  collections::HashMap,
  fs::{create_dir_all, write, File},
  io::Write,
};
use tempfile::TempDir;

use crate::metadata::CargoWorkspaceFiles;

pub fn basic_toml() -> &'static str {
  "
[package]
name = \"test\"
version = \"0.0.1\"

[lib]
path = \"not_a_file.rs\"
    "
}

pub fn basic_lock() -> &'static str {
  "
[[package]]
name = \"test\"
version = \"0.0.1\"
dependencies = [
]
    "
}

pub fn named_toml_contents(name: &str, version: &str) -> String {
  indoc::formatdoc! { r#"
    [package]
    name = "{name}"
    version = "{version}"

    [lib]
    path = "not_a_file.rs"

  "#, name = name, version = version }
}

pub fn named_lock_contents(name: &str, version: &str) -> String {
  indoc::formatdoc! { r#"
    [[package]]
    name = "{name}"
    version = "{version}"

    dependencies = [
    ]

  "#, name = name, version = version }
}

pub fn make_workspace(
  toml_file: &'static str,
  lock_file: Option<&'static str>,
) -> (TempDir, CargoWorkspaceFiles) {
  let dir = TempDir::new().unwrap();
  let toml_path = {
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
    },
    None => None,
  };
  let files = CargoWorkspaceFiles {
    lock_path_opt: lock_path,
    toml_path,
  };

  File::create(dir.as_ref().join("WORKSPACE.bazel")).unwrap();
  (dir, files)
}

pub fn make_basic_workspace() -> (TempDir, CargoWorkspaceFiles) {
  make_workspace(basic_toml(), Some(basic_lock()))
}

/**
 * Configures the given mock_server (representing a crates.io remote) to return
 * mock responses for the given crate and version .
 */
pub fn mock_remote_crate(name: &str, version: &str, mock_server: &MockServer) -> TempDir {
  // Crate info mock response
  mock_server.mock(|when, then| {
    when.method(GET).path(format!("/api/v1/crates/{}", name));
    // Note that `crate[versions]` is an arbitrary value that must only match a `versions[id]`
    then.status(200).json_body(json!({
        "crate": {
            "id": name,
            "name": name,
            "versions": [
                123456
            ],
        },
        "versions": [
            {
                "id": 123456,
                "crate": name,
                "num": version,
                "dl_path": format!("/api/v1/crates/{}/{}/download", name, version),
            }
        ],
    }));
  });

  // Create archive contents
  let dir = tempfile::TempDir::new().unwrap();
  let tar_path = dir.as_ref().join(format!("{}.tar.gz", name));
  {
    create_dir_all(dir.as_ref().join("archive")).unwrap();
    File::create(dir.as_ref().join("archive/test")).unwrap();
    write(
      dir.as_ref().join("archive/Cargo.toml"),
      named_toml_contents(name, version),
    )
    .unwrap();
    write(
      dir.as_ref().join("archive/Cargo.lock"),
      named_lock_contents(name, version),
    )
    .unwrap();

    let tar_gz: File = File::create(&tar_path).unwrap();
    let enc = flate2::write::GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar
      .append_dir_all(
        format!("{}-{}", name, version),
        dir.as_ref().join("archive"),
      )
      .unwrap();
  }

  // Create download mock response
  mock_server.mock(|when, then| {
    when
      .method(GET)
      .path(format!("/api/v1/crates/{}/{}/download", name, version));
    then
      .status(200)
      .header("content-type", "application/x-tar")
      .body_from_file(&tar_path.display().to_string());
  });

  dir
}

/** A helper macro for passing a `crates` to  `mock_crate_index` */
pub fn to_index_crates_map(list: Vec<(&str, &str)>) -> HashMap<String, String> {
  list
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

/** Create a mock cache in a temporary direcotry that contains a set of given crates */
pub fn mock_crate_index(crates: &HashMap<String, String>) -> TempDir {
  let index_url_mock_dir = TempDir::new().unwrap();

  for (name, version) in crates {
    let crate_index_path = if name.len() < 4 {
      index_url_mock_dir
        .as_ref()
        .join(name.len().to_string())
        .join(name)
    } else {
      index_url_mock_dir
        .as_ref()
        .join(&name.as_str()[0..2])
        .join(&name.as_str()[2..4])
        .join(name)
    };

    create_dir_all(crate_index_path.parent().unwrap()).unwrap();
    write(
      crate_index_path,
      json!({
        "name": name,
        "vers": version,
        "deps": [],
        "cksum": "8a648e87a02fa31d9d9a3b7c76dbfee469402fbb4af3ae98b36c099d8a82bb18",
        "features": {},
        "yanked": false,
        "links": null
      })
      .to_string(),
    )
    .unwrap();
  }

  index_url_mock_dir
}
