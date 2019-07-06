#[cfg(not(feature = "32_bit_limbs"))]
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::JoinHalves;
use malachite_nz::natural::arithmetic::div_mod::{
    limbs_div_mod_three_limb_by_two_limb, limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
#[cfg(not(feature = "32_bit_limbs"))]
use num::{BigUint, Integer};
#[cfg(not(feature = "32_bit_limbs"))]
use rug;

use common::test_properties;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{pairs_of_unsigneds_var_2, sextuples_of_limbs_var_1};
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals,
};
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_test::natural::arithmetic::div_mod::rug_ceiling_div_neg_mod;

fn verify_limbs_two_limb_inverse_helper(hi: Limb, lo: Limb, result: Limb) {
    let b = Natural::ONE << Limb::WIDTH;
    let b_cubed_minus_1 = (Natural::ONE << (Limb::WIDTH * 3)) - 1 as Limb;
    let x = Natural::from(DoubleLimb::join_halves(hi, lo));
    //TODO use /
    let expected_result = (&b_cubed_minus_1).div_mod(&x).0 - &b;
    assert_eq!(result, expected_result);
    assert!(b_cubed_minus_1 - (result + b) * &x < x);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_two_limb_inverse_helper() {
    let test = |hi, lo, result| {
        assert_eq!(limbs_two_limb_inverse_helper(hi, lo), result);
        verify_limbs_two_limb_inverse_helper(hi, lo, result);
    };
    // hi_product >= lo
    // hi_product >= lo_product_hi
    test(0x8000_0000, 0, 0xffff_ffff);
    test(0x8000_0000, 123, 0xffff_ffff);
    test(0x8000_0123, 1, 0xffff_fb74);
    test(0xffff_ffff, 0, 1);
    // hi_product < lo
    test(0xffff_ffff, 123, 0);
    test(0xffff_f123, 1, 0xedd);
    test(0xffff_ffff, 0xffff_ffff, 0);
    // hi_product < lo_product_hi
    // !(hi_product > hi || hi_product == hi && lo_product_lo >= lo)
    test(0x8000_0001, 3, 0xffff_fffb);
    // hi_product > hi || hi_product == hi && lo_product_lo >= lo
    test(2325651385, 3907343530, 3636893938);
}

#[test]
#[should_panic]
fn limbs_two_limb_inverse_helper_fail() {
    limbs_two_limb_inverse_helper(0, 10);
}

fn verify_limbs_div_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    q: Limb,
    r: DoubleLimb,
) {
    let n = Natural::from_owned_limbs_asc(vec![n_0, n_1, n_2]);
    let d = Natural::from(DoubleLimb::join_halves(d_1, d_0));
    let r = Natural::from(r);
    assert_eq!((&n).div_mod(&d), (Natural::from(q), r.clone()));
    assert!(r < d);
    assert_eq!(q * d + r, n);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_mod_three_limb_by_two_limb() {
    let test = |n_2, n_1, n_0, d_1, d_0, q, r| {
        assert_eq!(
            limbs_div_mod_three_limb_by_two_limb(
                n_2,
                n_1,
                n_0,
                d_1,
                d_0,
                limbs_two_limb_inverse_helper(d_1, d_0)
            ),
            (q, r)
        );
        verify_limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, q, r);
    };
    // r < d
    // r.upper_half() >= q_0
    test(1, 2, 3, 0x8000_0004, 5, 1, 0x7fff_fffd_ffff_fffe);
    test(2, 0x4000_0000, 4, 0x8000_0000, 0, 4, 0x4000_0000_0000_0004);
    // r >= d
    // r.upper_half() < q_0
    test(
        1614123406,
        3687984980,
        2695202596,
        2258238141,
        1642523191,
        3069918587,
        274277675918877623,
    );
}

