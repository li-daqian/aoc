use aoc_runner_derive::aoc;

#[derive(Default, Debug)]
struct ClawMachines {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    px: usize,
    py: usize,
}

impl ClawMachines {
    fn calculate(&self) -> usize {
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

    fn calculate_2(&self) -> usize {
        // ax * i + bx * j = px
        // ay * i + by * j = py
        // i = (px - bx * j) / ax
        // ay * (px - bx * j) / ax + by * j = py
        // ay * (px - bx * j) + by * j * ax = py * ax
        // ay * px - ay * bx * j + by * j * ax = py * ax
        // j * (by * ax - ay * bx) = py * ax - ay * px

        let i = (self.py as isize * self.bx as isize - self.px as isize * self.by as isize)
            / (self.ay as isize * self.bx as isize - self.ax as isize * self.by as isize);
        let j = (self.py as isize * self.ax as isize - self.px as isize * self.ay as isize)
            / (self.by as isize * self.ax as isize - self.bx as isize * self.ay as isize);
        if i * self.ax as isize + j * self.bx as isize == self.px as isize
            && i * self.ay as isize + j * self.by as isize == self.py as isize
        {
            return (i * 3 + j) as usize;
        }
        0
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let mut claw_machines = ClawMachines::default();
    input.lines().fold(0, |acc, line| {
        if line.starts_with("Button A: ") {
            line["Button A: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.ax = value,
                    "Y" => claw_machines.ay = value,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Button B: ") {
            line["Button B: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.bx = value,
                    "Y" => claw_machines.by = value,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Prize: ") {
            line["Prize: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('=').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.px = value,
                    "Y" => claw_machines.py = value,
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
pub fn part2(input: &str) -> usize {
    let mut claw_machines = ClawMachines::default();
    input.lines().fold(0, |acc, line| {
        if line.starts_with("Button A: ") {
            line["Button A: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.ax = value,
                    "Y" => claw_machines.ay = value,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Button B: ") {
            line["Button B: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('+').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.bx = value,
                    "Y" => claw_machines.by = value,
                    _ => unreachable!(),
                }
            });
        } else if line.starts_with("Prize: ") {
            line["Prize: ".len()..].split(", ").for_each(|s| {
                let (key, value) = s.split_once('=').unwrap();
                let value = value.parse::<usize>().unwrap();
                match key {
                    "X" => claw_machines.px = value + 10000000000000,
                    "Y" => claw_machines.py = value + 10000000000000,
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
