use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_test::integer::comparison::partial_eq_i32::{num_partial_eq_i32, select_inputs_1};
use num::BigInt;
use rugint;
use std::str::FromStr;

#[test]
fn test_partial_eq_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_i32(&BigInt::from_str(u).unwrap(), v), out);
        assert_eq!(rugint::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rugint::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("123", 5, false);
    test("-123", 123, false);
    test("123", -123, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn partial_eq_i32_properties() {
    // n == i is equivalent for malachite, num, and rugint.
    // n == Natural::from(i) is equivalent to n == i.
    //
    // i == n is equivalent for malachite and rugint.
    // Integer::from(i) == n is equivalent to i == n.
    // n == i is equivalent to i == n.
    let integer_and_i32 = |n: Integer, i: i32| {
        let eq_1 = n == i;
        assert_eq!(num_partial_eq_i32(&integer_to_bigint(&n), i), eq_1);
        assert_eq!(integer_to_rugint_integer(&n) == i, eq_1);
        assert_eq!(n == Integer::from(i), eq_1);

        let eq_2 = i == n;
        assert_eq!(i == integer_to_rugint_integer(&n), eq_2);
        assert_eq!(eq_1, eq_2);
        assert_eq!(Integer::from(i) == n, eq_2);
    };

    for (n, i) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }
}
