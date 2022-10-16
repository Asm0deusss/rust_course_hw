use take_n::take_n;

#[test]
fn it_works() {
    let v = take_n(1.., 5);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}

#[test]
fn it_works2() {
    let v = take_n(vec![10, 20, 30, 40, 50, 60].into_iter(), 5);
    assert_eq!(v, vec![10, 20, 30, 40, 50]);
}
