use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rugint_integer, natural_to_rugint_integer, GenerationMode};
use malachite_test::integer::comparison::partial_ord_natural::select_inputs_2;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_integer_natural() {
    let test = |u, v, out| {
        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .partial_cmp(&Natural::from_str(v).unwrap(),),
            out
        );

        assert_eq!(
            Natural::from_str(v)
                .unwrap()
                .partial_cmp(&Integer::from_str(u).unwrap())
                .map(|o| o.reverse()),
            out
        );
    };
    test("0", "0", Some(Ordering::Equal));
    test("0", "5", Some(Ordering::Less));
    test("123", "123", Some(Ordering::Equal));
    test("123", "124", Some(Ordering::Less));
    test("123", "122", Some(Ordering::Greater));
    test("1000000000000", "123", Some(Ordering::Greater));
    test("123", "1000000000000", Some(Ordering::Less));
    test("1000000000000", "1000000000000", Some(Ordering::Equal));
    test("-1000000000000", "1000000000000", Some(Ordering::Less));
    test("-1000000000000", "0", Some(Ordering::Less));
}

#[test]
fn partial_cmp_integer_natural_properties() {
    // x.partial_cmp(&y) is equivalent for malachite and rugint.
    // x.into_integer().partial_cmp(&y) is equivalent to x.partial_cmp(&y).
    // x < y <=> y > x, x > y <=> y < x, and x == y <=> y == x.
    let natural_and_integer = |x: Natural, y: Integer| {
        let cmp_1 = x.partial_cmp(&y);
        assert_eq!(
            natural_to_rugint_integer(&x).partial_cmp(&integer_to_rugint_integer(&y)),
            cmp_1
        );
        assert_eq!(x.to_integer().cmp(&y), cmp_1.unwrap());

        let cmp_2 = y.partial_cmp(&x);
        assert_eq!(
            integer_to_rugint_integer(&y).partial_cmp(&natural_to_rugint_integer(&x)),
            cmp_2
        );
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(y.cmp(&x.into_integer()), cmp_2.unwrap());
    };

    // x < y and y < z => x < z
    // x > y and y > z => x > z
    let natural_integer_and_natural = |x: Natural, y: Integer, z: Natural| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    // y < x and x < z => y < z
    // y > x and x > z => y > z
    let integer_natural_and_integer = |x: Integer, y: Natural, z: Integer| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in select_inputs_2(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y) in select_inputs_2(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y, z) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_integers(),
        exhaustive_naturals(),
    ).take(LARGE_LIMIT)
    {
        natural_integer_and_natural(x, y, z);
    }

    for (x, y, z) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        natural_integer_and_natural(x, y, z);
    }

    for (x, y, z) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_naturals(),
        exhaustive_integers(),
    ).take(LARGE_LIMIT)
    {
        integer_natural_and_integer(x, y, z);
    }

    for (x, y, z) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_integers(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_natural_and_integer(x, y, z);
    }
}
