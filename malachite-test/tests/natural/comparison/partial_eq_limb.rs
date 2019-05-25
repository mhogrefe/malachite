use std::str::FromStr;

use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use common::test_properties;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::natural::comparison::partial_eq_limb::num_partial_eq_limb;

#[test]
fn test_partial_eq_limb() {
    let test = |u, v: Limb, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_limb(&BigUint::from_str(u).unwrap(), v), out);
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
fn partial_eq_limb_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            let eq = *n == u;
            assert_eq!(num_partial_eq_limb(&natural_to_biguint(n), u), eq);
            assert_eq!(natural_to_rug_integer(n) == u, eq);
            assert_eq!(*n == Natural::from(u), eq);

            assert_eq!(u == *n, eq);
            assert_eq!(u == natural_to_rug_integer(n), eq);
            assert_eq!(Natural::from(u) == *n, eq);
        },
    );

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x) == y, x == y);
        assert_eq!(x == Natural::from(y), x == y);
    });
}
