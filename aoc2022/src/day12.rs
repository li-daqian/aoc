use std::collections::VecDeque;

use aoc_runner_derive::aoc;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let (hight, width) = (grid.len(), grid[0].len());

    let mut start = (0, 0);
    let mut end = (0, 0);
    for r in 0..hight {
        for c in 0..width {
            match grid[r][c] {
                b'S' => {
                    start = (r, c);
                    grid[r][c] = b'a';
                }
                b'E' => {
                    end = (r, c);
                    grid[r][c] = b'z';
                }
                _ => {}
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut cost = vec![vec![usize::MAX; width]; hight];
    queue.push_back((0, start.0, start.1));
    cost[start.0][start.1] = 0;

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((curr_cost, r, c)) = queue.pop_front() {
        if (r, c) == end {
            return curr_cost;
        }

        for (dr, dc) in dirs {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0 && nr < hight as isize && nc >= 0 && nc < width as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] <= grid[r][c] + 1 && curr_cost + 1 < cost[nr][nc] {
                    cost[nr][nc] = curr_cost + 1;
                    queue.push_back((curr_cost + 1, nr, nc));
                }
            }
        }
    }

    unreachable!("No path found");
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let (hight, width) = (grid.len(), grid[0].len());

    let mut starts = vec![];
    let mut end = (0, 0);
    for r in 0..hight {
        for c in 0..width {
            match grid[r][c] {
                b'S' => {
                    starts.push((r, c));
                    grid[r][c] = b'a';
                }
                b'E' => {
                    end = (r, c);
                    grid[r][c] = b'z';
                }
                b'a' => {
                    starts.push((r, c));
                }
                _ => {}
            }
        }
    }

    let mut cost = vec![vec![usize::MAX; width]; hight];
    let mut queue = VecDeque::new();
    for &(r, c) in &starts {
        cost[r][c] = 0;
        queue.push_back((0, r, c));
    }

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((curr_cost, r, c)) = queue.pop_front() {
        if (r, c) == end {
            return curr_cost;
        }

        for (dr, dc) in dirs {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0 && nr < hight as isize && nc >= 0 && nc < width as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] <= grid[r][c] + 1 && curr_cost + 1 < cost[nr][nc] {
                    cost[nr][nc] = curr_cost + 1;
                    queue.push_back((curr_cost + 1, nr, nc));
                }
            }
        }
    }

    unreachable!("No path found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 29);
    }
}
