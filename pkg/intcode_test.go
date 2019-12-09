package pkg

import (
	"fmt"
	"testing"

	"github.com/google/go-cmp/cmp"
)

/* With the input and output via channels, this internal inspecting is much harder */

func arrayToMap(a []int) (m map[int]int) {
	for i, k := range a {
		m[i] = k
	}
	return
}

func mapToArray(m map[int]int) (a []int) {
	max := 0
	for k := range m {
		if k > max {
			max = k
		}
	}
	fmt.Printf("%v -> %v\n", m, max)
	a = make([]int, max+1)
	for k, v := range m {
		a[k] = v
	}
	return
}

/* Verify that programs run until 99, changing memory as a side effect */
func TestRunProgram(t *testing.T) {
	cases := []struct {
		program []int
		out     []int
		pc      int
	}{
		{[]int{99}, []int{99}, 0},
		{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50},
			[]int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 8},
		{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99},
			[]int{30, 1, 1, 4, 2, 5, 6, 0, 99}, 8},
		{[]int{1101, 100, -1, 4, 0},
			[]int{1101, 100, -1, 4, 99}, 4},
		{[]int{22201, 1, 1, 5, 99},
			[]int{22201, 1, 1, 5, 99, 2}, 4},
		{[]int{109, 7, 22201, 0, 0, 0, 99, 2},
			[]int{109, 7, 22201, 0, 0, 0, 99, 4}, 6},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.program), func(t *testing.T) {
			cpu := NewIntCodeComputer(tt.program)
			cpu.Run()
			_, ok := <-cpu.Output
			if !ok {
				if !cmp.Equal(mapToArray(cpu.Memory), tt.out) || !cmp.Equal(cpu.PC, tt.pc) {
					t.Errorf("got %v, want %v", cpu.Memory, tt.out)
				}
			}
		})
	}
}

func TestRelativeMode(t *testing.T) {
	cases := []struct {
		base    int
		program map[int]int
		out     map[int]int
	}{
		{0, map[int]int{0: 00001, 1: 10, 2: 11, 3: 12, 10: 5, 11: 6, 12: 7}, map[int]int{0: 00001, 1: 10, 2: 11, 3: 12, 10: 5, 11: 6, 12: 11}},
		{0, map[int]int{0: 11101, 1: 10, 2: 11, 3: 12, 10: 5, 11: 6, 12: 7}, map[int]int{0: 11101, 1: 10, 2: 11, 3: 21, 10: 5, 11: 6, 12: 7}},
		{0, map[int]int{0: 22201, 1: 10, 2: 11, 3: 12, 10: 5, 11: 6, 12: 7}, map[int]int{0: 22201, 1: 10, 2: 11, 3: 12, 10: 5, 11: 6, 12: 11}},
		{100, map[int]int{0: 22201, 1: 10, 2: 11, 3: 12, 110: 5, 111: 6, 112: 7}, map[int]int{0: 22201, 1: 10, 2: 11, 3: 12, 110: 5, 111: 6, 112: 11}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.program), func(t *testing.T) {
			cpu := NewIntCodeComputer(nil)
			cpu.Memory = tt.program
			cpu.base = tt.base
			cpu.PC = 0
			cpu.Advance()
			if !cmp.Equal(cpu.Memory, tt.out) {
				t.Errorf("got %v, want %v", cpu.Memory, tt.out)
			}
		})
	}
}

func TestAdvance(t *testing.T) {
	cases := []struct {
		program []int
		startpc int
		out     []int
		pc      int
	}{
		{[]int{99}, 0, []int{99}, 0},
		{[]int{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 0, []int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4},
		{[]int{1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 4, []int{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}, 8},
		{[]int{1, 0, 0, 0, 99}, 0, []int{2, 0, 0, 0, 99}, 4},
		{[]int{2, 3, 0, 3, 99}, 0, []int{2, 3, 0, 6, 99}, 4},
		{[]int{2, 4, 4, 5, 99, 0}, 0, []int{2, 4, 4, 5, 99, 9801}, 4},
		{[]int{1, 1, 1, 4, 99, 5, 6, 0, 99}, 0, []int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4},
		{[]int{1, 1, 1, 4, 2, 5, 6, 0, 99}, 4, []int{30, 1, 1, 4, 2, 5, 6, 0, 99}, 8},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.program), func(t *testing.T) {
			cpu := NewIntCodeComputer(tt.program)
			cpu.PC = tt.startpc
			cpu.Advance()
			if !cmp.Equal(mapToArray(cpu.Memory), tt.out) || !cmp.Equal(cpu.PC, tt.pc) {
				t.Errorf("got %v(%v), want %v(%v)", cpu.Memory, cpu.PC, tt.out, tt.pc)
			}
		})
	}
}

