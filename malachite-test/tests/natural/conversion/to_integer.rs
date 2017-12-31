use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native, GenerationMode};
use malachite_test::natural::conversion::to_integer::select_inputs;
use std::str::FromStr;

#[test]
fn test_into_integer() {
    let test = |s| {
        let x = native::Natural::from_str(s).unwrap().into_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = gmp::Natural::from_str(s).unwrap().into_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = native::Natural::from_str(s).unwrap().to_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = gmp::Natural::from_str(s).unwrap().to_integer();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}

#[test]
fn to_integer_properties() {
    // x.into_integer() is equivalent for malachite-gmp and malachite-native.
    // x.into_integer() is valid.
    // x.into_integer().to_string() == x.to_string()
    //
    // x.to_integer() is equivalent for malachite-gmp and malachite-native.
    // x.to_integer() is valid.
    // x.to_integer() == x.into_integer()
    //
    // x.to_integer().to_natural() == x
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let native_integer = x.clone().into_integer();
        assert!(native_integer.is_valid());
        let gmp_integer = gmp_x.clone().into_integer();
        assert!(gmp_integer.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_integer), native_integer);
        assert_eq!(native_integer.to_string(), x.to_string());

        let native_integer_2 = x.to_integer();
        assert!(native_integer_2.is_valid());
        let gmp_integer_2 = gmp_x.to_integer();
        assert!(gmp_integer_2.is_valid());
        assert_eq!(native_integer_2, native_integer);
        assert_eq!(gmp_integer_to_native(&gmp_integer_2), native_integer_2);
        assert_eq!(gmp_integer_2, gmp_integer);

        assert_eq!(native_integer_2.to_natural().unwrap(), x);
        assert_eq!(native_integer_2.into_natural().unwrap(), x);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
