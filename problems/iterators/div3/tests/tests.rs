use div3::div3;

#[test]
fn it_works() {
    let it = div3();
    let v: Vec<i32> = it.take(10).collect();
    assert_eq!(v, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]);
}
