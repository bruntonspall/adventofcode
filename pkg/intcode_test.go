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
		{IntCodeComputer{[]int{99}, 0, nil, nil}, IntCodeComputer{[]int{99}, 0, nil, nil}},
		{IntCodeComputer{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0, nil, nil}, IntCodeComputer{[]int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 8, nil, nil}},
		{IntCodeComputer{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0, nil, nil}, IntCodeComputer{[]int{30, 1, 1, 4, 2, 5, 6, 0, 99}, 8, nil, nil}},
		{IntCodeComputer{[]int{1101, 100, -1, 4, 0}, 0, nil, nil}, IntCodeComputer{[]int{1101, 100, -1, 4, 99}, 4, nil, nil}},
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
		{IntCodeComputer{[]int{99}, 0, nil, nil}, IntCodeComputer{[]int{99}, 1, nil, nil}},
		{IntCodeComputer{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0, nil, nil}, IntCodeComputer{[]int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4, nil, nil}},
		{IntCodeComputer{[]int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4, nil, nil}, IntCodeComputer{[]int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 8, nil, nil}},
		{IntCodeComputer{[]int{1, 0, 0, 0, 99}, 0, nil, nil}, IntCodeComputer{[]int{2, 0, 0, 0, 99}, 4, nil, nil}},
		{IntCodeComputer{[]int{2, 3, 0, 3, 99}, 0, nil, nil}, IntCodeComputer{[]int{2, 3, 0, 6, 99}, 4, nil, nil}},
		{IntCodeComputer{[]int{2, 4, 4, 5, 99, 0}, 0, nil, nil}, IntCodeComputer{[]int{2, 4, 4, 5, 99, 9801}, 4, nil, nil}},
		{IntCodeComputer{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0, nil, nil}, IntCodeComputer{[]int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4, nil, nil}},
		{IntCodeComputer{[]int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4, nil, nil}, IntCodeComputer{[]int{30, 1, 1, 4, 2, 5, 6, 0, 99}, 8, nil, nil}},
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
		{[]int{99}, IntCodeComputer{[]int{99}, 0, nil, nil}},
		{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, IntCodeComputer{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0, nil, nil}},
		{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, IntCodeComputer{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0, nil, nil}},
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
		{IntCodeComputer{[]int{1, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{1, 5, 6, 7, 99, 20, 30, 50}, 4, nil, nil}},
		{IntCodeComputer{[]int{1101, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{1101, 5, 6, 7, 99, 20, 30, 11}, 4, nil, nil}},
		{IntCodeComputer{[]int{1001, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{1001, 5, 6, 7, 99, 20, 30, 26}, 4, nil, nil}},
		{IntCodeComputer{[]int{101, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{101, 5, 6, 7, 99, 20, 30, 35}, 4, nil, nil}},
		{IntCodeComputer{[]int{2, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{2, 5, 6, 7, 99, 20, 30, 600}, 4, nil, nil}},
		{IntCodeComputer{[]int{1102, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{1102, 5, 6, 7, 99, 20, 30, 30}, 4, nil, nil}},
		{IntCodeComputer{[]int{1002, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{1002, 5, 6, 7, 99, 20, 30, 120}, 4, nil, nil}},
		{IntCodeComputer{[]int{102, 5, 6, 7, 99, 20, 30, 0}, 0, nil, nil}, IntCodeComputer{[]int{102, 5, 6, 7, 99, 20, 30, 150}, 4, nil, nil}},
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

func TestIOInstructions(t *testing.T) {
	cases := []struct {
		in  IntCodeComputer
		out IntCodeComputer
	}{
		{IntCodeComputer{[]int{3, 2, 0, 0}, 0, []int{99}, []int{}}, IntCodeComputer{[]int{3, 2, 99, 0}, 2, []int{}, []int{}}},
		{IntCodeComputer{[]int{4, 3, 99, 4}, 0, []int{}, []int{}}, IntCodeComputer{[]int{4, 3, 99, 4}, 2, []int{}, []int{4}}},
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

func TestCompareAndJumpInstructions(t *testing.T) {
	cases := []struct {
		in  IntCodeComputer
		out IntCodeComputer
	}{
		{IntCodeComputer{[]int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 0, []int{8}, []int{}}, IntCodeComputer{[]int{3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8}, 8, []int{}, []int{1}}},
		{IntCodeComputer{[]int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 0, []int{6}, []int{}}, IntCodeComputer{[]int{3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8}, 8, []int{}, []int{0}}},
		{IntCodeComputer{[]int{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 0, []int{8}, []int{}}, IntCodeComputer{[]int{3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8}, 8, []int{}, []int{0}}},
		{IntCodeComputer{[]int{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 0, []int{6}, []int{}}, IntCodeComputer{[]int{3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8}, 8, []int{}, []int{1}}},
		{IntCodeComputer{[]int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 0, []int{8}, []int{}}, IntCodeComputer{[]int{3, 3, 1108, 1, 8, 3, 4, 3, 99}, 8, []int{}, []int{1}}},
		{IntCodeComputer{[]int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 0, []int{6}, []int{}}, IntCodeComputer{[]int{3, 3, 1108, 0, 8, 3, 4, 3, 99}, 8, []int{}, []int{0}}},
		{IntCodeComputer{[]int{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 0, []int{8}, []int{}}, IntCodeComputer{[]int{3, 3, 1107, 0, 8, 3, 4, 3, 99}, 8, []int{}, []int{0}}},
		{IntCodeComputer{[]int{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 0, []int{6}, []int{}}, IntCodeComputer{[]int{3, 3, 1107, 1, 8, 3, 4, 3, 99}, 8, []int{}, []int{1}}},
		{IntCodeComputer{[]int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, 0, []int{0}, []int{}}, IntCodeComputer{[]int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9}, 11, []int{}, []int{0}}},
		{IntCodeComputer{[]int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, 0, []int{-7}, []int{}}, IntCodeComputer{[]int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -7, 1, 1, 9}, 11, []int{}, []int{1}}},
		{IntCodeComputer{[]int{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 0, []int{0}, []int{}}, IntCodeComputer{[]int{3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0}, 11, []int{}, []int{0}}},
		{IntCodeComputer{[]int{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 0, []int{8}, []int{}}, IntCodeComputer{[]int{3, 3, 1105, 8, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 11, []int{}, []int{1}}},
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

func TestProgram(t *testing.T) {
	cases := []struct {
		in  []int
		out []int
	}{
		{[]int{7}, []int{999}},
		{[]int{8}, []int{1000}},
		{[]int{9}, []int{1001}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			var icc IntCodeComputer
			icc.Initialise([]int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
				1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
				999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99})
			icc.Input = tt.in
			icc.Run()
			if !cmp.Equal(icc.Output, tt.out) {
				t.Errorf("got %v, want %v", icc.Output, tt.out)
			}
		})
	}
}
