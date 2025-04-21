use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let groups: Vec<usize> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            let springs: Vec<u8> = springs.bytes().collect();
            let mut memo = HashMap::new();
            count_arrangement(&springs, &groups, &mut memo)
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let mut groups: Vec<usize> = groups.split(',').map(|s| s.parse().unwrap()).collect();
            let mut springs: Vec<u8> = springs.bytes().collect();
            let (groups_copy, springs_copy) = (groups.clone(), springs.clone());
            (0..4).for_each(|_| {
                springs.push(b'?');
                springs.extend(springs_copy.clone());
                groups.extend(groups_copy.clone());
            });
            let mut memo = HashMap::new();
            count_arrangement(&springs, &groups, &mut memo)
        })
        .sum()
}

fn count_arrangement(
    springs: &[u8],
    groups: &[usize],
    memo: &mut HashMap<(Vec<u8>, Vec<usize>), usize>,
) -> usize {
    if groups.is_empty() {
        if !springs.contains(&b'#') {
            return 1;
        }
        return 0;
    }

    let need_length = groups.iter().sum::<usize>() + groups.len() - 1;
    if need_length > springs.len() {
        return 0;
    }

    if springs[0] == b'.' {
        return count_arrangement(&springs[1..], groups, memo);
    }

    let key = (springs.to_vec(), groups.to_vec());
    if let Some(&result) = memo.get(&(key)) {
        return result;
    }

    let mut total = 0;
    let cur_group = groups[0];
    let all_springs_valid = springs[0..cur_group].iter().all(|&c| c != b'.');
    let last_char_valid = springs.len() == cur_group || springs[cur_group] != b'#';
    if all_springs_valid && last_char_valid {
        let max_idx = springs.len().min(cur_group + 1);
        total += count_arrangement(&springs[max_idx..], &groups[1..], memo);
    }

    if springs[0] == b'?' {
        total += count_arrangement(&springs[1..], groups, memo);
    }

    memo.insert(key, total);

    total
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 525152);
    }
}
