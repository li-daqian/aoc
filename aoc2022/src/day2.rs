use aoc_runner_derive::aoc;

fn outcome_score(opp: u8, you: u8) -> usize {
    if opp == you {
        3
    } else if (opp + 1) % 3 == you {
        6
    } else {
        0
    }
}

fn find_you(opp: u8, outcome: u8) -> u8 {
    if outcome == 0 {
        (opp + 2) % 3
    } else if outcome == 3 {
        opp
    } else {
        (opp + 1) % 3
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opp, you) = line.split_once(' ').unwrap();
            let opp = opp.as_bytes()[0] - b'A';
            let you = you.as_bytes()[0] - b'X';
            let score = outcome_score(opp, you) + you as usize + 1;
            score
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opp, you) = line.split_once(' ').unwrap();
            let opp = opp.as_bytes()[0] - b'A';
            let outcome = (you.as_bytes()[0] - b'X') * 3;
            let score = outcome as usize + find_you(opp, outcome) as usize + 1;
            score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
