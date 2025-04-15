use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let mut digits = (0..line.len()).filter_map(|i| match line[i] {
                b'0'..=b'9' => Some((line[i] - b'0') as usize),
                _ => None,
            });
            let a = digits.next().unwrap();
            let b = digits.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    const DIGITS: &[&[u8]] = &[
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let mut digits = (0..line.len()).filter_map(|i| match line[i] {
                b'0'..=b'9' => Some((line[i] - b'0') as usize),
                _ => DIGITS
                    .iter()
                    .enumerate()
                    .find_map(|(di, word)| line[i..].starts_with(word).then_some(di + 1)),
            });
            let a = digits.next().unwrap();
            let b = digits.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE_1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const SAMPLE_2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_2), 281);
    }
}
