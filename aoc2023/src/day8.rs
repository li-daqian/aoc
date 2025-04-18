use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let (instructions, networks) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .as_bytes()
        .iter()
        .map(|&b| if b == b'L' { 0 as usize } else { 1 })
        .collect::<Vec<_>>();
    let networks = networks
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(" = ").unwrap();
            let value = value
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ")
                .collect::<Vec<_>>();
            (name, value)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut value = "AAA";
    let target = "ZZZ";
    let mut steps = 0usize;
    loop {
        for &instruction in &instructions {
            steps += 1;
            value = networks[value][instruction];
            if value == target {
                return steps;
            }
        }
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let (instructions, networks) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .as_bytes()
        .iter()
        .map(|&b| if b == b'L' { 0 as usize } else { 1 })
        .collect::<Vec<_>>();
    let networks = networks
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(" = ").unwrap();
            let value = value
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ")
                .collect::<Vec<_>>();
            (name, value)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let cal_steps =
        |value: &str, networks: &HashMap<&str, Vec<&str>>, instructions: &Vec<usize>| {
            let mut steps = 0;
            let mut value = value;
            loop {
                for &instruction in instructions {
                    steps += 1;
                    value = networks[value][instruction];
                    if value.ends_with("Z") {
                        return steps;
                    }
                }
            }
        };

    let values = networks
        .keys()
        .filter(|&key| key.ends_with("A"))
        .collect::<Vec<_>>();
    values
        .iter()
        .map(|value| cal_steps(value, &networks, &instructions))
        .fold(1, |answers, steps| (answers * steps) / gcd(answers, steps))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE_1: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const SAMPLE_2: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_2), 6);
    }
}
