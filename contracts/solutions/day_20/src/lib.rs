extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ndarray::{s, Array2};
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day20 {}

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

fn find_start(data: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (y, row) in data.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Some((y, x));
            }
        }
    }
    None
}

fn traverse_maze(data: &Vec<Vec<char>>, start: (usize, usize)) -> Array2<i32> {
    let h = data.len();
    let w = data[0].len();
    let mut p = Array2::<i32>::from_elem((h, w), -1);

    let (mut y, mut x) = start;
    let mut c = 0;

    while data[y][x] != 'E' {
        p[[y, x]] = c;
        c += 1;

        for &(dx, dy) in NEIGHBOURS.iter() {
            let new_y = (y as i32 + dy) as usize;
            let new_x = (x as i32 + dx) as usize;

            if new_y < h && new_x < w && data[new_y][new_x] != '#' && p[[new_y, new_x]] == -1 {
                y = new_y;
                x = new_x;
                break;
            }
        }
    }
    p[[y, x]] = c;
    p
}

fn generate_flower(n: usize) -> Vec<(i32, i32)> {
    let mut flower = vec![vec![(1, 0), (0, 1), (0, -1), (-1, 0)]];

    for i in 2..=n {
        let last = flower.last().unwrap();
        let mid = last.len() / 2;

        let mut next = Vec::new();

        // First half shifted right
        for j in 0..=mid {
            let (x, y) = last[j];
            next.push((x + 1, y));
        }

        // Vertical elements
        next.push((0, i as i32));
        next.push((0, -(i as i32)));

        // Second half shifted left
        for j in (mid - 1)..last.len() {
            let (x, y) = last[j];
            next.push((x - 1, y));
        }

        flower.push(next);
    }

    flower.into_iter().flatten().collect()
}

fn solve(p: &Array2<i32>, n: usize) -> i32 {
    let flower = generate_flower(n);
    let (h, w) = p.dim();
    let mut result = 0;

    for &(dx, dy) in flower.iter() {
        let y1_start = (0.max(-dy) as usize).min(h);
        let y1_end = ((h as i32 - dy) as usize).min(h);
        let x1_start = (0.max(-dx) as usize).min(w);
        let x1_end = ((w as i32 - dx) as usize).min(w);

        let y2_start = (0.max(dy) as usize).min(h);
        let y2_end = ((h as i32 + dy) as usize).min(h);
        let x2_start = (0.max(dx) as usize).min(w);
        let x2_end = ((w as i32 + dx) as usize).min(w);

        let p1 = p.slice(s![y1_start..y1_end, x1_start..x1_end]);
        let p2 = p.slice(s![y2_start..y2_end, x2_start..x2_end]);

        result += p1
            .iter()
            .zip(p2.iter())
            .filter(|(&v1, &v2)| v2 >= 0 && v1 - v2 >= 100 + (dy.abs() + dx.abs()) as i32)
            .count() as i32;
    }
    result
}

#[public]
impl Solution for Day20 {
    fn solvepart1(&self, input: String) -> i64 {
        let data: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

        let start = find_start(&data).unwrap();
        let p = traverse_maze(&data, start);

        let result = solve(&p, 2);
        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let data: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

        let start = find_start(&data).unwrap();
        let p = traverse_maze(&data, start);

        let result = solve(&p, 20);
        result.try_into().unwrap()
    }
}
