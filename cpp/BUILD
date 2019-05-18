cc_binary(
    name = "basic-app",
    srcs = ["basic_app.cpp"],
    deps = [
        ":basic",
    ],
)

cc_library(
    name = "basic",
    srcs = ["basic_library.cpp"],
    hdrs = ["basic_library.h"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "basic-c",
    srcs = ["basic_c.cpp"],
    hdrs = ["basic_c.h"],
    visibility = ["//visibility:public"],
    deps = [":basic"],
)

cc_test(
    name = "basic-test",
    srcs = ["basic_library_test.cpp"],
    copts = ["-Iexternal/gtest/include"],
    deps = [
        "@gtest//:main",
        ":basic",
    ],
)
