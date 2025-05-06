use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::aoc;

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let over_laps = |a: &Vec<usize>, b: &Vec<usize>| {
        a[0].max(b[0]) <= a[3].min(b[3]) && a[1].max(b[1]) <= a[4].min(b[4])
    };

    let fill = |blocks: &mut Vec<Vec<usize>>| {
        blocks.sort_by(|a, b| a[2].min(a[5]).cmp(&b[2].min(b[5])));

        for i in 1..blocks.len() {
            let max_z = (0..i)
                .rev()
                .filter(|&j| over_laps(&blocks[i], &blocks[j]))
                .map(|j| blocks[j][5] + 1)
                .max()
                .unwrap_or(1);
            let diff_z = blocks[i][5] - blocks[i][2];
            blocks[i][2] = max_z;
            blocks[i][5] = max_z + diff_z;
        }
    };

    let compute_supports = |blocks: &mut Vec<Vec<usize>>| {
        blocks.sort_by(|a, b| a[2].min(a[5]).cmp(&b[2].min(b[5])));

        let mut k_supports_v = vec![HashSet::new(); blocks.len()];
        let mut v_supports_k = vec![HashSet::new(); blocks.len()];
        for (j, upper) in blocks.iter().enumerate() {
            for (i, lower) in blocks.iter().take(j).enumerate() {
                if over_laps(lower, upper) && upper[2] == lower[5] + 1 {
                    k_supports_v[i].insert(j);
                    v_supports_k[j].insert(i);
                }
            }
        }

        (0..blocks.len())
            .filter(|&i| k_supports_v[i].iter().all(|&j| v_supports_k[j].len() >= 2))
            .count()
    };

    let mut blocks = input
        .lines()
        .map(|line| {
            line.replace("~", ",")
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fill(&mut blocks);

    compute_supports(&mut blocks)
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let over_laps = |a: &Vec<usize>, b: &Vec<usize>| {
        a[0].max(b[0]) <= a[3].min(b[3]) && a[1].max(b[1]) <= a[4].min(b[4])
    };

    let fill = |blocks: &mut Vec<Vec<usize>>| {
        blocks.sort_by(|a, b| a[2].min(a[5]).cmp(&b[2].min(b[5])));

        for i in 1..blocks.len() {
            let max_z = (0..i)
                .rev()
                .filter(|&j| over_laps(&blocks[i], &blocks[j]))
                .map(|j| blocks[j][5] + 1)
                .max()
                .unwrap_or(1);
            let diff_z = blocks[i][5] - blocks[i][2];
            blocks[i][2] = max_z;
            blocks[i][5] = max_z + diff_z;
        }
    };

    let compute_supports = |blocks: &mut Vec<Vec<usize>>| {
        blocks.sort_by(|a, b| a[2].min(a[5]).cmp(&b[2].min(b[5])));

        let mut k_supports_v = vec![HashSet::new(); blocks.len()];
        let mut v_supports_k = vec![HashSet::new(); blocks.len()];
        for (j, upper) in blocks.iter().enumerate() {
            for (i, lower) in blocks.iter().take(j).enumerate() {
                if over_laps(lower, upper) && upper[2] == lower[5] + 1 {
                    k_supports_v[i].insert(j);
                    v_supports_k[j].insert(i);
                }
            }
        }

        let mut total = 0;
        for i in 0..blocks.len() {
            let mut q = k_supports_v[i]
                .iter()
                .filter(|&&j| v_supports_k[j].len() == 1)
                .map(|&j| j)
                .collect::<VecDeque<_>>();
            let mut falling = q.iter().map(|&j| j).collect::<HashSet<_>>();
            falling.insert(i);

            while let Some(j) = q.pop_front() {
                for &k in &k_supports_v[j] {
                    if !falling.contains(&k) && v_supports_k[k].is_subset(&falling) {
                        q.push_back(k);
                        falling.insert(k);
                    }
                }
            }

            total += falling.len() - 1;
        }

        total
    };

    let mut blocks = input
        .lines()
        .map(|line| {
            line.replace("~", ",")
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fill(&mut blocks);

    compute_supports(&mut blocks)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {r"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 7);
    }
}
