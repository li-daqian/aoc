use aoc_runner_derive::aoc;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let (locks, keys) =
        input
            .split("\n\n")
            .fold((vec![], vec![]), |(mut lockes, mut keys), chunk| {
                let grid = chunk
                    .lines()
                    .map(|line| line.as_bytes())
                    .collect::<Vec<_>>();
                if grid[0] == b"#####" && grid[6] == b"....." {
                    lockes.push(grid);
                } else {
                    keys.push(grid);
                }
                (lockes, keys)
            });

    let height = locks[0].len();
    let width = locks[0][0].len();
    locks
        .iter()
        .flat_map(|lock| {
            keys.iter().filter(move |key| {
                (1..height - 1).all(|row| {
                    (0..width).all(|col| !(lock[row][col] == b'#' && key[row][col] == b'#'))
                })
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3);
    }
}
