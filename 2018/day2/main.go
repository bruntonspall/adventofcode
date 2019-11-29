package main

import (
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

// Returns true if there is only one letter difference between two strings
func diffString(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	found := false
	for i := range s1 {
		if s1[i] != s2[i] {
			if found {
				// Found a second difference, return false
				return false
			}
			found = true
		}
	}
	return found
}

func charactersInCommon(s1, s2 string) string {
	buf := make([]byte, 0, len(s1))
	for i := range s1 {
		if s1[i] == s2[i] {
			buf = append(buf, s1[i])
		}
	}
	return string(buf)
}

func countRepeats(input string) (twos, threes int) {
	runeCount := make(map[rune]int)
	for _, rune := range input {
		runeCount[rune]++
	}
	for _, count := range runeCount {
		switch count {
		case 2:
			twos = 1
		case 3:
			threes = 1
		}
	}
	return
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// part1
	// Parse input and return output
	var twos, threes int
	for _, line := range strings.Split(input, "\n") {
		a, b := countRepeats(line)
		twos += a
		threes += b
	}
	part1 = strconv.Itoa(twos * threes)
	// Parse input and return output
	for _, line1 := range strings.Split(input, "\n") {
		for _, line2 := range strings.Split(input, "\n") {
			if diffString(line1, line2) {
				part2 = charactersInCommon(line1, line2)
			}
		}
	}
	return part1, part2
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
