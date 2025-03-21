use aoc_runner_derive::aoc;

#[derive(Default, Debug)]
struct ClawMachines {
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    px: isize,
    py: isize,
}

impl ClawMachines {
    fn calculate(&self) -> isize {
        let mut tokens = 0;
        for i in 0..=self.px / self.ax {
            let j = (self.px - self.ax * i) / self.bx;
            if self.ax * i + self.bx * j == self.px && self.ay * i + self.by * j == self.py {
                if tokens > 0 {
                    tokens = tokens.min(i * 3 + j)
                } else {
                    tokens = i * 3 + j;
                }
            }
        }

        tokens
    }

    fn calculate_2(&self) -> isize {
        // ax * i + bx * j = px
        // ay * i + by * j = py
        // i = (px - bx * j) / ax
        // ay * (px - bx * j) / ax + by * j = py
        // ay * (px - bx * j) + by * j * ax = py * ax
        // ay * px - ay * bx * j + by * j * ax = py * ax
        // j * (by * ax - ay * bx) = py * ax - ay * px

        let i = (self.py * self.bx - self.px * self.by) / (self.ay * self.bx - self.ax * self.by);
        let j = (self.py * self.ax - self.px * self.ay) / (self.by * self.ax - self.bx * self.ay);
        if i * self.ax + j * self.bx == self.px && i * self.ay + j * self.by == self.py {
            return i * 3 + j;
        }
        0
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> isize {
    let mut claw_machines = ClawMachines::default();
    input.lines().fold(0, |acc, line| {
        if line.starts_with("Button A: ") {
            line["Button A: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.ax = value as isize,
                    "Y" => claw_machines.ay = value as isize,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Button B: ") {
            line["Button B: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.bx = value as isize,
                    "Y" => claw_machines.by = value as isize,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Prize: ") {
            line["Prize: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('=').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.px = value as isize,
                    "Y" => claw_machines.py = value as isize,
                    _ => unreachable!(),
                }
            });
            let tokens = claw_machines.calculate();
            claw_machines = ClawMachines::default();
            return acc + tokens;
        }
        acc
    })
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> isize {
    let mut claw_machines = ClawMachines::default();
    input.lines().fold(0, |acc, line| {
        if line.starts_with("Button A: ") {
            line["Button A: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.ax = value as isize,
                    "Y" => claw_machines.ay = value as isize,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Button B: ") {
            line["Button B: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.bx = value as isize,
                    "Y" => claw_machines.by = value as isize,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Prize: ") {
            line["Prize: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('=').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.px = value as isize + 10000000000000,
                    "Y" => claw_machines.py = value as isize + 10000000000000,
                    _ => unreachable!(),
                }
            });
            let tokens = claw_machines.calculate_2();
            claw_machines = ClawMachines::default();
            return acc + tokens;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 480);
    }
}
