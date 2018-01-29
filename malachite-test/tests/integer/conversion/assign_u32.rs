use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::pairs_of_integer_and_unsigned;
use malachite_test::integer::conversion::assign_u32::num_assign_u32;
use num::BigInt;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;
use std::u32;

#[test]
fn test_assign_u32() {
    let test = |u, v: u32, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        num_assign_u32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("-123", 456, "456");
    test("123", u32::MAX, "4294967295");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_u32_properties() {
    // n.assign(u) is equivalent for malachite, num, and rugint.
    // n.assign(u) is valid.
    // n.assign(u); n == u
    // n.assign(Integer::from(u)) is equivalent to n.assign(u)
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n.assign(u);
        assert!(n.is_valid());
        assert_eq!(n, u);
        let mut alt_n = old_n.clone();
        alt_n.assign(Integer::from(u));
        assert_eq!(alt_n, n);

        let mut num_n = integer_to_bigint(&old_n);
        num_assign_u32(&mut num_n, u);
        assert_eq!(bigint_to_integer(&num_n), u);

        let mut rugint_n = integer_to_rugint_integer(&old_n);
        rugint_n.assign(u);
        assert_eq!(rugint_integer_to_integer(&rugint_n), u);
    };

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }
}
