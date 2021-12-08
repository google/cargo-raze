#!/usr/bin/env bash

# publish-new-version.sh
# EXAMPLE USAGE: publish-new-version.sh 0.1.1
#
# This script accepts a desired new crate version as an argument and updates
# the Cargo.toml version of the repository to the given version, builds the
# crate, commits the changes, publishes the crate, and then pushes the crate
# to github.
#
# The script performs some types of sanity checking, but in general you should
# read and understand this script before running it.

set -eu

declare -r MAIN_BRANCH="main"
if [[ -z "${BUILD_WORKSPACE_DIRECTORY+x}" ]]; then
  SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
  REPO_ROOT="$(dirname $SCRIPT_DIR)"
else
  REPO_ROOT="${BUILD_WORKSPACE_DIRECTORY}"
fi
readonly REPO_ROOT


command_exists() {
    command -v "$1" >/dev/null 2>&1 || ( echo "Command \`$1\` isn't available. Please install before continuing."; exit 1 )
}

update_cargo_toml_version() {
  local -r crate_version=$1
  sed -i "s/^version =.*/version = \"${crate_version}\"/g" Cargo.toml
}

cargo_build_crate() {
  # Build so that our `Cargo.lock` gets updated (before we commit).
  cargo build --release
}

git_commit_changes() {
  local -r crate_version=$1
  git add . && git commit -m "Up to ${crate_version} [via publish-new-version.sh]"
}

git_last_commit() {
  echo "$(git rev-parse --short HEAD)"
}

git_tag_commit_with_version() {
  local -r last_commit_hash=$1
  local -r crate_version=$2
  git tag "v${crate_version}" "${last_commit_hash}"
}

cargo_publish_crate() {
  cargo publish
}

git_push_changes_and_tags() {
  local -r remote_name=$1
  git push "${remote_name}" "${MAIN_BRANCH}" && git push "${remote_name}" --tags
}

usage() {
  echo "A version argument must be provided, of the form X.Y.Z."
  echo "A remote name can optionally be provided (origin is default)"
  echo "Example Usage: ./publish-new-version.sh 0.1.1 upstream"
}

main() {
  if [[ "$#" -lt 1 ]] || [[ "$#" -gt 2 ]]; then
    usage
    return
  fi
  local -r crate_version=$1
  local -r remote_name="${2:-origin}"

  if ! [[ -z "$(git status --porcelain)" ]]; then
    echo "Working directory is not clean. Commit pending changes before running command."
    exit 1
  fi

  command_exists "cargo"
  update_cargo_toml_version "${crate_version}"
  cargo_build_crate
  git_commit_changes "${crate_version}"
  last_commit_hash=$(git_last_commit)
  git_tag_commit_with_version "${last_commit_hash}" "${crate_version}"
  cargo_publish_crate
  git_push_changes_and_tags "${remote_name}"
}

pushd "${REPO_ROOT}/impl"
set -x
main "$@"
popd
