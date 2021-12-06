package day6

type Tracker [9]int

func (t *Tracker) Advance() *Tracker {
	spawns := 0
	for i, val := range t {
		if i == 0 {
			spawns = val
		} else {
			t[i-1] = val
		}
	}
	t[6] += spawns
	t[8] = spawns
	return t
}

func (t *Tracker) Simulate(days int) (out int) {
	for day := 0; day < days; day++ {
		t.Advance()
	}
	for _, val := range t {
		out += val
	}
	return
}

func Part1(tracker *Tracker) (out int) {
	return tracker.Simulate(80)
}

func Part2(tracker *Tracker) (out int) {
	return tracker.Simulate(256)
}
