extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day1 {}

#[public]
impl Solution for Day1 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut result = 0;

        for line in input.lines() {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            left.push(numbers[0]);
            right.push(numbers[1]);
        }

        left.sort();
        right.sort();

        for i in 0..left.len() {
            result += if left[i] > right[i] {
                left[i] - right[i]
            } else {
                right[i] - left[i]
            };
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut left = Vec::new();
        let mut right: HashMap<u32, u32> = HashMap::new();
        let mut result = 0;

        for line in input.lines() {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            left.push(numbers[0]);

            let count = right.entry(numbers[1]).or_insert(0);
            *count += 1;
        }

        for val in left {
            result += val * right.get(&val).unwrap_or(&0);
        }

        result.try_into().unwrap()
    }
}
