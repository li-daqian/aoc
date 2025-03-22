use std::fmt::Display;

use aoc_runner_derive::aoc;

#[derive(Default)]
struct WareHouse {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    pos: (usize, usize),
}

impl Display for WareHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if (i, j) == self.pos {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl WareHouse {
    #[inline]
    fn set(&mut self, (x, y): (usize, usize), c: char) {
        self.map[x][y] = c;
    }

    #[inline]
    fn next_pos(&self, pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        let (x, y) = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        if x < 0 || x >= self.height as isize || y < 0 || y >= self.width as isize {
            return None;
        }
        Some((x as usize, y as usize))
    }

    #[inline]
    fn get(&self, (x, y): (usize, usize)) -> char {
        self.map[x][y]
    }

    fn find_empty_pos(&self, pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        let next_p = self.next_pos(pos, dir)?;
        let next = self.get(next_p);
        if next == '#' {
            return None;
        } else if next == '.' {
            return Some(next_p);
        } else {
            return self.find_empty_pos(next_p, dir);
        }
    }

    fn try_move(&mut self, dir: char) {
        let dir = make_dir(dir);
        if let Some(next_pos) = self.next_pos(self.pos, dir) {
            let next = self.get(next_pos);
            if next == '#' {
                return;
            }
            if next == '.' {
                self.pos = next_pos;
            } else {
                if let Some(empty_pos) = self.find_empty_pos(next_pos, dir) {
                    self.set(next_pos, '.');
                    self.set(empty_pos, 'O');
                    self.pos = next_pos;
                }
            }
        }
    }

    fn try_move_2(&mut self, dir: char) {
        let dir = make_dir(dir);
        if let Some(next_pos) = self.next_pos(self.pos, dir) {
            let next = self.get(next_pos);
            if next == '#' {
                return;
            }
            if next == '.' {
                self.pos = next_pos;
            } else {
                let mut foods: Vec<(usize, usize, char)> = vec![];
                if self.move_food(&mut foods, next_pos, dir) {
                    foods.reverse();
                    foods.iter().for_each(|(x, y, c)| {
                        self.set(self.next_pos((*x, *y), dir).unwrap(), *c);
                        self.set((*x, *y), '.');
                    });
                    self.set(next_pos, '.');
                    self.pos = next_pos;
                }
            }
        }
    }

    fn move_food(
        &self,
        foods: &mut Vec<(usize, usize, char)>,
        pos: (usize, usize),
        dir: (isize, isize),
    ) -> bool {
        if dir.1 == 0 {
            let current_c1 = self.get(pos);
            let pos_2 = if current_c1 == '[' {
                self.next_pos(pos, (0, 1))
            } else if current_c1 == ']' {
                self.next_pos(pos, (0, -1))
            } else {
                None
            };
            if pos_2.is_none() {
                unreachable!();
            }
            let next_pos_1 = self.next_pos(pos, dir);
            if next_pos_1.is_none() {
                return false;
            }
            let next_pos_2 = self.next_pos(pos_2.unwrap(), dir);
            if next_pos_2.is_none() {
                return false;
            }
            let (next_c1, next_c2) = (self.get(next_pos_1.unwrap()), self.get(next_pos_2.unwrap()));
            if next_c1 == '#' || next_c2 == '#' {
                return false;
            }

            let current_c2 = self.get(pos_2.unwrap());
            foods.push((pos.0, pos.1, current_c1));
            foods.push((pos_2.unwrap().0, pos_2.unwrap().1, current_c2));
            if next_c1 == '.' && next_c2 == '.' {
                return true;
            }

            if (next_c1 == '[' || next_c1 == ']') && (next_c2 == '[' || next_c2 == ']') {
                return self.move_food(foods, next_pos_1.unwrap(), dir)
                    && self.move_food(foods, next_pos_2.unwrap(), dir);
            }

            if next_c1 == '[' || next_c1 == ']' {
                return self.move_food(foods, next_pos_1.unwrap(), dir);
            }
            if next_c2 == '[' || next_c2 == ']' {
                return self.move_food(foods, next_pos_2.unwrap(), dir);
            }
            unreachable!()
        } else {
            let next_pos = self.next_pos(pos, dir);
            if next_pos.is_none() {
                return false;
            }
            let next_c = self.get(next_pos.unwrap());
            if next_c == '#' {
                return false;
            }

            foods.push((pos.0, pos.1, self.get(pos)));
            if next_c == '.' {
                return true;
            }

            if next_c == '[' || next_c == ']' {
                return self.move_food(foods, next_pos.unwrap(), dir);
            }
            unreachable!()
        }
    }
}

#[inline]
fn make_dir(dir: char) -> (isize, isize) {
    match dir {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => unreachable!(),
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut ware_house = WareHouse::default();

    let mut is_house = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            is_house = false;
            ware_house.height = ware_house.map.len();
            ware_house.width = ware_house.map[0].len();
            return;
        }
        if is_house {
            ware_house.map.push(line.chars().collect());
            if ware_house.pos == (0, 0) {
                line.find('@').map(|i| {
                    ware_house.pos = (ware_house.map.len() - 1, i);
                    ware_house.set(ware_house.pos, '.');
                });
            }
        } else {
            line.chars().for_each(|c| {
                ware_house.try_move(c);
            });
        }
    });

    ware_house
        .map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &cell)| {
                    if cell == 'O' {
                        Some(i * 100 + j)
                    } else {
                        None
                    }
                },
            )
        })
        .sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut ware_house = WareHouse::default();

    let mut is_house = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            is_house = false;
            ware_house.height = ware_house.map.len();
            ware_house.width = ware_house.map[0].len();
            return;
        }
        if is_house {
            let row = line.chars().fold(vec![' '; 0], |mut acc, c| {
                if c == '#' {
                    acc.push('#');
                    acc.push('#');
                } else if c == '.' {
                    acc.push('.');
                    acc.push('.');
                } else if c == '@' {
                    ware_house.pos = (ware_house.map.len(), acc.len());
                    acc.push('.');
                    acc.push('.');
                } else if c == 'O' {
                    acc.push('[');
                    acc.push(']');
                }
                acc
            });
            ware_house.map.push(row);
        } else {
            line.chars().for_each(|c| {
                ware_house.try_move_2(c);
            });
        }
    });

    ware_house
        .map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, &cell)| {
                    if cell == '[' {
                        Some(i * 100 + j)
                    } else {
                        None
                    }
                },
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 9021);
    }
}
