package day6

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

func TestTracker_Advance(t *testing.T) {
	tests := []struct {
		name string
		tr   *Tracker
		want *Tracker
	}{
		{
			name: "simple",
			tr:   &Tracker{2, 0, 0, 0, 0, 0, 0, 1, 0},
			want: &Tracker{0, 0, 0, 0, 0, 0, 3, 0, 2},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.tr.Advance(); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Tracker.Advance() = %v, want %v", got, tt.want)
			}
		})
	}
}

func toTracker(r io.Reader) *Tracker {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)
	tracker := Tracker{}
	if scanner.Scan() {
		all := strings.Split(scanner.Text(), ",")
		for _, value := range all {
			pos, _ := strconv.Atoi(value)
			tracker[pos]++
		}

	}
	return &tracker
}

func Test_toTracker(t *testing.T) {
	type args struct {
		r io.Reader
	}
	tests := []struct {
		name string
		args args
		want Tracker
	}{
		{
			name: "Example",
			args: args{
				r: strings.NewReader("3,4,3,1,2"),
			},
			want: Tracker{0, 1, 1, 2, 1, 0, 0, 0, 0},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := toTracker(tt.args.r); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("toTracker() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestTracker_Simulate(t *testing.T) {
	type args struct {
		days int
	}
	tests := []struct {
		name    string
		tr      *Tracker
		args    args
		wantOut int
	}{
		{
			name: "Example-18Days",
			tr:   toTracker(strings.NewReader("3,4,3,1,2")),
			args: args{
				days: 18,
			},
			wantOut: 26,
		},
		{
			name: "Example-80Days",
			tr:   toTracker(strings.NewReader("3,4,3,1,2")),
			args: args{

				days: 80,
			},
			wantOut: 5934,
		},
		{
			name: "Example-80Days",
			tr:   toTracker(strings.NewReader("3,4,3,1,2")),
			args: args{

				days: 256,
			},
			wantOut: 26984457539,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := tt.tr.Simulate(tt.args.days); gotOut != tt.wantOut {
				t.Errorf("Tracker.Simulate() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func helperGetData() *Tracker {
	file, _ := os.Open("../../../_inputs/day6.txt")
	return toTracker(file)
}

func BenchmarkTracker_Simulate(b *testing.B) {
	tracker := helperGetData()
	for i := 0; i < b.N; i++ {
		tracker.Simulate(256)
	}
}

func Example_answers() {
	fmt.Println(Part1(helperGetData()), Part2(helperGetData()))
	// Output: 391671 1754000560399
}
