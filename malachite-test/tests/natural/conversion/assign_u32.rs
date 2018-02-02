use common::LARGE_LIMIT;
use malachite_base::num::Assign;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural, GenerationMode};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::natural::conversion::assign_u32::num_assign_u32;
use num::BigUint;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;
use std::u32;

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

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    test("123", u32::MAX, "4294967295");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_u32_properties() {
    // n.assign(u) is equivalent for malachite, num, and rug.
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

        let mut rug_n = natural_to_rug_integer(&old_n);
        rug_n.assign(u);
        assert_eq!(rug_integer_to_natural(&rug_n), u);
    };

    for (n, u) in pairs_of_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }

    for (n, u) in pairs_of_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(n, u);
    }
}
