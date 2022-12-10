// use adventofcode2022 crate 
use adventofcode2022::day1 as d1;
use adventofcode2022::day2 as d2;
use adventofcode2022::day3 as d3;
use adventofcode2022::day4 as d4;
use adventofcode2022::day5 as d5;
use adventofcode2022::day6 as d6;
use adventofcode2022::day7 as d7;
use adventofcode2022::day8 as d8;
use adventofcode2022::day9 as d9;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// include string from file at compile time
const INPUT1: &str = include_str!("../inputs/day1.txt");
const INPUT2: &str = include_str!("../inputs/day2.txt");
const INPUT3: &str = include_str!("../inputs/day3.txt");
const INPUT4: &str = include_str!("../inputs/day4.txt");
const INPUT5: &str = include_str!("../inputs/day5.txt");
const INPUT6: &str = include_str!("../inputs/day6.txt");
const INPUT7: &str = include_str!("../inputs/day7.txt");
const INPUT8: &str = include_str!("../inputs/day8.txt");
const INPUT9: &str = include_str!("../inputs/day9.txt");

// benchmark functions
fn bench_day1(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| d1::run(black_box(INPUT1.to_string()))));

}

fn bench_day2(c: &mut Criterion) {
    c.bench_function("day2", |b| b.iter(|| d2::run(black_box(INPUT2.to_string()))));

}

fn bench_day3(c: &mut Criterion) {
    c.bench_function("day3", |b| b.iter(|| d3::run(black_box(INPUT3.to_string()))));

}

fn bench_day4(c: &mut Criterion) {
    c.bench_function("day4", |b| b.iter(|| d4::run(black_box(INPUT4.to_string()))));

}

fn bench_day5(c: &mut Criterion) {
    c.bench_function("day5", |b| b.iter(|| d5::run(black_box(INPUT5.to_string()))));

}

fn bench_day6(c: &mut Criterion) {
    c.bench_function("day6", |b| b.iter(|| d6::run(black_box(INPUT6.to_string()))));

}

fn bench_day7(c: &mut Criterion) {
    c.bench_function("day7", |b| b.iter(|| d7::run(black_box(INPUT7.to_string()))));

}

fn bench_day8(c: &mut Criterion) {
    c.bench_function("day8", |b| b.iter(|| d8::run(black_box(INPUT8.to_string()))));

}

fn bench_day9(c: &mut Criterion) {
    c.bench_function("day9", |b| b.iter(|| d9::run(black_box(INPUT9.to_string()))));

}

// benchmark group
criterion_group!(
    benches,
    bench_day1,
    bench_day2,
    bench_day3,
    bench_day4,
    bench_day5,
    bench_day6,
    bench_day7,
    bench_day8,
    bench_day9,
);
criterion_main!(benches);

