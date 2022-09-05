#![forbid(unsafe_code)]

pub fn get_nth_fibonacci(n: u32) -> u32 {
    let mut left = 0;
    let mut right = 1;
    let mut counter = 0;

    while counter < n {
        let tmp = left;
        left = right;
        right += tmp;
        counter += 1;
    }

    left
}
