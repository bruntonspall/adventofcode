package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
)

var tests = pkg.TestCases{
	{
		"12",
		"2",
		"2",
	},
	{
		"14",
		"2",
		"2",
	},
	{
		"1969",
		"654",
		"966",
	},
	{
		"100756",
		"33583",
		"50346",
	},
	{
		`12
14`,
		"4",
		"4",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
