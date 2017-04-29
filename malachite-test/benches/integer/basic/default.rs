use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;
use test::Bencher;

#[bench]
fn bench_native(b: &mut Bencher) {
    b.iter(|| native::Integer::default());
}

#[bench]
fn bench_gmp(b: &mut Bencher) {
    b.iter(|| gmp::Integer::default());
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| num::BigInt::default());
}

#[bench]
fn bench_rugint(b: &mut Bencher) {
    b.iter(|| rugint::Integer::default());
}
