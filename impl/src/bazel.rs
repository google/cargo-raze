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

use tera::{self, Context, Tera};

use crate::{
  context::{CrateContext, WorkspaceContext},
  planning::PlannedBuild,
  rendering::{BuildRenderer, FileOutputs, RenderDetails},
  util::RazeError,
};

use std::{env, error::Error, path::PathBuf};

/** Returns whether or not the given path is a Bazel workspace root */
pub fn is_workspace_root(dir: &PathBuf) -> bool {
  let workspace_files = [dir.join("WORKSPACE.bazel"), dir.join("WORKSPACE")];

  for workspace in workspace_files.iter() {
    if workspace.exists() {
      return true;
    }
  }

  return false;
}

/** Returns a path to a Bazel workspace root based on the current working
 * directory, otherwise None if not workspace is detected.
 */
pub fn find_workspace_root() -> Option<PathBuf> {
  let mut dir = match env::current_dir() {
    Ok(result) => Some(result),
    Err(_) => None,
  };

  while let Some(current_dir) = dir {
    if is_workspace_root(&current_dir) {
      return Some(current_dir);
    }

    dir = match current_dir.parent() {
      Some(parent) => Some(parent.to_path_buf()),
      None => None,
    };
  }

  return None;
}

#[derive(Default)]
pub struct BazelRenderer {
  internal_renderer: Tera,
}

impl BazelRenderer {
  pub fn new() -> Self {
    // Configure tera with a bogus template dir: We don't want any runtime template support
    let mut internal_renderer = Tera::new("src/not/a/dir/*").unwrap();
    internal_renderer
      .add_raw_templates(vec![
        (
          "templates/partials/build_script.template",
          include_str!("templates/partials/build_script.template"),
        ),
        (
          "templates/partials/rust_binary.template",
          include_str!("templates/partials/rust_binary.template"),
        ),
        (
          "templates/partials/rust_library.template",
          include_str!("templates/partials/rust_library.template"),
        ),
        (
          "templates/partials/common_attrs.template",
          include_str!("templates/partials/common_attrs.template"),
        ),
        (
          "templates/workspace.BUILD.template",
          include_str!("templates/workspace.BUILD.template"),
        ),
        (
          "templates/crate.BUILD.template",
          include_str!("templates/crate.BUILD.template"),
        ),
        (
          "templates/remote_crates.bzl.template",
          include_str!("templates/remote_crates.bzl.template"),
        ),
        (
          "templates/partials/remote_crates_patch.template",
          include_str!("templates/partials/remote_crates_patch.template"),
        ),
      ])
      .unwrap();

    Self { internal_renderer }
  }

  pub fn render_crate(
    &self,
    workspace_context: &WorkspaceContext,
    package: &CrateContext,
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.insert("workspace", &workspace_context);
    context.insert("crate", &package);
    self
      .internal_renderer
      .render("templates/crate.BUILD.template", &context)
  }

  pub fn render_aliases(
    &self,
    workspace_context: &WorkspaceContext,
    all_packages: &[CrateContext],
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.insert("workspace", &workspace_context);
    context.insert("crates", &all_packages);
    self
      .internal_renderer
      .render("templates/workspace.BUILD.template", &context)
  }

  pub fn render_remote_crate(
    &self,
    workspace_context: &WorkspaceContext,
    package: &CrateContext,
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.insert("workspace", &workspace_context);
    context.insert("crate", &package);
    self
      .internal_renderer
      .render("templates/crate.BUILD.template", &context)
  }

  pub fn render_remote_aliases(
    &self,
    workspace_context: &WorkspaceContext,
    all_packages: &[CrateContext],
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.insert("workspace", &workspace_context);
    context.insert("crates", &all_packages);
    self
      .internal_renderer
      .render("templates/workspace.BUILD.template", &context)
  }

  pub fn render_bzl_fetch(
    &self,
    workspace_context: &WorkspaceContext,
    all_packages: &[CrateContext],
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.insert("workspace", &workspace_context);
    context.insert("crates", &all_packages);
    self
      .internal_renderer
      .render("templates/remote_crates.bzl.template", &context)
  }
}

fn include_additional_build_file(
  package: &CrateContext,
  existing_contents: String,
) -> Result<String> {
  match &package.raze_settings.additional_build_file {
    Some(file_path) => {
      let additional_content =
        std::fs::read_to_string(file_path).map_err(|e| RazeError::Rendering {
          crate_name_opt: Some(package.pkg_name.to_owned()),
          message: format!("failed to read additional_build_file: {}", e),
        })?;

      Ok(format!(
        "{}\n# Additional content from {}\n{}",
        existing_contents, file_path, additional_content
      ))
    }

    None => Ok(existing_contents),
  }
}

