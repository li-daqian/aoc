use aoc_runner_derive::aoc;

fn parse(input: &str) -> (Vec<Vec<bool>>, usize) {
    let (max_x, max_y) = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ").map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
        })
        .fold((0, 0), |(max_x, max_y), (x, y)| {
            (max_x.max(x), max_y.max(y))
        });

    // Add a large enough buffer for sand to spread
    let buffer = max_y + 3; // +3 for safety
    let width = max_x + buffer * 2;
    let mut blocked = vec![vec![false; width]; max_y + 1];
    for line in input.lines() {
        let points: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
        for w in points.windows(2) {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    blocked[y][x1 + buffer] = true;
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    blocked[y1][x + buffer] = true;
                }
            }
        }
    }
    (blocked, buffer)
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let (mut blocked, buffer) = parse(input);
    let max_y = blocked.len() - 1;

    let mut answer = 0;
    loop {
        let (mut x, mut y) = (500 + buffer, 0);
        loop {
            if y == max_y {
                return answer;
            }
            if !blocked[y + 1][x] {
                y += 1;
            } else if !blocked[y + 1][x - 1] {
                x -= 1;
                y += 1;
            } else if !blocked[y + 1][x + 1] {
                x += 1;
                y += 1;
            } else {
                blocked[y][x] = true;
                answer += 1;
                break;
            }
        }
    }
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let (mut blocked, buffer) = parse(input);
    let width = blocked[0].len();
    blocked.push(vec![false; width]);
    blocked.push(vec![true; width]);

    let mut answer = 0;
    loop {
        let (mut x, mut y) = (500 + buffer, 0);
        loop {
            if !blocked[y + 1][x] {
                y += 1;
            } else if !blocked[y + 1][x - 1] {
                x -= 1;
                y += 1;
            } else if !blocked[y + 1][x + 1] {
                x += 1;
                y += 1;
            } else {
                blocked[y][x] = true;
                answer += 1;
                if x == 500 + buffer && y == 0 {
                    return answer;
                }
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 93);
    }
}
