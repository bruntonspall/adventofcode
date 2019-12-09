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

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
