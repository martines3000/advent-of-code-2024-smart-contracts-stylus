extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day4 {}

#[public]
impl Solution for Day4 {
    fn solvepart1(&self, input: String) -> u32 {
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

    fn solvepart2(&self, input: String) -> u32 {
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
}
