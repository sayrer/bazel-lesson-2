package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "rustcpp_lib",
    srcs = [
        "src/infoprinter.rs",
        "src/lib.rs",
    ],
    deps = [
        "//cpp:basic-c",
        "@libc",
    ],
)