extern crate alloc;

use std::iter::repeat;

use alloc::string::String;
use alloc::vec;
use hashbrown::HashMap;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day21 {}

fn get_pos_1(c: char) -> (i32, i32) {
    let t = "789456123 0A".chars().position(|x| x == c).unwrap();
    ((t % 3) as i32, (t / 3) as i32)
}

fn get_pos_2(c: char) -> (i32, i32) {
    let t = " ^A<v>".chars().position(|x| x == c).unwrap();
    ((t % 3) as i32, (t / 3) as i32)
}

fn validate(x: i32, y: i32, dx: i32, dy: i32, xx: i32, yy: i32) -> bool {
    if (x == xx && y + dy == yy) || (x + dx == xx && y == yy) {
        return false;
    }
    true
}

fn solve1(code: &str) -> Vec<String> {
    let mut ret = vec![String::new()];
    let (mut x, mut y) = (2, 3);

    for c in code.chars() {
        let (tx, ty) = get_pos_1(c);
        let (dx, dy) = (tx - x, ty - y);
        let rx: String = repeat('>')
            .take(dx.max(0) as usize)
            .chain(repeat('<').take((-dx).max(0) as usize))
            .collect();
        let ry: String = repeat('v')
            .take(dy.max(0) as usize)
            .chain(repeat('^').take((-dy).max(0) as usize))
            .collect();

        let mut nr = Vec::new();
        if validate(x, y, dx, 0, 0, 3) {
            nr.extend(ret.iter().map(|e| format!("{}{}{}{}", e, rx, ry, 'A')));
        }
        if dx != 0 && dy != 0 && validate(x, y, 0, dy, 0, 3) {
            nr.extend(ret.iter().map(|e| format!("{}{}{}{}", e, ry, rx, 'A')));
        }
        ret = nr;
        x = tx;
        y = ty;
    }
    ret
}

fn solve2(code: &str, d: i32, cache: &mut HashMap<(String, i32), i64>) -> i64 {
    if d == 0 {
        return code.len() as i64;
    }

    if let Some(&result) = cache.get(&(code.to_string(), d)) {
        return result;
    }

    let mut ret = 0;
    let (mut x, mut y) = (2, 0);

    for c in code.chars() {
        let (tx, ty) = get_pos_2(c);
        let (dx, dy) = (tx - x, ty - y);
        let rx: String = repeat('>')
            .take(dx.max(0) as usize)
            .chain(repeat('<').take((-dx).max(0) as usize))
            .collect();
        let ry: String = repeat('v')
            .take(dy.max(0) as usize)
            .chain(repeat('^').take((-dy).max(0) as usize))
            .collect();

        let mut nr = Vec::new();
        if validate(x, y, dx, 0, 0, 0) {
            nr.push(format!("{}{}A", rx, ry));
        }
        if dx != 0 && dy != 0 && validate(x, y, 0, dy, 0, 0) {
            nr.push(format!("{}{}A", ry, rx));
        }

        ret += nr
            .iter()
            .map(|e| solve2(e, d - 1, cache))
            .min()
            .unwrap_or(0);

        x = tx;
        y = ty;
    }

    cache.insert((code.to_string(), d), ret);
    ret
}

fn solve(data: &Vec<String>, d: i32) -> i64 {
    let mut result = 0;
    let mut cache = HashMap::new();

    for e in data {
        let solutions1 = solve1(&e);
        let r = solutions1
            .iter()
            .map(|p| solve2(p, d, &mut cache))
            .min()
            .unwrap_or(0);
        result += r * e[..e.len() - 1].parse::<i64>().unwrap();
    }

    result
}

#[public]
impl Solution for Day21 {
    fn solvepart1(&self, input: String) -> i64 {
        let data = input
            .lines()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let result = solve(&data, 2);
        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let data = input
            .lines()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let result = solve(&data, 25);
        result.try_into().unwrap()
    }
}
