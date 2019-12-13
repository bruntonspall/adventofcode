package pkg

import (
	"strconv"
	"strings"
)

// Parse takes a string and a separator, splits the line by seperators
// (say , or \n) and then returns a slice of numbers
func Parse(line string, sep string) []int {
	var ans []int
	for _, ch := range strings.Split(line, sep) {
		num, _ := strconv.Atoi(strings.Trim(ch, " "))
		ans = append(ans, num)
	}
	return ans
}

func Abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}
