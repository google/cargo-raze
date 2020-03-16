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

use crate::settings::CrateSettings;
use crate::util::RazeResult;
use serde_derive::Serialize;
use std::fmt;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildableDependency {
  pub name: String,
  pub version: String,
  pub buildable_target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildableTarget {
  pub name: String,
  pub kind: String,
  pub path: String,
  pub edition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Metadep {
  pub name: String,
  pub min_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct LicenseData {
  pub name: String,
  pub rating: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GitRepo {
  pub remote: String,
  pub commit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceDetails {
  pub git_data: Option<GitRepo>,
}

#[derive(Debug, Clone)]
pub struct SourceId {
  pub kind: SourceKind,
  pub url: Url,
  pub precise: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SourceKind {
    /// A git repository.
    Git(GitReference),
    /// A local path..
    Path,
    /// A remote registry.
    Registry,
    /// A local filesystem-based registry.
    LocalRegistry,
    /// A directory-based registry.
    Directory,
}

impl SourceId {
    /// Creates a `SourceId` object from the kind and URL.
    ///
    /// The canonical url will be calculated, but the precise field will not
    fn new(kind: SourceKind, url: Url) -> SourceId {
        SourceId {
            kind,
            url,
            precise: None,
        }
    }

    /// Parses a source URL and returns the corresponding ID.
    ///
    /// Copied from cargo, but Raze only uses git SourceIds.
    ///    
    /// ## Example
    ///
    /// ```
    /// use cargo_raze::context::SourceId;
    /// SourceId::from_url("git+https://github.com/alexcrichton/\
    ///                     libssh2-static-sys#80e71a3021618eb05\
    ///                     656c58fb7c5ef5f12bc747f");
    ///
    /// ```
    pub fn from_url(string: &str) -> RazeResult<SourceId> {
        let mut parts = string.splitn(2, '+');
        let kind = parts.next().unwrap();
        let url_str= parts
            .next()
            .ok_or_else(|| anyhow::format_err!("invalid source `{}`", string))?;
        let mut url = Url::parse(url_str).map_err(|s| anyhow::format_err!("invalid url `{}`: {}", url_str, s))?;

        match kind {
            "git" => {
                let mut reference = GitReference::Branch("master".to_string());
                for (k, v) in url.query_pairs() {
                    match &k[..] {
                        // Map older 'ref' to branch.
                        "branch" | "ref" => reference = GitReference::Branch(v.into_owned()),

                        "rev" => reference = GitReference::Rev(v.into_owned()),
                        "tag" => reference = GitReference::Tag(v.into_owned()),
                        _ => {}
                    }
                }
                let precise = url.fragment().map(|s| s.to_owned());
                url.set_fragment(None);
                url.set_query(None);
                let mut id = SourceId::new(SourceKind::Git(reference), url);
                id.precise = precise;
                Ok(id)
            }
            "registry" => {
                let mut id = SourceId::new(SourceKind::Registry, url);
                id.precise = Some("locked".to_string());
                Ok(id)
            }
            "path" => {
                Ok(SourceId::new(SourceKind::Path, url))
            }
            kind => Err(anyhow::format_err!("unsupported source protocol: {}", kind)),
        }
    }

    pub fn is_git(&self) -> bool {
        match self.kind {
            SourceKind::Git(_) => true,
            _ => false,
        }
    }
}

/// Information to find a specific commit in a Git repository.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GitReference {
    /// From a tag.
    Tag(String),
    /// From the HEAD of a branch.
    Branch(String),
    /// From a specific revision.
    Rev(String),
}

impl GitReference {
    /// Returns a `Display`able view of this git reference, or None if using
    /// the head of the "master" branch
    pub fn pretty_ref(&self) -> Option<PrettyRef<'_>> {
        match *self {
            GitReference::Branch(ref s) if *s == "master" => None,
            _ => Some(PrettyRef { inner: self }),
        }
    }
}

/// A git reference that can be `Display`ed
pub struct PrettyRef<'a> {
    inner: &'a GitReference,
}

impl<'a> fmt::Display for PrettyRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.inner {
            GitReference::Branch(ref b) => write!(f, "branch={}", b),
            GitReference::Tag(ref s) => write!(f, "tag={}", s),
            GitReference::Rev(ref s) => write!(f, "rev={}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: String,
  pub edition: String,
  pub raze_settings: CrateSettings,
  pub licenses: Vec<LicenseData>,
  pub features: Vec<String>,
  pub workspace_path_to_crate: String,
  pub dependencies: Vec<BuildableDependency>,
  pub build_dependencies: Vec<BuildableDependency>,
  pub dev_dependencies: Vec<BuildableDependency>,
  pub is_root_dependency: bool,
  pub targets: Vec<BuildableTarget>,
  pub build_script_target: Option<BuildableTarget>,
  pub source_details: SourceDetails,
  pub sha256: Option<String>,

  // TODO(acmcarther): This is used internally by renderer to know where to put the build file. It
  // probably should live somewhere else. Renderer params (separate from context) should live
  // somewhere more explicit.
  //
  // I'm punting on this now because this requires a more serious look at the renderer code.
  pub expected_build_path: String,

  // The name of the main lib target for this crate (if present).
  // Currently only one such lib can exist per crate.
  pub lib_target_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  // The bazel path prefix to the vendor directory
  pub workspace_path: String,

  // The compilation target triple.
  pub platform_triple: String,

  // The generated new_http_library Bazel workspace prefix.
  //
  // This has no effect unless the GenMode setting is Remote.
  pub gen_workspace_prefix: String,

  // The file extension of generated BUILD files.
  //
  // Bare files will just be named after this setting. Named files, such as those passed to
  // repository rules, will take the form of $prefix.$this_value.
  pub output_buildfile_suffix: String,
}
