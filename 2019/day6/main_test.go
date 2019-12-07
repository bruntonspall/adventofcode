package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

var tests = pkg.TestCases{
	{
		"AAA)BBB",
		"1",
		"",
	},
	{
		`C)B
B)A`,
		"3",
		"",
	},
	{
		`COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L`,
		"42",
		"",
	},
}

func TestAncestors(t *testing.T) {
	cases := []struct {
		in  string
		out []string
	}{
		{"C", []string{"COM", "B", "C"}},
		{"G", []string{"COM", "B", "G"}},
		{"K", []string{"COM", "B", "C", "D", "E", "J", "K"}},
	}
	orbits := makeTree(tests[2].Input)

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := ancestors(orbits, tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestFindAncestor(t *testing.T) {
	cases := []struct {
		in  string
		out string
	}{
		{"H", "G"},
		{"K", "J"},
		{"C", "B"},
		{"B", "COM"},
	}
	orbits := makeTree(tests[2].Input)

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := findAncestor(orbits, tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestCountDistance(t *testing.T) {
	cases := []struct {
		a   string
		b   string
		out int
	}{
		{"B", "COM", 1},
		{"COM", "B", 1},
		{"K", "J", 1},
		{"B", "F", 4},
		{"H", "C", 3},
		{"I", "K", 4},
	}
	orbits := makeTree(tests[2].Input)

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v%v", tt.a, tt.b), func(t *testing.T) {
			s := countDistance(orbits, tt.a, tt.b)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
