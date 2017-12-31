use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native, GenerationMode};
use malachite_test::integer::conversion::to_natural::select_inputs;
use std::str::FromStr;

#[test]
fn test_into_natural() {
    let test = |n, out| {
        let on = native::Integer::from_str(n).unwrap().into_natural();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Integer::from_str(n).unwrap().into_natural();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = native::Integer::from_str(n).unwrap().to_natural();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Integer::from_str(n).unwrap().to_natural();
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Some(0)");
    test("123", "Some(123)");
    test("-123", "None");
    test("1000000000000", "Some(1000000000000)");
    test("-1000000000000", "None");
    test("2147483647", "Some(2147483647)");
    test("2147483648", "Some(2147483648)");
    test("-2147483648", "None");
    test("-2147483649", "None");
}

#[test]
fn to_natural_properties() {
    // x.into_natural() is equivalent for malachite-gmp and malachite-native.
    // x.into_natural() is valid.
    // x.into_natural().to_string() == x.to_string()
    //
    // x.to_natural() is equivalent for malachite-gmp and malachite-native.
    // x.to_natural() is valid.
    // x.to_natural() == x.into_natural()
    //
    // x.to_natural().is_some() == x >= 0
    // if x >= 0, x.to_natural().to_integer() == x
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_on = x.clone().into_natural();
        assert!(native_on.clone().map_or(true, |n| n.is_valid()));

        let raw_gmp_on = gmp_x.clone().into_natural();
        assert!(raw_gmp_on.clone().map_or(true, |n| n.is_valid()));
        let gmp_on = raw_gmp_on.map(|n| gmp_natural_to_native(&n));
        assert_eq!(gmp_on, native_on);

        let native_on_2 = x.to_natural();
        assert!(native_on_2.clone().map_or(true, |n| n.is_valid()));
        let raw_gmp_on_2 = gmp_x.to_natural();
        assert!(raw_gmp_on_2.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(native_on_2, native_on);
        let gmp_on_2 = raw_gmp_on_2.map(|n| gmp_natural_to_native(&n));
        assert_eq!(gmp_on_2, native_on_2);

        assert_eq!(native_on.is_some(), x >= 0);
        if let Some(n) = native_on_2 {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(n.to_integer(), x);
            assert_eq!(n.into_integer(), x);
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
