#![forbid(unsafe_code)]

pub fn add2(iterator: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    iterator.map(|x| x + 2)
}
