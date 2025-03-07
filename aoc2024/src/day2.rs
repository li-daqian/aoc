use aoc_runner_derive::aoc;

#[inline]
fn parse_line(line: &str) -> Vec<u8> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

#[inline]
fn check_values(values: &Vec<u8>) -> (bool, usize) {
    let mut inc = true;
    let mut dec = true;
    for (i, num) in values.iter().enumerate().skip(1) {
        let previous = values[i - 1];
        if previous.abs_diff(*num) > 3 {
            return (false, i);
        }
        inc &= previous < *num;
        dec &= previous > *num;
        if !inc && !dec {
            return (false, i);
        }
    }
    (true, 0)
}

#[inline]
fn copy_remove(values: &Vec<u8>, index: usize) -> Vec<u8> {
    let mut copy = values.clone();
    copy.remove(index);
    copy
}

#[aoc(day2, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| check_values(&parse_line(line)).0)
        .count()
}

#[aoc(day2, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let values = parse_line(line);
            let (ok, err_index) = check_values(&values);
            ok || (err_index.saturating_sub(2)..=err_index)
                .any(|i| check_values(&copy_remove(&values, i)).0)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 4);
    }
}
