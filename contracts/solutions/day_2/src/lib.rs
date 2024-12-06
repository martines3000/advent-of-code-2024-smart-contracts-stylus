extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day2 {}

#[public]
impl Solution for Day2 {
    fn solvepart1(&self, input: String) -> u32 {
        let mut result = 0;

        for line in input.lines() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let (is_safe, _) = check_safety(&numbers);

            if is_safe {
                result += 1;
            }
        }

        result
    }

    fn solvepart2(&self, input: String) -> u32 {
        let mut result = 0;

        for line in input.lines() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let (is_safe, problem_location) = check_safety(&numbers);

            if is_safe {
                result += 1;
                continue;
            }

            let mut nums_copy = numbers.clone();
            nums_copy.remove(problem_location as usize);
            if check_safety(&nums_copy).0 {
                result += 1;
                continue;
            }

            if problem_location as usize + 1 < numbers.len() {
                let mut nums_copy = numbers.clone();
                nums_copy.remove(problem_location as usize + 1);
                if check_safety(&nums_copy).0 {
                    result += 1;
                    continue;
                }
            }

            if problem_location - 1 >= 0 {
                let mut nums_copy = numbers.clone();
                nums_copy.remove((problem_location - 1) as usize);
                if check_safety(&nums_copy).0 {
                    result += 1;
                    continue;
                }
            }
        }

        result
    }
}

fn check_safety(numbers: &Vec<i32>) -> (bool, i32) {
    let mut diff: i32;
    let mut state = 0; // 1 = Increasing, 2 = Decreasing

    for i in 0..numbers.len() - 1 {
        if i == 0 {
            if numbers[i] == numbers[i + 1] {
                return (false, i as i32);
            }

            diff = numbers[i + 1] - numbers[i];

            if diff.abs() > 3 {
                return (false, i as i32);
            }

            state = if diff > 0 { 1 } else { 2 };
            continue;
        }

        diff = numbers[i + 1] - numbers[i];

        if diff.abs() > 3 || diff == 0 {
            return (false, i as i32);
        }

        if (state == 1 && diff < 0) || (state == 2 && diff > 0) {
            return (false, i as i32);
        }
    }

    (true, -1)
}
