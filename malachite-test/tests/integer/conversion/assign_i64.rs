use common::LARGE_LIMIT;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
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
    // n.assign(i) is valid.
    // n.assign(i); n == u
    // n.assign(Integer::from(i)) is equivalent to n.assign(i)
    let integer_and_i64 = |mut n: Integer, i: i64| {
        let old_n = n.clone();
        n.assign(i);
        assert!(n.is_valid());
        assert_eq!(n, Integer::from(i));
        let mut alt_n = old_n.clone();
        alt_n.assign(Integer::from(i));
        assert_eq!(alt_n, n);
    };

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i64(n, i);
    }

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i64(n, i);
    }
}
