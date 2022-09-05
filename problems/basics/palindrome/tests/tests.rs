use palindrome::is_palindrome;

#[test]
fn it_works() {
    assert_eq!(is_palindrome(0), true);
    assert_eq!(is_palindrome(9), true);
    assert_eq!(is_palindrome(12), false);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(4294967295), false);
    assert_eq!(is_palindrome(1234554321), true);
}
