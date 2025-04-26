use std::{cmp::Reverse, collections::BinaryHeap, usize, vec};

use aoc_runner_derive::aoc;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn cal_cost(grid: &Vec<Vec<usize>>, minstep: usize, maxstep: usize) -> usize {
    let (height, width) = (grid.len(), grid[0].len());

    let mut costs = vec![vec![[usize::MAX; 4]; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::<Reverse<(usize, usize, usize, usize)>>::new();
    queue.push(Reverse((0, 0, 0, 0)));
    queue.push(Reverse((0, 0, 0, 1)));

    while let Some(Reverse((cur_cost, r, c, curr_d))) = queue.pop() {
        if costs[r][c][curr_d] <= cur_cost {
            continue;
        }

        costs[r][c][curr_d] = cur_cost;

        for next_d in 0..4 {
            if curr_d == next_d || curr_d.abs_diff(next_d) == 2 {
                continue;
            }
            let mut next_cost = cur_cost;
            for dist in 1..=maxstep {
                let (dx, dy) = DIRECTIONS[next_d];
                let new_r = r as isize + dx * dist as isize;
                let new_c = c as isize + dy * dist as isize;

                if new_r >= 0 && new_c >= 0 && new_r < height as isize && new_c < width as isize {
                    next_cost += grid[new_r as usize][new_c as usize];
                    if dist < minstep {
                        continue;
                    }
                    let (new_r, new_c) = (new_r as usize, new_c as usize);
                    if costs[new_r][new_c][next_d] <= next_cost {
                        continue;
                    }
                    queue.push(Reverse((next_cost, new_r, new_c, next_d)));
                }
            }
        }
    }

    *costs[height - 1][width - 1].iter().min().unwrap()
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    cal_cost(&grid, 1, 3)
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    cal_cost(&grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 94);
    }
}
