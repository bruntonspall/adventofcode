package main

import (
	"fmt"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestRunProgram(t *testing.T) {
	cases := []struct {
		in  []int
		out []int
	}{
		{[]int{99}, []int{99}},
		{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}},
		{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, []int{30, 1, 1, 4, 2, 5, 6, 0, 99}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := runIntCode(tt.in)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}
}
func TestAdvance(t *testing.T) {
	cases := []struct {
		in  []int
		pc  int
		out []int
	}{
		{[]int{99}, 0, []int{99}},
		{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0, []int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}},
		{[]int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4, []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}},
		{[]int{1, 0, 0, 0, 99}, 0, []int{2, 0, 0, 0, 99}},
		{[]int{2, 3, 0, 3, 99}, 0, []int{2, 3, 0, 6, 99}},
		{[]int{2, 4, 4, 5, 99, 0}, 0, []int{2, 4, 4, 5, 99, 9801}},
		{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0, []int{1, 1, 1, 4, 2, 5, 6, 0, 99}},
		{[]int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4, []int{30, 1, 1, 4, 2, 5, 6, 0, 99}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			s := advance(tt.in, tt.pc)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}
}
