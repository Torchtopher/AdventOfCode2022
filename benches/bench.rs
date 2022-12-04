// use adventofcode2022 crate 
use adventofcode2022::day1 as d1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

1

