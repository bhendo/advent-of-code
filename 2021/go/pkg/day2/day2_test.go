package day2

import (
	"fmt"
	"io"
	"os"
	"reflect"
	"strings"
	"testing"
)

func Test_commandsFromReader(t *testing.T) {
	type args struct {
		r io.Reader
	}
	tests := []struct {
		name    string
		args    args
		want    []Command
		wantErr bool
	}{
		{
			name: "Empty",
			args: args{
				r: strings.NewReader(""),
			},
			want:    []Command{},
			wantErr: false,
		},
		{
			name: "Simple",
			args: args{
				r: strings.NewReader("forward 1\nup 2\ndown 3"),
			},
			want: []Command{
				{"forward", 1},
				{"up", 2},
				{"down", 3},
			},
			wantErr: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := commandsFromReader(tt.args.r)
			if (err != nil) != tt.wantErr {
				t.Errorf("commandsFromReader() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("commandsFromReader() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_part1(t *testing.T) {
	type args struct {
		in []Command
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "None",
			args: args{
				in: []Command{},
			},
			wantOut: 0,
		},
		{
			name: "Example",
			args: args{
				in: []Command{
					{"forward", 5},
					{"down", 5},
					{"forward", 8},
					{"up", 3},
					{"down", 8},
					{"forward", 2},
				},
			},
			wantOut: 150,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := Part1(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("part1() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	type args struct {
		in []Command
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "None",
			args: args{
				in: []Command{},
			},
			wantOut: 0,
		},
		{
			name: "Example",
			args: args{
				in: []Command{
					{"forward", 5},
					{"down", 5},
					{"forward", 8},
					{"up", 3},
					{"down", 8},
					{"forward", 2},
				},
			},
			wantOut: 900,
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

func Benchmark_part1(b *testing.B) {
	data := make([]Command, 2000)
	for i := 0; i < b.N; i++ {
		Part1(data)
	}
}

func Benchmark_part2(b *testing.B) {
	data := make([]Command, 2000)
	for i := 0; i < b.N; i++ {
		Part2(data)
	}
}

func Example_answers() {
	file, _ := os.Open("../../../_inputs/day2.txt")
	data, _ := commandsFromReader(file)
	fmt.Println(Part1(data), Part2(data))
	// Output: 1727835 1544000595
}
