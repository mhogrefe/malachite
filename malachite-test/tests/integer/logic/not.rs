use common::LARGE_LIMIT;
use malachite_base::traits::NotAssign;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_rugint,
                             rugint_integer_to_native, GenerationMode};
use malachite_test::integer::logic::not::select_inputs;
use rugint;
use std::str::FromStr;

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !native::Integer::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !gmp::Integer::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&native::Integer::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&gmp::Integer::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rugint::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = native::Integer::from_str(s).unwrap();
        x.not_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let mut x = gmp::Integer::from_str(s).unwrap();
        x.not_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("-123", "122");
    test("1000000000000", "-1000000000001");
    test("-1000000000000", "999999999999");
    test("-2147483648", "2147483647");
    test("2147483647", "-2147483648");
}

#[test]
fn not_properties() {
    // !x is equivalent for malachite-gmp, malachite-native, and rugint.
    // !x is valid.
    //
    // !&x is equivalent for malachite-gmp, malachite-native, and rugint.
    // !&x is valid.
    // !x and -!x are equivalent.
    //
    // !x != x
    // !!x == x
    // (x >= 0) == (!x < 0)
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_not = !x.clone();
        assert!(native_not.is_valid());

        let gmp_not = !gmp_x.clone();
        assert!(gmp_not.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_not), native_not);

        let rugint_not = !native_integer_to_rugint(&x);
        assert_eq!(rugint_integer_to_native(&rugint_not), native_not);

        let native_not_2 = !&x;
        assert!(native_not_2.is_valid());

        let gmp_not_2 = !&gmp_x;
        assert!(gmp_not_2.is_valid());

        assert_eq!(native_not_2, native_not);
        assert_eq!(gmp_not_2, gmp_not);

        assert_ne!(native_not, x);
        assert_eq!(!&native_not, x);
        assert_eq!(x >= 0, native_not < 0);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
