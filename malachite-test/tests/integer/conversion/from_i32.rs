use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, rug_integer_to_integer};
use malachite_test::inputs::base::signeds;
use num::BigInt;
use rug;
use std::i32;

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(i).to_string(), out);

        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::MIN, "-2147483648");
    test(i32::MAX, "2147483647");
}

#[test]
fn from_i32_properties() {
    test_properties(signeds, |&i: &i32| {
        let n = Integer::from(i);
        assert!(n.is_valid());
        assert_eq!(i32::checked_from(&n), Some(i));

        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });
}
