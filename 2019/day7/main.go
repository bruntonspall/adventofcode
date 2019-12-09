package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

type Circuit struct {
	computers []pkg.IntCodeComputer
	input     chan int
	output    chan int
}

func makeCircuit(numCircuits int, program []int) (circuit Circuit) {
	input := make(chan int, 2)
	t := input
	for i := 0; i < numCircuits; i++ {
		cpu := pkg.NewIntCodeComputer(program)
		cpu.Input = t
		t = cpu.Output
		circuit.computers = append(circuit.computers, *cpu)
	}
	circuit.input = input
	circuit.output = t
	return circuit
}

func (c *Circuit) setPhase(phase []int) {
	for i := range c.computers {
		c.computers[i].AddInput(phase[i])
	}
}

func (c *Circuit) Run() {
	for i := range c.computers {
		c.computers[i].Run()
	}
}

func (c *Circuit) RunToCompletion(phase []int, input int) int {
	for i := range c.computers {
		c.computers[i].Run()
	}
	c.setPhase(phase)
	c.input <- 0
	var o = 0
	for o = range c.output {
		c.input <- o
	}
	return o
}

func findMaxPhase(program []int) ([]int, int) {
	phase := []int{0, 0, 0, 0, 0}
	result := 0
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			for k := 0; k < 5; k++ {
				for l := 0; l < 5; l++ {
					for m := 0; m < 5; m++ {
						if i != j && i != k && i != l && i != m && j != k && j != l && j != m && k != l && k != m && l != m {
							c := makeCircuit(5, program)
							c.Run()
							c.setPhase([]int{i, j, k, l, m})
							c.input <- 0
							out := <-c.output
							if out > result {
								result = out
								phase = []int{i, j, k, l, m}
							}
						}
					}
				}
			}
		}
	}
	return phase, result
}

func findFeedbackMaxPhase(program []int) ([]int, int) {
	phase := []int{0, 0, 0, 0, 0}
	result := 0
	for i := 5; i < 10; i++ {
		for j := 5; j < 10; j++ {
			for k := 5; k < 10; k++ {
				for l := 5; l < 10; l++ {
					for m := 5; m < 10; m++ {
						if i != j && i != k && i != l && i != m && j != k && j != l && j != m && k != l && k != m && l != m {
							c := makeCircuit(5, program)
							out := c.RunToCompletion([]int{i, j, k, l, m}, 0)
							if out > result {
								result = out
								phase = []int{i, j, k, l, m}
							}
						}
					}
				}
			}
		}
	}
	return phase, result
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// Parse input and return output
	memory := make([]int, 0)
	for _, code := range strings.Split(input, ",") {
		icode, _ := strconv.Atoi(code)
		memory = append(memory, icode)
	}
	_, result := findMaxPhase(memory)

	part1 = fmt.Sprintf("%v", result)
	// Parse input and return output
	_, result = findFeedbackMaxPhase(memory)
	part2 = fmt.Sprintf("%v", result)
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
