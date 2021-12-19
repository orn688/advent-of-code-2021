use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use anyhow::Result;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<String> {
    shortest_distance(input)
}

pub fn part2(input: &str) -> Result<String> {
    // Great a 5x5 grid of the original grid, with each new section of grid
    // having its digits increased by the distance from the original, wrapping
    // around from 9 to 1.
    let multiplier: usize = 5;
    let mut parts = vec![];
    for y in 0..multiplier {
        for line in input.trim().lines() {
            for x in 0..multiplier {
                let new_line = line
                    .chars()
                    .map(|c| {
                        let orig = c.to_digit(10).unwrap() as usize;
                        let mut new = orig + x + y;
                        if new > 9 {
                            new = new % 10 + 1;
                        }
                        new.to_string()
                    })
                    .join("");
                parts.push(new_line);
            }
            parts.push("\n".to_string());
        }
    }
    let input = parts.join("");
    shortest_distance(&input)
}

fn shortest_distance(input: &str) -> Result<String> {
    let grid = parse_input(input);

    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();
    heap.push(HeapItem {
        distance: 0,
        index: 0,
    });

    let mut distances: HashMap<usize, u32> = HashMap::new();

    while let Some(item) = heap.pop() {
        if item.index == grid.last_index() {
            return Ok(item.distance.to_string());
        }

        for neighbor in grid.neighbors(item.index) {
            let distance = item.distance + grid.val(neighbor);
            if let Some(&min_distance) = distances.get(&neighbor) {
                if distance >= min_distance {
                    // Already found a shorter route to this neighbor, skip it.
                    continue;
                }
            }
            distances.insert(neighbor, distance);
            heap.push(HeapItem {
                distance,
                index: neighbor,
            });
        }
    }

    Err(anyhow::anyhow!("no path through the grid"))
}

#[derive(Eq, PartialEq, Debug)]
struct HeapItem {
    distance: u32,
    index: usize,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Grid {
    let lines: Vec<_> = input.trim().lines().collect();
    let nums = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .flatten()
        .collect();
    Grid {
        nums,
        width: lines[0].len() as isize,
        height: lines.len() as isize,
    }
}

struct Grid {
    nums: Vec<u32>,
    width: isize,
    height: isize,
}

impl Grid {
    fn neighbors(&self, index: usize) -> Vec<usize> {
        let x = (index as isize) % self.width;
        let y = (index as isize) / self.width;
        let mut res = vec![];
        if y > 0 {
            res.push((x, y - 1));
        }
        if x > 0 {
            res.push((x - 1, y));
        }
        if y + 1 < self.height {
            res.push((x, y + 1));
        }
        if x + 1 < self.width {
            res.push((x + 1, y));
        }
        res.iter()
            .map(|(x, y)| (y * self.width + x) as usize)
            .collect()
    }

    fn val(&self, index: usize) -> u32 {
        self.nums[index]
    }

    fn last_index(&self) -> usize {
        (self.width * self.height - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "40");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "315");
    }
}
