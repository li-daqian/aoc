use aoc_runner_derive::aoc;

type Position = (i32, i32);

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// Each entry contains the movement delta followed by the three cells that
// must be empty before proposing that movement.
const DIRECTIONS: [[(i32, i32); 4]; 4] = [
    [(-1, 0), (-1, -1), (-1, 0), (-1, 1)], // north
    [(1, 0), (1, -1), (1, 0), (1, 1)],     // south
    [(0, -1), (-1, -1), (0, -1), (1, -1)], // west
    [(0, 1), (-1, 1), (0, 1), (1, 1)],     // east
];

struct Grid {
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
    width: usize,
    occupied: Vec<bool>,
    proposal_count: Vec<u8>,
}

impl Grid {
    fn new(elves: &[Position]) -> Self {
        let mut grid = Self {
            min_row: 0,
            max_row: 0,
            min_col: 0,
            max_col: 0,
            width: 0,
            occupied: Vec::new(),
            proposal_count: Vec::new(),
        };
        grid.resize_for(elves);
        grid
    }

    fn resize_for(&mut self, elves: &[Position]) {
        let min_row = elves.iter().map(|&(row, _)| row).min().unwrap();
        let max_row = elves.iter().map(|&(row, _)| row).max().unwrap();
        let min_col = elves.iter().map(|&(_, col)| col).min().unwrap();
        let max_col = elves.iter().map(|&(_, col)| col).max().unwrap();

        // Keep a margin so a proposed one-step move always remains in bounds.
        self.min_row = min_row - 2;
        self.max_row = max_row + 2;
        self.min_col = min_col - 2;
        self.max_col = max_col + 2;
        self.width = (self.max_col - self.min_col + 1) as usize;
        let size = (self.max_row - self.min_row + 1) as usize * self.width;
        self.occupied = vec![false; size];
        self.proposal_count = vec![0; size];
        for &position in elves {
            let index = self.index(position);
            self.occupied[index] = true;
        }
    }

    fn ensure_margin(&mut self, elves: &[Position]) {
        let near_edge = elves.iter().any(|&(row, col)| {
            row <= self.min_row + 1
                || row >= self.max_row - 1
                || col <= self.min_col + 1
                || col >= self.max_col - 1
        });
        if near_edge {
            self.resize_for(elves);
        }
    }

    fn index(&self, (row, col): Position) -> usize {
        debug_assert!(self.min_row <= row && row <= self.max_row);
        debug_assert!(self.min_col <= col && col <= self.max_col);
        ((row - self.min_row) as usize) * self.width + (col - self.min_col) as usize
    }
}

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter_map(move |(col, cell)| (cell == b'#').then_some((row as i32, col as i32)))
        })
        .collect()
}

fn round(elves: &[Position], grid: &mut Grid, direction_offset: usize) -> (Vec<Position>, bool) {
    grid.ensure_margin(elves);
    let mut proposals = Vec::with_capacity(elves.len());

    for &(row, col) in elves {
        let has_neighbor = NEIGHBORS
            .iter()
            .any(|&(dr, dc)| grid.occupied[grid.index((row + dr, col + dc))]);
        let destination = if !has_neighbor {
            (row, col)
        } else {
            (0..4)
                .find_map(|offset| {
                    let direction = DIRECTIONS[(direction_offset + offset) % 4];
                    direction[1..]
                        .iter()
                        .all(|&(dr, dc)| !grid.occupied[grid.index((row + dr, col + dc))])
                        .then_some((row + direction[0].0, col + direction[0].1))
                })
                .unwrap_or((row, col))
        };
        let destination_index = grid.index(destination);
        grid.proposal_count[destination_index] += 1;
        proposals.push(((row, col), destination, destination_index));
    }

    let mut moved = false;
    let mut next_elves = Vec::with_capacity(elves.len());
    for &(source, destination, destination_index) in &proposals {
        if grid.proposal_count[destination_index] == 1 {
            moved |= source != destination;
            next_elves.push(destination);
        } else {
            next_elves.push(source);
        }
    }

    for &(_, _, destination_index) in &proposals {
        grid.proposal_count[destination_index] = 0;
    }
    for &position in elves {
        let index = grid.index(position);
        grid.occupied[index] = false;
    }
    for &position in &next_elves {
        let index = grid.index(position);
        grid.occupied[index] = true;
    }

    (next_elves, moved)
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let mut elves = parse(input);
    let mut grid = Grid::new(&elves);
    for round_number in 0..10 {
        elves = round(&elves, &mut grid, round_number % 4).0;
    }

    let min_row = elves
        .iter()
        .map(|&(row, _)| row)
        .min()
        .expect("at least one elf");
    let max_row = elves
        .iter()
        .map(|&(row, _)| row)
        .max()
        .expect("at least one elf");
    let min_col = elves
        .iter()
        .map(|&(_, col)| col)
        .min()
        .expect("at least one elf");
    let max_col = elves
        .iter()
        .map(|&(_, col)| col)
        .max()
        .expect("at least one elf");
    let area = i64::from(max_row - min_row + 1) * i64::from(max_col - min_col + 1);
    area - elves.len() as i64
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> i64 {
    let mut elves = parse(input);
    let mut grid = Grid::new(&elves);
    let mut round_number = 1;

    loop {
        let (next_elves, moved) = round(&elves, &mut grid, (round_number - 1) % 4);
        if !moved {
            return round_number as i64;
        }
        elves = next_elves;
        round_number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let sample = indoc! {
            "
            ....#..
            ..###.#
            #...#.#
            .#...##
            #.###..
            ##.#.##
            .#..#..
            "
        };
        assert_eq!(part1(sample), 110);
        assert_eq!(part2(sample), 20);
    }
}
