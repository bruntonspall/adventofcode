package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
)

var tests = pkg.TestCases{
	{
		"1,9,10,3,2,3,11,0,99,30,40,50, 1, 1, 1, 1",
		"100",
		"",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
