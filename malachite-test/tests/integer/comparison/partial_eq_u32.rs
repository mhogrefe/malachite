use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint};
use malachite_test::integer::comparison::partial_eq_u32::num_partial_eq_u32;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap() == v, out);
        assert_eq!(gmp::Integer::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&num::BigInt::from_str(u).unwrap(), v),
                   out);
        assert_eq!(rugint::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == native::Integer::from_str(u).unwrap(), out);
        assert_eq!(v == gmp::Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rugint::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("-123", 123, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn partial_eq_u32_properties() {
    // n == u is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n == Natural::from(u) is equivalent to n == u.
    //
    // u == n is equivalent for malachite-gmp, malachite-native, and rugint.
    // Integer::from(u) == n is equivalent to u == n.
    // n == u is equivalent to u == n.
    let integer_and_u32 = |gmp_n: gmp::Integer, u: u32| {
        let n = gmp_integer_to_native(&gmp_n);
        let eq_1 = n == u;
        assert_eq!(gmp_n == u, eq_1);
        assert_eq!(num_partial_eq_u32(&native_integer_to_num_bigint(&n), u),
                   eq_1);
        assert_eq!(native_integer_to_rugint(&n) == u, eq_1);
        assert_eq!(n == native::Integer::from(u), eq_1);

        let eq_2 = u == n;
        assert_eq!(u == gmp_n, eq_2);
        assert_eq!(u == native_integer_to_rugint(&n), eq_2);
        assert_eq!(eq_1, eq_2);
        assert_eq!(native::Integer::from(u) == n, eq_2);
    };

    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_integers(seed, 32)),
                               &(|seed| random_x::<u32>(seed)))
                .take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }
}
