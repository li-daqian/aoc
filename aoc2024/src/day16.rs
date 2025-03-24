use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

use aoc_runner_derive::aoc;

struct Maze {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

struct Race {
    maze: Maze,
    start: (usize, usize),
    end: (usize, usize),
    directions: [(i8, i8); 4],
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    position: (usize, usize),
    direction: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .position
            .0
            .cmp(&self.position.0)
            .then_with(|| other.position.1.cmp(&other.position.1))
            .then_with(|| other.direction.cmp(&self.direction))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Race {
    fn from(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid: Vec<Vec<char>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                let mut rows: Vec<char> = line.chars().collect();
                if let Some(col) = line.find('S') {
                    start = (row, col);
                    rows[col] = '.';
                }
                if let Some(col) = line.find('E') {
                    end = (row, col);
                    rows[col] = '.';
                }
                rows
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();

        Self {
            maze: Maze {
                grid,
                height,
                width,
            },
            start,
            end,
            directions: [(0, 1), (-1, 0), (0, -1), (1, 0)],
        }
    }

    fn find_min_cost(&self) -> usize {
        let mut queue = BinaryHeap::new();
        let mut costs: HashMap<Node, usize> = HashMap::new();
        let start = Node {
            position: self.start,
            direction: 0,
        };
        queue.push(start);
        costs.insert(start, 0);

        loop {
            if queue.is_empty() {
                break;
            }

            let current = queue.pop().unwrap();
            let dis = [
                current.direction,
                (current.direction + 1) % self.directions.len(),
                (current.direction + 3) % self.directions.len(),
            ];
            for i in 0..dis.len() {
                let mut next = current.clone();
                let mut cost = costs.get(&current).unwrap().clone();
                let d = self.directions[dis[i]];
                if i == 0 {
                    next.position = (
                        (current.position.0 as isize + d.0 as isize) as usize,
                        (current.position.1 as isize + d.1 as isize) as usize,
                    );
                    if self.maze.grid[next.position.0][next.position.1] == '#' {
                        continue;
                    }
                    if next.position.0 >= self.maze.height || next.position.1 >= self.maze.width {
                        panic!();
                    }
                    cost += 1;
                } else {
                    next.direction = dis[i];
                    cost += 1000;
                }

                if !costs.contains_key(&next) || costs[&next] > cost {
                    costs.insert(next, cost);
                    queue.push(next);
                }
            }
        }

        costs
            .iter()
            .filter(|(node, _)| node.position == self.end)
            .map(|(_, cost)| *cost)
            .min()
            .unwrap()
    }

    fn find_min_path(&self) -> usize {
        let mut queue = BinaryHeap::new();
        let mut costs: HashMap<Node, (usize, HashSet<(usize, usize)>)> = HashMap::new();
        let start = Node {
            position: self.start,
            direction: 0,
        };
        queue.push(start);
        costs.insert(start, (0, HashSet::from([start.position])));

        loop {
            if queue.is_empty() {
                break;
            }

            let current = queue.pop().unwrap();
            let dis = [
                current.direction,
                (current.direction + 1) % self.directions.len(),
                (current.direction + 3) % self.directions.len(),
            ];
            for i in 0..dis.len() {
                let mut next = current.clone();
                let (mut cost, mut paths) = costs.get(&current).unwrap().clone();
                let d = self.directions[dis[i]];
                if i == 0 {
                    next.position = (
                        (current.position.0 as isize + d.0 as isize) as usize,
                        (current.position.1 as isize + d.1 as isize) as usize,
                    );
                    if self.maze.grid[next.position.0][next.position.1] == '#' {
                        continue;
                    }
                    if next.position.0 >= self.maze.height || next.position.1 >= self.maze.width {
                        panic!();
                    }
                    paths.insert(next.position);
                    cost += 1;
                } else {
                    next.direction = dis[i];
                    cost += 1000;
                }

                if !costs.contains_key(&next) || cost < costs[&next].0 {
                    costs.insert(next, (cost, paths));
                    queue.push(next);
                } else if costs[&next].0 == cost {
                    if let Some(entry) = costs.get_mut(&next) {
                        entry.1.extend(paths);
                        queue.push(next);
                    }
                }
            }
        }

        costs
            .iter()
            .filter(|(node, _)| node.position == self.end)
            .fold(
                (usize::MAX, HashSet::new()),
                |(min_cost, mut agg_paths), (_, (cost, paths))| {
                    if *cost < min_cost {
                        (*cost, paths.clone())
                    } else if *cost == min_cost {
                        agg_paths.extend(paths);
                        (min_cost, agg_paths)
                    } else {
                        (min_cost, agg_paths)
                    }
                },
            )
            .1
            .len()
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let race = Race::from(input);
    race.find_min_cost()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let race = Race::from(input);
    race.find_min_path()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 64);
    }
}
