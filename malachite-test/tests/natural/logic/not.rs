use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native,
                             native_natural_to_rugint_integer, rugint_integer_to_native};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !native::Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !gmp::Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&native::Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&gmp::Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rugint::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("1000000000000", "-1000000000001");
    test("2147483647", "-2147483648");
}

#[test]
fn not_properties() {
    // !x is equivalent for malachite-gmp, malachite-native, and rugint.
    // !x is valid.
    //
    // !&x is equivalent for malachite-gmp, malachite-native, and rugint.
    // !&x is valid.
    // !x and !&x are equivalent.
    //
    // !x < 0
    // !x == !(x.to_integer())
    // !x != x
    // !!x == x
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let native_not = !x.clone();
        assert!(native_not.is_valid());

        let gmp_not = !gmp_x.clone();
        assert!(gmp_not.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_not), native_not);

        let rugint_not = !native_natural_to_rugint_integer(&x);
        assert_eq!(rugint_integer_to_native(&rugint_not), native_not);

        let native_not_2 = !&x;
        assert!(native_not_2.is_valid());

        let gmp_not_2 = !&gmp_x;
        assert!(gmp_not_2.is_valid());

        assert_eq!(native_not_2, native_not);
        assert_eq!(gmp_not_2, gmp_not);

        assert!(native_not < 0);
        assert_eq!(!x.to_integer(), native_not);
        assert_ne!(native_not, x);
        assert_eq!(!&native_not, x);
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
