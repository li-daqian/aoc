use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    let (height, width) = (grid.len(), grid[0].len());

    let mut answer = height * 2 + (width - 2) * 2;

    for r in 1..height - 1 {
        for c in 1..width - 1 {
            let h = grid[r][c];
            // Left
            if (0..c).all(|cc| grid[r][cc] < h)
            // Right
            || (c + 1..width).all(|cc| grid[r][cc] < h)
            // Up
            || (0..r).all(|rr| grid[rr][c] < h)
            // Down
            || (r + 1..height).all(|rr| grid[rr][c] < h)
            {
                answer += 1;
            }
        }
    }

    answer
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    let (height, width) = (grid.len(), grid[0].len());

    let mut best = 0;
    for r in 0..height {
        for c in 0..width {
            let h = grid[r][c];

            // Up
            let mut up = 0;
            for rr in (0..r).rev() {
                up += 1;
                if grid[rr][c] >= h {
                    break;
                }
            }
            // Down
            let mut down = 0;
            for rr in r + 1..height {
                down += 1;
                if grid[rr][c] >= h {
                    break;
                }
            }
            // Left
            let mut left = 0;
            for cc in (0..c).rev() {
                left += 1;
                if grid[r][cc] >= h {
                    break;
                }
            }
            // Right
            let mut right = 0;
            for cc in c + 1..width {
                right += 1;
                if grid[r][cc] >= h {
                    break;
                }
            }

            let score = up * down * left * right;
            if score > best {
                best = score;
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 8);
    }
}
