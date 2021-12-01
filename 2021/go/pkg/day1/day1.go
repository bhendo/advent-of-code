package day1

import "aoc2021/pkg/lib"

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
		if i > 0 && i+3 <= len(in) && lib.SumInts(in[i-1:i+2]) < lib.SumInts(in[i:i+3]) {
			out++
		}
	}
	return
}
