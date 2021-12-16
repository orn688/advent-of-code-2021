use std::collections::{HashMap, HashSet};

use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let graph = parse_input(input);
    let count = count_paths_part1(&graph, "start", "end", &mut HashSet::new());
    Ok(count.to_string())
}

fn count_paths_part1(
    graph: &Graph,
    curr: &str,
    target: &str,
    visited: &mut HashSet<String>,
) -> usize {
    if curr.eq(target) {
        return 1;
    }
    if visited.contains(curr) && !can_revisit_part1(curr) {
        return 0;
    }

    let newly_visited = visited.insert(curr.to_string());

    let count = graph
        .neighbors(curr)
        .iter()
        .map(|n| count_paths_part1(graph, n, target, visited))
        .sum();

    if newly_visited {
        visited.remove(curr);
    }

    count
}

/// A node can be revisited if its name is all-caps.
fn can_revisit_part1(node: &str) -> bool {
    is_uppercase(node)
}

pub fn part2(input: &str) -> Result<String> {
    let graph = parse_input(input);
    let count = count_paths_part2(&graph, "start", "end", &mut HashSet::new(), false);
    Ok(count.to_string())
}

fn count_paths_part2(
    graph: &Graph,
    curr: &str,
    target: &str,
    visited: &mut HashSet<String>,
    // Whether one lowercase node has already been revisited.
    mut revisited_one: bool,
) -> usize {
    if curr.eq(target) {
        return 1;
    }
    if visited.contains(curr) {
        if !can_revisit_part2(curr, revisited_one) {
            return 0;
        } else if !is_uppercase(curr) {
            revisited_one = true;
        }
    }

    let newly_visited = visited.insert(curr.to_string());

    let count = graph
        .neighbors(curr)
        .iter()
        .map(|n| count_paths_part2(graph, n, target, visited, revisited_one))
        .sum();

    if newly_visited {
        visited.remove(curr);
    }

    count
}

/// A node can be revisited if its name is all-caps. Likewise, it cannot be
/// revisited if its name is all lowercase, and each node can only be on or the
/// other, so it's sufficient to check if it has *any* uppercase letter.
fn can_revisit_part2(node: &str, revisited_one: bool) -> bool {
    if node == "start" || node == "end" {
        return false;
    }
    !revisited_one || is_uppercase(node)
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph {
        edges: HashMap::new(),
    };
    for line in input.trim().lines() {
        let ends: Vec<_> = line.split('-').collect();
        graph.add_edge(ends[0].to_string(), ends[1].to_string());
    }
    graph
}

struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn add_edge(&mut self, a: String, b: String) {
        for (src, target) in [(&a, &b), (&b, &a)] {
            let targets = self.edges.entry(src.clone()).or_default();
            targets.insert(target.to_string());
        }
    }

    fn neighbors(&self, src: &str) -> &HashSet<String> {
        self.edges.get(&*src).expect("no such key")
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "226");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "3509");
    }
}
