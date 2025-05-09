use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::aoc;

#[derive(Default, Debug, Clone, Copy)]
struct Register {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

#[derive(Debug)]
struct Simulater {
    register: Register,
    instructions: Vec<Instruction>,
    ip: usize,
}

fn combo(register: &Register, operand: u8) -> usize {
    match operand {
        0..=3 => operand as usize,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Invalid operand"),
    }
}

impl Simulater {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();

        let mut register = Register::default();
        register.a = lines.next().unwrap()[12..].parse().unwrap();
        register.b = lines.next().unwrap()[12..].parse().unwrap();
        register.c = lines.next().unwrap()[12..].parse().unwrap();

        lines.next();

        let instructions = lines.next().unwrap()[9..]
            .split(",")
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| Instruction {
                opcode: chunk[0].parse().unwrap(),
                operand: chunk[1].parse().unwrap(),
            })
            .collect();
        Self {
            register,
            instructions,
            ip: 0,
        }
    }

    fn execute(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let mut ip = self.ip;
        let register = &mut self.register;

        loop {
            if ip >= self.instructions.len() {
                break;
            }

            let instruction = &self.instructions[ip];
            let opcode = instruction.opcode;
            let operand = instruction.operand;
            match opcode {
                0 => {
                    register.a >>= combo(register, operand);
                }
                1 => {
                    register.b ^= operand as usize;
                }
                2 => {
                    register.b = combo(register, operand) % 8;
                }
                3 => {
                    if register.a != 0 {
                        ip = combo(register, operand);
                        continue;
                    }
                }
                4 => {
                    register.b ^= register.c;
                }
                5 => {
                    output.push((combo(register, operand) % 8) as u8);
                }
                6 => {
                    register.b = register.a >> combo(register, operand);
                }
                7 => {
                    register.c = register.a >> combo(register, operand);
                }
                _ => panic!("Invalid opcode"),
            }
            ip += 1;
        }

        output
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut simulater = Simulater::from(input);
    simulater
        .execute()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// 2,4 -> B=A%8
// 1,7 -> B^=7
// 7,5 -> C=A>>B
// 0,3 -> A>>=3
// 1,7 -> B^=7
// 4,1 -> B^=C
// 5,5 -> output.push(B%8)
// 3,0 -> if A!=0 GOTO 0
#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let mut simulater = Simulater::from(input);
    let targets = simulater
        .instructions
        .iter()
        .flat_map(|x| vec![x.opcode, x.operand])
        .collect::<Vec<_>>();

    let mut cands: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for i in 1..8 {
        cands.push(Reverse(i));
    }
    while let Some(Reverse(a)) = cands.pop() {
        simulater.register.a = a;
        simulater.ip = 0;
        let output = simulater.execute();
        if output == targets {
            return a;
        }

        if output == targets[targets.len() - output.len()..] {
            for i in 0..8 {
                cands.push(Reverse((a << 3) + i));
            }
        }
    }

    panic!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let sample: &str = indoc! {"
            Register A: 2024
            Register B: 0
            Register C: 0

            Program: 0,3,5,4,3,0
        "};
        assert_eq!(part2(sample), 117440);
    }
}
