extern crate alloc;
use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashSet;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day10 {}

#[public]
impl Solution for Day10 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let mut start_locations: Vec<(usize, usize)> = Vec::new();

        let grid: Vec<Vec<u8>> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let char = c.to_digit(10).unwrap().try_into().unwrap();

                        if char == 9 {
                            start_locations.push((i, j));
                        }

                        char
                    })
                    .collect()
            })
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        // Possible movement directions: up, right, down, left
        let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        // For each 9 starting point, do a DFS
        for (start_row, start_col) in start_locations {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back((start_row, start_col));
            visited.insert((start_row, start_col));

            while let Some((row, col)) = queue.pop_front() {
                // Check all 4 directions
                for &(dr, dc) in &directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    // Check bounds
                    if new_row >= 0
                        && new_row < rows as i32
                        && new_col >= 0
                        && new_col < cols as i32
                    {
                        let new_row = new_row as usize;
                        let new_col = new_col as usize;

                        // Check if we've already visited this cell
                        if visited.contains(&(new_row, new_col)) {
                            continue;
                        }

                        // Can only move to a cell that is exactly 1 less
                        if grid[new_row][new_col] == grid[row][col] - 1 {
                            // If we've reached 0, store its location
                            if grid[new_row][new_col] == 0 {
                                result += 1;
                            }

                            // Mark as visited and continue search
                            visited.insert((new_row, new_col));
                            queue.push_back((new_row, new_col));
                        }
                    }
                }
            }
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let mut start_locations: Vec<(usize, usize)> = Vec::new();

        let grid: Vec<Vec<u8>> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let char = c.to_digit(10).unwrap().try_into().unwrap();

                        if char == 9 {
                            start_locations.push((i, j));
                        }

                        char
                    })
                    .collect()
            })
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        // Possible movement directions: up, right, down, left
        let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        // For each 9 starting point, do a DFS
        for (start_row, start_col) in start_locations {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back((start_row, start_col));
            visited.insert((start_row, start_col));

            while let Some((row, col)) = queue.pop_front() {
                // Check all 4 directions
                for &(dr, dc) in &directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    // Check bounds
                    if new_row >= 0
                        && new_row < rows as i32
                        && new_col >= 0
                        && new_col < cols as i32
                    {
                        let new_row = new_row as usize;
                        let new_col = new_col as usize;

                        // Can only move to a cell that is exactly 1 less
                        if grid[new_row][new_col] == grid[row][col] - 1 {
                            // If we've reached 0, store its location
                            if grid[new_row][new_col] == 0 {
                                result += 1;
                            }

                            // Mark as visited and continue search
                            visited.insert((new_row, new_col));
                            queue.push_back((new_row, new_col));
                        }
                    }
                }
            }
        }

        result.try_into().unwrap()
    }
}
