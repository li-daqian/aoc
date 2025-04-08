use aoc_runner_derive::aoc;

const MOD: usize = 16777216;

fn next(number: &mut usize) {
    *number = (*number ^ (*number << 6)) % MOD;
    *number = (*number ^ (*number >> 5)) % MOD;
    *number = (*number ^ (*number << 11)) % MOD;
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut secret_number = line.parse::<usize>().unwrap();
            for _ in 0..2000 {
                next(&mut secret_number);
            }

            secret_number
        })
        .sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    const MAX_CHANGES: usize = 20 + 20 * 20 + 20 * 400 + 20 * 8000;
    let mut max_prices = [0usize; MAX_CHANGES];

    input.lines().for_each(|line| {
        let mut prices = [0isize; 2000];
        let mut visited = [false; MAX_CHANGES];
        let mut secret_number = line.parse::<usize>().unwrap();
        for i in 0..2000 {
            prices[i] = (secret_number % 10) as isize;
            next(&mut secret_number);
        }

        for i in 4..prices.len() {
            let a = prices[i - 4];
            let b = prices[i - 3];
            let c = prices[i - 2];
            let d = prices[i - 1];
            let e = prices[i];
            let changes = (b - a + 10) as usize
                + (c - b + 10) as usize * 20
                + (d - c + 10) as usize * 400
                + (e - d + 10) as usize * 8000;
            // let changes = ((b - a) + (c - b) * 20 + (d - c) * 400 + (e - d) * 8000) as usize;

            if !visited[changes] {
                visited[changes] = true;
                max_prices[changes] += e as usize;
            }
        }
    });

    *max_prices.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        1
        10
        100
        2024
    "};

    const SAMPLE_2: &str = indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_2), 23);
    }
}