macro_rules! unwind_tera_error {
  ($err:ident) => {{
    let mut messages = vec![$err.to_string()];
    let mut cause = $err.source();
    while let Some(e) = cause {
      messages.push(e.to_string());
      cause = e.source();
    }
    messages.join("\n└─")
  }};
}

impl BuildRenderer for BazelRenderer {
  fn render_planned_build(
    &mut self,
    render_details: &RenderDetails,
    planned_build: &PlannedBuild,
  ) -> Result<Vec<FileOutputs>> {
    let &RenderDetails {
      ref path_prefix,
      ref buildfile_suffix,
      ..
    } = render_details;
    let &PlannedBuild {
      ref workspace_context,
      ref crate_contexts,
      ..
    } = planned_build;
    let mut file_outputs = Vec::new();

    for package in crate_contexts {
      let rendered_crate_build_file =
        self
          .render_crate(&workspace_context, &package)
          .map_err(|e| RazeError::Rendering {
            crate_name_opt: None,
            message: unwind_tera_error!(e),
          })?;

      let final_crate_build_file =
        include_additional_build_file(package, rendered_crate_build_file)?;

      file_outputs.push(FileOutputs {
        path: format!("{}/{}", path_prefix, package.expected_build_path),
        contents: final_crate_build_file,
      })
    }

    let build_file_path = format!("{}/{}", &path_prefix, buildfile_suffix);
    let rendered_alias_build_file = self
      .render_aliases(&workspace_context, &crate_contexts)
      .map_err(|e| RazeError::Rendering {
        crate_name_opt: None,
        message: unwind_tera_error!(e),
      })?;

    file_outputs.push(FileOutputs {
      path: build_file_path,
      contents: rendered_alias_build_file,
    });
    Ok(file_outputs)
  }

  fn render_remote_planned_build(
    &mut self,
    render_details: &RenderDetails,
    planned_build: &PlannedBuild,
  ) -> Result<Vec<FileOutputs>> {
    let &RenderDetails {
      ref path_prefix,
      ref buildfile_suffix,
      ..
    } = render_details;
    let &PlannedBuild {
      ref workspace_context,
      ref crate_contexts,
      ..
    } = planned_build;
    let mut file_outputs = Vec::new();

    // N.B. File needs to exist so that contained xyz-1.2.3.BUILD can be referenced
    file_outputs.push(FileOutputs {
      path: format!("{}/remote/{}", path_prefix, buildfile_suffix),
      contents: String::new(),
    });

    for package in crate_contexts {
      let rendered_crate_build_file = self
        .render_remote_crate(&workspace_context, &package)
        .map_err(|e| RazeError::Rendering {
          crate_name_opt: Some(package.pkg_name.to_owned()),
          message: unwind_tera_error!(e),
        })?;

      let final_crate_build_file =
        include_additional_build_file(package, rendered_crate_build_file)?;

      file_outputs.push(FileOutputs {
        path: format!("{}/{}", path_prefix, package.expected_build_path),
        contents: final_crate_build_file,
      })
    }

    let alias_file_path = format!("{}/{}", &path_prefix, buildfile_suffix);
    let rendered_alias_build_file = self
      .render_remote_aliases(&workspace_context, &crate_contexts)
      .map_err(|e| RazeError::Rendering {
        crate_name_opt: None,
        message: unwind_tera_error!(e),
      })?;

    file_outputs.push(FileOutputs {
      path: alias_file_path,
      contents: rendered_alias_build_file,
    });

    let bzl_fetch_file_path = format!("{}/crates.bzl", &path_prefix);
    let rendered_bzl_fetch_file = self
      .render_bzl_fetch(&workspace_context, &crate_contexts)
      .map_err(|e| RazeError::Rendering {
        crate_name_opt: None,
        message: unwind_tera_error!(e),
      })?;

    file_outputs.push(FileOutputs {
      path: bzl_fetch_file_path,
      contents: rendered_bzl_fetch_file,
    });

    Ok(file_outputs)
  }
}

#[cfg(test)]
mod tests {
  use hamcrest2::{core::expect, prelude::*};

  use crate::{
    context::*,
    planning::PlannedBuild,
    rendering::{FileOutputs, RenderDetails},
    settings::CrateSettings,
  };

  use super::*;

  use std::fs::File;

  use tempfile::TempDir;

  fn dummy_render_details(buildfile_suffix: &str) -> RenderDetails {
    RenderDetails {
      path_prefix: "./some_render_prefix".to_owned(),
      buildfile_suffix: buildfile_suffix.to_owned(),
    }
  }

  fn dummy_planned_build(crate_contexts: Vec<CrateContext>) -> PlannedBuild {
    PlannedBuild {
      workspace_context: WorkspaceContext {
        workspace_path: "//workspace/prefix".to_owned(),
        platform_triple: "irrelevant".to_owned(),
        gen_workspace_prefix: "".to_owned(),
        output_buildfile_suffix: "BUILD".to_owned(),
      },
      crate_contexts,
    }
  }

