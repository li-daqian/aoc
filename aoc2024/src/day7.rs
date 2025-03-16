use aoc_runner_derive::aoc;

#[inline]
fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let answer = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            (answer, values)
        })
        .collect()
}

fn check_answer(answer: usize, current: usize, input: &Vec<usize>, i: usize) -> bool {
    if i == input.len() {
        return current == answer;
    }
    return check_answer(answer, current + input[i], input, i + 1)
        || check_answer(answer, current * input[i], input, i + 1);
}

fn check_answer_2(answer: usize, current: usize, input: &Vec<usize>, i: usize) -> bool {
    if i == input.len() {
        return current == answer;
    }
    return check_answer_2(answer, current + input[i], input, i + 1)
        || check_answer_2(answer, current * input[i], input, i + 1)
        || check_answer_2(
            answer,
            format!("{}{}", current, input[i]).parse::<usize>().unwrap(),
            input,
            i + 1,
        );
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let inputs = parse_input(input);
    inputs
        .iter()
        .filter(|(answer, input)| check_answer(*answer, 0, input, 0))
        .map(|(answer, _)| answer)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let inputs = parse_input(input);
    inputs
        .iter()
        .filter(|(answer, input)| check_answer_2(*answer, 0, input, 0))
        .map(|(answer, _)| answer)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 11387);
    }
}
