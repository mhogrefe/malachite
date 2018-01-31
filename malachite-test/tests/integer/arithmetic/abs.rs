use common::LARGE_LIMIT;
use malachite_base::num::AbsAssign;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::integers;
use num::{BigInt, Signed};
use rugint;
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        let abs = Integer::from_str(s).unwrap().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rugint::Integer::from_str(s).unwrap().abs().to_string(), out);

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
    // x.abs() is equivalent for malachite, num, and rugint.
    // x.abs() is valid.
    //
    // x.abs_ref() is equivalent for malachite, num, and rugint.
    // x.abs_ref() is valid.
    // x.abs() and x.abs_ref() are equivalent.
    //
    // x.abs() >= 0
    // x.abs().abs() == x.abs()
    //
    // x.natural_abs() is valid.
    //
    // x.natural_abs_ref() is valid.
    // x.natural_abs() and x.natural_abs_ref() are equivalent.
    //
    // x.natural_abs_ref() == x.abs_ref().to_natural()
    let one_integer = |x: Integer| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        let num_abs = integer_to_bigint(&x).abs();
        assert_eq!(bigint_to_integer(&num_abs), abs);

        let mut rugint_x = integer_to_rugint_integer(&x);
        let rugint_abs = rugint_x.abs();
        assert_eq!(rugint_integer_to_integer(rugint_abs), abs);

        let abs_2 = x.abs_ref();
        assert!(abs_2.is_valid());
        assert_eq!(abs_2, abs);

        assert!(abs >= 0);
        assert_eq!(abs == x, x >= 0);
        assert_eq!(abs.abs_ref(), abs);

        let abs_3 = x.clone().natural_abs();
        assert!(abs_3.is_valid());
        assert_eq!(Some(abs_3), abs.to_natural());

        let abs_4 = x.natural_abs_ref();
        assert!(abs_4.is_valid());
        assert_eq!(Some(abs_4), abs.to_natural());
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
