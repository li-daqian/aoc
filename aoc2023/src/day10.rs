use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let (maze, start) = parse_maze(input);
    let height = maze.len();
    let width = maze[0].len();

    let mut costs = vec![vec![usize::MAX; width]; height];
    let mut queue = BinaryHeap::<Reverse<(usize, (usize, usize))>>::new();

    queue.push(Reverse((0, start)));
    while let Some(Reverse((cost, (row, col)))) = queue.pop() {
        if cost > costs[row][col] {
            continue;
        }
        costs[row][col] = cost;
        for &dir in &maze[row][col] {
            let (dr, dc) = DIRECTIONS[dir];
            let (new_row, new_col) = (row as isize + dr, col as isize + dc);
            if new_row < 0 || new_row >= height as isize || new_col < 0 || new_col >= width as isize
            {
                continue;
            }
            let (new_row, new_col) = (new_row as usize, new_col as usize);
            if maze[new_row][new_col].iter().any(|&d| is_connected(d, dir)) {
                queue.push(Reverse((cost + 1, (new_row, new_col))));
            }
        }
    }

    *costs
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cost| cost < usize::MAX)
        .max()
        .unwrap()
}

// #[aoc(day10, part2)]
// pub fn part2(input: &str) -> usize {

// }

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_connected(a: usize, b: usize) -> bool {
    (a + 2) % DIRECTIONS.len() == b
}

fn parse_maze(input: &str) -> (Vec<Vec<Vec<usize>>>, (usize, usize)) {
    let mut start = (0, 0);
    let maze = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => vec![],
                    '|' => vec![NORTH, SOUTH],
                    '-' => vec![WEST, EAST],
                    'S' => {
                        start = (row, col);
                        vec![NORTH, EAST, SOUTH, WEST]
                    }
                    'J' => vec![NORTH, WEST],
                    'L' => vec![NORTH, EAST],
                    'F' => vec![SOUTH, EAST],
                    '7' => vec![SOUTH, WEST],
                    _ => panic!("Unknown character: {}", c),
                })
                .collect()
        })
        .collect();
    (maze, start)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 8);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE), 2);
    // }
}
