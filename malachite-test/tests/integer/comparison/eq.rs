use common::{test_eq_helper, LARGE_LIMIT};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use num::BigInt;
use rug;

#[test]
fn test_eq() {
    let strings = vec![
        "0",
        "1",
        "-1",
        "2",
        "-2",
        "123",
        "-123",
        "1000000000000",
        "-1000000000000",
    ];
    test_eq_helper::<Integer>(&strings);
    test_eq_helper::<BigInt>(&strings);
    test_eq_helper::<rug::Integer>(&strings);
}

#[test]
fn eq_properties() {
    // x == y is equivalent for malachite, num, and rug.
    // (x == y) == (y == x)
    let two_integers = |x: Integer, y: Integer| {
        let eq = x == y;
        assert_eq!(integer_to_bigint(&x) == integer_to_bigint(&y), eq);
        assert_eq!(integer_to_rug_integer(&x) == integer_to_rug_integer(&y), eq);
        assert_eq!(y == x, eq);
    };

    // x == x
    let one_integer = |x: Integer| {
        assert_eq!(x, x);
    };

    // x == y && x == z => x == z
    let three_integers = |x: Integer, y: Integer, z: Integer| {
        if x == y && x == z {
            assert_eq!(x, z);
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
