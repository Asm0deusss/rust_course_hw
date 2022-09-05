#![forbid(unsafe_code)]

pub fn is_palindrome(number: u32) -> bool {
    let mut digits = vec![];
    let mut number_copy = number;

    if number == 0 {
        return true;
    }

    while number_copy > 0 {
        digits.push(number_copy % 10);
        number_copy /= 10;
    }

    let mut left = 0;
    let mut right = digits.len() - 1;

    let mut ans = true;

    while left < right {
        if digits[left] != digits[right] {
            ans = false;
            break;
        } else {
            left += 1;
            right -= 1;
        }
    }

    ans
}
