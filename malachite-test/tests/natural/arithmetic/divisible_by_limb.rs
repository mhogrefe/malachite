use std::str::FromStr;

use malachite_base::num::traits::{DivisibleBy, One, Zero};
use malachite_nz::natural::arithmetic::divisible_by_limb::{
    _combined_limbs_divisible_by_limb, limbs_divisible_by_limb,
};
use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
#[cfg(feature = "32_bit_limbs")]
use rug;

use common::test_properties;
use malachite_test::common::natural_to_biguint;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_1, pairs_of_unsigneds, positive_unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_limb_var_1, pairs_of_natural_and_positive_limb_var_2,
    pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned_var_2,
    pairs_of_unsigned_and_natural, positive_naturals,
};
use malachite_test::natural::arithmetic::divisible_by_limb::num_divisible_by_limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_divisible_by_limb() {
    let test = |limbs: &[Limb], limb: Limb, divisible: bool| {
        assert_eq!(limbs_divisible_by_limb(limbs, limb), divisible);
        assert_eq!(limbs_mod_limb(limbs, limb) == 0, divisible);
        assert_eq!(_combined_limbs_divisible_by_limb(limbs, limb), divisible);
    };
    test(&[0, 0], 2, true);
    test(&[6, 7], 1, true);
    test(&[6, 7], 2, true);
    test(&[100, 101, 102], 10, false);
    test(&[123, 456], 789, false);
    test(&[369, 1368], 3, true);
    test(&[0xffff_ffff, 0xffff_ffff], 2, false);
    test(&[0xffff_ffff, 0xffff_ffff], 3, true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_divisible_by_limb_fail() {
    limbs_divisible_by_limb(&[10], 10);
}

#[test]
fn test_divisible_by_limb() {
    let test = |u, v: Limb, divisible| {
        let n = Natural::from_str(u).unwrap();
        assert_eq!(n.divisible_by(v), divisible);
        assert_eq!(n == 0 as Limb || v != 0 && n % v == 0, divisible);

        assert_eq!(
            num_divisible_by_limb(BigUint::from_str(u).unwrap(), v),
            divisible
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(u).unwrap().is_divisible_u(v),
            divisible
        );
    };
    test("0", 0, true);
    test("1", 0, false);
    test("1000000000000", 0, false);
    test("0", 1, true);
    test("0", 123, true);
    test("1", 1, true);
    test("123", 1, true);
    test("123", 123, true);
    test("123", 456, false);
    test("456", 123, false);
    test("369", 123, true);
    test("4294967295", 1, true);
    test("4294967295", 4_294_967_295, true);
    test("1000000000000", 1, true);
    test("1000000000000", 3, false);
    test("1000000000002", 3, true);
    test("1000000000000", 123, false);
    test("1000000000000", 4_294_967_295, false);
    test("1000000000000000000000000", 1, true);
    test("1000000000000000000000000", 3, false);
    test("1000000000000000000000002", 3, true);
    test("1000000000000000000000000", 123, false);
    test("1000000000000000000000000", 4_294_967_295, false);
}

#[test]
fn test_limb_divisible_by_natural() {
    let test = |u: Limb, v, divisible| {
        let n = Natural::from_str(v).unwrap();
        assert_eq!(u.divisible_by(&n), divisible);
        assert_eq!(u == 0 || n != 0 as Limb && u % n == 0, divisible);
    };
    test(0, "0", true);
    test(1, "0", false);
    test(0, "1", true);
    test(0, "123", true);
    test(1, "1", true);
    test(123, "1", true);
    test(123, "123", true);
    test(123, "456", false);
    test(456, "123", false);
    test(369, "123", true);
    test(4294967295, "1", true);
    test(4294967295, "4294967295", true);
    test(0, "1000000000000", true);
    test(123, "1000000000000", false);
}

#[test]
fn limbs_divisible_by_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let divisible = limbs_divisible_by_limb(limbs, limb);
            assert_eq!(Natural::from_limbs_asc(limbs).divisible_by(limb), divisible,);
            assert_eq!(limbs_mod_limb(limbs, limb) == 0, divisible,);
            assert_eq!(_combined_limbs_divisible_by_limb(limbs, limb), divisible,);
        },
    );
}

fn divisible_by_limb_properties_helper(n: &Natural, u: Limb) {
    let divisible = n.divisible_by(u);
    assert_eq!(*n == 0 as Limb || u != 0 && n % u == 0, divisible);

    //TODO assert_eq!(n.divisible_by(Natural::from(u)), remainder);

    assert_eq!(num_divisible_by_limb(natural_to_biguint(n), u), divisible);
    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(natural_to_rug_integer(n).is_divisible_u(u), divisible);
}

#[test]
fn divisible_by_limb_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            divisible_by_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            divisible_by_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            assert!(n.divisible_by(u));
            assert!(*n == 0 as Limb || u != 0 && n % u == 0);

            //TODO assert!(n.divisible_by(Natural::from(u));

            assert!(num_divisible_by_limb(natural_to_biguint(n), u));
            #[cfg(feature = "32_bit_limbs")]
            assert!(natural_to_rug_integer(n).is_divisible_u(u));
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            assert!(!n.divisible_by(u));
            assert!(*n != 0 as Limb && (u == 0 || n % u != 0));

            //TODO assert!(n.divisible_by(Natural::from(u));

            assert!(!num_divisible_by_limb(natural_to_biguint(n), u));
            #[cfg(feature = "32_bit_limbs")]
            assert!(!natural_to_rug_integer(n).is_divisible_u(u));
        },
    );

    test_properties(
        pairs_of_unsigned_and_natural,
        |&(u, ref n): &(Limb, Natural)| {
            let divisible = u.divisible_by(n);
            assert_eq!(u == 0 || *n != 0 as Limb && u % n == 0, divisible);
        },
    );

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(divisible, Natural::from(x).divisible_by(y));
        assert_eq!(divisible, x.divisible_by(&Natural::from(y)));
    });

    test_properties(naturals, |n| {
        assert!(n.divisible_by(1 as Limb));
    });

    test_properties(positive_naturals, |n| {
        assert!(!n.divisible_by(0 as Limb));
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert!(Natural::ZERO.divisible_by(u));
        if u > 1 {
            assert!(!Natural::ONE.divisible_by(u));
        }
        assert!(u.divisible_by(&Natural::from(u)));
    });
}
