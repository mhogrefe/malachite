use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use test::Bencher;

#[bench]
fn bench_native_0(b: &mut Bencher) {
    b.iter(|| native::Natural::from(0));
}

#[bench]
fn bench_gmp_0(b: &mut Bencher) {
    b.iter(|| gmp::Natural::from(0));
}

#[bench]
fn bench_num_0(b: &mut Bencher) {
    b.iter(|| num::BigUint::from(0u64));
}

#[bench]
fn bench_native_small(b: &mut Bencher) {
    b.iter(|| native::Natural::from(123));
}

#[bench]
fn bench_gmp_small(b: &mut Bencher) {
    b.iter(|| gmp::Natural::from(123));
}

#[bench]
fn bench_num_small(b: &mut Bencher) {
    b.iter(|| num::BigUint::from(123u64));
}

#[bench]
fn bench_native_large(b: &mut Bencher) {
    let u = u32::max_value();
    b.iter(|| native::Natural::from(u));
}

#[bench]
fn bench_gmp_large(b: &mut Bencher) {
    let u = u32::max_value();
    b.iter(|| gmp::Natural::from(u));
}

#[bench]
fn bench_num_large(b: &mut Bencher) {
    let u = u32::max_value() as u64;
    b.iter(|| num::BigUint::from(u));
}
