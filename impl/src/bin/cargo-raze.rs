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

use std::{
  fs::{self, File},
  io::{Read, Write},
  path::{Path, PathBuf},
};

#[cfg(cargo_internals)]
use cargo::util::Config;

use docopt::Docopt;

use cargo_raze::{
  bazel::BazelRenderer,
  metadata::{CargoSubcommandMetadataFetcher, CargoWorkspaceFiles, MetadataFetcher},
  planning::{BuildPlanner, BuildPlannerImpl},
  rendering::{BuildRenderer, FileOutputs, RenderDetails},
  settings::{CargoToml, GenMode, RazeSettings},
  util::{PlatformDetails, RazeError, RazeResult},
};

#[cfg(cargo_internals)]
use cargo_raze::metadata::CargoInternalsMetadataFetcher;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Options {
  arg_buildprefix: Option<String>,
  flag_verbose: u32,
  flag_quiet: Option<bool>,
  flag_host: Option<String>,
  flag_color: Option<String>,
  flag_target: Option<String>,
  flag_dryrun: Option<bool>,
  flag_deprecated_use_cargo_internals: Option<bool>,
  flag_cargo_bin_path: Option<String>,
}

const USAGE: &str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze (-h | --help)
    cargo raze [--verbose] [--quiet] [--color=<WHEN>] [--dryrun] [--cargo-bin-path=<PATH>] [--deprecated-use-cargo-internals]
    cargo raze <buildprefix> [--verbose] [--quiet] [--color=<WHEN>] [--dryrun] [--cargo-bin-path=<PATH>] [--deprecated-use-cargo-internals]

Options:
    -h, --help                         Print this message
    -v, --verbose                      Use verbose output
    -q, --quiet                        No output printed to stdout
    --color=<WHEN>                     Coloring: auto, always, never
    -d, --dryrun                       Do not emit any files
    --cargo-bin-path=<PATH>            Path to the cargo binary to be used for loading workspace metadata
    --deprecated-use-cargo-internals   Force usage of cargo internals (instead of cargo-metadata)
"#;

fn main() {
  let options = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  let metadata_fetcher = create_fetcher(&options);
  let result = real_main(&options, metadata_fetcher);

  if let Err(_e) = result {
    std::process::exit(-1);
  }
}

#[cfg(cargo_internals)]
fn create_fetcher(options: &Options) -> Box<dyn MetadataFetcher> {
  create_cargo_internals_fetcher(options)
}

#[cfg(not(cargo_internals))]
fn create_fetcher(options: &Options) -> Box<dyn MetadataFetcher> {
  create_subcommand_fetcher(options)
}

#[cfg(cargo_internals)]
fn create_cargo_internals_fetcher(options: &Options) -> Box<dyn MetadataFetcher> {
  if options.flag_deprecated_use_cargo_internals.unwrap_or(false) {
    let mut cargo_config = Config::default().unwrap();
    cargo_config.configure(
      options.flag_verbose,
      options.flag_quiet,
      &options.flag_color,
      /* frozen = */ false,
      /* locked = */ false,
      /* offline */ false,
      /* target_dir = */ &None,
      &[],
    )?;
    Box::new(CargoInternalsMetadataFetcher::new(cargo_config))
  } else {
     create_subcommand_fetcher()
  }
}

fn create_subcommand_fetcher(options: &Options) -> Box<dyn MetadataFetcher> {
  match options.flag_cargo_bin_path {
    Some(ref p) => Box::new(CargoSubcommandMetadataFetcher::new(p)),
    None => Box::new(CargoSubcommandMetadataFetcher::default()),
  }
}

fn real_main(options: &Options, mut metadata_fetcher: Box<dyn MetadataFetcher>) -> RazeResult<()> {
  let mut settings = load_settings("Cargo.toml")?;
  println!("Loaded override settings: {:#?}", settings);
  validate_settings(&mut settings)?;

  let mut planner = BuildPlannerImpl::new(&mut* metadata_fetcher);

  let toml_path = PathBuf::from("./Cargo.toml");
  let lock_path_opt = fs::metadata("./Cargo.lock")
    .ok()
    .map(|_| PathBuf::from("./Cargo.lock"));
  let files = CargoWorkspaceFiles {
    toml_path,
    lock_path_opt,
  };
  let platform_details = PlatformDetails::new_using_rustc(&settings.target)?;
  let planned_build = planner.plan_build(&settings, files, platform_details)?;
  let mut bazel_renderer = BazelRenderer::new();
  let render_details = RenderDetails {
    path_prefix: "./".to_owned(),
    buildfile_suffix: settings.output_buildfile_suffix,
  };

  let bazel_file_outputs = match settings.genmode {
    GenMode::Vendored => bazel_renderer.render_planned_build(&render_details, &planned_build)?,
    GenMode::Remote => {
      // Create "remote/" if it doesn't exist
      if fs::metadata("remote/").is_err() {
        match fs::create_dir("remote/") {
          Ok(()) => (),
          Err(_e) => return Err(anyhow::format_err!("couldn't create remote/ dir")),
        }
      }

      bazel_renderer.render_remote_planned_build(&render_details, &planned_build)?
    } /* exhaustive, we control the definition */
  };

  let dry_run = options.flag_dryrun.unwrap_or(false);
  for FileOutputs { path, contents } in bazel_file_outputs {
    if dry_run {
      println!("{}:\n{}", path, contents);
    } else {
      write_to_file_loudly(&path, &contents)?;
    }
  }

  Ok(())
}

/** Verifies that the provided settings make sense. */
fn validate_settings(settings: &mut RazeSettings) -> RazeResult<()> {
  if !settings.workspace_path.starts_with("//") {
    return Err(
      RazeError::Config {
        field_path_opt: Some("raze.workspace_path".to_owned()),
        message: concat!(
          "Path must start with \"//\". Paths into local repositories (such as ",
          "@local//path) are currently unsupported."
        )
        .to_owned(),
      }
      .into(),
    );
  }

  if settings.workspace_path != "//" && settings.workspace_path.ends_with('/') {
    settings.workspace_path.pop();
  }

  Ok(())
}

fn write_to_file_loudly(path: &str, contents: &str) -> RazeResult<()> {
  File::create(&path).and_then(|mut f| f.write_all(contents.as_bytes()))?;
  println!("Generated {} successfully", path);
  Ok(())
}

fn load_settings<T: AsRef<Path>>(cargo_toml_path: T) -> RazeResult<RazeSettings> {
  let path = cargo_toml_path.as_ref();
  let mut toml = File::open(path)?;
  let mut toml_contents = String::new();
  toml.read_to_string(&mut toml_contents)?;
  toml::from_str::<CargoToml>(&toml_contents)
    .map_err(|e| e.into())
    .map(|toml| toml.raze)
}
