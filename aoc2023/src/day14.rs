use aoc_runner_derive::aoc;

fn go_north(grid: &mut Vec<Vec<char>>, (height, width): (usize, usize)) {
    for col in 0..width {
        let mut empty_row = 0;
        for row in 0..height {
            if grid[row][col] == '#' {
                empty_row = row + 1;
            } else if grid[row][col] == 'O' {
                if empty_row != row {
                    grid[empty_row][col] = 'O';
                    grid[row][col] = '.';
                }
                empty_row += 1;
            }
        }
    }
}

fn go_east(grid: &mut Vec<Vec<char>>, (height, width): (usize, usize)) {
    for row in 0..height {
        let mut empty_col = width - 1;
        for col in (0..width).rev() {
            if grid[row][col] == '#' {
                if col > 0 {
                    empty_col = col - 1;
                }
            } else if grid[row][col] == 'O' {
                if empty_col != col {
                    grid[row][empty_col] = 'O';
                    grid[row][col] = '.';
                }
                if empty_col > 0 {
                    empty_col -= 1;
                }
            }
        }
    }
}

fn go_south(grid: &mut Vec<Vec<char>>, (height, width): (usize, usize)) {
    for col in 0..width {
        let mut empty_row = height - 1;
        for row in (0..height).rev() {
            if grid[row][col] == '#' {
                if row > 0 {
                    empty_row = row - 1;
                }
            } else if grid[row][col] == 'O' {
                if empty_row != row {
                    grid[empty_row][col] = 'O';
                    grid[row][col] = '.';
                }
                if empty_row > 0 {
                    empty_row -= 1;
                }
            }
        }
    }
}

fn go_west(grid: &mut Vec<Vec<char>>, (height, width): (usize, usize)) {
    for row in 0..height {
        let mut empty_col = 0;
        for col in 0..width {
            if grid[row][col] == '#' {
                empty_col = col + 1;
            } else if grid[row][col] == 'O' {
                if empty_col != col {
                    grid[row][empty_col] = 'O';
                    grid[row][col] = '.';
                }
                empty_col += 1;
            }
        }
    }
}

fn cal_load(grid: &Vec<Vec<char>>, (height, width): (usize, usize)) -> usize {
    (0..height)
        .flat_map(|row| (0..width).map(move |col| (row, col)))
        .filter(|&(row, col)| grid[row][col] == 'O')
        .map(|(row, _)| height - row)
        .sum()
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (height, width) = (grid.len(), grid[0].len());

    go_north(&mut grid, (height, width));
    cal_load(&grid, (height, width))
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (height, width) = (grid.len(), grid[0].len());

    let mut seen = vec![];
    loop {
        go_north(&mut grid, (height, width));
        go_west(&mut grid, (height, width));
        go_south(&mut grid, (height, width));
        go_east(&mut grid, (height, width));

        if seen.contains(&grid) {
            break;
        }
        seen.push(grid.clone());
    }

    let first = seen.iter().position(|g| g == &grid).unwrap();
    cal_load(
        &seen[(1000000000 - first) % (seen.len() - first) + first - 1],
        (height, width),
    )
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 64);
    }
}
