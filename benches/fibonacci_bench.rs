#![feature(test)]

extern crate rust_practice;
extern crate test;

use rust_practice::fibonacci;
use test::Bencher;

const SMALL_FIBONACCI_INDEX: u32 = 10;
const MEDIUM_FIBONACCI_INDEX: u32 = 20;
const LARGE_FIBONACCI_INDEX: u32 = 40;

#[bench]
fn recursive_fibonacci_small_bench(b: &mut Bencher) {
    b.iter(|| fibonacci::compute(SMALL_FIBONACCI_INDEX, fibonacci::Method::Recursive));
}

#[bench]
fn dynamic_fibonacci_small_bench(b: &mut Bencher) {
    b.iter(|| fibonacci::compute(SMALL_FIBONACCI_INDEX, fibonacci::Method::Dynamic));
}

#[bench]
fn recursive_fibonacci_medium_bench(b: &mut Bencher) {
    b.iter(|| fibonacci::compute(MEDIUM_FIBONACCI_INDEX, fibonacci::Method::Recursive));
}

#[bench]
fn dynamic_fibonacci_medium_bench(b: &mut Bencher) {
    b.iter(|| fibonacci::compute(MEDIUM_FIBONACCI_INDEX, fibonacci::Method::Dynamic));
}

#[bench]
fn recursive_fibonacci_large_bench(b: &mut Bencher) {
    b.iter(|| fibonacci::compute(LARGE_FIBONACCI_INDEX, fibonacci::Method::Recursive));
}

#[bench]
fn dynamic_fibonacci_large_bench(b: &mut Bencher) {
    b.iter(|| fibonacci::compute(LARGE_FIBONACCI_INDEX, fibonacci::Method::Dynamic));
}