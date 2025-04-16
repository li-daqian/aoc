use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (wanted_numbers, numbers) = numbers.split_once(" | ").unwrap();
            let wanted_numbers = wanted_numbers
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let win_count = numbers
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .filter(|n| wanted_numbers.contains(n))
                .count();
            if win_count <= 1 {
                win_count
            } else {
                1 << (win_count - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut card_instrances = [1usize; 208];
    let mut card_index = 0;
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (wanted_numbers, numbers) = numbers.split_once(" | ").unwrap();
            let wanted_numbers = wanted_numbers
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let win_count = numbers
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .filter(|n| wanted_numbers.contains(n))
                .count();
            let instance = card_instrances[card_index];

            for i in 0..win_count {
                if card_index + i + 1 < card_instrances.len() {
                    card_instrances[card_index + i + 1] += 1 * instance;
                }
            }

            card_index += 1;
            instance
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 30);
    }
}
