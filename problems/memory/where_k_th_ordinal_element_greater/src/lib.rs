#![forbid(unsafe_code)]

pub fn where_k_th_ordinal_element_greater<'a>(
    lhs: &'a Vec<i32>,
    rhs: &'a Vec<i32>,
    k: usize,
) -> &'a Vec<i32> {
    let mut left_copy = lhs.clone();
    let mut right_copy = rhs.clone();

    left_copy.sort();
    right_copy.sort();

    let left_stat = left_copy[k];
    let right_stat = right_copy[k];

    if left_stat <= right_stat {
        rhs
    } else {
        lhs
    }
}
