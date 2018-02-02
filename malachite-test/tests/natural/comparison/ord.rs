use common::{test_cmp_helper, LARGE_LIMIT};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use num::BigUint;
use rug;
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    let strings = vec![
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Natural>(&strings);
    test_cmp_helper::<BigUint>(&strings);
    test_cmp_helper::<rug::Integer>(&strings);
}

#[test]
fn cmp_properties() {
    // x.cmp(&y) is equivalent for malachite, num, and rug.
    // x.cmp(&y) == y.cmp(&x).reverse()
    // x.cmp(&y) == (-y).cmp(-x)
    let two_naturals = |x: Natural, y: Natural| {
        let ord = x.cmp(&y);
        assert_eq!(natural_to_biguint(&x).cmp(&natural_to_biguint(&y)), ord);
        assert_eq!(
            natural_to_rug_integer(&x).cmp(&natural_to_rug_integer(&y)),
            ord
        );
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!((-y).cmp(&(-x)), ord);
    };

    // x == x
    let one_natural = |x: Natural| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    };

    // x < y && x < z => x < z, x > y && x > z => x > z
    let three_naturals = |x: Natural, y: Natural, z: Natural| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    for (x, y) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y, z) in triples_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }

    for (x, y, z) in triples_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }
}
