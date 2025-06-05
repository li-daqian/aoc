use aoc_runner_derive::aoc;
use std::collections::{HashSet, VecDeque};

type Pos = (i32, i32, i32);

fn parse(input: &str) -> HashSet<Pos> {
    input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split(',').filter_map(|n| n.parse().ok());
            Some((nums.next()?, nums.next()?, nums.next()?))
        })
        .collect()
}

const DIRS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let cubes = parse(input);
    cubes
        .iter()
        .map(|&(x, y, z)| {
            DIRS.iter()
                .filter(|&&(dx, dy, dz)| !cubes.contains(&(x + dx, y + dy, z + dz)))
                .count()
        })
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let cubes = parse(input);

    // Find bounding box
    let min_x = cubes.iter().map(|&(x, _, _)| x).min().unwrap();
    let max_x = cubes.iter().map(|&(x, _, _)| x).max().unwrap();
    let min_y = cubes.iter().map(|&(_, y, _)| y).min().unwrap();
    let max_y = cubes.iter().map(|&(_, y, _)| y).max().unwrap();
    let min_z = cubes.iter().map(|&(_, _, z)| z).min().unwrap();
    let max_z = cubes.iter().map(|&(_, _, z)| z).max().unwrap();

    // Flood fill from outside the bounding box
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((min_x - 1, min_y - 1, min_z - 1));
    visited.insert((min_x - 1, min_y - 1, min_z - 1));

    while let Some((x, y, z)) = queue.pop_front() {
        for (dx, dy, dz) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            let nz = z + dz;
            // Stay within expanded bounding box
            if nx < min_x - 1
                || nx > max_x + 1
                || ny < min_y - 1
                || ny > max_y + 1
                || nz < min_z - 1
                || nz > max_z + 1
            {
                continue;
            }
            let np = (nx, ny, nz);
            if cubes.contains(&np) || visited.contains(&np) {
                continue;
            }
            visited.insert(np);
            queue.push_back(np);
        }
    }

    // Count faces adjacent to outside air
    cubes
        .iter()
        .map(|&(x, y, z)| {
            DIRS.iter()
                .filter(|&&(dx, dy, dz)| visited.contains(&(x + dx, y + dy, z + dz)))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 58);
    }
}
