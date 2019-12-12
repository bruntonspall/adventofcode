package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

var tests = pkg.TestCases{
	{
		puzzle,
		"1909",
		"JUFEKHPH",
	},
}

func TestRobotProcess(t *testing.T) {
	cases := []struct {
		robot    Robot
		start    pkg.Coord2D
		grid     map[pkg.Coord2D]bool
		location pkg.Coord2D
	}{
		{NewRobot([]int{3, 9, 4, 10, 4, 11, 99, 0, 0, 0, 1, 0}), pkg.Coord2D{0, 0}, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: true}, pkg.Coord2D{-1, 0}},
		{NewRobot([]int{3, 9, 4, 10, 4, 11, 99, 0, 0, 0, 1, 1}), pkg.Coord2D{0, 0}, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: true}, pkg.Coord2D{1, 0}},
		{NewRobot([]int{3, 9, 4, 10, 4, 11, 99, 0, 0, 0, 0, 0}), pkg.Coord2D{0, 0}, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: false}, pkg.Coord2D{-1, 0}},
		{NewRobot([]int{3, 9, 4, 10, 4, 11, 99, 0, 0, 0, 0, 1}), pkg.Coord2D{0, 0}, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: false}, pkg.Coord2D{1, 0}},
		{NewRobot([]int{3, 0, 4, 13, 4, 14, 3, 0, 4, 13, 4, 14, 99, 1, 0}), pkg.Coord2D{0, 0}, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: true, pkg.Coord2D{-1, 0}: true}, pkg.Coord2D{-1, -1}},
	}

	for _, tt := range cases {
		// t.Run(fmt.Sprintf("%v", tt.robot), func(t *testing.T) {
		tt.robot.X = tt.start.X
		tt.robot.Y = tt.start.Y
		grid := tt.robot.Process(map[pkg.Coord2D]bool{})
		if !cmp.Equal(grid, tt.grid) {
			t.Errorf("got %v, want %v", grid, tt.grid)
		}
		if !cmp.Equal(tt.robot.Location(), tt.location) {
			t.Errorf("got %v, want %v", tt.robot.Location(), tt.location)
		}
		// })
	}

}

func TestRobotMove(t *testing.T) {
	cases := []struct {
		in      int
		x       int
		y       int
		heading int
	}{
		{0, -1, 0, 270},
		{1, 1, 0, 90},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			robot := NewRobot(nil)
			robot.Move(tt.in)
			if !cmp.Equal(robot.X, tt.x) {
				t.Errorf("got %v, want %v", robot.X, tt.x)
			}
			if !cmp.Equal(robot.Y, tt.y) {
				t.Errorf("got %v, want %v", robot.Y, tt.y)
			}
			if !cmp.Equal(robot.Heading, tt.heading) {
				t.Errorf("got %v, want %v", robot.Heading, tt.heading)
			}
		})
	}

}
func TestRobotPaint(t *testing.T) {
	cases := []struct {
		in   int
		grid map[pkg.Coord2D]bool
	}{
		{0, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: false}},
		{1, map[pkg.Coord2D]bool{pkg.Coord2D{0, 0}: true}},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.in), func(t *testing.T) {
			robot := NewRobot(nil)
			grid := robot.Paint(map[pkg.Coord2D]bool{}, tt.in)
			if !cmp.Equal(grid, tt.grid) {
				t.Errorf("got %v, want %v", grid, tt.grid)
			}
		})
	}

}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
