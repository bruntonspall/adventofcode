package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
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
		`<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>`,
		"14645",
		"4686774924",
	},
	{
		puzzle,
		"5517",
		"303070460651184",
	},
}

func TestGCD(t *testing.T) {
	cases := []struct {
		a   int
		b   int
		gcd int
	}{
		{54, 24, 6},
		{24, 54, 6},
		{48, 180, 12},
		{180, 48, 12},
	}

	for _, tt := range cases {
		gcd := GCD(tt.a, tt.b)
		if !cmp.Equal(gcd, tt.gcd) {
			t.Errorf("got %v, want %v", gcd, tt.gcd)
		}
	}

}
func TestLCM(t *testing.T) {
	cases := []struct {
		a   int
		b   int
		lcm int
	}{
		{2, 3, 6},
		{5, 3, 15},
		{15, 9, 45},
	}

	for _, tt := range cases {
		lcm := LCM(tt.a, tt.b)
		if !cmp.Equal(lcm, tt.lcm) {
			t.Errorf("got %v, want %v", lcm, tt.lcm)
		}
	}

}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
