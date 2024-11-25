#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]

// Allow `cargo stylus export-abi` to generate a main function.
extern crate alloc;

use alloc::string::String;

use stylus_sdk::{console, prelude::*};

// /// Import items from the SDK. The prelude contains common traits and macros.
// use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

/// The storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[storage]
/// The entrypoint macro defines where Stylus execution begins. External methods
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct AOC2024 {}

#[public]
impl AOC2024 {
    pub fn solve01(&self, input: String) -> u32 {
        console!("Hello World!");
        let mut result = 0;

        let mut first_digit = 0;
        let mut last_digit = 0;

        for line in input.lines() {
            for c in line.chars() {
                if c.is_ascii_digit() {
                    if first_digit == 0 {
                        first_digit = c.to_digit(10).unwrap();
                    }

                    last_digit = c.to_digit(10).unwrap();
                }
            }

            result += first_digit * 10 + last_digit;
            first_digit = 0;
            last_digit = 0;
        }

        result
    }

    pub fn solve02(&self, input: String) -> u32 {
        console!("Hello World!");

        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut result = 0;

        let mut first_digit = 0;
        let mut last_digit = 0;

        let mut current_word = String::new();

        for line in input.lines() {
            for i in 0..line.len() {
                for j in 0..5 {
                    if (i + j) < line.len() {
                        let c = line.chars().nth(i + j).unwrap();

                        current_word.push(c);

                        if current_word.len() >= 3 {
                            let position = words.iter().position(|&word| word == current_word);

                            if let Some(position) = position {
                                if first_digit == 0 {
                                    first_digit = position as u32 + 1;
                                }

                                last_digit = position as u32 + 1;
                            }
                        }
                    }
                }

                current_word.clear();

                let c = line.chars().nth(i).unwrap();

                if c.is_ascii_digit() {
                    if first_digit == 0 {
                        first_digit = c.to_digit(10).unwrap();
                    }

                    last_digit = c.to_digit(10).unwrap();
                }
            }

            result += first_digit * 10 + last_digit;
            first_digit = 0;
            last_digit = 0;
        }

        result
    }
}
