package day5

type Point struct {
	X, Y int
}

type Line struct {
	Start, End Point
}

func (l Line) IsOrthogonal() bool {
	if l.Start.X == l.End.X || l.Start.Y == l.End.Y {
		return true
	}
	return false
}

func (l Line) Points() []Point {
	points := []Point{}
	if l.Start.X == l.End.X { // vertical
		if l.Start.Y > l.End.Y {
			for y := l.End.Y; y <= l.Start.Y; y++ {
				points = append(points, Point{l.Start.X, y})
			}
		} else {
			for y := l.Start.Y; y <= l.End.Y; y++ {
				points = append(points, Point{l.Start.X, y})
			}
		}
	} else { // horizontal and 45 degree diagonals only
		m := (l.End.Y - l.Start.Y) / (l.End.X - l.Start.X)
		b := l.Start.Y - (m * l.Start.X)
		if l.Start.X > l.End.X {
			for x := l.End.X; x <= l.Start.X; x++ {
				y := (m * x) + b
				points = append(points, Point{x, y})
			}
		} else {
			for x := l.Start.X; x <= l.End.X; x++ {
				y := (m * x) + b
				points = append(points, Point{x, y})
			}
		}
	}
	return points
}

func Part1(lines []Line) (out int) {
	ventMap := map[Point]int{}
	for _, line := range lines {
		if line.IsOrthogonal() {
			for _, point := range line.Points() {
				if val, ok := ventMap[point]; ok {
					ventMap[point] = val + 1
				} else {
					ventMap[point] = 1
				}
			}
		}
	}
	for _, val := range ventMap {
		if val > 1 {
			out++
		}
	}
	return
}

func Part2(lines []Line) (out int) {
	ventMap := map[Point]int{}
	for _, line := range lines {
		for _, point := range line.Points() {
			if val, ok := ventMap[point]; ok {
				ventMap[point] = val + 1
			} else {
				ventMap[point] = 1
			}
		}
	}
	for _, val := range ventMap {
		if val > 1 {
			out++
		}
	}
	return
}
