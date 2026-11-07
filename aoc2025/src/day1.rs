use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let (mut position, mut password) = (50i32, 0u32);

    for line in input.lines().filter(|line| !line.is_empty()) {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance
            .parse()
            .expect("rotation distance must be an integer");

        position = match direction {
            "L" => (position - distance).rem_euclid(100),
            "R" => (position + distance).rem_euclid(100),
            _ => panic!("rotation must start with L or R"),
        };

        if position == 0 {
            password += 1;
        }
    }

    password
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let (mut position, mut password) = (50i32, 0u32);

    for line in input.lines().filter(|line| !line.is_empty()) {
        let (direction, distance) = line.split_at(1);
        let distance: u32 = distance
            .parse()
            .expect("rotation distance must be an integer");
        let step = match direction {
            "L" => -1,
            "R" => 1,
            _ => panic!("rotation must start with L or R"),
        };

        for _ in 0..distance {
            position = (position + step).rem_euclid(100);
            if position == 0 {
                password += 1;
            }
        }
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
