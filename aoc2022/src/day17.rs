use std::{collections::HashMap, fmt::Display};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone)]
struct Game<'a> {
    tower: Vec<u8>,

    jets: &'a [u8],
    jet_index: usize,

    rocks: &'a Vec<Vec<u8>>,
    rock_index: usize,
    rock_y: usize,
}

impl<'a> Game<'a> {
    pub fn new(input: &'a str, rocks: &'a Vec<Vec<u8>>) -> Self {
        Game {
            tower: vec![],
            jets: input.as_bytes(),
            jet_index: 0,
            rocks: rocks,
            rock_index: 0,
            rock_y: 0,
        }
    }

    pub fn next_rock(&mut self) -> Vec<u8> {
        let rock = &self.rocks[self.rock_index];
        self.rock_index += 1;
        if self.rock_index >= self.rocks.len() {
            self.rock_index = 0;
        }

        rock.clone()
    }

    pub fn make_space(&mut self, space: usize) {
        self.rock_y = self.tower.len() + space;
    }

    pub fn try_shift(&mut self, rock: &mut Vec<u8>) {
        let jet = self.jets[self.jet_index];
        self.jet_index += 1;
        if self.jet_index >= self.jets.len() {
            self.jet_index = 0;
        }

        match jet {
            b'>' => {
                if !self.hit_wall(rock, &0b00000001) {
                    for row in rock.iter_mut() {
                        *row >>= 1;
                    }
                    if self.hit_rock(rock, self.rock_y) {
                        for row in rock.iter_mut() {
                            *row <<= 1;
                        }
                    }
                }
            }
            b'<' => {
                if !self.hit_wall(rock, &0b01000000) {
                    for row in rock.iter_mut() {
                        *row <<= 1;
                    }
                    if self.hit_rock(rock, self.rock_y) {
                        for row in rock.iter_mut() {
                            *row >>= 1;
                        }
                    }
                }
            }
            _ => {
                panic!("Unknown jet direction: {}", jet as char);
            }
        }
    }

    pub fn can_fall(&self, rock: &Vec<u8>) -> bool {
        self.rock_y > 0 && !self.hit_rock(rock, self.rock_y - 1)
    }

    pub fn fall(&mut self) {
        self.rock_y -= 1;
    }

    pub fn stack(&mut self, rock: &Vec<u8>) {
        for (i, row) in rock.iter().enumerate() {
            if self.rock_y + i < self.tower.len() {
                self.tower[self.rock_y + i] |= row;
            } else {
                self.tower.push(*row);
            }
        }
    }

    fn hit_wall(&self, rock: &Vec<u8>, wall: &u8) -> bool {
        for row in rock {
            if row & wall != 0 {
                return true;
            }
        }
        false
    }

    fn hit_rock(&self, rock: &Vec<u8>, rock_y: usize) -> bool {
        for (i, row) in rock.iter().enumerate() {
            if rock_y + i < self.tower.len() && (self.tower[rock_y + i] & row) != 0 {
                return true;
            }
        }
        false
    }
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.tower.len()).rev() {
            write!(f, "{:08b}\n", self.tower[i])?;
        }
        Ok(())
    }
}

fn get_rocks() -> Vec<Vec<u8>> {
    let mut rocks = vec![];

    /*
    ####
    */
    rocks.push(vec![0b00011110]);

    /*
    .#.
    ###
    .#.
    */
    rocks.push(vec![0b00001000, 0b00011100, 0b00001000]);

    /*
    ..#
    ..#
    ###
    */
    rocks.push(vec![0b00011100, 0b00000100, 0b00000100]);

    /*
    #
    #
    #
    #
    */
    rocks.push(vec![0b00010000, 0b00010000, 0b00010000, 0b00010000]);

    /*
    ##
    ##
    */
    rocks.push(vec![0b00011000, 0b00011000]);

    rocks
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let rocks = get_rocks();
    let mut game = Game::new(input, &rocks);

    for _ in 0..2022 {
        let mut rock = game.next_rock();
        game.make_space(3);

        loop {
            game.try_shift(&mut rock);

            if game.can_fall(&rock) {
                game.fall();
            } else {
                game.stack(&rock);
                break;
            }
        }
    }

    game.tower.len()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let rocks = get_rocks();
    let mut game = Game::new(input, &rocks);

    let mut seen = HashMap::new();
    let mut added_height = 0;
    let mut i = 0;
    let total_rocks = 1_000_000_000_000usize;

    while i < total_rocks {
        let mut rock = game.next_rock();
        game.make_space(3);

        loop {
            game.try_shift(&mut rock);

            if game.can_fall(&rock) {
                game.fall();
            } else {
                game.stack(&rock);
                break;
            }
        }

        // Only keep the top N rows for state (e.g., 20)
        let n = 20;
        let top = game.tower.iter().rev().take(n).cloned().collect::<Vec<_>>();
        let state = (game.rock_index, game.jet_index, top);

        if let Some(&(old_i, old_height)) = seen.get(&state) {
            let cycle_len = i - old_i;
            let cycle_height = game.tower.len() - old_height;
            let cycles = (total_rocks - i) / cycle_len;

            added_height += cycles * cycle_height;
            i += cycles * cycle_len;
        } else {
            seen.insert(state, (i, game.tower.len()));
        }

        i += 1;
    }

    game.tower.len() + added_height
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1514285714288);
    }
}
