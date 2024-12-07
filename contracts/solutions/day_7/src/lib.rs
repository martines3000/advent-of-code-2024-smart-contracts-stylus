extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day7 {}

#[public]
impl Solution for Day7 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        for line in input.lines() {
            let values: Vec<&str> = line.split(':').collect();
            let test_value: u64 = values[0].parse().unwrap();
            let numbers: Vec<u64> = values[1][1..]
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();

            let mut queue: VecDeque<u64> = VecDeque::new();

            queue.push_back(numbers[0]);

            for i in 1..numbers.len() {
                let pop_amount = u64::pow(2, (i - 1) as u32);

                for _ in 0..pop_amount {
                    let current = queue.pop_front().unwrap();

                    queue.push_back(current + numbers[i]);
                    queue.push_back(current * numbers[i]);
                }
            }

            while !queue.is_empty() {
                let current = queue.pop_front().unwrap();
                if current == test_value {
                    result += test_value;
                    queue.clear();
                }
            }

            if !queue.is_empty() {
                queue.clear();
            }
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        for line in input.lines() {
            let values: Vec<&str> = line.split(':').collect();
            let test_value: u64 = values[0].parse().unwrap();
            let numbers: Vec<u64> = values[1][1..]
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();

            // if solve(numbers[0], &numbers, 1, test_value) {
            //     result += test_value;
            // }

            let mut queue: VecDeque<u64> = VecDeque::new();

            queue.push_back(numbers[0]);

            for i in 1..numbers.len() {
                let pop_amount = u64::pow(3, (i - 1) as u32);

                for _ in 0..pop_amount {
                    let current = queue.pop_front().unwrap();

                    queue.push_back(current + numbers[i]);
                    queue.push_back(current * numbers[i]);
                    queue.push_back(format!("{}{}", current, numbers[i]).parse().unwrap());
                }
            }

            while !queue.is_empty() {
                let current = queue.pop_front().unwrap();
                if current == test_value {
                    result += test_value;
                    queue.clear();
                }
            }

            if !queue.is_empty() {
                queue.clear();
            }
        }

        result.try_into().unwrap()
    }
}

// pub fn solve(val: u64, numbers: &Vec<u64>, idx: usize, target: u64) -> bool {
//     if val > target {
//         return false;
//     }
//     if idx == numbers.len() {
//         return val == target;
//     }
//     solve(val + numbers[idx], numbers, idx + 1, target)
//         || solve(val * numbers[idx], numbers, idx + 1, target)
//         || (solve(
//             format!("{}{}", val, numbers[idx]).parse().unwrap(),
//             numbers,
//             idx + 1,
//             target,
//         ))
// }
