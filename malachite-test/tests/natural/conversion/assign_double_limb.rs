use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::conversion::traits::Assign;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use num::BigUint;

#[cfg(feature = "32_bit_limbs")]
use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{biguint_to_natural, natural_to_biguint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::base::pairs_of_unsigneds;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::natural::conversion::assign_double_limb::num_assign_double_limb;

#[cfg(feature = "64_bit_limbs")]
fn num_assign_double_limb(x: &mut BigUint, u: DoubleLimb) {
    *x = BigUint::from(u);
}

#[test]
fn test_assign_double_limb() {
    let test = |u, v: DoubleLimb, out| {
        let mut x = Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigUint::from_str(u).unwrap();
        num_assign_double_limb(&mut x, v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    #[cfg(feature = "32_bit_limbs")]
    {
        test("123", Limb::MAX.into(), "4294967295");
        test("123", DoubleLimb::MAX, "18446744073709551615");
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("123", Limb::MAX.into(), "18446744073709551615");
        test(
            "123",
            DoubleLimb::MAX,
            "340282366920938463463374607431768211455",
        );
    }
    test("1000000000000000000000000", 123, "123");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn assign_double_limb_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, DoubleLimb)| {
            let natural_u = Natural::from(u);
            let mut mut_n = n.clone();
            mut_n.assign(u);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, natural_u);

            let mut mut_n = n.clone();
            mut_n.assign(Natural::from(u));
            assert_eq!(mut_n, natural_u);

            let mut num_n = natural_to_biguint(n);
            num_assign_double_limb(&mut num_n, u);
            assert_eq!(biguint_to_natural(&num_n), natural_u);
        },
    );

    test_properties(pairs_of_unsigneds::<DoubleLimb>, #[allow(
        unused_assignments
    )]
    |&(u, v)| {
        let mut mut_u = u;
        let mut mut_n = Natural::from(u);
        mut_u = v;
        mut_n.assign(v);
        assert_eq!(Natural::from(mut_u), mut_n);
    });
}
