#![forbid(unsafe_code)]

pub fn take_n(iterator: impl Iterator<Item = i32>, n: usize) -> Vec<i32> {
    iterator.take(n).collect::<Vec<i32>>()
}
