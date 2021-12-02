package day1

import (
	"fmt"
	"io"
	"os"
	"reflect"
	"strings"
	"testing"
)

func Test_sumInts(t *testing.T) {
	type args struct {
		in []int
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "Zero",
			args: args{
				in: []int{},
			},
			wantOut: 0,
		},
		{
			name: "Simple",
			args: args{
				in: []int{1, 2, 3},
			},
			wantOut: 6,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := sumInts(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("sumInts() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func Test_intsFromReader(t *testing.T) {
	type args struct {
		r io.Reader
	}
	tests := []struct {
		name    string
		args    args
		want    []int
		wantErr bool
	}{
		{
			name: "Empty",
			args: args{
				r: strings.NewReader(""),
			},
			want:    []int{},
			wantErr: false,
		},
		{
			name: "NewLine",
			args: args{
				r: strings.NewReader("1\n2\n3\n"),
			},
			want:    []int{1, 2, 3},
			wantErr: false,
		},
		{
			name: "Spaces",
			args: args{
				r: strings.NewReader("1 2 3"),
			},
			want:    []int{1, 2, 3},
			wantErr: false,
		},
		{
			name: "Tabs",
			args: args{
				r: strings.NewReader("1\t2\t3"),
			},
			want:    []int{1, 2, 3},
			wantErr: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := intsFromReader(tt.args.r)
			if (err != nil) != tt.wantErr {
				t.Errorf("intsFromReader() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("intsFromReader() = %v, want %v", got, tt.want)
			}
		})
	}
}

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
				in: []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263},
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
	data := make([]int, 2000)
	for i := 0; i < b.N; i++ {
		part1(data)
	}
}

func Benchmark_part2(b *testing.B) {
	data := make([]int, 2000)
	for i := 0; i < b.N; i++ {
		part2(data)
	}
}

func Example_Answers() {
	file, _ := os.Open("../../../_inputs/day1.txt")
	data, _ := intsFromReader(file)
	fmt.Println(part1(data), part2(data))
	// Output: 1559 1600
}
