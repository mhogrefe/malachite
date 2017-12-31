use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, GenerationMode};
use malachite_test::integer::comparison::partial_eq_i32::{num_partial_eq_i32, select_inputs_1};
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_partial_eq_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap() == v, out);
        assert_eq!(gmp::Integer::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_i32(&num::BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rugint::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == native::Integer::from_str(u).unwrap(), out);
        assert_eq!(v == gmp::Integer::from_str(u).unwrap(), out);
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
    // n == i is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n == Natural::from(i) is equivalent to n == i.
    //
    // i == n is equivalent for malachite-gmp, malachite-native, and rugint.
    // Integer::from(i) == n is equivalent to i == n.
    // n == i is equivalent to i == n.
    let integer_and_i32 = |gmp_n: gmp::Integer, i: i32| {
        let n = gmp_integer_to_native(&gmp_n);
        let eq_1 = n == i;
        assert_eq!(gmp_n == i, eq_1);
        assert_eq!(
            num_partial_eq_i32(&native_integer_to_num_bigint(&n), i),
            eq_1
        );
        assert_eq!(native_integer_to_rugint(&n) == i, eq_1);
        assert_eq!(n == native::Integer::from(i), eq_1);

        let eq_2 = i == n;
        assert_eq!(i == gmp_n, eq_2);
        assert_eq!(i == native_integer_to_rugint(&n), eq_2);
        assert_eq!(eq_1, eq_2);
        assert_eq!(native::Integer::from(i) == n, eq_2);
    };

    for (n, i) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }
}
