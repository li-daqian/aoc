use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("Sensor at x=", "")
                .replace(": closest beacon is at x=", ",")
                .replace(" y=", "");
            let mut nums = line.split(',').map(|n| n.parse::<i64>().unwrap());
            let sx = nums.next().unwrap();
            let sy = nums.next().unwrap();
            let bx = nums.next().unwrap();
            let by = nums.next().unwrap();
            ((sx, sy), (bx, by))
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let sensors = parse(input);
    let target_y = 2_000_000;

    let mut cannot_be = vec![];
    let mut beacons_on_row = HashSet::new();

    for &((sx, sy), (bx, by)) in &sensors {
        let dist = (sx - bx).abs() + (sy - by).abs();
        let dy = (sy - target_y).abs();
        if dy > dist {
            continue;
        }
        let dx = dist - dy;
        cannot_be.push((sx - dx, sx + dx));
        if by == target_y {
            beacons_on_row.insert(bx);
        }
    }

    cannot_be.sort_unstable();
    let mut answer = 0;
    let mut max = i64::MIN;
    for &(start, end) in &cannot_be {
        if start > max {
            answer += end - start + 1;
        } else if end > max {
            answer += end - max;
        }
        max = max.max(end);
    }

    for &bx in &beacons_on_row {
        for &(start, end) in &cannot_be {
            if bx >= start && bx <= end {
                answer -= 1;
                break;
            }
        }
    }
    answer as usize
}

fn dist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn ipts(s1x: i64, s1y: i64, d1: i64, s2x: i64, s2y: i64, d2: i64) -> Vec<(i64, i64)> {
    let pts = [
        (s1x - d1, s2x - d2),
        (s1x - d1, s2x + d2),
        (s1x + d1, s2x - d2),
        (s1x + d1, s2x + d2),
    ];
    pts.iter()
        .map(|&(x1, x2)| ((x2 + s2y + x1 - s1y) / 2, (x2 + s2y - x1 + s1y) / 2))
        .collect()
}

fn intersect(s1: (i64, i64), d1: i64, s2: (i64, i64), d2: i64) -> Vec<(i64, i64)> {
    let mut v = ipts(s1.0, s1.1, d1, s2.0, s2.1, d2);
    v.extend(ipts(s2.0, s2.1, d2, s1.0, s1.1, d1));
    v
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let sensors: Vec<((i64, i64), i64)> = input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            let s = (nums[0], nums[1]);
            let b = (nums[2], nums[3]);
            let d = dist(s, b);
            (s, d)
        })
        .collect();

    let max_coord = 4_000_000;

    for (i, &(s1, d1)) in sensors.iter().enumerate() {
        for (j, &(s2, d2)) in sensors.iter().enumerate() {
            if i == j {
                continue;
            }
            for (x, y) in intersect(s1, d1 + 1, s2, d2 + 1) {
                if x < 0 || x > max_coord || y < 0 || y > max_coord {
                    continue;
                }
                if sensors.iter().all(|&(s, d)| dist((x, y), s) > d) {
                    return x * 4_000_000 + y;
                }
            }
        }
    }
    panic!("No solution found");
}
