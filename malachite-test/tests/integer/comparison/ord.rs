use common::{test_cmp_helper, LARGE_LIMIT};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rugint_integer, GenerationMode};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use num::BigInt;
use rugint;
use std::cmp::Ordering;

#[test]
fn test_ord() {
    let strings = vec![
        "-1000000000001",
        "-1000000000000",
        "-999999999999",
        "-123",
        "-2",
        "-1",
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Integer>(&strings);
    test_cmp_helper::<BigInt>(&strings);
    test_cmp_helper::<rugint::Integer>(&strings);
}

#[test]
fn cmp_properties() {
    // x.cmp(&y) is equivalent for malachite, num, and rugint.
    // x.cmp(&y) == y.cmp(&x).reverse()
    // x.cmp(&y) == (-y).cmp(-x)
    let two_integers = |x: Integer, y: Integer| {
        let ord = x.cmp(&y);
        assert_eq!(
            integer_to_rugint_integer(&x).cmp(&integer_to_rugint_integer(&y)),
            ord
        );
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!((-y).cmp(&(-x)), ord);
    };

    // x == x
    let one_integer = |x: Integer| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_integers = |x: Integer, y: Integer, z: Integer| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in pairs_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in pairs_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y, z) in triples_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }

    for (x, y, z) in triples_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }
}
