use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num::{self, Zero};
use rugint;
use test::Bencher;

#[bench]
fn bench_native(b: &mut Bencher) {
    b.iter(|| native::Integer::new());
}

#[bench]
fn bench_gmp(b: &mut Bencher) {
    b.iter(|| gmp::Integer::new());
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| num::BigInt::zero());
}

#[bench]
fn bench_rugint(b: &mut Bencher) {
    b.iter(|| rugint::Integer::new());
}
