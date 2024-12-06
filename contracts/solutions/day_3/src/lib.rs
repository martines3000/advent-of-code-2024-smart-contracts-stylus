extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day3 {}

#[public]
impl Solution for Day3 {
    fn solvepart1(&self, input: String) -> u32 {
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

    fn solvepart2(&self, input: String) -> u32 {
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
}
