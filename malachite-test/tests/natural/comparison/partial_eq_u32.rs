use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_test::natural::comparison::partial_eq_u32::{num_partial_eq_u32, select_inputs_1};
use num::BigUint;
use rugint;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(num_partial_eq_u32(&BigUint::from_str(u).unwrap(), v), out);
        assert_eq!(rugint::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rugint::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn partial_eq_u32_properties() {
    // n == u is equivalent for malachite, num, and rugint.
    // n == Natural::from(u) is equivalent to n == u.
    //
    // u == n is equivalent for malachite and rugint.
    // Natural::from(u) == n is equivalent to u == n.
    // n == u is equivalent to u == n.
    let natural_and_u32 = |n: Natural, u: u32| {
        let eq_1 = n == u;
        assert_eq!(num_partial_eq_u32(&natural_to_biguint(&n), u), eq_1);
        assert_eq!(natural_to_rugint_integer(&n) == u, eq_1);
        assert_eq!(n == Natural::from(u), eq_1);

        let eq_2 = u == n;
        assert_eq!(u == natural_to_rugint_integer(&n), eq_2);
        assert_eq!(eq_1, eq_2);
        assert_eq!(Natural::from(u) == n, eq_2);
    };

    for (n, u) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }
}
