use std::{collections::HashMap, mem};

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let stones: HashMap<usize, usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_default() += 1;
            acc
        });
    count_stones(stones, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let stones: HashMap<usize, usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_default() += 1;
            acc
        });
    count_stones(stones, 75)
}

fn count_stones(mut stones: HashMap<usize, usize>, blinks: usize) -> usize {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();

    for _ in 0..blinks {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new_stones.entry(1).or_default() += count;
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let left = stone / 10_usize.pow(digits / 2);
                    let right = stone % 10_usize.pow(digits / 2);
                    *new_stones.entry(left).or_default() += count;
                    *new_stones.entry(right).or_default() += count;
                } else {
                    *new_stones.entry(stone * 2024).or_default() += count;
                }
            }
        }

        mem::swap(&mut stones, &mut new_stones);
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"125 17"};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 55312);
    }
}
