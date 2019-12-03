package main

import (
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// Parse input and return output
	total := 0
	for _, line := range strings.Split(input, "\n") {
		num, _ := strconv.Atoi(line)
		total += num/3 - 2
	}
	part1 = strconv.Itoa(total)

	// Parse input and return output
	total = 0
	for _, line := range strings.Split(input, "\n") {
		fuel := 0
		num, _ := strconv.Atoi(line)
		calc := num/3 - 2
		for calc > 0 {
			fuel += calc
			calc = calc/3 - 2
		}
		total += fuel
	}

	part2 = strconv.Itoa(total)
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
