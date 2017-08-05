use common::LARGE_LIMIT;
use malachite_native as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native, native_integer_to_gmp,
                             native_integer_to_rugint, native_natural_to_rugint_integer,
                             rugint_integer_to_native};
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_assign_natural() {
    let test = |u, v, out| {
        // assign Integer by value
        let mut x = native::integer::Integer::from_str(u).unwrap();
        x.assign(&native::natural::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::integer::Integer::from_str(u).unwrap();
        x.assign(&gmp::natural::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Integer by reference
        let mut x = native::integer::Integer::from_str(u).unwrap();
        x.assign(&native::natural::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::integer::Integer::from_str(u).unwrap();
        x.assign(&gmp::natural::Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(&rugint::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn assign_natural_properties() {
    // x.assign(y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.assign(y) is valid.
    // x.assign(y); x == y
    // x.assign(&y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.assign(&y) is valid.
    // x.assign(&y); x == y
    let integer_and_natural = |mut gmp_x: gmp::integer::Integer, gmp_y: gmp::natural::Natural| {
        let mut x = gmp_integer_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let old_x = x.clone();
        gmp_x.assign(gmp_y.clone());
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.assign(y.clone());
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = native_integer_to_rugint(&old_x);
        let rugint_y = native_natural_to_rugint_integer(&y);
        rugint_x.assign(rugint_y);
        assert_eq!(rugint_integer_to_native(&rugint_x), y);

        x = old_x.clone();
        gmp_x = native_integer_to_gmp(&old_x);
        gmp_x.assign(&gmp_y);
        assert!(gmp_x.is_valid());
        assert_eq!(gmp_x, gmp_y);
        x.assign(&y);
        assert!(x.is_valid());
        assert_eq!(x, y);
        let mut rugint_x = native_integer_to_rugint(&old_x);
        let rugint_y = native_natural_to_rugint_integer(&y);
        rugint_x.assign(&rugint_y);
        assert_eq!(rugint_integer_to_native(&rugint_x), y);
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
}
