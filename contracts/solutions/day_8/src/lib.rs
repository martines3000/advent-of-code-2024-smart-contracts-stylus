extern crate alloc;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day8 {}

#[public]
impl Solution for Day8 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let mut map = BTreeMap::new();
        let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
        let mut occupied: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '.' {
                    if !map.contains_key(&grid[i][j]) {
                        map.insert(grid[i][j], Vec::from([(i, j)]));
                    } else {
                        map.get_mut(&grid[i][j]).unwrap().push((i, j));
                    }
                }
            }
        }

        let mut diff;
        let mut option;

        for value in map.values() {
            for i in 0..value.len() {
                for j in 0..value.len() {
                    if i == j {
                        continue;
                    }

                    diff = (
                        value[j].0 as i32 - value[i].0 as i32,
                        value[j].1 as i32 - value[i].1 as i32,
                    );

                    option = (value[j].0 as i32 + diff.0, value[j].1 as i32 + diff.1);

                    if option.0 >= 0
                        && option.1 >= 0
                        && option.0 < grid.len() as i32
                        && option.1 < grid[0].len() as i32
                        && !occupied[option.0 as usize][option.1 as usize]
                    {
                        occupied[option.0 as usize][option.1 as usize] = true;
                        result += 1;
                    }
                }
            }
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let mut map = BTreeMap::new();
        let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
        let mut occupied: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '.' {
                    // occupied[i][j] = true;
                    if !map.contains_key(&grid[i][j]) {
                        map.insert(grid[i][j], Vec::from([(i, j)]));
                    } else {
                        map.get_mut(&grid[i][j]).unwrap().push((i, j));
                    }
                }
            }
        }

        let mut diff;
        let mut option;

        for value in map.values() {
            for i in 0..value.len() {
                for j in 0..value.len() {
                    if i == j {
                        continue;
                    }

                    if !occupied[value[j].0 as usize][value[j].1 as usize] {
                        result += 1;
                        occupied[value[j].0 as usize][value[j].1 as usize] = true;
                    }

                    diff = (
                        value[j].0 as i32 - value[i].0 as i32,
                        value[j].1 as i32 - value[i].1 as i32,
                    );

                    option = (value[j].0 as i32 + diff.0, value[j].1 as i32 + diff.1);

                    while option.0 >= 0
                        && option.1 >= 0
                        && option.0 < grid.len() as i32
                        && option.1 < grid[0].len() as i32
                    {
                        if !occupied[option.0 as usize][option.1 as usize] {
                            result += 1;
                            occupied[option.0 as usize][option.1 as usize] = true;
                        }

                        option = (option.0 + diff.0, option.1 + diff.1);
                    }
                }
            }
        }

        result.try_into().unwrap()
    }
}
