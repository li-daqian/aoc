use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn accept(workflows: &HashMap<&str, Vec<&str>>, input: &str, states: &Vec<usize>) -> bool {
    if input == "A" || input == "R" {
        return input == "A";
    }
    for rule in workflows.get(input).unwrap().iter() {
        if let Some((rule, target)) = rule.split_once(':') {
            let (left, op, right) = (
                &rule[0..1],
                &rule[1..2],
                rule[2..].parse::<usize>().unwrap(),
            );
            let left = match left {
                "x" => states[0],
                "m" => states[1],
                "a" => states[2],
                "s" => states[3],
                _ => unreachable!(),
            };
            match op {
                "<" => {
                    if left < right {
                        return accept(workflows, target, states);
                    }
                }
                ">" => {
                    if left > right {
                        return accept(workflows, target, states);
                    }
                }
                _ => unreachable!(),
            };
        } else {
            return accept(workflows, rule, states);
        }
    }

    false
}

fn count(
    workflows: &HashMap<&str, Vec<&str>>,
    input: &str,
    ranges: &mut Vec<(usize, usize)>,
) -> usize {
    if input == "R" {
        return 0;
    }
    if input == "A" {
        return ranges.iter().fold(1, |mut product, (low, high)| {
            product *= high - low + 1;
            product
        });
    }

    workflows
        .get(input)
        .unwrap()
        .iter()
        .map(|rule| {
            if let Some((rule, target)) = rule.split_once(':') {
                let mut total = 0;
                let (left, op, right) = (
                    &rule[0..1],
                    &rule[1..2],
                    rule[2..].parse::<usize>().unwrap(),
                );
                let range_index = match left {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => unreachable!(),
                };
                let (low, high) = ranges[range_index];
                let matches = match op {
                    "<" => [(low, high.min(right - 1)), (low.max(right), high)],
                    ">" => [(low.max(right + 1), high), (low, high.min(right))],
                    _ => unreachable!(),
                };
                if matches[0].0 <= matches[0].1 {
                    let mut ranges_copy = ranges.clone();
                    ranges_copy[range_index] = matches[0];
                    total += count(workflows, target, &mut ranges_copy)
                }
                if matches[1].0 <= matches[1].1 {
                    ranges[range_index] = matches[1];
                }

                total
            } else {
                count(workflows, rule, ranges)
            }
        })
        .sum()
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let (workflows, inputs) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (key, value) = line.split_once('{').unwrap();
            let value = value.trim_end_matches('}').split(',').collect();
            (key, value)
        })
        .collect::<HashMap<&str, Vec<&str>>>();
    inputs
        .lines()
        .filter_map(|line| {
            let line = line.trim_start_matches('{').trim_end_matches('}');
            let states = line
                .split(',')
                .map(|s| {
                    let (_, value) = s.split_once('=').unwrap();
                    value.parse::<usize>().unwrap()
                })
                .collect::<Vec<_>>();
            if accept(&workflows, "in", &states) {
                Some(states.iter().sum::<usize>())
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let (workflows, _inputs) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (key, value) = line.split_once('{').unwrap();
            let value = value.trim_end_matches('}').split(',').collect();
            (key, value)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut ranges = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    count(&workflows, "in", &mut ranges)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 167409079868000);
    }
}
