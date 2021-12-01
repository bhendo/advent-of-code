package lib

import (
	"bufio"
	"io"
	"strconv"
)

func SumInts(in []int) (out int) {
	for _, v := range in {
		out += v
	}
	return
}

func ReadInts(r io.Reader) ([]int, error) {
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
