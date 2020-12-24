"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of normal dependencies for the Rust targets of that package.
_DEPENDENCIES = {
    "vendored/cargo_workspace/num_printer": {
        "clap": "//vendored/cargo_workspace/cargo/vendor/clap-2.33.3:clap",
    },
    "vendored/cargo_workspace/printer": {
        "ferris-says": "//vendored/cargo_workspace/cargo/vendor/ferris-says-0.2.0:ferris_says",
    },
    "vendored/cargo_workspace/rng": {
        "rand": "//vendored/cargo_workspace/cargo/vendor/rand-0.7.3:rand",
    },
}

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of proc_macro dependencies for the Rust targets of that package.
_PROC_MACRO_DEPENDENCIES = {
    "vendored/cargo_workspace/num_printer": {
    },
    "vendored/cargo_workspace/printer": {
    },
    "vendored/cargo_workspace/rng": {
    },
}

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of normal dev dependencies for the Rust targets of that package.
_DEV_DEPENDENCIES = {
    "vendored/cargo_workspace/num_printer": {
    },
    "vendored/cargo_workspace/printer": {
    },
    "vendored/cargo_workspace/rng": {
    },
}

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of proc_macro dev dependencies for the Rust targets of that package.
_DEV_PROC_MACRO_DEPENDENCIES = {
    "vendored/cargo_workspace/num_printer": {
    },
    "vendored/cargo_workspace/printer": {
    },
    "vendored/cargo_workspace/rng": {
    },
}

def crates(deps, package_name = None):
    """EXPERIMENTAL -- MAY CHANGE AT ANY TIME: Finds the fully qualified label of the requested crates for the package where this macro is called.

    WARNING: This macro is part of an expeirmental API and is subject to change.

    Args:
        deps (list): The desired list of crate targets.
        package_name (str, optional): The package name of the set of dependencies to look up.
            Defaults to `native.package_name()`.
    Returns:
        list: A list of labels to cargo-raze generated targets (str)
    """

    if not package_name:
        package_name = native.package_name()

    # Join both sets of dependencies
    dependencies = dict()
    for dep_map in [_DEPENDENCIES, _PROC_MACRO_DEPENDENCIES, _DEV_DEPENDENCIES, _DEV_PROC_MACRO_DEPENDENCIES]:
        for pkg_name in _DEPENDENCIES:
            if pkg_name in dependencies:
                dependencies[pkg_name].extend(dep_map[pkg_name])
            else:
                dependencies[pkg_name].update(dep_map[pkg_name])

    if not deps:
        fail("An invalid argument has been provided. Please pass a crate name or a list of crate names")

    if not dependencies:
        return []

    if type(deps) == "string":
        deps = [deps]

    errors = []
    crates = []
    for crate in deps:
        if crate not in dependencies[package_name]:
            errors.append(crate)
        else:
            crates.append(dependencies[package_name][crate])

    if errors:
        fail("Missing crates `{}` for package `{}`. Available crates `{}".format(
            errors,
            package_name,
            dependencies[package_name],
        ))

    return crates

def all_crates(normal = False, normal_dev = False, proc_macro = False, proc_macro_dev = False, package_name = None):
    """EXPERIMENTAL -- MAY CHANGE AT ANY TIME: Finds the fully qualified label of all requested direct crate dependencies \
    for the package where this macro is called.

    If no parameters are set, all normal dependencies are returned. Setting any one flag will
    otherwise impact the contents of the returned list.

    Args:
        normal (bool, optional): If True, normal dependencies are included in the
            output list. Defaults to False.
        normal_dev (bool, optional): If True, normla dev dependencies will be
            included in the output list. Defaults to False.
        proc_macro (bool, optional): If True, proc_macro dependencies are included
            in the output list. Defaults to False.
        proc_macro_dev (bool, optional): If True, dev proc_macro dependencies are
            included in the output list. Defaults to False.
        package_name (str, optional): The package name of the set of dependencies to look up.
            Defaults to `native.package_name()`.

    Returns:
        list: A list of labels to cargo-raze generated targets (str)
    """

    if not package_name:
        package_name = native.package_name()

    # Determine the relevant maps to use
    all_dependency_maps = []
    if normal:
        all_dependency_maps.append(_DEPENDENCIES)
    if normal_dev:
        all_dependency_maps.append(_DEV_DEPENDENCIES)
    if proc_macro:
        all_dependency_maps.append(_PROC_MACRO_DEPENDENCIES)
    if proc_macro:
        all_dependency_maps.append(_DEV_PROC_MACRO_DEPENDENCIES)

    # Default to always using normal dependencies
    if not all_dependency_maps:
        all_dependency_maps.append(_DEPENDENCIES)

    dependencies = dict()
    for dep_map in all_dependency_maps:
        for pkg_name in dep_map:
            if pkg_name in dependencies:
                dependencies[pkg_name].extend(dep_map[pkg_name])
            else:
                dependencies[pkg_name] = dep_map[pkg_name]

    if not dependencies:
        return []

    return dependencies[package_name].values()
