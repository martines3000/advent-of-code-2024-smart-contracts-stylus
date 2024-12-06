extern crate alloc;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day5 {}

#[public]
impl Solution for Day5 {
    fn solvepart1(&self, input: String) -> u32 {
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

    fn solvepart2(&self, input: String) -> u32 {
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
}
