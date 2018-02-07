use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::natural::comparison::partial_eq_u32::num_partial_eq_u32;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&BigUint::from_str(u).unwrap(), v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn partial_eq_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let eq = *n == u;
            assert_eq!(num_partial_eq_u32(&natural_to_biguint(n), u), eq);
            assert_eq!(natural_to_rug_integer(n) == u, eq);
            assert_eq!(*n == Natural::from(u), eq);

            assert_eq!(u == *n, eq);
            assert_eq!(u == natural_to_rug_integer(n), eq);
            assert_eq!(Natural::from(u) == *n, eq);
        },
    );
}
