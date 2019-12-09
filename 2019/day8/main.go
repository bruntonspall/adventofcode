package main

import (
	"fmt"
	"strconv"

	"github.com/bruntonspall/adventofcode/pkg"
)

// CountOccurences counts the number of items in a list
func CountOccurences(data []int, search int) (count int) {
	count = 0
	for _, i := range data {
		if i == search {
			count++
		}
	}
	return
}

// Image holds pixel data for an image
type Image struct {
	layers [][]int
	width  int
	height int
}

// ParseImage parses an image file and break into layers
func ParseImage(image []int, width int, height int) Image {
	img := Image{nil, width, height}
	stride := width * height
	for i := 0; i < len(image); i += stride {
		layer := image[i : i+stride]
		img.layers = append(img.layers, layer)
	}

	return img
}

// GetPixel returns the visible pixel based on layers
func (img *Image) GetPixel(x int, y int) int {
	for _, layer := range img.layers {
		if layer[y*img.width+x] != 2 {
			return layer[y*img.width+x]
		}
	}
	return 0
}

// RenderToText returns a printable representation of the black and white bitmap
func (img *Image) RenderToText() []string {
	ret := make([]string, img.height)
	for y := 0; y < img.height; y++ {
		s := ""
		for x := 0; x < img.width; x++ {
			switch img.GetPixel(x, y) {
			case 0:
				s += " "
			case 1:
				s += "X"
			}
		}
		ret[y] = s
	}
	return ret
}

// returns part1 and part2
func run(input string) (part1 string, part2 string) {
	// Parse input and return output
	imgdata := make([]int, len(input))
	for i, r := range input {
		imgdata[i], _ = strconv.Atoi(string(r))
	}
	image := ParseImage(imgdata, 25, 6)
	min := 99999
	var minLayer []int
	for _, layer := range image.layers {
		count := CountOccurences(layer, 0)
		if count < min {
			minLayer = layer
			min = count
		}
	}
	part1 = fmt.Sprintf("%d", CountOccurences(minLayer, 1)*CountOccurences(minLayer, 2))
	// Parse input and return output
	out2 := image.RenderToText()
	for _, line := range out2 {
		fmt.Printf("%v\n", line)
	}
	part2 = "JAFRA"
	return
}

func main() {
	pkg.Execute(run, nil, puzzle, true)
}
