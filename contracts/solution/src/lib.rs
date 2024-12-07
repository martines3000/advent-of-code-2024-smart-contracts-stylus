extern crate alloc;

use alloc::string::String;

pub trait Solution {
    fn solvepart1(&self, input: String) -> i64;
    fn solvepart2(&self, input: String) -> i64;
}
