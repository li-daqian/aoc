use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let mut lan = HashMap::<_, HashSet<_>>::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        lan.entry(a).or_default().insert(b);
        lan.entry(b).or_default().insert(a);
    });

    let mut answer = HashSet::new();
    for &node1 in lan.keys() {
        if !node1.starts_with("t") {
            continue;
        }
        for &node2 in &lan[node1] {
            for &node3 in lan[node1].intersection(&lan[node2]) {
                let mut nodes = [node1, node2, node3];
                nodes.sort();
                answer.insert(nodes);
            }
        }
    }

    answer.len()
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let mut edges = HashMap::<_, Vec<_>>::new();
    let mut pairs = HashSet::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
        pairs.insert((a, b));
        pairs.insert((b, a));
    });

    edges
        .keys()
        .map(|&a| {
            let mut clique = vec![a];
            resovle(&edges, &pairs, a, 0, &mut clique);
            clique
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .map(|mut clique| {
            clique.sort();
            clique.join(",")
        })
        .unwrap()
}

fn resovle<'a>(
    edges: &HashMap<&'a str, Vec<&'a str>>,
    pairs: &HashSet<(&'a str, &'a str)>,
    a: &'a str,
    i: usize,
    clique: &mut Vec<&'a str>,
) {
    if i == edges[a].len() {
        return;
    }

    resovle(edges, pairs, a, i + 1, clique);

    let b = edges[a][i];
    for n in clique.clone() {
        if !pairs.contains(&(b, n)) {
            return;
        }
    }
    clique.push(b);
    resovle(edges, pairs, b, i + 1, clique);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), "co,de,ka,ta");
    }
}
