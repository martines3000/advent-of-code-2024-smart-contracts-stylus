extern crate alloc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::{console, prelude::*};

#[storage]
#[entrypoint]
pub struct Day14 {}

#[public]
impl Solution for Day14 {
    fn solvepart1(&self, input: String) -> i64 {
        let result: u64;

        let size = (101_i64, 103_i64);
        let mut grid = vec![vec![0; size.0 as usize]; size.1 as usize];

        let data = input.lines().map(|line| {
            line.split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| {
                    let data = x[2..]
                        .split(',')
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();

                    (data[0], data[1])
                })
                .collect::<Vec<(i64, i64)>>()
        });

        let mut p: (i64, i64);
        let mut v: (i64, i64);
        let mut x: i64;
        let mut y: i64;
        let mut quadrants = vec![0; 4];

        for entry in data {
            p = entry[0];
            v = entry[1];

            x = (p.0 + v.0 * 100) % size.0;
            y = (p.1 + v.1 * 100) % size.1;

            if x < 0 {
                x += size.0;
            }

            if y < 0 {
                y += size.1;
            }

            grid[y as usize][x as usize] += 1;

            let mid = (size.0 / 2, size.1 / 2);

            if x < mid.0 && y < mid.1 {
                quadrants[0] += 1;
            } else if x < mid.0 && y > mid.1 {
                quadrants[1] += 1;
            } else if x > mid.0 && y < mid.1 {
                quadrants[2] += 1;
            } else if x > mid.0 && y > mid.1 {
                quadrants[3] += 1;
            }
        }

        result = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let result: u64;

        let size = (101_i64, 103_i64);

        let data = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| {
                        let data = x[2..]
                            .split(',')
                            .map(|x| x.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();

                        (data[0], data[1])
                    })
                    .collect::<Vec<(i64, i64)>>()
            })
            .collect::<Vec<Vec<(i64, i64)>>>();

        let mut p: (i64, i64);
        let mut v: (i64, i64);
        let mut x: i64;
        let mut y: i64;

        // Starting with 3k, but not sure if this will work for all.
        // Without this we hit out of gas.
        let mut i = 3000;
        let mut count;

        let mut grid = vec![vec![false; size.0 as usize]; size.1 as usize];

        loop {
            // Reset grid
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = false;
                }
            }

            count = 0;
            for entry in &data {
                p = entry[0];
                v = entry[1];

                x = (p.0 + v.0 * i) % size.0;
                y = (p.1 + v.1 * i) % size.1;

                if x < 0 {
                    x += size.0;
                }

                if y < 0 {
                    y += size.1;
                }

                if !grid[y as usize][x as usize] {
                    count += 1;
                }

                grid[y as usize][x as usize] = true;
            }

            if count == data.len() {
                result = i as u64;
                break;
            }

            i += 1;
        }

        result.try_into().unwrap()
    }
}
