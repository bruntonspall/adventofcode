package pkg

import (
	"fmt"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestRunProgram(t *testing.T) {
	cases := []struct {
		in  IntCodeComputer
		out IntCodeComputer
	}{
		{IntCodeComputer{[]int{99}, 0}, IntCodeComputer{[]int{99}, 0}},
		{IntCodeComputer{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0}, IntCodeComputer{[]int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 8}},
		{IntCodeComputer{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0}, IntCodeComputer{[]int{30, 1, 1, 4, 2, 5, 6, 0, 99}, 8}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			tt.in.Run()
			if !cmp.Equal(tt.in, tt.out) {
				t.Errorf("got %v, want %v", tt.in, tt.out)
			}
		})
	}
}
func TestAdvance(t *testing.T) {
	cases := []struct {
		in  IntCodeComputer
		out IntCodeComputer
	}{
		{IntCodeComputer{[]int{99}, 0}, IntCodeComputer{[]int{99}, 0}},
		{IntCodeComputer{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0}, IntCodeComputer{[]int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4}},
		{IntCodeComputer{[]int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4}, IntCodeComputer{[]int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 8}},
		{IntCodeComputer{[]int{1, 0, 0, 0, 99}, 0}, IntCodeComputer{[]int{2, 0, 0, 0, 99}, 4}},
		{IntCodeComputer{[]int{2, 3, 0, 3, 99}, 0}, IntCodeComputer{[]int{2, 3, 0, 6, 99}, 4}},
		{IntCodeComputer{[]int{2, 4, 4, 5, 99, 0}, 0}, IntCodeComputer{[]int{2, 4, 4, 5, 99, 9801}, 4}},
		{IntCodeComputer{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0}, IntCodeComputer{[]int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4}},
		{IntCodeComputer{[]int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4}, IntCodeComputer{[]int{30, 1, 1, 4, 2, 5, 6, 0, 99}, 8}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			tt.in.Advance()
			if !cmp.Equal(tt.in, tt.out) {
				t.Errorf("got %v, want %v", tt.in, tt.out)
			}
		})
	}
}

func TestInitialise(t *testing.T) {
	cases := []struct {
		in  []int
		out IntCodeComputer
	}{
		{[]int{99}, IntCodeComputer{[]int{99}, 0}},
		{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, IntCodeComputer{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0}},
		{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, IntCodeComputer{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			var icc IntCodeComputer
			icc.Initialise(tt.in)
			if !cmp.Equal(icc, tt.out) {
				t.Errorf("got %v, want %v", icc, tt.out)
			}
		})
	}
}

func TestParameterModes(t *testing.T) {
	cases := []struct {
		in  IntCodeComputer
		out IntCodeComputer
	}{
		{IntCodeComputer{[]int{1, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{1, 5, 6, 7, 99, 20, 30, 50}, 4}},
		{IntCodeComputer{[]int{1101, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{1101, 5, 6, 7, 99, 20, 30, 11}, 4}},
		{IntCodeComputer{[]int{1001, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{1001, 5, 6, 7, 99, 20, 30, 26}, 4}},
		{IntCodeComputer{[]int{101, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{101, 5, 6, 7, 99, 20, 30, 35}, 4}},
		{IntCodeComputer{[]int{2, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{2, 5, 6, 7, 99, 20, 30, 600}, 4}},
		{IntCodeComputer{[]int{1102, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{1102, 5, 6, 7, 99, 20, 30, 30}, 4}},
		{IntCodeComputer{[]int{1002, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{1002, 5, 6, 7, 99, 20, 30, 120}, 4}},
		{IntCodeComputer{[]int{102, 5, 6, 7, 99, 20, 30, 0}, 0}, IntCodeComputer{[]int{102, 5, 6, 7, 99, 20, 30, 150}, 4}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			tt.in.Advance()
			if !cmp.Equal(tt.in, tt.out) {
				t.Errorf("got %v, want %v", tt.in, tt.out)
			}
		})
	}
}
