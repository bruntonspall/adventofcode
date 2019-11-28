package pkg

import (
	"strconv"
	"strings"
)

func Parse(line string, sep string) []int {
	var ans []int
	for _, ch := range strings.Split(line, sep) {
		num, _ := strconv.Atoi(strings.Trim(ch, " "))
		ans = append(ans, num)
	}
	return ans
}
