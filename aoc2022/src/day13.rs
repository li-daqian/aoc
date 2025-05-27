use aoc_runner_derive::aoc;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(s: &str) -> Packet {
        fn parse_inner(chars: &mut std::iter::Peekable<std::str::Chars>) -> Packet {
            let mut list = Vec::new();
            while let Some(&c) = chars.peek() {
                match c {
                    '[' => {
                        chars.next();
                        list.push(parse_inner(chars));
                    }
                    ']' => {
                        chars.next();
                        break;
                    }
                    ',' => {
                        chars.next();
                    }
                    d if d.is_ascii_digit() || d == '-' => {
                        let mut num = String::new();
                        while let Some(&d) = chars.peek() {
                            if d.is_ascii_digit() || d == '-' {
                                num.push(d);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        list.push(Packet::Int(num.parse().unwrap()));
                    }
                    _ => {
                        chars.next();
                    }
                }
            }
            Packet::List(list)
        }
        let mut chars = s.chars().peekable();
        parse_inner(&mut chars)
    }

    fn cmp_packets(left: &Packet, right: &Packet) -> Ordering {
        match (left, right) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    let ord = Packet::cmp_packets(x, y);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                a.len().cmp(&b.len())
            }
            (Packet::Int(_), Packet::List(_)) => {
                Packet::cmp_packets(&Packet::List(vec![left.clone()]), right)
            }
            (Packet::List(_), Packet::Int(_)) => {
                Packet::cmp_packets(left, &Packet::List(vec![right.clone()]))
            }
        }
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair)| {
            let mut lines = pair.lines();
            let left = Packet::parse(lines.next().unwrap());
            let right = Packet::parse(lines.next().unwrap());
            if Packet::cmp_packets(&left, &right) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Packet::parse)
        .collect();

    let divider1 = Packet::parse("[[2]]");
    let divider2 = Packet::parse("[[6]]");
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(|a, b| Packet::cmp_packets(a, b));

    let idx1 = packets.iter().position(|p| *p == divider1).unwrap() + 1;
    let idx2 = packets.iter().position(|p| *p == divider2).unwrap() + 1;

    idx1 * idx2
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 140);
    }
}
