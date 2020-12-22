"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

# EXPERIMENTAL: A mapping of package names to a set of normal dependencies for the Rust targets of that package.
_DEPENDENCIES = {
    "vendored/non_cratesio_library": {
        "env_logger": "//vendored/non_cratesio_library/cargo/vendor/env_logger-0.5.5:env_logger",
        "futures": "//vendored/non_cratesio_library/cargo/vendor/futures-0.2.0:futures",
        "log": "//vendored/non_cratesio_library/cargo/vendor/log-0.4.0:log",
    },
}

# EXPERIMENTAL: A mapping of package names to a set of proc_macro dependencies for the Rust targets of that package.
_PROC_MACRO_DEPENDENCIES = {
    "vendored/non_cratesio_library": {
    },
}

# EXPERIMENTAL: A mapping of package names to a set of normal dev dependencies for the Rust targets of that package.
_DEV_DEPENDENCIES = {
    "vendored/non_cratesio_library": {
    },
}

# EXPERIMENTAL: A mapping of package names to a set of proc_macro dev dependencies for the Rust targets of that package.
_DEV_PROC_MACRO_DEPENDENCIES = {
    "vendored/non_cratesio_library": {
    },
}

def crates(deps):
    """EXPERIMENTAL: Finds the fully qualified label of the requested crates for the package where this macro is called.

    WARNING: This macro is part of an expeirmental API and is subject to change.

    Args:
        deps (list or str): Either a list of dependencies or a string of one which will
            be converted into a list.
    Returns:
        list: A list of labels to cargo-raze generated targets (str)
    """

    # Join both sets of dependencies
    dependencies = dict()
    for dep_map in [_DEPENDENCIES, _PROC_MACRO_DEPENDENCIES, _DEV_DEPENDENCIES, _DEV_PROC_MACRO_DEPENDENCIES]:
        for package_name in _DEPENDENCIES:
            if package_name in dependencies:
                dependencies[package_name].extend(dep_map[package_name])
            else:
                dependencies[package_name].update(dep_map[package_name])

    if not deps:
        fail("An invalid argument has been provided. Please pass a crate name or a list of crate names")

    if not dependencies:
        return []

    if type(deps) == "string":
        deps = [deps]

    errors = []
    crates = []
    for crate in deps:
        if crate not in dependencies[native.package_name()]:
            errors.append(crate)
        else:
            crates.append(dependencies[native.package_name()][crate])

    if errors:
        fail("Missing crates `{}` for package `{}`. Available crates `{}".format(
            errors,
            native.package_name(),
            dependencies[native.package_name()],
        ))

    return crates

def all_crates(normal = False, proc_macro = False, dev = False, dev_only = False):
    """EXPERIMENTAL: Finds the fully qualified label of all requested direct crate dependencies \
    for the package where this macro is called.

    If no parameters are set, all normal and proc_macro dependencies are returned.
    Setting any one flag will otherwise impact the contents of the returned list

    WARNING: This macro is part of an expeirmental API and is subject to change.

    Args:
        normal (bool, optional): If True, normal dependencies are included in the
            output list. Defaults to False.
        proc_macro (bool, optional): If True, proc_macro dependencies will be
            included in the output list. Defaults to False.
        dev (bool, optional): If True, dev dependencies are included when the
            `normal` and `proc_macro` parameters are used. Defaults to False.
        dev_only (bool, optional): If True, only development dependencies will be
            returned by this list. This paramter otherwise follows the same rules
            as `dev`. Defaults to False.

    Returns:
        list: A list of labels to cargo-raze generated targets (str)
    """

    # Determine the relevant maps to use
    dependencies = dict()
    all_maps = []
    if normal:
        if not dev_only:
            all_maps.append(_DEPENDENCIES)
        if dev or dev_only:
            all_maps.append(_DEV_DEPENDENCIES)
    if proc_macro:
        if not dev_only:
            all_maps.append(_PROC_MACRO_DEPENDENCIES)
        if dev or dev_only:
            all_maps.append(_DEV_PROC_MACRO_DEPENDENCIES)

    # Default to always using normal dependencies
    if not all_maps:
        if not dev_only:
            all_maps.append(_DEPENDENCIES)
        if dev or dev_only:
            all_maps.append(_DEV_DEPENDENCIES)

    if not all_maps:
        fail("Failed to add at least 1 map to the `all_maps` list with parameters: " +
             "normal = {normal}, proc_macro = {proc_macro}, dev = {dev}, dev_only = {dev_only}".format(
                 normal = normal,
                 proc_macro = proc_macro,
                 dev = dev,
                 dev_only = dev_only,
             ))

    for dep_map in all_maps:
        for package_name in dep_map:
            if package_name in dependencies:
                dependencies[package_name].extend(dep_map[package_name])
            else:
                dependencies[package_name] = dep_map[package_name]

    if not dependencies:
        return []

    return dependencies[native.package_name()].values()
