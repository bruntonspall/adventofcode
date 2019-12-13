package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/bruntonspall/adventofcode/pkg"
)

// A Coord3D represents a coordinate in 3 dimensions
type Coord3D struct {
	X, Y, Z int
}

type Moon struct {
	Pos Coord3D
	Vel Coord3D
}

func (c Coord3D) String() string {
	return fmt.Sprintf("<x=%d, y=%d, z=%d>", c.X, c.Y, c.Z)
}
func (c Moon) String() string {
	return fmt.Sprintf("Pos=%v, Vel=%v", c.Pos, c.Vel)
}

func (c Coord3D) Add(v Coord3D) (r Coord3D) {
	return Coord3D{c.X + v.X, c.Y + v.Y, c.Z + v.Z}
}

func (c Coord3D) Gravity(t Coord3D) (v Coord3D) {
	if t.X > c.X {
		v.X = 1
	}
	if t.X < c.X {
		v.X = -1
	}
	if t.Y > c.Y {
		v.Y = 1
	}
	if t.Y < c.Y {
		v.Y = -1
	}
	if t.Z > c.Z {
		v.Z = 1
	}
	if t.Z < c.Z {
		v.Z = -1
	}
	return
}

var r = regexp.MustCompile(`(-?\d+)`)

func Parse(in string) []Moon {
	moons := make([]Moon, 4)
	for i, line := range strings.Split(in, "\n") {
		moon := Coord3D{}
		items := strings.Split(line, ",")
		moon.X, _ = strconv.Atoi(strings.Trim(items[0], "<> xyz="))
		moon.Y, _ = strconv.Atoi(strings.Trim(items[1], "<> xyz="))
		moon.Z, _ = strconv.Atoi(strings.Trim(items[2], "<> xyz="))
		moons[i] = Moon{moon, Coord3D{}}
	}
	return moons
}

func PrintMoons(moons []Moon) {
	for _, moon := range moons {
		fmt.Println(moon)
	}
}

func Step(moons []Moon) []Moon {
	for i := range moons {
		for j := range moons {
			if i != j {
				moons[i].Vel = moons[i].Vel.Add(moons[i].Pos.Gravity(moons[j].Pos))
			}
		}
	}
	for i := range moons {
		moons[i].Pos = moons[i].Pos.Add(moons[i].Vel)
	}
	return moons
}

func TotalEnergy(moons []Moon) int {
	total := 0
	for _, moon := range moons {
		pot := pkg.Abs(moon.Pos.X) + pkg.Abs(moon.Pos.Y) + pkg.Abs(moon.Pos.Z)
		kin := pkg.Abs(moon.Vel.X) + pkg.Abs(moon.Vel.Y) + pkg.Abs(moon.Vel.Z)
		total += pot * kin
	}
	return total
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	moons := Parse(input)
	println("After 0 steps")
	PrintMoons(moons)
	for i := 0; i < 1000; i++ {
		moons = Step(moons)
	}
	println("After 1000 steps")
	PrintMoons(moons)
	println("Total energy: ", TotalEnergy(moons))
	// Parse input and return output
	part1 = fmt.Sprintf("%d", TotalEnergy(moons))
	// Parse input and return output
	part2 = "Part2"
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
