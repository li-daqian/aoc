use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
    Broadcaster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let (module_connections, mut module_states) = input.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut module_connections, mut module_states), line| {
            let (src, connections) = line.split_once(" -> ").unwrap();
            let (name, module_type) = match src.as_bytes()[0] {
                b'%' => (&src[1..], Module::FlipFlop(false)),
                b'&' => (&src[1..], Module::Conjunction(HashMap::new())),
                b'b' => (src, Module::Broadcaster),
                _ => unreachable!(),
            };
            module_connections.insert(name, connections.split(", ").collect::<Vec<_>>());
            module_states.insert(name, module_type);
            (module_connections, module_states)
        },
    );

    for (&name, nexts) in &module_connections {
        for &next in nexts {
            if let Some(module) = module_states.get_mut(next) {
                if let Module::Conjunction(ref mut input_map) = module {
                    input_map.insert(name, Pulse::Low);
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    let (mut high_count, mut low_count) = (0, 0);
    for _ in 0..1000 {
        queue.push_back(("button", Pulse::Low, "broadcaster"));
        while let Some((from, pulse, to)) = queue.pop_front() {
            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }

            let next_pulse = match module_states.get_mut(to) {
                Some(Module::FlipFlop(on)) => {
                    if pulse == Pulse::High {
                        continue;
                    } else {
                        *on = !*on;
                        if *on {
                            Pulse::High
                        } else {
                            Pulse::Low
                        }
                    }
                }
                Some(Module::Conjunction(input_map)) => {
                    *input_map.get_mut(from).unwrap() = pulse;
                    if input_map.values().any(|pulse| *pulse == Pulse::Low) {
                        Pulse::High
                    } else {
                        Pulse::Low
                    }
                }
                Some(Module::Broadcaster) => Pulse::Low,
                None => continue,
            };

            queue.extend(
                module_connections[to]
                    .iter()
                    .map(|&next| (to, next_pulse, next)),
            );
        }
    }

    high_count * low_count
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let (module_connections, mut module_states) = input.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut module_connections, mut module_states), line| {
            let (src, connections) = line.split_once(" -> ").unwrap();
            let (name, module_type) = match src.as_bytes()[0] {
                b'%' => (&src[1..], Module::FlipFlop(false)),
                b'&' => (&src[1..], Module::Conjunction(HashMap::new())),
                b'b' => (src, Module::Broadcaster),
                _ => unreachable!(),
            };
            module_connections.insert(name, connections.split(", ").collect::<Vec<_>>());
            module_states.insert(name, module_type);
            (module_connections, module_states)
        },
    );

    let mut rx_previous = "";
    for (&name, nexts) in &module_connections {
        for &next in nexts {
            match module_states.get_mut(next) {
                Some(Module::Conjunction(ref mut input_map)) => {
                    input_map.insert(name, Pulse::Low);
                }
                Some(_) => {}
                None => {
                    rx_previous = name;
                }
            }
        }
    }

    let mut rx_previous_counts = {
        let Module::Conjunction(input_map) = &module_states[rx_previous] else {
            panic!("Expected a conjunction module");
        };
        input_map
            .keys()
            .map(|&name| (name, None))
            .collect::<HashMap<_, _>>()
    };

    let mut queue = VecDeque::new();
    for count in 1.. {
        queue.push_back(("button", Pulse::Low, "broadcaster"));
        while let Some((from, pulse, to)) = queue.pop_front() {
            if pulse == Pulse::High && to == rx_previous {
                let rx_prevous_count = rx_previous_counts.get_mut(from).unwrap();
                if rx_prevous_count.is_none() {
                    *rx_prevous_count = Some(count);
                    if rx_previous_counts.values().all(|c| c.is_some()) {
                        return rx_previous_counts.values().fold(1, |mut answer, c| {
                            answer *= c.unwrap();
                            answer
                        });
                    }
                }
            }

            let next_pulse = match module_states.get_mut(to) {
                Some(Module::FlipFlop(on)) => {
                    if pulse == Pulse::High {
                        continue;
                    } else {
                        *on = !*on;
                        if *on {
                            Pulse::High
                        } else {
                            Pulse::Low
                        }
                    }
                }
                Some(Module::Conjunction(input_map)) => {
                    *input_map.get_mut(from).unwrap() = pulse;
                    if input_map.values().any(|pulse| *pulse == Pulse::Low) {
                        Pulse::High
                    } else {
                        Pulse::Low
                    }
                }
                Some(Module::Broadcaster) => Pulse::Low,
                None => continue,
            };

            queue.extend(
                module_connections[to]
                    .iter()
                    .map(|&next| (to, next_pulse, next)),
            );
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE_1: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    const SAMPLE_2: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_1), 32000000);
        assert_eq!(part1(SAMPLE_2), 11687500);
    }
}
