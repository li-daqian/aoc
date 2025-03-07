use std::{
    collections::{BinaryHeap, HashMap},
    vec,
};

use aoc_runner_derive::aoc;

#[inline]
fn parse_line(line: &str) -> (u32, u32) {
    let (left, right) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
    (
        left.parse().unwrap(),
        right.trim_ascii_start().parse().unwrap(),
    )
}

#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let mut left_heap = BinaryHeap::with_capacity(1024);
    let mut right_heap = BinaryHeap::with_capacity(1024);

    input.lines().for_each(|line| {
        let (left, right) = parse_line(line);
        left_heap.push(left);
        right_heap.push(right);
    });

    let mut result = 0u32;
    while !left_heap.is_empty() {
        let left = left_heap.pop().unwrap();
        let right = right_heap.pop().unwrap();
        result += left.abs_diff(right);
    }
    result
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let mut left_vec = vec![0u32; 1024];
    let mut right_map: HashMap<u32, u32> = HashMap::with_capacity(1024);

    input.lines().for_each(|line| {
        let (left, right) = parse_line(line);
        left_vec.push(left);
        right_map
            .contains_key(&right)
            .then(|| {
                right_map.insert(right, right_map.get(&right).unwrap() + 1);
            })
            .or_else(|| {
                right_map.insert(right, 1);
                Some(())
            });
    });

    left_vec
        .iter()
        .map(|left| left * right_map.get(left).unwrap_or(&0u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 31);
    }
}
