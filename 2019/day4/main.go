package main

import (
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

func ValidatePin(in int) bool {
	ins := strconv.Itoa(in)
	adjacant := false
	increase := true
	var last rune = '0'
	for _, ch := range ins {
		if ch == last {
			adjacant = true
		}
		if ch < last {
			increase = false
		}
		last = ch
	}
	return adjacant && increase
}

func ValidatePin2(in int) bool {
	ins := strconv.Itoa(in)
	adjacant := make(map[rune]bool)
	adjacancy := false
	increase := true
	var last rune = '0'
	counts := make(map[rune]int)
	for _, ch := range ins {
		counts[ch]++
		if ch == last {
			adjacant[ch] = true
		}
		if ch < last {
			increase = false
		}
		last = ch
	}
	for ch, c := range counts {
		if c == 2 && adjacant[ch] {
			adjacancy = true
		}
	}
	return adjacancy && increase
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	startstop := strings.Split(input, "-")
	start, _ := strconv.Atoi(startstop[0])
	stop, _ := strconv.Atoi(startstop[1])
	count1 := 0
	count2 := 0
	for i := start; i <= stop; i++ {
		if ValidatePin(i) {
			count1++
		}
		if ValidatePin2(i) {
			count2++
		}
	}
	// Parse input and return output
	part1 = strconv.Itoa(count1)
	// Parse input and return output
	part2 = strconv.Itoa(count2)
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
