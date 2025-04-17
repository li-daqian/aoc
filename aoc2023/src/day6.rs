use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    let time = time
        .split_whitespace()
        .skip(1)
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distance = distance
        .split_whitespace()
        .skip(1)
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    time.iter().enumerate().fold(1usize, |acc, (i, &t)| {
        let target = distance[i];
        let win_count = (1..t)
            .filter(|&j| {
                let moved = j * (t - j);
                moved > target
            })
            .count();
        acc * win_count
    })
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    let time = time
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, x| {
            acc.push_str(x.trim());
            acc
        })
        .parse::<usize>()
        .unwrap();
    let distance = distance
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, x| {
            acc.push_str(x.trim());
            acc
        })
        .parse::<usize>()
        .unwrap();

    // d=j*(t-j)
    // d=j*t-j^2
    // j^2-j*t+d=0 ax^2+bx+c=0 => x=(-b±√(b^2-4ac))/(2a)
    // j=(t±√(t^2-4d))/2

    let high = (time as f64 + ((time * time) as f64 - 4.0 * distance as f64).sqrt()) / 2.0;
    let low = (time as f64 - ((time * time) as f64 - 4.0 * distance as f64).sqrt()) / 2.0;
    (high.floor() - low.ceil()) as usize + 1
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 71503);
    }
}
