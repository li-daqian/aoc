use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in input.lines() {
        let (dir, n) = line.split_once(' ').unwrap();
        let n: i32 = n.parse().unwrap();
        let (dx, dy) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..n {
            head.0 += dx;
            head.1 += dy;

            let (hx, hy) = head;
            let (tx, ty) = tail;
            let dx = hx - tx;
            let dy = hy - ty;
            if dx.abs() > 1 || dy.abs() > 1 {
                tail.0 += dx.signum();
                tail.1 += dy.signum();
            }
            visited.insert(tail);
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    use std::collections::HashSet;

    let mut knots = [(0i32, 0i32); 10];
    let mut visited = HashSet::new();
    visited.insert(knots[9]);

    for line in input.lines() {
        let (dir, n) = line.split_once(' ').unwrap();
        let n: i32 = n.parse().unwrap();
        let (dx, dy) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..n {
            knots[0].0 += dx;
            knots[0].1 += dy;
            for i in 1..10 {
                let (hx, hy) = knots[i - 1];
                let (tx, ty) = knots[i];
                let dx = hx - tx;
                let dy = hy - ty;
                if dx.abs() > 1 || dy.abs() > 1 {
                    knots[i].0 += dx.signum();
                    knots[i].1 += dy.signum();
                }
            }
            visited.insert(knots[9]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE1: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const SAMPLE2: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn test_part2_small() {
        assert_eq!(part2(SAMPLE1), 1);
    }

    #[test]
    fn test_part2_large() {
        assert_eq!(part2(SAMPLE2), 36);
    }
}
