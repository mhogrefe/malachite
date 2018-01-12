use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rugint_integer, natural_to_rugint_integer, GenerationMode};
use malachite_test::integer::comparison::partial_eq_natural::select_inputs_1;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_integer_partial_eq_natural() {
    let test = |u, v, out| {
        assert_eq!(
            Integer::from_str(v).unwrap() == Natural::from_str(u).unwrap(),
            out
        );

        assert_eq!(
            Natural::from_str(u).unwrap() == Integer::from_str(v).unwrap(),
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
    // x == y is equivalent for malachite and rugint.
    // x == y.into_integer() is equivalent to x == y.
    let integer_and_natural = |x: Integer, y: Natural| {
        let eq = x == y;
        assert_eq!(
            integer_to_rugint_integer(&x) == natural_to_rugint_integer(&y),
            eq
        );
        assert_eq!(x == y.into_integer(), eq)
    };

    // x == y is equivalent for malachite and rugint.
    // x.into_integer() == y is equivalent to x == y.
    let natural_and_integer = |x: Natural, y: Integer| {
        let eq = x == y;
        assert_eq!(
            natural_to_rugint_integer(&x) == integer_to_rugint_integer(&y),
            eq
        );
        assert_eq!(x.into_integer() == y, eq)
    };

    for (x, y) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_natural(x, y);
    }

    for (x, y) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
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
