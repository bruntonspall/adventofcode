package main

import (
	"fmt"

	"github.com/bruntonspall/adventofcode/pkg"
)

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	memory := pkg.Parse(input, ",")
	icc := pkg.NewIntCodeComputer(memory)
	icc.Run()
	icc.AddInput(1)
	out := icc.GetOutputs()
	part1 = fmt.Sprintf("%v", out[0])

	icc = pkg.NewIntCodeComputer(memory)
	icc.Run()
	icc.AddInput(2)
	out = icc.GetOutputs()
	part2 = fmt.Sprintf("%v", out[0])
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
