use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    usize,
};

use aoc_runner_derive::aoc;

type Keypad<'a> = &'a [&'a [u8]];
const OUT_PAD: Keypad = &[b"789", b"456", b"123", b" 0A"];
const MOVE_PAD: Keypad = &[b" ^A", b"<v>"];

fn pad_move(keypad: Keypad, row: usize, col: usize, m: u8) -> (Option<(usize, usize)>, Option<u8>) {
    if m == b'A' {
        return (Some((row, col)), Some(keypad[row][col]));
    }
    let (row, col) = (row as isize, col as isize);
    let (height, width) = (keypad.len() as isize, keypad[0].len() as isize);
    let (next_row, next_col) = match m {
        b'^' => (row - 1, col),
        b'>' => (row, col + 1),
        b'v' => (row + 1, col),
        b'<' => (row, col - 1),
        _ => unreachable!(),
    };
    if next_row < 0 || next_row >= height || next_col < 0 || next_col >= width {
        return (None, None);
    }
    let (next_row, next_col) = (next_row as usize, next_col as usize);
    if keypad[next_row][next_col] == b' ' {
        return (None, None);
    }
    return (Some((next_row, next_col)), None);
}

fn calculate_cost(
    cache: &mut HashMap<(u8, u8, usize), usize>,
    goal: u8,
    prev_m: u8,
    pads: usize,
) -> usize {
    if pads == 0 {
        return 1;
    }
    if let Some(&d) = cache.get(&(goal, prev_m, pads)) {
        return d;
    }
    let start = match prev_m {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => unreachable!(),
    };
    let mut q = BinaryHeap::from(vec![Reverse((0, start, b'A', 0))]);
    while let Some(Reverse((d, (row, col), prev, out))) = q.pop() {
        if out == goal {
            cache.insert((goal, prev_m, pads), d);
            return d;
        }
        for &m in b"A^<v>" {
            let (next, x) = pad_move(MOVE_PAD, row, col, m);
            if next.is_none() {
                continue;
            }
            let x = x.unwrap_or(0);
            if x != 0 && x != goal {
                continue;
            }
            let d = d + calculate_cost(cache, m, prev, pads - 1);
            q.push(Reverse((d, next.unwrap(), m, x)));
        }
    }
    unreachable!()
}

fn solve(cache: &mut HashMap<(u8, u8, usize), usize>, code: &[u8], pads: usize) -> usize {
    let mut q = BinaryHeap::from(vec![Reverse((0, (3, 2), b'A', 0))]);
    let mut seen = HashMap::new();
    while let Some(Reverse((d, (row, col), prev, l))) = q.pop() {
        if l == code.len() {
            return d;
        }
        let k = ((row, col), prev, l);
        if seen.contains_key(&k) {
            continue;
        }
        seen.insert(k, d);
        for &m in b"A^<v>" {
            let (next, x) = pad_move(OUT_PAD, row, col, m);
            if next.is_none() {
                continue;
            }
            let mut l = l;
            if let Some(x) = x {
                if x != code[l] {
                    continue;
                }
                l += 1;
            }
            let d = d + calculate_cost(cache, m, prev, pads);
            q.push(Reverse((d, next.unwrap(), m, l)));
        }
    }
    unreachable!()
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let mut answer = 0;
    let mut cache = HashMap::new();
    for line in input.lines() {
        let n = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        answer += n * solve(&mut cache, line.as_bytes(), 2);
        // solve(&mut cache, line.as_bytes(), 2);
    }
    answer
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    let mut answer = 0;
    let mut cache = HashMap::new();
    for line in input.lines() {
        let n = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        answer += n * solve(&mut cache, line.as_bytes(), 25);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 126384);
    }
}
