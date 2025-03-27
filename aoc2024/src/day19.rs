use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let (pattern_set, min_pattern_len) = line.split(',').fold(
        (HashSet::new(), usize::MAX),
        |(mut set, min_len), pattern| {
            let pattern = pattern.trim();
            let len = pattern.len();
            set.insert(pattern);
            (set, min_len.min(len))
        },
    );

    lines.skip(1).fold(0, |acc, line| {
        let mut memo: HashMap<&str, bool> = HashMap::new();
        if can_resovle(&line, &pattern_set, min_pattern_len, &mut memo) {
            acc + 1
        } else {
            acc
        }
    })
}

fn can_resovle<'a>(
    line: &'a str,
    pattern_set: &HashSet<&str>,
    min_pattern_len: usize,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if line.is_empty() {
        return true;
    }

    if let Some(&cached) = memo.get(line) {
        return cached;
    }

    let len = line.len();
    for j in min_pattern_len..=len {
        if pattern_set.contains(&line[..j])
            && can_resovle(&line[j..], pattern_set, min_pattern_len, memo)
        {
            memo.insert(&line[..j], true);
            memo.insert(line, true);
            return true;
        }
    }

    memo.insert(line, false);
    false
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let (pattern_set, min_pattern_len) = line.split(',').fold(
        (HashSet::new(), usize::MAX),
        |(mut set, min_len), pattern| {
            let pattern = pattern.trim();
            let len = pattern.len();
            set.insert(pattern);
            (set, min_len.min(len))
        },
    );

    let mut global_dp: HashMap<&str, usize> = HashMap::new();

    lines.skip(1).fold(0, |acc, line| {
        acc + count_ways(&line, &pattern_set, min_pattern_len, &mut global_dp)
    })
}

fn count_ways<'a>(
    line: &'a str,
    pattern_set: &HashSet<&str>,
    min_pattern_len: usize,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if line.is_empty() {
        return 1;
    }

    if let Some(&cached) = memo.get(line) {
        return cached;
    }

    let mut total = 0;
    let len = line.len();
    for j in min_pattern_len..=len {
        if pattern_set.contains(&line[..j]) {
            total += count_ways(&line[j..], pattern_set, min_pattern_len, memo);
        }
    }

    memo.insert(line, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 16);
    }
}
