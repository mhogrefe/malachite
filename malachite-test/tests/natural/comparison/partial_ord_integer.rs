use common::LARGE_LIMIT;
use malachite_native as native;
use malachite_gmp as gmp;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native, native_integer_to_rugint,
                             native_natural_to_rugint_integer};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_triples, random_pairs,
                                     random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_integer() {
    let test = |u, v, out| {
        assert_eq!(native::natural::Natural::from_str(u)
                       .unwrap()
                       .partial_cmp(&native::integer::Integer::from_str(v).unwrap()),
                   out);
        assert_eq!(gmp::natural::Natural::from_str(u)
                       .unwrap()
                       .partial_cmp(&gmp::integer::Integer::from_str(v).unwrap()),
                   out);
    };
    test("0", "0", Some(Ordering::Equal));
    test("0", "5", Some(Ordering::Less));
    test("123", "123", Some(Ordering::Equal));
    test("123", "124", Some(Ordering::Less));
    test("123", "122", Some(Ordering::Greater));
    test("1000000000000", "123", Some(Ordering::Greater));
    test("123", "1000000000000", Some(Ordering::Less));
    test("1000000000000", "1000000000000", Some(Ordering::Equal));
    test("1000000000000", "-1000000000000", Some(Ordering::Greater));
    test("0", "-1000000000000", Some(Ordering::Greater));
}

#[test]
fn partial_cmp_integer_properties() {
    // x.partial_cmp(&y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.into_integer().partial_cmp(&y) is equivalent to x.partial_cmp(&y).
    // x < y <=> y > x, x > y <=> y < x, and x == y <=> y == x.
    let natural_and_integer = |gmp_x: gmp::natural::Natural, gmp_y: gmp::integer::Integer| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let cmp_1 = x.partial_cmp(&y);
        assert_eq!(gmp_x.partial_cmp(&gmp_y), cmp_1);
        assert_eq!(native_natural_to_rugint_integer(&x).partial_cmp(&native_integer_to_rugint(&y)),
                   cmp_1);
        assert_eq!(x.to_integer().cmp(&y), cmp_1.unwrap());

        let cmp_2 = y.partial_cmp(&x);
        assert_eq!(gmp_y.partial_cmp(&gmp_x), cmp_2);
        assert_eq!(native_integer_to_rugint(&y).partial_cmp(&native_natural_to_rugint_integer(&x)),
                   cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(y.cmp(&x.into_integer()), cmp_2.unwrap());
    };

    // x < y and y < z => x < z
    // x > y and y > z => x > z
    let natural_integer_and_natural = |gmp_x: gmp::natural::Natural,
                                       gmp_y: gmp::integer::Integer,
                                       gmp_z: gmp::natural::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let z = gmp_natural_to_native(&gmp_z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    // y < x and x < z => y < z
    // y > x and x > z => y > z
    let integer_natural_and_integer = |gmp_x: gmp::integer::Integer,
                                       gmp_y: gmp::natural::Natural,
                                       gmp_z: gmp::integer::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_natural_to_native(&gmp_y);
        let z = gmp_integer_to_native(&gmp_z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_naturals(seed, 32)),
                               &(|seed| random_integers(seed, 32)))
                .take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y, z) in exhaustive_triples(exhaustive_naturals(),
                                        exhaustive_integers(),
                                        exhaustive_naturals())
                .take(LARGE_LIMIT) {
        natural_integer_and_natural(x, y, z);
    }

    for (x, y, z) in random_triples(&EXAMPLE_SEED,
                                    &(|seed| random_naturals(seed, 32)),
                                    &(|seed| random_integers(seed, 32)),
                                    &(|seed| random_naturals(seed, 32)))
                .take(LARGE_LIMIT) {
        natural_integer_and_natural(x, y, z);
    }

    for (x, y, z) in exhaustive_triples(exhaustive_integers(),
                                        exhaustive_naturals(),
                                        exhaustive_integers())
                .take(LARGE_LIMIT) {
        integer_natural_and_integer(x, y, z);
    }

    for (x, y, z) in random_triples(&EXAMPLE_SEED,
                                    &(|seed| random_integers(seed, 32)),
                                    &(|seed| random_naturals(seed, 32)),
                                    &(|seed| random_integers(seed, 32)))
                .take(LARGE_LIMIT) {
        integer_natural_and_integer(x, y, z);
    }
}
