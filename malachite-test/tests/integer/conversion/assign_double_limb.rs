#[cfg(feature = "32_bit_limbs")]
use common::test_properties;
use malachite_base::misc::Max;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{DoubleLimb, Limb};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::base::pairs_of_unsigneds;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::integer::pairs_of_integer_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::integer::conversion::assign_double_limb::num_assign_double_limb;
use num::BigInt;
use std::str::FromStr;

#[cfg(feature = "64_bit_limbs")]
fn num_assign_double_limb(x: &mut BigInt, u: DoubleLimb) {
    *x = BigInt::from(u);
}

#[test]
fn test_assign_double_limb() {
    let test = |u, v: DoubleLimb, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        num_assign_double_limb(&mut x, v);
        assert_eq!(x.to_string(), out);
    };
    test("-123", 456, "456");
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
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, DoubleLimb)| {
            let mut mut_n = n.clone();
            mut_n.assign(u);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, Integer::from(u));
        },
    );

    test_properties(pairs_of_unsigneds::<DoubleLimb>, #[allow(
        unused_assignments
    )]
    |&(u, v)| {
        let mut mut_u = u;
        let mut mut_n = Integer::from(u);
        mut_u = v;
        mut_n.assign(v);
        assert_eq!(Integer::from(mut_u), mut_n);
    });
}
