extern crate alloc;

use alloc::string::String;
use alloc::vec;
use ndarray::{s, stack, Array1, Axis};
use solution::Solution;
use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Day24 {}

#[public]
impl Solution for Day24 {
    fn solvepart1(&self, input: String) -> i64 {
        let initial_data: Vec<i64> = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut data_vec = Vec::new();
        data_vec.push(Array1::from(initial_data));

        // Generate sequences
        for _ in 0..2000 {
            let seq = &data_vec[data_vec.len() - 1];
            let mut new_seq = (seq * 64) ^ seq;
            new_seq.mapv_inplace(|x| x % 16777216);
            let mut temp = (&new_seq / 32) ^ &new_seq;
            temp.mapv_inplace(|x| x % 16777216);
            let mut final_seq = (&temp * 2048) ^ &temp;
            final_seq.mapv_inplace(|x| x % 16777216);
            data_vec.push(final_seq);
        }

        let data = stack(
            Axis(1),
            &data_vec.iter().map(|x| x.view()).collect::<Vec<_>>(),
        )
        .unwrap();

        let result = data.slice(s![.., -1]).sum();

        result.try_into().unwrap()
    }

    fn solvepart2(&self, input: String) -> i64 {
        let initial_data: Vec<i64> = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        // Convert initial data to Array1
        let mut data_vec = Vec::new();
        data_vec.push(Array1::from(initial_data));

        let data = stack(
            Axis(1),
            &data_vec.iter().map(|x| x.view()).collect::<Vec<_>>(),
        )
        .unwrap();

        let data = data.mapv(|x| x % 10);

        // Calculate diffs
        let diff = (&data.slice(s![.., 1..]) - &data.slice(s![.., ..-1]) + 10).to_owned();

        // Calculate key
        let n = diff.shape()[1];
        let key = &diff.slice(s![.., 3..]) * 8000
            + &diff.slice(s![.., 2..n - 1]) * 400
            + &diff.slice(s![.., 1..n - 2]) * 20
            + &diff.slice(s![.., ..n - 3]);

        // Results
        let mut dd = Array1::<i16>::zeros(160000);

        // Process each row
        for j in 0..data.shape()[0] {
            let mut s = Array1::<i16>::zeros(160000);

            let row_key = key.slice(s![j, ..]).to_vec();
            let row_data = data.slice(s![j, 4..]).to_vec();

            // Process in reverse order
            for (_, (&k, &d)) in row_key.iter().rev().zip(row_data.iter().rev()).enumerate() {
                if k < 160000 {
                    s[k as usize] = d as i16;
                }
            }

            dd += &s;
        }

        let result = *dd.iter().max().unwrap();

        result.try_into().unwrap()
    }
}
