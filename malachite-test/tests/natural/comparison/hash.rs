use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::gmp_natural_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn native_hash(n: &native::Natural) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

fn gmp_hash(n: &gmp::Natural) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

#[test]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        assert_eq!(native_hash(&x), native_hash(&x.clone()));
        assert_eq!(gmp_hash(&gmp_x), gmp_hash(&gmp_x.clone()));
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
