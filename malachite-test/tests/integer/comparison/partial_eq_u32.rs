use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use malachite_test::inputs::integer::pairs_of_integer_and_unsigned;
use malachite_test::integer::comparison::partial_eq_u32::num_partial_eq_u32;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&BigInt::from_str(u).unwrap(), v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("-123", 123, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn partial_eq_u32_properties() {
    // n == u is equivalent for malachite, num, and rug.
    // n == Natural::from(u) is equivalent to n == u.
    //
    // u == n is equivalent for malachite and rug.
    // Integer::from(u) == n is equivalent to u == n.
    // n == u is equivalent to u == n.
    let integer_and_u32 = |n: Integer, u: u32| {
        let eq_1 = n == u;
        assert_eq!(num_partial_eq_u32(&integer_to_bigint(&n), u), eq_1);
        assert_eq!(integer_to_rug_integer(&n) == u, eq_1);
        assert_eq!(n == Integer::from(u), eq_1);

        let eq_2 = u == n;
        assert_eq!(u == integer_to_rug_integer(&n), eq_2);
        assert_eq!(eq_1, eq_2);
        assert_eq!(Integer::from(u) == n, eq_2);
    };

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }
}
