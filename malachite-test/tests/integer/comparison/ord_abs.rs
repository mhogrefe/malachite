use common::{test_custom_cmp_helper, LARGE_LIMIT};
use malachite_base::num::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rugint_integer, GenerationMode};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use rugint;
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
    test_custom_cmp_helper::<Integer, _>(&strings, |x, y| x.cmp_abs(y));
    test_custom_cmp_helper::<rugint::Integer, _>(&strings, |x, y| x.cmp_abs(y));
}

#[test]
fn cmp_properties() {
    // x.cmp_abs(&y) is equivalent for malachite and rugint.
    // x.cmp_abs(&y) == x.abs().cmp(&y.abs())
    // x.cmp_abs(&y) == y.cmp_abs(&x).reverse()
    // x.cmp_abs(&y) == (-x).cmp_abs(-y)
    let two_integers = |x: Integer, y: Integer| {
        let ord = x.cmp_abs(&y);
        assert_eq!(
            integer_to_rugint_integer(&x).cmp_abs(&integer_to_rugint_integer(&y)),
            ord
        );
        assert_eq!(x.abs_ref().cmp(&y.abs_ref()), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    };

    // x == x
    // x == -x
    let one_integer = |x: Integer| {
        assert_eq!(x.cmp_abs(&x), Ordering::Equal);
        assert_eq!(x.cmp_abs(&-&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_integers = |x: Integer, y: Integer, z: Integer| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
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
