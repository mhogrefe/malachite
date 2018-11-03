use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::integer::pairs_of_integer_and_unsigned;
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::integer::comparison::partial_eq_u32::num_partial_eq_u32;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&BigInt::from_str(u).unwrap(), v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("-123", 123, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn partial_eq_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let eq = *n == u;
            assert_eq!(num_partial_eq_u32(&integer_to_bigint(n), u), eq);
            assert_eq!(integer_to_rug_integer(n) == u, eq);
            assert_eq!(*n == Integer::from(u), eq);

            assert_eq!(u == *n, eq);
            assert_eq!(u == integer_to_rug_integer(n), eq);
            assert_eq!(Integer::from(u) == *n, eq);
        },
    );

    test_properties(pairs_of_natural_and_unsigned::<u32>, |&(ref x, y)| {
        assert_eq!(Integer::from(x) == y, *x == y);
        assert_eq!(*x == Integer::from(y), *x == y);
    });

    test_properties(pairs_of_unsigneds::<u32>, |&(x, y)| {
        assert_eq!(Integer::from(x) == y, x == y);
        assert_eq!(x == Integer::from(y), x == y);
    });
}
