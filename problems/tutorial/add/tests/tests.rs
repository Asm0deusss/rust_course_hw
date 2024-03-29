use add::add;

#[test]
fn it_works() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(1, 0), 1);
    assert_eq!(add(0, 1), 1);
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(4, 7), 11);
    assert_eq!(add(i32::MIN, i32::MAX), -1);
    assert_eq!(add(add(i32::MIN, i32::MAX), i32::MAX), add(i32::MAX, -1));
}

#[test]
fn saturating_add() {
    assert_eq!(i32::MAX, add(i32::MAX, 1));
    assert_eq!(i32::MIN, add(i32::MIN, -1));
}
