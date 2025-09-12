use aoc_runner_derive::aoc;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    ore_robot_ore: u32,
    clay_robot_ore: u32,
    obsidian_robot_ore: u32,
    obsidian_robot_clay: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32,
}

impl FromStr for Blueprint {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 15 obsidian.
        let parts: Vec<_> = s.split(':').collect();
        let id = parts[0].split_whitespace().last().unwrap().parse().unwrap();
        let rest = parts[1];
        let nums: Vec<u32> = rest
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|w| w.parse().ok())
            .collect();
        Ok(Blueprint {
            id,
            ore_robot_ore: nums[0],
            clay_robot_ore: nums[1],
            obsidian_robot_ore: nums[2],
            obsidian_robot_clay: nums[3],
            geode_robot_ore: nums[4],
            geode_robot_obsidian: nums[5],
        })
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

use std::collections::HashMap;

fn max_geodes(blueprint: &Blueprint, minutes: u32) -> u32 {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    struct State {
        time: u32,
        ore: u32,
        clay: u32,
        obsidian: u32,
        geode: u32,
        ore_robots: u32,
        clay_robots: u32,
        obsidian_robots: u32,
        geode_robots: u32,
    }

    let mut memo = HashMap::new();
    let max_ore_cost = *[
        blueprint.ore_robot_ore,
        blueprint.clay_robot_ore,
        blueprint.obsidian_robot_ore,
        blueprint.geode_robot_ore,
    ]
    .iter()
    .max()
    .unwrap();

    fn dfs(
        blueprint: &Blueprint,
        state: State,
        minutes: u32,
        max_ore_cost: u32,
        memo: &mut HashMap<State, u32>,
        best: &mut u32,
    ) -> u32 {
        if state.time == minutes {
            if state.geode > *best {
                *best = state.geode;
            }
            return state.geode;
        }

        // Prune if even building a geode robot every minute can't beat best
        let rem = minutes - state.time;
        let optimistic = state.geode + state.geode_robots * rem + rem * (rem - 1) / 2;
        if optimistic <= *best {
            return 0;
        }

        let mut key = state;
        // Cap resources to avoid state explosion
        key.ore = key.ore.min(max_ore_cost * rem);
        key.clay = key.clay.min(blueprint.obsidian_robot_clay * rem);
        key.obsidian = key.obsidian.min(blueprint.geode_robot_obsidian * rem);
        if let Some(&v) = memo.get(&key) {
            return v;
        }

        let mut max_geode = 0;

        // Try to build geode robot if possible (always best)
        if state.ore >= blueprint.geode_robot_ore
            && state.obsidian >= blueprint.geode_robot_obsidian
        {
            let mut next = state;
            next.time += 1;
            next.ore += state.ore_robots;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geode += state.geode_robots;
            next.ore -= blueprint.geode_robot_ore;
            next.obsidian -= blueprint.geode_robot_obsidian;
            next.geode_robots += 1;
            max_geode = max_geode.max(dfs(blueprint, next, minutes, max_ore_cost, memo, best));
            memo.insert(key, max_geode);
            return max_geode;
        }

        // Try to build obsidian robot
        if state.ore >= blueprint.obsidian_robot_ore
            && state.clay >= blueprint.obsidian_robot_clay
            && state.obsidian_robots < blueprint.geode_robot_obsidian
        {
            let mut next = state;
            next.time += 1;
            next.ore += state.ore_robots;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geode += state.geode_robots;
            next.ore -= blueprint.obsidian_robot_ore;
            next.clay -= blueprint.obsidian_robot_clay;
            next.obsidian_robots += 1;
            max_geode = max_geode.max(dfs(blueprint, next, minutes, max_ore_cost, memo, best));
        }

        // Try to build clay robot
        if state.ore >= blueprint.clay_robot_ore
            && state.clay_robots < blueprint.obsidian_robot_clay
        {
            let mut next = state;
            next.time += 1;
            next.ore += state.ore_robots;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geode += state.geode_robots;
            next.ore -= blueprint.clay_robot_ore;
            next.clay_robots += 1;
            max_geode = max_geode.max(dfs(blueprint, next, minutes, max_ore_cost, memo, best));
        }

        // Try to build ore robot
        if state.ore >= blueprint.ore_robot_ore && state.ore_robots < max_ore_cost {
            let mut next = state;
            next.time += 1;
            next.ore += state.ore_robots;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geode += state.geode_robots;
            next.ore -= blueprint.ore_robot_ore;
            next.ore_robots += 1;
            max_geode = max_geode.max(dfs(blueprint, next, minutes, max_ore_cost, memo, best));
        }

        // Or do nothing (collect only)
        {
            let mut next = state;
            next.time += 1;
            next.ore += state.ore_robots;
            next.clay += state.clay_robots;
            next.obsidian += state.obsidian_robots;
            next.geode += state.geode_robots;
            max_geode = max_geode.max(dfs(blueprint, next, minutes, max_ore_cost, memo, best));
        }

        memo.insert(key, max_geode);
        max_geode
    }

    let mut best = 0;
    let start = State {
        time: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };
    dfs(
        blueprint,
        start,
        minutes,
        max_ore_cost,
        &mut memo,
        &mut best,
    )
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let blueprints = parse(input);
    blueprints
        .iter()
        .map(|bp| max_geodes(bp, 24) as usize * bp.id)
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let blueprints = parse(input);
    blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(bp, 32) as usize)
        .product()
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 3472);
    }
}
