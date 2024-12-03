#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// #![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use stylus_sdk::{console, prelude::*};

/// The storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[storage]
/// The entrypoint macro defines where Stylus execution begins. External methodsf
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct AOC2024 {}

#[public]
impl AOC2024 {
    pub fn solve11(&self, input: String) -> u32 {
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

        result
    }

    pub fn solve12(&self, input: String) -> u32 {
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

        result
    }

    pub fn solve21(&self, input: String) -> u32 {
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

    pub fn solve22(&self, input: String) -> u32 {
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

    pub fn solve31(&self, input: String) -> u32 {
        let mut result = 0;

        let indices: Vec<_> = input.match_indices("mul(").collect();

        let bytes_input = input.as_bytes();
        let mut i;

        let mut first_num;
        let mut second_num;
        let mut state;
        let mut max_i;
        let mut is_valid;

        for (start, _) in indices {
            state = 0;
            first_num = String::new();
            second_num = String::new();
            is_valid = false;

            i = start + 4;
            max_i = if input.len() < start + 12 {
                input.len()
            } else {
                start + 12
            };

            while i < max_i {
                match state {
                    0 => {
                        if bytes_input[i].is_ascii_digit() && first_num.len() < 3 {
                            first_num.push(char::from_u32(bytes_input[i] as u32).unwrap());
                        } else if !first_num.is_empty() && bytes_input[i] == b',' {
                            state = 1;
                        } else {
                            break;
                        }
                    }
                    1 => {
                        if bytes_input[i].is_ascii_digit() && second_num.len() < 3 {
                            second_num.push(char::from_u32(bytes_input[i] as u32).unwrap());
                        } else if !second_num.is_empty() && bytes_input[i] == b')' {
                            is_valid = true;
                            break;
                        } else {
                            break;
                        }
                    }
                    _ => {
                        break;
                    }
                }

                i += 1;
            }

            if is_valid {
                result += first_num.parse::<u32>().unwrap() * second_num.parse::<u32>().unwrap();
            }
        }

        result
    }

    pub fn solve32(&self, input: String) -> u32 {
        let mut result = 0;

        let indices: Vec<_> = input.match_indices("mul(").collect();
        let does: Vec<_> = input
            .match_indices("do()")
            .into_iter()
            .map(|x| (x.0, true))
            .collect();

        let donts: Vec<_> = input
            .match_indices("don't()")
            .into_iter()
            .map(|x| (x.0, false))
            .collect();

        let mut mixed: Vec<&(usize, bool)> = does.iter().chain(donts.iter()).collect::<Vec<_>>();
        mixed.sort_by(|a, b| a.0.cmp(&b.0));

        let bytes_input = input.as_bytes();
        let mut i;

        let mut first_num;
        let mut second_num;
        let mut state;
        let mut max_i;
        let mut is_valid;
        let mut j = 0;
        let mut process = true;

        for (start, _) in indices {
            state = 0;
            first_num = String::new();
            second_num = String::new();
            is_valid = false;

            i = start + 4;
            max_i = if input.len() < start + 12 {
                input.len()
            } else {
                start + 12
            };

            while j < mixed.len() && mixed[j].0 < i {
                process = mixed[j].1;
                j += 1;
            }

            if !process {
                continue;
            }

            while i < max_i {
                match state {
                    0 => {
                        if bytes_input[i].is_ascii_digit() && first_num.len() < 3 {
                            first_num.push(char::from_u32(bytes_input[i] as u32).unwrap());
                        } else if !first_num.is_empty() && bytes_input[i] == b',' {
                            state = 1;
                        } else {
                            break;
                        }
                    }
                    1 => {
                        if bytes_input[i].is_ascii_digit() && second_num.len() < 3 {
                            second_num.push(char::from_u32(bytes_input[i] as u32).unwrap());
                        } else if !second_num.is_empty() && bytes_input[i] == b')' {
                            is_valid = true;
                            break;
                        } else {
                            break;
                        }
                    }
                    _ => {
                        break;
                    }
                }

                i += 1;
            }

            if is_valid {
                result += first_num.parse::<u32>().unwrap() * second_num.parse::<u32>().unwrap();
            }
        }

        result
    }
    // This was used for testing the contracts.
    // Solves day 1 part 1 of AOC 2023.
    // pub fn solve01(&self, input: String) -> u32 {
    //     let mut result = 0;

    //     let mut first_digit = 0;
    //     let mut last_digit = 0;

    //     for line in input.lines() {
    //         for c in line.chars() {
    //             if c.is_ascii_digit() {
    //                 if first_digit == 0 {
    //                     first_digit = c.to_digit(10).unwrap();
    //                 }

    //                 last_digit = c.to_digit(10).unwrap();
    //             }
    //         }

    //         result += first_digit * 10 + last_digit;
    //         first_digit = 0;
    //         last_digit = 0;
    //     }

    //     result
    // }

    // This was used for testing the contracts.
    // Solves day 1 part 2 of AOC 2023.
    // pub fn solve02(&self, input: String) -> u32 {
    //     let words = [
    //         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    //     ];

    //     let mut result = 0;

    //     let mut first_digit = 0;
    //     let mut last_digit = 0;

    //     let mut current_word = String::new();

    //     for line in input.lines() {
    //         for i in 0..line.len() {
    //             for j in 0..5 {
    //                 if (i + j) < line.len() {
    //                     let c = line.chars().nth(i + j).unwrap();

    //                     current_word.push(c);

    //                     if current_word.len() >= 3 {
    //                         let position = words.iter().position(|&word| word == current_word);

    //                         if let Some(position) = position {
    //                             if first_digit == 0 {
    //                                 first_digit = position as u32 + 1;
    //                             }

    //                             last_digit = position as u32 + 1;
    //                         }
    //                     }
    //                 }
    //             }

    //             current_word.clear();

    //             let c = line.chars().nth(i).unwrap();

    //             if c.is_ascii_digit() {
    //                 if first_digit == 0 {
    //                     first_digit = c.to_digit(10).unwrap();
    //                 }

    //                 last_digit = c.to_digit(10).unwrap();
    //             }
    //         }

    //         result += first_digit * 10 + last_digit;
    //         first_digit = 0;
    //         last_digit = 0;
    //     }

    //     result
    // }
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

// #[cfg(target_arch = "wasm32")]
// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
