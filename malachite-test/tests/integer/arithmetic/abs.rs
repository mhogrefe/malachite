use common::test_properties;
use malachite_base::num::AbsAssign;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rug_integer,
                             rug_integer_to_integer};
use malachite_test::inputs::integer::integers;
use num::{BigInt, Signed};
use rug;
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        let abs = Integer::from_str(s).unwrap().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().abs().to_string(), out);

        let abs = Integer::from_str(s).unwrap().natural_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let mut x = Integer::from_str(s).unwrap();
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
    test_properties(integers, |x| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        assert_eq!(bigint_to_integer(&integer_to_bigint(x).abs()), abs);

        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(x).abs()),
            abs
        );

        let abs_alt = x.abs_ref();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        assert!(abs >= 0);
        assert_eq!(abs == *x, *x >= 0);
        assert_eq!(abs.abs_ref(), abs);

        let abs_alt = x.clone().natural_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Some(abs_alt), abs.to_natural());

        let abs_alt = x.natural_abs_ref();
        assert!(abs_alt.is_valid());
        assert_eq!(Some(abs_alt), abs.to_natural());
    });
}
