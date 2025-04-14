use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let (values, connections) = input.split_once("\n\n").unwrap();
    let mut values = values
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(": ").unwrap();
            (k, v == "1")
        })
        .collect();

    let connections = connections
        .lines()
        .map(|line| {
            let (input, out) = line.split_once(" -> ").unwrap();
            let input = input.split(" ").collect::<Vec<_>>();
            (out, input)
        })
        .collect();

    for (&out, _) in &connections {
        cal(out, &mut values, &connections);
    }

    let answer = {
        let mut keys = values
            .keys()
            .filter(|&k| k.starts_with("z"))
            .collect::<Vec<_>>();
        keys.sort();
        keys.into_iter().enumerate().fold(
            0usize,
            |acc, (i, k)| {
                if values[k] {
                    acc | (1 << i)
                } else {
                    acc
                }
            },
        )
    };

    answer
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
    let (_, connections) = input.split_once("\n\n").unwrap();

    let connections: Vec<Vec<&str>> = connections
        .lines()
        .map(|line| {
            let (input, out) = line.split_once(" -> ").unwrap();
            let mut input = input.split(" ").collect::<Vec<_>>();
            input.push(out);
            input
        })
        .collect();

    let related_gates = connections
        .clone()
        .iter()
        .fold(HashSet::new(), |mut acc, c| {
            let (l, op, r) = (c[0], c[1], c[2]);
            acc.insert((l, op));
            acc.insert((r, op));
            acc
        });

    let mut answers = connections
        .iter()
        .filter_map(|c| {
            let (l, op, r, out) = (c[0], c[1], c[2], c[3]);
            match op {
                "AND" => (l != "x00" && r != "x00" && !related_gates.contains(&(out, "OR")))
                    .then_some(out),
                "OR" => (out.starts_with("z") && out != "z45").then_some(out),
                "XOR" => ((l.starts_with("x") || r.starts_with("x"))
                    && (l != "x00" && r != "x00" && !related_gates.contains(&(out, "XOR")))
                    || (!out.starts_with("z") && !l.starts_with("x") && !r.starts_with("x")))
                .then_some(out),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
    answers.sort();
    answers.join(",")
}

fn cal<'a>(
    k: &'a str,
    kv: &mut HashMap<&'a str, bool>,
    connections: &HashMap<&'a str, Vec<&'a str>>,
) -> bool {
    let input = &connections[k];
    let (l, op, r) = (input[0], input[1], input[2]);
    if !kv.contains_key(l) {
        let v = cal(l, kv, connections);
        kv.insert(l, v);
    }
    if !kv.contains_key(r) {
        let v = cal(r, kv, connections);
        kv.insert(r, v);
    }
    let v = match op {
        "AND" => kv[l] && kv[r],
        "OR" => kv[l] || kv[r],
        "XOR" => kv[l] != kv[r],
        _ => unreachable!(),
    };
    kv.insert(k, v);

    v
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2024);
    }
}
