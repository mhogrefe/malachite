use common::test_properties;
use malachite_base::num::NegAssign;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::natural::naturals;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-BigInt::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rug::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = Integer::from_str(s).unwrap();
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
    test_properties(integers, |x| {
        let negative = -x;
        assert!(negative.is_valid());
        assert!(negative.is_valid());

        let negative_alt = -x.clone();
        assert!(negative_alt.is_valid());
        assert_eq!(negative_alt, negative);

        assert_eq!(bigint_to_integer(&(-integer_to_bigint(x))), negative);
        assert_eq!(
            rug_integer_to_integer(&(-integer_to_rug_integer(x))),
            negative
        );

        assert_eq!(negative == *x, *x == 0 as Limb);
        assert_eq!(-negative, *x);
    });

    test_properties(signeds::<SignedLimb>, |&x| {
        assert_eq!(Integer::from(-i64::from(x)), -Integer::from(x));
    });

    test_properties(naturals, |x| {
        assert_eq!(-x, -Integer::from(x));
    });
}
