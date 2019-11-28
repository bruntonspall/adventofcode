package pkg

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestParse(t *testing.T) {
	cases := []struct {
		in  string
		sep string
		out []int
	}{
		{`1, 2, 3, 4`, ",", []int{1, 2, 3, 4}},
		{`+1, +2, +3, +4`, ",", []int{1, 2, 3, 4}},
		{"-1\n-2\n-3\n-4", "\n", []int{-1, -2, -3, -4}},
		{"1\n2\n3\n4", "\n", []int{1, 2, 3, 4}},
	}
	for _, tt := range cases {
		t.Run(tt.in, func(t *testing.T) {
			s := Parse(tt.in, tt.sep)
			if !cmp.Equal(s, tt.out) {
				t.Errorf("got %q, want %q", s, tt.out)
			}
		})
	}
}
