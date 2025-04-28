use aoc_runner_derive::aoc;

const DIRECTIONS: [&str; 4] = ["R", "D", "L", "U"];

// Shoelace formula
// A = 1/2 * ((y1 + y2) * (x2 - x1) + (y2 + y3) * (x3 - x2) + ... + (yn + y1) * (x1 - xn))
fn cal_area<'a>(instructions: impl Iterator<Item = (&'a str, isize)>) -> isize {
    let mut area = 0;
    let mut r = 0;
    let mut c = 0;

    for (dir, dist) in instructions {
        let (next_r, next_c) = match dir {
            "U" => (r - dist, c),
            "R" => (r, c + dist),
            "D" => (r + dist, c),
            "L" => (r, c - dist),
            _ => unreachable!(),
        };
        area += (c + next_c) * (next_r - r) + dist;
        r = next_r;
        c = next_c;
    }
    area / 2 + 1
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> isize {
    let instructions = input.lines().map(|line| {
        (
            &line[0..1],
            line[2..]
                .split_once(' ')
                .unwrap()
                .0
                .parse::<isize>()
                .unwrap(),
        )
    });
    cal_area(instructions)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> isize {
    let instructions = input.lines().map(|line| {
        let (_, color) = line.split_once('#').unwrap();
        (
            DIRECTIONS[color[5..6].parse::<usize>().unwrap()],
            isize::from_str_radix(&color[0..5], 16).unwrap(),
        )
    });
    cal_area(instructions)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 952408144115);
    }
}
