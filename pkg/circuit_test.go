package pkg

import (
	"fmt"
	"testing"

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
		t.Run(fmt.Sprintf("%v%v", tt.program, tt.phase), func(t *testing.T) {
			circuit := NewCircuit(5, tt.program)
			circuit.Run()
			circuit.SetPhase(tt.phase)
			circuit.AddInput(0)
			s := circuit.GetOutput()
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}

func TestFeedbackCircuit(t *testing.T) {
	cases := []struct {
		program []int
		phase   []int
		out     int
	}{
		{[]int{3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
			27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5}, []int{9, 8, 7, 6, 5}, 139629729},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v%v", tt.program, tt.phase), func(t *testing.T) {
			circuit := NewCircuit(5, tt.program)
			s := circuit.RunToCompletion(tt.phase, 0)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %v, want %v", s, tt.out)
			}
		})
	}

}
