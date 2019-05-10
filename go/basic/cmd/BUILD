load("@io_bazel_rules_go//go:def.bzl", "go_binary", "go_library")

go_binary(
    name = "command",
    embed = [":go_default_library"],
    visibility = ["//visibility:public"],
)

go_library(
    name = "go_default_library",
    srcs = ["main.go"],
    importpath = "github.com/sayrer/bazel-lesson-2/go/basic/cmd",
    visibility = ["//visibility:private"],
    deps = ["//go/basic:go_default_library"],
)
