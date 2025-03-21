use aoc_runner_derive::aoc;

struct Garden {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

impl Garden {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();

        Garden {
            grid,
            height,
            width,
        }
    }

    fn get_next(&self, (r, c): (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        let (r1, c1) = (r as isize + dir.0, c as isize + dir.1);
        if r1 < 0 || r1 >= self.height as isize || c1 < 0 || c1 >= self.width as isize {
            return None;
        }
        let (r1, c1) = (r1 as usize, c1 as usize);
        if self.grid[r][c] == self.grid[r1][c1] {
            return Some((r1, c1));
        }
        None
    }

    fn calcuate(
        &self,
        (r, c): (usize, usize),
        (area, perimeter): (&mut usize, &mut usize),
        visisted: &mut Vec<Vec<bool>>,
    ) {
        visisted[r][c] = true;
        *area += 1;

        for &dir in DIRECTIONS.iter() {
            if let Some(next) = self.get_next((r, c), dir) {
                if !visisted[next.0][next.1] {
                    self.calcuate(next, (area, perimeter), visisted);
                }
            } else {
                *perimeter += 1;
            }
        }
    }

    fn calcuate_2(
        &self,
        (r, c): (usize, usize),
        (area, perimeter): (&mut usize, &mut usize),
        visisted: &mut Vec<Vec<bool>>,
    ) {
        visisted[r][c] = true;
        *area += 1;

        for i in 0..DIRECTIONS.len() {
            let dir = DIRECTIONS[i];
            let dir1 = DIRECTIONS[(i + 1) % DIRECTIONS.len()];
            if !self.get_next((r, c), dir).is_some() && !self.get_next((r, c), dir1).is_some() {
                *perimeter += 1;
            }

            if self.get_next((r, c), dir).is_some()
                && self.get_next((r, c), dir1).is_some()
                && !self
                    .get_next((r, c), (dir.0 + dir1.0, dir.1 + dir1.1))
                    .is_some()
            {
                *perimeter += 1;
            }
        }

        for &dir in DIRECTIONS.iter() {
            if let Some(next) = self.get_next((r, c), dir) {
                if !visisted[next.0][next.1] {
                    self.calcuate_2(next, (area, perimeter), visisted);
                }
            }
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let garden = Garden::from(input);
    let mut visisted = vec![vec![false; garden.width]; garden.height];
    let mut answer = 0;
    for r in 0..garden.height {
        for c in 0..garden.width {
            if !visisted[r][c] {
                let (mut area, mut perimeter) = (0, 0);
                garden.calcuate((r, c), (&mut area, &mut perimeter), &mut visisted);
                answer += area * perimeter;
            }
        }
    }
    answer
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let garden = Garden::from(input);
    let mut visisted = vec![vec![false; garden.width]; garden.height];
    let mut answer = 0;
    for r in 0..garden.height {
        for c in 0..garden.width {
            if !visisted[r][c] {
                let (mut area, mut perimeter) = (0, 0);
                garden.calcuate_2((r, c), (&mut area, &mut perimeter), &mut visisted);
                answer += area * perimeter;
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1206);
    }
}
