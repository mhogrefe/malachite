use common::LARGE_LIMIT;
use malachite_native as native;
use malachite_gmp as gmp;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native,
                             native_integer_to_rugint, native_natural_to_rugint_integer};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_integer_partial_eq_natural() {
    let test =
        |u, v, out| {
            assert_eq!(
                native::integer::Integer::from_str(v).unwrap() ==
                    native::natural::Natural::from_str(u).unwrap(),
                out
            );
            assert_eq!(
                gmp::integer::Integer::from_str(v).unwrap() ==
                    gmp::natural::Natural::from_str(u).unwrap(),
                out
            );

            assert_eq!(
                native::natural::Natural::from_str(u).unwrap() ==
                    native::integer::Integer::from_str(v).unwrap(),
                out
            );
            assert_eq!(
                gmp::natural::Natural::from_str(u).unwrap() ==
                    gmp::integer::Integer::from_str(v).unwrap(),
                out
            );

            assert_eq!(
                rugint::Integer::from_str(u).unwrap() == rugint::Integer::from_str(v).unwrap(),
                out
            );
        };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("123", "-123", false);
    test("123", "5", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("1000000000000", "-1000000000000", false);
}

#[test]
fn partial_eq_natural_properties() {
    // x == y is equivalent for malachite-gmp, malachite-native, and rugint.
    // x == y.into_integer() is equivalent to x == y.
    let integer_and_natural = |gmp_x: gmp::integer::Integer, gmp_y: gmp::natural::Natural| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let eq = x == y;
        assert_eq!(gmp_x == gmp_y, eq);
        assert_eq!(
            native_integer_to_rugint(&x) == native_natural_to_rugint_integer(&y),
            eq
        );
        assert_eq!(x == y.into_integer(), eq)
    };

    // x == y is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.into_integer() == y is equivalent to x == y.
    let natural_and_integer = |gmp_x: gmp::natural::Natural, gmp_y: gmp::integer::Integer| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let eq = x == y;
        assert_eq!(gmp_x == gmp_y, eq);
        assert_eq!(
            native_natural_to_rugint_integer(&x) == native_integer_to_rugint(&y),
            eq
        );
        assert_eq!(x.into_integer() == y, eq)
    };

    for (x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()).take(LARGE_LIMIT) {
        integer_and_natural(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_and_natural(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_integers(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        natural_and_integer(x, y);
    }
}
