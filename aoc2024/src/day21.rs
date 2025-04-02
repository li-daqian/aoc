use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    usize,
};

use aoc_runner_derive::aoc;

type Keypad<'a> = &'a [&'a [u8]];
const PAD1: Keypad = &[b"789", b"456", b"123", b" 0A"];
const PAD2: Keypad = &[b" ^A", b"<v>"];

fn pad_move(row: usize, col: usize, m: u8, keypad: Keypad) -> (usize, usize, Option<u8>) {
    match m {
        b'^' => (row.checked_sub(1).unwrap_or(usize::MAX), col, None),
        b'>' => (row, col.checked_add(1).unwrap_or(usize::MAX), None),
        b'v' => (row.checked_add(1).unwrap_or(usize::MAX), col, None),
        b'<' => (row, col.checked_sub(1).unwrap_or(usize::MAX), None),
        b'A' => (row, col, Some(keypad[row][col])),
        _ => unreachable!(),
    }
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
    while let Some(Reverse((d, (r, c), prev, out))) = q.pop() {
        if out == goal {
            cache.insert((goal, prev_m, pads), d);
            return d;
        }
        for &m in b"A^<v>" {
            let (rr, cc, x) = pad_move(r, c, m, PAD2);
            if *PAD2.get(rr).and_then(|row| row.get(cc)).unwrap_or(&b' ') == b' ' {
                continue;
            }
            let x = x.unwrap_or(0);
            if x != 0 && x != goal {
                continue;
            }
            // println!(
            //     "move:{} curr:{} prev:{} goal: {} pads:{}",
            //     m as char, PAD2[r][c] as char, prev as char, goal as char, pads
            // );
            // println!("{} {} {} {}", x as char, goal as char, prev as char, pads);
            let d = d + calculate_cost(cache, m, prev, pads - 1);
            q.push(Reverse((d, (rr, cc), m, x)));
        }
    }
    unreachable!()
}

fn solve(cache: &mut HashMap<(u8, u8, usize), usize>, code: &[u8], pads: usize) -> usize {
    let mut q = BinaryHeap::from(vec![Reverse((0, (3, 2), b'A', 0))]);
    let mut seen = HashMap::new();
    while let Some(Reverse((d, (r, c), prev, l))) = q.pop() {
        if l == code.len() {
            return d;
        }
        let k = ((r, c), prev, l);
        if seen.contains_key(&k) {
            continue;
        }
        seen.insert(k, d);
        for &m in b"A^<v>" {
            let (rr, cc, x) = pad_move(r, c, m, PAD1);
            if *PAD1.get(rr).and_then(|row| row.get(cc)).unwrap_or(&b' ') == b' ' {
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
            q.push(Reverse((d, (rr, cc), m, l)));
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
