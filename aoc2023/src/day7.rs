use std::{cmp::Ordering, collections::HashMap};

use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut counts = HashMap::new();
    let mut cards = input
        .lines()
        .map(|line| {
            counts.clear();
            let (card, bit) = line.split_once(" ").unwrap();
            let card = card.as_bytes();
            card.iter().for_each(|&c| {
                *counts.entry(c).or_insert(0usize) += 1;
            });

            let level = match counts.len() {
                1 => 0 as usize, // 5
                2 => {
                    if counts.values().any(|&v| v == 4) {
                        1 // 4,1
                    } else {
                        2 // 3,2
                    }
                }
                3 => {
                    if counts.values().any(|&v| v == 3) {
                        3 // 3,1,1
                    } else {
                        4 // 2,2,1
                    }
                }
                4 => 5, // 2,1,1,1
                5 => 6, // 1,1,1,1,1
                _ => unreachable!(),
            };
            (card, bit.parse::<usize>().unwrap(), level)
        })
        .collect::<Vec<_>>();

    const CARD_ORDER: &[u8] = b"AKQJT98765432";
    cards.sort_by(|a, b| {
        let level_a = a.2;
        let level_b = b.2;
        if level_a != level_b {
            return level_b.cmp(&level_a);
        }

        for i in 0..a.0.len() {
            let card_a = a.0[i];
            let card_b = b.0[i];
            if card_a != card_b {
                let index_a = CARD_ORDER.iter().position(|&c| c == card_a).unwrap();
                let index_b = CARD_ORDER.iter().position(|&c| c == card_b).unwrap();
                return index_b.cmp(&index_a);
            }
        }
        Ordering::Equal
    });

    cards
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (_card, bit, _level))| {
            acc + bit * (index + 1)
        })
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let mut counts = HashMap::new();
    let mut cards = input
        .lines()
        .map(|line| {
            counts.clear();
            let (card, bit) = line.split_once(" ").unwrap();
            let card = card.as_bytes();
            card.iter().for_each(|&c| {
                *counts.entry(c).or_insert(0usize) += 1;
            });

            let j_count = counts.remove(&b'J').unwrap_or(0);
            let max_count_key = counts
                .iter()
                .max_by_key(|&(_, &v)| v)
                .map(|(&k, _)| k)
                .unwrap_or(b'J');
            *counts.entry(max_count_key).or_insert(0) += j_count;
            let max_count = *counts.get(&max_count_key).unwrap();
            let level = match counts.len() {
                1 => 0 as usize, // 5
                2 => {
                    if max_count == 4 {
                        1 // 4,1
                    } else {
                        2 // 3,2
                    }
                }
                3 => {
                    if max_count == 3 {
                        3 // 3,1,1
                    } else {
                        4 // 2,2,1
                    }
                }
                4 => 5, // 2,1,1,1
                5 => 6, // 1,1,1,1,1
                _ => unreachable!(),
            };
            (card, bit.parse::<usize>().unwrap(), level)
        })
        .collect::<Vec<_>>();

    const CARD_ORDER: &[u8] = b"AKQT98765432J";
    cards.sort_by(|a, b| {
        let level_a = a.2;
        let level_b = b.2;
        if level_a != level_b {
            return level_b.cmp(&level_a);
        }

        for i in 0..a.0.len() {
            let card_a = a.0[i];
            let card_b = b.0[i];
            if card_a != card_b {
                let index_a = CARD_ORDER.iter().position(|&c| c == card_a).unwrap();
                let index_b = CARD_ORDER.iter().position(|&c| c == card_b).unwrap();
                return index_b.cmp(&index_a);
            }
        }
        Ordering::Equal
    });

    cards
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (_card, bit, _level))| {
            acc + bit * (index + 1)
        })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 5905);
    }
}
