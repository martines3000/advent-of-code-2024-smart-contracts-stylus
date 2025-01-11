extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day17 {}

fn combo(op: i64, a: i64, b: i64, c: i64) -> Option<i64> {
    match op {
        0 | 1 | 2 | 3 => Some(op),
        4 => Some(a),
        5 => Some(b),
        6 => Some(c),
        7 => None,
        _ => None,
    }
}

fn solve(a: i64, b: i64, c: i64, program: &[i64]) -> Vec<i64> {
    let mut stack: Vec<i64> = Vec::new();
    let mut ip = 0;
    let mut a = a;
    let mut b = b;
    let mut c = c;

    while ip < program.len() {
        match program[ip] {
            0 => {
                if let Some(val) = combo(program[ip + 1], a, b, c) {
                    a /= 1 << val;
                }
                ip += 2;
            }
            1 => {
                b ^= program[ip + 1];
                ip += 2;
            }
            2 => {
                if let Some(val) = combo(program[ip + 1], a, b, c) {
                    b = val & 0b111;
                }
                ip += 2;
            }
            3 => {
                if a != 0 {
                    ip = program[ip + 1] as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                b ^= c;
                ip += 2;
            }
            5 => {
                if let Some(val) = combo(program[ip + 1], a, b, c) {
                    stack.push(val & 0b111);
                }
                ip += 2;
            }
            6 => {
                if let Some(val) = combo(program[ip + 1], a, b, c) {
                    b = a / (1 << val);
                }
                ip += 2;
            }
            7 => {
                if let Some(val) = combo(program[ip + 1], a, b, c) {
                    c = a / (1 << val);
                }
                ip += 2;
            }
            _ => break,
        }
    }
    stack
}

fn go(val: i64, i: i32, program: &[i64]) -> Option<i64> {
    if i < 0 {
        return Some(val);
    }

    let mul = 8_i64.pow(i as u32);
    for k in 0..8 {
        let t = val + mul * k;
        let res = solve(t, program[1], program[2], program);

        if res[i as usize] != program[i as usize] {
            continue;
        }

        if let Some(result) = go(t, i - 1, program) {
            return Some(result);
        }
    }
    None
}

#[public]
impl Day17 {
    fn solvepart1(&self, input: String) -> String {
        let parts: Vec<&str> = input.split_terminator("\n\n").collect::<Vec<_>>();
        let (mut a, mut b, mut c) = (0, 0, 0);

        parts[0]
            .split("\n")
            .into_iter()
            .enumerate()
            .for_each(|(idx, line)| {
                let split_line = line.split(" ").collect::<Vec<_>>();
                match idx {
                    0 => {
                        a = split_line.last().unwrap().parse::<i64>().unwrap();
                    }
                    1 => {
                        b = split_line.last().unwrap().parse::<i64>().unwrap();
                    }
                    2 => {
                        c = split_line.last().unwrap().parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            });

        let second_part_split = parts[1].split("\n").collect::<Vec<_>>()[0]
            .split(" ")
            .collect::<Vec<_>>();
        let program: Vec<i64> = second_part_split[1]
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let result = solve(a, b, c, &program);

        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn solvepart2(&self, input: String) -> i64 {
        let parts: Vec<&str> = input.split_terminator("\n\n").collect::<Vec<_>>();
        let (mut a, mut b, mut c) = (0, 0, 0);

        parts[0]
            .split("\n")
            .into_iter()
            .enumerate()
            .for_each(|(idx, line)| {
                let split_line = line.split(" ").collect::<Vec<_>>();
                match idx {
                    0 => {
                        a = split_line.last().unwrap().parse::<i64>().unwrap();
                    }
                    1 => {
                        b = split_line.last().unwrap().parse::<i64>().unwrap();
                    }
                    2 => {
                        c = split_line.last().unwrap().parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            });

        let second_part_split = parts[1].split("\n").collect::<Vec<_>>()[0]
            .split(" ")
            .collect::<Vec<_>>();
        let program: Vec<i64> = second_part_split[1]
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        // Part 2
        let t = (program.len() - 1) as i32;
        if let Some(result) = go(8_i64.pow(t as u32), t, &program) {
            return result;
        }

        return 0;
    }
}
