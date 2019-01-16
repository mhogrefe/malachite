use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max};
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::sub_limb::{
    limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::{biguint_to_natural, natural_to_biguint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_unsigned, pairs_of_unsigneds_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_limb_and_natural_var_1, pairs_of_natural_and_limb_var_1,
};
use num::BigUint;
#[cfg(feature = "32_bit_limbs")]
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb() {
    let test = |limbs: &[Limb], limb: Limb, out: (Vec<Limb>, bool)| {
        assert_eq!(limbs_sub_limb(limbs, limb), out);
    };
    test(&[], 0, (vec![], false));
    test(&[1], 2, (vec![4_294_967_295], true));
    test(&[6, 7], 2, (vec![4, 7], false));
    test(&[100, 101, 102], 10, (vec![90, 101, 102], false));
    test(&[123, 456], 78, (vec![45, 456], false));
    test(&[123, 456], 789, (vec![4_294_966_630, 455], false));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb_to_out() {
    let test = |limbs_out_before: &[Limb],
                limbs_in: &[Limb],
                limb: Limb,
                borrow: bool,
                limbs_out_after: &[Limb]| {
        let mut limbs_out = limbs_out_before.to_vec();
        assert_eq!(
            limbs_sub_limb_to_out(&mut limbs_out, limbs_in, limb),
            borrow
        );
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[], 0, false, &[10, 10, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[1],
        2,
        true,
        &[4_294_967_295, 10, 10, 10],
    );
    test(&[10, 10, 10, 10], &[6, 7], 2, false, &[4, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        false,
        &[90, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        78,
        false,
        &[45, 456, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        false,
        &[4_294_966_630, 455, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_sub_limb_to_out_fail() {
    limbs_sub_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, borrow: bool, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_sub_limb_in_place(&mut limbs, limb), borrow);
        assert_eq!(limbs, out);
    };
    test(&[], 0, false, &[]);
    test(&[1], 2, true, &[4_294_967_295]);
    test(&[6, 7], 2, false, &[4, 7]);
    test(&[100, 101, 102], 10, false, &[90, 101, 102]);
    test(&[123, 456], 78, false, &[45, 456]);
    test(&[123, 456], 789, false, &[4_294_966_630, 455]);
}

#[test]
fn test_sub_assign_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("1000000000000", 123, "999999999877");
    test("4294967296", 1, "4294967295");
    test("18446744073709551616", 1, "18446744073709551615");
}

#[test]
#[should_panic(expected = "Cannot subtract a Limb from a smaller Natural. self: 123, other: 456")]
fn sub_assign_limb_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= 456;
}

#[test]
fn test_sub_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!((BigUint::from_str(u).unwrap() - v).to_string(), out);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!((rug::Integer::from_str(u).unwrap() - v).to_string(), out);
    };
    test("0", 0, "0");
    test("123", 123, "0");
    test("123", 0, "123");
    test("456", 123, "333");
    test("1000000000000", 123, "999999999877");
    test("4294967296", 1, "4294967295");
    test("18446744073709551616", 1, "18446744073709551615");
}

#[test]
#[should_panic(expected = "Cannot subtract a Limb from a smaller Natural")]
#[allow(unused_must_use)]
fn sub_limb_fail_1() {
    Natural::from(123u32) - 456;
}

#[test]
#[should_panic(expected = "Cannot subtract a Limb from a smaller Natural. self: 123, other: 456")]
#[allow(unused_must_use)]
fn sub_limb_fail_2() {
    &Natural::from(123u32) - 456;
}

#[test]
fn test_limb_sub_natural() {
    let test = |u: Limb, v, out| {
        assert_eq!(u - &Natural::from_str(v).unwrap(), out);
    };
    test(0, "0", 0);
    test(123, "123", 0);
    test(123, "0", 123);
    test(456, "123", 333);
    #[cfg(feature = "32_bit_limbs")]
    test(Limb::MAX, "4294967295", 0);
    #[cfg(feature = "64_bit_limbs")]
    test(Limb::MAX, "18446744073709551615", 0);
}

#[test]
#[should_panic(expected = "Cannot subtract a Limb from a smaller Natural. self: 123, other: 456")]
#[allow(unused_must_use)]
fn limb_sub_natural_fail() {
    123 - &Natural::from_str("456").unwrap();
}

#[test]
fn limbs_sub_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let (result_limbs, borrow) = limbs_sub_limb(limbs, limb);
        if borrow {
            if limbs.is_empty() {
                assert_ne!(limb, 0);
                assert!(result_limbs.is_empty());
            } else {
                let mut result_limbs = result_limbs;
                result_limbs.push(Limb::MAX);
                assert_eq!(
                    Integer::from_owned_twos_complement_limbs_asc(result_limbs),
                    Integer::from(Natural::from_limbs_asc(limbs)) - limb
                );
            }
        } else {
            assert_eq!(
                Natural::from_owned_limbs_asc(result_limbs),
                Natural::from_limbs_asc(limbs) - limb
            );
        }
    });
}

