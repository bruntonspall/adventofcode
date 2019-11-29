package main

import (
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

var tests = pkg.TestCases{
	{
		`abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab`,
		"12",
		"",
	},
	{
		`abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz`,
		"",
		"fgij",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}

func TestCountOccurences(t *testing.T) {
	cases := []struct {
		in     string
		twos   int
		threes int
	}{
		{"abcdef", 0, 0},
		{"bababc", 1, 1},
		{"abbcde", 1, 0},
		{"abcccd", 0, 1},
		{"aabcdd", 1, 0},
		{"abcdee", 1, 0},
		{"ababab", 0, 1},
	}
	for _, tt := range cases {
		t.Run(tt.in, func(t *testing.T) {
			twos, threes := countRepeats(tt.in)
			if !cmp.Equal(twos, tt.twos) {
				t.Errorf("%q should have got %d, actual %d", tt.in, tt.twos, twos)
			}
			if !cmp.Equal(threes, tt.threes) {
				t.Errorf("%q should have got %d, actual %d", tt.in, tt.threes, threes)
			}
		})
	}
}

func TestDiffString(t *testing.T) {
	cases := []struct {
		s1       string
		s2       string
		expected bool
	}{
		{"abcde", "fghij", false},
		{"abcde", "axcye", false},
		{"fghij", "fguij", true},
	}
	for _, tt := range cases {
		t.Run(tt.s1, func(t *testing.T) {
			actual := diffString(tt.s1, tt.s2)
			if !cmp.Equal(actual, tt.expected) {
				t.Errorf("%v should have got %v, actual %v\n", tt.s1, tt.expected, actual)
			}
		})
	}
}

func TestCharactersInCommon(t *testing.T) {
	cases := []struct {
		s1       string
		s2       string
		expected string
	}{
		{"abcde", "abzde", "abde"},
		{"abcde", "axcye", "ace"},
		{"fghij", "fguij", "fgij"},
	}
	for _, tt := range cases {
		t.Run(tt.s1, func(t *testing.T) {
			actual := charactersInCommon(tt.s1, tt.s2)
			if !cmp.Equal(actual, tt.expected) {
				t.Errorf("%v should have got %q, actual %q\n", tt.s1, tt.expected, actual)
			}
		})
	}
}
