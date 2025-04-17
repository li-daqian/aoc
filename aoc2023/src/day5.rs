use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let inputs = input.split("\n\n").collect::<Vec<_>>();

    let seeds = inputs[0]
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut layers = inputs
        .iter()
        .skip(1)
        .map(|line| {
            line.lines()
                .skip(1)
                .map(|line| {
                    line.split(" ")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    layers.iter_mut().for_each(|layer| {
        layer.sort_by(|a, b| a[1].cmp(&b[1]));
    });

    seeds
        .iter()
        .map(|seed| {
            let mut number = seed.clone();
            for layer in &layers {
                number = layer
                    .iter()
                    .find_map(|mapping| {
                        let (begin, end, target) =
                            (mapping[1], mapping[1] + mapping[2], mapping[0]);
                        if number >= begin && number < end {
                            Some(target + (number - begin))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(number);
            }
            number
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let inputs = input.split("\n\n").collect::<Vec<_>>();

    let seeds = inputs[0]
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut layers = inputs
        .iter()
        .skip(1)
        .map(|line| {
            line.lines()
                .skip(1)
                .map(|line| {
                    line.split(" ")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    layers.iter_mut().for_each(|layer| {
        layer.sort_by(|a, b| a[1].cmp(&b[1]));
    });

    let get_values = |layer: &Vec<Vec<usize>>, (start, end): (usize, usize)| {
        let mut values = vec![];
        for (index, mapping) in layer.iter().enumerate() {
            let (mapping_start, mapping_end, target) =
                (mapping[1], mapping[1] + mapping[2], mapping[0]);
            if index == 0 {
                if end < mapping_start {
                    values.push((start, end));
                } else if start < mapping_start {
                    values.push((start, mapping_start));
                }
            }
            if index == layer.len() - 1 {
                if start > mapping_end {
                    values.push((start, end));
                } else if end > mapping_end {
                    values.push((mapping_end, end));
                }
            }
            if start < mapping_end && end > mapping_start {
                let (start, end) = (mapping_start.max(start), mapping_end.min(end));
                values.push((
                    target + (start - mapping_start),
                    target + (end - mapping_start),
                ));
            }
        }
        values
    };

    seeds
        .windows(2)
        .step_by(2)
        .filter_map(|pair| {
            let (start, end) = (pair[0], pair[0] + pair[1]);
            let mut values = vec![(start, end)];
            for layer in &layers {
                let mut new_values = vec![];
                for (start, end) in values {
                    new_values.extend(get_values(layer, (start, end)));
                }
                values = new_values;
            }
            values
                .into_iter()
                .min_by(|a, b| a.0.cmp(&b.0))
                .map(|(start, _)| start)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 46);
    }
}
