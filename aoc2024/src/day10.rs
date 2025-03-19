use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect()
        })
        .collect()
}

fn resovle_head_point(
    map: &Vec<Vec<u8>>,
    (height, width): (usize, usize),
    position: (usize, usize),
    heads: &mut HashSet<(usize, usize)>,
) {
    if map[position.0 as usize][position.1 as usize] == 9 {
        heads.insert(position);
    } else {
        DIRECTIONS.iter().for_each(|direction| {
            if let Some(new_position) =
                get_possible_position(map, (height, width), position, *direction)
            {
                resovle_head_point(map, (height, width), new_position, heads);
            }
        });
    }
}

fn get_possible_position(
    map: &Vec<Vec<u8>>,
    (height, width): (usize, usize),
    position: (usize, usize),
    direction: (isize, isize),
) -> Option<(usize, usize)> {
    let next_x = position.0 as isize + direction.0;
    let next_y = position.1 as isize + direction.1;
    if next_x >= 0 && next_x < height as isize && next_y >= 0 && next_y < width as isize {
        let current = map[position.0 as usize][position.1 as usize];
        let next = map[next_x as usize][next_y as usize];
        if next == current + 1 {
            return Some((next_x as usize, next_y as usize));
        }
    };

    return None;
}

fn resovle_rating(
    map: &Vec<Vec<u8>>,
    (height, width): (usize, usize),
    position: (usize, usize),
    rating: &mut usize,
) {
    if map[position.0 as usize][position.1 as usize] == 9 {
        *rating += 1;
        return;
    }

    DIRECTIONS.iter().for_each(|direction| {
        if let Some(new_position) =
            get_possible_position(map, (height, width), position, *direction)
        {
            resovle_rating(map, (height, width), new_position, rating)
        }
    });
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let (height, width) = (map.len(), map[0].len());

    let mut trails: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 0 {
                let mut heads = HashSet::new();
                resovle_head_point(&map, (height, width), (i, j), &mut heads);
                trails.insert((i, j), heads);
            }
        }
    }

    trails.values().fold(0, |acc, heads| acc + heads.len())
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let (height, width) = (map.len(), map[0].len());

    let mut answer = 0;
    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 0 {
                resovle_rating(&map, (height, width), (i, j), &mut answer);
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 81);
    }
}
