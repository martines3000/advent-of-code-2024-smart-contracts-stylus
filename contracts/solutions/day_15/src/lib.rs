extern crate alloc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day15 {}

fn box_move(grid: &mut Vec<char>, x: i32, i: i32, dir: i32) -> bool {
    let mut swaps: Vec<i32> = Vec::with_capacity(32);

    if recurse(grid, &mut swaps, x, dir) {
        for &swap in swaps.iter() {
            grid.swap(swap as usize, (swap - dir) as usize);
            grid.swap((swap + 1) as usize, (swap + 1 - dir) as usize);
        }

        grid.swap(i as usize, (i - dir) as usize);

        return true;
    }

    false
}

fn recurse(grid: &mut Vec<char>, swaps: &mut Vec<i32>, x: i32, dir: i32) -> bool {
    let x = x + dir;

    if swaps.contains(&x) {
        return true;
    }

    if match (grid[x as usize], grid[(x + 1) as usize]) {
        ('.', '.') => true,
        ('#', _) | (_, '#') => false,
        ('[', _) => recurse(grid, swaps, x, dir),
        ('.', '[') => recurse(grid, swaps, x + 1, dir),
        (']', '.') => recurse(grid, swaps, x - 1, dir),
        (']', '[') => recurse(grid, swaps, x - 1, dir) && recurse(grid, swaps, x + 1, dir),
        _ => false,
    } {
        swaps.push(x);
        return true;
    }

    false
}

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
        let input = input.split_terminator("\n\n").collect::<Vec<_>>();
        let mut result: u64 = 0;

        let mut grid: Vec<char> = input[0]
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => "##",
                        '.' => "..",
                        '@' => "@.",
                        'O' => "[]",
                        _ => "",
                    })
                    .flat_map(|s| s.chars())
            })
            .collect();

        let mut robot = grid.iter().position(|&c| c == '@').unwrap() as i32;
        let width = 2 * input[0].lines().next().unwrap().len() as i32;

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

            if dir.abs() == 1 {
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
                        '[' | ']' => x += dir * 2,
                        '#' => break,
                        _ => break 'outer,
                    }
                }
            } else {
                if match grid[x as usize] {
                    '.' => {
                        grid.swap(x as usize, robot as usize);
                        true
                    }
                    '[' => box_move(&mut grid, x, x, dir),
                    ']' => box_move(&mut grid, x - 1, x, dir),
                    '#' => false,
                    _ => false,
                } {
                    robot = x;
                }
            }
        }

        for (i, c) in grid.iter().enumerate() {
            if *c == '[' {
                result += (100 * (i as i32 / width) + i as i32 % width) as u64;
            }
        }

        result.try_into().unwrap()
    }
}
