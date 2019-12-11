package pkg

import "math"

// Coord2D is an 2 dimensional coordinate in space
type Coord2D struct {
	X, Y int
}

func (pt1 Coord2D) DistanceTo(pt2 Coord2D) float64 {
	return math.Sqrt(math.Pow(float64(pt1.X-pt2.X), 2) + math.Pow(float64(pt1.Y-pt2.Y), 2))
}

type Line struct {
	pt1, pt2   Coord2D
	slope      float64
	yintercept float64
}

func NewLine(pt1 Coord2D, pt2 Coord2D) Line {
	l := Line{pt1, pt2, 0, 0}
	dy := l.pt2.Y - l.pt1.Y
	dx := l.pt2.X - l.pt1.X
	l.slope = math.NaN()
	if dx != 0 {
		l.slope = float64(dy) / float64(dx)
		l.yintercept = float64(pt2.Y) - (l.slope * float64(pt2.X))
	} else {
		l.yintercept = float64(pt2.X)
	}
	return l
}

func (l *Line) HasPoint(pt Coord2D) bool {
	// Check pt is inside bounding box of the end points
	if pt.X < l.pt1.X && pt.X < l.pt2.X {
		return false
	}
	if pt.X > l.pt1.X && pt.X > l.pt2.X {
		return false
	}
	if pt.Y < l.pt1.Y && pt.Y < l.pt2.Y {
		return false
	}
	if pt.Y > l.pt1.Y && pt.Y > l.pt2.Y {
		return false
	}

	if math.IsNaN(l.slope) {
		return float64(pt.X) == l.yintercept
	}
	return float64(pt.Y) == (l.slope*float64(pt.X))+l.yintercept
}
