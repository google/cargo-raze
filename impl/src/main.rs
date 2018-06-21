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

extern crate cargo;
extern crate itertools;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tempdir;
extern crate tera;
extern crate toml;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod bazel;
mod context;
mod license;
mod metadata;
mod planning;
mod rendering;
mod settings;
mod util;

use bazel::BazelRenderer;
use cargo::CargoError;
use cargo::CliResult;
use cargo::util::CargoResult;
use cargo::util::Config;
use metadata::CargoInternalsMetadataFetcher;
use metadata::CargoWorkspaceFiles;
use planning::BuildPlanner;
use planning::BuildPlannerImpl;
use rendering::BuildRenderer;
use rendering::FileOutputs;
use rendering::RenderDetails;
use settings::GenMode;
use settings::RazeSettings;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, RustcDecodable)]
struct Options {
  arg_buildprefix: Option<String>,
  flag_verbose: u32,
  flag_quiet: Option<bool>,
  flag_host: Option<String>,
  flag_color: Option<String>,
  flag_target: Option<String>,
  flag_dryrun: Option<bool>,
}

const USAGE: &'static str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze

Options:
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    -q, --quiet               No output printed to stdout
    --color WHEN              Coloring: auto, always, never
    -d, --dryrun              Do not emit any files
"#;

fn main() {
  let cargo_config = Config::default().unwrap();
  let args = env::args().collect::<Vec<_>>();
  let result = cargo::call_main_without_stdin(real_main, &cargo_config, USAGE, &args, false);

  if let Err(e) = result {
    cargo::exit_with_error(e, &mut *cargo_config.shell());
  }
}

fn real_main(options: Options, cargo_config: &Config) -> CliResult {
  try!(cargo_config.configure(
    options.flag_verbose,
    options.flag_quiet,
    &options.flag_color,
    /* frozen = */ false,
    /* locked = */ false
  ));
  let mut settings = try!(load_settings("Cargo.toml"));
  println!("Loaded override settings: {:#?}", settings);

  try!(validate_settings(&mut settings));

  let mut metadata_fetcher = CargoInternalsMetadataFetcher::new(&cargo_config);
  let mut planner = BuildPlannerImpl::new(&mut metadata_fetcher);

  let toml_path = PathBuf::from("./Cargo.toml");
  let mut lock_path_opt = None;
  if fs::metadata("./Cargo.lock").is_ok() {
    lock_path_opt = Some(PathBuf::from("./Cargo.lock"));
  }
  let files = CargoWorkspaceFiles {
    toml_path: toml_path,
    lock_path_opt: lock_path_opt,
  };
  let planned_build = planner.plan_build(&settings, files).unwrap();
  let mut bazel_renderer = BazelRenderer::new();
  let render_details = RenderDetails {
    path_prefix: "./".to_owned(),
  };

  let bazel_file_outputs = match settings.genmode {
    GenMode::Vendored => try!(bazel_renderer.render_planned_build(&render_details, &planned_build)),
    GenMode::Remote => {
      // Create "remote/" if it doesn't exist
      if fs::metadata("remote/").is_err() {
        try!(fs::create_dir("remote/").map_err(|e| CargoError::from(e.to_string())));
      }

      try!(bazel_renderer.render_remote_planned_build(&render_details, &planned_build))
    },
    /* exhaustive, we control the definition */
  };

  let dry_run = options.flag_dryrun.unwrap_or(false);
  for FileOutputs { path, contents } in bazel_file_outputs {
    if !dry_run {
      try!(write_to_file_loudly(&path, &contents));
    } else {
      println!("{}:\n{}", path, contents);
    }
  }

  Ok(())
}

/** Verifies that the provided settings make sense. */
fn validate_settings(settings: &mut RazeSettings) -> CargoResult<()> {
  if !settings.workspace_path.starts_with("//") {
    return Err(CargoError::from(
      "raze.workspace_path must start with \"//\". Paths into local repositories (such as \
       @local//path) are currently unsupported.",
    ));
  }

  if settings.workspace_path == "//" {
    return Err(CargoError::from(
      "raze.workspace_path must not be '//' (it is currently unsupported). Its probably not what \
       you want anyway, as this would vendor the crates directly into //vendor.",
    ));
  }

  if settings.workspace_path.ends_with("/") {
    settings.workspace_path.pop();
  }

  return Ok(());
}

fn write_to_file_loudly(path: &str, contents: &str) -> CargoResult<()> {
  try!(
    File::create(&path)
      .and_then(|mut f| f.write_all(contents.as_bytes()))
      .map_err(|_| CargoError::from(format!("failed to create {}", path)))
  );
  println!("Generated {} successfully", path);
  Ok(())
}

fn load_settings<T: AsRef<Path>>(cargo_toml_path: T) -> Result<RazeSettings, CargoError> {
  let path = cargo_toml_path.as_ref();
  let mut toml = try!(File::open(path).map_err(|e| {
    println!("{:?}", e);
    CargoError::from(format!("Could not load {:?}", path))
  }));
  let mut toml_contents = String::new();
  try!(toml.read_to_string(&mut toml_contents).map_err(|e| {
    println!("{:?}", e);
    CargoError::from(format!("failed to read {:?}", path))
  }));
  toml::from_str::<settings::CargoToml>(&toml_contents)
    .map_err(|e| {
      println!("{:?}", e);
      CargoError::from(format!("failed to parse {:?}", path))
    })
    .map(|toml| toml.raze)
}
