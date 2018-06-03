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
use cargo::util::CargoResult;
use context::CrateContext;
use context::WorkspaceContext;
use planning::PlannedBuild;
use rendering::BuildRenderer;
use rendering::FileOutputs;
use rendering::RenderDetails;
use tera;
use tera::Context;
use tera::Tera;

pub struct BazelRenderer {
  internal_renderer: Tera,
}

impl BazelRenderer {
  pub fn new() -> BazelRenderer {
    // Configure tera with a bogus template dir: We don't want any runtime template support
    let mut renderer = Tera::new("src/not/a/dir/*").unwrap();
    renderer
      .add_raw_templates(vec![
        (
          "templates/partials/build_script.template",
          include_str!("templates/partials/build_script.template"),
        ),
        (
          "templates/partials/remote_build_script.template",
          include_str!("templates/partials/remote_build_script.template"),
        ),
        (
          "templates/partials/rust_binary.template",
          include_str!("templates/partials/rust_binary.template"),
        ),
        (
          "templates/partials/remote_rust_binary.template",
          include_str!("templates/partials/remote_rust_binary.template"),
        ),
        (
          "templates/partials/rust_library.template",
          include_str!("templates/partials/rust_library.template"),
        ),
        (
          "templates/partials/remote_rust_library.template",
          include_str!("templates/partials/remote_rust_library.template"),
        ),
        (
          "templates/workspace.BUILD.template",
          include_str!("templates/workspace.BUILD.template"),
        ),
        (
          "templates/remote_workspace.BUILD.template",
          include_str!("templates/remote_workspace.BUILD.template"),
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
          "templates/remote_crate.BUILD.template",
          include_str!("templates/remote_crate.BUILD.template"),
        ),
      ])
      .unwrap();

    BazelRenderer {
      internal_renderer: renderer,
    }
  }

  pub fn render_crate(
    &self,
    workspace_context: &WorkspaceContext,
    package: &CrateContext,
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("workspace", &workspace_context);
    context.add("crate", &package);
    self
      .internal_renderer
      .render("templates/crate.BUILD.template", &context)
  }

  pub fn render_aliases(
    &self,
    workspace_context: &WorkspaceContext,
    all_packages: &Vec<CrateContext>,
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("workspace", &workspace_context);
    context.add("crates", &all_packages);
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
    context.add("workspace", &workspace_context);
    context.add("crate", &package);
    self
      .internal_renderer
      .render("templates/remote_crate.BUILD.template", &context)
  }

  pub fn render_remote_aliases(
    &self,
    workspace_context: &WorkspaceContext,
    all_packages: &Vec<CrateContext>,
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("workspace", &workspace_context);
    context.add("crates", &all_packages);
    self
      .internal_renderer
      .render("templates/remote_workspace.BUILD.template", &context)
  }

  pub fn render_bzl_fetch(
    &self,
    workspace_context: &WorkspaceContext,
    all_packages: &Vec<CrateContext>,
  ) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("workspace", &workspace_context);
    context.add("crates", &all_packages);
    self
      .internal_renderer
      .render("templates/remote_crates.bzl.template", &context)
  }
}

