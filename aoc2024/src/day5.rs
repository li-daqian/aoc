use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut answer = 0usize;
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        if let Some((l, r)) = line.split_once("|") {
            if let (Ok(l), Ok(r)) = (l.parse::<usize>(), r.parse::<usize>()) {
                edges.entry(l).or_insert_with(Vec::new).push(r);
            }
        } else {
            let page_numbers: Vec<usize> = line
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            let mut earlier: HashSet<usize> = HashSet::new();
            let mut ok = true;
            for page_number in &page_numbers {
                edges
                    .get(&page_number)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .for_each(|after| {
                        if earlier.contains(after) {
                            ok = false;
                        }
                    });
                earlier.insert(*page_number);
            }

            if ok {
                answer += page_numbers.get(page_numbers.len() / 2).unwrap_or(&0);
            }
        }
    });

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 143);
    }
}
