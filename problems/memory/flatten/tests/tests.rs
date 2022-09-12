use flatten::{flatten, transform_to_fixed_arr};

fn flatten_arr_ok(before: &Vec<Vec<i32>>, after: &Vec<&mut i32>) -> bool {
    for i in 0..before.len() {
        for j in 0..before[i].len() {
            if before[i][j] != *after[i * before[i].len() + j] {
                return false;
            }
        }
    }
    true
}

#[test]
#[allow(unused_mut)]
fn it_works() {
    let mut v = vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![10, 3, 5, 2]];
    let mut v_ref = v.clone();
    let mut x = transform_to_fixed_arr::<4>(&mut v);
    let flatten = flatten(x);
    assert!(flatten_arr_ok(&v_ref, &flatten));
}

#[test]
#[should_panic(expected = "Inner vectors are of different size")]
fn test_empty_insert() {
    let mut v = vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4, 5]];
    let _ = transform_to_fixed_arr::<4>(&mut v);
}

#[test]
fn test_big_arrays() {
    macro_rules! big_array_test {
        ($inner_size:literal, $outer_size:literal) => {
            let mut v = vec![vec![0; $inner_size]; $outer_size];
            let v_ref = v.clone();
            let x = transform_to_fixed_arr::<{ $inner_size }>(&mut v);
            assert!(flatten_arr_ok(&v_ref, &flatten(x)));
        };
    }

    big_array_test!(1000, 10);
    big_array_test!(100, 100);
    big_array_test!(10, 1000);
}
