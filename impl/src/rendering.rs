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

use crate::planning::PlannedBuild;
use cargo::util::CargoResult;

/**
 * An object that can convert a prepared build plan into a series of files for a Bazel-like build
 * system.
 */
pub trait BuildRenderer {
  fn render_planned_build(
    &mut self,
    render_details: &RenderDetails,
    planned_build: &PlannedBuild,
  ) -> CargoResult<Vec<FileOutputs>>;
  fn render_remote_planned_build(
    &mut self,
    render_details: &RenderDetails,
    planned_build: &PlannedBuild,
  ) -> CargoResult<Vec<FileOutputs>>;
}

#[derive(Debug, Clone)]
pub struct FileOutputs {
  pub path: String,
  pub contents: String,
}

#[derive(Debug, Clone)]
pub struct RenderDetails {
  pub path_prefix: String,
  pub buildfile_suffix: String,
}
