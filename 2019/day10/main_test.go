package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

func TestParseMap(t *testing.T) {
	cases := []struct {
		in  string
		out []pkg.Coord2D
	}{
		{"...\n.#.\n...", []pkg.Coord2D{pkg.Coord2D{1, 1}}},
		{`#..
.#.
..#`, []pkg.Coord2D{pkg.Coord2D{0, 0}, pkg.Coord2D{1, 1}, pkg.Coord2D{2, 2}}},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			out := ParseMap(tt.in)
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}

}

var testmap = []pkg.Coord2D{
	pkg.Coord2D{1, 0},
	pkg.Coord2D{4, 0},
	pkg.Coord2D{0, 2},
	pkg.Coord2D{1, 2},
	pkg.Coord2D{2, 2},
	pkg.Coord2D{3, 2},
	pkg.Coord2D{4, 2},
	pkg.Coord2D{4, 3},
	pkg.Coord2D{3, 4},
	pkg.Coord2D{4, 4},
}

var testcounts = map[pkg.Coord2D]int{
	pkg.Coord2D{1, 0}: 7,
	pkg.Coord2D{4, 0}: 7,
	pkg.Coord2D{0, 2}: 6,
	pkg.Coord2D{1, 2}: 7,
	pkg.Coord2D{2, 2}: 7,
	pkg.Coord2D{3, 2}: 7,
	pkg.Coord2D{4, 2}: 5,
	pkg.Coord2D{4, 3}: 7,
	pkg.Coord2D{3, 4}: 8,
	pkg.Coord2D{4, 4}: 7,
}

/*
0123456789
#......... 0
...A...... 1
...B..a... 2
.EDCG....a 3
..F.c.b... 4
.....c.... 5
..efd.c.gb 6
.......c.. 7
....f...c. 8
...e..d..c 9
*/
var visibilityTestMap = map[pkg.Coord2D]map[pkg.Coord2D]bool{
	pkg.Coord2D{3, 1}: map[pkg.Coord2D]bool{
		pkg.Coord2D{6, 2}: false,
		pkg.Coord2D{9, 3}: false,
	},
	pkg.Coord2D{3, 2}: map[pkg.Coord2D]bool{
		pkg.Coord2D{6, 4}: false,
		pkg.Coord2D{9, 6}: false,
	},
	pkg.Coord2D{3, 3}: map[pkg.Coord2D]bool{
		pkg.Coord2D{4, 4}: false,
		pkg.Coord2D{5, 5}: false,
		pkg.Coord2D{6, 6}: false,
		pkg.Coord2D{7, 7}: false,
		pkg.Coord2D{8, 8}: false,
		pkg.Coord2D{9, 9}: false,
	},
	pkg.Coord2D{2, 3}: map[pkg.Coord2D]bool{
		pkg.Coord2D{4, 6}: false,
		pkg.Coord2D{6, 9}: false,
	},
	pkg.Coord2D{1, 3}: map[pkg.Coord2D]bool{
		pkg.Coord2D{2, 6}: false,
		pkg.Coord2D{3, 9}: false,
	},
	pkg.Coord2D{2, 4}: map[pkg.Coord2D]bool{
		pkg.Coord2D{3, 6}: false,
		pkg.Coord2D{4, 8}: false,
	},
	pkg.Coord2D{4, 3}: map[pkg.Coord2D]bool{
		pkg.Coord2D{8, 6}: false,
	},
}

func TestCanSee(t *testing.T) {
	cases := []struct {
		asteroids []pkg.Coord2D
		from      pkg.Coord2D
		to        pkg.Coord2D
		expected  bool
	}{
		{[]pkg.Coord2D{pkg.Coord2D{0, 0}, pkg.Coord2D{1, 1}, pkg.Coord2D{2, 2}}, pkg.Coord2D{0, 0}, pkg.Coord2D{2, 0}, true},
		{[]pkg.Coord2D{pkg.Coord2D{0, 0}, pkg.Coord2D{1, 1}, pkg.Coord2D{2, 2}}, pkg.Coord2D{0, 0}, pkg.Coord2D{0, 2}, true},
		{[]pkg.Coord2D{pkg.Coord2D{0, 0}, pkg.Coord2D{1, 1}, pkg.Coord2D{2, 2}}, pkg.Coord2D{0, 0}, pkg.Coord2D{2, 0}, true},
		{[]pkg.Coord2D{pkg.Coord2D{0, 0}, pkg.Coord2D{1, 1}, pkg.Coord2D{2, 2}}, pkg.Coord2D{0, 0}, pkg.Coord2D{2, 2}, false},
		{testmap, pkg.Coord2D{3, 4}, pkg.Coord2D{2, 2}, true},
		{testmap, pkg.Coord2D{3, 4}, pkg.Coord2D{4, 0}, true},
		{testmap, pkg.Coord2D{3, 4}, pkg.Coord2D{1, 0}, false},
		{testmap, pkg.Coord2D{4, 0}, pkg.Coord2D{4, 2}, true},
		{testmap, pkg.Coord2D{4, 0}, pkg.Coord2D{4, 3}, false},
		{testmap, pkg.Coord2D{4, 0}, pkg.Coord2D{4, 4}, false},
		{testmap, pkg.Coord2D{4, 0}, pkg.Coord2D{3, 4}, true},
		{testmap, pkg.Coord2D{3, 2}, pkg.Coord2D{1, 0}, true},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt), func(t *testing.T) {
			out := CanSee(tt.asteroids, tt.from, tt.to)
			if !cmp.Equal(out, tt.expected) {
				t.Errorf("got %v, want %v", out, tt.expected)
			}
		})
	}
	for pt, shadow := range visibilityTestMap {
		t.Run(fmt.Sprintf("%v-%v", pt, shadow), func(t *testing.T) {
			for to := range shadow {
				out := CanSee([]pkg.Coord2D{pt}, pkg.Coord2D{0, 0}, to)
				if !cmp.Equal(out, shadow[to]) {
					t.Errorf("%v - got %v, want %v", to, out, shadow[to])
				}
			}
		})

	}

}

func TestCountVisibleAsteroids(t *testing.T) {
	for key, count := range testcounts {
		t.Run(fmt.Sprintf("%v", key), func(t *testing.T) {
			out := CountVisibleAsteroids(testmap, key)
			if !cmp.Equal(out, count) {
				t.Errorf("got %v, want %v", out, count)
			}
		})

	}
}

func TestGetCounts(t *testing.T) {
	cases := []struct {
		asteroids []pkg.Coord2D
		counts    map[pkg.Coord2D]int
	}{
		{[]pkg.Coord2D{pkg.Coord2D{0, 0}, pkg.Coord2D{1, 1}, pkg.Coord2D{2, 2}},
			map[pkg.Coord2D]int{pkg.Coord2D{0, 0}: 1, pkg.Coord2D{1, 1}: 2, pkg.Coord2D{2, 2}: 1}},
		{testmap, testcounts},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt), func(t *testing.T) {
			out := GetCounts(tt.asteroids)
			if !cmp.Equal(out, tt.counts) {
				t.Errorf("got %v, want %v", out, tt.counts)
			}
		})
	}

}

var tests = pkg.TestCases{
	{
		`.#..#
.....
#####
....#
...##`,
		"8 - {3 4}",
		"",
	},
	{
		`......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####`,
		"33 - {5 8}",
		"",
	},
	{
		`#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.`,
		"35 - {1 2}",
		"",
	},
	{
		`.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..`,
		"41 - {6 3}",
		"",
	},
	{
		`.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##`,
		"210 - {11 13}",
		"",
	},
	{
		puzzle,
		"Foo",
		"",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
