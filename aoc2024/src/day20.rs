use aoc_runner_derive::aoc;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Node = (usize, usize);

struct Maze {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

struct Race {
    maze: Maze,
    start: Node,
    end: Node,
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
        }
    }

    fn get_dist_from(&self, node: Node) -> Vec<Vec<usize>> {
        let (height, width) = (self.maze.height, self.maze.width);
        let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; width]; height];
        dist[node.0][node.1] = 0;
        let mut q: Vec<Node> = vec![node];
        while let Some((row, col)) = q.pop() {
            if self.maze.grid[row][col] == '.' {
                for d in DIRS {
                    if let Some((next_row, next_col)) = self.try_move((row, col), d) {
                        if dist[next_row][next_col] == usize::MAX {
                            dist[next_row][next_col] = dist[row][col] + 1;
                            q.push((next_row, next_col));
                        }
                    }
                }
            }
        }
        dist
    }

    fn try_move(&self, (row, col): (usize, usize), d: (i8, i8)) -> Option<(usize, usize)> {
        let new_row = row as isize + d.0 as isize;
        let new_col = col as isize + d.1 as isize;
        if new_row >= 0
            && new_row < self.maze.height as isize
            && new_col >= 0
            && new_col < self.maze.width as isize
        {
            Some((new_row as usize, new_col as usize))
        } else {
            None
        }
    }

    fn inside(&self, node: Node) -> bool {
        node.0 < self.maze.height && node.1 < self.maze.width
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let race = Race::from(input);
    let (start, end) = (race.start, race.end);

    let dist_from_start = race.get_dist_from(start);
    let dist_from_end = race.get_dist_from(end);
    let normal_dist = dist_from_start[end.0][end.1];

    let mut answer = 0usize;
    for row in 0..race.maze.height {
        for col in 0..race.maze.width {
            if race.maze.grid[row][col] == '#' {
                for d in DIRS {
                    if let Some((next_row, next_col)) = race.try_move((row, col), d) {
                        if race.maze.grid[next_row][next_col] == '.' {
                            let new_dist =
                                dist_from_end[next_row][next_col] + dist_from_start[row][col] + 1;
                            if new_dist + 100 <= normal_dist {
                                answer += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    answer
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let race = Race::from(input);
    let (height, width) = (race.maze.height, race.maze.width);
    let (start, end) = (race.start, race.end);

    let dist_from_start = race.get_dist_from(start);
    let dist_from_end = race.get_dist_from(end);
    let normal_dist = dist_from_start[end.0][end.1];

    const M: usize = 20; // Maximum cheat duration
    let mut answer = 0usize;

    for row in 0..race.maze.height {
        for col in 0..race.maze.width {
            if race.maze.grid[row][col] == '#' && dist_from_start[row][col] != usize::MAX {
                for next_row in row.checked_sub(M).unwrap_or(0)..=(row + M).min(height - 1) {
                    for next_col in col.checked_sub(M).unwrap_or(0)..=(col + M).min(width - 1) {
                        let cheat_distance = next_row.abs_diff(row) + next_col.abs_diff(col);
                        if cheat_distance <= M {
                            if race.inside((next_row, next_col))
                                && race.maze.grid[next_row][next_col] == '.'
                            {
                                let new_dist = dist_from_end[next_row][next_col]
                                    + dist_from_start[row][col]
                                    + cheat_distance;
                                if new_dist + 100 <= normal_dist {
                                    answer += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 0);
    }
}
