package main

import (
	"fmt"

	"github.com/bruntonspall/adventofcode/pkg"
)

func findMaxPhase(program []int) ([]int, int) {
	phase := []int{0, 0, 0, 0, 0}
	result := 0
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			for k := 0; k < 5; k++ {
				for l := 0; l < 5; l++ {
					for m := 0; m < 5; m++ {
						if i != j && i != k && i != l && i != m && j != k && j != l && j != m && k != l && k != m && l != m {
							c := pkg.NewCircuit(5, program)
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

func findFeedbackMaxPhase(program []int) ([]int, int) {
	phase := []int{0, 0, 0, 0, 0}
	result := 0
	for i := 5; i < 10; i++ {
		for j := 5; j < 10; j++ {
			for k := 5; k < 10; k++ {
				for l := 5; l < 10; l++ {
					for m := 5; m < 10; m++ {
						if i != j && i != k && i != l && i != m && j != k && j != l && j != m && k != l && k != m && l != m {
							c := pkg.NewCircuit(5, program)
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
	memory := pkg.Parse(input, ",")
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
