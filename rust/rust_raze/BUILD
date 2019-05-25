load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "rust_raze",
    srcs = ["src/main.rs"],
    deps = [
        "//rust/rust_raze/cargo:num_cpus",
        "//rust/hello_lib",
        "//rust/rustcpp_lib",
    ],
)