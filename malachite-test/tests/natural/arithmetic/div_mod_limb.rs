use common::test_properties;
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem, DivRound,
    NegMod,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_mod_limb::{
    limbs_div_limb_in_place_mod, limbs_div_limb_mod, limbs_div_limb_to_out_mod,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_positive_unsigned, pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    positive_unsigneds, triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_limb_var_1, pairs_of_natural_and_positive_unsigned,
    pairs_of_natural_and_unsigned_var_2, pairs_of_unsigned_and_positive_natural,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::natural::arithmetic::div_mod_limb::{
    num_div_mod_u32, num_div_rem_u32, rug_ceiling_div_neg_mod_u32, rug_div_mod_u32, rug_div_rem_u32,
};
#[cfg(feature = "32_bit_limbs")]
use num::BigUint;
#[cfg(feature = "32_bit_limbs")]
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_limb_mod_and_limbs_div_limb_in_place_mod() {
    let test = |limbs: &[Limb], limb: Limb, quotient: Vec<Limb>, remainder: Limb| {
        let (quotient_alt, remainder_alt) = limbs_div_limb_mod(limbs, limb);
        assert_eq!(quotient_alt, quotient);
        assert_eq!(remainder_alt, remainder);

        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, limb), remainder);
        assert_eq!(limbs, quotient);
    };
    test(&[0, 0], 2, vec![0, 0], 0);
    test(&[6, 7], 1, vec![6, 7], 0);
    test(&[6, 7], 2, vec![2_147_483_651, 3], 0);
    test(
        &[100, 101, 102],
        10,
        vec![1_288_490_198, 858_993_469, 10],
        8,
    );
    test(&[123, 456], 789, vec![2_482_262_467, 0], 636);
    test(
        &[0xffff_ffff, 0xffff_ffff],
        2,
        vec![0xffff_ffff, 0x7fff_ffff],
        1,
    );
    test(
        &[0xffff_ffff, 0xffff_ffff],
        3,
        vec![0x5555_5555, 0x5555_5555],
        0,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_mod_fail_1() {
    limbs_div_limb_mod(&[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_mod_fail_2() {
    limbs_div_limb_mod(&[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_in_place_mod_fail_1() {
    limbs_div_limb_in_place_mod(&mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_in_place_mod_fail_2() {
    limbs_div_limb_in_place_mod(&mut [10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_limb_to_out_mod() {
    let test = |limbs_out_before: &[Limb],
                limbs_in: &[Limb],
                limb: Limb,
                remainder: Limb,
                limbs_out_after: &[Limb]| {
        let mut limbs_out = limbs_out_before.to_vec();
        assert_eq!(
            limbs_div_limb_to_out_mod(&mut limbs_out, limbs_in, limb),
            remainder
        );
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[0, 0], 2, 0, &[0, 0, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 1, 0, &[6, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[6, 7],
        2,
        0,
        &[2_147_483_651, 3, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        8,
        &[1_288_490_198, 858_993_469, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        636,
        &[2_482_262_467, 0, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 0xffff_ffff],
        2,
        1,
        &[0xffff_ffff, 0x7fff_ffff, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 0xffff_ffff],
        3,
        0,
        &[0x5555_5555, 0x5555_5555, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_to_out_mod_fail_1() {
    limbs_div_limb_to_out_mod(&mut [10], &[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_to_out_mod_fail_2() {
    limbs_div_limb_to_out_mod(&mut [10, 10], &[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_to_out_mod_fail_3() {
    limbs_div_limb_to_out_mod(&mut [10], &[10, 10], 10);
}

#[test]
fn test_div_mod_limb() {
    let test = |u, v: Limb, quotient, remainder| {
        let mut n = Natural::from_str(u).unwrap();
        assert_eq!(n.div_assign_mod(v), remainder);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let (q, r) = Natural::from_str(u).unwrap().div_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let mut n = Natural::from_str(u).unwrap();
        assert_eq!(n.div_assign_rem(v), remainder);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let (q, r) = Natural::from_str(u).unwrap().div_rem(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_rem(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = Natural::from_str(u).unwrap()._div_mod_limb_naive(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        #[cfg(feature = "32_bit_limbs")]
        {
            let (q, r) = num_div_mod_u32(BigUint::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);

            let (q, r) = num_div_rem_u32(BigUint::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);

            let (q, r) = rug_div_mod_u32(rug::Integer::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);

            let (q, r) = rug_div_rem_u32(rug::Integer::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);
        }

        let (q, r) = (
            Natural::from_str(u).unwrap() / v,
            Natural::from_str(u).unwrap() % v,
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
    };
    test("0", 1, "0", 0);
    test("0", 123, "0", 0);
    test("1", 1, "1", 0);
    test("123", 1, "123", 0);
    test("123", 123, "1", 0);
    test("123", 456, "0", 123);
    test("456", 123, "3", 87);
    test("4294967295", 1, "4294967295", 0);
    test("4294967295", 4_294_967_295, "1", 0);
    test("1000000000000", 1, "1000000000000", 0);
    test("1000000000000", 3, "333333333333", 1);
    test("1000000000000", 123, "8130081300", 100);
    test("1000000000000", 4_294_967_295, "232", 3_567_587_560);
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        0,
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333333",
        1,
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130081",
        37,
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        "232830643708079",
        3_167_723_695,
    );
}

#[test]
#[should_panic]
fn div_assign_mod_limb_fail() {
    Natural::from(10u32).div_assign_mod(0 as Limb);
}

#[test]
#[should_panic]
fn div_mod_limb_fail() {
    Natural::from(10u32).div_mod(0 as Limb);
}

#[test]
#[should_panic]
fn div_mod_limb_ref_fail() {
    (&Natural::from(10u32)).div_mod(0 as Limb);
}

#[test]
#[should_panic]
fn div_assign_rem_limb_fail() {
    Natural::from(10u32).div_assign_rem(0 as Limb);
}

#[test]
#[should_panic]
fn div_rem_limb_fail() {
    Natural::from(10u32).div_rem(0 as Limb);
}

#[test]
#[should_panic]
fn div_rem_limb_ref_fail() {
    (&Natural::from(10u32)).div_rem(0 as Limb);
}

#[test]
fn test_ceiling_div_neg_mod_limb() {
    let test = |u, v: Limb, quotient, remainder| {
        let mut n = Natural::from_str(u).unwrap();
        assert_eq!(n.ceiling_div_assign_neg_mod(v), remainder);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let (q, r) = Natural::from_str(u).unwrap().ceiling_div_neg_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).ceiling_div_neg_mod(v);
        assert_eq!(q.to_string(), quotient);
        assert!(q.is_valid());
        assert_eq!(r, remainder);

        #[cfg(feature = "32_bit_limbs")]
        {
            let (q, r) = rug_ceiling_div_neg_mod_u32(rug::Integer::from_str(u).unwrap(), v);
            assert_eq!(q.to_string(), quotient);
            assert_eq!(r, remainder);
        }
    };
    test("0", 1, "0", 0);
    test("0", 123, "0", 0);
    test("1", 1, "1", 0);
    test("123", 1, "123", 0);
    test("123", 123, "1", 0);
    test("123", 456, "1", 333);
    test("456", 123, "4", 36);
    test("4294967295", 1, "4294967295", 0);
    test("4294967295", 4_294_967_295, "1", 0);
    test("1000000000000", 1, "1000000000000", 0);
    test("1000000000000", 3, "333333333334", 2);
    test("1000000000000", 123, "8130081301", 23);
    test("1000000000000", 4_294_967_295, "233", 727_379_735);
    test(
        "1000000000000000000000000",
        1,
        "1000000000000000000000000",
        0,
    );
    test(
        "1000000000000000000000000",
        3,
        "333333333333333333333334",
        2,
    );
    test(
        "1000000000000000000000000",
        123,
        "8130081300813008130082",
        86,
    );
    test(
        "1000000000000000000000000",
        4_294_967_295,
        "232830643708080",
        1_127_243_600,
    );
}

#[test]
#[should_panic]
fn ceiling_div_assign_neg_mod_limb_fail() {
    Natural::from(10u32).ceiling_div_assign_neg_mod(0 as Limb);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_limb_fail() {
    Natural::from(10u32).ceiling_div_neg_mod(0 as Limb);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_limb_ref_fail() {
    (&Natural::from(10u32)).ceiling_div_neg_mod(0 as Limb);
}

#[test]
fn test_limb_div_mod_natural() {
    let test = |u: Limb, v, quotient, remainder| {
        let mut mut_u = u;
        assert_eq!(
            mut_u.div_assign_mod(Natural::from_str(v).unwrap()),
            remainder
        );
        assert_eq!(mut_u, quotient);

        let mut mut_u = u;
        assert_eq!(
            mut_u.div_assign_mod(&Natural::from_str(v).unwrap()),
            remainder
        );
        assert_eq!(mut_u, quotient);

        let (q, r) = u.div_mod(Natural::from_str(v).unwrap());
        assert_eq!(q, quotient);
        assert_eq!(r, remainder);

        let (q, r) = u.div_mod(&Natural::from_str(v).unwrap());
        assert_eq!(q, quotient);
        assert_eq!(r, remainder);

        let mut mut_u = u;
        assert_eq!(
            mut_u.div_assign_rem(Natural::from_str(v).unwrap()),
            remainder
        );
        assert_eq!(mut_u, quotient);

        let mut mut_u = u;
        assert_eq!(
            mut_u.div_assign_rem(&Natural::from_str(v).unwrap()),
            remainder
        );
        assert_eq!(mut_u, quotient);

        let (q, r) = u.div_rem(Natural::from_str(v).unwrap());
        assert_eq!(q, quotient);
        assert_eq!(r, remainder);

        let (q, r) = u.div_rem(&Natural::from_str(v).unwrap());
        assert_eq!(q, quotient);
        assert_eq!(r, remainder);
    };
    test(0, "1", 0, 0);
    test(0, "123", 0, 0);
    test(1, "1", 1, 0);
    test(123, "1", 123, 0);
    test(123, "123", 1, 0);
    test(123, "456", 0, 123);
    test(456, "123", 3, 87);
    test(4_294_967_295, "1", 4_294_967_295, 0);
    test(4_294_967_295, "4294967295", 1, 0);
    test(0, "1000000000000", 0, 0);
    test(123, "1000000000000", 0, 123);
}

#[test]
#[should_panic]
fn limb_div_mod_natural_fail() {
    (10 as Limb).div_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_mod_natural_ref_fail() {
    (10 as Limb).div_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_assign_mod_natural_fail() {
    (10 as Limb).div_assign_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_assign_mod_natural_ref_fail() {
    (10 as Limb).div_assign_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_rem_natural_fail() {
    (10 as Limb).div_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_rem_natural_ref_fail() {
    (10 as Limb).div_rem(&Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_assign_rem_natural_fail() {
    (10 as Limb).div_assign_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_assign_rem_natural_ref_fail() {
    (10 as Limb).div_assign_rem(&Natural::ZERO);
}

#[test]
fn test_limb_ceiling_div_neg_mod_natural() {
    let test = |u: Limb, v, quotient, remainder| {
        let mut mut_u = u;
        let r = mut_u.ceiling_div_assign_neg_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(mut_u, quotient);
        assert_eq!(r.to_string(), remainder);

        let mut mut_u = u;
        let r = mut_u.ceiling_div_assign_neg_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(mut_u, quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = u.ceiling_div_neg_mod(Natural::from_str(v).unwrap());
        assert_eq!(q, quotient);
        assert_eq!(r.to_string(), remainder);
        assert!(r.is_valid());

        let (q, r) = u.ceiling_div_neg_mod(&Natural::from_str(v).unwrap());
        assert_eq!(q, quotient);
        assert_eq!(r.to_string(), remainder);
        assert!(r.is_valid());
    };
    test(0, "1", 0, "0");
    test(0, "123", 0, "0");
    test(1, "1", 1, "0");
    test(123, "1", 123, "0");
    test(123, "123", 1, "0");
    test(123, "456", 1, "333");
    test(456, "123", 4, "36");
    test(4_294_967_295, "1", 4_294_967_295, "0");
    test(4_294_967_295, "4294967295", 1, "0");
    test(0, "1000000000000", 0, "0");
    test(123, "1000000000000", 1, "999999999877");
}

#[test]
#[should_panic]
fn limb_ceiling_div_assign_neg_mod_natural_fail() {
    (10 as Limb).ceiling_div_assign_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_ceiling_div_assign_neg_mod_natural_ref_fail() {
    (10 as Limb).ceiling_div_assign_neg_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_ceiling_div_neg_mod_natural_fail() {
    (10 as Limb).ceiling_div_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_ceiling_div_neg_mod_natural_ref_fail() {
    (10 as Limb).ceiling_div_neg_mod(&Natural::ZERO);
}

#[test]
fn limbs_div_limb_mod_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let (quotient_limbs, remainder) = limbs_div_limb_mod(limbs, limb);
            let (quotient, remainder_alt) = Natural::from_limbs_asc(limbs).div_mod(limb);
            assert_eq!(Natural::from_owned_limbs_asc(quotient_limbs), quotient);
            assert_eq!(remainder, remainder_alt);
        },
    );
}

#[test]
fn limbs_div_limb_to_out_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let remainder = limbs_div_limb_to_out_mod(&mut out, in_limbs, limb);
            let (quotient, remainder_alt) = Natural::from_limbs_asc(in_limbs).div_mod(limb);
            assert_eq!(remainder, remainder_alt);
            let len = in_limbs.len();
            assert_eq!(Natural::from_limbs_asc(&out[..len]), quotient);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_div_limb_in_place_mod_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            let remainder = limbs_div_limb_in_place_mod(&mut limbs, limb);
            let (quotient, remainder_alt) = Natural::from_limbs_asc(&old_limbs).div_mod(limb);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), quotient);
            assert_eq!(remainder, remainder_alt);
        },
    );
}

fn div_mod_limb_properties_helper(n: &Natural, u: Limb) {
    let mut mut_n = n.clone();
    let remainder = mut_n.div_assign_mod(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let mut quotient_alt = n.clone();
    let remainder_alt = quotient_alt.div_assign_rem(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.div_rem(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().div_rem(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone()._div_mod_limb_naive(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n / u, n % u);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    assert_eq!(
        n.div_mod(Natural::from(u)),
        (quotient.clone(), Natural::from(remainder))
    );

    #[cfg(feature = "32_bit_limbs")]
    {
        let (num_quotient, num_remainder) = num_div_mod_u32(natural_to_biguint(n), u);
        assert_eq!(biguint_to_natural(&num_quotient), quotient);
        assert_eq!(num_remainder, remainder);

        let (num_quotient, num_remainder) = num_div_rem_u32(natural_to_biguint(n), u);
        assert_eq!(biguint_to_natural(&num_quotient), quotient);
        assert_eq!(num_remainder, remainder);

        let (rug_quotient, rug_remainder) = rug_div_mod_u32(natural_to_rug_integer(n), u);
        assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
        assert_eq!(rug_remainder, remainder);

        let (rug_quotient, rug_remainder) = rug_div_rem_u32(natural_to_rug_integer(n), u);
        assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
        assert_eq!(rug_remainder, remainder);
    }

    assert!(remainder < u);
    assert_eq!(quotient * u + remainder, *n);
}

#[test]
fn div_mod_limb_properties() {
    test_properties(
        pairs_of_natural_and_positive_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            div_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural,
        |&(u, ref n): &(Limb, Natural)| {
            let (quotient, remainder) = u.div_mod(n);

            let (quotient_alt, remainder_alt) = u.div_mod(n.clone());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            let mut mut_u = u;
            assert_eq!(mut_u.div_assign_mod(n), remainder);
            assert_eq!(mut_u, quotient);

            let mut mut_u = u;
            assert_eq!(mut_u.div_assign_mod(n.clone()), remainder);
            assert_eq!(mut_u, quotient);

            let (quotient_alt, remainder_alt) = u.div_rem(n);
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            let (quotient_alt, remainder_alt) = u.div_rem(n.clone());
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            let mut mut_u = u;
            assert_eq!(mut_u.div_assign_rem(n), remainder);
            assert_eq!(mut_u, quotient);

            let mut mut_u = u;
            assert_eq!(mut_u.div_assign_rem(n.clone()), remainder);
            assert_eq!(mut_u, quotient);

            let (quotient_alt, remainder_alt) = (u / n, u % n);
            assert_eq!(quotient_alt, quotient);
            assert_eq!(remainder_alt, remainder);

            assert_eq!(
                Natural::from(u).div_mod(n),
                (Natural::from(quotient), Natural::from(remainder))
            );

            if u != 0 && u < *n {
                assert_eq!(remainder, u);
            }
            assert!(remainder < *n);
            assert_eq!(quotient * n + remainder, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let result = Limb::from(x).div_mod(Limb::from(y));
            let (q_alt, r_alt) = Natural::from(x).div_mod(y);
            assert_eq!(result, (Limb::checked_from(q_alt).unwrap(), r_alt));
            assert_eq!(result, x.div_mod(Natural::from(y)));
        },
    );

    test_properties(naturals, |n| {
        let (q, r) = n.div_mod(1 as Limb);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(u.div_mod(Natural::ONE), (u, 0));
        assert_eq!(u.div_mod(Natural::from(u)), (1, 0));
        assert_eq!(Natural::from(u).div_mod(u), (Natural::ONE, 0));
        assert_eq!(Natural::ZERO.div_mod(u), (Natural::ZERO, 0));
        if u > 1 {
            assert_eq!(Natural::ONE.div_mod(u), (Natural::ZERO, 1));
        }
    });
}

fn ceiling_div_neg_mod_limb_properties_helper(n: &Natural, u: Limb) {
    let mut mut_n = n.clone();
    let remainder = mut_n.ceiling_div_assign_neg_mod(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let (quotient_alt, remainder_alt) = n.ceiling_div_neg_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = n.clone().ceiling_div_neg_mod(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = (n.div_round(u, RoundingMode::Ceiling), n.neg_mod(u));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);

    assert_eq!(
        n.ceiling_div_neg_mod(Natural::from(u)),
        (quotient.clone(), Natural::from(remainder))
    );

    #[cfg(feature = "32_bit_limbs")]
    {
        let (rug_quotient, rug_remainder) =
            rug_ceiling_div_neg_mod_u32(natural_to_rug_integer(n), u);
        assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
        assert_eq!(rug_remainder, remainder);
    }

    assert!(remainder < u);
    assert_eq!(quotient * u - remainder, *n);
}

#[test]
fn ceiling_div_neg_mod_limb_properties() {
    test_properties(
        pairs_of_natural_and_positive_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            ceiling_div_neg_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            ceiling_div_neg_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            ceiling_div_neg_mod_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural,
        |&(u, ref n): &(Limb, Natural)| {
            let (quotient, remainder) = u.ceiling_div_neg_mod(n);
            assert!(remainder.is_valid());

            let (quotient_alt, remainder_alt) = u.ceiling_div_neg_mod(n.clone());
            assert_eq!(quotient_alt, quotient);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);

            let mut mut_u = u;
            let remainder_alt = mut_u.ceiling_div_assign_neg_mod(n);
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);
            assert_eq!(mut_u, quotient);

            let mut mut_u = u;
            let remainder_alt = mut_u.ceiling_div_assign_neg_mod(n.clone());
            assert!(remainder_alt.is_valid());
            assert_eq!(remainder_alt, remainder);
            assert_eq!(mut_u, quotient);

            assert_eq!(
                Natural::from(u).ceiling_div_neg_mod(n),
                (Natural::from(quotient), remainder_alt)
            );

            if u != 0 && u < *n {
                assert_eq!(remainder, n - u);
            }
            assert!(remainder < *n);
            assert_eq!(quotient * n - remainder, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let result = Limb::from(x).ceiling_div_neg_mod(Limb::from(y));
            let (q_alt, r_alt) = Natural::from(x).ceiling_div_neg_mod(y);
            assert_eq!(result, (Limb::checked_from(q_alt).unwrap(), r_alt));
            let (q_alt, r_alt) = x.ceiling_div_neg_mod(Natural::from(y));
            assert_eq!(result, (q_alt, Limb::checked_from(r_alt).unwrap()));
        },
    );

    test_properties(naturals, |n| {
        let (q, r) = n.ceiling_div_neg_mod(1 as Limb);
        assert_eq!(q, *n);
        assert_eq!(r, 0);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(u.ceiling_div_neg_mod(Natural::ONE), (u, Natural::ZERO));
        assert_eq!(u.ceiling_div_neg_mod(Natural::from(u)), (1, Natural::ZERO));
        assert_eq!(Natural::from(u).ceiling_div_neg_mod(u), (Natural::ONE, 0));
        assert_eq!(Natural::ZERO.ceiling_div_neg_mod(u), (Natural::ZERO, 0));
        assert_eq!(Natural::ONE.ceiling_div_neg_mod(u), (Natural::ONE, u - 1));
    });
}
