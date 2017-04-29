use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use test::Bencher;

#[bench]
fn bench_native(b: &mut Bencher) {
    b.iter(|| native::Natural::default());
}

#[bench]
fn bench_gmp(b: &mut Bencher) {
    b.iter(|| gmp::Natural::default());
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| num::BigUint::default());
}
