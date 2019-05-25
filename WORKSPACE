#
# Bazel infrastructure
#
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")

#
# Go Rules
#
http_archive(
    name = "io_bazel_rules_go",
    url = "https://github.com/bazelbuild/rules_go/releases/download/0.18.3/rules_go-0.18.3.tar.gz",
    sha256 = "86ae934bd4c43b99893fc64be9d9fc684b81461581df7ea8fc291c816f5ee8c5",
)

load("@io_bazel_rules_go//go:deps.bzl", "go_rules_dependencies", "go_register_toolchains")

go_rules_dependencies()

go_register_toolchains()

#
# Gazelle (creates Bazel rules from standard Go builds)
#
http_archive(
    name = "bazel_gazelle",
    urls = ["https://github.com/bazelbuild/bazel-gazelle/releases/download/0.17.0/bazel-gazelle-0.17.0.tar.gz"],
    sha256 = "3c681998538231a2d24d0c07ed5a7658cb72bfb5fd4bf9911157c0e9ac6a2687",
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")

gazelle_dependencies()

#
# Go libraries
#
go_repository(
    name = "io_rsc_quote",
    importpath = "rsc.io/quote",
    tag = "v1.5.2",
)

go_repository(
    name = "io_rsc_sampler",
    importpath = "rsc.io/sampler",
    tag = "v1.3.0",
)

go_repository(
    name = "org_golang_x_text",
    commit = "14c0d48ead0c",
    importpath = "golang.org/x/text",
)

#
# Skylib (Rust rules depend on this)
#
http_archive(
    name = "bazel_skylib",
    sha256 = "2c62d8cd4ab1e65c08647eb4afe38f51591f43f7f0885e7769832fa137633dcb",
    strip_prefix = "bazel-skylib-0.7.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/0.7.0.tar.gz",
)

#
# Rust Rules
#
git_repository(
    name = "io_bazel_rules_rust",
    commit = "d28b121396974a628b9cdb29b6ed7f4e370edb4e",
    remote = "https://github.com/bazelbuild/rules_rust",
    shallow_since = "1557167838 -0400",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")

#
# FFI for Rust
#
new_git_repository(
    name = "libc",
    build_file = "libc.BUILD",
    remote = "https://github.com/rust-lang/libc",
    commit = "6ec4f81a3852797410b80296d3afd61f2b255a36",
    shallow_since = "1484672371 +0000"
)

#
# Testing for C++
#
http_archive(
    name = "gtest",
    url = "https://github.com/google/googletest/archive/release-1.7.0.zip",
    sha256 = "b58cb7547a28b2c718d1e38aee18a3659c9e3ff52440297e965f5edffe34b6d0",
    build_file = "gtest.BUILD",
    strip_prefix = "googletest-release-1.7.0",
)
