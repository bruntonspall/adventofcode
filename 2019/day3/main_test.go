package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

var tests = pkg.TestCases{
	{
		`R8,U5,L5,D3
U7,R6,D4,L4`,
		"6",
		"",
	},
	{
		`R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83`,
		"159",
		"610",
	},
	{
		`R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7`,
		"135",
		"410",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}

func TestParsePath(t *testing.T) {
	cases := []struct {
		in  string
		out []Vector
	}{
		{"R8,U5,L5,D3", []Vector{Vector{East, 8}, Vector{North, 5}, Vector{West, 5}, Vector{South, 3}}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := ParsePath(tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestPathToMap(t *testing.T) {
	cases := []struct {
		in  []Vector
		out map[Coord]int
	}{
		{[]Vector{Vector{North, 1}}, map[Coord]int{Coord{0, 1}: 1}},
		{[]Vector{
			Vector{East, 8}, Vector{North, 5}, Vector{West, 5}, Vector{South, 3}},
			map[Coord]int{
				Coord{1, 0}: 1,
				Coord{2, 0}: 1,
				Coord{3, 0}: 1,
				Coord{4, 0}: 1,
				Coord{5, 0}: 1,
				Coord{6, 0}: 1,
				Coord{7, 0}: 1,
				Coord{8, 0}: 1,
				Coord{8, 1}: 1,
				Coord{8, 2}: 1,
				Coord{8, 3}: 1,
				Coord{8, 4}: 1,
				Coord{8, 5}: 1,
				Coord{7, 5}: 1,
				Coord{6, 5}: 1,
				Coord{5, 5}: 1,
				Coord{4, 5}: 1,
				Coord{3, 5}: 1,
				Coord{3, 4}: 1,
				Coord{3, 3}: 1,
				Coord{3, 2}: 1,
			}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			grid := make(map[Coord]int)
			s := PathToMap(grid, 1, tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}
}

func TestCountedPathToMap(t *testing.T) {
	cases := []struct {
		in  []Vector
		out map[Coord][]int
	}{
		{[]Vector{Vector{North, 1}}, map[Coord][]int{Coord{0, 1}: {1, 0}}},
		{[]Vector{
			Vector{East, 8}, Vector{North, 5}, Vector{West, 5}, Vector{South, 3}},
			map[Coord][]int{
				Coord{1, 0}: {1, 0},
				Coord{2, 0}: {2, 0},
				Coord{3, 0}: {3, 0},
				Coord{4, 0}: {4, 0},
				Coord{5, 0}: {5, 0},
				Coord{6, 0}: {6, 0},
				Coord{7, 0}: {7, 0},
				Coord{8, 0}: {8, 0},
				Coord{8, 1}: {9, 0},
				Coord{8, 2}: {10, 0},
				Coord{8, 3}: {11, 0},
				Coord{8, 4}: {12, 0},
				Coord{8, 5}: {13, 0},
				Coord{7, 5}: {14, 0},
				Coord{6, 5}: {15, 0},
				Coord{5, 5}: {16, 0},
				Coord{4, 5}: {17, 0},
				Coord{3, 5}: {18, 0},
				Coord{3, 4}: {19, 0},
				Coord{3, 3}: {20, 0},
				Coord{3, 2}: {21, 0},
			},
		},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			grid := make(map[Coord][]int)
			s := CountedPathToMap(grid, 0, tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}
}
