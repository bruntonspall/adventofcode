package main

import (
	"fmt"
	"time"

	"github.com/bruntonspall/adventofcode/pkg"
)

// A Robot processes input and moves around
type Robot struct {
	X       int
	Y       int
	Heading int
	program []int
	brain   pkg.IntCodeComputer
	end     chan int
	paints  int
}

func NewRobot(program []int) Robot {
	robot := Robot{}
	robot.brain = *pkg.NewIntCodeComputer(program)
	robot.end = robot.brain.Run()
	return robot
}

func (robot *Robot) Location() pkg.Coord2D {
	return pkg.Coord2D{robot.X, robot.Y}
}

func (robot *Robot) Process(grid map[pkg.Coord2D]bool) map[pkg.Coord2D]bool {
	for {
		select {
		case <-robot.end:
			return grid
		default:
			in := 0
			if grid[pkg.Coord2D{robot.X, robot.Y}] {
				in = 1
			}
			robot.brain.AddInput(in)
			robot.Paint(grid, robot.brain.GetOutput())
			robot.Move(robot.brain.GetOutput())
			time.Sleep(10 * time.Microsecond)
		}
	}
}

func (robot *Robot) Move(in int) {
	fmt.Printf("Move from %v (%v)", robot.Location(), robot.Heading)
	switch in {
	case 1:
		robot.Heading = (robot.Heading + 90) % 360
	case 0:
		robot.Heading = (360 + robot.Heading - 90) % 360
	}

	switch robot.Heading {
	case 0:
		robot.Y += 1
	case 90:
		robot.X += 1
	case 180:
		robot.Y -= 1
	case 270:
		robot.X -= 1
	}
	fmt.Printf(" to %v(%v)\n", robot.Location(), robot.Heading)

}

func (robot *Robot) Paint(grid map[pkg.Coord2D]bool, in int) map[pkg.Coord2D]bool {
	switch in {
	case 0:
		grid[pkg.Coord2D{robot.X, robot.Y}] = false
	case 1:
		grid[pkg.Coord2D{robot.X, robot.Y}] = true
	}
	return grid
}

func PrintMap(grid map[pkg.Coord2D]bool) {
	minx, miny := 9999, 9999
	maxx, maxy := 0, 0
	for k := range grid {
		if k.X > maxx {
			maxx = k.X
		}
		if k.Y > maxy {
			maxy = k.Y
		}
		if k.X < minx {
			minx = k.X
		}
		if k.Y < miny {
			miny = k.Y
		}
	}
	for y := maxy; y >= miny; y-- {
		s := ""
		for x := minx; x <= maxx; x++ {
			if grid[pkg.Coord2D{x, y}] {
				s += "#"
			} else {
				s += " "
			}
		}
		fmt.Printf("%v\n", s)
	}
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// Parse input and return output
	program := pkg.Parse(input, ",")
	robot := NewRobot(program)
	grid := make(map[pkg.Coord2D]bool)
	grid = robot.Process(grid)
	part1 = fmt.Sprintf("%v", len(grid))
	// Part 2
	grid = make(map[pkg.Coord2D]bool)
	grid[pkg.Coord2D{0, 0}] = true
	robot = NewRobot(program)
	grid = robot.Process(grid)
	PrintMap(grid)
	part2 = "JUFEKHPH"
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
