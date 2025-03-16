use std::collections::HashSet;

use aoc_runner_derive::aoc;

struct Game {
    mazes: Mazes,
    player: Player,
    directions: [(isize, isize); 4],
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct Player {
    location: (isize, isize),
    direction: usize,
}

struct Mazes {
    matrix: Vec<Vec<char>>,
    height: isize,
    width: isize,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut start = (0, 0);
        let matrix: Vec<Vec<char>> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == '^' {
                            start = (i as isize, j as isize);
                            '.'
                        } else {
                            c
                        }
                    })
                    .collect()
            })
            .collect();
        let (height, width) = (matrix.len() as isize, matrix[0].len() as isize);
        Self {
            mazes: Mazes {
                matrix,
                height,
                width,
            },
            player: Player {
                location: start,
                direction: 0,
            },
            directions: [(-1, 0), (0, 1), (1, 0), (0, -1)],
        }
    }

    fn next(&self) -> (isize, isize) {
        let (x, y) = self.player.location;
        let (dx, dy) = self.directions[self.player.direction];
        (x + dx, y + dy)
    }

    fn peek(&self) -> Option<char> {
        let (x, y) = self.next();
        if x < 0 || y < 0 || x >= self.mazes.height || y >= self.mazes.width {
            return None;
        }
        Some(self.get_value((x, y)))
    }

    fn change_direction(&mut self) {
        self.player.direction = (self.player.direction + 1) % 4;
    }

    fn move_forward(&mut self) {
        self.player.location = self.next();
    }

    fn get_value(&self, (x, y): (isize, isize)) -> char {
        self.mazes.matrix[x as usize][y as usize]
    }

    fn set_value(&mut self, (x, y): (isize, isize), c: char) {
        self.mazes.matrix[x as usize][y as usize] = c;
    }

    fn has_loop(&mut self) -> bool {
        let mut walked = vec![
            false;
            (self.mazes.width * self.mazes.height * self.directions.len() as isize)
                as usize
        ];
        loop {
            let i = ((self.player.location.0 * self.mazes.width) as usize
                + (self.player.location.1) as usize)
                * self.directions.len()
                + self.player.direction;
            if walked[i] == true {
                return true;
            }
            walked[i] = true;

            if let Some(next) = self.peek() {
                if next == '.' {
                    self.move_forward();
                } else {
                    self.change_direction();
                }
            } else {
                return false;
            }
        }
    }
}

#[aoc(day6, part1)]
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut game = Game::from_input(input);
    let mut walked_path: HashSet<(isize, isize)> = HashSet::new();
    loop {
        walked_path.insert(game.player.location);
        if let Some(next) = game.peek() {
            if next == '.' {
                game.move_forward();
            } else {
                game.change_direction();
            }
        } else {
            return walked_path.len();
        }
    }
}

#[aoc(day6, part2)]
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut game = Game::from_input(input);
    let mut answer = 0;
    let mut walked_path: HashSet<(isize, isize)> = HashSet::new();
    loop {
        walked_path.insert(game.player.location);
        if let Some(next) = game.peek() {
            if next == '.' {
                let (next_x, next_y) = game.next();
                if !walked_path.contains(&(next_x, next_y)) {
                    let start = game.player.clone();
                    game.set_value((next_x, next_y), '#');
                    if game.has_loop() {
                        answer += 1;
                    }
                    game.player = start;
                    game.set_value((next_x, next_y), '.');
                }
                game.move_forward();
            } else {
                game.change_direction();
            }
        } else {
            return answer;
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 6);
    }
}
