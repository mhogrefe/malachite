use common::test_properties;
use malachite_base::num::{DivExact, DivExactAssign, DivRound, One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_exact_u32::{
    self, _limbs_div_exact_3_in_place_alt, _limbs_div_exact_3_to_out_alt, limbs_div_exact_3,
    limbs_div_exact_3_in_place, limbs_div_exact_3_to_out, limbs_div_exact_limb,
    limbs_div_exact_limb_in_place, limbs_div_exact_limb_to_out, limbs_invert_limb,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{
    odd_u32s, pairs_of_u32_vec_and_positive_u32_var_2, pairs_of_u32_vec_var_3, positive_unsigneds,
    triples_of_u32_vec_u32_vec_and_positive_u32_var_2, vecs_of_unsigned_var_5,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_u32_var_2, pairs_of_natural_and_u32_var_3,
    pairs_of_u32_and_positive_natural_var_2,
};
use malachite_test::natural::arithmetic::div_exact_u32::rug_div_exact_u32;
use rug;
use std::str::FromStr;

#[test]
fn test_invert_limb_table() {
    div_exact_u32::test_invert_limb_table();
}

#[test]
fn test_limbs_invert_limb() {
    let test = |limb, inverse| {
        assert_eq!(limbs_invert_limb(limb), inverse);
    };
    test(1, 1);
    test(3, 2_863_311_531);
    test(5, 3_435_973_837);
    test(7, 3_067_833_783);
    test(123, 3_945_782_963);
    test(1_000_000_001, 2_211_001_857);
    test(2_147_483_647, 2_147_483_647);
    test(2_863_311_531, 3);
    test(4_294_967_295, 4_294_967_295);
}

#[test]
#[should_panic(expected = "assertion failed: limb.is_odd()")]
fn limbs_invert_limb_fail_1() {
    limbs_invert_limb(0);
}

#[test]
#[should_panic(expected = "assertion failed: limb.is_odd()")]
fn limbs_invert_limb_fail_2() {
    limbs_invert_limb(2);
}

#[test]
fn test_limbs_div_exact_limb_and_limbs_div_exact_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, quotient: &[u32]| {
        assert_eq!(limbs_div_exact_limb(limbs, limb), quotient);

        let mut limbs = limbs.to_vec();
        limbs_div_exact_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, quotient);
    };
    test(&[0], 2, &[0]);
    test(&[6], 2, &[3]);
    test(&[0, 0], 2, &[0, 0]);
    test(&[6, 7], 1, &[6, 7]);
    test(&[6, 7], 2, &[2_147_483_651, 3]);
    test(&[92, 101, 102], 10, &[1_288_490_198, 858_993_469, 10]);
    test(&[4_294_966_783, 455], 789, &[2_482_262_467, 0]);
    test(&[0xffff_fffe, 0xffff_ffff], 2, &[0xffff_ffff, 0x7fff_ffff]);
    test(&[0xffff_ffff, 0xffff_ffff], 3, &[0x5555_5555, 0x5555_5555]);
}

#[test]
#[should_panic(expected = "assertion failed: len > 0")]
fn limbs_div_exact_limb_fail_1() {
    limbs_div_exact_limb(&[], 10);
}

#[test]
#[should_panic(expected = "assertion failed: divisor > 0")]
fn limbs_div_exact_limb_fail_2() {
    limbs_div_exact_limb(&[10, 10], 0);
}

#[test]
#[should_panic(expected = "assertion failed: len > 0")]
fn limbs_div_exact_limb_in_place_fail_1() {
    limbs_div_exact_limb_in_place(&mut [], 10);
}

#[test]
#[should_panic(expected = "assertion failed: divisor > 0")]
fn limbs_div_exact_limb_in_place_fail_2() {
    limbs_div_exact_limb_in_place(&mut [10, 10], 0);
}

