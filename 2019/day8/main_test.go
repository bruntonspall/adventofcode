package main

import (
	"fmt"
	"testing"

	"github.com/bruntonspall/adventofcode/pkg"
	"github.com/google/go-cmp/cmp"
)

func TestCount(t *testing.T) {
	cases := []struct {
		data   []int
		search int
		qty    int
	}{
		{[]int{4, 1, 4, 2, 2, 4, 3, 3, 3, 4}, 1, 1},
		{[]int{4, 1, 4, 2, 2, 4, 3, 3, 3, 4}, 2, 2},
		{[]int{4, 1, 4, 2, 2, 4, 3, 3, 3, 4}, 3, 3},
		{[]int{4, 1, 4, 2, 2, 4, 3, 3, 3, 4}, 4, 4},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.search), func(t *testing.T) {
			out := CountOccurences(tt.data, tt.search)
			if !cmp.Equal(out, tt.qty) {
				t.Errorf("got %v, want %v", out, tt.qty)
			}
		})
	}

}

func TestImageToLayer(t *testing.T) {
	cases := []struct {
		image  []int
		width  int
		height int
		layers [][]int
	}{
		{[]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12}, 3, 2, [][]int{[]int{1, 2, 3, 4, 5, 6}, []int{7, 8, 9, 10, 11, 12}}},
	}

	for _, tt := range cases {
		t.Run(fmt.Sprintf("%v", tt.image), func(t *testing.T) {
			layers := ParseImage(tt.image, tt.width, tt.height)
			if !cmp.Equal(layers.layers, tt.layers) {
				t.Errorf("got %v, want %v", layers, tt.layers)
			}
		})
	}

}

func TestGetVisiblePixel(t *testing.T) {
	cases := []struct {
		x     int
		y     int
		pixel int
	}{
		{0, 0, 0},
		{1, 0, 1},
		{0, 1, 1},
		{1, 1, 0},
	}

	for _, tt := range cases {
		img := ParseImage([]int{0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0}, 2, 2)
		t.Run(fmt.Sprintf("%v%v", tt.x, tt.y), func(t *testing.T) {
			pixel := img.GetPixel(tt.x, tt.y)
			if !cmp.Equal(pixel, tt.pixel) {
				t.Errorf("got %v, want %v", pixel, tt.pixel)
			}
		})
	}

}

func TestRenderImage(t *testing.T) {
	cases := []struct {
		img []int
		out []string
	}{
		{[]int{0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0}, []string{" X", "X "}},
		{[]int{1, 1, 2, 2, 1, 1, 0, 0}, []string{"XX", "  "}},
	}

	for _, tt := range cases {
		img := ParseImage(tt.img, 2, 2)
		t.Run(fmt.Sprintf("%v", tt.img), func(t *testing.T) {
			out := img.RenderToText()
			if !cmp.Equal(out, tt.out) {
				t.Errorf("got %v, want %v", out, tt.out)
			}
		})
	}

}

var tests = pkg.TestCases{
	{
		puzzle,
		"1806",
		"JAFRA",
	},
}

func TestMain(t *testing.T) {
	tests.Run2(run, t, false)
}
