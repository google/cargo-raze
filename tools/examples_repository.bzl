"""A module defining a repository rule that ensures the vendored examples have 
actual vendored sources"""

load("@io_bazel_rules_rust//rust:repositories.bzl", "load_arbitrary_tool")

def _examples_dir(repository_ctx):
    """Returns the path to the cargo-raze workspace root

    Args:
        repository_ctx (repository_ctx): The current rule's context object

    Returns:
        path: A path to the cargo-raze workspace root
    """
    workspace_root = repository_ctx.path(repository_ctx.attr._script).dirname.dirname
    return repository_ctx.path(str(workspace_root) + "/" + repository_ctx.attr._examples_dir.lstrip("/"))

EXECUTE_FAIL_MESSAGE = """\
Failed to setup examples repository with exit code ({}).
--------stdout:
{}
--------stderr:
{}
"""

def _examples_repository_impl(repository_ctx):
    """Implementation of the `examples_repository` repository rule

    Args:
        repository_ctx (repository_ctx): The current rule's context object
    """

    examples_dir = _examples_dir(repository_ctx)
    vendor_script = repository_ctx.attr._script

    if repository_ctx.attr.target_triple:
        target_triple = repository_ctx.attr.target_triple
    elif "mac" in repository_ctx.os.name:
        target_triple = "x86_64-apple-darwin"
    elif "windows" in repository_ctx.os.name:
        target_triple = "x86_64-pc-windows-msvc"
    else:
        target_triple = "x86_64-unknown-linux-gnu"

    # Download cargo
    load_arbitrary_tool(
        ctx = repository_ctx,
        tool_name = "cargo",
        tool_subdirectories = ["cargo"],
        version = repository_ctx.attr.cargo_version,
        iso_date = None,
        target_triple = target_triple,
    )

    # Add example contents
    for item in examples_dir.readdir():
        # Skip bazel output symlinks
        if item.basename.startswith("bazel-"):
            continue
        repository_ctx.symlink(item, item.basename)

    # Vendor sources
    results = repository_ctx.execute(
        [
            vendor_script,
            "vendor",
        ],
        quiet = True,
        environment = {
            "EXAMPLES_DIR": str(examples_dir),
            "CARGO": "{}/bin/cargo".format(repository_ctx.path(".")),
        },
    )

    if results.return_code != 0:
        fail(EXECUTE_FAIL_MESSAGE.format(
            results.return_code,
            results.stdout,
            results.stderr,
        ))

_examples_repository = repository_rule(
    implementation = _examples_repository_impl,
    doc = "A rule for guaranteeing the Vendored examples have vendored source",
    attrs = {
        "cargo_version": attr.string(
            doc = "The version of cargo to use",
            default = "1.49.0",
        ),
        "target_triple": attr.string(
            doc = "The target triple of the cargo binary to download",
        ),
        "_examples_dir": attr.string(
            doc = "The path to the examples directory relative to `_root_file",
            default = "examples",
        ),
        "_script": attr.label(
            doc = "A script containing the ability to vendor crates into examples",
            default = Label("@cargo_raze//tools:examples_repository_tools.sh"),
            allow_single_file = True,
        ),
    },
)

def examples_repository():
    """Defines the examples repository"""

    _examples_repository(
        name = "cargo_raze_examples",
    )
