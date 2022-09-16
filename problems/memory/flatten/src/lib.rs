#![forbid(unsafe_code)]

use core::panic;
use std::convert::TryInto;

pub fn flatten<const N: usize>(data: Vec<Box<[&mut i32; N]>>) -> Vec<&mut i32> {
    let mut ans_vec: Vec<&mut i32> = vec![];

    for i in data {
        for j in i.into_iter() {
            ans_vec.push(j);
        }
    }

    ans_vec
}

pub fn transform_to_fixed_arr<const N: usize>(data: &mut Vec<Vec<i32>>) -> Vec<Box<[&mut i32; N]>> {
    for val in data.iter() {
        if val.len() != N {
            panic!("Inner vectors are of different size");
        }
    }

    let mut ans_vec: Vec<Box<[&mut i32; N]>> = vec![];

    for cur_vec in data.iter_mut() {
        let mut tmp_vec: Vec<&mut i32> = vec![];

        for cur_value in cur_vec.iter_mut() {
            tmp_vec.push(cur_value);
        }

        let tmp_arr: [&mut i32; N] = tmp_vec.try_into().unwrap();
        ans_vec.push(Box::new(tmp_arr));
    }

    ans_vec
}
