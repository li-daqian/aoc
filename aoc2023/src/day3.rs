use std::collections::HashMap;

use aoc_runner_derive::aoc;

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut numbers = vec![];
    for (row, line) in grid.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let c = line[col];
            if c.is_ascii_digit() {
                let start = col;
                while col + 1 < line.len() && line[col + 1].is_ascii_digit() {
                    col += 1;
                }
                numbers.push((row, start, col));
            }
            col += 1;
        }
    }

    let find_symbol = |grid: &[&[u8]], row: usize, col: usize| -> bool {
        let height = grid.len();
        let width = grid[0].len();

        for (dr, dc) in DIRECTIONS {
            let mut r = row as i32;
            let mut c = col as i32;

            r += dr;
            c += dc;

            if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                continue;
            }

            let symbol = grid[r as usize][c as usize];
            if !symbol.is_ascii_digit() && symbol != b'.' {
                return true;
            }
        }

        false
    };

    numbers
        .iter()
        .filter_map(|&(row, start_col, end_col)| {
            let mut valid = false;
            for col in start_col..=end_col {
                if find_symbol(&grid, row, col) {
                    valid = true;
                    break;
                }
            }
            if valid {
                let mut num = (grid[row][start_col] - b'0') as usize;
                for col in start_col + 1..=end_col {
                    num = num * 10 + (grid[row][col] - b'0') as usize;
                }
                Some(num)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut numbers = vec![];
    for (row, line) in grid.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let c = line[col];
            if c.is_ascii_digit() {
                let start = col;
                while col + 1 < line.len() && line[col + 1].is_ascii_digit() {
                    col += 1;
                }
                numbers.push((row, start, col));
            }
            col += 1;
        }
    }

    let find_symbol = |grid: &[&[u8]], row: usize, col: usize| -> Option<(usize, usize)> {
        let height = grid.len();
        let width = grid[0].len();

        for (dr, dc) in DIRECTIONS {
            let mut r = row as i32;
            let mut c = col as i32;

            r += dr;
            c += dc;

            if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                continue;
            }

            let symbol = grid[r as usize][c as usize];
            if symbol == b'*' {
                return Some((r as usize, c as usize));
            }
        }

        None
    };

    numbers
        .iter()
        .fold(
            HashMap::<(usize, usize), Vec<usize>>::new(),
            |mut acc, &(row, start_col, end_col)| {
                for col in start_col..=end_col {
                    if let Some((r, c)) = find_symbol(&grid, row, col) {
                        let mut num = (grid[row][start_col] - b'0') as usize;
                        for col in start_col + 1..=end_col {
                            num = num * 10 + (grid[row][col] - b'0') as usize;
                        }
                        acc.entry((r, c)).or_default().push(num);
                        break;
                    }
                }
                acc
            },
        )
        .values()
        .filter_map(|numbers| {
            if numbers.len() != 2 {
                return None;
            }
            Some(numbers.iter().fold(1usize, |acc, &num| acc * num))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 467835);
    }
}
