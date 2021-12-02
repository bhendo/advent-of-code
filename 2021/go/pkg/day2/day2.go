package day2

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type Command struct {
	Direction string
	Units     int
}

func commandFromString(in string) (Command, error) {
	fields := strings.Fields(in)
	if len(fields) != 2 {
		return Command{}, fmt.Errorf("invalid format")
	}

	direction := fields[0]
	if units, err := strconv.Atoi(fields[1]); err == nil {
		return Command{direction, units}, nil
	} else {
		return Command{}, err
	}
}

func commandsFromFile(r io.Reader) ([]Command, error) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanLines)
	out := []Command{}
	for scanner.Scan() {
		c, err := commandFromString(scanner.Text())
		if err != nil {
			return out, err
		}
		out = append(out, c)
	}
	return out, nil
}

func Part1(in []Command) (out int) {
	var horizontal, depth int
	for _, command := range in {
		switch command.Direction {
		case "forward":
			horizontal += command.Units
		case "up":
			depth -= command.Units
		case "down":
			depth += command.Units
		}
	}
	return horizontal * depth
}

func Part2(in []Command) (out int) {
	var horizontal, depth, aim int
	for _, command := range in {
		switch command.Direction {
		case "forward":
			horizontal += command.Units
			depth += (aim * command.Units)
		case "up":
			aim -= command.Units
		case "down":
			aim += command.Units
		}
	}
	return horizontal * depth
}
