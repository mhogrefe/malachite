use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::integer::pairs_of_integer_and_unsigned;
use malachite_test::integer::conversion::assign_u32::num_assign_u32;
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
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

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("-123", 456, "456");
    test("123", u32::MAX, "4294967295");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let mut mut_n = n.clone();
            mut_n.assign(u);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, u);

            let mut num_n = integer_to_bigint(n);
            num_assign_u32(&mut num_n, u);
            assert_eq!(bigint_to_integer(&num_n), u);

            let mut rug_n = integer_to_rug_integer(n);
            rug_n.assign(u);
            assert_eq!(rug_integer_to_integer(&rug_n), u);
        },
    );

    test_properties(pairs_of_unsigneds::<u32>, #[allow(unused_assignments)]
    |&(u, v)| {
        let mut mut_u = u;
        let mut mut_n = Integer::from(u);
        mut_u = v;
        mut_n.assign(v);
        assert_eq!(mut_u, mut_n);
    });
}
