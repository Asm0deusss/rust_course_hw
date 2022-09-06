#![forbid(unsafe_code)]

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn get_n_prime_numbers(n: u32) -> Vec<u32> {
    let mut ans = vec![];

    let mut i = 2;

    let mut count = 0;

    while count < n {
        if is_prime(i) {
            ans.push(i);
            count += 1;
        }
        i += 1;
    }

    ans
}
