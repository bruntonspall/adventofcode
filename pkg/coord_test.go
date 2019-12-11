package pkg

import (
	"fmt"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestPointOnLine(t *testing.T) {
	cases := []struct {
		pt1 Coord2D
		pt2 Coord2D
		pt3 Coord2D
		out bool
	}{
		// Horizontal line
		{Coord2D{1, 1}, Coord2D{3, 1}, Coord2D{2, 1}, true},
		{Coord2D{1, 1}, Coord2D{3, 1}, Coord2D{4, 1}, false},
		{Coord2D{1, 1}, Coord2D{3, 1}, Coord2D{4, 4}, false},
		// Vertical line
		{Coord2D{1, 1}, Coord2D{1, 3}, Coord2D{1, 2}, true},
		{Coord2D{1, 1}, Coord2D{1, 3}, Coord2D{1, 4}, false},
		{Coord2D{1, 1}, Coord2D{1, 3}, Coord2D{4, 4}, false},

		// Diagonal line, midpoint
		{Coord2D{1, 1}, Coord2D{3, 3}, Coord2D{2, 2}, true},
		// Diagonial line, extends beyond
		{Coord2D{1, 1}, Coord2D{3, 3}, Coord2D{4, 4}, false},
		// Diagonal line, off the line
		{Coord2D{1, 1}, Coord2D{3, 3}, Coord2D{1, 2}, false},
		// Examples from the map
		{Coord2D{3, 4}, Coord2D{1, 0}, Coord2D{2, 2}, true},
		{Coord2D{3, 4}, Coord2D{4, 0}, Coord2D{4, 3}, false},
		{Coord2D{3, 4}, Coord2D{4, 0}, Coord2D{3, 3}, false},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt), func(t *testing.T) {
			line := NewLine(tt.pt1, tt.pt2)
			out := line.HasPoint(tt.pt3)
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}

}

func TestDistanceTo(t *testing.T) {
	cases := []struct {
		pt1 Coord2D
		pt2 Coord2D
		out float64
	}{
		// Horizontal line
		{Coord2D{0, 0}, Coord2D{3, 0}, 3.0},
		// Vertical line
		{Coord2D{0, 0}, Coord2D{0, 3}, 3.0},
		// Diagonals
		{Coord2D{0, 0}, Coord2D{4, 3}, 5.0},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt), func(t *testing.T) {
			out := tt.pt1.DistanceTo(tt.pt2)
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}

}
