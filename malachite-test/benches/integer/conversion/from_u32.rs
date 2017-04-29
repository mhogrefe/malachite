use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;
use test::Bencher;

#[bench]
fn bench_native_0(b: &mut Bencher) {
    b.iter(|| native::Integer::from(0u32));
}

#[bench]
fn bench_gmp_0(b: &mut Bencher) {
    b.iter(|| gmp::Integer::from(0u32));
}

#[bench]
fn bench_num_0(b: &mut Bencher) {
    b.iter(|| num::BigInt::from(0u64));
}

#[bench]
fn bench_rugint_0(b: &mut Bencher) {
    b.iter(|| rugint::Integer::from(0));
}

#[bench]
fn bench_native_small(b: &mut Bencher) {
    b.iter(|| native::Integer::from(123u32));
}

#[bench]
fn bench_gmp_small(b: &mut Bencher) {
    b.iter(|| gmp::Integer::from(123u32));
}

#[bench]
fn bench_num_small(b: &mut Bencher) {
    b.iter(|| num::BigInt::from(123u64));
}

#[bench]
fn bench_rugint_small(b: &mut Bencher) {
    b.iter(|| rugint::Integer::from(123u32));
}

#[bench]
fn bench_native_large(b: &mut Bencher) {
    let u = u32::max_value();
    b.iter(|| native::Integer::from(u));
}

#[bench]
fn bench_gmp_large(b: &mut Bencher) {
    let u = u32::max_value();
    b.iter(|| gmp::Integer::from(u));
}

#[bench]
fn bench_num_large(b: &mut Bencher) {
    let u = u32::max_value() as u64;
    b.iter(|| num::BigInt::from(u));
}

#[bench]
fn bench_rugint_large(b: &mut Bencher) {
    let u = u32::max_value();
    b.iter(|| rugint::Integer::from(u));
}
