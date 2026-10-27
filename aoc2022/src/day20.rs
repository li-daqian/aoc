use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Node {
    value: i64,
    prev: usize,
    next: usize,
}

fn parse(input: &str, decryption_key: i64) -> (Vec<Node>, Vec<usize>) {
    let values: Vec<i64> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<i64>().unwrap() * decryption_key)
        .collect();

    let len = values.len();
    assert!(
        len > 1,
        "the encrypted file must contain at least two numbers"
    );

    let nodes = values
        .into_iter()
        .enumerate()
        .map(|(id, value)| Node {
            value,
            prev: (id + len - 1) % len,
            next: (id + 1) % len,
        })
        .collect();
    let order = (0..len).collect();
    (nodes, order)
}

fn insert_after(nodes: &mut [Node], node: usize, anchor: usize) {
    let next = nodes[anchor].next;
    nodes[node].prev = anchor;
    nodes[node].next = next;
    nodes[anchor].next = node;
    nodes[next].prev = node;
}

fn detach(nodes: &mut [Node], node: usize) {
    let prev = nodes[node].prev;
    let next = nodes[node].next;
    nodes[prev].next = next;
    nodes[next].prev = prev;
}

fn mix(nodes: &mut [Node], order: &[usize], rounds: usize) {
    let modulo = (nodes.len() - 1) as i64;

    for _ in 0..rounds {
        for &node in order {
            let value = nodes[node].value;
            if value == 0 {
                continue;
            }

            let mut anchor = nodes[node].prev;
            detach(nodes, node);

            let steps = if value > 0 {
                value.rem_euclid(modulo)
            } else {
                (-value).rem_euclid(modulo)
            };
            let use_shorter_path = steps > modulo / 2;
            let distance = if use_shorter_path {
                modulo - steps
            } else {
                steps
            };

            let move_forward = (value > 0) != use_shorter_path;
            if move_forward {
                for _ in 0..distance {
                    anchor = nodes[anchor].next;
                }
            } else {
                for _ in 0..distance {
                    anchor = nodes[anchor].prev;
                }
            }

            insert_after(nodes, node, anchor);
        }
    }
}

fn grove_coordinates(nodes: &[Node]) -> i64 {
    let zero = nodes.iter().position(|node| node.value == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|distance| {
            let mut node = zero;
            for _ in 0..(distance % nodes.len()) {
                node = nodes[node].next;
            }
            nodes[node].value
        })
        .sum()
}

fn solve(input: &str, decryption_key: i64, rounds: usize) -> i64 {
    let (mut nodes, order) = parse(input, decryption_key);
    mix(&mut nodes, &order, rounds);
    grove_coordinates(&nodes)
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input, 1, 1)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input, 811_589_153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1_623_178_306);
    }
}
