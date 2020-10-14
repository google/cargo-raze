// Copyright 2020 Google Inc.
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

use std::fmt;

pub const PLEASE_FILE_A_BUG: &str =
"Please file an issue at github.com/google/cargo-raze with details.";

#[derive(Debug)]
pub enum RazeError {
Generic(String),
Internal(String),
Rendering {
  crate_name_opt: Option<String>,
  message: String,
},
Planning {
  dependency_name_opt: Option<String>,
  message: String,
},
Config {
  field_path_opt: Option<String>,
  message: String,
},
}

impl std::error::Error for RazeError {}

impl fmt::Display for RazeError {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  match &self {
    Self::Generic(s) => write!(f, "Raze failed with cause: \"{}\"", s),
    Self::Internal(s) => write!(
      f,
      "Raze failed unexpectedly with cause: \"{}\". {}",
      s, PLEASE_FILE_A_BUG
    ),
    Self::Config {
      field_path_opt,
      message,
    } => match field_path_opt {
      Some(path) => write!(
        f,
        "Raze config problem in field \"{}\" with cause: \"{}\"",
        path, message
      ),
      None => write!(f, "Raze config problem with cause: \"{}\"", message),
    },
    Self::Rendering {
      crate_name_opt,
      message,
    } => match crate_name_opt {
      Some(name) => write!(
        f,
        "Raze failed to render crate \"{}\" with cause: \"{}\"",
        name, message
      ),
      None => write!(f, "Raze failed to render with cause: \"{}\"", message),
    },
    Self::Planning {
      dependency_name_opt,
      message,
    } => match dependency_name_opt {
      Some(dep_name) => write!(
        f,
        "Raze failed to plan crate \"{}\" with cause: \"{}\"",
        dep_name, message
      ),
      None => write!(f, "Raze failed to render with cause: \"{}\"", message),
    },
  }
}
}