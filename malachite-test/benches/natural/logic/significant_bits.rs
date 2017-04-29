use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num::{self, Zero};
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bench_native_0(b: &mut Bencher) {
    let x = native::Natural::new();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_0(b: &mut Bencher) {
    let x = gmp::Natural::new();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_0(b: &mut Bencher) {
    let x = num::BigUint::zero();
    b.iter(|| x.bits());
}

#[bench]
fn bench_native_small(b: &mut Bencher) {
    let x = native::Natural::from(123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_small(b: &mut Bencher) {
    let x = gmp::Natural::from(123);
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_small(b: &mut Bencher) {
    let x = num::BigUint::from(123u64);
    b.iter(|| x.bits());
}

#[bench]
fn bench_native_large(b: &mut Bencher) {
    let x = native::Natural::from_str("1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_gmp_large(b: &mut Bencher) {
    let x = gmp::Natural::from_str("1000000000000").unwrap();
    b.iter(|| x.significant_bits());
}

#[bench]
fn bench_num_large(b: &mut Bencher) {
    let x = num::BigUint::from_str("1000000000000").unwrap();
    b.iter(|| x.bits());
}
