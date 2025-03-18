use std::collections::VecDeque;

use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let mut result: Vec<usize> = Vec::new();
    let mut avail: VecDeque<usize> = VecDeque::new();
    for (i, &c) in chars.iter().enumerate() {
        let digit = c.to_digit(10).unwrap() as usize;
        for _ in 0..digit {
            if i % 2 == 0 {
                let id = i / 2;
                result.push(id);
            } else {
                avail.push_back(result.len());
                result.push(0);
            }
        }
    }

    for i in (0..result.len()).rev() {
        if result[i] != 0 {
            if let Some(&first_empty) = avail.front() {
                if first_empty < i {
                    result[first_empty] = result[i];
                    result[i] = 0;
                    avail.pop_front();
                }
            } else {
                break;
            }
        }
    }

    result
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &val)| acc + val * i)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1928);
    }
}
