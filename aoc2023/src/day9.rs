use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<isize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            generate_next(&numbers)
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<isize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            generate_first(&numbers)
        })
        .sum()
}

fn generate_next(numbers: &Vec<isize>) -> isize {
    if numbers.last() == Some(&0) {
        return 0;
    }

    let mut differs = vec![];
    for i in 0..numbers.len() - 1 {
        differs.push(numbers[i + 1] - numbers[i]);
    }

    generate_next(&differs) + numbers.last().unwrap()
}

fn generate_first(numbers: &Vec<isize>) -> isize {
    if numbers.last() == Some(&0) {
        return 0;
    }

    let mut differs = vec![];
    for i in 0..numbers.len() - 1 {
        differs.push(numbers[i + 1] - numbers[i]);
    }

    numbers.first().unwrap() - generate_first(&differs)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2);
    }
}
