use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use aoc_runner_derive::aoc;

struct Maze {
    gird: Vec<Vec<char>>,
    bytes: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.gird {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Maze {
    const DIRS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn from(input: &str, height: usize, width: usize) -> Self {
        let mut bytes: Vec<(usize, usize)> = vec![];
        input.lines().for_each(|line| {
            let (col, row) = line.split_once(',').unwrap();
            let (col, row) = (col.parse::<usize>().unwrap(), row.parse::<usize>().unwrap());
            bytes.push((row, col))
        });
        Self {
            gird: vec![vec!['.'; width]; height],
            bytes,
            height,
            width,
        }
    }

    fn fill(&mut self, bytes: usize) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.gird[row][col] = '.';
            }
        }
        for i in 0..bytes {
            let (row, col) = self.bytes[i];
            self.gird[row][col] = '#';
        }
    }

    fn try_move(&self, pos: (usize, usize), dir: (i8, i8)) -> Option<(usize, usize)> {
        let (row, col) = pos;
        let (dr, dc) = dir;
        let new_row = row as isize + dr as isize;
        let new_col = col as isize + dc as isize;
        if new_row >= 0
            && new_row < self.height as isize
            && new_col >= 0
            && new_col < self.width as isize
        {
            let (new_row, new_col) = (new_row as usize, new_col as usize);
            if self.gird[new_row][new_col] == '#' {
                return None;
            } else {
                return Some((new_row, new_col));
            }
        }
        None
    }

    fn find_min_step(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut queue: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
        let mut steps: Vec<Vec<usize>> = vec![vec![usize::MAX; self.width]; self.height];
        queue.push(Reverse((0, start)));
        steps[start.1][start.0] = 0;
        while let Some(Reverse((dist, (row, col)))) = queue.pop() {
            for d in Self::DIRS {
                if let Some((new_row, new_col)) = self.try_move((row, col), d) {
                    let new_dist = dist + 1;
                    if new_dist < steps[new_row][new_col] {
                        steps[new_row][new_col] = new_dist;
                        queue.push(Reverse((new_dist, (new_row, new_col))));
                    }
                }
            }
        }
        steps[end.0][end.1]
    }

    fn can_escape(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let mut queue: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        queue.push(Reverse((0, start)));
        visited[start.1][start.0] = true;
        while let Some(Reverse((_, (row, col)))) = queue.pop() {
            if (row, col) == end {
                return true;
            }
            for d in Self::DIRS {
                if let Some((new_row, new_col)) = self.try_move((row, col), d) {
                    if !visited[new_row][new_col] {
                        visited[new_row][new_col] = true;
                        queue.push(Reverse((0, (new_row, new_col))));
                    }
                }
            }
        }
        false
    }

    fn find_first_byte(&mut self, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut min = 2usize;
        let mut max = self.bytes.len() - 1;
        while min < max {
            let mid = (min + max) / 2;
            self.fill(mid + 1);
            if self.can_escape(start, end) {
                min = mid + 1;
            } else {
                max = mid;
            }
        }
        max
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let mut maze = Maze::from(input, 71, 71);
    maze.fill(1024);
    maze.find_min_step((0, 0), (70, 70))
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let mut maze = Maze::from(input, 71, 71);
    let first_byte = maze.find_first_byte((0, 0), (70, 70));
    format!("{},{}", maze.bytes[first_byte].1, maze.bytes[first_byte].0)
}

#[cfg(test)]
pub mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn test_part1() {
        let mut maze = Maze::from(SAMPLE, 7, 7);
        maze.fill(12);
        assert_eq!(maze.find_min_step((0, 0), (6, 6)), 22);
    }

    #[test]
    fn test_part2() {
        let mut maze = Maze::from(SAMPLE, 7, 7);
        let first_byte = maze.find_first_byte((0, 0), (6, 6));
        assert_eq!(first_byte, 20);
    }
}
