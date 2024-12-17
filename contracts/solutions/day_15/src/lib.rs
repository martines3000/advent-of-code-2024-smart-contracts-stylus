extern crate alloc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day15 {}

#[public]
impl Solution for Day15 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let input = input.split_terminator("\n\n").collect::<Vec<_>>();

        let mut grid: Vec<char> = input[0].lines().flat_map(|line| line.chars()).collect();
        let mut robot = grid.iter().position(|&c| c == '@').unwrap() as i32;
        let width = input[0].lines().next().unwrap().len() as i32;

        let moves: Vec<i32> = input[1]
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '>' => 1,
                    '<' => -1,
                    'v' => width,
                    '^' => -width,
                    _ => 0,
                })
            })
            .collect();

        for dir in moves {
            let mut x = robot + dir;

            'outer: loop {
                match grid[x as usize] {
                    '.' => loop {
                        grid[x as usize] = grid[(x - dir) as usize];

                        if grid[x as usize] == '@' {
                            grid[(x - dir) as usize] = '.';
                            robot = x;
                            break 'outer;
                        }

                        x -= dir;
                    },
                    'O' => x += dir,
                    '#' => break,
                    _ => break 'outer,
                }
            }
        }

        for (i, c) in grid.iter().enumerate() {
            if *c == 'O' {
                result += (100 * (i as i32 / width) + i as i32 % width) as u64;
            }
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        // TODO:

        result.try_into().unwrap()
    }
}
