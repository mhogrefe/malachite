use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use malachite_test::common::gmp_integer_to_native;
use malachite_test::integer::conversion::assign_u64::num_assign_u64;
use num;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_assign_u64() {
    let test = |u, v: u64, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = num::BigInt::from_str(u).unwrap();
        num_assign_u64(&mut x, v);
        assert_eq!(x.to_string(), out);
    };
    test("-123", 456, "456");
    test("123", u32::max_value().into(), "4294967295");
    test("123", u64::max_value(), "18446744073709551615");
    test("1000000000000000000000000", 123, "123");
}

#[test]
fn assign_u64_properties() {
    // n.assign(u) is equivalent for malachite-gmp and malachite-native.
    // n.assign(u) is valid.
    // n.assign(u); n == u
    // n.assign(Integer::from(u)) is equivalent to n.assign(u)
    let integer_and_u64 = |mut gmp_n: gmp::Integer, u: u64| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign(u);
        assert!(gmp_n.is_valid());
        assert_eq!(gmp_n, gmp::Integer::from(u));
        n.assign(u);
        assert!(n.is_valid());
        assert_eq!(n, native::Integer::from(u));
        let mut alt_n = old_n.clone();
        //TODO assign by value
        alt_n.assign(&native::Integer::from(u));
        assert_eq!(alt_n, n);
    };

    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(LARGE_LIMIT) {
        integer_and_u64(n, u);
    }

    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_integers(seed, 32)),
                               &(|seed| random_x::<u64>(seed)))
                .take(LARGE_LIMIT) {
        integer_and_u64(n, u);
    }
}
