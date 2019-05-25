package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "rustcpp_cmd",
    srcs = ["src/main.rs"],
    deps = [
        "//rust/hello_lib",
        "//rust/rustcpp_lib",
    ],
)