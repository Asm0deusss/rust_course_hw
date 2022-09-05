#![forbid(unsafe_code)]

pub fn add(x: i32, y: i32) -> i32 {
    let x_copy = x as i64;
    let y_copy = y as i64;
    let ans = x_copy + y_copy;
    let int_mx: i64 = i32::MAX as i64;
    let int_mn: i64 = i32::MIN as i64;
    
    if ans > int_mx {
        i32::MAX
    } else if ans < int_mn {
        i32::MIN
    } else {
        ans as i32
    }
}
