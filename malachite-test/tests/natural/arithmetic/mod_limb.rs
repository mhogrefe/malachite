use std::str::FromStr;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{
    CeilingDivNegMod, DivMod, Mod, ModAssign, NegMod, NegModAssign, One, Zero,
};
use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use num::BigUint;
#[cfg(feature = "32_bit_limbs")]
use rug;

use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_positive_unsigned, pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    positive_unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_limb_var_1, pairs_of_natural_and_positive_unsigned,
    pairs_of_natural_and_unsigned_var_2, pairs_of_unsigned_and_positive_natural,
    triples_of_natural_natural_and_positive_unsigned,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::natural::arithmetic::mod_limb::{num_rem_u32, rug_neg_mod_u32};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_limb() {
    let test = |limbs: &[Limb], limb: Limb, remainder: Limb| {
        assert_eq!(limbs_mod_limb(limbs, limb), remainder);
    };
    test(&[0, 0], 2, 0);
    test(&[6, 7], 1, 0);
    test(&[6, 7], 2, 0);
    test(&[100, 101, 102], 10, 8);
    test(&[123, 456], 789, 636);
    test(&[0xffff_ffff, 0xffff_ffff], 2, 1);
    test(&[0xffff_ffff, 0xffff_ffff], 3, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_limb_fail_1() {
    limbs_mod_limb(&[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_limb_fail_2() {
    limbs_mod_limb(&[10, 10], 0);
}

#[test]
fn test_mod_limb() {
    let test = |u, v: Limb, remainder| {
        let mut n = Natural::from_str(u).unwrap();
        n %= v;
        assert!(n.is_valid());
        assert_eq!(n, remainder);

        assert_eq!(Natural::from_str(u).unwrap() % v, remainder);
        assert_eq!(&Natural::from_str(u).unwrap() % v, remainder);

        let mut n = Natural::from_str(u).unwrap();
        n.mod_assign(v);
        assert!(n.is_valid());
        assert_eq!(n, remainder);

        assert_eq!(Natural::from_str(u).unwrap().mod_op(v), remainder);
        assert_eq!((&Natural::from_str(u).unwrap()).mod_op(v), remainder);

        assert_eq!(Natural::from_str(u).unwrap()._mod_limb_naive(v), remainder);

        #[cfg(feature = "32_bit_limbs")]
        {
            assert_eq!(num_rem_u32(BigUint::from_str(u).unwrap(), v), remainder);
            assert_eq!(rug::Integer::from_str(u).unwrap() % v, remainder);
        }
    };
    test("0", 1, 0);
    test("0", 123, 0);
    test("1", 1, 0);
    test("123", 1, 0);
    test("123", 123, 0);
    test("123", 456, 123);
    test("456", 123, 87);
    test("4294967295", 1, 0);
    test("4294967295", 4_294_967_295, 0);
    test("1000000000000", 1, 0);
    test("1000000000000", 3, 1);
    test("1000000000000", 123, 100);
    test("1000000000000", 4_294_967_295, 3_567_587_560);
    test("1000000000000000000000000", 1, 0);
    test("1000000000000000000000000", 3, 1);
    test("1000000000000000000000000", 123, 37);
    test("1000000000000000000000000", 4_294_967_295, 3_167_723_695);
}

#[test]
#[should_panic]
fn rem_assign_limb_fail() {
    let mut n = Natural::from(10u32);
    n %= 0 as Limb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn rem_limb_fail() {
    Natural::from(10u32) % 0 as Limb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn rem_limb_ref_fail() {
    &Natural::from(10u32) % 0 as Limb;
}

#[test]
#[should_panic]
fn mod_assign_limb_fail() {
    Natural::from(10u32).mod_assign(0 as Limb);
}

#[test]
#[should_panic]
fn mod_limb_fail() {
    Natural::from(10u32).mod_op(0 as Limb);
}

#[test]
#[should_panic]
fn mod_limb_ref_fail() {
    (&Natural::from(10u32)).mod_op(0 as Limb);
}

#[test]
fn test_neg_mod_limb() {
    let test = |u, v: Limb, remainder| {
        let mut n = Natural::from_str(u).unwrap();
        n.neg_mod_assign(v);
        assert_eq!(n, remainder);

        assert_eq!(Natural::from_str(u).unwrap().neg_mod(v), remainder);
        assert_eq!((&Natural::from_str(u).unwrap()).neg_mod(v), remainder);

        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug_neg_mod_u32(rug::Integer::from_str(u).unwrap(), v),
            remainder
        );
    };
    test("0", 1, 0);
    test("0", 123, 0);
    test("1", 1, 0);
    test("123", 1, 0);
    test("123", 123, 0);
    test("123", 456, 333);
    test("456", 123, 36);
    test("4294967295", 1, 0);
    test("4294967295", 4_294_967_295, 0);
    test("1000000000000", 1, 0);
    test("1000000000000", 3, 2);
    test("1000000000000", 123, 23);
    test("1000000000000", 4_294_967_295, 727_379_735);
    test("1000000000000000000000000", 1, 0);
    test("1000000000000000000000000", 3, 2);
    test("1000000000000000000000000", 123, 86);
    test("1000000000000000000000000", 4_294_967_295, 1_127_243_600);
}

#[test]
#[should_panic]
fn neg_mod_assign_limb_fail() {
    Natural::from(10u32).neg_mod_assign(0 as Limb);
}

#[test]
#[should_panic]
fn neg_mod_limb_fail() {
    Natural::from(10u32).neg_mod(0 as Limb);
}

#[test]
#[should_panic]
fn neg_mod_limb_ref_fail() {
    (&Natural::from(10u32)).neg_mod(0 as Limb);
}

#[test]
fn test_limb_mod_natural() {
    let test = |u: Limb, v, remainder| {
        let mut mut_u = u;
        mut_u %= Natural::from_str(v).unwrap();
        assert_eq!(mut_u, remainder);

        let mut mut_u = u;
        mut_u %= &Natural::from_str(v).unwrap();
        assert_eq!(mut_u, remainder);

        assert_eq!(u % Natural::from_str(v).unwrap(), remainder);
        assert_eq!(u % &Natural::from_str(v).unwrap(), remainder);

        let mut mut_u = u;
        mut_u.mod_assign(Natural::from_str(v).unwrap());
        assert_eq!(mut_u, remainder);

        let mut mut_u = u;
        mut_u.mod_assign(&Natural::from_str(v).unwrap());
        assert_eq!(mut_u, remainder);

        assert_eq!(u.mod_op(Natural::from_str(v).unwrap()), remainder);
        assert_eq!(u.mod_op(&Natural::from_str(v).unwrap()), remainder);
    };
    test(0, "1", 0);
    test(0, "123", 0);
    test(1, "1", 0);
    test(123, "1", 0);
    test(123, "123", 0);
    test(123, "456", 123);
    test(456, "123", 87);
    test(4_294_967_295, "1", 0);
    test(4_294_967_295, "4294967295", 0);
    test(0, "1000000000000", 0);
    test(123, "1000000000000", 123);
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn limb_rem_natural_fail() {
    10 as Limb % Natural::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn limb_rem_natural_ref_fail() {
    10 as Limb % &Natural::ZERO;
}

#[test]
#[should_panic]
fn limb_rem_assign_natural_fail() {
    let mut n: Limb = 10;
    n %= Natural::ZERO;
}

#[test]
#[should_panic]
fn limb_rem_assign_natural_ref_fail() {
    let mut n: Limb = 10;
    n %= &Natural::ZERO;
}

#[test]
#[should_panic]
fn limb_mod_natural_fail() {
    (10 as Limb).mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_mod_natural_ref_fail() {
    (10 as Limb).mod_op(&Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_mod_assign_natural_fail() {
    let mut n: Limb = 10;
    n.mod_assign(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_mod_assign_natural_ref_fail() {
    let mut n: Limb = 10;
    n.mod_assign(&Natural::ZERO);
}

#[test]
fn test_limb_neg_mod_natural() {
    let test = |u: Limb, v, remainder| {
        let n = u.neg_mod(Natural::from_str(v).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);

        let n = u.neg_mod(&Natural::from_str(v).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), remainder);
    };
    test(0, "1", "0");
    test(0, "123", "0");
    test(1, "1", "0");
    test(123, "1", "0");
    test(123, "123", "0");
    test(123, "456", "333");
    test(456, "123", "36");
    test(4_294_967_295, "1", "0");
    test(4_294_967_295, "4294967295", "0");
    test(0, "1000000000000", "0");
    test(123, "1000000000000", "999999999877");
}

#[test]
#[should_panic]
fn limb_neg_mod_natural_fail() {
    (10 as Limb).neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_neg_mod_natural_ref_fail() {
    (10 as Limb).neg_mod(&Natural::ZERO);
}

#[test]
fn limbs_mod_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            assert_eq!(
                limbs_mod_limb(limbs, limb),
                Natural::from_limbs_asc(limbs) % limb
            );
        },
    );
}

fn mod_limb_properties_helper(n: &Natural, u: Limb) {
    let mut mut_n = n.clone();
    mut_n %= u;
    assert!(mut_n.is_valid());
    let remainder = Limb::checked_from(mut_n).unwrap();

    assert_eq!(n % u, remainder);
    assert_eq!(n.clone() % u, remainder);

    let mut mut_n = n.clone();
    mut_n.mod_assign(u);
    assert!(mut_n.is_valid());
    assert_eq!(mut_n, remainder);

    assert_eq!(n.mod_op(u), remainder);
    assert_eq!(n.clone().mod_op(u), remainder);
    assert_eq!(n._mod_limb_naive(u), remainder);

    assert_eq!(n.div_mod(u).1, remainder);

    //TODO assert_eq!(n.mod_op(Natural::from(u)), remainder);

    #[cfg(feature = "32_bit_limbs")]
    {
        assert_eq!(num_rem_u32(natural_to_biguint(n), u), remainder);
        assert_eq!(natural_to_rug_integer(n) % u, remainder);
    }

    assert!(remainder < u);
}

#[test]
fn mod_limb_properties() {
    test_properties(
        pairs_of_natural_and_positive_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural,
        |&(u, ref n): &(Limb, Natural)| {
            let remainder = u % n;
            assert_eq!(u % n.clone(), remainder);

            let mut mut_u = u;
            mut_u %= n;
            assert_eq!(mut_u, remainder);

            let mut mut_u = u;
            mut_u %= n.clone();
            assert_eq!(mut_u, remainder);

            assert_eq!(u.mod_op(n), remainder);
            assert_eq!(u.mod_op(n.clone()), remainder);

            let mut mut_u = u;
            mut_u.mod_assign(n);
            assert_eq!(mut_u, remainder);

            let mut mut_u = u;
            mut_u.mod_assign(n.clone());
            assert_eq!(mut_u, remainder);

            assert_eq!(u.div_mod(n).1, remainder);

            if u != 0 && u < *n {
                assert_eq!(remainder, u);
            }
            assert!(remainder < *n);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let result = x % y;
            assert_eq!(result, Natural::from(x) % y);
            assert_eq!(result, x % Natural::from(y));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n % 1 as Limb, 0);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(u % Natural::ONE, 0);
        assert_eq!(u % Natural::from(u), 0);
        assert_eq!(Natural::ZERO % u, 0);
        if u > 1 {
            assert_eq!(Natural::ONE % u, 1);
        }
    });

    test_properties(
        triples_of_natural_natural_and_positive_unsigned::<Limb>,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y) % u,
                (Natural::from(x % u) + Natural::from(y % u)) % u,
            );
            assert_eq!(x * y % u, Natural::from(x % u) * Natural::from(y % u) % u,);
        },
    );
}

fn neg_mod_limb_properties_helper(n: &Natural, u: Limb) {
    let mut mut_n = n.clone();
    mut_n.neg_mod_assign(u);
    assert!(mut_n.is_valid());
    let remainder = Limb::checked_from(mut_n).unwrap();

    assert_eq!(n.neg_mod(u), remainder);
    assert_eq!(n.clone().neg_mod(u), remainder);

    assert_eq!(n.ceiling_div_neg_mod(u).1, remainder);

    //TODO assert_eq!(n.neg_mod(Natural::from(u)), remainder);

    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(rug_neg_mod_u32(natural_to_rug_integer(n), u), remainder);
    assert!(remainder < u);
}

#[test]
fn neg_mod_limb_properties() {
    test_properties(
        pairs_of_natural_and_positive_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            neg_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            neg_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            neg_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural,
        |&(u, ref n): &(Limb, Natural)| {
            let remainder = u.neg_mod(n);
            assert!(remainder.is_valid());

            let remainder_alt = u.neg_mod(n.clone());
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            assert_eq!(u.ceiling_div_neg_mod(n).1, remainder);

            if u != 0 && u < *n {
                assert_eq!(remainder, n - u);
            }
            assert!(remainder < *n);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let result = x.neg_mod(y);
            assert_eq!(result, Natural::from(x).neg_mod(y));
            assert_eq!(result, x.neg_mod(Natural::from(y)));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.neg_mod(1 as Limb), 0);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(u.neg_mod(Natural::ONE), 0 as Limb);
        assert_eq!(u.neg_mod(Natural::from(u)), 0 as Limb);
        assert_eq!(Natural::ZERO.neg_mod(u), 0);
        assert_eq!(Natural::ONE.neg_mod(u), u - 1);
    });

    test_properties(
        triples_of_natural_natural_and_positive_unsigned::<Limb>,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).neg_mod(u),
                (Natural::from(x % u) + Natural::from(y % u)).neg_mod(u)
            );
            assert_eq!(
                (x * y).neg_mod(u),
                (Natural::from(x % u) * Natural::from(y % u)).neg_mod(u)
            );
        },
    );
}
