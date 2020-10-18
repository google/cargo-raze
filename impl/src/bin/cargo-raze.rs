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
  io::Write,
  path::{Path, PathBuf},
};

use anyhow::Result;

use docopt::Docopt;

use cargo_raze::{
  metadata::{CargoMetadataFetcher, CargoWorkspaceFiles, MetadataFetcher},
  planning::{BuildPlanner, BuildPlannerImpl},
  rendering::{bazel::BazelRenderer, BuildRenderer, FileOutputs, RenderDetails},
  settings::{load_settings, GenMode},
  util::{find_bazel_workspace_root, PlatformDetails},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Options {
  arg_buildprefix: Option<String>,
  flag_verbose: u32,
  flag_quiet: Option<bool>,
  flag_host: Option<String>,
  flag_color: Option<String>,
  flag_target: Option<String>,
  flag_dryrun: Option<bool>,
  flag_cargo_bin_path: Option<String>,
  flag_output: String,
}

const USAGE: &str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze (-h | --help)
    cargo raze [--verbose] [--quiet] [--color=<WHEN>] [--dryrun] [--cargo-bin-path=<PATH>] [--output=<PATH>]
    cargo raze <buildprefix> [--verbose] [--quiet] [--color=<WHEN>] [--dryrun] [--cargo-bin-path=<PATH>]
                             [--output=<PATH>]

Options:
    -h, --help                         Print this message
    -v, --verbose                      Use verbose output
    -q, --quiet                        No output printed to stdout
    --color=<WHEN>                     Coloring: auto, always, never
    -d, --dryrun                       Do not emit any files
    --cargo-bin-path=<PATH>            Path to the cargo binary to be used for loading workspace metadata
    --output=<PATH>                    Path to output the generated into.
"#;

fn main() -> Result<()> {
  let options: Options = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  let settings = load_settings("Cargo.toml")?;
  if options.flag_verbose > 0 {
    println!("Loaded override settings: {:#?}", settings);
  }
  
  let mut metadata_fetcher: Box<dyn MetadataFetcher> = match options.flag_cargo_bin_path {
    Some(ref p) => Box::new(CargoMetadataFetcher::new(p, /*use_tempdir: */ true)),
    None => Box::new(CargoMetadataFetcher::default()),
  };
  let mut planner = BuildPlannerImpl::new(&mut *metadata_fetcher);

  let toml_path = PathBuf::from("./Cargo.toml");
  let lock_path_opt = fs::metadata("./Cargo.lock")
    .ok()
    .map(|_| PathBuf::from("./Cargo.lock"));
  let files = CargoWorkspaceFiles {
    toml_path,
    lock_path_opt,
  };

  let platform_details = match &settings.target {
    Some(target) => Some(PlatformDetails::new_using_rustc(target)?),
    None => None,
  };

  let prefix_path = calculate_workspace_root(
    &settings.workspace_path,
    match !options.flag_output.is_empty() {
      true => Some(&options.flag_output),
      false => None,
    },
    settings.incompatible_relative_workspace_path,
  )?;

  let planned_build = planner.plan_build(&settings, &prefix_path, files, platform_details)?;
  let mut bazel_renderer = BazelRenderer::new();

  let render_details = RenderDetails {
    path_prefix: prefix_path,
    buildfile_suffix: settings.output_buildfile_suffix,
  };

  let dry_run = options.flag_dryrun.unwrap_or(false);
  if !dry_run {
    fs::create_dir_all(&render_details.path_prefix)?;
  }

  let bazel_file_outputs = match settings.genmode {
    GenMode::Vendored => bazel_renderer.render_planned_build(&render_details, &planned_build)?,
    GenMode::Remote => {
      if !dry_run {
        // Create the "remote" directory if it doesn't exist
        fs::create_dir_all(render_details.path_prefix.as_path().join("remote"))?;

        if !settings.binary_deps.is_empty() {
          fs::create_dir_all(
            render_details
              .path_prefix
              .clone()
              .as_path()
              .join("lockfiles"),
          )?;
        }
      }

      bazel_renderer.render_remote_planned_build(&render_details, &planned_build)?
    }, /* exhaustive, we control the definition */
  };

  for FileOutputs {
    path,
    contents,
  } in bazel_file_outputs
  {
    if dry_run {
      println!("{}:\n{}", path.display(), contents);
    } else {
      write_to_file(&path, &contents, options.flag_verbose > 0)?;
    }
  }

  Ok(())
}

fn write_to_file(path: &Path, contents: &str, verbose: bool) -> Result<()> {
  File::create(&path).and_then(|mut f| f.write_all(contents.as_bytes()))?;
  if verbose {
    println!("Generated {} successfully", path.display());
  }
  Ok(())
}

fn calculate_workspace_root(
  path: &str,
  output_override: Option<&str>,
  new_behavior: bool,
) -> Result<PathBuf> {
  // Default to the current directory '.'
  let mut prefix_path: PathBuf = std::env::current_dir()?;

  // Allow the command line option to take precedence
  match output_override {
    Some(output) => {
      prefix_path.clear();
      prefix_path.push(output);
    },
    None => {
      if new_behavior {
        if let Some(workspace_root) = find_bazel_workspace_root() {
          prefix_path.clear();
          prefix_path.push(workspace_root);
          prefix_path.push(
            path
              // Remove the leading "//" from the path
              .trim_start_matches('/'),
          );
        }
      }
    },
  }

  Ok(prefix_path)
}

#[test]
fn test_calculate_workspace_root() {
  assert_eq!(
    calculate_workspace_root(&"path_a".to_string(), None, true).unwrap(),
    std::env::current_dir().unwrap()
  );

  assert_eq!(
    calculate_workspace_root(
      &"path_b".to_string(),
      Some(&"direct/path".to_string()),
      true
    )
    .unwrap(),
    std::path::Path::new("direct").join("path")
  );
}
