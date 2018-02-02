use common::{test_eq_helper, LARGE_LIMIT};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use num::BigUint;
use rug;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<Natural>(&strings);
    test_eq_helper::<BigUint>(&strings);
    test_eq_helper::<rug::Integer>(&strings);
}

#[test]
fn eq_properties() {
    // x == y is equivalent for malachite, num, and rug.
    // (x == y) == (y == x)
    let two_naturals = |x: Natural, y: Natural| {
        let eq = x == y;
        assert_eq!(natural_to_biguint(&x) == natural_to_biguint(&y), eq);
        assert_eq!(natural_to_rug_integer(&x) == natural_to_rug_integer(&y), eq);
        assert_eq!(y == x, eq);
    };

    // x == x
    let one_natural = |x: Natural| {
        assert_eq!(x, x);
    };

    // x == y && x == z => x == z
    let three_naturals = |x: Natural, y: Natural, z: Natural| {
        if x == y && x == z {
            assert_eq!(x, z);
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
