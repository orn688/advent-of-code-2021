use std::collections::HashMap;

use anyhow::Result;
use regex::{Captures, Regex};

/// Counts the number of points at which two lines intersect, only counting
/// horizontal and vertical lines.
pub fn part1(input: &str) -> Result<String> {
    let lines = parse_input(input)
        .into_iter()
        .filter(|line| line.horizontal() || line.vertical())
        .collect();
    let overlaps = count_overlaps(lines);
    Ok(overlaps.to_string())
}

/// Counts the number of points at which two lines intersect, including diagonal
/// lines.
pub fn part2(input: &str) -> Result<String> {
    let lines = parse_input(input);
    let overlaps = count_overlaps(lines);
    Ok(overlaps.to_string())
}

fn count_overlaps(lines: Vec<Line>) -> usize {
    let mut counts = HashMap::new();
    for line in lines {
        for pt in line.points() {
            let count = counts.entry(pt).or_insert(0);
            *count += 1;
        }
    }
    counts.into_values().filter(|v| *v > 1).count()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        let (mut x, mut y) = (self.start.x, self.start.y);
        let x_diff = norm_diff(self.start.x, self.end.x);
        let y_diff = norm_diff(self.start.y, self.end.y);
        while x != self.end.x || y != self.end.y {
            points.push(Point { x, y });
            x += x_diff;
            y += y_diff;
        }
        points.push(self.end.clone());
        points
    }
}

fn norm_diff(start: i32, end: i32) -> i32 {
    let diff = end - start;
    if diff == 0 {
        0
    } else {
        diff / diff.abs()
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Line {
                start: Point {
                    x: int_from_captures(&caps, 1),
                    y: int_from_captures(&caps, 2),
                },
                end: Point {
                    x: int_from_captures(&caps, 3),
                    y: int_from_captures(&caps, 4),
                },
            }
        })
        .collect()
}

fn int_from_captures(caps: &Captures, group: usize) -> i32 {
    caps.get(group).unwrap().as_str().parse().unwrap()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "5");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "12");
}
