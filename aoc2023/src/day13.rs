use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grid| {
            let grid = grid.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
            find_row(&grid, 0)
                .map(|n| (n + 1) * 100)
                .or_else(|| find_column(&grid, 0).map(|n| n + 1))
                .unwrap()
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grid| {
            let grid = grid.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
            find_row(&grid, 1)
                .map(|n| (n + 1) * 100)
                .or_else(|| find_column(&grid, 1).map(|n| n + 1))
                .unwrap()
        })
        .sum()
}

fn find_row(grid: &Vec<&[u8]>, limit: usize) -> Option<usize> {
    (0..grid.len() - 1).find(|i| {
        let incorrect = (0..(i + 1).min(grid.len() - i - 1))
            .map(|dr| {
                let left = i - dr;
                let right = i + dr + 1;
                (0..grid[0].len())
                    .filter(|&c| grid[left][c] != grid[right][c])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

fn find_column(grid: &Vec<&[u8]>, limit: usize) -> Option<usize> {
    (0..grid[0].len() - 1).find(|i| {
        let incorrect = (0..(i + 1).min(grid[0].len() - i - 1))
            .map(|dr| {
                let left = i - dr;
                let right = i + dr + 1;
                (0..grid.len())
                    .filter(|&r| grid[r][left] != grid[r][right])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 400);
    }
}
