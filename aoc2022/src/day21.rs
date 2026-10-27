use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Job {
    Value(i64),
    Operation {
        left: String,
        operator: char,
        right: String,
    },
}

fn parse(input: &str) -> HashMap<String, Job> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (name, job) = line.split_once(':').expect("each line has a monkey name");
            let parts: Vec<_> = job.split_whitespace().collect();
            let job = if parts.len() == 1 {
                Job::Value(parts[0].parse().expect("value is an integer"))
            } else {
                Job::Operation {
                    left: parts[0].to_owned(),
                    operator: parts[1].chars().next().expect("operator is present"),
                    right: parts[2].to_owned(),
                }
            };
            (name.trim().to_owned(), job)
        })
        .collect()
}

fn evaluate(name: &str, jobs: &HashMap<String, Job>, memo: &mut HashMap<String, i64>) -> i64 {
    if let Some(&value) = memo.get(name) {
        return value;
    }

    let value = match jobs.get(name).expect("referenced monkey exists") {
        Job::Value(value) => *value,
        Job::Operation {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left, jobs, memo);
            let right = evaluate(right, jobs, memo);
            match operator {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!("unsupported operator: {operator}"),
            }
        }
    };

    memo.insert(name.to_owned(), value);
    value
}

fn contains_humn(name: &str, jobs: &HashMap<String, Job>, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(&contains) = memo.get(name) {
        return contains;
    }

    let contains = if name == "humn" {
        true
    } else {
        match jobs.get(name).expect("referenced monkey exists") {
            Job::Value(_) => false,
            Job::Operation { left, right, .. } => {
                contains_humn(left, jobs, memo) || contains_humn(right, jobs, memo)
            }
        }
    };

    memo.insert(name.to_owned(), contains);
    contains
}

fn solve_for_humn(
    name: &str,
    target: i64,
    jobs: &HashMap<String, Job>,
    contains_memo: &mut HashMap<String, bool>,
    value_memo: &mut HashMap<String, i64>,
) -> i64 {
    if name == "humn" {
        return target;
    }

    let Job::Operation {
        left,
        operator,
        right,
    } = jobs.get(name).expect("humn must be below an operation")
    else {
        panic!("humn is not reachable through an operation")
    };

    if contains_humn(left, jobs, contains_memo) {
        let known = evaluate(right, jobs, value_memo);
        let next_target = match operator {
            '+' => target - known,
            '-' => target + known,
            '*' => target / known,
            '/' => target * known,
            _ => panic!("unsupported operator: {operator}"),
        };
        solve_for_humn(left, next_target, jobs, contains_memo, value_memo)
    } else {
        let known = evaluate(left, jobs, value_memo);
        let next_target = match operator {
            '+' => target - known,
            '-' => known - target,
            '*' => target / known,
            '/' => known / target,
            _ => panic!("unsupported operator: {operator}"),
        };
        solve_for_humn(right, next_target, jobs, contains_memo, value_memo)
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    evaluate("root", &parse(input), &mut HashMap::new())
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    let jobs = parse(input);
    let Job::Operation { left, right, .. } = jobs.get("root").expect("root exists") else {
        panic!("root must be an operation")
    };
    let mut contains_memo = HashMap::new();
    let mut value_memo = HashMap::new();
    if contains_humn(left, &jobs, &mut contains_memo) {
        let target = evaluate(right, &jobs, &mut value_memo);
        solve_for_humn(left, target, &jobs, &mut contains_memo, &mut value_memo)
    } else {
        let target = evaluate(left, &jobs, &mut value_memo);
        solve_for_humn(right, target, &jobs, &mut contains_memo, &mut value_memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let sample = indoc! {
            "
            root: pppw + sjmn
            pppw: cczh / lfqf
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32
            "
        };
        assert_eq!(part1(sample), 152);
        assert_eq!(part2(sample), 301);
    }
}
