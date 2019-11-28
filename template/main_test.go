package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
)

var tests = pkg.TestCases{
	{
		"Input",
		"Part1",
		"Part2",
	},
}

func TestMain(m *testing.M) {
	tests.Run(run, false)
}
