use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::pairs_of_integer_and_signed;
use malachite_test::integer::conversion::assign_i32::num_assign_i32;
use num::BigInt;
use rugint;
use rugint::Assign as rugint_assign;
use std::i32;
use std::str::FromStr;

#[test]
fn test_assign_i32() {
    let test = |u, v: i32, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        num_assign_i32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("123", -456, "-456");
    test("-123", i32::MAX, "2147483647");
    test("123", i32::MIN, "-2147483648");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_i32_properties() {
    // n.assign(i) is equivalent for malachite, num, and rugint.
    // n.assign(i) is valid.
    // n.assign(i); n == u
    // n.assign(Integer::from(i)) is equivalent to n.assign(i)
    let integer_and_i32 = |mut n: Integer, i: i32| {
        let old_n = n.clone();
        n.assign(i);
        assert!(n.is_valid());
        assert_eq!(n, i);
        let mut alt_n = old_n.clone();
        alt_n.assign(Integer::from(i));
        assert_eq!(alt_n, n);

        let mut num_n = integer_to_bigint(&old_n);
        num_assign_i32(&mut num_n, i);
        assert_eq!(bigint_to_integer(&num_n), i);

        let mut rugint_n = integer_to_rugint_integer(&old_n);
        rugint_n.assign(i);
        assert_eq!(rugint_integer_to_integer(&rugint_n), i);
    };

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }
}
