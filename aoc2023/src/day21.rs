use aoc_runner_derive::aoc;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn cal_positions(grid: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
    let (height, width) = (grid.len(), grid[0].len());

    let mut positions = vec![vec![false; width]; height];
    positions[start.0][start.1] = true;

    for _ in 0..steps {
        let mut new_positions = vec![vec![false; width]; height];
        for r in 0..height {
            for c in 0..width {
                if positions[r][c] {
                    for (dr, dc) in &DIRECTIONS {
                        let (nr, nc) = (r as isize + dr, c as isize + dc);
                        if nr >= 0
                            && nr < height as isize
                            && nc >= 0
                            && nc < width as isize
                            && grid[nr as usize][nc as usize] == '.'
                        {
                            new_positions[nr as usize][nc as usize] = true;
                        }
                    }
                }
            }
        }
        positions = new_positions;
    }

    positions
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&b| b)
        .count()
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let (grid, start) = input
        .lines()
        .fold((vec![], (0, 0)), |(mut grid, mut start), line| {
            grid.push(line.chars().collect::<Vec<_>>());
            if let Some(c) = grid.last().unwrap().iter().position(|b| *b == 'S') {
                start = (grid.len() - 1, c);
                grid.last_mut().unwrap()[c] = '.';
            }
            (grid, start)
        });

    cal_positions(&grid, start, 64)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    let (grid, start) = input
        .lines()
        .fold((vec![], (0, 0)), |(mut grid, mut start), line| {
            grid.push(line.chars().collect::<Vec<_>>());
            if let Some(c) = grid.last().unwrap().iter().position(|b| *b == 'S') {
                start = (grid.len() - 1, c);
                grid.last_mut().unwrap()[c] = '.';
            }
            (grid, start)
        });

    let steps = 26501365;
    let (height, width) = (grid.len(), grid[0].len());
    assert_eq!(height, width);
    assert_eq!(start.0, start.1);
    assert_eq!(start.0, height / 2);
    assert_eq!(steps % height, height / 2);

    let steps = 26501365;
    let size = height;
    let grid_width = steps / size - 1;

    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = cal_positions(&grid, start, size * 2 + 1);
    let even_points = cal_positions(&grid, start, size * 2);

    let (sr, sc) = start;

    // Corners
    let corner_t = cal_positions(&grid, (size - 1, sc), size - 1);
    let corner_r = cal_positions(&grid, (sr, 0), size - 1);
    let corner_b = cal_positions(&grid, (0, sc), size - 1);
    let corner_l = cal_positions(&grid, (sr, size - 1), size - 1);

    // Small corners
    let small_tr = cal_positions(&grid, (size - 1, 0), size / 2 - 1);
    let small_tl = cal_positions(&grid, (size - 1, size - 1), size / 2 - 1);
    let small_br = cal_positions(&grid, (0, 0), size / 2 - 1);
    let small_bl = cal_positions(&grid, (0, size - 1), size / 2 - 1);

    // Large corners
    let large_tr = cal_positions(&grid, (size - 1, 0), size * 3 / 2 - 1);
    let large_tl = cal_positions(&grid, (size - 1, size - 1), size * 3 / 2 - 1);
    let large_br = cal_positions(&grid, (0, 0), size * 3 / 2 - 1);
    let large_bl = cal_positions(&grid, (0, size - 1), size * 3 / 2 - 1);

    odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 42);
    }
}
