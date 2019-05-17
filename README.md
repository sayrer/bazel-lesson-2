# Bazel Lesson 2 (incomplete WIP)

If you haven't read [Bazel Lesson 1](https://github.com/sayrer/bazel-lesson-1) yet, go do that now. It's not a very long read, and skipping it would only be cheating yourself.

Lesson 1 might have seemed like a bit of a parlor trick, where one build system just happened to be able to build both C++ and Java, and then combine them with JNI. Lesson 2 will show that this is a general pattern in Bazel, and not a narrowly-defined capability.

To recap, Lesson 1 built a number of C++ and Java tests and binaries. It then showed that only the build products using a basic C++ static library had precise dependencies on it. For example, if the static library were changed, only tests that actually depended on it would need to be rerun. This feature is quite a bit more powerful than incremental compilation as seen in many compiler toolchains--it is cross-language and reflects test results and dependencies. Here's an illustration that shows which build products actually depend on [Lesson 1](https://github.com/sayrer/bazel-lesson-1)'s basic C++ static library:

![JNI dependency graph](./jni_graphic.png)

Lesson 1 didn't go into the contents of [its WORKSPACE](https://github.com/sayrer/bazel-lesson-1/blob/master/WORKSPACE), and there really isn't much in there. All it does is load one function, `http_archive`, which gives Bazel the ability to download files over http. The `http_archive` rule has smarts to decompress `.zip`, `.tar.gz`, `.bzip2`, etc, and then use the supplied `BUILD` file to create a build product from its decompressed contents. In this case, `gtest.BUILD` builds the GoogleTest C++ library, and it's used in the C++ tests.

