use aoc_runner_derive::aoc;
use std::collections::VecDeque;

#[derive(Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test_div: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for block in input.split("\n\n") {
        let lines: Vec<_> = block.lines().collect();
        let items = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let op_line = lines[2].split_once("= ").unwrap().1;
        let op = if op_line == "old * old" {
            Op::Square
        } else if let Some(v) = op_line.strip_prefix("old * ") {
            Op::Mul(v.parse().unwrap())
        } else if let Some(v) = op_line.strip_prefix("old + ") {
            Op::Add(v.parse().unwrap())
        } else {
            panic!("Unknown op: {op_line}");
        };
        let test_div = lines[3].split_whitespace().last().unwrap().parse().unwrap();
        let if_true = lines[4].split_whitespace().last().unwrap().parse().unwrap();
        let if_false = lines[5].split_whitespace().last().unwrap().parse().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            test_div,
            if_true,
            if_false,
            inspections: 0,
        });
    }
    monkeys
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut monkeys = parse(input);

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                monkeys[i].inspections += 1;
                item = match monkeys[i].op {
                    Op::Add(v) => item + v,
                    Op::Mul(v) => item * v,
                    Op::Square => item * item,
                };
                item /= 3;
                let target = if item % monkeys[i].test_div == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut monkeys = parse(input);

    // Product of all test divisors
    let modulo: u64 = monkeys.iter().map(|m| m.test_div).product();

    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                monkeys[i].inspections += 1;
                item = match monkeys[i].op {
                    Op::Add(v) => item + v,
                    Op::Mul(v) => item * v,
                    Op::Square => item * item,
                };
                item %= modulo;
                let target = if item % monkeys[i].test_div == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Monkey 0:
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
            If false: throw to monkey 1
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2713310158);
    }
}