func TestParameterModes(t *testing.T) {
	cases := []struct {
		in  []int
		out []int
	}{
		{[]int{1, 5, 6, 7, 99, 20, 30, 0}, []int{1, 5, 6, 7, 99, 20, 30, 50}},
		{[]int{1101, 5, 6, 7, 99, 20, 30, 0}, []int{1101, 5, 6, 7, 99, 20, 30, 11}},
		{[]int{1001, 5, 6, 7, 99, 20, 30, 0}, []int{1001, 5, 6, 7, 99, 20, 30, 26}},
		{[]int{101, 5, 6, 7, 99, 20, 30, 0}, []int{101, 5, 6, 7, 99, 20, 30, 35}},
		{[]int{2, 5, 6, 7, 99, 20, 30, 0}, []int{2, 5, 6, 7, 99, 20, 30, 600}},
		{[]int{1102, 5, 6, 7, 99, 20, 30, 0}, []int{1102, 5, 6, 7, 99, 20, 30, 30}},
		{[]int{1002, 5, 6, 7, 99, 20, 30, 0}, []int{1002, 5, 6, 7, 99, 20, 30, 120}},
		{[]int{102, 5, 6, 7, 99, 20, 30, 0}, []int{102, 5, 6, 7, 99, 20, 30, 150}},
		{[]int{201, 5, 6, 7, 99, 20, 30, 0}, []int{201, 5, 6, 7, 99, 20, 30, 50}},
		{[]int{2201, 5, 6, 7, 99, 20, 30, 0}, []int{2201, 5, 6, 7, 99, 20, 30, 50}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			cpu := NewIntCodeComputer(tt.in)
			cpu.Advance()
			if !cmp.Equal(mapToArray(cpu.Memory), tt.out) {
				t.Errorf("got %v, want %v", cpu.Memory, tt.out)
			}
		})
	}
}

func TestIOInstructions(t *testing.T) {
	cases := []struct {
		program []int
		in      int
		out     int
	}{
		{[]int{3, 0, 4, 0, 99}, 4, 4},
		{[]int{3, 0, 4, 0, 99}, 7, 7},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			icc := NewIntCodeComputer(tt.program)
			icc.Run()
			icc.AddInput(tt.in)
			out := icc.GetOutput()
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}
}

func TestCompareAndJumpInstructions(t *testing.T) {
	cases := []struct {
		program []int
		in      int
		out     int
	}{
		{[]int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 8, 1},
		{[]int{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 6, 0},
		{[]int{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 8, 0},
		{[]int{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 6, 1},
		{[]int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 8, 1},
		{[]int{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 6, 0},
		{[]int{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 8, 0},
		{[]int{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 6, 1},
		{[]int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, 0, 0},
		{[]int{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, -7, 1},
		{[]int{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 0, 0},
		{[]int{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 8, 1},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v->%v", tt.in, tt.program), func(t *testing.T) {
			icc := NewIntCodeComputer(tt.program)
			icc.Run()
			icc.AddInput(tt.in)
			out := icc.GetOutput()
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}
}

func TestProgram(t *testing.T) {
	cases := []struct {
		in  int
		out int
	}{
		{7, 999},
		{8, 1000},
		{9, 1001},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			icc := NewIntCodeComputer([]int{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
				1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
				999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99})
			icc.Run()
			icc.AddInput(tt.in)
			if !cmp.Equal(icc.GetOutput(), tt.out) {
				t.Errorf("got %v, want %v", icc.Output, tt.out)
			}
		})
	}
}

func TestDay9Programs(t *testing.T) {
	cases := []struct {
		program []int
		out     []int
	}{
		{[]int{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}, []int{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}},
		{[]int{1102, 34915192, 34915192, 7, 4, 7, 99, 0}, []int{1219070632396864}},
		{[]int{104, 1125899906842624, 99}, []int{1125899906842624}},
	}
	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.program), func(t *testing.T) {
			icc := NewIntCodeComputer(tt.program)
			icc.Run()
			out := icc.GetOutputs()
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}
}
