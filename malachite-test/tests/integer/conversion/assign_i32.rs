use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::integer::pairs_of_integer_and_signed;
use malachite_test::integer::conversion::assign_i32::num_assign_i32;
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
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

        let mut x = rug::Integer::from_str(u).unwrap();
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
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            let mut mut_n = n.clone();
            mut_n.assign(i);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, i);

            let mut num_n = integer_to_bigint(n);
            num_assign_i32(&mut num_n, i);
            assert_eq!(bigint_to_integer(&num_n), i);

            let mut rug_n = integer_to_rug_integer(n);
            rug_n.assign(i);
            assert_eq!(rug_integer_to_integer(&rug_n), i);
        },
    );
}
