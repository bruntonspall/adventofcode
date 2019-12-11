package main

import (
	"fmt"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

// ParseMap takes a 2D map and returns coordinates of all the asteroids
func ParseMap(in string) (out []pkg.Coord2D) {
	lines := strings.Split(in, "\n")
	for y := 0; y < len(lines); y++ {
		for x := 0; x < len(lines[y]); x++ {
			switch lines[y][x] {
			case '#':
				out = append(out, pkg.Coord2D{x, y})
			}
		}
	}
	return
}

// CanSee determins whether pt1 can see pt2
func CanSee(asteroids []pkg.Coord2D, pt1 pkg.Coord2D, pt2 pkg.Coord2D) bool {
	l := pkg.NewLine(pt1, pt2)
	distance := pt1.DistanceTo(pt2)
	for _, point := range asteroids {
		if distance > pt1.DistanceTo(point) && point != pt1 && point != pt2 {
			if l.HasPoint(point) {
				// fmt.Printf("Line %v(%v) is blocked at %v\n", l, distance, point)
				return false
			}
		}
	}
	return true
}

func CountVisibleAsteroids(asteroids []pkg.Coord2D, source pkg.Coord2D) (count int) {
	for _, asteroid := range asteroids {
		if asteroid != source {
			// fmt.Printf("Checking to see if %v can see %v - ", source, asteroid)
			if CanSee(asteroids, source, asteroid) {
				// fmt.Printf("Yes\n")
				count++
				// } else {
				// fmt.Printf("No\n")
			}
		}
	}
	return
}
func GetCounts(asteroids []pkg.Coord2D) (counts map[pkg.Coord2D]int) {
	counts = make(map[pkg.Coord2D]int)
	for _, point := range asteroids {
		counts[point] = CountVisibleAsteroids(asteroids, point)
	}
	return
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	asteroids := ParseMap(input)
	counts := GetCounts(asteroids)
	max := 0
	maxAsteroid := asteroids[0]
	for asteroid, count := range counts {
		if count > max {
			max = count
			maxAsteroid = asteroid
		}
	}
	// Parse input and return output
	part1 = fmt.Sprintf("%v - %v", max, maxAsteroid)
	// Parse input and return output
	part2 = ""
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
