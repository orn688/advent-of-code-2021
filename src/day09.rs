use std::collections::{BinaryHeap, HashSet};

use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let grid = parse_input(input);
    let mut sum = 0;
    for (row, vals) in grid.grid.iter().enumerate() {
        for (col, val) in vals.iter().enumerate() {
            let neighbors = grid.neighbor_vals(row, col);
            if !neighbors.is_empty() && neighbors.iter().all(|n| n > val) {
                sum += val + 1;
            }
        }
    }

    Ok(sum.to_string())
}

/// Uses DFS to explore each "basin" (enclosed group of numbers < 9) and returns
/// the product of the sizes of the three largest basins.
pub fn part2(input: &str) -> Result<String> {
    let grid = parse_input(input);
    let mut visited = HashSet::new();
    let mut basins = BinaryHeap::new();
    for start_row in 0..grid.height() {
        for start_col in 0..grid.width() {
            let mut basin_size = 0;
            let mut stack = vec![(start_row, start_col)];
            while let Some((row, col)) = stack.pop() {
                if visited.contains(&(row, col)) || grid.val(row, col) == 9 {
                    continue;
                }
                visited.insert((row, col));
                basin_size += 1;

                for coord in grid.neighbors(row, col) {
                    stack.push(coord);
                }
            }
            if basin_size > 0 {
                basins.push(basin_size);
            }
        }
    }
    let mut res = 1;
    for _ in 0..3 {
        res *= basins.pop().unwrap();
    }
    Ok(res.to_string())
}

fn parse_input(input: &str) -> Grid {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    Grid { grid }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn val(&self, row: usize, col: usize) -> i32 {
        *self.grid.get(row).unwrap().get(col).unwrap()
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        if row > 0 {
            v.push((row - 1, col));
        }
        if col > 0 {
            v.push((row, col - 1));
        }
        if row + 1 < self.height() {
            v.push((row + 1, col));
        }
        if col + 1 < self.width() {
            v.push((row, col + 1));
        }
        v
    }

    fn neighbor_vals(&self, row: usize, col: usize) -> Vec<i32> {
        self.neighbors(row, col)
            .iter()
            .map(|(r, c)| self.val(*r, *c))
            .collect()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "15");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "1134");
    }
}
