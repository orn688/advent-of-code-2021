use std::collections::HashSet;

use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let (points, folds) = parse_input(input)?;
    let fold = folds.get(0).unwrap();

    let folded_points: HashSet<Point> =
        HashSet::from_iter(points.into_iter().map(|point| match *fold {
            Fold::Vertical(i) => Point {
                x: if point.x > i {
                    i - (point.x - i)
                } else {
                    point.x
                },
                y: point.y,
            },
            Fold::Horizontal(i) => Point {
                x: point.x,
                y: if point.y > i {
                    i - (point.y - i)
                } else {
                    point.y
                },
            },
        }));

    Ok(folded_points.len().to_string())
}

pub fn part2(_: &str) -> Result<String> {
    Ok(String::new())
}

enum Fold {
    Vertical(u32),
    Horizontal(u32),
}

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

fn parse_input(input: &str) -> Result<(Vec<Point>, Vec<Fold>)> {
    let parts: Vec<&str> = input.trim().splitn(2, "\n\n").collect();

    let points = parts[0]
        .lines()
        .map_while(|line| {
            if line.is_empty() {
                return None;
            }
            let parts: Vec<u32> = line.splitn(2, ',').map(|s| s.parse().unwrap()).collect();
            Some(Point {
                x: parts[0],
                y: parts[1],
            })
        })
        .collect();

    let folds = parts[1]
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split_whitespace()
                .last()
                .unwrap()
                .splitn(2, '=')
                .collect();
            let idx: u32 = parts[1].parse().unwrap();
            match parts[0] {
                "x" => Fold::Vertical(idx),
                "y" => Fold::Horizontal(idx),
                _ => panic!("invalid fold line"),
            }
        })
        .collect();

    Ok((points, folds))
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "17");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "");
}
