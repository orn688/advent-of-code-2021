use std::collections::{HashMap, HashSet};

use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let graph = parse_input(input);
    let count = count_paths(&graph, "start", "end", &mut HashSet::new());
    Ok(count.to_string())
}

fn count_paths(graph: &Graph, curr: &str, target: &str, visited: &mut HashSet<String>) -> usize {
    if curr.eq(target) {
        return 1;
    }
    if visited.contains(curr) && !can_revisit(curr) {
        return 0;
    }

    let newly_visited = visited.insert(curr.to_string());

    let count = graph
        .neighbors(curr)
        .iter()
        .map(|n| count_paths(graph, n, target, visited))
        .sum();

    if newly_visited {
        visited.remove(curr);
    }

    count
}

pub fn part2(_: &str) -> Result<String> {
    Ok(String::new())
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

/// A node can be revisited if its name is all-caps. Likewise, it cannot be
/// revisited if its name is all lowercase, and each node can only be on or the
/// other, so it's sufficient to check if it has *any* uppercase letter.
fn can_revisit(node: &str) -> bool {
    node.chars().any(|c| c.is_uppercase())
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

#[allow(dead_code)]
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
    assert_eq!(part2(TEST_INPUT).unwrap(), "");
}
