package main

import (
	"fmt"
	"strconv"

	"github.com/bruntonspall/adventofcode/pkg"
)

// returns part1 and part2
func run(input string) (string, string) {
	// part1
	// Parse input and return output
	nums := pkg.Parse(input, "\n")
	sum := 0
	for _, i := range nums {
		sum += i
	}
	part1 := fmt.Sprintf("%d", sum)
	// Parse input and return output
	sum = 0
	seen := make(map[int]bool)
	for {
		for _, i := range nums {
			sum += i
			if seen[sum] {
				return part1, strconv.Itoa(sum)
			}
			seen[sum] = true
		}
	}
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
