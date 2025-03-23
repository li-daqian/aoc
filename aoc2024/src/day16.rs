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

impl Race {
    fn from(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid: Vec<Vec<char>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                if let Some(col) = line.find('S') {
                    start = (row, col);
                    line.replace('S', ".");
                }
                if let Some(col) = line.find('E') {
                    end = (row, col);
                    line.replace('E', ".");
                }
                line.chars().collect()
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
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 11048);
    }
}
