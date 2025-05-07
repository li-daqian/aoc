use std::collections::HashMap;

use aoc_runner_derive::aoc;

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dfs(
    graph: &[Vec<(usize, usize)>],
    seen: &mut [bool],
    goal: usize,
    curr: usize,
) -> Option<usize> {
    if curr == goal {
        return Some(0);
    }
    let mut max_dist = None;
    for &(next, d) in &graph[curr] {
        if !seen[next] {
            seen[next] = true;
            if let Some(dist) = dfs(graph, seen, goal, next) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[next] = false;
        }
    }
    max_dist
}

fn solve(grid: &[&[u8]], part2: bool) -> usize {
    let mut graph = HashMap::<_, Vec<_>>::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let neighbors = match grid[r][c] {
                b'#' => continue,
                _ if part2 => DIRECTIONS.to_vec(),
                b'.' => DIRECTIONS.to_vec(),
                b'^' => vec![DIRECTIONS[NORTH]],
                b'>' => vec![DIRECTIONS[EAST]],
                b'v' => vec![DIRECTIONS[SOUTH]],
                b'<' => vec![DIRECTIONS[WEST]],
                _ => unreachable!(),
            };
            let e = graph.entry((r, c)).or_default();
            for (dr, dc) in neighbors {
                let rr = (r as isize + dr) as usize;
                let cc = (c as isize + dc) as usize;
                if grid
                    .get(rr)
                    .and_then(|row| row.get(cc))
                    .is_some_and(|&t| t != b'#')
                {
                    e.push((rr, cc, 1));
                }
            }
        }
    }

    // edge contraction (i.e contract maze corridors)
    let corridors = graph
        .iter()
        .filter(|(_, n)| n.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();
    for (r, c) in corridors {
        let neighbors = graph.remove(&(r, c)).unwrap();
        let (r1, c1, d1) = neighbors[0];
        let (r2, c2, d2) = neighbors[1];
        let n1 = graph.get_mut(&(r1, c1)).unwrap();
        if let Some(i) = n1.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n1[i] = (r2, c2, d1 + d2);
        }
        let n2 = graph.get_mut(&(r2, c2)).unwrap();
        if let Some(i) = n2.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n2[i] = (r1, c1, d1 + d2);
        }
    }

    // convert: (r,c) hashmap -> index vector
    let indexes = graph
        .keys()
        .enumerate()
        .map(|(i, pos)| (pos, i))
        .collect::<HashMap<_, _>>();
    let mut idx_graph = vec![Vec::new(); graph.len()];
    for (pos, neighbors) in &graph {
        idx_graph[indexes[pos]] = neighbors
            .iter()
            .map(|&(r, c, d)| (indexes[&(r, c)], d))
            .collect();
    }

    let goal = indexes[&(grid.len() - 1, grid[0].len() - 2)];
    dfs(
        &idx_graph,
        &mut vec![false; idx_graph.len()],
        goal,
        indexes[&(0, 1)],
    )
    .unwrap()
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    solve(&grid, false)
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    solve(&grid, true)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {r"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 154);
    }
}
