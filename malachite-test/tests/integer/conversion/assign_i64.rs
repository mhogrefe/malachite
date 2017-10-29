use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_assign_i64() {
    let test = |u, v: i64, out| {
        let mut x = native::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("123", -456, "-456");
    test("-123", i32::max_value().into(), "2147483647");
    test("123", i32::min_value().into(), "-2147483648");
    test("-123", i64::max_value(), "9223372036854775807");
    test("123", i64::min_value(), "-9223372036854775808");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_i64_properties() {
    // n.assign(i) is equivalent for malachite-gmp and malachite-native.
    // n.assign(i) is valid.
    // n.assign(i); n == u
    // n.assign(Integer::from(i)) is equivalent to n.assign(i)
    let integer_and_i64 = |mut gmp_n: gmp::Integer, i: i64| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign(i);
        assert!(gmp_n.is_valid());
        assert_eq!(gmp_n, gmp::Integer::from(i));
        n.assign(i);
        assert!(n.is_valid());
        assert_eq!(n, native::Integer::from(i));
        let mut alt_n = old_n.clone();
        alt_n.assign(native::Integer::from(i));
        assert_eq!(alt_n, n);
    };

    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i64>()).take(LARGE_LIMIT) {
        integer_and_i64(n, i);
    }

    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i64>(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i64(n, i);
    }
}
