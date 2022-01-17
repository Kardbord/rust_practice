extern crate rust_practice;

use rust_practice::fibonacci;

#[test]
fn recursive_fibonacci_test() {
    assert_eq!(0, fibonacci::compute(0, fibonacci::Method::Recursive));
    assert_eq!(1, fibonacci::compute(1, fibonacci::Method::Recursive));
    assert_eq!(1, fibonacci::compute(2, fibonacci::Method::Recursive));
    assert_eq!(2, fibonacci::compute(3, fibonacci::Method::Recursive));
    assert_eq!(3, fibonacci::compute(4, fibonacci::Method::Recursive));
    assert_eq!(5, fibonacci::compute(5, fibonacci::Method::Recursive));
    assert_eq!(8, fibonacci::compute(6, fibonacci::Method::Recursive));
    assert_eq!(13, fibonacci::compute(7, fibonacci::Method::Recursive));
    assert_eq!(21, fibonacci::compute(8, fibonacci::Method::Recursive));
    assert_eq!(34, fibonacci::compute(9, fibonacci::Method::Recursive));
    assert_eq!(55, fibonacci::compute(10, fibonacci::Method::Recursive));
    assert_eq!(89, fibonacci::compute(11, fibonacci::Method::Recursive));
    assert_eq!(144, fibonacci::compute(12, fibonacci::Method::Recursive));
    assert_eq!(233, fibonacci::compute(13, fibonacci::Method::Recursive));
    assert_eq!(377, fibonacci::compute(14, fibonacci::Method::Recursive));
    assert_eq!(610, fibonacci::compute(15, fibonacci::Method::Recursive));
}

#[test]
fn dynamic_fibonacci_test() {
    assert_eq!(0, fibonacci::compute(0, fibonacci::Method::Dynamic));
    assert_eq!(1, fibonacci::compute(1, fibonacci::Method::Dynamic));
    assert_eq!(1, fibonacci::compute(2, fibonacci::Method::Dynamic));
    assert_eq!(2, fibonacci::compute(3, fibonacci::Method::Dynamic));
    assert_eq!(3, fibonacci::compute(4, fibonacci::Method::Dynamic));
    assert_eq!(5, fibonacci::compute(5, fibonacci::Method::Dynamic));
    assert_eq!(8, fibonacci::compute(6, fibonacci::Method::Dynamic));
    assert_eq!(13, fibonacci::compute(7, fibonacci::Method::Dynamic));
    assert_eq!(21, fibonacci::compute(8, fibonacci::Method::Dynamic));
    assert_eq!(34, fibonacci::compute(9, fibonacci::Method::Dynamic));
    assert_eq!(55, fibonacci::compute(10, fibonacci::Method::Dynamic));
    assert_eq!(89, fibonacci::compute(11, fibonacci::Method::Dynamic));
    assert_eq!(144, fibonacci::compute(12, fibonacci::Method::Dynamic));
    assert_eq!(233, fibonacci::compute(13, fibonacci::Method::Dynamic));
    assert_eq!(377, fibonacci::compute(14, fibonacci::Method::Dynamic));
    assert_eq!(610, fibonacci::compute(15, fibonacci::Method::Dynamic));
}
