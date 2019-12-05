package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

var tests = pkg.TestCases{
	{
		"111109-111111",
		"1",
		"0",
	},
	{
		"138307-654504",
		"1855",
		"1253",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}

func TestValidatePin(t *testing.T) {
	cases := []struct {
		in  int
		out bool
	}{
		{111109, false},
		{111110, false},
		{111111, true},
		{111112, true},
		{223450, false},
		{123789, false},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := ValidatePin(tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestValidatePin2(t *testing.T) {
	cases := []struct {
		in  int
		out bool
	}{
		{111109, false},
		{111110, false},
		{111111, false},
		{111112, false},
		{223450, false},
		{123789, false},
		{112233, true},
		{123444, false},
		{111122, true},
		{112222, true},
		{112221, false},
		{443334, false},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := ValidatePin2(tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}
