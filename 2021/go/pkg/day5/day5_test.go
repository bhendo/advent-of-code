package day5

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"reflect"
	"strconv"
	"strings"
	"testing"
)

var ExampleData = []Line{
	{Start: Point{X: 0, Y: 9}, End: Point{X: 5, Y: 9}},
	{Start: Point{X: 8, Y: 0}, End: Point{X: 0, Y: 8}},
	{Start: Point{X: 9, Y: 4}, End: Point{X: 3, Y: 4}},
	{Start: Point{X: 2, Y: 2}, End: Point{X: 2, Y: 1}},
	{Start: Point{X: 7, Y: 0}, End: Point{X: 7, Y: 4}},
	{Start: Point{X: 6, Y: 4}, End: Point{X: 2, Y: 0}},
	{Start: Point{X: 0, Y: 9}, End: Point{X: 2, Y: 9}},
	{Start: Point{X: 3, Y: 4}, End: Point{X: 1, Y: 4}},
	{Start: Point{X: 0, Y: 0}, End: Point{X: 8, Y: 8}},
	{Start: Point{X: 5, Y: 5}, End: Point{X: 8, Y: 2}},
}

func TestLine_IsOrthogonal(t *testing.T) {
	type fields struct {
		Start Point
		End   Point
	}
	tests := []struct {
		name   string
		fields fields
		want   bool
	}{
		{
			name: "Vertical",
			fields: fields{
				Start: Point{1, 1},
				End:   Point{1, 2},
			},
			want: true,
		},
		{
			name: "Horizontal",
			fields: fields{
				Start: Point{1, 1},
				End:   Point{2, 1},
			},
			want: true,
		},
		{
			name: "Diagonal",
			fields: fields{
				Start: Point{1, 1},
				End:   Point{2, 1},
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			l := Line{
				Start: tt.fields.Start,
				End:   tt.fields.End,
			}
			if got := l.IsOrthogonal(); got != tt.want {
				t.Errorf("Line.IsOrthogonal() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestPart1(t *testing.T) {
	type args struct {
		lines []Line
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "none",
			args: args{lines: []Line{}},
			want: 0,
		},
		{
			name: "example",
			args: args{lines: ExampleData},
			want: 5,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Part1(tt.args.lines); got != tt.want {
				t.Errorf("Part1() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLine_Points(t *testing.T) {
	type fields struct {
		Start Point
		End   Point
	}
	tests := []struct {
		name   string
		fields fields
		want   []Point
	}{
		{
			name:   "OnePoint",
			fields: fields{Start: Point{0, 0}, End: Point{0, 0}},
			want:   []Point{{0, 0}},
		},
		{
			name:   "Horizontal",
			fields: fields{Start: Point{0, 0}, End: Point{2, 0}},
			want:   []Point{{0, 0}, {1, 0}, {2, 0}},
		},
		{
			name:   "Vertical",
			fields: fields{Start: Point{0, 0}, End: Point{0, 2}},
			want:   []Point{{0, 0}, {0, 1}, {0, 2}},
		},
		{
			name:   "Diagonal",
			fields: fields{Start: Point{0, 0}, End: Point{2, 2}},
			want:   []Point{{0, 0}, {1, 1}, {2, 2}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			l := Line{
				Start: tt.fields.Start,
				End:   tt.fields.End,
			}
			if got := l.Points(); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Line.Points() = %v, want %v", got, tt.want)
			}
		})
	}
}

func parseLine(in string) Line {
	startAndEnd := strings.Split(in, " -> ")
	startXAndY := strings.Split(startAndEnd[0], ",")
	endXAndY := strings.Split(startAndEnd[1], ",")

	startX, _ := strconv.Atoi(startXAndY[0])
	startY, _ := strconv.Atoi(startXAndY[1])

	endX, _ := strconv.Atoi(endXAndY[0])
	endY, _ := strconv.Atoi(endXAndY[1])

	return Line{Start: Point{X: startX, Y: startY}, End: Point{X: endX, Y: endY}}
}

func scanLines(r io.Reader) []Line {
	lines := []Line{}
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		lines = append(lines, parseLine(scanner.Text()))
	}
	return lines
}

func Test_scanLines(t *testing.T) {
	type args struct {
		r io.Reader
	}
	tests := []struct {
		name string
		args args
		want []Line
	}{
		{
			name: "None",
			args: args{r: strings.NewReader(``)},
			want: []Line{},
		},
		{
			name: "Example",
			args: args{r: strings.NewReader(`0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`)},
			want: ExampleData,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotLines := scanLines(tt.args.r); !reflect.DeepEqual(gotLines, tt.want) {
				t.Errorf("scanLines() = %v, want %v", gotLines, tt.want)
			}
		})
	}
}

func helperGetData() []Line {
	file, _ := os.Open("../../../_inputs/day5.txt")
	return scanLines(file)
}

func BenchmarkPart1(b *testing.B) {
	lines := helperGetData()
	for i := 0; i < b.N; i++ {
		Part1(lines)
	}
}

func BenchmarkPart2(b *testing.B) {
	lines := helperGetData()
	for i := 0; i < b.N; i++ {
		Part2(lines)
	}
}

func Example_answers() {
	lines := helperGetData()
	fmt.Println(Part1(lines), Part2(lines))
	// Output: 6189 19164
}
