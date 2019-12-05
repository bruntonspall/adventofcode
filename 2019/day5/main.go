package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	memory := make([]int, 0)
	for _, code := range strings.Split(input, ",") {
		icode, _ := strconv.Atoi(code)
		memory = append(memory, icode)
	}
	var icc pkg.IntCodeComputer
	icc.Initialise(memory)
	icc.Input = []int{1}
	icc.Run()
	part1 = fmt.Sprintf("%v", icc.Output)
	// Parse input and return output
	icc.Initialise(memory)
	icc.Input = []int{5}
	icc.Run()
	part2 = fmt.Sprintf("%v", icc.Output)
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
