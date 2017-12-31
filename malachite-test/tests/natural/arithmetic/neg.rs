use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_integer_to_native, gmp_natural_to_native,
                             native_natural_to_rugint_integer, rugint_integer_to_native,
                             GenerationMode};
use malachite_test::natural::arithmetic::neg::select_inputs;
use rugint;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -native::Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -gmp::Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -(&native::Natural::from_str(s).unwrap());
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -(&gmp::Natural::from_str(s).unwrap());
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-rugint::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("1000000000000", "-1000000000000");
    test("2147483648", "-2147483648");
}

#[test]
fn neg_properties() {
    // -x is equivalent for malachite-gmp, malachite-native, and rugint.
    // -x is valid.
    //
    // -&x is equivalent for malachite-gmp, malachite-native, and rugint.
    // -&x is valid.
    // -x and -&x are equivalent.
    //
    // -x == -(x.to_integer())
    // (-x == x) == (x == 0)
    // --x == x
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let native_neg = -x.clone();
        assert!(native_neg.is_valid());

        let gmp_neg = -gmp_x.clone();
        assert!(gmp_neg.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_neg), native_neg);

        let rugint_neg = -native_natural_to_rugint_integer(&x);
        assert_eq!(rugint_integer_to_native(&rugint_neg), native_neg);

        let native_neg_2 = -&x;
        assert!(native_neg_2.is_valid());

        let gmp_neg_2 = -&gmp_x;
        assert!(gmp_neg_2.is_valid());

        assert_eq!(native_neg_2, native_neg);
        assert_eq!(gmp_neg_2, gmp_neg);

        assert_eq!(-x.to_integer(), native_neg);
        assert_eq!(native_neg == x, x == 0);
        assert_eq!(-&native_neg, x);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
