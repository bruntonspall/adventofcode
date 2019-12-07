package main

import (
	"fmt"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

func countChildren(tree map[string][]string, root string) (sum int) {
	_, ok := tree[root]
	if !ok {
		return 0
	}
	for _, child := range tree[root] {
		sum += countChildren(tree, child)
		sum++
	}
	return sum
}

func findAncestor(tree map[string][]string, target string) string {
	for node, children := range tree {
		for _, child := range children {
			if child == target {
				return node
			}
		}
	}
	return ""
}

func reverse(t []string) (r []string) {
	for i := len(t); i > 0; i-- {
		r = append(r, t[i-1])
	}
	return r
}

func ancestors(tree map[string][]string, root string) []string {
	node := root
	found := []string{}
	for node != "" {
		found = append(found, node)
		node = findAncestor(tree, node)
	}
	return reverse(found)
}

func makeTree(in string) map[string][]string {
	orbitlist := strings.Split(in, "\n")
	orbits := map[string][]string{}
	for _, orbit := range orbitlist {
		kv := strings.Split(orbit, ")")
		orbits[kv[0]] = append(orbits[kv[0]], kv[1])
	}
	fmt.Printf("Orbits %v\n", orbits)
	return orbits
}

func countDistance(tree map[string][]string, a, b string) int {
	you := ancestors(tree, a)
	san := ancestors(tree, b)
	fmt.Printf("countDistance(%v, %v) {%v, %v}\n", a, b, you, san)
	var l *[]string
	if len(you) > len(san) {
		l = &you
	} else {
		l = &san
	}
	path := []string{}
	for i := range *l {
		if i == len(san) || i == len(you) || san[i] != you[i] {
			path = append(path, san[i:]...)
			path = append(path, you[i:]...)
			break
		}
	}
	fmt.Printf("Path from %v to %v is via %v\n", a, b, path)
	return len(path)
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	orbits := makeTree(input)
	count := 0

	// Walk map and count roots
	for k := range orbits {
		count += countChildren(orbits, k)
	}
	// Parse input and return output
	part1 = fmt.Sprintf("%v", count)
	// Parse input and return output

	// Subtract 2 because You and SAN don't count
	part2 = fmt.Sprintf("%v", countDistance(orbits, "YOU", "SAN")-2)
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
