package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
)

var tests = pkg.TestCases{
	{
		`<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>`,
		"183",
		"2772",
	},
	{
		puzzle,
		"5517",
		"Part2",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
