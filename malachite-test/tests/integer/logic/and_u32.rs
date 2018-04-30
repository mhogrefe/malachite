use common::test_properties;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::logic::and_u32::{integer_and_u32_alt_1, integer_and_u32_alt_2};
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_and_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = v & &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v & rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v & &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("123", 456, "72");
    test("1000000000000", 123, "0");
    test("1000000000001", 123, "1");
    test("12345678987654321", 987_654_321, "579887281");
    test("-123", 0, "0");
    test("-123", 456, "384");
    test("-1000000000000", 123, "0");
    test("-1000000000001", 123, "123");
    test("-12345678987654321", 987_654_321, "407767041");
}

#[test]
fn and_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let mut mut_n = n.clone();
            mut_n &= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n &= u;
            assert_eq!(rug_integer_to_integer(&rug_n), result);

            assert_eq!(n & u, result);
            assert_eq!(u & n, result);
            assert_eq!(integer_and_u32_alt_1(&n, u), result);
            assert_eq!(integer_and_u32_alt_2(&n, u), result);

            //TODO assert_eq!(n & Integer::from(u), result);
            //TODO assert_eq!(Integer::from(u) & n, result);

            assert_eq!(&result & u, result);

            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) & u)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n & 0u32, 0);
        assert_eq!(0u32 & n, 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(&Integer::ZERO & u, 0);
        assert_eq!(u & &Integer::ZERO, 0);
        assert_eq!(&Integer::NEGATIVE_ONE & u, u);
        assert_eq!(u & &Integer::NEGATIVE_ONE, u);
    });
}
