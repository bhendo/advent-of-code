package day1

import (
	"aoc2021/pkg/lib"
	"fmt"
	"os"
	"testing"
)

var (
	example = []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}
)

func Test_part1(t *testing.T) {
	type args struct {
		in []int
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "NoIncrease",
			args: args{
				in: []int{},
			},
			wantOut: 0,
		},
		{
			name: "Example",
			args: args{
				in: example,
			},
			wantOut: 7,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := part1(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("part1() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func Test_part2(t *testing.T) {
	type args struct {
		in []int
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "NoIncrease",
			args: args{
				in: []int{},
			},
			wantOut: 0,
		},
		{
			name: "Example",
			args: args{
				in: []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263},
			},
			wantOut: 5,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := part2(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("part2() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func Benchmark_part1(b *testing.B) {
	file, _ := os.Open("input.txt")
	data, _ := lib.ReadInts(file)
	for i := 0; i < b.N; i++ {
		part1(data)
	}
}

func Benchmark_part2(b *testing.B) {
	file, _ := os.Open("input.txt")
	data, _ := lib.ReadInts(file)
	for i := 0; i < b.N; i++ {
		part2(data)
	}
}

func Example_Answers() {
	file, _ := os.Open("input.txt")
	data, _ := lib.ReadInts(file)
	fmt.Println(part1(data), part2(data))
	// Output: 1559 1600
}
