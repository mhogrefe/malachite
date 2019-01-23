use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::arithmetic::add_limb::{
    limbs_add_limb, limbs_add_limb_to_out, limbs_slice_add_limb_in_place,
    limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_test::common::{biguint_to_natural, natural_to_biguint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_and_unsigned,
    pairs_of_unsigneds, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::add_limb::num_add_limb;
use num::BigUint;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_limb() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_add_limb(limbs, limb), out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[5]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[912, 456]);
    test(&[0xffff_ffff, 5], 2, &[1, 6]);
    test(&[0xffff_ffff], 2, &[1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_limb_to_out() {
    let test = |limbs_out_before: &[Limb],
                limbs_in: &[Limb],
                limb: Limb,
                carry: bool,
                limbs_out_after: &[Limb]| {
        let mut limbs_out = limbs_out_before.to_vec();
        assert_eq!(limbs_add_limb_to_out(&mut limbs_out, limbs_in, limb), carry);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[], 0, false, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[], 5, true, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, false, &[8, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        false,
        &[110, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        false,
        &[912, 456, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 5],
        2,
        false,
        &[1, 6, 10, 10],
    );
    test(&[10, 10, 10, 10], &[0xffff_ffff], 2, true, &[1, 10, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_limb_to_out_fail() {
    limbs_add_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, carry: bool, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_add_limb_in_place(&mut limbs, limb), carry);
        assert_eq!(limbs, out);
    };
    test(&[], 0, false, &[]);
    test(&[], 5, true, &[]);
    test(&[6, 7], 2, false, &[8, 7]);
    test(&[100, 101, 102], 10, false, &[110, 101, 102]);
    test(&[123, 456], 789, false, &[912, 456]);
    test(&[0xffff_ffff, 5], 2, false, &[1, 6]);
    test(&[0xffff_ffff], 2, true, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        limbs_vec_add_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[912, 456]);
    test(&[0xffff_ffff, 5], 2, &[1, 6]);
    test(&[0xffff_ffff], 2, &[1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_add_limb_in_place_fail() {
    limbs_vec_add_limb_in_place(&mut vec![], 10);
}

#[test]
fn test_add_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Natural::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(u).unwrap();
            n += v;
            assert_eq!(n.to_string(), out);
        }

        let n = Natural::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_limb(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() + v;
            assert_eq!(n.to_string(), out);
        }

        let n = &Natural::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = v + rug::Integer::from_str(u).unwrap();
            assert_eq!(n.to_string(), out);
        }

        let n = v + &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from(0);
            n.assign(v + &rug::Integer::from_str(u).unwrap());
            assert_eq!(n.to_string(), out);
        }
    };
    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("1000000000000", 123, "1000000000123");
    test("4294967295", 1, "4294967296");
    test("18446744073709551615", 1, "18446744073709551616");
}

#[test]
fn limbs_add_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_add_limb(limbs, limb)),
            Natural::from_limbs_asc(limbs) + limb
        );
    });
}

#[test]
fn limbs_add_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            let carry = limbs_add_limb_to_out(&mut out_limbs, in_limbs, limb);
            let n = Natural::from_limbs_asc(in_limbs) + limb;
            let len = in_limbs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out_limbs[..len]);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_slice_add_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_add_limb_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) + limb;
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry, expected_limbs.len() == limbs.len() + 1);
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_add_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_add_limb_in_place(&mut limbs, limb);
            let n = Natural::from_limbs_asc(&old_limbs) + limb;
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn add_limb_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            let mut mut_n = n.clone();
            mut_n += u;
            assert!(mut_n.is_valid());
            let sum = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = natural_to_rug_integer(n);
                rug_n += u;
                assert_eq!(rug_integer_to_natural(&rug_n), sum);
            }

            let sum_alt = n + u;
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            let sum_alt = n.clone() + u;
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            let sum_alt = u + n;
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            let sum_alt = u + n.clone();
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            assert_eq!(n + Natural::from(u), sum);
            assert_eq!(Natural::from(u) + n, sum);

            assert_eq!(
                biguint_to_natural(&num_add_limb(natural_to_biguint(n), u)),
                sum
            );
            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) + u)),
                sum
            );

            assert!(sum >= *n);
            assert!(sum >= u);
            assert_eq!(&sum - u, *n);
            assert_eq!(sum - n, u);
        },
    );

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        let sum = Natural::from(DoubleLimb::from(x) + DoubleLimb::from(y));
        assert_eq!(sum, Natural::from(x) + y);
        assert_eq!(sum, x + Natural::from(y));
    });

    #[allow(unknown_lints, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!(n + 0 as Limb, *n);
        assert_eq!(0 as Limb + n, *n);
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(Natural::ZERO + u, u);
        assert_eq!(u + Natural::ZERO, u);
    });
}
