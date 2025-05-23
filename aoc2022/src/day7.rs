use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Node {
    File(usize),
    Dir(HashMap<String, Node>),
}

impl Node {
    fn get_dir_mut<'a>(&'a mut self, path: &[String]) -> Option<&'a mut Node> {
        let mut node = self;
        for p in path {
            match node {
                Node::Dir(map) => node = map.get_mut(p)?,
                Node::File(_) => return None,
            }
        }
        Some(node)
    }

    fn collect_dir_sizes(&self, sizes: &mut Vec<usize>) -> usize {
        match self {
            Node::File(size) => *size,
            Node::Dir(map) => {
                let sum = map.values().map(|n| n.collect_dir_sizes(sizes)).sum();
                sizes.push(sum);
                sum
            }
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut root = Node::Dir(HashMap::new());
    let mut cwd: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let arg = &line[5..];
            match arg {
                "/" => cwd.clear(),
                ".." => {
                    cwd.pop();
                }
                dir => cwd.push(dir.to_string()),
            }
        } else if line.starts_with("$ ls") {
            // ignore
        } else if line.starts_with("dir ") {
            let dir_name = &line[4..];
            let dir = root.get_dir_mut(&cwd).unwrap();
            if let Node::Dir(map) = dir {
                map.entry(dir_name.to_string())
                    .or_insert(Node::Dir(HashMap::new()));
            }
        } else if !line.is_empty() {
            let (size, name) = line.split_once(' ').unwrap();
            let size: usize = size.parse().unwrap();
            let dir = root.get_dir_mut(&cwd).unwrap();
            if let Node::Dir(map) = dir {
                map.entry(name.to_string()).or_insert(Node::File(size));
            }
        }
    }

    let mut sizes = Vec::new();
    root.collect_dir_sizes(&mut sizes);
    sizes.into_iter().filter(|&s| s <= 100_000).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let mut root = Node::Dir(HashMap::new());
    let mut cwd: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let arg = &line[5..];
            match arg {
                "/" => cwd.clear(),
                ".." => {
                    cwd.pop();
                }
                dir => cwd.push(dir.to_string()),
            }
        } else if line.starts_with("$ ls") {
            // ignore
        } else if line.starts_with("dir ") {
            let dir_name = &line[4..];
            let dir = root.get_dir_mut(&cwd).unwrap();
            if let Node::Dir(map) = dir {
                map.entry(dir_name.to_string())
                    .or_insert(Node::Dir(HashMap::new()));
            }
        } else if !line.is_empty() {
            let (size, name) = line.split_once(' ').unwrap();
            let size: usize = size.parse().unwrap();
            let dir = root.get_dir_mut(&cwd).unwrap();
            if let Node::Dir(map) = dir {
                map.entry(name.to_string()).or_insert(Node::File(size));
            }
        }
    }

    let mut sizes = Vec::new();
    let used = root.collect_dir_sizes(&mut sizes);
    let total = 70_000_000;
    let needed = 30_000_000;
    let unused = total - used;
    let must_free = needed - unused;

    sizes.into_iter().filter(|&s| s >= must_free).min().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 24933642);
    }
}
