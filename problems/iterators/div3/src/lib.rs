#![forbid(unsafe_code)]

pub fn div3() -> impl Iterator<Item = i32> {
    (1..).into_iter().filter(|x| x % 3 == 0)
}
