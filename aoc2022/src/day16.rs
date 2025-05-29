use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: u32,
    connections: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValveError;

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.trim().split_once(";").ok_or(ParseValveError)?;

        let (raw_name, raw_rate) = a.trim().split_once(" has flow ").ok_or(ParseValveError)?;

        let name = raw_name
            .trim()
            .strip_prefix("Valve ")
            .and_then(|s| Some(String::from(s)))
            .ok_or(ParseValveError)?;

        let rate = raw_rate
            .trim()
            .strip_prefix("rate=")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(ParseValveError)?;

        let connections = Some(b.trim())
            .and_then(|s| s.strip_prefix("tunnel ").or(s.strip_prefix("tunnels ")))
            .and_then(|s| s.strip_prefix("lead ").or(s.strip_prefix("leads ")))
            .and_then(|x| x.strip_prefix("to "))
            .and_then(|s| s.strip_prefix("valve ").or(s.strip_prefix("valves ")))
            .and_then(|s| {
                Some(
                    s.split(",")
                        .map(|c| c.trim().to_string())
                        .collect::<Vec<String>>(),
                )
            })
            .ok_or(ParseValveError)?;

        Ok(Valve {
            name,
            rate,
            connections,
        })
    }
}

#[derive(Debug)]
struct SimpleValve {
    name: String,
    rate: u32,
    links: Vec<usize>,
}

fn parse(input: &str) -> Vec<SimpleValve> {
    let valves: Vec<Valve> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<Valve>().unwrap())
        .collect();

    let idx_map: HashMap<String, usize> =
        valves
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut m, (i, x)| {
                m.insert(x.name.clone(), i);
                m
            });

    valves
        .into_iter()
        .map(|v| SimpleValve {
            name: v.name,
            rate: v.rate,
            links: v
                .connections
                .iter()
                .map(|x| *idx_map.get(x).unwrap())
                .collect(),
        })
        .collect()
}

fn simulate(
    valves: &Vec<SimpleValve>,
    dist: &Vec<Vec<u32>>,
    init_mask: u64,
    start_idx: usize,
    minutes: u32,
) -> (u32, HashMap<u64, u32>) {
    let non_zero_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, x)| x.rate > 0)
        .map(|(i, _)| i)
        .collect();

    let flow = 0;
    let mut mask_flow: HashMap<u64, u32> = HashMap::new();

    let flow = traveling_salesman(
        valves,
        &mut mask_flow,
        &non_zero_valves,
        &dist,
        init_mask,
        minutes,
        flow,
        start_idx,
        0,
    );

    (flow, mask_flow)
}

fn traveling_salesman(
    valves: &Vec<SimpleValve>,
    memo: &mut HashMap<u64, u32>,
    non_zero_valves: &Vec<usize>,
    dist: &Vec<Vec<u32>>,
    mask: u64,
    minutes: u32,
    flow: u32,
    i: usize,
    depth: u32,
) -> u32 {
    let mut max_flow = flow;

    memo.insert(mask, *memo.get(&mask).unwrap_or(&0).max(&flow));

    for &j in non_zero_valves.iter() {
        let cur_minutes = minutes
            .checked_sub(dist[i][j])
            .and_then(|x| x.checked_sub(1))
            .unwrap_or(0);

        if (mask & (1 << j)) == 0 || cur_minutes <= 0 {
            continue;
        }

        let cur_mask = mask & !(1 << j);

        let cur_flow = flow + (cur_minutes * valves[j].rate);

        max_flow = max_flow.max(traveling_salesman(
            valves,
            memo,
            non_zero_valves,
            dist,
            cur_mask,
            cur_minutes,
            cur_flow,
            j,
            depth + 1,
        ));
    }

    return max_flow;
}

fn floyd_warshall(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let l = graph.len();
    let mut dist = graph.clone();

    for k in 0..l {
        for i in 0..l {
            for j in 0..l {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    dist
}

fn init_graph<T, F>(list: &Vec<T>, get_links: F) -> Vec<Vec<u32>>
where
    F: Fn(&T) -> &Vec<usize>,
{
    let l = list.len();
    let mut graph = vec![vec![u32::MAX / 4; l]; l];

    list.iter().enumerate().for_each(|(i, x)| {
        get_links(x).iter().for_each(|&j| graph[i][j as usize] = 1);
    });

    graph
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    let valves = parse(input);

    let graph = init_graph(&valves, |x| &x.links);
    let dist = floyd_warshall(graph);

    let start_idx = valves.iter().position(|x| x.name == "AA").unwrap();
    let len = dist.len();

    // I spent 20 hours trying to find a bug that later appeared to be happening because of this line
    // APPARENTLY THE STARTING VALVE IS ALWAYS "AA" AND NOT THE FIRST VALVE IN THE INPUT
    let init_mask: u64 = (1 << len) - 1;

    let (flow, _) = simulate(&valves, &dist, init_mask, start_idx, 30);

    flow
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    let valves = parse(input);

    let graph = init_graph(&valves, |x| &x.links);
    let dist = floyd_warshall(graph);

    let start_idx = valves.iter().position(|x| x.name == "AA").unwrap();
    let init_mask: u64 = (1 << dist.len()) - 1;

    let (_, elf_memo) = simulate(&valves, &dist, init_mask, start_idx, 26);
    let (_, elephant_memo) = simulate(&valves, &dist, init_mask, start_idx, 26);

    elf_memo.iter().fold(0, |max, (&elf_mask, &elf_flow)| {
        elephant_memo
            .iter()
            .fold(max, |max, (&mask, &elephant_flow)| {
                // Check that there's no overlap between the 2 paths
                if (!mask) & (!elf_mask) & init_mask == 0 {
                    return max.max(elephant_flow + elf_flow);
                }

                max
            })
    })
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
