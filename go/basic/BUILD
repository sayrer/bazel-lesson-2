load("@io_bazel_rules_go//go:def.bzl", "go_library", "go_test")

go_library(
    name = "go_default_library",
    srcs = ["basic.go"],
    importpath = "github.com/sayrer/bazel-lesson-2/go/basic",
    visibility = ["//visibility:public"],
)

go_test(
    name = "go_default_test",
    srcs = ["basic_test.go"],
    embed = [":go_default_library"],
)
