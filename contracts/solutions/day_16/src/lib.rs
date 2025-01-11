extern crate alloc;

use std::{cmp::Ordering, collections::BinaryHeap};

use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::{HashMap, HashSet};
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day16 {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reindeer1 {
    score: u64,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

// Min-heap
impl Ord for Reindeer1 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Reindeer1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reindeer2 {
    score: u64,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    path: Vec<(i32, i32)>,
}

// Min-heap
impl Ord for Reindeer2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Reindeer2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn check_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

#[public]
impl Solution for Day16 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let (start_x, start_y) = ((grid.len() - 2) as i32, 1_i32);
        let (end_x, end_y) = (1_i32, (grid[0].len() - 2) as i32);

        let mut visited = HashMap::new();
        let mut pqueue = BinaryHeap::new();

        pqueue.push(Reindeer1 {
            score: 0,
            x: start_x,
            y: start_y,
            dx: 1,
            dy: 0,
        });

        visited.insert((start_x, start_y, 1, 0), 0);

        loop {
            let current_reindeer = pqueue.peek().unwrap();
            if current_reindeer.x == end_x && current_reindeer.y == end_y {
                break;
            }

            let reindeer = pqueue.pop().unwrap();

            if check_bounds(
                reindeer.x + reindeer.dx,
                reindeer.y + reindeer.dy,
                grid.len() as i32,
                grid[0].len() as i32,
            ) && grid[(reindeer.y + reindeer.dy) as usize][(reindeer.x + reindeer.dx) as usize]
                != '#'
            {
                if !visited.contains_key(&(
                    reindeer.x + reindeer.dx,
                    reindeer.y + reindeer.dy,
                    reindeer.dx,
                    reindeer.dy,
                )) {
                    visited.insert(
                        (
                            reindeer.x + reindeer.dx,
                            reindeer.y + reindeer.dy,
                            reindeer.dx,
                            reindeer.dy,
                        ),
                        reindeer.score + 1,
                    );

                    pqueue.push(Reindeer1 {
                        score: reindeer.score + 1,
                        x: reindeer.x + reindeer.dx,
                        y: reindeer.y + reindeer.dy,
                        dx: reindeer.dx,
                        dy: reindeer.dy,
                    });
                }
            }

            if !visited.contains_key(&(reindeer.x, reindeer.y, -reindeer.dy, reindeer.dx)) {
                visited.insert(
                    (reindeer.x, reindeer.y, -reindeer.dy, reindeer.dx),
                    reindeer.score + 1000,
                );

                pqueue.push(Reindeer1 {
                    score: reindeer.score + 1000,
                    x: reindeer.x,
                    y: reindeer.y,
                    dx: -reindeer.dy,
                    dy: reindeer.dx,
                });
            }

            if !visited.contains_key(&(reindeer.x, reindeer.y, reindeer.dy, -reindeer.dx)) {
                visited.insert(
                    (reindeer.x, reindeer.y, reindeer.dy, -reindeer.dx),
                    reindeer.score + 1000,
                );

                pqueue.push(Reindeer1 {
                    score: reindeer.score + 1000,
                    x: reindeer.x,
                    y: reindeer.y,
                    dx: reindeer.dy,
                    dy: -reindeer.dx,
                });
            }
        }

        result = pqueue.pop().unwrap().score;

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let (start_x, start_y) = ((grid.len() - 2) as i32, 1_i32);
        let (end_x, end_y) = (1_i32, (grid[0].len() - 2) as i32);

        let mut visited = HashMap::new();
        let mut pqueue = BinaryHeap::new();

        pqueue.push(Reindeer2 {
            score: 0,
            x: start_x,
            y: start_y,
            dx: 1,
            dy: 0,
            path: Vec::new(),
        });

        visited.insert((start_x, start_y, 1, 0), 0);

        let mut paths: HashSet<(i32, i32)> = HashSet::new();
        let mut best: Option<u64> = None;

        while !pqueue.is_empty() {
            let mut reindeer = pqueue.pop().unwrap();
            reindeer.path.push((reindeer.x, reindeer.y));

            if reindeer.x == end_x && reindeer.y == end_y {
                if best.is_none() || reindeer.score <= best.unwrap() {
                    best = Some(reindeer.score);
                    paths.extend(reindeer.path.iter().map(|(x, y)| (*x, *y)));
                }

                pqueue.pop().unwrap();

                continue;
            }

            if check_bounds(
                reindeer.x + reindeer.dx,
                reindeer.y + reindeer.dy,
                grid.len() as i32,
                grid[0].len() as i32,
            ) && grid[(reindeer.y + reindeer.dy) as usize][(reindeer.x + reindeer.dx) as usize]
                != '#'
            {
                if !visited.contains_key(&(
                    reindeer.x + reindeer.dx,
                    reindeer.y + reindeer.dy,
                    reindeer.dx,
                    reindeer.dy,
                )) || *visited
                    .get(&(
                        reindeer.x + reindeer.dx,
                        reindeer.y + reindeer.dy,
                        reindeer.dx,
                        reindeer.dy,
                    ))
                    .unwrap()
                    >= (reindeer.score + 1)
                {
                    visited.insert(
                        (
                            reindeer.x + reindeer.dx,
                            reindeer.y + reindeer.dy,
                            reindeer.dx,
                            reindeer.dy,
                        ),
                        reindeer.score + 1,
                    );

                    pqueue.push(Reindeer2 {
                        score: reindeer.score + 1,
                        x: reindeer.x + reindeer.dx,
                        y: reindeer.y + reindeer.dy,
                        dx: reindeer.dx,
                        dy: reindeer.dy,
                        path: reindeer.path.clone(),
                    });
                }
            }

            if !visited.contains_key(&(reindeer.x, reindeer.y, -reindeer.dy, reindeer.dx))
                || *visited
                    .get(&(reindeer.x, reindeer.y, -reindeer.dy, reindeer.dx))
                    .unwrap()
                    >= (reindeer.score + 1000)
            {
                visited.insert(
                    (reindeer.x, reindeer.y, -reindeer.dy, reindeer.dx),
                    reindeer.score + 1000,
                );

                pqueue.push(Reindeer2 {
                    score: reindeer.score + 1000,
                    x: reindeer.x,
                    y: reindeer.y,
                    dx: -reindeer.dy,
                    dy: reindeer.dx,
                    path: reindeer.path.clone(),
                });
            }

            if !visited.contains_key(&(reindeer.x, reindeer.y, reindeer.dy, -reindeer.dx))
                || *visited
                    .get(&(reindeer.x, reindeer.y, reindeer.dy, -reindeer.dx))
                    .unwrap()
                    >= (reindeer.score + 1000)
            {
                visited.insert(
                    (reindeer.x, reindeer.y, reindeer.dy, -reindeer.dx),
                    reindeer.score + 1000,
                );

                pqueue.push(Reindeer2 {
                    score: reindeer.score + 1000,
                    x: reindeer.x,
                    y: reindeer.y,
                    dx: reindeer.dy,
                    dy: -reindeer.dx,
                    path: reindeer.path.clone(),
                });
            }
        }

        let result = paths.len() as u64;

        result.try_into().unwrap()
    }
}
