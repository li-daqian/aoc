use aoc_runner_derive::aoc;

fn is_invalid(id: u64) -> bool {
    let digits = id.to_string();
    let midpoint = digits.len() / 2;

    digits.len().is_multiple_of(2) && digits[..midpoint] == digits[midpoint..]
}

fn is_invalid_repeated(id: u64) -> bool {
    let digits = id.to_string();
    let bytes = digits.as_bytes();

    (1..=bytes.len() / 2).any(|pattern_len| {
        bytes.len().is_multiple_of(pattern_len)
            && bytes
                .chunks(pattern_len)
                .all(|chunk| chunk == &bytes[..pattern_len])
    })
}

#[aoc(day2, part1)]
#[must_use]
pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let range = range.trim();
            let (first, last) = range.split_once('-').expect("range must contain a dash");
            (
                first.parse::<u64>().expect("first ID must be an integer"),
                last.parse::<u64>().expect("last ID must be an integer"),
            )
        })
        .flat_map(|(first, last)| first..=last)
        .filter(|&id| is_invalid(id))
        .sum()
}

#[aoc(day2, part2)]
#[must_use]
pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let range = range.trim();
            let (first, last) = range.split_once('-').expect("range must contain a dash");
            (
                first.parse::<u64>().expect("first ID must be an integer"),
                last.parse::<u64>().expect("last ID must be an integer"),
            )
        })
        .flat_map(|(first, last)| first..=last)
        .filter(|&id| is_invalid_repeated(id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE), 1_227_775_554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE), 4_174_379_265);
    }
}
