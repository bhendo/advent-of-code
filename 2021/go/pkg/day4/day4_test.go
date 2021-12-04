package day4

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
	"strconv"
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	type args struct {
		numbers []int
		boards  []*BingoBoard
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "Empty",
			args: args{
				[]int{},
				[]*BingoBoard{},
			},
			wantOut: 0,
		},
		{
			name: "Example",
			args: args{
				[]int{7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1},
				[]*BingoBoard{
					&BingoBoard{

						{{Number: 22}, {Number: 13}, {Number: 17}, {Number: 11}, {Number: 0}},
						{{Number: 8}, {Number: 2}, {Number: 23}, {Number: 4}, {Number: 24}},
						{{Number: 21}, {Number: 9}, {Number: 14}, {Number: 16}, {Number: 7}},
						{{Number: 6}, {Number: 10}, {Number: 3}, {Number: 18}, {Number: 5}},
						{{Number: 1}, {Number: 12}, {Number: 20}, {Number: 15}, {Number: 19}},
					},
					&BingoBoard{

						{{Number: 3}, {Number: 15}, {Number: 0}, {Number: 2}, {Number: 22}},
						{{Number: 9}, {Number: 18}, {Number: 13}, {Number: 17}, {Number: 5}},
						{{Number: 19}, {Number: 8}, {Number: 7}, {Number: 25}, {Number: 23}},
						{{Number: 20}, {Number: 11}, {Number: 10}, {Number: 24}, {Number: 4}},
						{{Number: 14}, {Number: 21}, {Number: 16}, {Number: 12}, {Number: 6}},
					},
					&BingoBoard{

						{{Number: 14}, {Number: 21}, {Number: 17}, {Number: 24}, {Number: 4}},
						{{Number: 10}, {Number: 16}, {Number: 15}, {Number: 9}, {Number: 19}},
						{{Number: 18}, {Number: 8}, {Number: 23}, {Number: 26}, {Number: 20}},
						{{Number: 22}, {Number: 11}, {Number: 13}, {Number: 6}, {Number: 5}},
						{{Number: 2}, {Number: 0}, {Number: 12}, {Number: 3}, {Number: 7}},
					},
				},
			},
			wantOut: 4512,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := Part1(tt.args.numbers, tt.args.boards); gotOut != tt.wantOut {
				t.Errorf("Part1() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func Test_stampBoard(t *testing.T) {
	type args struct {
		number int
		board  *BingoBoard
	}
	tests := []struct {
		name string
		args args
		want *BingoBoard
	}{
		{
			name: "Simple",
			args: args{
				14,
				&BingoBoard{

					{{Number: 14}, {Number: 21}, {Number: 17}, {Number: 24}, {Number: 4}},
					{{Number: 10}, {Number: 16}, {Number: 15}, {Number: 9}, {Number: 19}},
					{{Number: 18}, {Number: 8}, {Number: 23}, {Number: 26}, {Number: 20}},
					{{Number: 22}, {Number: 11}, {Number: 13}, {Number: 6}, {Number: 5}},
					{{Number: 2}, {Number: 0}, {Number: 12}, {Number: 3}, {Number: 7}},
				},
			},
			want: &BingoBoard{

				{{Number: 14, Marked: true}, {Number: 21}, {Number: 17}, {Number: 24}, {Number: 4}},
				{{Number: 10}, {Number: 16}, {Number: 15}, {Number: 9}, {Number: 19}},
				{{Number: 18}, {Number: 8}, {Number: 23}, {Number: 26}, {Number: 20}},
				{{Number: 22}, {Number: 11}, {Number: 13}, {Number: 6}, {Number: 5}},
				{{Number: 2}, {Number: 0}, {Number: 12}, {Number: 3}, {Number: 7}},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := stampBoard(tt.args.number, tt.args.board); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("stampBoard() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_checkBoard(t *testing.T) {
	type args struct {
		bard *BingoBoard
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "None",
			args: args{
				&BingoBoard{

					{{Number: 14}, {Number: 21}, {Number: 17}, {Number: 24}, {Number: 4}},
					{{Number: 10}, {Number: 16}, {Number: 15}, {Number: 9}, {Number: 19}},
					{{Number: 18}, {Number: 8}, {Number: 23}, {Number: 26}, {Number: 20}},
					{{Number: 22}, {Number: 11}, {Number: 13}, {Number: 6}, {Number: 5}},
					{{Number: 2}, {Number: 0}, {Number: 12}, {Number: 3}, {Number: 7}},
				},
			},
			want: false,
		},
		{
			name: "Horizontal",
			args: args{
				&BingoBoard{

					{{Number: 14, Marked: true}, {Number: 21, Marked: true}, {Number: 17, Marked: true}, {Number: 24, Marked: true}, {Number: 4, Marked: true}},
					{{Number: 10}, {Number: 16}, {Number: 15}, {Number: 9}, {Number: 19}},
					{{Number: 18}, {Number: 8}, {Number: 23}, {Number: 26}, {Number: 20}},
					{{Number: 22}, {Number: 11}, {Number: 13}, {Number: 6}, {Number: 5}},
					{{Number: 2}, {Number: 0}, {Number: 12}, {Number: 3}, {Number: 7}},
				},
			},
			want: true,
		},
		{
			name: "None",
			args: args{
				&BingoBoard{

					{{Number: 14, Marked: true}, {Number: 21}, {Number: 17}, {Number: 24}, {Number: 4}},
					{{Number: 10, Marked: true}, {Number: 16}, {Number: 15}, {Number: 9}, {Number: 19}},
					{{Number: 18, Marked: true}, {Number: 8}, {Number: 23}, {Number: 26}, {Number: 20}},
					{{Number: 22, Marked: true}, {Number: 11}, {Number: 13}, {Number: 6}, {Number: 5}},
					{{Number: 2, Marked: true}, {Number: 0}, {Number: 12}, {Number: 3}, {Number: 7}},
				},
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := checkBoard(tt.args.bard); got != tt.want {
				t.Errorf("checkBoard() = %v, want %v", got, tt.want)
			}
		})
	}
}

func intCSVToSlice(in string) (out []int) {
	for _, s := range strings.Split(in, ",") {
		number, _ := strconv.Atoi(s)
		out = append(out, number)
	}
	return
}

func lineToRow(in string) (out [5]BingoSpace) {
	for i, s := range strings.Fields(in) {
		number, _ := strconv.Atoi(s)
		out[i] = BingoSpace{number, false}
	}
	return out
}

func helperGetData() (numbers []int, boards []*BingoBoard) {
	file, _ := os.Open("../../../_inputs/day4.txt")
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	if scanner.Scan() {
		numbers = intCSVToSlice(scanner.Text())
	}

	for scanner.Scan() {
		board := BingoBoard{}
		for i := 0; i < 5; i++ {
			scanner.Scan()
			row := lineToRow(scanner.Text())
			board[i] = row
		}
		boards = append(boards, &board)
	}

	return
}

func BenchmarkPart1(b *testing.B) {
	numbers, boards := helperGetData()
	for i := 0; i < b.N; i++ {
		Part1(numbers, boards)
	}
}

func BenchmarkPart2(b *testing.B) {
	numbers, boards := helperGetData()
	for i := 0; i < b.N; i++ {
		Part2(numbers, boards)
	}
}

func Example_answers() {
	numbers, boards := helperGetData()
	fmt.Println(Part1(numbers, boards), Part2(numbers, boards))
	// Output: 12796 18063
}
