use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::natural::comparison::partial_eq_u32::num_partial_eq_u32;
use num;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bench_native_small_small(b: &mut Bencher) {
    let x = native::Natural::from(123);
    b.iter(|| x == 0);
}

#[bench]
fn bench_gmp_small_small(b: &mut Bencher) {
    let x = gmp::Natural::from(123);
    b.iter(|| x == 0);
}

#[bench]
fn bench_num_small_small(b: &mut Bencher) {
    let x = num::BigUint::from(123u64);
    b.iter(|| num_partial_eq_u32(&x, 0));
}

#[bench]
fn bench_native_small_large(b: &mut Bencher) {
    let x = native::Natural::from(123);
    let u = u32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_gmp_small_large(b: &mut Bencher) {
    let x = gmp::Natural::from(123);
    let u = u32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_num_small_large(b: &mut Bencher) {
    let x = num::BigUint::from(123u64);
    let u = u32::max_value();
    b.iter(|| num_partial_eq_u32(&x, u));
}

#[bench]
fn bench_native_large_small(b: &mut Bencher) {
    let x = native::Natural::from_str("1000000000000").unwrap();
    b.iter(|| x == 123);
}

#[bench]
fn bench_gmp_large_small(b: &mut Bencher) {
    let x = gmp::Natural::from_str("1000000000000").unwrap();
    b.iter(|| x == 123);
}

#[bench]
fn bench_num_large_small(b: &mut Bencher) {
    let x = num::BigUint::from_str("1000000000000").unwrap();
    b.iter(|| num_partial_eq_u32(&x, 123));
}

#[bench]
fn bench_native_large_large(b: &mut Bencher) {
    let x = native::Natural::from_str("1000000000000").unwrap();
    let u = u32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_gmp_large_large(b: &mut Bencher) {
    let x = gmp::Natural::from_str("1000000000000").unwrap();
    let u = u32::max_value();
    b.iter(|| x == u);
}

#[bench]
fn bench_num_large_large(b: &mut Bencher) {
    let x = num::BigUint::from_str("1000000000000").unwrap();
    let u = u32::max_value();
    b.iter(|| num_partial_eq_u32(&x, u));
}
