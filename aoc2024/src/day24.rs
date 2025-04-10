use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let (input1, input2) = input.split_once("\n\n").unwrap();
    let mut kv = input1
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(": ").unwrap();
            (k, v == "1")
        })
        .collect();

    let operation = input2
        .lines()
        .map(|line| {
            let (input, out) = line.split_once(" -> ").unwrap();
            let input = input.split(" ").collect::<Vec<_>>();
            (out, input)
        })
        .collect();

    for (&out, input) in &operation {
        cal(input, out, &mut kv, &operation);
    }

    let answer = {
        let mut keys = kv
            .keys()
            .filter(|&k| k.starts_with("z"))
            .collect::<Vec<_>>();
        keys.sort();
        keys.into_iter().enumerate().fold(
            0usize,
            |acc, (i, k)| {
                if kv[k] {
                    acc | (1 << i)
                } else {
                    acc
                }
            },
        )
    };

    answer
}

fn cal<'a>(
    input: &Vec<&'a str>,
    k: &'a str,
    kv: &mut HashMap<&'a str, bool>,
    operation: &HashMap<&'a str, Vec<&'a str>>,
) -> bool {
    let (a, op, c) = (input[0], input[1], input[2]);
    if !kv.contains_key(a) {
        let v = cal(&operation[a], a, kv, operation);
        kv.insert(a, v);
    }
    if !kv.contains_key(c) {
        let v = cal(&operation[c], c, kv, operation);
        kv.insert(c, v);
    }
    let v = match op {
        "AND" => kv[a] && kv[c],
        "OR" => kv[a] || kv[c],
        "XOR" => kv[a] != kv[c],
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
