package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

type Circuit struct {
	computers [5]pkg.IntCodeComputer
}

func makeCircuit(program []int) (circuit Circuit) {
	for i := range circuit.computers {
		circuit.computers[i].Initialise(program)
	}
	return
}

func (c *Circuit) Initialise(program []int) {
	for i := range c.computers {
		c.computers[i].Initialise(program)
	}
}

func (c *Circuit) setPhase(phase []int) {
	for i := range c.computers {
		c.computers[i].Input = append(c.computers[i].Input, phase[i])
	}
}

func (c *Circuit) Run(input int) int {
	for i := range c.computers {
		c.computers[i].Input = append(c.computers[i].Input, input)
		c.computers[i].Run()
		input = c.computers[i].Output[0]
	}
	return input
}

func (c *Circuit) findMaxPhase(program []int) []int {
	phase := []int{0, 0, 0, 0, 0}
	result := 0
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			for k := 0; k < 5; k++ {
				for l := 0; l < 5; l++ {
					for m := 0; m < 5; m++ {
						if i != j && i != k && i != l && i != m && j != k && j != l && j != m && k != l && k != m && l != m {
							c.Initialise(program)
							c.setPhase([]int{i, j, k, l, m})
							c.Run(0)
							if c.computers[4].Output[0] > result {
								result = c.computers[4].Output[0]
								phase = []int{i, j, k, l, m}
							}
						}
					}
				}
			}
		}
	}
	return phase
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// Parse input and return output
	memory := make([]int, 0)
	for _, code := range strings.Split(input, ",") {
		icode, _ := strconv.Atoi(code)
		memory = append(memory, icode)
	}
	circuit := makeCircuit(memory)
	phase := circuit.findMaxPhase(memory)
	circuit.Initialise(memory)
	circuit.setPhase(phase)
	part1 = fmt.Sprintf("%v", circuit.Run(0))
	// Parse input and return output
	part2 = ""
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