impl BuildRenderer for BazelRenderer {
  fn render_planned_build(
    &mut self,
    render_details: &RenderDetails,
    planned_build: &PlannedBuild,
  ) -> CargoResult<Vec<FileOutputs>> {
    let &RenderDetails {
      ref path_prefix, ..
    } = render_details;
    let &PlannedBuild {
      ref workspace_context,
      ref crate_contexts,
      ..
    } = planned_build;
    let mut file_outputs = Vec::new();

    for package in crate_contexts {
      let build_file_path = format!("{}/{}BUILD", &path_prefix, &package.path);
      let rendered_crate_build_file = try!(
        self
          .render_crate(&workspace_context, &package)
          .map_err(|e| CargoError::from(e.to_string()))
      );
      file_outputs.push(FileOutputs {
        path: build_file_path,
        contents: rendered_crate_build_file,
      })
    }

    let build_file_path = format!("{}/BUILD", &path_prefix);
    let rendered_alias_build_file = try!(
      self
        .render_aliases(&workspace_context, &crate_contexts)
        .map_err(|e| CargoError::from(e.to_string()))
    );
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
  ) -> CargoResult<Vec<FileOutputs>> {
    let &RenderDetails {
      ref path_prefix, ..
    } = render_details;
    let &PlannedBuild {
      ref workspace_context,
      ref crate_contexts,
      ..
    } = planned_build;
    let mut file_outputs = Vec::new();

    // N.B. File needs to exist so that contained xyz-1.2.3.BUILD can be referenced
    file_outputs.push(FileOutputs {
      path: "remote/BUILD".to_owned(),
      contents: String::new(),
    });

    for package in crate_contexts {
      let build_file_path = format!(
        "remote/{}-{}.BUILD",
        &package.pkg_name, &package.pkg_version
      );
      let rendered_crate_build_file = try!(
        self
          .render_remote_crate(&workspace_context, &package)
          .map_err(|e| CargoError::from(e.to_string()))
      );
      file_outputs.push(FileOutputs {
        path: build_file_path,
        contents: rendered_crate_build_file,
      })
    }

    let alias_file_path = format!("{}/BUILD", &path_prefix);
    let rendered_alias_build_file = try!(
      self
        .render_remote_aliases(&workspace_context, &crate_contexts)
        .map_err(|e| CargoError::from(e.to_string()))
    );
    file_outputs.push(FileOutputs {
      path: alias_file_path,
      contents: rendered_alias_build_file,
    });

    let bzl_fetch_file_path = format!("{}/crates.bzl", &path_prefix);
    let rendered_bzl_fetch_file = try!(
      self
        .render_bzl_fetch(&workspace_context, &crate_contexts)
        .map_err(|e| CargoError::from(e.to_string()))
    );
    file_outputs.push(FileOutputs {
      path: bzl_fetch_file_path,
      contents: rendered_bzl_fetch_file,
    });

    Ok(file_outputs)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use context::*;
  use hamcrest::core::expect;
  use hamcrest::prelude::*;
  use planning::PlannedBuild;
  use rendering::FileOutputs;
  use rendering::RenderDetails;

  fn dummy_render_details() -> RenderDetails {
    RenderDetails {
      path_prefix: "./some_render_prefix".to_owned(),
    }
  }

  fn dummy_planned_build(crate_contexts: Vec<CrateContext>) -> PlannedBuild {
    PlannedBuild {
      workspace_context: WorkspaceContext {
        workspace_path: "//workspace/prefix".to_owned(),
        platform_triple: "irrelevant".to_owned(),
        gen_workspace_prefix: "".to_owned(),
      },
      crate_contexts: crate_contexts,
    }
  }

  fn dummy_binary_crate() -> CrateContext {
    CrateContext {
      pkg_name: "test-binary".to_owned(),
      pkg_version: "1.1.1".to_owned(),
      features: vec!["feature1".to_owned(), "feature2".to_owned()].to_owned(),
      path: "vendor/test-binary-1.1.1/".to_owned(),
      licenses: Vec::new(),
      dependencies: Vec::new(),
      build_dependencies: Vec::new(),
      dev_dependencies: Vec::new(),
      is_root_dependency: true,
      metadeps: Vec::new(),
      platform_triple: "irrelevant".to_owned(),
      targets: vec![
        BuildTarget {
          name: "some_binary".to_owned(),
          kind: "bin".to_owned(),
          path: "bin/main.rs".to_owned(),
        },
      ],
      build_script_target: None,
      additional_deps: Vec::new(),
      additional_flags: Vec::new(),
      extra_aliased_targets: Vec::new(),
      data_attr: None,
      sha256: None,
    }
  }

  fn dummy_library_crate() -> CrateContext {
    CrateContext {
      pkg_name: "test-library".to_owned(),
      pkg_version: "1.1.1".to_owned(),
      licenses: Vec::new(),
      features: vec!["feature1".to_owned(), "feature2".to_owned()].to_owned(),
      path: "vendor/test-library-1.1.1/".to_owned(),
      dependencies: Vec::new(),
      build_dependencies: Vec::new(),
      dev_dependencies: Vec::new(),
      is_root_dependency: true,
      metadeps: Vec::new(),
      platform_triple: "irrelevant".to_owned(),
      targets: vec![
        BuildTarget {
          name: "some_library".to_owned(),
          kind: "lib".to_owned(),
          path: "path/lib.rs".to_owned(),
        },
      ],
      build_script_target: None,
      additional_deps: Vec::new(),
      additional_flags: Vec::new(),
      extra_aliased_targets: Vec::new(),
      data_attr: None,
      sha256: None,
    }
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

  fn render_crates_for_test(contexts: Vec<CrateContext>) -> Vec<FileOutputs> {
    BazelRenderer::new()
      .render_planned_build(&dummy_render_details(), &dummy_planned_build(contexts))
      .unwrap()
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
      ]).exactly()
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
    ).unwrap();
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
    ).unwrap();
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
    ).unwrap();
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
    ).unwrap();
  }
}
