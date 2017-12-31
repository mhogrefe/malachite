use common::{test_custom_cmp_helper, LARGE_LIMIT};
use malachite_base::traits::{OrdAbs, PartialOrdAbs};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_rugint, GenerationMode};
use malachite_test::integer::comparison::ord_abs::select_inputs;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};
use std::cmp::Ordering;

#[test]
fn test_ord_abs() {
    let strings = vec![
        "0",
        "1",
        "-2",
        "123",
        "-124",
        "999999999999",
        "-1000000000000",
        "1000000000001",
    ];
    test_custom_cmp_helper::<native::Integer, _>(&strings, |x, y| x.cmp_abs(y));
    test_custom_cmp_helper::<gmp::Integer, _>(&strings, |x, y| x.cmp_abs(y));
    test_custom_cmp_helper::<rugint::Integer, _>(&strings, |x, y| x.cmp_abs(y));
}

#[test]
fn cmp_properties() {
    // x.cmp_abs(&y) is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.cmp_abs(&y) == x.abs().cmp(&y.abs())
    // x.cmp_abs(&y) == y.cmp_abs(&x).reverse()
    // x.cmp_abs(&y) == (-x).cmp_abs(-y)
    let two_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let ord = x.cmp_abs(&y);
        assert_eq!(gmp_x.cmp_abs(&gmp_y), ord);
        assert_eq!(
            native_integer_to_rugint(&x).cmp_abs(&native_integer_to_rugint(&y)),
            ord
        );
        assert_eq!(x.abs_ref().cmp(&y.abs_ref()), ord);
        assert_eq!(y.cmp_abs(&x).reverse(), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    };

    // x == x
    // x == -x
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        assert_eq!(x.cmp_abs(&x), Ordering::Equal);
        assert_eq!(x.cmp_abs(&-&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer, gmp_z: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let z = gmp_integer_to_native(&gmp_z);
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    };

    for (x, y) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }

    for (x, y, z) in
        random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT)
    {
        three_integers(x, y, z);
    }
}
