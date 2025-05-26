use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let check = |cycle: i32, x: i32, sum: &mut i32| {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            *sum += cycle * x;
        }
    };

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            check(cycle, x, &mut sum);
        } else if let Some(v) = line.strip_prefix("addx ") {
            let v: i32 = v.parse().unwrap();
            cycle += 1;
            check(cycle, x, &mut sum);
            cycle += 1;
            check(cycle, x, &mut sum);
            x += v;
        }
    }
    sum
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> String {
    let mut x = 1;
    let mut cycle = 0;
    let mut crt = String::new();

    let draw = |cycle: i32, x: i32, crt: &mut String| {
        let pos = (cycle - 1) % 40;
        if (x - 1..=x + 1).contains(&pos) {
            crt.push('#');
        } else {
            crt.push('.');
        }
        if pos == 39 {
            crt.push('\n');
        }
    };

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            draw(cycle, x, &mut crt);
        } else if let Some(v) = line.strip_prefix("addx ") {
            let v: i32 = v.parse().unwrap();
            cycle += 1;
            draw(cycle, x, &mut crt);
            cycle += 1;
            draw(cycle, x, &mut crt);
            x += v;
        }
    }
    crt
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE), 13140);
    }

    #[test]
    fn test_part2_sample() {
        let expected = indoc! {"
        ##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....
    "};
        assert_eq!(part2(SAMPLE), expected);
    }
}
