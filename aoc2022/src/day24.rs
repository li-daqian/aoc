use aoc_runner_derive::aoc;
use std::collections::{HashSet, VecDeque};

type Position = (i32, i32);

#[derive(Clone, Copy)]
struct Blizzard {
    position: Position,
    direction: Position,
}

struct Valley {
    height: i32,
    width: i32,
    start: Position,
    end: Position,
    blizzards_by_time: Vec<HashSet<Position>>,
}

impl Valley {
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();
        assert!(
            lines.iter().all(|line| line.len() == lines[0].len()),
            "valley rows must have equal width"
        );
        let height = lines.len() as i32 - 2;
        let width = lines[0].len() as i32 - 2;
        let start = (
            0,
            lines[0].bytes().position(|cell| cell == b'.').unwrap() as i32,
        );
        let end = (
            height + 1,
            lines
                .last()
                .unwrap()
                .bytes()
                .position(|cell| cell == b'.')
                .unwrap() as i32,
        );

        let mut blizzards = Vec::new();
        for (row, line) in lines.iter().enumerate().skip(1).take(height as usize) {
            for (col, cell) in line.bytes().enumerate().skip(1).take(width as usize) {
                let direction = match cell {
                    b'^' => Some((-1, 0)),
                    b'v' => Some((1, 0)),
                    b'<' => Some((0, -1)),
                    b'>' => Some((0, 1)),
                    b'.' => None,
                    _ => panic!("unexpected valley cell: {cell}"),
                };
                if let Some(direction) = direction {
                    blizzards.push(Blizzard {
                        position: (row as i32, col as i32),
                        direction,
                    });
                }
            }
        }

        let cycle = lcm(height, width) as usize;
        let blizzards_by_time = (0..cycle)
            .map(|time| {
                blizzards
                    .iter()
                    .map(|blizzard| {
                        let (row, col) = blizzard.position;
                        let (dr, dc) = blizzard.direction;
                        (
                            1 + (row - 1 + dr * time as i32).rem_euclid(height),
                            1 + (col - 1 + dc * time as i32).rem_euclid(width),
                        )
                    })
                    .collect()
            })
            .collect();

        Self {
            height,
            width,
            start,
            end,
            blizzards_by_time,
        }
    }

    fn shortest_time(&self, start: Position, target: Position, start_time: i32) -> i32 {
        const MOVES: [Position; 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];
        let cycle = self.blizzards_by_time.len() as i32;
        let mut queue = VecDeque::from([(start_time, start)]);
        let mut visited = HashSet::from([(start_time.rem_euclid(cycle), start)]);

        while let Some((time, position)) = queue.pop_front() {
            let next_time = time + 1;
            let hazards = &self.blizzards_by_time[next_time.rem_euclid(cycle) as usize];
            for (dr, dc) in MOVES {
                let next = (position.0 + dr, position.1 + dc);
                let in_valley =
                    (1..=self.height).contains(&next.0) && (1..=self.width).contains(&next.1);
                if next != self.start && next != self.end && !in_valley {
                    continue;
                }
                if hazards.contains(&next) {
                    continue;
                }
                if next == target {
                    return next_time;
                }
                let state = (next_time.rem_euclid(cycle), next);
                if visited.insert(state) {
                    queue.push_back((next_time, next));
                }
            }
        }

        panic!("target is unreachable")
    }
}

fn gcd(mut left: i32, mut right: i32) -> i32 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left.abs()
}

fn lcm(left: i32, right: i32) -> i32 {
    left / gcd(left, right) * right
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> i64 {
    let valley = Valley::parse(input);
    valley.shortest_time(valley.start, valley.end, 0) as i64
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> i64 {
    let valley = Valley::parse(input);
    let there = valley.shortest_time(valley.start, valley.end, 0);
    let back = valley.shortest_time(valley.end, valley.start, there);
    valley.shortest_time(valley.start, valley.end, back) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {
            "
            #.######
            #>>.<^<#
            #.<..<<#
            #>v.><>#
            #<^v^^>#
            ######.#
            "
        };
        assert_eq!(part1(input), 18);
        assert_eq!(part2(input), 54);
    }
}
