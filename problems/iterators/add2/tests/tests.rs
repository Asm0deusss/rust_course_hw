use add2::add2;

#[test]
fn numbers() {
    let it = add2(1..);
    let v: Vec<i32> = it.take(10).collect();
    assert_eq!(v, vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn vector() {
    let it = add2(vec![10, 20, 30, 40].into_iter());
    let v: Vec<i32> = it.take(4).collect();
    assert_eq!(v, vec![12, 22, 32, 42]);
}
