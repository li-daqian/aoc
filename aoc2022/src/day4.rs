use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a_start, a_end) = a.split_once('-').unwrap();
            let (b_start, b_end) = b.split_once('-').unwrap();
            let (a_start, a_end): (u32, u32) = (a_start.parse().unwrap(), a_end.parse().unwrap());
            let (b_start, b_end): (u32, u32) = (b_start.parse().unwrap(), b_end.parse().unwrap());
            (a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end)
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a_start, a_end) = a.split_once('-').unwrap();
            let (b_start, b_end) = b.split_once('-').unwrap();
            let (a_start, a_end): (u32, u32) = (a_start.parse().unwrap(), a_end.parse().unwrap());
            let (b_start, b_end): (u32, u32) = (b_start.parse().unwrap(), b_end.parse().unwrap());
            a_start <= b_end && b_start <= a_end
        })
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4);
    }
}
