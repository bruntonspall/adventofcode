package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
)

var tests = pkg.TestCases{
	{
		"-6\n+3\n+8\n+5\n-6",
		`4`,
		`5`,
	},
	{
		"+3\n+3\n+4\n-2\n-4",
		`4`,
		`10`,
	},
}

func TestMain(m *testing.M) {
	tests.Run(run, false)
}
