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
  collections::HashMap,
  fs::{self, File},
  io::Write,
  path::{Path, PathBuf},
};

use anyhow::Result;

use docopt::Docopt;

use cargo_raze::{
  checks,
  metadata::{CargoWorkspaceFiles, RazeMetadataFetcher},
  planning::{BuildPlanner, BuildPlannerImpl},
  rendering::{bazel::BazelRenderer, BuildRenderer, RenderDetails},
  settings::RazeSettings,
  settings::{load_settings, GenMode},
  util::{find_bazel_workspace_root, PlatformDetails},
};

use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
struct Options {
  flag_verbose: Option<bool>,
  flag_quiet: Option<bool>,
  flag_host: Option<String>,
  flag_color: Option<String>,
  flag_target: Option<String>,
  flag_dryrun: Option<bool>,
  flag_cargo_bin_path: Option<String>,
  flag_output: Option<String>,
  flag_manifest_path: Option<String>,
}

const USAGE: &str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze (-h | --help)
    cargo raze [--verbose] [--quiet] [--color=<WHEN>] [--dryrun] [--cargo-bin-path=<PATH>] 
               [--manifest-path=<PATH>] [--output=<PATH>] 
    cargo raze (-V | --version)

Options:
    -h, --help                          Print this message
    -V, --version                       Print version info and exit
    -v, --verbose                       Use verbose output
    -q, --quiet                         No output printed to stdout
    --color=<WHEN>                      Coloring: auto, always, never
    -d, --dryrun                        Do not emit any files
    --cargo-bin-path=<PATH>             Path to the cargo binary to be used for loading workspace metadata
    --manifest-path=<PATH>              Path to the Cargo.toml file to generate BUILD files for
    --output=<PATH>                     Path to output the generated into.
"#;

fn main() -> Result<()> {
  // Parse options
  let options: Options = Docopt::new(USAGE)
    .map(|d| {
      d.version(Some(
        concat!("cargo-raze ", env!("CARGO_PKG_VERSION")).to_string(),
      ))
    })
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  // Load settings
  let manifest_path = PathBuf::from(
    options
      .flag_manifest_path
      .unwrap_or("./Cargo.toml".to_owned()),
  )
  .canonicalize()?;
  let settings = load_settings(&manifest_path)?;
  if options.flag_verbose.unwrap_or(false) {
    println!("Loaded override settings: {:#?}", settings);
  }

  // Fetch metadata
  let metadata_fetcher: RazeMetadataFetcher = match options.flag_cargo_bin_path {
    Some(ref cargo_bin_path) => RazeMetadataFetcher::new(
      cargo_bin_path,
      Url::parse(&settings.registry)?,
      Url::parse(&settings.index_url)?,
    ),
    None => RazeMetadataFetcher::default(),
  };
  let cargo_raze_working_dir =
    find_bazel_workspace_root(&manifest_path).unwrap_or(std::env::current_dir()?);
  let lock_path_opt = if let Some(parent) = manifest_path.parent() {
    let lock_path = parent.join("Cargo.lock");
    fs::metadata(&lock_path).ok().map(|_| lock_path)
  } else {
    None
  };
  let files = CargoWorkspaceFiles {
    toml_path: manifest_path,
    lock_path_opt,
  };
  let remote_genmode_inputs = gather_remote_genmode_inputs(&cargo_raze_working_dir, &settings);
  let metadata = metadata_fetcher.fetch_metadata(
    &files,
    remote_genmode_inputs.binary_deps,
    remote_genmode_inputs.override_lockfile,
  )?;
  checks::check_metadata(&metadata.metadata, &settings, &cargo_raze_working_dir)?;

  // Do Planning
  let platform_details = match &settings.target {
    Some(target) => Some(PlatformDetails::new_using_rustc(target)?),
    None => None,
  };
  let planned_build =
    BuildPlannerImpl::new(metadata.clone(), settings.clone()).plan_build(platform_details)?;

  // Render BUILD files
  let mut bazel_renderer = BazelRenderer::new();
  let render_details = RenderDetails {
    cargo_root: metadata.cargo_workspace_root,
    path_prefix: PathBuf::from(&settings.workspace_path.trim_start_matches("/")),
    workspace_member_output_dir: settings.workspace_member_dir,
    vendored_buildfile_name: settings.output_buildfile_suffix,
    bazel_root: cargo_raze_working_dir,
  };
  let bazel_file_outputs = match &settings.genmode {
    GenMode::Vendored => bazel_renderer.render_planned_build(&render_details, &planned_build)?,
    GenMode::Remote => {
      bazel_renderer.render_remote_planned_build(&render_details, &planned_build)?
    }, /* exhaustive, we control the definition */
    // There are no file outputs to produce if `genmode` is Unspecified
    GenMode::Unspecified => Vec::new(),
  };

  // Write BUILD files
  if &settings.genmode == &GenMode::Remote {
    let remote_dir = render_details
      .bazel_root
      .join(render_details.path_prefix)
      .join("remote");
    // Clean out the "remote" directory so users can easily see what build files are relevant
    if remote_dir.exists() {
      let build_glob = format!("{}/BUILD*.bazel", remote_dir.display());
      for entry in glob::glob(&build_glob)? {
        if let Ok(path) = entry {
          fs::remove_file(path)?;
        }
      }
    }
  }
  for output in bazel_file_outputs.iter() {
    if options.flag_dryrun.unwrap_or(false) {
      println!("{}:\n{}", output.path.display(), output.contents);
      continue;
    }
    // Ensure all parent directories exist
    if let Some(parent) = &output.path.parent() {
      fs::create_dir_all(parent)?
    }
    write_to_file(
      &output.path,
      &output.contents,
      options.flag_verbose.unwrap_or(false),
    )?;
  }

  Ok(())
}

/** Writes rendered files to filesystem. */
fn write_to_file(path: &Path, contents: &str, verbose: bool) -> Result<()> {
  File::create(&path).and_then(|mut f| f.write_all(contents.as_bytes()))?;
  if verbose {
    println!("Generated {} successfully", path.display());
  }
  Ok(())
}

/* Represents the inputs specific to the Remote genmode. */
struct RemoteGenModeInputs<'settings> {
  pub override_lockfile: Option<PathBuf>,
  pub binary_deps: Option<&'settings HashMap<String, cargo_toml::Dependency>>,
}

/** Gathers inputs for the `genmode = "Remote"` builds. */
fn gather_remote_genmode_inputs<'settings>(
  bazel_root: &Path,
  settings: &'settings RazeSettings,
) -> RemoteGenModeInputs<'settings> {
  if settings.genmode != GenMode::Remote {
    return RemoteGenModeInputs {
      override_lockfile: None,
      binary_deps: None,
    };
  }

  let lockfile = bazel_root
    .join(settings.workspace_path.trim_start_matches("/"))
    .join("Cargo.raze.lock");

  RemoteGenModeInputs {
    override_lockfile: if lockfile.exists() {
      Some(lockfile)
    } else {
      None
    },
    binary_deps: Some(&settings.binary_deps),
  }
}
