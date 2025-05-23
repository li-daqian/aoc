use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let bytes = input.as_bytes();
    for i in 3..bytes.len() {
        let w = &bytes[i - 3..=i];
        if w[0] != w[1]
            && w[0] != w[2]
            && w[0] != w[3]
            && w[1] != w[2]
            && w[1] != w[3]
            && w[2] != w[3]
        {
            return i + 1;
        }
    }
    panic!("No marker found");
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    for i in 13..bytes.len() {
        let w = &bytes[i - 13..=i];
        let mut seen = [0u8; 26];
        let mut unique = true;
        for &b in w {
            let idx = (b - b'a') as usize;
            seen[idx] += 1;
            if seen[idx] > 1 {
                unique = false;
                break;
            }
        }
        if unique {
            return i + 1;
        }
    }

    panic!("No marker found");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        bvwbjplbgvbhsrlpgdmjqwftvncz
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 23);
    }
}
