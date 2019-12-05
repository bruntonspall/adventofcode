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
	var icc pkg.IntCodeComputer
	icc.Initialise(memory)
	icc.Memory[1] = 12
	icc.Memory[2] = 2
	icc.Run()
	part1 = strconv.Itoa(icc.Memory[0])
	// Parse input and return output
	// Test all possible noun and verb combinations until we find result
	if len(memory) < 100 {
		for i := 0; i < 100; i++ {
			memory = append(memory, 99)
		}
	}
	for noun := 0; noun < 100; noun++ {
		for verb := 0; verb < 100; verb++ {
			icc.Initialise(memory)
			icc.Memory[1] = noun
			icc.Memory[2] = verb
			icc.Run()
			if icc.Memory[0] == 19690720 {
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
