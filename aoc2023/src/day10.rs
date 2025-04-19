use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    vec,
};

use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let (maze, start) = parse_maze(input);
    let height = maze.len();
    let width = maze[0].len();

    let mut costs = vec![vec![usize::MAX; width]; height];
    let mut queue = BinaryHeap::<Reverse<(usize, (usize, usize))>>::new();

    queue.push(Reverse((0, start)));
    while let Some(Reverse((cost, (row, col)))) = queue.pop() {
        if cost > costs[row][col] {
            continue;
        }
        costs[row][col] = cost;
        for &dir in &maze[row][col].0 {
            let (dr, dc) = DIRECTIONS[dir];
            let (new_row, new_col) = (row as isize + dr, col as isize + dc);
            if new_row < 0 || new_row >= height as isize || new_col < 0 || new_col >= width as isize
            {
                continue;
            }
            let (new_row, new_col) = (new_row as usize, new_col as usize);
            if maze[new_row][new_col]
                .0
                .iter()
                .any(|&d| is_connected(d, dir))
            {
                queue.push(Reverse((cost + 1, (new_row, new_col))));
            }
        }
    }

    *costs
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cost| cost < usize::MAX)
        .max()
        .unwrap()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let (maze, start) = parse_maze(input);
    let (height, width) = (maze.len(), maze[0].len());

    let mut answers = 0;
    let loop_path = find_loop(&maze, start).unwrap();
    for row in 0..height {
        let mut inside = false;
        for col in 0..width {
            if !loop_path.contains(&(row, col)) {
                answers += inside as usize;
            } else {
                match maze[row][col].1 {
                    b'|' | b'J' | b'L' => inside = !inside,
                    _ => {}
                }
            }
        }
    }
    answers
}

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_connected(a: usize, b: usize) -> bool {
    (a + 2) % DIRECTIONS.len() == b
}

fn parse_maze(input: &str) -> (Vec<Vec<(Vec<usize>, u8)>>, (usize, usize)) {
    let mut start = (0, 0);
    let maze = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => (vec![], b'.'),
                    '|' => (vec![NORTH, SOUTH], b'|'),
                    '-' => (vec![WEST, EAST], b'-'),
                    'S' => {
                        start = (row, col);
                        (vec![NORTH, EAST, SOUTH, WEST], b'S')
                    }
                    'J' => (vec![NORTH, WEST], b'J'),
                    'L' => (vec![NORTH, EAST], b'L'),
                    'F' => (vec![SOUTH, EAST], b'F'),
                    '7' => (vec![SOUTH, WEST], b'7'),
                    _ => panic!("Unknown character: {}", c),
                })
                .collect()
        })
        .collect();
    (maze, start)
}

fn find_loop(
    maze: &Vec<Vec<(Vec<usize>, u8)>>,
    start: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let (height, width) = (maze.len(), maze[0].len());

    let mut path = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((start, usize::MAX));
    while let Some((pos, prev_dir)) = queue.pop_front() {
        if path.contains(&pos) {
            continue;
        }
        path.insert(pos);
        let (row, col) = pos;
        for &dir in &maze[row][col].0 {
            if is_connected(dir, prev_dir) {
                continue;
            }
            let (dr, dc) = DIRECTIONS[dir];
            let (new_row, new_col) = (row as isize + dr, col as isize + dc);
            if new_row < 0 || new_row >= height as isize || new_col < 0 || new_col >= width as isize
            {
                continue;
            }
            let (new_row, new_col) = (new_row as usize, new_col as usize);
            if maze[new_row][new_col]
                .0
                .iter()
                .any(|&d| is_connected(d, dir))
            {
                if path.contains(&(new_row, new_col)) {
                    return Some(path);
                }
                queue.push_back(((new_row, new_col), dir));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 8);
    }

    #[test]
    fn test_part2() {
        let sample = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(part2(sample), 10);

        let sample = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};
        assert_eq!(part2(sample), 4);

        let sample = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
        assert_eq!(part2(sample), 8);
    }
}
