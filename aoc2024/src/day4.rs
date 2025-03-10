use aoc_runner_derive::aoc;

#[inline]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let matrix = parse(input);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let directions = [
        Direction::North,
        Direction::NorthWest,
        Direction::West,
        Direction::SouthWest,
        Direction::South,
        Direction::SouthEast,
        Direction::East,
        Direction::NorthEast,
    ];

    matrix
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, &tile)| (tile == 'X').then_some((x, y)))
        })
        .map(|(x, y)| {
            directions
                .iter()
                .filter(|direction| {
                    ['M', 'A', 'S']
                        .iter()
                        .try_fold((x, y), |(x, y), &letter| {
                            direction
                                .propagate((x, y), rows, cols)
                                .and_then(|(x, y)| (matrix[x][y] == letter).then_some((x, y)))
                        })
                        .is_some()
                })
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let matrix = parse(input);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let directions = [
        (Direction::NorthWest, Direction::SouthEast),
        (Direction::NorthEast, Direction::SouthWest),
    ];
    matrix
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, &tile)| (tile == 'A').then_some((x, y)))
        })
        .filter(|&(x, y)| {
            directions.iter().all(|(dir1, dir2)| {
                dir1.propagate((x, y), rows, cols).and_then(|(x1, y1)| {
                    dir2.propagate((x, y), rows, cols).and_then(|(x2, y2)| {
                        Some(
                            (matrix[x1][y1] == 'M' && matrix[x2][y2] == 'S')
                                || (matrix[x1][y1] == 'S' && matrix[x2][y2] == 'M'),
                        )
                    })
                }) == Some(true)
            })
        })
        .count()
}

enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl Direction {
    fn propagate(
        &self,
        (x, y): (usize, usize),
        rows: usize,
        cols: usize,
    ) -> Option<(usize, usize)> {
        Some(match self {
            Self::North => (x, y.checked_sub(1)?),
            Self::NorthWest => (x.checked_sub(1)?, y.checked_sub(1)?),
            Self::West => (x.checked_sub(1)?, y),
            Self::SouthWest => (x.checked_sub(1)?, {
                let y = y + 1;
                (y < rows).then_some(y)?
            }),
            Self::South => (x, {
                let y = y + 1;
                (y < rows).then_some(y)?
            }),
            Self::SouthEast => (
                {
                    let x = x + 1;
                    (x < cols).then_some(x)?
                },
                {
                    let y = y + 1;
                    (y < rows).then_some(y)?
                },
            ),
            Self::East => (
                {
                    let x = x + 1;
                    (x < cols).then_some(x)?
                },
                y,
            ),
            Self::NorthEast => (
                {
                    let x = x + 1;
                    (x < cols).then_some(x)?
                },
                y.checked_sub(1)?,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 18);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 9);
    }
}
