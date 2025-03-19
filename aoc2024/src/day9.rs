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

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let mut result: Vec<usize> = Vec::new();
    let mut used: VecDeque<(usize, usize)> = VecDeque::new();
    let mut avail: VecDeque<(usize, usize)> = VecDeque::new();
    for (i, &c) in chars.iter().enumerate() {
        let digit = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            used.push_back((result.len(), digit));
        } else {
            avail.push_back((result.len(), digit));
        }
        let value = if i % 2 == 0 { i / 2 } else { 0 };
        for _ in 0..digit {
            result.push(value);
        }
    }

    let find_available =
        |avail: &VecDeque<(usize, usize)>, used: (usize, usize)| -> Option<usize> {
            for i in 0..avail.len() {
                let (avail_index, avail_size) = avail[i];
                if avail_index > used.0 {
                    return None;
                }
                if avail_size >= used.1 {
                    return Some(i);
                }
            }
            return None;
        };

    for i in (0..used.len()).rev() {
        let (used_position, used_size) = used[i];
        if let Some(avail_index) = find_available(&avail, (used_position, used_size)) {
            let (avail_position, avail_size) = avail[avail_index];
            for j in 0..used_size {
                result[avail_position + j] = result[used_position + j];
                result[used_position + j] = 0;
            }
            if avail_size == used_size {
                avail.remove(avail_index);
            } else {
                avail[avail_index] = (avail_position + used_size, avail_size - used_size);
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2858);
    }
}