//TODO make 32-bit limbs work too
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_div_mod() {
    let test = |u, v, quotient, remainder| {
        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_rem(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_rem(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_rem(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_rem(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_rem(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_rem(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = BigUint::from_str(u)
            .unwrap()
            .div_mod_floor(&BigUint::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = BigUint::from_str(u)
            .unwrap()
            .div_rem(&BigUint::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug::Integer::from_str(u)
            .unwrap()
            .div_rem_floor(rug::Integer::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug::Integer::from_str(u)
            .unwrap()
            .div_rem(rug::Integer::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        //TODO
        /*
        let (q, r) = (
            Natural::from_str(u).unwrap() / v,
            Natural::from_str(u).unwrap() % v,
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
        */
    };
    test("0", "1", "0", "0");
    test("0", "123", "0", "0");
    test("1", "1", "1", "0");
    test("123", "1", "123", "0");
    test("123", "123", "1", "0");
    test("123", "456", "0", "123");
    test("456", "123", "3", "87");
    test("4294967295", "1", "4294967295", "0");
    test("4294967295", "4294967295", "1", "0");
    test("1000000000000", "1", "1000000000000", "0");
    test("1000000000000", "3", "333333333333", "1");
    test("1000000000000", "123", "8130081300", "100");
    test("1000000000000", "4294967295", "232", "3567587560");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        "3",
        "333333333333333333333333",
        "1",
    );
    test(
        "1000000000000000000000000",
        "123",
        "8130081300813008130081",
        "37",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        "232830643708079",
        "3167723695",
    );
    test(
        "1000000000000000000000000",
        "1234567890987",
        "810000006723",
        "530068894399",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
        "779655053998040854338961591319296066",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
        "37816691783627670491375998320948925696",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290113",
        "1520301762334",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "0",
        "3768477692975601",
    );
    test(
        "3356605361737854",
        "3081095617839357",
        "1",
        "275509743898497",
    );
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "1",
        "145347900158016763719",
    );
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
        "0",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
        "0",
    );
    test("0", "1000000000000000000000000", "0", "0");
    test("123", "1000000000000000000000000", "0", "123");
}

#[test]
#[should_panic]
fn div_assign_mod_fail() {
    Natural::from(10u32).div_assign_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_mod_ref_fail() {
    Natural::from(10u32).div_assign_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_fail() {
    Natural::from(10u32).div_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_val_ref_fail() {
    Natural::from(10u32).div_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_ref_val_fail() {
    (&Natural::from(10u32)).div_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_ref_ref_fail() {
    (&Natural::from(10u32)).div_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_fail() {
    Natural::from(10u32).div_assign_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_ref_fail() {
    Natural::from(10u32).div_assign_rem(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_fail() {
    Natural::from(10u32).div_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_val_ref_fail() {
    Natural::from(10u32).div_rem(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_val_fail() {
    (&Natural::from(10u32)).div_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_ref_fail() {
    (&Natural::from(10u32)).div_rem(&Natural::ZERO);
}

//TODO make 32-bit limbs work too
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_ceiling_div_neg_mod() {
    let test = |u, v, quotient, remainder| {
        let mut x = Natural::from_str(u).unwrap();
        let r = x.ceiling_div_assign_neg_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.ceiling_div_assign_neg_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .ceiling_div_neg_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .ceiling_div_neg_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) =
            (&Natural::from_str(u).unwrap()).ceiling_div_neg_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) =
            (&Natural::from_str(u).unwrap()).ceiling_div_neg_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_ceiling_div_neg_mod(
            rug::Integer::from_str(u).unwrap(),
            rug::Integer::from_str(v).unwrap(),
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        //TODO
        /*
        let (q, r) = (
            Natural::from_str(u).unwrap().div_round(v, RoundingMode::Ceiling),
            Natural::from_str(u).unwrap().neg_mod(v),
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
        */
    };
    test("0", "1", "0", "0");
    test("0", "123", "0", "0");
    test("1", "1", "1", "0");
    test("123", "1", "123", "0");
    test("123", "123", "1", "0");
    test("123", "456", "1", "333");
    test("456", "123", "4", "36");
    test("4294967295", "1", "4294967295", "0");
    test("4294967295", "4294967295", "1", "0");
    test("1000000000000", "1", "1000000000000", "0");
    test("1000000000000", "3", "333333333334", "2");
    test("1000000000000", "123", "8130081301", "23");
    test("1000000000000", "4294967295", "233", "727379735");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        "3",
        "333333333333333333333334",
        "2",
    );
    test(
        "1000000000000000000000000",
        "123",
        "8130081300813008130082",
        "86",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        "232830643708080",
        "1127243600",
    );
    test(
        "1000000000000000000000000",
        "1234567890987",
        "810000006724",
        "704498996588",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018655",
        "454912836989613466895606299668358255",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253980",
        "278232688309211835744673381771890580480",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290114",
        "1149635115107",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "1",
        "11443608136364852355",
    );
    test(
        "3356605361737854",
        "3081095617839357",
        "2",
        "2805585873940860",
    );
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "2",
        "808034397882141086757",
    );
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
        "0",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
        "0",
    );
    test("0", "1000000000000000000000000", "0", "0");
    test(
        "123",
        "1000000000000000000000000",
        "1",
        "999999999999999999999877",
    );
}

#[test]
#[should_panic]
fn ceiling_div_assign_neg_mod_fail() {
    Natural::from(10u32).ceiling_div_assign_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_assign_neg_mod_ref_fail() {
    Natural::from(10u32).ceiling_div_assign_neg_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_fail() {
    Natural::from(10u32).ceiling_div_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_val_ref_fail() {
    Natural::from(10u32).ceiling_div_neg_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_ref_val_fail() {
    (&Natural::from(10u32)).ceiling_div_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_ref_ref_fail() {
    (&Natural::from(10u32)).ceiling_div_neg_mod(&Natural::ZERO);
}

#[test]
fn limbs_two_limb_inverse_helper_properties() {
    test_properties(pairs_of_unsigneds_var_2, |&(hi, lo)| {
        let result = limbs_two_limb_inverse_helper(hi, lo);
        verify_limbs_two_limb_inverse_helper(hi, lo, result);
    });
}

#[test]
fn limbs_div_mod_three_limb_by_two_limb_properties() {
    test_properties(
        sextuples_of_limbs_var_1,
        |&(n_2, n_1, n_0, d_1, d_0, inverse)| {
            let (q, r) = limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
            verify_limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, q, r);
        },
    );
}

//TODO make 32-bit limbs work too
#[cfg(not(feature = "32_bit_limbs"))]
fn div_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    let remainder = mut_x.div_assign_mod(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    let remainder_alt = mut_x.div_assign_mod(y.clone());
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut quotient_alt = x.clone();
    let remainder_alt = quotient_alt.div_assign_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut quotient_alt = x.clone();
    let remainder_alt = quotient_alt.div_assign_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    //TODO
    /*
    let (quotient_alt, remainder_alt) = (x / y, x % y);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);
    */

    let (num_quotient, num_remainder) = natural_to_biguint(x).div_mod_floor(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_quotient), quotient);
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let (num_quotient, num_remainder) = natural_to_biguint(x).div_rem(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_quotient), quotient);
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) =
        natural_to_rug_integer(x).div_rem_floor(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    let (rug_quotient, rug_remainder) =
        natural_to_rug_integer(x).div_rem(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
    assert_eq!(quotient * y + remainder, *x);
}

//TODO make 32-bit limbs work too
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn div_mod_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        div_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            div_mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        let (q, r) = n.div_mod(Natural::ONE);
        assert_eq!(q, *n);
        assert_eq!(r, 0 as Limb);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(n.div_mod(Natural::ONE), (n.clone(), Natural::ZERO));
        assert_eq!(n.div_mod(n), (Natural::ONE, Natural::ZERO));
        assert_eq!(Natural::ZERO.div_mod(n), (Natural::ZERO, Natural::ZERO));
        if *n > 1 as Limb {
            assert_eq!(Natural::ONE.div_mod(n), (Natural::ZERO, Natural::ONE));
        }
    });
}

//TODO make 32-bit limbs work too
#[cfg(not(feature = "32_bit_limbs"))]
fn ceiling_div_neg_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    let remainder = mut_x.ceiling_div_assign_neg_mod(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    let remainder_alt = mut_x.ceiling_div_assign_neg_mod(y.clone());
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.ceiling_div_neg_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.ceiling_div_neg_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().ceiling_div_neg_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().ceiling_div_neg_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    //TODO
    /*
    let (quotient_alt, remainder_alt) = (x.div_round(y, RoundingMode::Ceiling), x.neg_mod(y));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);
    */

    let (rug_quotient, rug_remainder) =
        rug_ceiling_div_neg_mod(natural_to_rug_integer(x), natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
    assert_eq!(quotient * y - remainder, *x);
}

//TODO make 32-bit limbs work too
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn ceiling_div_neg_mod_limb_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        ceiling_div_neg_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            ceiling_div_neg_mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        let (q, r) = n.ceiling_div_neg_mod(Natural::ONE);
        assert_eq!(q, *n);
        assert_eq!(r, 0 as Limb);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(
            n.ceiling_div_neg_mod(Natural::ONE),
            (n.clone(), Natural::ZERO)
        );
        assert_eq!(n.ceiling_div_neg_mod(n), (Natural::ONE, Natural::ZERO));
        assert_eq!(
            Natural::ZERO.ceiling_div_neg_mod(n),
            (Natural::ZERO, Natural::ZERO)
        );
        if *n > 1 as Limb {
            assert_eq!(
                Natural::ONE.ceiling_div_neg_mod(n),
                (Natural::ONE, n - 1 as Limb)
            );
        }
    });
}
