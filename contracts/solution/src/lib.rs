extern crate alloc;

use alloc::string::String;

pub trait Solution {
    fn solvepart1(&self, input: String) -> u32;
    fn solvepart2(&self, input: String) -> u32;
}
