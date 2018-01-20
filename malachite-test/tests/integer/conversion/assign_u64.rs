use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::conversion::assign_u64::{select_inputs, num_assign_u64};
use num::BigInt;
use std::str::FromStr;
use std::{u32, u64};

#[test]
fn test_assign_u64() {
    let test = |u, v: u64, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        num_assign_u64(&mut x, v);
        assert_eq!(x.to_string(), out);
    };
    test("-123", 456, "456");
    test("123", u32::MAX.into(), "4294967295");
    test("123", u64::MAX, "18446744073709551615");
    test("1000000000000000000000000", 123, "123");
}

#[test]
fn assign_u64_properties() {
    // n.assign(u) is valid.
    // n.assign(u); n == u
    // n.assign(Integer::from(u)) is equivalent to n.assign(u)
    let integer_and_u64 = |mut n: Integer, u: u64| {
        let old_n = n.clone();
        n.assign(u);
        assert!(n.is_valid());
        assert_eq!(n, Integer::from(u));
        let mut alt_n = old_n.clone();
        alt_n.assign(Integer::from(u));
        assert_eq!(alt_n, n);
    };

    for (n, u) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u64(n, u);
    }

    for (n, u) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u64(n, u);
    }
}
