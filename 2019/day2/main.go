package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// Parse input and return output
	memory := make([]int, 0)
	for _, code := range strings.Split(input, ",") {
		icode, _ := strconv.Atoi(code)
		memory = append(memory, icode)
	}
	var original [120]int
	copy(original[:], memory)
	memory[1] = 12
	memory[2] = 2
	memory = runIntCode(memory)
	part1 = strconv.Itoa(memory[0])
	// Parse input and return output
	// Test all possible noun and verb combinations until we find result
	for noun := 0; noun < 100; noun++ {
		for verb := 0; verb < 100; verb++ {
			copy(memory, original[:])
			memory[1] = noun
			memory[2] = verb
			memory = runIntCode(memory)
			if memory[0] == 19690720 {
				part2 = fmt.Sprintf("%d", noun*100+verb)
				return
			}
		}
	}
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
