package main

import (
	"fmt"

	"github.com/sayrer/bazel-lesson-2/go/basic"
	"github.com/sayrer/bazel-lesson-2/go/cgodemo"
)

func main() {
	fmt.Printf("\nA Go string: \"%s\"\n", basic.GetString())
	fmt.Printf("\nA Go string from C++: \"%s\"\n\n", cgodemo.GetCMessage())
}