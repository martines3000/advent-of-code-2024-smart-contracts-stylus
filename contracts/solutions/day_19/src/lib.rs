extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashMap;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day19 {}

struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn add(&mut self, s: &str) {
        if s.is_empty() {
            self.is_end = true;
        } else {
            let first_char = s.chars().next().unwrap();
            self.children
                .entry(first_char)
                .or_insert_with(Trie::new)
                .add(&s[1..]);
        }
    }

    fn get(&self, s: &str, d: usize) -> Vec<usize> {
        let mut ret = Vec::new();

        if !s.is_empty() {
            if let Some(child) = self.children.get(&s.chars().next().unwrap()) {
                ret.extend(child.get(&s[1..], d + 1));
            }
        }

        if self.is_end {
            ret.push(d);
        }

        ret
    }
}

fn solve(pattern: &str, root: &Trie, memo: &mut HashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(&result) = memo.get(pattern) {
        return result;
    }

    let result = root
        .get(pattern, 0)
        .iter()
        .map(|&t| solve(&pattern[t..], root, memo))
        .sum();

    memo.insert(pattern.to_string(), result);
    result
}

#[public]
impl Solution for Day19 {
    fn solvepart1(&self, input: String) -> i64 {
        let parts: Vec<&str> = input.split_terminator("\n\n").collect::<Vec<_>>();

        let towels: Vec<String> = parts[0].split(", ").map(|s| s.to_string()).collect();
        let patterns: Vec<String> = parts[1].split("\n").map(|s| s.to_string()).collect();

        let mut root = Trie::new();
        for towel in towels {
            root.add(&towel);
        }

        let mut memo = HashMap::new();
        let results: Vec<usize> = patterns
            .iter()
            .map(|pattern| {
                if pattern.is_empty() {
                    0
                } else {
                    solve(pattern, &root, &mut memo)
                }
            })
            .collect();

        let result = results.iter().filter(|&&x| x > 0).count();
        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let parts: Vec<&str> = input.split_terminator("\n\n").collect::<Vec<_>>();

        let towels: Vec<String> = parts[0].split(", ").map(|s| s.to_string()).collect();
        let patterns: Vec<String> = parts[1].split("\n").map(|s| s.to_string()).collect();

        let mut root = Trie::new();
        for towel in towels {
            root.add(&towel);
        }

        let mut memo = HashMap::new();
        let results: Vec<usize> = patterns
            .iter()
            .map(|pattern| {
                if pattern.is_empty() {
                    0
                } else {
                    solve(pattern, &root, &mut memo)
                }
            })
            .collect();

        let result = results.iter().sum::<usize>();
        result.try_into().unwrap()
    }
}
