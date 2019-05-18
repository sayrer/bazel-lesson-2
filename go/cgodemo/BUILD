load("@io_bazel_rules_go//go:def.bzl", "go_library")

go_library(
    name = "go_default_library",
    srcs = ["cgodemo.go"],
    cgo = True,
    importpath = "github.com/sayrer/bazel-lesson-2/go/cgodemo",
    visibility = ["//visibility:public"],
    cdeps = ["//cpp:basic-c"],
)