  fn dummy_binary_crate_with_name(buildfile_suffix: &str) -> CrateContext {
    CrateContext {
      pkg_name: "test-binary".to_owned(),
      pkg_version: "1.1.1".to_owned(),
      edition: "2015".to_owned(),
      features: vec!["feature1".to_owned(), "feature2".to_owned()].to_owned(),
      expected_build_path: format!("vendor/test-binary-1.1.1/{}", buildfile_suffix),
      license: LicenseData::default(),
      raze_settings: CrateSettings::default(),
      dependencies: Vec::new(),
      proc_macro_dependencies: Vec::new(),
      build_dependencies: Vec::new(),
      build_proc_macro_dependencies: Vec::new(),
      dev_dependencies: Vec::new(),
      aliased_dependencies: Vec::new(),
      is_root_dependency: true,
      workspace_path_to_crate: "@raze__test_binary__1_1_1//".to_owned(),
      targets: vec![BuildableTarget {
        name: "some_binary".to_owned(),
        kind: "bin".to_owned(),
        path: "bin/main.rs".to_owned(),
        edition: "2015".to_owned(),
      }],
      build_script_target: None,
      source_details: SourceDetails { git_data: None },
      sha256: None,
      lib_target_name: None,
    }
  }

  fn dummy_binary_crate() -> CrateContext {
    return dummy_binary_crate_with_name("BUILD");
  }

  fn dummy_library_crate_with_name(buildfile_suffix: &str) -> CrateContext {
    CrateContext {
      pkg_name: "test-library".to_owned(),
      pkg_version: "1.1.1".to_owned(),
      edition: "2015".to_owned(),
      license: LicenseData::default(),
      raze_settings: CrateSettings::default(),
      features: vec!["feature1".to_owned(), "feature2".to_owned()].to_owned(),
      expected_build_path: format!("vendor/test-library-1.1.1/{}", buildfile_suffix),
      dependencies: Vec::new(),
      proc_macro_dependencies: Vec::new(),
      build_dependencies: Vec::new(),
      build_proc_macro_dependencies: Vec::new(),
      dev_dependencies: Vec::new(),
      aliased_dependencies: Vec::new(),
      is_root_dependency: true,
      workspace_path_to_crate: "@raze__test_library__1_1_1//".to_owned(),
      targets: vec![BuildableTarget {
        name: "some_library".to_owned(),
        kind: "lib".to_owned(),
        path: "path/lib.rs".to_owned(),
        edition: "2015".to_owned(),
      }],
      build_script_target: None,
      source_details: SourceDetails { git_data: None },
      sha256: None,
      lib_target_name: Some("test_library".to_owned()),
    }
  }

  fn dummy_library_crate() -> CrateContext {
    return dummy_library_crate_with_name("BUILD");
  }

  fn extract_contents_matching_path(file_outputs: &Vec<FileOutputs>, file_name: &str) -> String {
    println!("Known files :{:?}", file_outputs);
    let mut matching_files_contents = file_outputs
      .iter()
      .filter(|output| output.path.starts_with(file_name))
      .map(|output| output.contents.to_owned())
      .collect::<Vec<String>>();

    assert_that!(matching_files_contents.len(), equal_to(1));
    matching_files_contents.pop().unwrap()
  }

  fn render_crates_for_test_with_name(
    buildfile_suffix: &str,
    crate_contexts: Vec<CrateContext>,
  ) -> Vec<FileOutputs> {
    BazelRenderer::new()
      .render_planned_build(
        &dummy_render_details(buildfile_suffix),
        &dummy_planned_build(crate_contexts),
      )
      .unwrap()
  }

  fn render_crates_for_test(crate_contexts: Vec<CrateContext>) -> Vec<FileOutputs> {
    return render_crates_for_test_with_name("BUILD", crate_contexts);
  }

  #[test]
  fn all_plans_contain_root_build_file() {
    let file_outputs = render_crates_for_test(Vec::new());
    let file_names = file_outputs
      .iter()
      .map(|output| output.path.as_ref())
      .collect::<Vec<&str>>();

    assert_that!(
      &file_names,
      contains(vec!["./some_render_prefix/BUILD"]).exactly()
    );
  }

  #[test]
  fn crates_generate_build_files() {
    let file_outputs = render_crates_for_test(vec![dummy_library_crate()]);
    let file_names = file_outputs
      .iter()
      .map(|output| output.path.as_ref())
      .collect::<Vec<&str>>();

    assert_that!(
      &file_names,
      contains(vec![
        "./some_render_prefix/vendor/test-library-1.1.1/BUILD",
        "./some_render_prefix/BUILD",
      ])
      .exactly()
    );
  }

