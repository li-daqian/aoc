use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let mut result = 0;

    let mut word: String = "".to_string();
    input.lines().for_each(|line| {
        line.chars().for_each(|c| match c {
            'm' => {
                word.push(c);
            }
            'u' if &word == "m" => {
                word.push(c);
            }
            'l' if &word == "mu" => {
                word.push(c);
            }
            '(' if &word == "mul" => {
                word.push(c);
            }
            d if d.is_ascii_digit()
                && word.starts_with("mul(")
                && word.chars().skip(4).all(|c| c.is_ascii_digit() || c == ',') =>
            {
                word.push(d);
            }
            ',' if word.starts_with("mul(") && word.chars().skip(4).all(|c| c.is_ascii_digit()) => {
                word.push(c);
            }
            ')' if word.starts_with("mul(") => {
                if let Some((a, b)) = word[4..].split_once(',') {
                    if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                        result += a * b;
                    }
                }
                word.clear();
            }
            _ => {
                word.clear();
            }
        });
    });
    result
}

#[aoc(day3, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let mut result = 0;

    let mut word: String = "".to_string();
    let mut enabled = true;
    input.lines().for_each(|line| {
        line.chars().for_each(|c| match c {
            'm' if enabled => {
                word.push(c);
            }
            'u' if enabled && &word == "m" => {
                word.push(c);
            }
            'l' if enabled && &word == "mu" => {
                word.push(c);
            }
            'd' => {
                word.push(c);
            }
            'o' if &word == "d" => {
                word.push(c);
            }
            'n' if enabled && &word == "do" => {
                word.push(c);
            }
            '\'' if enabled && &word == "don" => {
                word.push(c);
            }
            't' if enabled && &word == "don'" => {
                word.push(c);
            }
            '(' if (enabled && (&word == "mul" || &word == "don't")) || &word == "do" => {
                word.push(c);
            }
            d if enabled
                && d.is_ascii_digit()
                && word.starts_with("mul(")
                && word.chars().skip(4).all(|c| c.is_ascii_digit() || c == ',') =>
            {
                word.push(d);
            }
            ',' if enabled
                && word.starts_with("mul(")
                && word.chars().skip(4).all(|c| c.is_ascii_digit()) =>
            {
                word.push(c);
            }
            ')' => {
                if enabled {
                    if let Some(word) = word.strip_prefix("mul(") {
                        if let Some((a, b)) = word.split_once(',') {
                            if let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                                result += a * b;
                            }
                        }
                    } else if &word == "don't(" {
                        enabled = false;
                    }
                } else if &word == "do(" {
                    enabled = true;
                }
                word.clear();
            }
            _ => {
                word.clear();
            }
        });
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn part2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}
