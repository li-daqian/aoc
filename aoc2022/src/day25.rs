use aoc_runner_derive::aoc;

fn snafu_to_decimal(number: &str) -> i64 {
    number.bytes().fold(0, |value, digit| {
        let digit = match digit {
            b'2' => 2,
            b'1' => 1,
            b'0' => 0,
            b'-' => -1,
            b'=' => -2,
            _ => panic!("invalid SNAFU digit: {digit}"),
        };
        value * 5 + digit
    })
}

fn decimal_to_snafu(mut value: i64) -> String {
    if value == 0 {
        return "0".to_owned();
    }

    let mut digits = Vec::new();
    while value != 0 {
        let remainder = value.rem_euclid(5);
        let (digit, next_value) = match remainder {
            0 => ('0', value / 5),
            1 => ('1', value / 5),
            2 => ('2', value / 5),
            3 => ('=', (value + 2) / 5),
            4 => ('-', (value + 1) / 5),
            _ => unreachable!(),
        };
        digits.push(digit);
        value = next_value;
    }

    digits.into_iter().rev().collect()
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> String {
    let total = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(snafu_to_decimal)
        .sum();
    decimal_to_snafu(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {
            "
            1=-0-2
            12111
            2=0=
            21
            2=01
            111
            20012
            112
            1=-1=
            1-12
            12
            1=
            122
            "
        };
        assert_eq!(part1(input), "2=-1=0");
    }

    #[test]
    fn converts_each_digit() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(4), "1-");
    }
}
