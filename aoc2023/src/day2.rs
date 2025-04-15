use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    const COLORS: &[&[u8]] = &[b"red", b"green", b"blue"];
    const MAX_COLORS: &[usize; 3] = &[12, 13, 14];

    let games = input
        .lines()
        .map(|line| {
            let (id, game) = line.trim_start_matches("Game ").split_once(":").unwrap();
            let colors = game
                .split([';', ','])
                .map(|s| {
                    let (count, color) = s.trim().split_once(' ').unwrap();
                    let color = COLORS.iter().position(|&c| c == color.as_bytes()).unwrap();
                    (count.parse::<usize>().unwrap(), color)
                })
                .collect::<Vec<_>>();
            (id.parse::<usize>().unwrap(), colors)
        })
        .collect::<Vec<_>>();

    games
        .iter()
        .filter_map(|(id, actions)| {
            actions
                .iter()
                .all(|&(count, color)| count <= MAX_COLORS[color])
                .then_some(id)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    const COLORS: &[&[u8]] = &[b"red", b"green", b"blue"];

    let games = input
        .lines()
        .map(|line| {
            let (id, game) = line.trim_start_matches("Game ").split_once(":").unwrap();
            let colors = game
                .split([';', ','])
                .map(|s| {
                    let (count, color) = s.trim().split_once(' ').unwrap();
                    let color = COLORS.iter().position(|&c| c == color.as_bytes()).unwrap();
                    (count.parse::<usize>().unwrap(), color)
                })
                .collect::<Vec<_>>();
            (id.parse::<usize>().unwrap(), colors)
        })
        .collect::<Vec<_>>();

    games
        .iter()
        .map(|(_, actions)| {
            let mut color_counts = [0; 3];
            for &(count, color) in actions {
                if count > color_counts[color] {
                    color_counts[color] = count;
                }
            }
            color_counts.iter().fold(1usize, |acc, &count| acc * count)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2286);
    }
}
