extern crate alloc;

use alloc::string::String;
use alloc::vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day25 {}

#[public]
impl Solution for Day25 {
    fn solvepart1(&self, input: String) -> i64 {
        let result = 0;
        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let result = 0;
        result.try_into().unwrap()
    }
}
