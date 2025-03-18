use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut height = -1;
    let mut width = -1;
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        height = row as isize + 1;
        width = line.len() as isize;
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert(vec![])
                    .push((row as isize, col as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for positions in antennas.values() {
        for (i, (r1, c1)) in positions.iter().enumerate() {
            for (j, (r2, c2)) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let row = r2 + (r2 - r1);
                let col = c2 + (c2 - c1);
                if row >= 0 && row < height && col >= 0 && col < width {
                    antinodes.insert((row, col));
                }
            }
        }
    }

    return antinodes.len();
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut height = -1;
    let mut width = -1;
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        height = row as isize + 1;
        width = line.len() as isize;
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert(vec![])
                    .push((row as isize, col as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for positions in antennas.values() {
        for (i, (r1, c1)) in positions.iter().enumerate() {
            for (j, (r2, c2)) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let dr = r2 - r1;
                let dc = c2 - c1;
                for k in 0..=isize::MAX {
                    let row = r2 + dr * k;
                    let col = c2 + dc * k;
                    if row >= 0 && row < height && col >= 0 && col < width {
                        antinodes.insert((row, col));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return antinodes.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 34);
    }
}
