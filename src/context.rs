#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildDependency {
  pub name: String,
  pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildTarget {
  pub name: String,
  pub kind: String,
  pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Metadep {
  pub name: String,
  pub min_version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: String,
  pub features: Vec<String>,
  pub path: String,
  pub dependencies: Vec<BuildDependency>,
  pub build_dependencies: Vec<BuildDependency>,
  pub dev_dependencies: Vec<BuildDependency>,
  pub is_root_dependency: bool,
  pub metadeps: Vec<Metadep>,
  pub platform_triple: String,
  pub targets: Vec<BuildTarget>,
  pub build_script_target: Option<BuildTarget>,
  pub additional_deps: Vec<String>,
  pub additional_flags: Vec<String>,
  pub extra_aliased_targets: Vec<String>,
  // TODO(acmcarther): Consider plugin topic
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  /** The bazel path prefix to the vendor directory */
  pub workspace_path: String,

  /** The compilation target triple */
  pub platform_triple: String,

  /**
   * The generated new_http_library Bazel workspace prefix.
   *
   * This has no effect unless the GenMode setting is Remote.
   */
  pub gen_workspace_prefix: String
}
