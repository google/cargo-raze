# Regenerating BUILD files

raze-generated BUILD files are present in several places which must be manually
regenerated to test changes. Commands to update these:
```console
$ bazel run //tools:examples_check
$ bazel run //:raze -- --manifest-path $(readlink -f impl/Cargo.toml)
```
