extern crate alloc;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day6 {}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[public]
impl Solution for Day6 {
    fn solvepart1(&self, input: String) -> u32 {
        let mut guard_position = (0, 0);
        let (mut x, mut y) = (-1, -1);

        let grid: Vec<Vec<bool>> = input
            .lines()
            .map(|line| {
                x += 1;
                y = -1;
                line.chars()
                    .map(|c| {
                        y += 1;
                        if c == '^' {
                            guard_position = (x, y);
                        }
                        c == '#'
                    })
                    .collect()
            })
            .collect();

        let mut current_direction = 0;

        // Set all visited cells to false
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        visited[guard_position.0 as usize][guard_position.1 as usize] = true;

        let mut result = 1;

        'outer: loop {
            let (x, y) = guard_position;

            let mut next_position = (
                x + DIRECTIONS[current_direction].0,
                y + DIRECTIONS[current_direction].1,
            );

            // Out of bounds
            if next_position.0 < 0
                || next_position.0 >= grid.len() as i32
                || next_position.1 < 0
                || next_position.1 >= grid[0].len() as i32
            {
                break;
            }

            // Obstacles
            while grid[next_position.0 as usize][next_position.1 as usize] {
                current_direction = (current_direction + 1) % 4;
                next_position = (
                    x + DIRECTIONS[current_direction].0,
                    y + DIRECTIONS[current_direction].1,
                );

                // Out of bounds
                if next_position.0 < 0
                    || next_position.0 >= grid.len() as i32
                    || next_position.1 < 0
                    || next_position.1 >= grid[0].len() as i32
                {
                    break 'outer;
                }
            }

            guard_position = next_position;

            if !visited[next_position.0 as usize][next_position.1 as usize] {
                result += 1;
            }

            visited[next_position.0 as usize][next_position.1 as usize] = true;
        }

        result
    }

    fn solvepart2(&self, input: String) -> u32 {
        let mut guard_position = (0, 0);
        let (mut x, mut y) = (-1, -1);

        let mut grid: Vec<Vec<bool>> = input
            .lines()
            .map(|line| {
                x += 1;
                y = -1;
                line.chars()
                    .map(|c| {
                        y += 1;
                        if c == '^' {
                            guard_position = (x, y);
                        }
                        c == '#'
                    })
                    .collect()
            })
            .collect();

        let mut stuck = vec![vec![false; grid[0].len()]; grid.len()];
        stuck[guard_position.0 as usize][guard_position.1 as usize] = true;

        let start_position = guard_position;
        let mut current_direction = 0;
        let mut result = 0;

        'outer: loop {
            let (x, y) = guard_position;

            let mut next_position = (
                x + DIRECTIONS[current_direction].0,
                y + DIRECTIONS[current_direction].1,
            );

            // Out of bounds
            if next_position.0 < 0
                || next_position.0 >= grid.len() as i32
                || next_position.1 < 0
                || next_position.1 >= grid[0].len() as i32
            {
                break;
            }

            // Obstacles
            while grid[next_position.0 as usize][next_position.1 as usize] {
                current_direction = (current_direction + 1) % 4;
                next_position = (
                    x + DIRECTIONS[current_direction].0,
                    y + DIRECTIONS[current_direction].1,
                );

                // Out of bounds
                if next_position.0 < 0
                    || next_position.0 >= grid.len() as i32
                    || next_position.1 < 0
                    || next_position.1 >= grid[0].len() as i32
                {
                    break 'outer;
                }
            }
            grid[next_position.0 as usize][next_position.1 as usize] = true;

            if !stuck[next_position.0 as usize][next_position.1 as usize]
                && is_stuck(&mut grid, start_position, 0)
            {
                result += 1;
                // console!("{}", result);
                stuck[next_position.0 as usize][next_position.1 as usize] = true;
            }

            grid[next_position.0 as usize][next_position.1 as usize] = false;
            guard_position = next_position;
        }

        result
    }
}

fn is_stuck(grid: &mut Vec<Vec<bool>>, guard_position: (i32, i32), direction: usize) -> bool {
    let mut current_guard_position = guard_position;
    let mut current_direction = direction;

    let mut visited = BTreeSet::new();

    loop {
        let (x, y) = current_guard_position;
        let mut next_position = (
            x + DIRECTIONS[current_direction].0,
            y + DIRECTIONS[current_direction].1,
        );

        // Out of bounds
        if next_position.0 < 0
            || next_position.0 >= grid.len() as i32
            || next_position.1 < 0
            || next_position.1 >= grid[0].len() as i32
        {
            return false;
        }

        // Obstacles
        while grid[next_position.0 as usize][next_position.1 as usize] {
            current_direction = (current_direction + 1) % 4;
            next_position = (
                x + DIRECTIONS[current_direction].0,
                y + DIRECTIONS[current_direction].1,
            );

            // Out of bounds
            if next_position.0 < 0
                || next_position.0 >= grid.len() as i32
                || next_position.1 < 0
                || next_position.1 >= grid[0].len() as i32
            {
                return false;
            }
        }

        current_guard_position = next_position;

        if visited.contains(&(current_guard_position, current_direction)) {
            return true;
        }

        visited.insert((current_guard_position, current_direction));
    }
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    let mut value = String::from("\n");
    for row in grid {
        for cell in row {
            if *cell {
                value.push('#');
            } else {
                value.push('.');
            }
        }
        value.push('\n');
    }

    // console!("{}", value);
}

fn print_visited(visited: &Vec<Vec<i32>>) {
    let mut value = String::from("\n");
    for row in visited {
        for cell in row {
            let cell_content = cell.to_string();
            value.push_str(if *cell == 0 { "." } else { &cell_content });
            value.push_str(" ");
        }
        value.push('\n');
    }

    // console!("{}", value);
}
