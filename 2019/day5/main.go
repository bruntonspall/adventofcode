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
	var icc = pkg.NewIntCodeComputer(memory)
	icc.Run()
	icc.AddInput(1)
	part1 = fmt.Sprintf("%v", icc.GetOutputs())
	// Parse input and return output
	icc = pkg.NewIntCodeComputer(memory)
	icc.Run()
	icc.AddInput(5)
	part2 = fmt.Sprintf("%v", icc.GetOutputs())
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
