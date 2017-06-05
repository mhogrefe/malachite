use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_num_biguint,
                             native_natural_to_rugint_integer};
use malachite_test::natural::comparison::partial_eq_u32::num_partial_eq_u32;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Natural::from_str(u).unwrap() == v, out);
        assert_eq!(gmp::Natural::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&num::BigUint::from_str(u).unwrap(), v),
                   out);
        assert_eq!(rugint::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == native::Natural::from_str(u).unwrap(), out);
        assert_eq!(v == gmp::Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rugint::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn partial_eq_u32_properties() {
    // n == u is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n == Natural::from(u) is equivalent to n == u.
    //
    // u == n is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // Natural::from(u) == n is equivalent to u == n.
    // n == u is equivalent to u == n.
    let natural_and_u32 = |gmp_n: gmp::Natural, u: u32| {
        let n = gmp_natural_to_native(&gmp_n);
        let eq_1 = n == u;
        assert_eq!(gmp_n == u, eq_1);
        assert_eq!(num_partial_eq_u32(&mut native_natural_to_num_biguint(&n), u),
                   eq_1);
        assert_eq!(native_natural_to_rugint_integer(&n) == u, eq_1);
        assert_eq!(n == native::Natural::from(u), eq_1);

        let eq_2 = u == n;
        assert_eq!(u == gmp_n, eq_2);
        assert_eq!(u == native_natural_to_rugint_integer(&n), eq_2);
        assert_eq!(eq_1, eq_2);
        assert_eq!(native::Natural::from(u) == n, eq_2);
    };

    for (n, u) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_naturals(seed, 32)),
                               &(|seed| random_x::<u32>(seed)))
                .take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }
}
