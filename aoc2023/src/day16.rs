use std::{usize, vec};

use aoc_runner_derive::aoc;

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn energized_tiles(grid: &Vec<Vec<char>>, (r, c, d): (usize, usize, usize)) -> usize {
    let mut seen = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
    let mut queue = vec![(r, c, d)];
    while let Some((r, c, d)) = queue.pop() {
        if seen[r][c][d] {
            continue;
        }
        seen[r][c][d] = true;

        let next_directions = match grid[r][c] {
            '.' => {
                vec![d]
            }
            '|' => {
                if d == NORTH || d == SOUTH {
                    vec![d]
                } else {
                    vec![NORTH, SOUTH]
                }
            }
            '-' => {
                if d == WEST || d == EAST {
                    vec![d]
                } else {
                    vec![WEST, EAST]
                }
            }
            '\\' => match d {
                NORTH => vec![WEST],
                EAST => vec![SOUTH],
                SOUTH => vec![EAST],
                WEST => vec![NORTH],
                _ => unreachable!(),
            },
            '/' => match d {
                NORTH => vec![EAST],
                EAST => vec![NORTH],
                SOUTH => vec![WEST],
                WEST => vec![SOUTH],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        for nd in next_directions {
            let (dr, dc) = DIRECTIONS[nd];
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0 && nc >= 0 && nr < grid.len() as isize && nc < grid[0].len() as isize {
                queue.push((nr as usize, nc as usize, nd));
            }
        }
    }

    seen.iter()
        .flatten()
        .filter(|ds| ds.iter().any(|&d| d))
        .count()
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    energized_tiles(&grid, (0, 0, EAST))
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut max_energized = 0;
    for c in 0..grid[0].len() {
        max_energized = energized_tiles(&grid, (0, c, SOUTH)).max(max_energized);
        max_energized = energized_tiles(&grid, (grid.len() - 1, c, NORTH)).max(max_energized);
    }
    for r in 0..grid.len() {
        max_energized = energized_tiles(&grid, (r, 0, EAST)).max(max_energized);
        max_energized = energized_tiles(&grid, (r, grid[0].len() - 1, WEST)).max(max_energized);
    }

    max_energized
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 51);
    }
}
