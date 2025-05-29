use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Valve<'a> {
    flow: u32,
    leads: Vec<&'a str>,
}

fn parse(input: &str) -> HashMap<&str, Valve<'_>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split("; ").collect();
        let (name, flow) = {
            let mut s = parts[0].split_whitespace();
            let name = s.nth(1).unwrap();
            let flow = s
                .nth(2)
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            (name, flow)
        };
        let leads = parts[1]
            .split("valve")
            .nth(1)
            .unwrap()
            .trim_start_matches('s')
            .trim()
            .split(", ")
            .map(|s| s.trim())
            .collect();
        map.insert(name, Valve { flow, leads });
    }
    map
}

fn floyd_warshall<'a>(graph: &HashMap<&'a str, Valve<'a>>) -> HashMap<(&'a str, &'a str), u32> {
    let valves: Vec<_> = graph.keys().copied().collect();
    let mut dist = HashMap::new();
    for &v in &valves {
        for &u in &valves {
            if v == u {
                dist.insert((v, u), 0);
            } else if graph[v].leads.contains(&u) {
                dist.insert((v, u), 1);
            } else {
                dist.insert((v, u), u32::MAX / 2);
            }
        }
    }
    for &k in &valves {
        for &i in &valves {
            for &j in &valves {
                let ij = dist[&(i, j)];
                let ik = dist[&(i, k)];
                let kj = dist[&(k, j)];
                if ik + kj < ij {
                    dist.insert((i, j), ik + kj);
                }
            }
        }
    }
    dist
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    let graph = parse(input);
    let dist = floyd_warshall(&graph);

    // Only consider valves with flow > 0
    let useful: Vec<&str> = graph
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(&k, _)| k)
        .collect();

    let mut memo = HashMap::new();

    fn dfs<'a>(
        pos: &'a str,
        time: u32,
        opened: u64,
        useful: &[&'a str],
        graph: &HashMap<&'a str, Valve<'a>>,
        dist: &HashMap<(&'a str, &'a str), u32>,
        memo: &mut HashMap<(&'a str, u32, u64), u32>,
    ) -> u32 {
        if let Some(&v) = memo.get(&(pos, time, opened)) {
            return v;
        }
        let mut max = 0;
        for (i, &next) in useful.iter().enumerate() {
            if (opened & (1 << i)) != 0 {
                continue;
            }
            let d = dist[&(pos, next)];
            if d + 1 > time {
                continue;
            }
            let rem = time - d - 1;
            let gain = graph[next].flow * rem;
            let total = gain + dfs(next, rem, opened | (1 << i), useful, graph, dist, memo);
            if total > max {
                max = total;
            }
        }
        memo.insert((pos, time, opened), max);
        max
    }

    dfs("AA", 30, 0, &useful, &graph, &dist, &mut memo)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    let graph = parse(input);
    let dist = floyd_warshall(&graph);

    let useful: Vec<&str> = graph
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(&k, _)| k)
        .collect();

    let mut memo = HashMap::new();

    fn dfs<'a>(
        pos: &'a str,
        time: u32,
        opened: u64,
        elephant: bool,
        useful: &[&'a str],
        graph: &HashMap<&'a str, Valve<'a>>,
        dist: &HashMap<(&'a str, &'a str), u32>,
        memo: &mut HashMap<(&'a str, u32, u64, bool), u32>,
    ) -> u32 {
        if let Some(&v) = memo.get(&(pos, time, opened, elephant)) {
            return v;
        }
        let mut max = 0;
        for (i, &next) in useful.iter().enumerate() {
            if (opened & (1 << i)) != 0 {
                continue;
            }
            let d = dist[&(pos, next)];
            if d + 1 > time {
                continue;
            }
            let rem = time - d - 1;
            let gain = graph[next].flow * rem;
            let total = gain
                + dfs(
                    next,
                    rem,
                    opened | (1 << i),
                    elephant,
                    useful,
                    graph,
                    dist,
                    memo,
                );
            if total > max {
                max = total;
            }
        }
        // Handoff to elephant if available
        if elephant {
            let total = dfs("AA", 26, opened, false, useful, graph, dist, memo);
            if total > max {
                max = total;
            }
        }
        memo.insert((pos, time, opened, elephant), max);
        max
    }

    dfs("AA", 26, 0, true, &useful, &graph, &dist, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1707);
    }
}
