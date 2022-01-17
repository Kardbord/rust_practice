extern crate rust_practice;

use rust_practice::fizzbuzz;

#[test]
fn is_fizz_test() {
    assert!(fizzbuzz::is_fizz(0));
    assert!(!fizzbuzz::is_fizz(1));
    assert!(!fizzbuzz::is_fizz(2));
    assert!(fizzbuzz::is_fizz(3));
    assert!(!fizzbuzz::is_fizz(4));
    assert!(!fizzbuzz::is_fizz(5));
    assert!(fizzbuzz::is_fizz(6));
    assert!(fizzbuzz::is_fizz(15));
}

#[test]
fn is_buzz_test() {
    assert!(fizzbuzz::is_buzz(0));
    assert!(!fizzbuzz::is_buzz(1));
    assert!(!fizzbuzz::is_buzz(2));
    assert!(!fizzbuzz::is_buzz(3));
    assert!(!fizzbuzz::is_buzz(4));
    assert!(fizzbuzz::is_buzz(5));
    assert!(fizzbuzz::is_buzz(10));
    assert!(fizzbuzz::is_buzz(15));
    assert!(fizzbuzz::is_buzz(20));
}

#[test]
fn is_fizzbuzz_test() {
    assert!(fizzbuzz::is_fizzbuzz(0));
    assert!(!fizzbuzz::is_fizzbuzz(1));
    assert!(!fizzbuzz::is_fizzbuzz(2));
    assert!(!fizzbuzz::is_fizzbuzz(3));
    assert!(!fizzbuzz::is_fizzbuzz(4));
    assert!(!fizzbuzz::is_fizzbuzz(5));
    assert!(!fizzbuzz::is_fizzbuzz(6));
    assert!(!fizzbuzz::is_fizzbuzz(7));
    assert!(!fizzbuzz::is_fizzbuzz(8));
    assert!(!fizzbuzz::is_fizzbuzz(9));
    assert!(!fizzbuzz::is_fizzbuzz(10));
    assert!(!fizzbuzz::is_fizzbuzz(11));
    assert!(!fizzbuzz::is_fizzbuzz(12));
    assert!(!fizzbuzz::is_fizzbuzz(13));
    assert!(!fizzbuzz::is_fizzbuzz(14));
    assert!(fizzbuzz::is_fizzbuzz(15));
}

#[test]
fn fizzbuzz_test() {
    {
        let expected = String::from("fizzbuzz\n1\n2\nfizz\n4\nbuzz\nfizz\n");
        assert_eq!(expected, fizzbuzz::fizzbuzz(0..7));
    }
    {
        let expected = String::from("fizz\n13\n14\nfizzbuzz\n16\n17\nfizz\n19\nbuzz\n");
        assert_eq!(expected, fizzbuzz::fizzbuzz(12..21));
    }
}
