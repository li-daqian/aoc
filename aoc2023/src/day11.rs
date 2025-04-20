use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let galaxies = parse_input(input, &1);

    (0..galaxies.len())
        .into_iter()
        .map(|i| {
            let (row, col) = galaxies[i];
            (i..galaxies.len())
                .into_iter()
                .map(|j| {
                    let (row2, col2) = galaxies[j];
                    row2.abs_diff(row) + col2.abs_diff(col)
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let galaxies = parse_input(input, &999999);

    (0..galaxies.len())
        .into_iter()
        .map(|i| {
            let (row, col) = galaxies[i];
            (i..galaxies.len())
                .into_iter()
                .map(|j| {
                    let (row2, col2) = galaxies[j];
                    row2.abs_diff(row) + col2.abs_diff(col)
                })
                .sum::<usize>()
        })
        .sum()
}

fn parse_input(input: &str, expansion_time: &usize) -> Vec<(usize, usize)> {
    let (mut height, mut width) = (0, 0);
    let mut galaxies = Vec::new();
    for (row, line) in input.lines().enumerate() {
        height = row + 1;
        width = line.len();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut galaxies_row = vec![false; height];
    let mut galaxies_col = vec![false; width];
    for (row, col) in &galaxies {
        galaxies_row[*row] = true;
        galaxies_col[*col] = true;
    }

    let expansion_galaxies = galaxies
        .iter()
        .map(|&(row, col)| {
            let new_row =
                row + galaxies_row.iter().take(row).filter(|&&x| !x).count() * expansion_time;
            let new_col =
                col + galaxies_col.iter().take(col).filter(|&&x| !x).count() * expansion_time;
            (new_row, new_col)
        })
        .collect::<Vec<_>>();

    expansion_galaxies
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 82000210);
    }
}
