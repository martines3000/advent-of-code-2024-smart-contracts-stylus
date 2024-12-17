extern crate alloc;
use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day12 {}

#[public]
impl Solution for Day12 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let grid = input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        // BFS
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        let mut area;
        let mut perimeter;
        let neighborhood = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited[i][j] {
                    area = 0;
                    perimeter = 0;
                    queue.push_back((i, j));

                    while let Some((y, x)) = queue.pop_front() {
                        area += 1;
                        visited[y][x] = true;

                        // Check neighbors
                        for (dy, dx) in &neighborhood {
                            let neighbor = (y as i32 + dy, x as i32 + dx);

                            if neighbor.0 < 0
                                || neighbor.1 < 0
                                || neighbor.0 >= grid.len() as i32
                                || neighbor.1 >= grid[0].len() as i32
                                || grid[neighbor.0 as usize][neighbor.1 as usize] != grid[y][x]
                            {
                                perimeter += 1;
                                continue;
                            }

                            if grid[neighbor.0 as usize][neighbor.1 as usize] == grid[y][x]
                                && !visited[neighbor.0 as usize][neighbor.1 as usize]
                            {
                                queue.push_back((neighbor.0 as usize, neighbor.1 as usize));
                                visited[neighbor.0 as usize][neighbor.1 as usize] = true;
                            }
                        }
                    }

                    result += area * perimeter;
                }
            }
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let grid = input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        // BFS
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        let mut area;
        let mut perimeter;
        let neighborhood = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited[i][j] {
                    area = 0;
                    perimeter = 0;
                    queue.push_back((i, j));

                    while let Some((y, x)) = queue.pop_front() {
                        area += 1;
                        visited[y][x] = true;

                        // Check neighbors
                        for (dy, dx) in &neighborhood {
                            let (ny, nx) = (y as i32 + dy, x as i32 + dx);
                            let (py, px) = (y as i32 - dx, x as i32 + dy);
                            let (qy, qx) = (y as i32 + dy - dx, x as i32 + dx + dy);

                            let n_val = if ny < 0
                                || nx < 0
                                || ny >= grid.len() as i32
                                || nx >= grid[0].len() as i32
                            {
                                '_'
                            } else {
                                grid[ny as usize][nx as usize]
                            };

                            let p_val = if py < 0
                                || px < 0
                                || py >= grid.len() as i32
                                || px >= grid[0].len() as i32
                            {
                                '_'
                            } else {
                                grid[py as usize][px as usize]
                            };

                            let q_val = if qy < 0
                                || qx < 0
                                || qy >= grid.len() as i32
                                || qx >= grid[0].len() as i32
                            {
                                '_'
                            } else {
                                grid[qy as usize][qx as usize]
                            };

                            if n_val != grid[y][x] {
                                if p_val != grid[y][x] {
                                    perimeter += 1;
                                }
                                continue;
                            }

                            if !visited[ny as usize][nx as usize] {
                                queue.push_back((ny as usize, nx as usize));
                                visited[ny as usize][nx as usize] = true;
                            }

                            if p_val == grid[y][x] && q_val != grid[y][x] {
                                perimeter += 1;
                            }
                        }
                    }

                    result += area * perimeter;
                }
            }
        }

        result.try_into().unwrap()
    }
}
