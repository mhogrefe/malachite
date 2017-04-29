use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;
use test::Bencher;

#[bench]
fn bench_native_0(b: &mut Bencher) {
    b.iter(|| native::Integer::from(0i32));
}

#[bench]
fn bench_gmp_0(b: &mut Bencher) {
    b.iter(|| gmp::Integer::from(0i32));
}

#[bench]
fn bench_num_0(b: &mut Bencher) {
    b.iter(|| num::BigInt::from(0u64));
}

#[bench]
fn bench_rugint_0(b: &mut Bencher) {
    b.iter(|| rugint::Integer::from(0i32));
}

#[bench]
fn bench_native_small(b: &mut Bencher) {
    b.iter(|| native::Integer::from(-123i32));
}

#[bench]
fn bench_gmp_small(b: &mut Bencher) {
    b.iter(|| gmp::Integer::from(-123i32));
}

#[bench]
fn bench_num_small(b: &mut Bencher) {
    b.iter(|| num::BigInt::from(-123i64));
}

#[bench]
fn bench_rugint_small(b: &mut Bencher) {
    b.iter(|| rugint::Integer::from(123i32));
}

#[bench]
fn bench_native_large_positive(b: &mut Bencher) {
    let u = i32::max_value();
    b.iter(|| native::Integer::from(u));
}

#[bench]
fn bench_gmp_large_positive(b: &mut Bencher) {
    let u = i32::max_value();
    b.iter(|| gmp::Integer::from(u));
}

#[bench]
fn bench_num_large_positive(b: &mut Bencher) {
    let u = i32::max_value() as i64;
    b.iter(|| num::BigInt::from(u));
}

#[bench]
fn bench_rugint_large_positive(b: &mut Bencher) {
    let u = i32::max_value();
    b.iter(|| rugint::Integer::from(u));
}

#[bench]
fn bench_native_large_negative(b: &mut Bencher) {
    let u = i32::min_value();
    b.iter(|| native::Integer::from(u));
}

#[bench]
fn bench_gmp_large_negative(b: &mut Bencher) {
    let u = i32::min_value();
    b.iter(|| gmp::Integer::from(u));
}

#[bench]
fn bench_num_large_negative(b: &mut Bencher) {
    let u = i32::min_value() as i64;
    b.iter(|| num::BigInt::from(u));
}

#[bench]
fn bench_rugint_large_negative(b: &mut Bencher) {
    let u = i32::min_value();
    b.iter(|| rugint::Integer::from(u));
}
