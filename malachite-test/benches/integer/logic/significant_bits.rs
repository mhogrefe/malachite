use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num::{self, Zero};
use rugint;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bench_native_0(b: &mut Bencher) {
    let x = native::Integer::new();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_0(b: &mut Bencher) {
    let x = gmp::Integer::new();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_0(b: &mut Bencher) {
    let x = num::BigInt::zero();
    b.iter(|| x.bits());
}

#[bench]
fn bench_rugint_0(b: &mut Bencher) {
    let x = rugint::Integer::new();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_native_small_positive(b: &mut Bencher) {
    let x = native::Integer::from(123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_small_positive(b: &mut Bencher) {
    let x = gmp::Integer::from(123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_small_positive(b: &mut Bencher) {
    let x = num::BigInt::from(123);
    b.iter(|| x.bits());
}

#[bench]
fn bench_rugint_small_positive(b: &mut Bencher) {
    let x = rugint::Integer::from(123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_native_small_negative(b: &mut Bencher) {
    let x = native::Integer::from(-123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_small_negative(b: &mut Bencher) {
    let x = gmp::Integer::from(-123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_small_negative(b: &mut Bencher) {
    let x = num::BigInt::from(-123);
    b.iter(|| x.bits());
}

#[bench]
fn bench_rugint_small_negative(b: &mut Bencher) {
    let x = rugint::Integer::from(-123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_native_large_positive(b: &mut Bencher) {
    let x = native::Integer::from_str("1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_large_positive(b: &mut Bencher) {
    let x = gmp::Integer::from_str("1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_large_positive(b: &mut Bencher) {
    let x = num::BigInt::from_str("1000000000000").unwrap();
    b.iter(|| x.bits());
}

#[bench]
fn bench_rugint_large_positive(b: &mut Bencher) {
    let x = rugint::Integer::from_str("1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_native_large_negative(b: &mut Bencher) {
    let x = native::Integer::from_str("-1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_large_negative(b: &mut Bencher) {
    let x = gmp::Integer::from_str("-1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_large_negative(b: &mut Bencher) {
    let x = num::BigInt::from_str("-1000000000000").unwrap();
    b.iter(|| x.bits());
}

#[bench]
fn bench_rugint_large_negative(b: &mut Bencher) {
    let x = rugint::Integer::from_str("-1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}
