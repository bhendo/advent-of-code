package day1

import (
	"bufio"
	"io"
	"strconv"
)

func sumInts(in []int) (out int) {
	for _, v := range in {
		out += v
	}
	return
}

func intsFromReader(r io.Reader) ([]int, error) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanWords)
	out := []int{}
	for scanner.Scan() {
		x, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return out, err
		}
		out = append(out, x)
	}
	return out, scanner.Err()
}

func part1(in []int) (out int) {
	if len(in) < 2 {
		return
	}
	for i := range in {
		if i > 0 && in[i-1] < in[i] {
			out++
		}
	}
	return
}

func part2(in []int) (out int) {
	if len(in) < 4 {
		return
	}
	for i := range in {
		if i > 0 && i+3 <= len(in) && sumInts(in[i-1:i+2]) < sumInts(in[i:i+3]) {
			out++
		}
	}
	return
}
