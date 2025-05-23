use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let mut lines = drawing.lines().rev();
    let stack_count = lines.next().unwrap().split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];

    for line in drawing.lines().rev().skip(1) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let idx = 1 + i * 4;
            if let Some(c) = line.chars().nth(idx) {
                if c.is_ascii_alphabetic() {
                    stack.push(c);
                }
            }
        }
    }

    for line in moves.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let count: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse::<usize>().unwrap() - 1;
        let to: usize = parts[5].parse::<usize>().unwrap() - 1;
        for _ in 0..count {
            if let Some(c) = stacks[from].pop() {
                stacks[to].push(c);
            }
        }
    }

    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

// #[aoc(day5, part2)]
// pub fn part2(input: &str) -> usize {}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
        1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), "CMZ");
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE), 4);
    // }
}
