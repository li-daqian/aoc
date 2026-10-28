use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn(self, turn: char) -> Self {
        use Direction::*;
        match (self, turn) {
            (Right, 'R') | (Left, 'L') => Down,
            (Down, 'R') | (Up, 'L') => Left,
            (Left, 'R') | (Right, 'L') => Up,
            (Up, 'R') | (Down, 'L') => Right,
            _ => panic!("unsupported turn: {turn}"),
        }
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
        }
    }

    fn password_value(self) -> i64 {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<u8>>, &str) {
    let (map, instructions) = input
        .split_once("\n\n")
        .expect("map and instructions must be separated by a blank line");
    let map_lines: Vec<&str> = map.lines().collect();
    let width = map_lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let grid = map_lines
        .into_iter()
        .map(|line| {
            let mut row = line.as_bytes().to_vec();
            row.resize(width, b' ');
            row
        })
        .collect();
    (grid, instructions.trim())
}

fn move_steps(
    grid: &[Vec<u8>],
    row: &mut usize,
    col: &mut usize,
    direction: Direction,
    steps: usize,
) {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let (dr, dc) = direction.delta();

    for _ in 0..steps {
        let mut next_row = *row as isize + dr;
        let mut next_col = *col as isize + dc;

        // Skip the void by wrapping until reaching the first map cell in this
        // row or column.
        let mut wrapped = 0;
        while next_row < 0
            || next_row >= height
            || next_col < 0
            || next_col >= width
            || grid[next_row as usize][next_col as usize] == b' '
        {
            wrapped += 1;
            assert!(wrapped <= height.max(width), "no map cell in movement line");
            next_row = (next_row + dr).rem_euclid(height);
            next_col = (next_col + dc).rem_euclid(width);
        }

        match grid[next_row as usize][next_col as usize] {
            b'#' => break,
            b'.' => {
                *row = next_row as usize;
                *col = next_col as usize;
            }
            cell => panic!("unexpected map cell: {cell:?}"),
        }
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    let (grid, instructions) = parse(input);
    let mut row = 0;
    let mut col = grid[0]
        .iter()
        .position(|&cell| cell == b'.')
        .expect("the first row has a starting path");
    let mut direction = Direction::Right;
    let mut steps = 0;

    for token in instructions.bytes() {
        if token.is_ascii_digit() {
            steps = steps * 10 + usize::from(token - b'0');
        } else {
            move_steps(&grid, &mut row, &mut col, direction, steps);
            steps = 0;
            direction = direction.turn(token as char);
        }
    }
    move_steps(&grid, &mut row, &mut col, direction, steps);

    1000 * (row as i64 + 1) + 4 * (col as i64 + 1) + direction.password_value()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector(i8, i8, i8);

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Orientation {
    right: Vector,
    down: Vector,
    normal: Vector,
}

impl Orientation {
    fn fold(self, direction: Direction) -> Self {
        let Self {
            right,
            down,
            normal,
        } = self;
        match direction {
            Direction::Right => Self {
                right: -normal,
                down,
                normal: right,
            },
            Direction::Down => Self {
                right,
                down: -normal,
                normal: down,
            },
            Direction::Left => Self {
                right: normal,
                down,
                normal: -right,
            },
            Direction::Up => Self {
                right,
                down: normal,
                normal: -down,
            },
        }
    }

    fn movement(self, direction: Direction) -> Vector {
        match direction {
            Direction::Right => self.right,
            Direction::Down => self.down,
            Direction::Left => -self.right,
            Direction::Up => -self.down,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

struct Cube {
    size: usize,
    origins: Vec<(usize, usize)>,
    orientations: Vec<Orientation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    face: usize,
    row: usize,
    col: usize,
    direction: Direction,
}

impl Cube {
    fn new(grid: &[Vec<u8>]) -> Self {
        let area = grid.iter().flatten().filter(|&&cell| cell != b' ').count();
        let mut size = 1;
        while 6 * size * size < area {
            size += 1;
        }
        assert_eq!(
            6 * size * size,
            area,
            "map must contain six equal square faces"
        );
        let mut origins = Vec::new();
        for row in (0..grid.len()).step_by(size) {
            for col in (0..grid[0].len()).step_by(size) {
                if grid[row][col] != b' ' {
                    assert!((row..row + size).all(|r| (col..col + size).all(|c| grid
                        .get(r)
                        .and_then(|line| line.get(c))
                        .is_some_and(|&cell| cell == b'.' || cell == b'#'))));
                    origins.push((row, col));
                }
            }
        }
        assert_eq!(origins.len(), 6);
        let mut orientations = vec![None; 6];
        orientations[0] = Some(Orientation {
            right: Vector(1, 0, 0),
            down: Vector(0, 1, 0),
            normal: Vector(0, 0, 1),
        });
        let mut queue = std::collections::VecDeque::from([0]);
        while let Some(face) = queue.pop_front() {
            let (row, col) = origins[face];
            for direction in DIRECTIONS {
                let (dr, dc) = direction.delta();
                let neighbor = (
                    row as isize + dr * size as isize,
                    col as isize + dc * size as isize,
                );
                let Some(next) = origins
                    .iter()
                    .position(|&(r, c)| (r as isize, c as isize) == neighbor)
                else {
                    continue;
                };
                let folded = orientations[face].unwrap().fold(direction);
                if let Some(existing) = orientations[next] {
                    assert_eq!(existing, folded, "inconsistent cube net");
                } else {
                    orientations[next] = Some(folded);
                    queue.push_back(next);
                }
            }
        }
        let orientations: Vec<_> = orientations
            .into_iter()
            .map(|o| o.expect("connected cube net"))
            .collect();
        for face in 0..6 {
            assert!(
                orientations[..face]
                    .iter()
                    .all(|o| o.normal != orientations[face].normal),
                "cube faces must have distinct normals"
            );
        }
        Self {
            size,
            origins,
            orientations,
        }
    }

    fn next(&self, position: Position) -> Position {
        let Position {
            face,
            row,
            col,
            direction,
        } = position;
        let (dr, dc) = direction.delta();
        let (r, c) = (row as isize + dr, col as isize + dc);
        if (0..self.size as isize).contains(&r) && (0..self.size as isize).contains(&c) {
            return Position {
                row: r as usize,
                col: c as usize,
                ..position
            };
        }
        let source = self.orientations[face];
        let target_face = self
            .orientations
            .iter()
            .position(|o| o.normal == source.movement(direction))
            .unwrap();
        let target = self.orientations[target_face];
        // After crossing an edge, forward points along the old face's inward normal.
        let new_direction = DIRECTIONS
            .into_iter()
            .find(|&d| target.movement(d) == -source.normal)
            .unwrap();
        let (offset, source_axis) = match direction {
            Direction::Right | Direction::Left => (row, source.down),
            Direction::Down | Direction::Up => (col, source.right),
        };
        let target_axis = match new_direction {
            Direction::Right | Direction::Left => target.down,
            Direction::Down | Direction::Up => target.right,
        };
        assert!(source_axis == target_axis || source_axis == -target_axis);
        let offset = if source_axis == target_axis {
            offset
        } else {
            self.size - 1 - offset
        };
        let (row, col) = match new_direction {
            Direction::Right => (offset, 0),
            Direction::Down => (0, offset),
            Direction::Left => (offset, self.size - 1),
            Direction::Up => (self.size - 1, offset),
        };
        Position {
            face: target_face,
            row,
            col,
            direction: new_direction,
        }
    }

    fn advance(&self, grid: &[Vec<u8>], position: &mut Position, steps: usize) {
        for _ in 0..steps {
            let next = self.next(*position);
            let (top, left) = self.origins[next.face];
            if grid[top + next.row][left + next.col] == b'#' {
                break;
            }
            *position = next;
        }
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    let (grid, instructions) = parse(input);
    let cube = Cube::new(&grid);
    let start_col = grid[0].iter().position(|&cell| cell == b'.').unwrap();
    let face = cube
        .origins
        .iter()
        .position(|&(top, left)| top == 0 && (left..left + cube.size).contains(&start_col))
        .unwrap();
    let mut position = Position {
        face,
        row: 0,
        col: start_col - cube.origins[face].1,
        direction: Direction::Right,
    };
    let mut steps = 0;
    for token in instructions.bytes() {
        if token.is_ascii_digit() {
            steps = steps * 10 + usize::from(token - b'0');
        } else {
            cube.advance(&grid, &mut position, steps);
            steps = 0;
            position.direction = position.direction.turn(token as char);
        }
    }
    cube.advance(&grid, &mut position, steps);
    let (top, left) = cube.origins[position.face];
    1000 * (top + position.row + 1) as i64
        + 4 * (left + position.col + 1) as i64
        + position.direction.password_value()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 6032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 5031);
    }

    #[test]
    fn cube_edges_are_reversible() {
        for input in [SAMPLE, include_str!("../input/2022/day22.txt")] {
            let (grid, _) = parse(input);
            let cube = Cube::new(&grid);
            for face in 0..6 {
                for direction in DIRECTIONS {
                    for offset in 0..cube.size {
                        let (row, col) = match direction {
                            Direction::Right => (offset, cube.size - 1),
                            Direction::Down => (cube.size - 1, offset),
                            Direction::Left => (offset, 0),
                            Direction::Up => (0, offset),
                        };
                        let start = Position {
                            face,
                            row,
                            col,
                            direction,
                        };
                        let next = cube.next(start);
                        let back = cube.next(Position {
                            direction: next.direction.turn('R').turn('R'),
                            ..next
                        });
                        assert_eq!(
                            back,
                            Position {
                                direction: direction.turn('R').turn('R'),
                                ..start
                            }
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn wall_across_edge_preserves_position_and_direction() {
        let (mut grid, _) = parse(SAMPLE);
        let cube = Cube::new(&grid);
        let start = Position {
            face: 0,
            row: 0,
            col: 0,
            direction: Direction::Up,
        };
        let target = cube.next(start);
        let (top, left) = cube.origins[target.face];
        grid[top + target.row][left + target.col] = b'#';
        let mut position = start;
        cube.advance(&grid, &mut position, 10);
        assert_eq!(position, start);
    }
}
