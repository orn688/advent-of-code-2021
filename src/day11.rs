use std::collections::HashSet;

use anyhow::Result;

const FLASH_VALUE: u32 = 10;

/// Computes the total number of flashes (times a square exceeds 9) in the first
/// 100 steps.
pub fn part1(input: &str) -> Result<String> {
    let mut grid = parse_input(input);
    let flashes: usize = (0..100).map(|_| grid.step()).sum();
    Ok(flashes.to_string())
}

/// Computes the number of the first step on which all the squares will flash.
pub fn part2(input: &str) -> Result<String> {
    let mut grid = parse_input(input);
    let mut step_number = 0;
    loop {
        step_number += 1;
        let flashes = grid.step();
        if flashes == grid.nums.len() {
            return Ok(step_number.to_string());
        }
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
        let center_x = (index as isize) % self.width;
        let center_y = (index as isize) / self.width;
        let mut res = vec![];
        for x_diff in -1..=1 {
            for y_diff in -1..=1 {
                if x_diff == 0 && y_diff == 0 {
                    continue;
                }
                let x = center_x + x_diff;
                let y = center_y + y_diff;
                if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
                    res.push((y * self.width + x) as usize);
                }
            }
        }
        res
    }

    fn step(&mut self) -> usize {
        for num in self.nums.iter_mut() {
            *num += 1;
        }
        let mut flashed = HashSet::new();
        loop {
            let mut updated = false;
            for i in 0..self.nums.len() {
                if self.nums[i] < FLASH_VALUE {
                    continue;
                }
                updated = true;
                flashed.insert(i);
                self.nums[i] = 0;
                for ni in self.neighbors(i) {
                    if !flashed.contains(&ni) {
                        self.nums[ni] += 1;
                    }
                }
            }
            if !updated {
                break;
            }
        }
        flashed.len()
    }

    // Useful for debugging.
    #[allow(dead_code)]
    fn print(&self) {
        for l in self.nums.chunks(self.width as usize) {
            println!(
                "{}",
                l.to_vec().iter().map(|d| d.to_string()).collect::<String>()
            );
        }
        println!();
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "1656");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "195");
}
