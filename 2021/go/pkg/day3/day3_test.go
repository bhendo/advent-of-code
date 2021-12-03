package day3

import (
	"bufio"
	"fmt"
	"os"
	"testing"
)

func TestPart1(t *testing.T) {
	type args struct {
		in []string
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name:    "empty",
			args:    args{in: []string{}},
			wantOut: 0,
		},
		{
			name:    "example",
			args:    args{in: []string{"00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"}},
			wantOut: 198,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := Part1(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("Part1() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	type args struct {
		in []string
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name:    "empty",
			args:    args{in: []string{}},
			wantOut: 0,
		},
		{
			name:    "example",
			args:    args{in: []string{"00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"}},
			wantOut: 230,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := Part2(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("Part2() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func helperGetData() (data []string) {
	file, _ := os.Open("../../../_inputs/day3.txt")
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		data = append(data, scanner.Text())
	}
	return
}

func Example_answers() {
	data := helperGetData()
	fmt.Println(Part1(data), Part2(data))
	// Output: 1092896 4672151
}

func BenchmarkPart1(b *testing.B) {
	data := helperGetData()
	for i := 0; i < b.N; i++ {
		Part1(data)
	}
}

func BenchmarkPart2(b *testing.B) {
	data := helperGetData()
	for i := 0; i < b.N; i++ {
		Part2(data)
	}
}
