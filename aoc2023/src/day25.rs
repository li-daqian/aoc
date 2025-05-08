use std::{
    collections::{HashMap, HashSet, VecDeque},
    i64,
};

use aoc_runner_derive::aoc;

fn min_count(graph: &HashMap<&str, HashSet<&str>>, s: &str, t: &str) -> Option<usize> {
    let mut flow = HashMap::new();
    let mut f = 0;
    while f <= 3 {
        let mut pred = HashMap::new();
        let mut queue = VecDeque::from_iter([s]);
        let mut seen_components = 0;
        while let Some(cur) = queue.pop_front() {
            if pred.contains_key(t) {
                break;
            }
            for &next in &graph[cur] {
                if next != s
                    && !pred.contains_key(next)
                    && *flow.get(&(cur, next)).unwrap_or(&0) < 1
                {
                    pred.insert(next, cur);
                    queue.push_back(next);
                }
            }
            seen_components += 1;
        }
        if !pred.contains_key(t) {
            if seen_components == graph.len() {
                return None;
            }
            return Some(seen_components * (graph.len() - seen_components));
        }

        let mut df = i64::MAX;
        let mut cur = t;
        while let Some(&prev) = pred.get(cur) {
            df = df.min(1 - *flow.get(&(prev, cur)).unwrap_or(&0));
            cur = prev;
        }
        let mut cur = t;
        while let Some(&prev) = pred.get(cur) {
            *flow.entry((prev, cur)).or_default() += df;
            *flow.entry((cur, prev)).or_default() -= df;
            cur = prev;
        }
        f += df;
    }

    None
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    for l in input.lines() {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split(' ') {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    }

    let start = graph.keys().next().unwrap();
    graph
        .keys()
        .skip(1)
        .find_map(|k| min_count(&graph, &start, &k))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {r"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 54);
    }
}
