extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core;
use hashbrown::HashMap;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day11 {}

fn blink(d: u64, count: i32, memo: &mut HashMap<(u64, i32), u64>) -> u64 {
    if count == 0 {
        return 1;
    }

    if let Some(&result) = memo.get(&(d, count)) {
        return result;
    }

    if d == 0 {
        let tmp = blink(1, count - 1, memo);
        memo.insert((d, count), tmp);
        return tmp;
    }

    let s = d.to_string();
    let ls = s.len();

    if ls % 2 == 0 {
        let half = ls / 2;
        let first_half: u64 = s[..half].parse().unwrap();
        let second_half: u64 = s[half..].parse().unwrap();
        let tmp = blink(first_half, count - 1, memo) + blink(second_half, count - 1, memo);
        memo.insert((d, count), tmp);
        return tmp;
    }

    let tmp = blink(d * 2024, count - 1, memo);
    memo.insert((d, count), tmp);
    tmp
}

#[public]
impl Solution for Day11 {
    fn solvepart1(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let data = input
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut store = Vec::new();
        let mut temp = Vec::new();
        let (mut left, mut right);

        for i in 0..data.len() {
            store.push(data[i]);

            for _ in 0..25 {
                for j in 0..store.len() {
                    if store[j] == 0 {
                        temp.push(1);
                    } else {
                        let current_string = store[j].to_string();
                        if current_string.len() % 2 == 0 {
                            (left, right) = current_string.split_at(current_string.len() / 2);

                            temp.push(left.parse::<u64>().unwrap());
                            temp.push(right.parse::<u64>().unwrap());
                        } else {
                            temp.push(store[j] * 2024);
                        }
                    }
                }

                store = core::mem::take(&mut temp);
            }

            result += store.len() as u64;
            store.clear();
        }

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let mut result: u64 = 0;

        let mut memo = HashMap::new();

        let data = input
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for num in data {
            result += blink(num, 75, &mut memo);
        }

        result.try_into().unwrap()
    }
}
