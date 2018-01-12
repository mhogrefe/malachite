use common::LARGE_LIMIT;
use malachite_base::traits::Assign;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::natural::conversion::assign_u32::{select_inputs, num_assign_u32};
use num::BigUint;
use rugint;
use rugint::Assign as rugint_assign;
use std::str::FromStr;

#[test]
fn test_assign_u32() {
    let test = |u, v: u32, out| {
        let mut x = Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigUint::from_str(u).unwrap();
        num_assign_u32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rugint::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    test("123", u32::max_value(), "4294967295");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_u32_properties() {
    // n.assign(u) is equivalent for malachite, num, and rugint.
    // n.assign(u) is valid.
    // n.assign(u); n == u
    // n.assign(Natural::from(u)) is equivalent to n.assign(u)
    let natural_and_u32 = |mut n: Natural, u: u32| {
        let old_n = n.clone();
        n.assign(u);
        assert!(n.is_valid());
        assert_eq!(n, u);
        let mut alt_n = old_n.clone();
        alt_n.assign(Natural::from(u));
        assert_eq!(alt_n, n);

        let mut num_n = natural_to_biguint(&old_n);
        num_assign_u32(&mut num_n, u);
        assert_eq!(biguint_to_natural(&num_n), u);

        let mut rugint_n = natural_to_rugint_integer(&old_n);
        rugint_n.assign(u);
        assert_eq!(rugint_integer_to_natural(&rugint_n), u);
    };

    for (n, u) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }
}