  #[test]
  fn crates_generate_build_files_bazel() {
    let file_outputs = render_crates_for_test_with_name(
      "BUILD.bazel",
      vec![dummy_library_crate_with_name("BUILD.bazel")],
    );
    let file_names = file_outputs
      .iter()
      .map(|output| output.path.as_ref())
      .collect::<Vec<&str>>();

    assert_that!(
      &file_names,
      contains(vec![
        "./some_render_prefix/vendor/test-library-1.1.1/BUILD.bazel",
        "./some_render_prefix/BUILD.bazel",
      ])
      .exactly()
    );
  }

  #[test]
  fn root_crates_get_build_aliases() {
    let file_outputs = render_crates_for_test(vec![dummy_library_crate()]);
    let root_build_contents =
      extract_contents_matching_path(&file_outputs, "./some_render_prefix/BUILD");

    expect(
      root_build_contents.contains("alias"),
      format!(
        "expected root build contents to contain an alias for test-library crate, but it just \
         contained [{}]",
        root_build_contents
      ),
    )
    .unwrap();
  }

  #[test]
  fn non_root_crates_dont_get_build_aliases() {
    let mut non_root_crate = dummy_library_crate();
    non_root_crate.is_root_dependency = false;

    let file_outputs = render_crates_for_test(vec![non_root_crate]);
    let root_build_contents =
      extract_contents_matching_path(&file_outputs, "./some_render_prefix/BUILD");

    expect(
      !root_build_contents.contains("alias"),
      format!(
        "expected root build contents not to contain an alias for test-library crate, but it just \
         contained [{}]",
        root_build_contents
      ),
    )
    .unwrap();
  }

  #[test]
  fn binaries_get_rust_binary_rules() {
    let file_outputs = render_crates_for_test(vec![dummy_binary_crate()]);
    let crate_build_contents = extract_contents_matching_path(
      &file_outputs,
      "./some_render_prefix/vendor/test-binary-1.1.1/BUILD",
    );

    expect(
      crate_build_contents.contains("rust_binary("),
      format!(
        "expected crate build contents to contain rust_binary, but it just contained [{}]",
        crate_build_contents
      ),
    )
    .unwrap();
  }

  #[test]
  fn libraries_get_rust_library_rules() {
    let file_outputs = render_crates_for_test(vec![dummy_library_crate()]);
    let crate_build_contents = extract_contents_matching_path(
      &file_outputs,
      "./some_render_prefix/vendor/test-library-1.1.1/BUILD",
    );

    expect(
      crate_build_contents.contains("rust_library("),
      format!(
        "expected crate build contents to contain rust_library, but it just contained [{}]",
        crate_build_contents
      ),
    )
    .unwrap();
  }

  #[test]
  fn additional_build_file_missing_file_failure() {
    let render_result = BazelRenderer::new().render_planned_build(
      &dummy_render_details("BUILD"),
      &dummy_planned_build(vec![CrateContext {
        raze_settings: CrateSettings {
          additional_build_file: Some("non-existent-file".into()),
          ..Default::default()
        },
        ..dummy_library_crate()
      }]),
    );

    assert_that!(render_result, err());
  }

  #[test]
  fn additional_build_file_included() {
    let file_outputs = render_crates_for_test(vec![CrateContext {
      raze_settings: CrateSettings {
        additional_build_file: Some("README.md".into()),
        ..Default::default()
      },
      ..dummy_library_crate()
    }]);
    let crate_build_contents = extract_contents_matching_path(
      &file_outputs,
      "./some_render_prefix/vendor/test-library-1.1.1/BUILD",
    );

    expect(
      crate_build_contents.contains("# Additional content from README.md"),
      format!(
        "expected crate build contents to include additional_build_file, but it just contained [{}]",
        crate_build_contents
      ),
    )
    .unwrap();
  }

  #[test]
  fn detecting_workspace_root() {
    // Cache the cwd
    let cwd = env::current_dir().unwrap();

    // Run test
    let result = std::panic::catch_unwind(|| {
      // Generate a temporary directory to do testing in
      let bazel_root = TempDir::new().unwrap();
      assert!(env::set_current_dir(&bazel_root).is_ok());

      // Starting within the temp directory, we'll find that there are no WORKSPACE.bazel files
      // and thus return None to indicate a Bazel workspace root could not be found.
      assert_eq!(find_workspace_root(), None);

      // After creating a WORKSPACE.bazel file in that directory, we expect to find to be
      // returned a path to the temporary directory
      File::create(bazel_root.path().join("WORKSPACE.bazel")).unwrap();
      assert_eq!(
        find_workspace_root().unwrap().canonicalize().unwrap(),
        bazel_root.into_path().canonicalize().unwrap()
      );
    });

    // Restore cwd
    assert!(env::set_current_dir(&cwd).is_ok());

    // Ensure test results were successful
    assert!(result.is_ok());
  }
}
