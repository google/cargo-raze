use std::{fs::File, io::Write};
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
