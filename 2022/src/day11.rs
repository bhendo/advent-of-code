use std::collections::HashMap;

enum Operator {
    Multiply,
    Add,
}

struct Operation {
    operator: Operator,
    value: i64,
}
struct Test {
    divisible: i64,
    if_true: usize,
    if_false: usize,
}
pub struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn inspect_and_throw<WR>(&mut self, worry_reducer: WR) -> Vec<(usize, i64)>
    where
        WR: Fn(i64) -> i64,
    {
        self.items
            .drain(..)
            .map(|item| {
                let i = worry_reducer(match self.operation.operator {
                    Operator::Add => {
                        if self.operation.value == -1 {
                            item + item
                        } else {
                            item + self.operation.value
                        }
                    }
                    Operator::Multiply => {
                        if self.operation.value == -1 {
                            item * item
                        } else {
                            item * self.operation.value
                        }
                    }
                });
                let m = match (i % self.test.divisible) == 0 {
                    true => self.test.if_true,
                    false => self.test.if_false,
                };
                (m, i)
            })
            .collect()
    }

    pub fn catch(&mut self, item: i64) {
        self.items.push(item);
    }
}

pub fn part1(mut monkeys: Vec<Monkey>) -> i64 {
    let mut monkey_inspections: HashMap<usize, i64> = HashMap::new();

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            monkey_inspections
                .entry(m)
                .and_modify(|e| *e += monkeys[m].items.len() as i64)
                .or_insert(monkeys[m].items.len() as i64);
            let items_in_flight = monkeys[m].inspect_and_throw(|n| n / 3);
            for i in 0..items_in_flight.len() {
                let (m, item) = items_in_flight[i];
                monkeys[m].catch(item);
            }
        }
    }
    let mut inspection_counts = monkey_inspections.values().collect::<Vec<&i64>>();
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

pub fn part2(mut monkeys: Vec<Monkey>) -> i64 {
    let mut monkey_inspections: HashMap<usize, i64> = HashMap::new();

    let lcd: i64 = monkeys.iter().map(|m| m.test.divisible).product();
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            monkey_inspections
                .entry(m)
                .and_modify(|e| *e += monkeys[m].items.len() as i64)
                .or_insert(monkeys[m].items.len() as i64);
            let items_in_flight = monkeys[m].inspect_and_throw(|n: i64| n % lcd);
            for i in 0..items_in_flight.len() {
                let (m, item) = items_in_flight[i];
                monkeys[m].catch(item);
            }
        }
    }
    let mut inspection_counts = monkey_inspections.values().collect::<Vec<&i64>>();
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let l = m.split("\n").collect::<Vec<&str>>();
            let items = l[1]
                .split(": ")
                .last()
                .map(|items| {
                    items
                        .split(", ")
                        .map(|i| i.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .unwrap();
            let operation = l[2]
                .split("Operation: new = old ")
                .last()
                .map(|op| {
                    op.split_once(" ")
                        .and_then(|(operator, value)| {
                            let o = match operator {
                                "*" => Operator::Multiply,
                                _ => Operator::Add,
                            };
                            let v = value.parse::<i64>().unwrap_or(-1);
                            Some(Operation {
                                operator: o,
                                value: v,
                            })
                        })
                        .unwrap()
                })
                .unwrap();
            let divisible = l[3]
                .split("Test: divisible by ")
                .last()
                .map(|d| d.parse::<i64>().unwrap())
                .unwrap();
            let t = l[4]
                .split("If true: throw to monkey ")
                .last()
                .map(|m| m.parse::<usize>().unwrap())
                .unwrap();
            let f = l[5]
                .split("If false: throw to monkey ")
                .last()
                .map(|m| m.parse::<usize>().unwrap())
                .unwrap();
            Monkey {
                items: items,
                operation: operation,
                test: Test {
                    divisible: divisible,
                    if_true: t,
                    if_false: f,
                },
            }
        })
        .collect::<Vec<Monkey>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 10605);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day11.txt"))),
            316888
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 2713310158);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day11.txt"))),
            35270398814
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE)))
    }
}
