use cargo::util::CargoResult;
use planning::PlannedBuild;

#[derive(Debug, Clone)]
pub struct FileOutputs {
  pub path: String,
  pub contents: String
}

#[derive(Debug, Clone)]
pub struct RenderDetails {
  pub path_prefix: String
}

pub trait BuildRenderer {
  fn render_planned_build(&mut self, render_details: &RenderDetails, planned_build: &PlannedBuild) -> CargoResult<Vec<FileOutputs>>;
  fn render_remote_planned_build(&mut self, render_details: &RenderDetails, planned_build: &PlannedBuild) -> CargoResult<Vec<FileOutputs>>;
}
