use common::LARGE_LIMIT;
use malachite_base::traits::NegAssign;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, num_bigint_to_native_integer,
                             rugint_integer_to_native, GenerationMode};
use malachite_test::integer::arithmetic::neg::select_inputs;
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -native::Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -gmp::Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -(&native::Integer::from_str(s).unwrap());
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -(&gmp::Integer::from_str(s).unwrap());
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-num::BigInt::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rugint::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = native::Integer::from_str(s).unwrap();
        x.neg_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let mut x = gmp::Integer::from_str(s).unwrap();
        x.neg_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("-123", "123");
    test("1000000000000", "-1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
    test("2147483648", "-2147483648");
}

#[test]
fn neg_properties() {
    // -x is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // -x is valid.
    //
    // -&x is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // -&x is valid.
    // -x and -&x are equivalent.
    //
    // (-x == x) == (x == 0)
    // --x == x
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_neg = -x.clone();
        assert!(native_neg.is_valid());

        let gmp_neg = -gmp_x.clone();
        assert!(gmp_neg.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_neg), native_neg);

        let num_neg = -native_integer_to_num_bigint(&x);
        assert_eq!(num_bigint_to_native_integer(&num_neg), native_neg);

        let rugint_neg = -native_integer_to_rugint(&x);
        assert_eq!(rugint_integer_to_native(&rugint_neg), native_neg);

        let native_neg_2 = -&x;
        assert!(native_neg_2.is_valid());

        let gmp_neg_2 = -&gmp_x;
        assert!(gmp_neg_2.is_valid());

        assert_eq!(native_neg_2, native_neg);
        assert_eq!(gmp_neg_2, gmp_neg);

        assert_eq!(native_neg == x, x == 0);
        assert_eq!(-&native_neg, x);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
