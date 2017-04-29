use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num::{self, Zero};
use test::Bencher;

#[bench]
fn bench_native(b: &mut Bencher) {
    b.iter(|| native::Natural::new());
}

#[bench]
fn bench_gmp(b: &mut Bencher) {
    b.iter(|| gmp::Natural::new());
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| num::BigUint::zero());
}
