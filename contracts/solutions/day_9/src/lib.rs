extern crate alloc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::{console, prelude::*};

#[storage]
#[entrypoint]
pub struct Day9 {}

#[public]
impl Solution for Day9 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let input = input.lines().next().unwrap();
        let mut full = vec![0; input.len() / 2 + 1];
        let mut total_nums: u64 = 0;
        let mut digit: u64;

        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                digit = c.to_digit(10).unwrap() as u64;
                total_nums += digit;
                full[i / 2] = digit;
            }
        }

        total_nums -= 1;

        let mut j = full.len() - 1;
        let mut id: u64;
        let mut pos = 0;

        for (i, c) in input.chars().enumerate() {
            if pos > total_nums {
                continue;
            }

            if i % 2 == 0 {
                id = i as u64 / 2;

                for _ in 0..c.to_digit(10).unwrap() {
                    result += pos * id;
                    pos += 1;

                    if pos > total_nums {
                        break;
                    }
                }
            } else {
                for _ in 0..c.to_digit(10).unwrap() {
                    if pos > total_nums {
                        break;
                    }

                    while j > 0 && full[j] == 0 {
                        j -= 1;
                    }

                    if full[j] == 0 {
                        break;
                    }

                    id = j as u64;
                    full[j] -= 1;

                    result += pos * id;
                    pos += 1;
                }
            }
        }
        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let input = input.lines().next().unwrap();
        let mut full = vec![0; input.len() / 2 + 1];
        let mut moved = vec![Vec::<(usize, u8)>::new(); input.len() / 2 + 1];
        let mut moved_ids = vec![false; input.len() / 2 + 1];
        let mut free_space = vec![0; input.len() / 2 + 1];
        let mut digit: u8;

        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                digit = c.to_digit(10).unwrap() as u8;
                full[i / 2] = digit;
            } else {
                free_space[i / 2] = c.to_digit(10).unwrap() as u64;
            }
        }

        let mut pos = 0;
        let full_len = full.len();

        for j in 0..full_len {
            for i in 0..full_len - j - 1 {
                if !moved_ids[full_len - j - 1] && free_space[i] >= full[full_len - j - 1] as u64 {
                    free_space[i] -= full[full_len - j - 1] as u64;
                    moved[i].push((full_len - j - 1, full[full_len - j - 1]));
                    moved_ids[full_len - j - 1] = true;
                    break;
                }
            }
        }

        for j in 1..=full_len {
            for _ in 0..full[j - 1] {
                if !moved_ids[j - 1] {
                    result += pos * (j - 1) as u64;
                }

                pos += 1;
            }

            for (id, count) in &moved[j - 1] {
                for _ in 0..*count {
                    result += *id as u64 * pos;

                    pos += 1;
                }
            }

            for _ in 0..free_space[j - 1] {
                pos += 1;
            }
        }

        result.try_into().unwrap()
    }
}
