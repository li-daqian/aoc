use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(":").unwrap();
            let (winning_numbers, numbers) = numbers.split_once("|").unwrap();
            let winning_numbers = winning_numbers
                .split(' ')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            numbers.split(" ")
        })
        .sum()
}
