use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::integer::pairs_of_integer_and_unsigned;
use malachite_test::integer::conversion::assign_u64::num_assign_u64;
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
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u64)| {
            let mut mut_n = n.clone();
            mut_n.assign(u);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, Integer::from(u));
        },
    );

    test_properties(pairs_of_unsigneds::<u64>, #[allow(unused_assignments)]
    |&(u, v)| {
        let mut mut_u = u;
        let mut mut_n = Integer::from(u);
        mut_u = v;
        mut_n.assign(v);
        assert_eq!(Integer::from(mut_u), mut_n);
    });
}
