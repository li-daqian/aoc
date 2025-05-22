use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|line| line.parse::<usize>().ok())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut clories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|line| line.parse::<usize>().ok())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    clories.sort_unstable_by(|a, b| b.cmp(a));

    clories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 45000);
    }
}
