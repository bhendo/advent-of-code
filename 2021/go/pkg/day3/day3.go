package day3

import (
	"strconv"
)

func bitCountsAtPos(in []string, pos int) (b0, b1 int) {
	for _, s := range in {
		switch s[pos : pos+1] {
		case "0":
			b0++
		case "1":
			b1++
		}
	}
	return
}

func commonBits(in []string) (most, least string) {
	if len(in) < 1 {
		return
	}

	for i := range in[0] {
		b0, b1 := bitCountsAtPos(in, i)
		if b0 <= b1 {
			most += "1"
			least += "0"
		} else {
			most += "0"
			least += "1"
		}
	}

	return
}

func filter(in []string, bit string, pos int) (out []string) {
	for _, bits := range in {
		if bits[pos:pos+1] == bit {
			out = append(out, bits)
		}
	}
	return
}

func ratings(in []string, mostCommon bool, pos int) (out string) {
	if len(in) == 1 || pos >= len(in[0]) {
		return in[0]
	}
	b0, b1 := bitCountsAtPos(in, pos)
	filterBit := ""
	if mostCommon {
		if b1 >= b0 {
			filterBit = "1"
		} else {
			filterBit = "0"
		}
	} else {
		if b1 >= b0 {
			filterBit = "0"
		} else {
			filterBit = "1"
		}
	}

	in = filter(in, filterBit, pos)

	pos++
	return ratings(in, mostCommon, pos)
}

func Part1(in []string) (out int) {
	if len(in) < 1 {
		return out
	}
	most, least := commonBits(in)

	gammaRate, _ := strconv.ParseInt(most, 2, 64)
	epsilonRate, _ := strconv.ParseInt(least, 2, 64)

	return int(gammaRate) * int(epsilonRate)
}

func Part2(in []string) (out int) {
	if len(in) < 1 {
		return out
	}

	ogr, _ := strconv.ParseInt(ratings(in, true, 0), 2, 64)
	csr, _ := strconv.ParseInt(ratings(in, false, 0), 2, 64)

	return int(ogr) * int(csr)
}
