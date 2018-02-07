use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::pairs_of_integer_and_signed;
use std::{i32, i64};
use std::str::FromStr;

#[test]
fn test_assign_i64() {
    let test = |u, v: i64, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("123", -456, "-456");
    test("-123", i32::MAX.into(), "2147483647");
    test("123", i32::MIN.into(), "-2147483648");
    test("-123", i64::MAX, "9223372036854775807");
    test("123", i64::MIN, "-9223372036854775808");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_i64_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i64)| {
            let mut mut_n = n.clone();
            mut_n.assign(i);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, Integer::from(i));
        },
    );
}
