use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, DivRound};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_exact_limb::{
    self, _limbs_div_exact_3_in_place_alt, _limbs_div_exact_3_to_out_alt, limbs_div_exact_3,
    limbs_div_exact_3_in_place, limbs_div_exact_3_to_out, limbs_div_exact_limb,
    limbs_div_exact_limb_in_place, limbs_div_exact_limb_to_out, limbs_modular_invert_limb,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{
    odd_limbs, pairs_of_limb_and_positive_limb_var_1, pairs_of_limb_vec_and_positive_limb_var_2,
    pairs_of_unsigned_vec_var_8, positive_unsigneds,
    triples_of_limb_vec_limb_vec_and_positive_limb_var_2, vecs_of_unsigned_var_5,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_limb_and_positive_natural_var_2, pairs_of_natural_and_limb_var_3,
    pairs_of_natural_and_positive_limb_var_1, pairs_of_natural_var_1_and_3, positive_naturals,
};
use malachite_test::natural::arithmetic::div_exact_limb::rug_div_exact_limb;

#[test]
fn test_invert_limb_table() {
    div_exact_limb::test_invert_limb_table();
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_modular_invert_limb() {
    let test = |limb, inverse| {
        assert_eq!(limbs_modular_invert_limb(limb), inverse);
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_invert_limb_fail_1() {
    limbs_modular_invert_limb(0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_invert_limb_fail_2() {
    limbs_modular_invert_limb(2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_exact_limb_and_limbs_div_exact_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, quotient: &[Limb]| {
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_fail_1() {
    limbs_div_exact_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_fail_2() {
    limbs_div_exact_limb(&[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_in_place_fail_1() {
    limbs_div_exact_limb_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_in_place_fail_2() {
    limbs_div_exact_limb_in_place(&mut [10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_exact_limb_to_out() {
    let test = |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_div_exact_limb_to_out(&mut out, limbs_in, limb);
        assert_eq!(out, out_after);
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_to_out_fail_1() {
    limbs_div_exact_limb_to_out(&mut [10, 10], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_to_out_fail_2() {
    limbs_div_exact_limb_to_out(&mut [10, 10], &[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_limb_to_out_fail_3() {
    limbs_div_exact_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_exact_3_and_limbs_div_exact_3_in_place() {
    let test = |limbs: &[Limb], quotient: &[Limb]| {
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_3_fail() {
    limbs_div_exact_3(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_3_in_place_fail() {
    limbs_div_exact_3_in_place(&mut []);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_exact_3_to_out() {
    let test = |out_before: &[Limb], limbs_in: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_div_exact_3_to_out(&mut out, limbs_in);
        assert_eq!(out, out_after);

        let mut out = out_before.to_vec();
        _limbs_div_exact_3_to_out_alt(&mut out, limbs_in);
        assert_eq!(out, out_after);
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_3_to_out_fail_1() {
    limbs_div_exact_3_to_out(&mut [10, 10], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_exact_3_to_out_fail_2() {
    limbs_div_exact_3_to_out(&mut [10], &[10, 10]);
}

#[test]
fn test_div_exact_limb() {
    let test = |u, v: Limb, quotient| {
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

        let q = rug_div_exact_limb(rug::Integer::from_str(u).unwrap(), v);
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
#[should_panic]
fn div_exact_assign_limb_fail() {
    let mut n = Natural::from(10u32);
    n.div_exact_assign(0 as Limb);
}

#[test]
#[should_panic]
fn div_exact_limb_fail() {
    Natural::from(10u32).div_exact(0 as Limb);
}

#[test]
#[should_panic]
fn div_exact_limb_ref_fail() {
    (&Natural::from(10u32)).div_exact(0 as Limb);
}

#[test]
fn test_limb_div_exact_natural() {
    let test = |u: Limb, v, quotient| {
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
    test(0, "1000000000000", 0);
}

#[test]
#[should_panic]
fn limb_div_exact_natural_fail() {
    (10 as Limb).div_exact(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_exact_natural_ref_fail() {
    (10 as Limb).div_exact(&Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_exact_assign_natural_fail() {
    let mut n: Limb = 10;
    n.div_exact_assign(Natural::ZERO);
}

#[test]
#[should_panic]
fn limb_div_exact_assign_natural_ref_fail() {
    let mut n: Limb = 10;
    n.div_exact_assign(&Natural::ZERO);
}

#[test]
fn limbs_modular_invert_limb_properties() {
    test_properties(odd_limbs, |&limb| {
        let inverse = limbs_modular_invert_limb(limb);
        assert_eq!(limb.wrapping_mul(inverse), 1);
        assert_eq!(limbs_modular_invert_limb(inverse), limb);
    });
}

#[test]
fn limbs_div_exact_limb_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_2,
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
        triples_of_limb_vec_limb_vec_and_positive_limb_var_2,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_div_exact_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out[..len]),
                Natural::from_limbs_asc(in_limbs).div_exact(limb)
            );
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_div_exact_limb_in_place_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_2,
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
        assert_eq!(
            Natural::from_limbs_asc(limbs).div_exact(3 as Limb),
            quotient_limbs
        );
    });
}

#[test]
fn limbs_div_exact_3_to_out_properties() {
    test_properties(pairs_of_unsigned_vec_var_8, |&(ref out, ref in_limbs)| {
        let mut out = out.to_vec();
        let old_out = out.clone();
        limbs_div_exact_3_to_out(&mut out, in_limbs);
        let len = in_limbs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Natural::from_limbs_asc(in_limbs).div_exact(3 as Limb)
        );
        assert_eq!(&out[len..], &old_out[len..]);

        let mut out_alt = old_out.clone();
        limbs_div_exact_limb_to_out(&mut out_alt, in_limbs, 3);
        assert_eq!(out_alt, out);

        let mut out_alt = old_out.clone();
        _limbs_div_exact_3_to_out_alt(&mut out_alt, in_limbs);
        assert_eq!(out_alt, out);
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
            Natural::from_limbs_asc(&old_limbs).div_exact(3 as Limb)
        );

        let mut limbs_alt = old_limbs.clone();
        limbs_div_exact_limb_in_place(&mut limbs_alt, 3);
        assert_eq!(limbs_alt, limbs);

        let mut limbs_alt = old_limbs.clone();
        _limbs_div_exact_3_in_place_alt(&mut limbs_alt);
        assert_eq!(limbs_alt, limbs);
    });
}

fn div_exact_limb_properties_helper(n: &Natural, u: Limb) {
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
    assert_eq!(quotient_alt, quotient);

    assert_eq!(
        rug_integer_to_natural(&rug_div_exact_limb(natural_to_rug_integer(n), u)),
        quotient
    );

    assert_eq!(quotient * Natural::from(u), *n);
}

#[test]
fn div_exact_limb_properties() {
    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            div_exact_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_limb_var_3,
        |&(ref n, u): &(Natural, Limb)| {
            div_exact_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_var_1_and_3,
        |&(ref n, u): &(Natural, Limb)| {
            div_exact_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_limb_and_positive_natural_var_2,
        |&(u, ref n): &(Limb, Natural)| {
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

            assert_eq!(Natural::from(quotient) * n, u);
        },
    );

    test_properties(pairs_of_limb_and_positive_limb_var_1, |&(x, y)| {
        let quotient = x.div_exact(y);
        assert_eq!(quotient, Natural::from(x).div_exact(y));
        assert_eq!(quotient, DivExact::div_exact(x, Natural::from(y)));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.div_exact(1 as Limb), *n);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!((0 as Limb).div_exact(n), 0);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(Natural::ZERO.div_exact(u), 0 as Limb);
        assert_eq!(u.div_exact(Natural::ONE), u);
        assert_eq!(u.div_exact(Natural::from(u)), 1);
        assert_eq!(Natural::from(u).div_exact(u), 1 as Limb);
    });
}