#[test]
fn limbs_sub_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            if limbs_sub_limb_to_out(&mut out_limbs, in_limbs, limb) {
                let n = Integer::from(Natural::from_limbs_asc(in_limbs)) - limb;
                let len = in_limbs.len();
                let mut limbs = n.into_twos_complement_limbs_asc();
                limbs.resize(len, Limb::MAX);
                assert_eq!(limbs, &out_limbs[..len]);
                assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
            } else {
                let n = Natural::from_limbs_asc(in_limbs) - limb;
                let len = in_limbs.len();
                let mut limbs = n.into_limbs_asc();
                limbs.resize(len, 0);
                assert_eq!(limbs, &out_limbs[..len]);
                assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
            }
        },
    );
}

#[test]
fn limbs_sub_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        if limbs_sub_limb_in_place(&mut limbs, limb) {
            let n = Integer::from(Natural::from_limbs_asc(&old_limbs)) - limb;
            let mut expected_limbs = n.into_twos_complement_limbs_asc();
            expected_limbs.resize(limbs.len(), Limb::MAX);
            assert_eq!(limbs, expected_limbs);
        } else {
            let n = Natural::from_limbs_asc(&old_limbs) - limb;
            let mut expected_limbs = n.into_limbs_asc();
            expected_limbs.resize(limbs.len(), 0);
            assert_eq!(limbs, expected_limbs);
        }
    });
}

#[test]
fn sub_limb_properties() {
    test_properties(
        pairs_of_natural_and_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            let mut mut_n = n.clone();
            mut_n -= u;
            assert!(mut_n.is_valid());
            let difference = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = natural_to_rug_integer(n);
                rug_n -= u;
                assert_eq!(rug_integer_to_natural(&rug_n), difference);
            }

            let difference_alt = n - u;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);

            let difference_alt = n.clone() - u;
            assert!(difference_alt.is_valid());
            assert_eq!(difference_alt, difference);

            assert_eq!(n - &Natural::from(u), difference);
            assert_eq!(n - &difference, u);

            assert_eq!(biguint_to_natural(&(natural_to_biguint(n) - u)), difference);
            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) - u)),
                difference
            );

            assert!(difference <= *n);
            assert_eq!(difference + u, *n);
        },
    );

    test_properties(
        pairs_of_limb_and_natural_var_1,
        |&(u, ref n): &(Limb, Natural)| {
            let mut mut_u = u;
            mut_u -= n;
            let difference = mut_u;

            let mut mut_u = u;
            mut_u -= n.clone();
            let difference_alt = mut_u;
            assert_eq!(difference_alt, difference);

            let difference_alt = u - n;
            assert_eq!(difference_alt, difference);

            let difference_alt = u - n.clone();
            assert_eq!(difference_alt, difference);

            assert_eq!(
                Natural::from(u) - Limb::checked_from(n).unwrap(),
                difference
            );
            assert_eq!(difference + n, u);
        },
    );

    test_properties(pairs_of_unsigneds_var_1::<Limb>, |&(x, y)| {
        let difference = x - y;
        assert_eq!(difference, Natural::from(x) - y);
        assert_eq!(difference, x - Natural::from(y));
    });

    #[allow(unknown_lints, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!(n - 0, *n);
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(u - &Natural::ZERO, u);
    });
}
