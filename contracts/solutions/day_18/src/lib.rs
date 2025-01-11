extern crate alloc;

use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashSet;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day18 {}

const H: usize = 71;
const W: usize = 71;
const NEIGHBOURS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn solve(lim: usize, p: &Vec<Vec<usize>>) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0, 0));

    while let Some((x, y, s)) = queue.pop_front() {
        if x == W - 1 && y == H - 1 {
            return Some(s);
        }

        for &(dx, dy) in NEIGHBOURS.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < W as i32 && ny >= 0 && ny < H as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if lim <= p[ny][nx] && !visited.contains(&(nx, ny)) {
                    queue.push_back((nx, ny, s + 1));
                    visited.insert((nx, ny));
                }
            }
        }
    }
    None
}

fn binary_search<F>(mut left: usize, mut right: usize, predicate: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while left < right {
        let mid = (left + right) / 2;
        if predicate(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

#[public]
impl Day18 {
    fn solvepart1(&self, input: String) -> i64 {
        let data: Vec<(usize, usize)> = input
            .lines()
            .map(|line| {
                let split_line = line.split(",").collect::<Vec<_>>();
                (
                    split_line[0].parse::<usize>().unwrap(),
                    split_line[1].parse::<usize>().unwrap(),
                )
            })
            .collect();

        let mut p = vec![vec![data.len(); W]; H];
        for (i, &(x, y)) in data.iter().enumerate() {
            p[y][x] = i;
        }

        solve(1024, &p).unwrap().try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> String {
        let data: Vec<(usize, usize)> = input
            .lines()
            .map(|line| {
                let split_line = line.split(",").collect::<Vec<_>>();
                (
                    split_line[0].parse::<usize>().unwrap(),
                    split_line[1].parse::<usize>().unwrap(),
                )
            })
            .collect();

        let mut p = vec![vec![data.len(); W]; H];
        for (i, &(x, y)) in data.iter().enumerate() {
            p[y][x] = i;
        }

        let r = binary_search(1024, data.len(), |x| solve(x, &p).is_none()) - 1;
        format!("{},{}", data[r].0, data[r].1)
    }
}
