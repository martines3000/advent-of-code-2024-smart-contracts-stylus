extern crate alloc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day13 {}

#[public]
impl Solution for Day13 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let (mut a, mut b, mut p) = ((0, 0), (0, 0), (0, 0));
        let mut idx = 0;

        input.lines().enumerate().for_each(|(i, line)| {
            idx = i % 4;
            let data = line.split_whitespace().collect::<Vec<&str>>();

            let (x, y) = if idx != 3 {
                (
                    data[data.len() - 2][2..data[data.len() - 2].len() - 1]
                        .parse::<u64>()
                        .unwrap(),
                    data[data.len() - 1][2..].parse::<u64>().unwrap(),
                )
            } else {
                (0, 0)
            };

            match idx {
                0 => {
                    a = (x as i64, y as i64);
                }
                1 => {
                    b = (x as i64, y as i64);
                }
                2 => {
                    p = (x as i64, y as i64);
                }
                _ => {
                    result += solve_2(a, b, p).unwrap_or(0);
                }
            }
        });

        if idx != 3 {
            result += solve_2(a, b, p).unwrap_or(0);
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let (mut a, mut b, mut p) = ((0, 0), (0, 0), (0, 0));
        let mut idx = 0;

        input.lines().enumerate().for_each(|(i, line)| {
            idx = i % 4;
            let data = line.split_whitespace().collect::<Vec<&str>>();

            let (x, y) = if idx != 3 {
                (
                    data[data.len() - 2][2..data[data.len() - 2].len() - 1]
                        .parse::<u64>()
                        .unwrap(),
                    data[data.len() - 1][2..].parse::<u64>().unwrap(),
                )
            } else {
                (0, 0)
            };

            match idx {
                0 => {
                    a = (x as i64, y as i64);
                }
                1 => {
                    b = (x as i64, y as i64);
                }
                2 => {
                    p = ((x + 10000000000000) as i64, (y + 10000000000000) as i64);
                }
                _ => {
                    result += solve_2(a, b, p).unwrap_or(0);
                }
            }
        });

        if idx != 3 {
            result += solve_2(a, b, p).unwrap_or(0);
        }

        result.try_into().unwrap()
    }
}

// Out of gas, but solve_2 is better anyway.
// fn solve_1(a: (u64, u64), b: (u64, u64), p: (u64, u64)) -> Option<u64> {
//     let mut grid = vec![vec![((0, 0), 0); 100]; 100];

//     let mut smallest: Option<u64> = None;

//     for y in 0..100_u64 {
//         for x in 0..100_u64 {
//             grid[y as usize][x as usize] = ((y * a.0 + x * b.0, y * a.1 + x * b.1), 3 * y + x);

//             if grid[y as usize][x as usize].0 == p
//                 && (smallest.is_none() || grid[y as usize][x as usize].1 < smallest.unwrap())
//             {
//                 smallest = Some(grid[y as usize][x as usize].1);
//             }
//         }
//     }

//     return smallest;
// }

// Linear algebra
// Px = a_click * Ax + b_click * Bx
// Py = a_click * Ay + b_click * By
fn solve_2(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> Option<u64> {
    let b_click = (p.0 * a.1 - p.1 * a.0) / (b.0 * a.1 - b.1 * a.0);
    let a_click = (p.0 - b_click * b.0) / a.0;

    let res = (a_click * a.0 + b_click * b.0, a_click * a.1 + b_click * b.1);

    // Check click number is whole positive number and result is correct
    if a_click < 0 || b_click < 0 || res != p {
        return None;
    }

    return Some(3 * a_click as u64 + b_click as u64);
}
