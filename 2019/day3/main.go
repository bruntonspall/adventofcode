package main

import (
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

type Direction int

const (
	North Direction = 0
	East  Direction = 1
	South Direction = 2
	West  Direction = 3
)

type Vector struct {
	Direction Direction
	Distance  int
}

type Coord struct {
	X, Y int
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func PathToMap(grid map[Coord]int, mask int, input []Vector) map[Coord]int {
	x, y := 0, 0
	for _, vec := range input {
		for d := 0; d < vec.Distance; d++ {
			switch vec.Direction {
			case North:
				y++
				grid[Coord{x, y}] |= mask
			case East:
				x++
				grid[Coord{x, y}] |= mask
			case South:
				y--
				grid[Coord{x, y}] |= mask
			case West:
				x--
				grid[Coord{x, y}] |= mask
			}
		}
	}
	return grid
}

func CountedPathToMap(grid map[Coord][]int, mask int, input []Vector) map[Coord][]int {
	x, y := 0, 0
	count := 0
	for _, vec := range input {
		for d := 0; d < vec.Distance; d++ {
			count++
			switch vec.Direction {
			case North:
				y++
			case East:
				x++
			case South:
				y--
			case West:
				x--
			}
			w, ok := grid[Coord{x, y}]
			if !ok {
				w = make([]int, 2)
			}
			if count < w[mask] || w[mask] == 0 {
				w[mask] = count
			}
			grid[Coord{x, y}] = w
		}
	}
	return grid
}

func ParsePath(line string) (path []Vector) {
	path = make([]Vector, 0)
	for _, node := range strings.Split(line, ",") {
		distance, _ := strconv.Atoi(node[1:])
		switch node[0] {
		case 'U':
			path = append(path, Vector{North, distance})
		case 'D':
			path = append(path, Vector{South, distance})
		case 'L':
			path = append(path, Vector{West, distance})
		case 'R':
			path = append(path, Vector{East, distance})
		}
	}
	return
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	grid := make(map[Coord]int)
	for i, p := range strings.Split(input, "\n") {
		path := ParsePath(p)
		grid = PathToMap(grid, 2>>i, path)
	}
	distance := 99999
	for k, v := range grid {
		if v == 3 {
			cost := Abs(k.X) + Abs(k.Y)
			if cost < distance {
				distance = cost
			}
		}
	}
	// Parse input and return output
	part1 = strconv.Itoa(distance)
	// Parse input and return output
	grid2 := make(map[Coord][]int)
	for i, p := range strings.Split(input, "\n") {
		path := ParsePath(p)
		grid2 = CountedPathToMap(grid2, i, path)
	}
	distance2 := 99999
	for _, v := range grid2 {
		if v[0] != 0 && v[1] != 0 {
			cost := v[0] + v[1]
			if cost < distance2 {
				distance2 = cost
			}
		}
	}
	part2 = strconv.Itoa(distance2)
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
