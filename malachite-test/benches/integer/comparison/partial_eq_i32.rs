use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::integer::comparison::partial_eq_i32::num_partial_eq_i32;
use num;
use rugint;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bench_native_small_small(b: &mut Bencher) {
    let x = native::Integer::from(123);
    b.iter(|| x == 0i32);
}

#[bench]
fn bench_gmp_small_small(b: &mut Bencher) {
    let x = gmp::Integer::from(123);
    b.iter(|| x == 0i32);
}

#[bench]
fn bench_num_small_small(b: &mut Bencher) {
    let x = num::BigInt::from(123u64);
    b.iter(|| num_partial_eq_i32(&x, 0i32));
}

#[bench]
fn bench_rugint_small_small(b: &mut Bencher) {
    let x = rugint::Integer::from(123);
    b.iter(|| x == 0i32);
}

#[bench]
fn bench_native_small_large(b: &mut Bencher) {
    let x = native::Integer::from(123);
    let u = i32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_gmp_small_large(b: &mut Bencher) {
    let x = gmp::Integer::from(123);
    let u = i32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_num_small_large(b: &mut Bencher) {
    let x = num::BigInt::from(123u64);
    let u = i32::max_value();
    b.iter(|| num_partial_eq_i32(&x, u));
}

#[bench]
fn bench_rugint_small_large(b: &mut Bencher) {
    let x = rugint::Integer::from(123);
    let u = i32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_native_large_small(b: &mut Bencher) {
    let x = native::Integer::from_str("1000000000000").unwrap();
    b.iter(|| x == 123i32);
}

#[bench]
fn bench_gmp_large_small(b: &mut Bencher) {
    let x = gmp::Integer::from_str("1000000000000").unwrap();
    b.iter(|| x == 123i32);
}

#[bench]
fn bench_num_large_small(b: &mut Bencher) {
    let x = num::BigInt::from_str("1000000000000").unwrap();
    b.iter(|| num_partial_eq_i32(&x, 123i32));
}

#[bench]
fn bench_native_large_large(b: &mut Bencher) {
    let x = native::Integer::from_str("1000000000000").unwrap();
    let u = i32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_gmp_large_large(b: &mut Bencher) {
    let x = gmp::Integer::from_str("1000000000000").unwrap();
    let u = i32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_num_large_large(b: &mut Bencher) {
    let x = num::BigInt::from_str("1000000000000").unwrap();
    let u = i32::max_value();
    b.iter(|| num_partial_eq_i32(&x, u));
}

#[bench]
fn bench_rugint_large_large(b: &mut Bencher) {
    let x = rugint::Integer::from_str("1000000000000").unwrap();
    let u = i32::max_value();
    b.iter(|| x == u);
}
