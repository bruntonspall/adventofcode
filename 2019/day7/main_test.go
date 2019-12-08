package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

func TestCircuit(t *testing.T) {
	cases := []struct {
		program []int
		phase   []int
		out     int
	}{
		{[]int{3, 7, 3, 8, 4, 8, 99, 0, 0}, []int{0, 1, 2, 3, 4}, 0},
		{[]int{3, 7, 3, 8, 4, 7, 99, 0, 0}, []int{0, 1, 2, 3, 4}, 4},
		{[]int{3, 7, 3, 8, 4, 7, 99, 0, 0}, []int{0, 1, 2, 3, 7}, 7},
		{[]int{3, 11, 3, 12, 1, 11, 12, 12, 4, 12, 99, 0, 0}, []int{0, 1, 2, 3, 4}, 10},
		{[]int{3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0}, []int{4, 3, 2, 1, 0}, 43210},
	}

	for _, tt := range cases {
		circuit := makeCircuit(tt.program)
		circuit.setPhase(tt.phase)
		t.Run(fmt.Sprintf("%v%v", tt.program, tt.phase), func(t *testing.T) {
			s := circuit.Run(0)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestFindPhase(t *testing.T) {
	cases := []struct {
		program []int
		phase   []int
		out     int
	}{
		{[]int{3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0}, []int{4, 3, 2, 1, 0}, 43210},
		{[]int{3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0}, []int{0, 1, 2, 3, 4}, 54321},
	}

	for _, tt := range cases {
		circuit := makeCircuit(tt.program)
		t.Run(fmt.Sprintf("%v%v", tt.program, tt.phase), func(t *testing.T) {
			phase := circuit.findMaxPhase(tt.program)
			circuit.Initialise(tt.program)
			circuit.setPhase(phase)
			s := circuit.Run(0)
			if !cmp.Equal(s, tt.out) || !cmp.Equal(phase, tt.phase) {
				t.Errorf("got %v, want %v (got phase %v, wanted phase %v)", s, tt.out, phase, tt.phase)
			}
		})
	}

}

var tests = pkg.TestCases{
	{
		"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
		"43210",
		"",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
