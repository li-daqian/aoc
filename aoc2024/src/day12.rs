use aoc_runner_derive::aoc;

struct Garden {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Garden {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();

        Garden {
            grid,
            height,
            width,
        }
    }

    fn get_next(&self, (r, c): (usize, usize), dir: &Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::North if r > 0 => Some((r - 1, c)),
            Direction::East if c < self.width - 1 => Some((r, c + 1)),
            Direction::South if r < self.height - 1 => Some((r + 1, c)),
            Direction::West if c > 0 => Some((r, c - 1)),
            _ => None,
        }
    }

    fn calcuate(
        &self,
        (r, c): (usize, usize),
        (area, perimeter): (&mut usize, &mut usize),
        visisted: &mut Vec<Vec<bool>>,
    ) {
        if visisted[r][c] {
            return;
        }
        visisted[r][c] = true;
        *area += 1;

        for d in DIRECTIONS.iter() {
            if let Some(p) = self.get_next((r, c), d) {
                if self.grid[p.0][p.1] == self.grid[r][c] {
                    self.calcuate(p, (area, perimeter), visisted);
                } else {
                    *perimeter += 1;
                }
            } else {
                *perimeter += 1;
            }
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let garden = Garden::new(input);
    let mut visisted = vec![vec![false; garden.width]; garden.height];
    let mut answer = 0;
    for r in 0..garden.height {
        for c in 0..garden.width {
            let (mut area, mut perimeter) = (0, 0);
            garden.calcuate((r, c), (&mut area, &mut perimeter), &mut visisted);
            answer += area * perimeter;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1930);
    }
}
