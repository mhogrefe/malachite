use common::LARGE_LIMIT;
use malachite_base::traits::AbsAssign;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, num_bigint_to_native_integer,
                             rugint_integer_to_native, GenerationMode};
use malachite_test::integer::arithmetic::abs::select_inputs;
use num::{self, Signed};
use rugint;
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        let abs = native::Integer::from_str(s).unwrap().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = gmp::Integer::from_str(s).unwrap().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(num::BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rugint::Integer::from_str(s).unwrap().abs().to_string(), out);

        let abs = native::Integer::from_str(s).unwrap().natural_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = gmp::Integer::from_str(s).unwrap().natural_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let mut x = native::Integer::from_str(s).unwrap();
        x.abs_assign();
        assert!(abs.is_valid());
        assert_eq!(x.to_string(), out);

        let mut x = gmp::Integer::from_str(s).unwrap();
        x.abs_assign();
        assert!(abs.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "1000000000000");
    test("3000000000", "3000000000");
    test("-3000000000", "3000000000");
    test("-2147483648", "2147483648");
}

#[test]
fn abs_properties() {
    // x.abs() is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.abs() is valid.
    //
    // x.abs_ref() is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x.abs_ref() is valid.
    // x.abs() and x.abs_ref() are equivalent.
    //
    // x.abs() >= 0
    // x.abs().abs() == x.abs()
    //
    // x.natural_abs() is equivalent for malachite-gmp and malachite-native.
    // x.natural_abs() is valid.
    //
    // x.natural_abs_ref() is equivalent for malachite-gmp and malachite-native.
    // x.natural_abs_ref() is valid.
    // x.natural_abs() and x.natural_abs_ref() are equivalent.
    //
    // x.natural_abs_ref() == x.abs_ref().to_natural()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_abs = x.clone().abs();
        assert!(native_abs.is_valid());

        let gmp_abs = gmp_x.clone().abs();
        assert!(gmp_abs.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_abs), native_abs);

        let num_abs = native_integer_to_num_bigint(&x).abs();
        assert_eq!(num_bigint_to_native_integer(&num_abs), native_abs);

        let mut rugint_x = native_integer_to_rugint(&x);
        let rugint_abs = rugint_x.abs();
        assert_eq!(rugint_integer_to_native(rugint_abs), native_abs);

        let native_abs_2 = x.abs_ref();
        assert!(native_abs_2.is_valid());

        let gmp_abs_2 = gmp_x.abs_ref();
        assert!(gmp_abs_2.is_valid());

        assert_eq!(native_abs_2, native_abs);
        assert_eq!(gmp_abs_2, gmp_abs);

        assert!(native_abs >= 0);
        assert_eq!(native_abs == x, x >= 0);
        assert_eq!(native_abs.abs_ref(), native_abs);

        let native_abs_3 = x.clone().natural_abs();
        assert!(native_abs_3.is_valid());
        assert_eq!(Some(native_abs_3), native_abs.to_natural());

        let native_abs_3 = gmp_x.clone().natural_abs();
        assert!(native_abs_3.is_valid());
        assert_eq!(Some(native_abs_3), gmp_abs.to_natural());

        let native_abs_4 = x.natural_abs_ref();
        assert!(native_abs_4.is_valid());
        assert_eq!(Some(native_abs_4), native_abs.to_natural());

        let native_abs_4 = gmp_x.natural_abs_ref();
        assert!(native_abs_4.is_valid());
        assert_eq!(Some(native_abs_4), gmp_abs.to_natural());
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
