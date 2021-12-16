use anyhow::Result;

// Length of each side of the bingo board.
const BOARD_SIZE: usize = 5;

pub fn part1(input: &str) -> Result<String> {
    let mut boards = vec![];
    let numbers = parse_input(input, &mut boards);
    for num in numbers {
        for board in &mut boards {
            board.apply_num(num);
            if board.has_bingo() {
                let score = num * board.unmarked_sum();
                return Ok(score.to_string());
            }
        }
    }
    Err(anyhow::anyhow!("no board won"))
}

pub fn part2(input: &str) -> Result<String> {
    let mut boards = vec![];
    let numbers = parse_input(input, &mut boards);
    for num in numbers {
        for board in &mut boards {
            board.apply_num(num);
        }
        let last_board_sum = boards.last().unwrap().unmarked_sum();
        // Remove all boards that now have a bingo. If there are no longer any
        // boards without a bingo, return the score of the last board to get a
        // bingo.
        let new_boards: Vec<Board> = boards.into_iter().filter(|b| !b.has_bingo()).collect();
        if new_boards.is_empty() {
            return Ok((num * last_board_sum).to_string());
        }
        boards = new_boards;
    }
    Err(anyhow::anyhow!("some boards never won"))
}

fn parse_input(input: &str, boards: &mut Vec<Board>) -> Vec<i32> {
    let groups: Vec<&str> = input.trim().split("\n\n").collect();
    let numbers: Vec<i32> = groups[0]
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    for group in groups.iter().skip(1) {
        let mut grid = [0; BOARD_SIZE * BOARD_SIZE];
        for (i, num) in group
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .enumerate()
        {
            grid[i] = num;
        }
        boards.push(Board {
            grid,
            chosen: [false; BOARD_SIZE * BOARD_SIZE],
        });
    }
    numbers
}

#[derive(Debug)]
struct Board {
    grid: [i32; BOARD_SIZE * BOARD_SIZE],
    chosen: [bool; BOARD_SIZE * BOARD_SIZE],
}

impl Board {
    fn has_bingo(&self) -> bool {
        for row in 0..BOARD_SIZE {
            if (0..BOARD_SIZE)
                .map(|col| self.chosen[self.index(row, col)])
                .all(|chosen| chosen)
            {
                return true;
            }
        }
        for col in 0..BOARD_SIZE {
            if (0..BOARD_SIZE)
                .map(|row| self.chosen[self.index(row, col)])
                .all(|chosen| chosen)
            {
                return true;
            }
        }
        false
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * BOARD_SIZE + col
    }

    fn unmarked_sum(&self) -> i32 {
        self.chosen
            .into_iter()
            .enumerate()
            .filter(|(_, chosen)| !*chosen)
            .map(|(i, _)| self.grid[i])
            .sum()
    }

    fn apply_num(&mut self, choice: i32) {
        for (i, num) in self.grid.into_iter().enumerate() {
            if num == choice {
                self.chosen[i] = true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "4512");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "1924");
    }
}
