#![feature(test)]

extern crate test;

use algoritmer::math::*;

use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 10;

#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    b.iter(|| {
        (0..BENCH_SIZE)
            .map(Fibonacci::recursive)
            .collect::<Vec<_>>()
    });
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| {
        (0..BENCH_SIZE)
            .map(Fibonacci::iterative)
            .collect::<Vec<_>>()
    })
}

#[bench]
fn memoized_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(Fibonacci::memoized).collect::<Vec<_>>())
}

#[bench]
fn analytic_fibonacci(b: &mut Bencher) {
    b.iter(|| (0..BENCH_SIZE).map(Fibonacci::analytic).collect::<Vec<_>>())
}
