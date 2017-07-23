use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn native_hash(n: &native::Integer) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

fn gmp_hash(n: &gmp::Integer) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

#[test]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        assert_eq!(native_hash(&x), native_hash(&x.clone()));
        assert_eq!(gmp_hash(&gmp_x), gmp_hash(&gmp_x.clone()));
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
