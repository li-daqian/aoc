use std::vec;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Bathroom {
    height: isize,
    width: isize,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

impl Robot {
    fn from(input: &str) -> Self {
        let (p, v) = input.split_once(' ').unwrap();
        let p = p[2..].split_once(',').unwrap();
        let v = v[2..].split_once(',').unwrap();
        let p = (p.0.parse::<isize>().unwrap(), p.1.parse::<isize>().unwrap());
        let v = (v.0.parse::<isize>().unwrap(), v.1.parse::<isize>().unwrap());
        Robot { p, v }
    }
}

impl Bathroom {
    fn from(input: &str, width: isize, height: isize) -> Self {
        Bathroom {
            height,
            width,
            robots: input.lines().map(Robot::from).collect(),
        }
    }

    fn elapse(&mut self, seconds: usize) {
        for robot in &mut self.robots {
            let x = &mut robot.p.0;
            let y = &mut robot.p.1;
            *x += robot.v.0 * seconds as isize;
            *x = ((*x % self.width) + self.width) % self.width;
            *y += robot.v.1 * seconds as isize;
            *y = ((*y % self.height) + self.height) % self.height;
        }
    }

    fn get_safety_factor(&self) -> usize {
        self.robots
            .iter()
            .fold([[0; 2]; 2], |mut acc, robot| {
                let x = robot.p.0;
                let y = robot.p.1;
                if x != self.width / 2 && y != self.height / 2 {
                    let px = if x > self.width / 2 { 1 } else { 0 };
                    let py = if y > self.height / 2 { 1 } else { 0 };
                    acc[px][py] += 1;
                }
                acc
            })
            .iter()
            .flatten()
            .fold(1, |acc, &x| acc * x)
    }

    fn is_tree(&self) -> bool {
        let height = self.height as usize;
        let width = self.width as usize;
        let robot_bits =
            self.robots
                .iter()
                .fold(vec![vec![false; width]; height], |mut acc, robot| {
                    let col = robot.p.0 as usize;
                    let row = robot.p.1 as usize;
                    acc[row][col] = true;
                    acc
                });
        let mut count = 0;
        self.robots.iter().for_each(|robot| {
            let col = robot.p.0 as usize;
            let row = robot.p.1 as usize;
            if row < height - 1 - row && robot_bits[height - 1 - row][col] {
                count += 1;
            }
        });

        if count >= 50 {
            for row in 0..height {
                for col in 0..width {
                    if robot_bits[row][col] {
                        print!("X");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            return true;
        }
        false
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut bathroom = Bathroom::from(input, 101, 103);
    bathroom.elapse(100);
    bathroom.get_safety_factor()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let mut bathroom = Bathroom::from(input, 101, 103);
    let mut seconds = 0;
    loop {
        bathroom.elapse(1);
        seconds += 1;
        if bathroom.is_tree() {
            break;
        }
    }
    seconds
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn test_part1() {
        let mut bathroom = Bathroom::from(SAMPLE, 7, 11);
        bathroom.elapse(100);
        let answer = bathroom.get_safety_factor();
        assert_eq!(answer, 12);
    }
}
