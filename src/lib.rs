#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// #![no_std]
extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::vec;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use stylus_sdk::prelude::*;

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

    pub fn solve41(&self, input: String) -> u32 {
        let char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut result = 0;

        for i in 0..char_vec.len() {
            for j in 0..char_vec[i].len() {
                // LR
                if j + 3 < char_vec[i].len()
                    && char_vec[i][j] == 'X'
                    && char_vec[i][j + 1] == 'M'
                    && char_vec[i][j + 2] == 'A'
                    && char_vec[i][j + 3] == 'S'
                {
                    result += 1;
                }

                // RL
                if j + 3 < char_vec[i].len()
                    && char_vec[i][j] == 'S'
                    && char_vec[i][j + 1] == 'A'
                    && char_vec[i][j + 2] == 'M'
                    && char_vec[i][j + 3] == 'X'
                {
                    result += 1;
                }

                // TB
                if i + 3 < char_vec.len()
                    && char_vec[i][j] == 'X'
                    && char_vec[i + 1][j] == 'M'
                    && char_vec[i + 2][j] == 'A'
                    && char_vec[i + 3][j] == 'S'
                {
                    result += 1;
                }

                // BT
                if i + 3 < char_vec.len()
                    && char_vec[i][j] == 'S'
                    && char_vec[i + 1][j] == 'A'
                    && char_vec[i + 2][j] == 'M'
                    && char_vec[i + 3][j] == 'X'
                {
                    result += 1;
                }

                // TRBL
                if i + 3 < char_vec.len()
                    && j + 3 < char_vec[i].len()
                    && char_vec[i][j + 3] == 'X'
                    && char_vec[i + 1][j + 2] == 'M'
                    && char_vec[i + 2][j + 1] == 'A'
                    && char_vec[i + 3][j] == 'S'
                {
                    result += 1;
                }

                // BLTR
                if i + 3 < char_vec.len()
                    && j + 3 < char_vec[i].len()
                    && char_vec[i][j + 3] == 'S'
                    && char_vec[i + 1][j + 2] == 'A'
                    && char_vec[i + 2][j + 1] == 'M'
                    && char_vec[i + 3][j] == 'X'
                {
                    result += 1;
                }

                // TLBR
                if i + 3 < char_vec.len()
                    && j + 3 < char_vec[i].len()
                    && char_vec[i][j] == 'X'
                    && char_vec[i + 1][j + 1] == 'M'
                    && char_vec[i + 2][j + 2] == 'A'
                    && char_vec[i + 3][j + 3] == 'S'
                {
                    result += 1;
                }

                // BRTL
                if i + 3 < char_vec.len()
                    && j + 3 < char_vec[i].len()
                    && char_vec[i][j] == 'S'
                    && char_vec[i + 1][j + 1] == 'A'
                    && char_vec[i + 2][j + 2] == 'M'
                    && char_vec[i + 3][j + 3] == 'X'
                {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn solve42(&self, input: String) -> u32 {
        let char_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut result = 0;

        for i in 0..char_vec.len() {
            for j in 0..char_vec[i].len() {
                if i + 2 < char_vec.len()
                    && j + 2 < char_vec[i].len()
                    && ((char_vec[i][j] == 'M'
                        && char_vec[i + 1][j + 1] == 'A'
                        && char_vec[i + 2][j + 2] == 'S')
                        || (char_vec[i][j] == 'S'
                            && char_vec[i + 1][j + 1] == 'A'
                            && char_vec[i + 2][j + 2] == 'M'))
                    && ((char_vec[i][j + 2] == 'M'
                        && char_vec[i + 1][j + 1] == 'A'
                        && char_vec[i + 2][j] == 'S')
                        || (char_vec[i][j + 2] == 'S'
                            && char_vec[i + 1][j + 1] == 'A'
                            && char_vec[i + 2][j] == 'M'))
                {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn solve51(&self, input: String) -> u32 {
        // Split input on 2 new lines
        let input_vec: Vec<&str> = input.split("\n\n").collect();

        let mut result = 0;

        let mut rules: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); 100];

        for line in input_vec[0].lines() {
            let rule: Vec<u32> = line.split("|").map(|x| x.parse::<u32>().unwrap()).collect();
            rules[rule[1] as usize - 1].insert(rule[0]);
        }

        let mut current_rules = BTreeSet::new();
        let mut is_valid;

        for line in input_vec[1].lines() {
            let numbers: Vec<u32> = line
                .split_terminator(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            is_valid = true;
            current_rules.clear();
            for i in 0..numbers.len() {
                let number = numbers[i];

                if !current_rules.is_empty() {
                    if current_rules.contains(&number) {
                        is_valid = false;
                        break;
                    }
                }
                current_rules.extend(rules[number as usize - 1].iter().cloned());
            }

            if is_valid {
                result += numbers[numbers.len() / 2];
            }
        }

        result
    }

    pub fn solve52(&self, input: String) -> u32 {
        // Split input on 2 new lines
        let input_vec: Vec<&str> = input.split("\n\n").collect();

        let mut result = 0;

        let mut rules: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); 100];

        for line in input_vec[0].lines() {
            let rule: Vec<u32> = line.split("|").map(|x| x.parse::<u32>().unwrap()).collect();
            rules[rule[1] as usize - 1].insert(rule[0]);
        }

        let mut current_rules = BTreeSet::new();

        let mut invalid_lines: Vec<Vec<u32>> = Vec::new();

        for line in input_vec[1].lines() {
            let numbers: Vec<u32> = line
                .split_terminator(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            current_rules.clear();
            for i in 0..numbers.len() {
                let number = numbers[i];

                if !current_rules.is_empty() {
                    if current_rules.contains(&number) {
                        invalid_lines.push(numbers);
                        break;
                    }
                }
                current_rules.extend(rules[number as usize - 1].iter().cloned());
            }
        }

        let mut current_rules: Vec<BTreeSet<u32>> = Vec::new();
        let mut is_valid;

        // Fix invalid lines
        for line in invalid_lines {
            let mut i = 0;
            let mut current_numbers = line.clone();
            let mut new_numbers = Vec::new();
            current_rules.clear();

            while i < current_numbers.len() {
                let number = current_numbers[i];

                if !current_rules.is_empty() {
                    is_valid = true;

                    for j in 0..current_rules.len() {
                        if current_rules[j].contains(&number) {
                            current_numbers.remove(i);
                            let (first, second) = current_numbers.split_at(j);
                            new_numbers.extend_from_slice(first);
                            new_numbers.push(number);
                            new_numbers.extend_from_slice(second);

                            current_numbers = new_numbers.clone();
                            new_numbers.clear();
                            i = 0;
                            current_rules.clear();
                            is_valid = false;
                            break;
                        }
                    }

                    if !is_valid {
                        continue;
                    }
                }
                // current_rules.extend(rules[number as usize - 1].iter().cloned());
                current_rules.push(rules[number as usize - 1].clone());

                i += 1;
            }

            result += current_numbers[current_numbers.len() / 2]
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
