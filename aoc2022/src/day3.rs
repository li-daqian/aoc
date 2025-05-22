use aoc_runner_derive::aoc;

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let (mut first, mut second) = (
                first.chars().collect::<Vec<_>>(),
                second.chars().collect::<Vec<_>>(),
            );

            first.sort_unstable();
            second.sort_unstable();

            let mut i = 0;
            let mut j = 0;
            while i < first.len() && j < second.len() {
                if first[i] == second[j] {
                    return priority(first[i]);
                } else if first[i] < second[j] {
                    i += 1;
                } else {
                    j += 1;
                }
            }

            panic!("No common character found");
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut answer = 0;
    let mut lines = input.lines();
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        let (mut l1, mut l2, mut l3) = (
            l1.chars().collect::<Vec<_>>(),
            l2.chars().collect::<Vec<_>>(),
            l3.chars().collect::<Vec<_>>(),
        );

        l1.sort_unstable();
        l2.sort_unstable();
        l3.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < l1.len() && j < l2.len() && k < l3.len() {
            if l1[i] == l2[j] && l1[i] == l3[k] {
                answer += priority(l1[i]);
                break;
            } else if l1[i] < l2[j] || l1[i] < l3[k] {
                i += 1;
            } else if l2[j] < l1[i] || l2[j] < l3[k] {
                j += 1;
            } else {
                k += 1;
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 70);
    }
}
