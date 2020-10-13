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

pub mod context;
pub mod error;
pub mod metadata;
pub mod planning {
  mod checks;
  mod crate_catalog;
  mod license;
  mod planning;
  mod subplanners;
  pub use planning::*;
}
pub mod rendering {
  pub mod bazel;
  mod rendering;
  pub use rendering::*;
}
pub mod settings;
pub mod util;

#[cfg(test)]
mod testing;
