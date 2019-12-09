package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
)

var tests = pkg.TestCases{
	{
		puzzle,
		"2789104029",
		"32869",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