One flaw in [Lesson 1](https://github.com/sayrer/bazel-lesson-1)'s setup is that it depends on the C++ and Java toolchain installed on the host machine. This could lead to situations that are colloquially known as "works on my machine", where a project has dependencies not reflected in its build system (fortunate developers might find them written down in a wiki or something).

# C++ and Java toolchains

On macOS, it's difficult to use a custom C/C++ toolchain, unless you're building something equivalent to a Unix command line tool or targeting a non-Apple device. Although it's possible to use a custom compiler and linker, macOS and iOS apps will still depend on the SDKs installed on the build machine. On Linux, it's possible to supply everything a build depends on via a [sysroot](https://stackoverflow.com/questions/39920712/what-is-a-sysroot-exactly-and-how-do-i-create-one). Another way to sidestep these issues is to build inside a Docker container where possible. This method can be a lot easier at first, but it tends to accumulate cruft in Dockerfiles that's hard to unwind.

Bazel ships with a minimal JDK used to run Bazel itself, and then downloads a full JDK if you actually build JVM programs. You can control this with the [java_toolchain](https://docs.bazel.build/versions/master/be/java.html#java_toolchain) rule and arguments. As an example, the Gerrit project has a fairly detailed [Bazel setup](https://gerrit.googlesource.com/gerrit/+/master/Documentation/dev-bazel.txt). Bazel doesn't yet seem to be fully free of dependencies on the local JDK, or it is at least buggy when macOS contains a stub `javac` command (the one that pops up a Java installation dialog).

Most projects can probably sidestep these issues by standardizing on a Docker image for Linux builds that contains a JDK. macOS workstations will need to require a specific XCode and JDK version. Then, absolutely everything else should be built by Bazel if possible. If you run a [caching build server](https://docs.bazel.build/versions/master/remote-caching.html), you can check its hit rate to take a pulse on these issues.

# Installing support for other languages

Lesson 2 adds support for [Go](https://golang.org/) and [Rust](https://www.rust-lang.org/). Here's the section of `WORKSPACE` that loads the Go toolchain:

```
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
```

For Go, we'll first build a project with hand-written Bazel rules.

```
$ more go/basic/BUILD 
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
```

There's nothing too surprising here in comparison to the C++ and Java projects. At the top of the file, the load function makes the Go extensions available to Bazel. In the `go_library` rule, there's an argument called `importpath` that allows other Go files to load the library with Go's URL-inspired import syntax.

There's also add a simple command line app in a subpackage:

```
$ more go/basic/cmd/BUILD 
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
```
It can be run with the same Bazel command as the C++ and Java apps. This `BUILD` file is a bit verbose, and could just be one `go_binary` rule without a library dependency. However, this is the layout generated by the `gazelle` tool, covered below.

```
$ bazel run //go/basic/cmd:command
INFO: Analysed target //go/basic/cmd:command (0 packages loaded, 0 targets configured).
INFO: Found 1 target...
Target //go/basic/cmd:command up-to-date:
  bazel-bin/go/basic/cmd/darwin_amd64_stripped/command
INFO: Elapsed time: 0.185s, Critical Path: 0.00s
INFO: 0 processes.
INFO: Build completed successfully, 1 total action
INFO: Build completed successfully, 1 total action

A Go string: "Hello from Go"

```

The test runs the same way as well (note the caching in the second pass):

```
$ bazel test //go/basic/...
INFO: Analysed 3 targets (0 packages loaded, 0 targets configured).
INFO: Found 2 targets and 1 test target...
INFO: Elapsed time: 0.899s, Critical Path: 0.68s
INFO: 6 processes: 6 darwin-sandbox.
INFO: Build completed successfully, 6 total actions
//go/basic:go_default_test                                               PASSED in 0.1s

Executed 1 out of 1 test: 1 test passes.
INFO: Build completed successfully, 6 total actions

$ bazel test //go/basic/...
INFO: Analysed 3 targets (0 packages loaded, 0 targets configured).
INFO: Found 2 targets and 1 test target...
INFO: Elapsed time: 0.198s, Critical Path: 0.01s
INFO: 0 processes.
INFO: Build completed successfully, 1 total action
//go/basic:go_default_test                                      (cached) PASSED in 0.1s

Executed 0 out of 1 test: 1 test passes.
INFO: Build completed successfully, 1 total action
```
## Building conventional Go packages 

In `go/nonbazel`, there's a package that was initially created with the standard Go process, [go mod init](https://github.com/golang/go/wiki/Modules#quick-start). This package directly depends on the `rsc.io/quote` package, and transitively depends on `rsc.io/sampler` and `golang.org/x/text`.

To adapt this project to Bazel, the `gazelle` converter program is used. It's downloaded in `WORKSPACE`:

```
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
```

and made into an invokable target in a `BUILD` file at the root of the project:

```
load("@bazel_gazelle//:def.bzl", "gazelle")

# gazelle:prefix github.com/sayrer/bazel-lesson-2
gazelle(name = "gazelle")
```

Using this target, it's possible to generate a `BUILD.bazel` file for the non-Bazel Go package in go/nonbazel.

```
$ bazel run //:gazelle
$ more go/nonbazel/BUILD.bazel 
load("@io_bazel_rules_go//go:def.bzl", "go_library")

go_library(
    name = "go_default_library",
    srcs = ["nonbazel.go"],
    importpath = "github.com/sayrer/bazel-lesson-2/go/nonbazel",
    visibility = ["//visibility:public"],
    deps = ["@io_rsc_quote//:go_default_library"],
)
```

This `BUILD` file is valid, but the `@io_rsc_quote` dependency needs to be added to the `WORKSPACE`. Running the command

```
$ bazel run //:gazelle -- update-repos -from_file=go/nonbazel/go.mod
```

will add the required dependencies to `WORKSPACE`:

```
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
```

## Combining C++ and Go with cgo

In the `go/cgodemo` directory, there's a Bazel library that combines the C++ library in `cpp` with the Go library in `go/basic`.

```
$ more go/cgodemo/BUILD 
load("@io_bazel_rules_go//go:def.bzl", "go_library")

go_library(
    name = "go_default_library",
    srcs = [
        "cgodemo.go",
        "cpp-basic.h",
        "cpp-basic.cpp"
    ],
    cgo = True,
    importpath = "github.com/sayrer/bazel-lesson-2/go/cgodemo",
    visibility = ["//visibility:public"],
    cdeps = ["//cpp:basic"],
)
```

There's a go file along with a small wrapper around the library C++ library. The library also has `cgo = True` set to get the right linker flags passed to the build command. There's a simple command line app in the `go/cgodemo/cmd` directory.

```
$ bazel run //go/cgodemo/cmd:command
INFO: Analysed target //go/cgodemo/cmd:command (0 packages loaded, 0 targets configured).
INFO: Found 1 target...
Target //go/cgodemo/cmd:command up-to-date:
  bazel-bin/go/cgodemo/cmd/darwin_amd64_stripped/command
INFO: Elapsed time: 0.257s, Critical Path: 0.00s
INFO: 0 processes.
INFO: Build completed successfully, 1 total action
INFO: Build completed successfully, 1 total action

A Go string: "Hello from Go"

A Go string from C++: "I'm a C++ string!"
```
## Rust

This is the section of the `WORKSPACE` file that downloads the dependencies needed for the Rust rules. Skylib is a library of utilities for writing [Starlark](https://github.com/bazelbuild/starlark) (formerly known as Skylark), Bazel's extension and configuration language.

```
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
```

In `rust/hello_lib`, there's a Rust library and a variety of build styles and tests.

```
$ more rust/hello_lib/BUILD 
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

rust_library(
    name = "hello_lib",
    srcs = [
        "src/greeter.rs",
        "src/lib.rs",
    ],
    rustc_flags = ["--cap-lints=allow"],
    crate_features = ["default"],
)

rust_library(
    name = "hello_dylib",
    srcs = [
        "src/greeter.rs",
        "src/lib.rs",
    ],
    crate_type = "dylib",
)

rust_library(
    name = "hello_cdylib",
    srcs = [
        "src/greeter.rs",
        "src/lib.rs",
    ],
    crate_type = "cdylib",
)

rust_library(
    name = "hello_staticlib",
    srcs = [
        "src/greeter.rs",
        "src/lib.rs",
    ],
    crate_type = "staticlib",
)

rust_test(
    name = "hello_lib_test",
    crate = ":hello_lib",
)

rust_test(
    name = "greeting_test",
    srcs = ["tests/greeting.rs"],
    deps = [":hello_lib"],
)

rust_doc(
    name = "hello_lib_doc",
    dep = ":hello_lib",
)

rust_doc_test(
    name = "hello_lib_doc_test",
    dep = ":hello_lib",
)
```

There's a Rust binary in the `rust/hello_world` directory:

```
$ bazel run //rust/hello_world:hello_world
INFO: Analyzed target //rust/hello_world:hello_world (0 packages loaded, 0 targets configured).
INFO: Found 1 target...
Target //rust/hello_world:hello_world up-to-date:
  bazel-bin/rust/hello_world/hello_world
INFO: Elapsed time: 0.432s, Critical Path: 0.27s
INFO: 1 process: 1 darwin-sandbox.
INFO: Build completed successfully, 2 total actions
INFO: Build completed successfully, 2 total actions

Hello from Rust!

```

