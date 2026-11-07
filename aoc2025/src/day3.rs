use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
#[must_use]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let digits: Vec<u32> = line
                .bytes()
                .map(|byte| {
                    byte.checked_sub(b'0')
                        .filter(|digit| *digit <= 9)
                        .expect("battery ratings must be digits") as u32
                })
                .collect();
            assert!(
                digits.len() >= 2,
                "each bank must contain at least two batteries"
            );

            let mut largest_suffix = digits[digits.len() - 1];
            let mut largest_joltage = 0;

            for &digit in digits[..digits.len() - 1].iter().rev() {
                largest_joltage = largest_joltage.max(digit * 10 + largest_suffix);
                largest_suffix = largest_suffix.max(digit);
            }

            u64::from(largest_joltage)
        })
        .sum()
}

fn max_joltage(line: &str, battery_count: usize) -> u64 {
    let digits: Vec<u8> = line
        .bytes()
        .map(|byte| {
            byte.checked_sub(b'0')
                .filter(|digit| *digit <= 9)
                .expect("battery ratings must be digits")
        })
        .collect();
    assert!(
        digits.len() >= battery_count,
        "each bank must contain enough batteries"
    );

    let mut joltage = 0;
    let mut start = 0;

    for selected_count in (0..battery_count).rev() {
        let end = digits.len() - selected_count;
        let mut offset = 0;
        for (candidate_offset, &candidate_digit) in digits[start..end].iter().enumerate().skip(1) {
            if candidate_digit > digits[start + offset] {
                offset = candidate_offset;
            }
        }
        let digit = digits[start + offset];

        joltage = joltage * 10 + u64::from(digit);
        start += offset + 1;
    }

    joltage
}

#[aoc(day3, part2)]
#[must_use]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| max_joltage(line, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE), 357);
    }

    #[test]
    fn part1_preserves_battery_order() {
        assert_eq!(part1("8119"), 89);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE), 3_121_910_778_619);
    }

    #[test]
    fn part2_selects_largest_ordered_subsequence() {
        assert_eq!(max_joltage("234234234234278", 12), 434_234_234_278);
    }
}
