use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rugint_integer, rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::natural::naturals;
use rugint;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -(&Natural::from_str(s).unwrap());
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
    // -x is equivalent for malachite and rugint.
    // -x is valid.
    //
    // -&x is equivalent for malachite and rugint.
    // -&x is valid.
    // -x and -&x are equivalent.
    //
    // -x == -(x.to_integer())
    // (-x == x) == (x == 0)
    // --x == x
    let one_natural = |x: Natural| {
        let neg = -x.clone();
        assert!(neg.is_valid());

        let rugint_neg = -natural_to_rugint_integer(&x);
        assert_eq!(rugint_integer_to_integer(&rugint_neg), neg);

        let neg_2 = -&x;
        assert!(neg_2.is_valid());

        assert_eq!(neg_2, neg);

        assert_eq!(-x.to_integer(), neg);
        assert_eq!(neg == x, x == 0);
        assert_eq!(-&neg, x);
    };

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