#[test]
fn test_limbs_div_exact_limb_to_out() {
    let test = |limbs_out_before: &[u32], limbs_in: &[u32], limb: u32, limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        limbs_div_exact_limb_to_out(&mut limbs_out, limbs_in, limb);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[0], 2, &[0, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6], 2, &[3, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[0, 0], 2, &[0, 0, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 1, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[2_147_483_651, 3, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[92, 101, 102],
        10,
        &[1_288_490_198, 858_993_469, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[4_294_966_783, 455],
        789,
        &[2_482_262_467, 0, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_fffe, 0xffff_ffff],
        2,
        &[0xffff_ffff, 0x7fff_ffff, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 0xffff_ffff],
        3,
        &[0x5555_5555, 0x5555_5555, 10, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: len > 0")]
fn limbs_div_exact_limb_to_out_fail_1() {
    limbs_div_exact_limb_to_out(&mut [10, 10], &[], 10);
}

#[test]
#[should_panic(expected = "assertion failed: divisor > 0")]
fn limbs_div_exact_limb_to_out_fail_2() {
    limbs_div_exact_limb_to_out(&mut [10, 10], &[10, 10], 0);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_div_exact_limb_to_out_fail_3() {
    limbs_div_exact_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_div_exact_3_and_limbs_div_exact_3_in_place() {
    let test = |limbs: &[u32], quotient: &[u32]| {
        let old_limbs = limbs.clone();
        assert_eq!(limbs_div_exact_3(limbs), quotient);

        let mut limbs = old_limbs.to_vec();
        limbs_div_exact_3_in_place(&mut limbs);
        assert_eq!(limbs, quotient);

        let mut limbs = old_limbs.to_vec();
        _limbs_div_exact_3_in_place_alt(&mut limbs);
        assert_eq!(limbs, quotient);
    };
    test(&[0], &[0]);
    test(&[6], &[2]);
    test(&[0, 0], &[0, 0]);
    test(&[8, 7], &[1_431_655_768, 2]);
    test(&[100, 101, 102], &[2_863_311_564, 33, 34]);
    test(&[0xffff_ffff, 0xffff_ffff], &[0x5555_5555, 0x5555_5555]);
}

#[test]
#[should_panic(expected = "assertion failed: len > 0")]
fn limbs_div_exact_3_fail() {
    limbs_div_exact_3(&[]);
}

#[test]
#[should_panic(expected = "assertion failed: len > 0")]
fn limbs_div_exact_3_in_place_fail() {
    limbs_div_exact_3_in_place(&mut []);
}

#[test]
fn test_limbs_div_exact_3_to_out() {
    let test = |limbs_out_before: &[u32], limbs_in: &[u32], limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        limbs_div_exact_3_to_out(&mut limbs_out, limbs_in);
        assert_eq!(limbs_out, limbs_out_after);

        let mut limbs_out = limbs_out_before.to_vec();
        _limbs_div_exact_3_to_out_alt(&mut limbs_out, limbs_in);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[0], &[0, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6], &[2, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[0, 0], &[0, 0, 10, 10]);
    test(&[10, 10, 10, 10], &[8, 7], &[1_431_655_768, 2, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        &[2_863_311_564, 33, 34, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 0xffff_ffff],
        &[0x5555_5555, 0x5555_5555, 10, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: len > 0")]
fn limbs_div_exact_3_to_out_fail_1() {
    limbs_div_exact_3_to_out(&mut [10, 10], &[]);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_div_exact_3_to_out_fail_2() {
    limbs_div_exact_3_to_out(&mut [10], &[10, 10]);
}

#[test]
fn test_div_exact_u32() {
    let test = |u, v: u32, quotient| {
        let mut n = Natural::from_str(u).unwrap();
        n.div_exact_assign(v);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Natural::from_str(u).unwrap().div_exact(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&Natural::from_str(u).unwrap()).div_exact(v);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Natural::from_str(u)
            .unwrap()
            .div_round(v, RoundingMode::Exact);
        assert_eq!(q.to_string(), quotient);

        let q = rug_div_exact_u32(rug::Integer::from_str(u).unwrap(), v);
        assert_eq!(q.to_string(), quotient);
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "1");
    test("123", 1, "123");
    test("123", 123, "1");
    test("912", 456, "2");
    test("369", 123, "3");
    test("4294967295", 1, "4294967295");
    test("4294967295", 4_294_967_295, "1");
    test("1000000000000", 1, "1000000000000");
    test("333333333333", 3, "111111111111");
    test("999999999900", 123, "8130081300");
    test("996432412440", 4_294_967_295, "232");
    test("1000000000000000000000000", 1, "1000000000000000000000000");
    test("333333333333333333333333", 3, "111111111111111111111111");
    test("999999999999999999999963", 123, "8130081300813008130081");
    test("999999999999996832276305", 4_294_967_295, "232830643708079");
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_exact_assign_u32_fail() {
    let mut n = Natural::from(10u32);
    n.div_exact_assign(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_exact_u32_fail() {
    Natural::from(10u32).div_exact(0);
}

#[test]
#[should_panic(expected = "division by zero")]
fn div_exact_u32_ref_fail() {
    (&Natural::from(10u32)).div_exact(0);
}

#[test]
fn test_u32_div_exact_natural() {
    let test = |u: u32, v, quotient| {
        let mut mut_u = u;
        mut_u.div_exact_assign(Natural::from_str(v).unwrap());
        assert_eq!(mut_u, quotient);

        let mut mut_u = u;
        mut_u.div_exact_assign(&Natural::from_str(v).unwrap());
        assert_eq!(mut_u, quotient);

        assert_eq!(u.div_exact(Natural::from_str(v).unwrap()), quotient);
        assert_eq!(u.div_exact(&Natural::from_str(v).unwrap()), quotient);
    };
    test(0, "1", 0);
    test(0, "123", 0);
    test(1, "1", 1);
    test(123, "1", 123);
    test(123, "123", 1);
    test(912, "456", 2);
    test(369, "123", 3);
    test(4_294_967_295, "1", 4_294_967_295);
    test(4_294_967_295, "4294967295", 1);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_exact_natural_fail() {
    10.div_exact(Natural::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_exact_natural_ref_fail() {
    10.div_exact(&Natural::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_exact_assign_natural_fail() {
    let mut n = 10;
    n.div_exact_assign(Natural::ZERO);
}

#[test]
#[should_panic(expected = "division by zero")]
fn u32_div_exact_assign_natural_ref_fail() {
    let mut n = 10;
    n.div_exact_assign(&Natural::ZERO);
}

#[test]
fn limbs_invert_limb_properties() {
    test_properties(odd_u32s, |&limb| {
        let inverse = limbs_invert_limb(limb);
        assert_eq!(limb.wrapping_mul(inverse), 1);
        assert_eq!(limbs_invert_limb(inverse), limb);
    });
}

#[test]
fn limbs_div_exact_limb_properties() {
    test_properties(
        pairs_of_u32_vec_and_positive_u32_var_2,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_div_exact_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs).div_exact(limb)
            );
        },
    );
}

#[test]
fn limbs_div_exact_limb_to_out_properties() {
    test_properties(
        triples_of_u32_vec_u32_vec_and_positive_u32_var_2,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_div_exact_limb_to_out(&mut out_limbs, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out_limbs[..len]),
                Natural::from_limbs_asc(in_limbs).div_exact(limb)
            );
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_div_exact_limb_in_place_properties() {
    test_properties(
        pairs_of_u32_vec_and_positive_u32_var_2,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_div_exact_limb_in_place(&mut limbs, limb);
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs),
                Natural::from_limbs_asc(&old_limbs).div_exact(limb)
            );
        },
    );
}

#[test]
fn limbs_div_exact_3_properties() {
    test_properties(vecs_of_unsigned_var_5, |ref limbs| {
        let quotient_limbs = Natural::from_owned_limbs_asc(limbs_div_exact_3(limbs));
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_div_exact_limb(limbs, 3)),
            quotient_limbs,
        );
        assert_eq!(Natural::from_limbs_asc(limbs).div_exact(3), quotient_limbs);
    });
}

#[test]
fn limbs_div_exact_3_to_out_properties() {
    test_properties(pairs_of_u32_vec_var_3, |&(ref out_limbs, ref in_limbs)| {
        let mut out_limbs = out_limbs.to_vec();
        let old_out_limbs = out_limbs.clone();
        limbs_div_exact_3_to_out(&mut out_limbs, in_limbs);
        let len = in_limbs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out_limbs[..len]),
            Natural::from_limbs_asc(in_limbs).div_exact(3)
        );
        assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);

        let mut out_limbs_alt = old_out_limbs.clone();
        limbs_div_exact_limb_to_out(&mut out_limbs_alt, in_limbs, 3);
        assert_eq!(out_limbs_alt, out_limbs);

        let mut out_limbs_alt = old_out_limbs.clone();
        _limbs_div_exact_3_to_out_alt(&mut out_limbs_alt, in_limbs);
        assert_eq!(out_limbs_alt, out_limbs);
    });
}

#[test]
fn limbs_div_exact_3_in_place_properties() {
    test_properties(vecs_of_unsigned_var_5, |ref limbs| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_div_exact_3_in_place(&mut limbs);
        assert_eq!(
            Natural::from_limbs_asc(&limbs),
            Natural::from_limbs_asc(&old_limbs).div_exact(3)
        );

        let mut limbs_alt = old_limbs.clone();
        limbs_div_exact_limb_in_place(&mut limbs_alt, 3);
        assert_eq!(limbs_alt, limbs);

        let mut limbs_alt = old_limbs.clone();
        _limbs_div_exact_3_in_place_alt(&mut limbs_alt);
        assert_eq!(limbs_alt, limbs);
    });
}

fn div_exact_u32_properties_helper(n: &Natural, u: u32) {
    let mut mut_n = n.clone();
    mut_n.div_exact_assign(u);
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let quotient_alt = n.div_exact(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.clone().div_exact(u);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.div_round(u, RoundingMode::Exact);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    //TODO assert_eq!(n.div_exact(Natural::from(u)), quotient);

    assert_eq!(
        rug_integer_to_natural(&rug_div_exact_u32(natural_to_rug_integer(n), u)),
        quotient
    );

    assert_eq!(quotient * u, *n);
}

#[test]
fn div_exact_u32_properties() {
    test_properties(
        pairs_of_natural_and_positive_u32_var_2,
        |&(ref n, u): &(Natural, u32)| {
            div_exact_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_u32_var_3,
        |&(ref n, u): &(Natural, u32)| {
            div_exact_u32_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_u32_and_positive_natural_var_2,
        |&(u, ref n): &(u32, Natural)| {
            let mut mut_u = u;
            mut_u.div_exact_assign(n);
            let quotient = mut_u;

            let mut mut_u = u;
            mut_u.div_exact_assign(n.clone());
            assert_eq!(mut_u, quotient);

            let quotient_alt = u.div_exact(n);
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = u.div_exact(n.clone());
            assert_eq!(quotient_alt, quotient);

            assert_eq!(u.div_round(n, RoundingMode::Exact), quotient);

            assert_eq!(quotient * n, u);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.div_exact(1), *n);
    });

    test_properties(positive_unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO.div_exact(u), 0);
        assert_eq!(u.div_exact(Natural::ONE), u);
    });
}
