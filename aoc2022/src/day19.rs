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

#[derive(Clone, Copy)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn wait_for_resource(have: u32, robots: u32, required: u32) -> Option<u32> {
    if have >= required {
        Some(0)
    } else if robots == 0 {
        None
    } else {
        Some((required - have).div_ceil(robots))
    }
}

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

    fn next_state(
        state: State,
        remaining: u32,
        ore_cost: u32,
        clay_cost: u32,
        obsidian_cost: u32,
        robot: Robot,
    ) -> Option<State> {
        let wait_ore = wait_for_resource(state.ore, state.ore_robots, ore_cost)?;
        let wait_clay = wait_for_resource(state.clay, state.clay_robots, clay_cost)?;
        let wait_obsidian =
            wait_for_resource(state.obsidian, state.obsidian_robots, obsidian_cost)?;
        let wait = wait_ore.max(wait_clay).max(wait_obsidian);
        let elapsed = wait + 1;
        if elapsed > remaining {
            return None;
        }

        let mut next = state;
        next.time += elapsed;
        next.ore = state.ore + state.ore_robots * elapsed - ore_cost;
        next.clay = state.clay + state.clay_robots * elapsed - clay_cost;
        next.obsidian = state.obsidian + state.obsidian_robots * elapsed - obsidian_cost;
        next.geode += state.geode_robots * elapsed;

        match robot {
            Robot::Ore => next.ore_robots += 1,
            Robot::Clay => next.clay_robots += 1,
            Robot::Obsidian => next.obsidian_robots += 1,
            Robot::Geode => next.geode_robots += 1,
        }
        Some(next)
    }

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

        // Jump directly to the next robot-building event instead of exploring
        // every minute spent waiting for resources.
        let actions = [
            (
                Robot::Geode,
                blueprint.geode_robot_ore,
                0,
                blueprint.geode_robot_obsidian,
                true,
            ),
            (
                Robot::Obsidian,
                blueprint.obsidian_robot_ore,
                blueprint.obsidian_robot_clay,
                0,
                state.obsidian_robots < blueprint.geode_robot_obsidian,
            ),
            (
                Robot::Clay,
                blueprint.clay_robot_ore,
                0,
                0,
                state.clay_robots < blueprint.obsidian_robot_clay,
            ),
            (
                Robot::Ore,
                blueprint.ore_robot_ore,
                0,
                0,
                state.ore_robots < max_ore_cost,
            ),
        ];

        for (robot, ore_cost, clay_cost, obsidian_cost, allowed) in actions {
            if !allowed {
                continue;
            }
            if let Some(next) = next_state(state, rem, ore_cost, clay_cost, obsidian_cost, robot) {
                max_geode = max_geode.max(dfs(blueprint, next, minutes, max_ore_cost, memo, best));
            }
        }

        // If no more robot can be built, collect geodes for the remaining time.
        max_geode = max_geode.max(state.geode + state.geode_robots * rem);

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
