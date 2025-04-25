use aoc_runner_derive::aoc;

fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |mut hash, &byte| {
        hash += byte as usize;
        hash *= 17;
        hash %= 256;
        hash
    })
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    input.split(",").map(|s| hash(s)).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let map = input.split(",").fold(vec![vec![]; 256], |mut map, s| {
        let (key, value) = s.split_once(['=', '-']).unwrap();
        let values = &mut map[hash(key)];
        if value.len() == 0 {
            if let Some(pos) = values.iter().position(|&(k, _)| k == key) {
                values.remove(pos);
            }
        } else {
            if let Some(slot) = values.iter_mut().find(|(k, _)| *k == key) {
                slot.1 = value;
            } else {
                values.push((key, value));
            }
        }
        map
    });

    map.iter()
        .enumerate()
        .map(|(box_index, slots)| {
            slots
                .iter()
                .enumerate()
                .map(|(slot_index, (_, value))| {
                    (box_index + 1) * (slot_index + 1) * value.parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 145);
    }
}
