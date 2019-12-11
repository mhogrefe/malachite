use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{Mod, ModAssign, NegMod, NegModAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::arithmetic::mod_limb::{
    _limbs_mod_limb_alt_1, _limbs_mod_limb_alt_2, _limbs_mod_limb_alt_3,
    _limbs_mod_limb_any_leading_zeros_1, _limbs_mod_limb_any_leading_zeros_2,
    _limbs_mod_limb_at_least_1_leading_zero, _limbs_mod_limb_at_least_2_leading_zeros,
    _limbs_mod_limb_small_normalized, _limbs_mod_limb_small_unnormalized, limbs_mod_limb,
};
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
    pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1,
    pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2,
    pairs_of_nonempty_unsigned_vec_and_unsigned_var_1, pairs_of_unsigned_and_positive_unsigned,
    pairs_of_unsigned_vec_and_positive_unsigned_var_1, positive_unsigneds,
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
    let test = |limbs: &[Limb], divisor: Limb, remainder: Limb| {
        assert_eq!(limbs_mod_limb(limbs, divisor), remainder);
        assert_eq!(
            _limbs_mod_limb_any_leading_zeros_1(limbs, divisor),
            remainder
        );
        assert_eq!(
            _limbs_mod_limb_any_leading_zeros_2(limbs, divisor),
            remainder
        );
        assert_eq!(_limbs_mod_limb_alt_1(limbs, divisor), remainder);
        assert_eq!(_limbs_mod_limb_alt_2(limbs, divisor), remainder);
        assert_eq!(_limbs_mod_limb_alt_3(limbs, divisor), remainder);
    };
    test(&[0, 0], 2, 0);
    // shift != 0 in _limbs_mod_limb_any_leading_zeros_1
    // r_hi < b in _limbs_mod_limb_any_leading_zeros_1
    // n == 2 in _limbs_mod_limb_any_leading_zeros_1
    // !divisor.get_highest_bit() in _limbs_mod_limb_alt_2
    // !divisor.get_highest_bit() && len < MOD_1U_TO_MOD_1_1_THRESHOLD in _limbs_mod_limb_alt_2
    test(&[6, 7], 1, 0);
    test(&[6, 7], 2, 0);
    // n > 2 in _limbs_mod_limb_any_leading_zeros_1
    // !divisor.get_highest_bit() &&
    //      MOD_1U_TO_MOD_1_1_THRESHOLD <= len < MOD_1_1_TO_MOD_1_2_THRESHOLD
    //      in _limbs_mod_limb_alt_2
    test(&[100, 101, 102], 10, 8);
    test(&[123, 456], 789, 636);
    test(&[0, 0], 0xa000_0000, 0);
    // shift == 0 in _limbs_mod_limb_any_leading_zeros_1
    // divisor.get_highest_bit() in _limbs_mod_limb_alt_2
    // divisor.get_highest_bit() && len < MOD_1N_TO_MOD_1_1_THRESHOLD in _limbs_mod_limb_alt_2
    test(&[6, 7], 0x8000_0000, 6);
    test(&[6, 7], 0xa000_0000, 536870918);
    // divisor.get_highest_bit() && len >= MOD_1N_TO_MOD_1_1_THRESHOLD in _limbs_mod_limb_alt_2
    test(&[100, 101, 102], 0xabcd_dcba, 2152689614);
    // r_hi >= b in _limbs_mod_limb_any_leading_zeros_1
    test(&[0xffff_ffff, 0xffff_ffff], 2, 1);
    test(&[0xffff_ffff, 0xffff_ffff], 3, 0);
    test(&[0xffff_ffff, 0xffff_ffff], 0xffff_ffff, 0);
    test(&[0xffff_ffff, 0xffff_ffff], 0xa000_0000, 1610612735);
    test(&[100, 101, 102], 0xffff_ffff, 303);
    test(&[1, 2, 3, 4], 6, 1);
    // !divisor.get_highest_bit() && len >= MOD_1_1_TO_MOD_1_2_THRESHOLD &&
    //      (len < MOD_1_2_TO_MOD_1_4_THRESHOLD || divisor & HIGHEST_TWO_BITS_MASK != 0)
    //      in _limbs_mod_limb_alt_2
    test(
        &[
            3713432036, 2475243626, 3960734766, 244755020, 3760002601, 301563516, 2499010086,
            1451814771, 1299826235, 3628218184, 2565364972, 3729936002,
        ],
        565832495,
        295492150,
    );
    // !divisor.get_highest_bit() && len >= MOD_1_2_TO_MOD_1_4_THRESHOLD &&
    //      divisor & HIGHEST_TWO_BITS_MASK == 0
    //      in _limbs_mod_limb_alt_2
    test(
        &[
            540286473, 1475101238, 1863380542, 2517905739, 81646271, 3172818884, 2759300635,
            852345965, 3647245071, 3875987988, 4229899590, 4100778302, 1641902155, 1289745333,
            3414845068, 119899697, 2175381145, 2490291811, 3047506964, 1815484255, 3379971995,
            1695675424, 1418284338,
        ],
        436775226,
        165213921,
    );
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_any_leading_zeros_1_fail_1() {
    _limbs_mod_limb_any_leading_zeros_1(&[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_any_leading_zeros_1_fail_2() {
    _limbs_mod_limb_any_leading_zeros_1(&[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_any_leading_zeros_2_fail_1() {
    _limbs_mod_limb_any_leading_zeros_2(&[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_any_leading_zeros_2_fail_2() {
    _limbs_mod_limb_any_leading_zeros_2(&[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_limb_small_normalized() {
    let test = |limbs: &[Limb], divisor: Limb, remainder: Limb| {
        assert_eq!(_limbs_mod_limb_small_normalized(limbs, divisor), remainder);
    };
    test(&[0x8000_0123], 0x8000_0000, 0x123);
    test(&[0, 0], 0xa000_0000, 0);
    test(&[6, 7], 0x8000_0000, 6);
    test(&[6, 7], 0xa000_0000, 536870918);
    test(&[100, 101, 102], 0xabcd_dcba, 2152689614);
    test(&[0xffff_ffff, 0xffff_ffff], 0xffff_ffff, 0);
    test(&[0xffff_ffff, 0xffff_ffff], 0xa000_0000, 1610612735);
    test(&[100, 101, 102], 0xffff_ffff, 303);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_small_normalized_fail_1() {
    _limbs_mod_limb_small_normalized(&[], 0xffff_ffff);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_small_normalized_fail_2() {
    _limbs_mod_limb_small_normalized(&[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_limb_small_unnormalized() {
    let test = |limbs: &[Limb], divisor: Limb, remainder: Limb| {
        assert_eq!(
            _limbs_mod_limb_small_unnormalized(limbs, divisor),
            remainder
        );
        assert_eq!(
            _limbs_mod_limb_at_least_1_leading_zero(limbs, divisor),
            remainder
        );
    };
    test(&[0, 0], 2, 0);
    test(&[0], 2, 0);
    // remainder >= divisor in _limbs_mod_limb_small_unnormalized
    // len.odd() in _limbs_mod_limb_at_least_1_leading_zero
    // len == 1 in _limbs_mod_limb_at_least_1_leading_zero
    test(&[6], 2, 0);
    test(&[6], 4, 2);
    // len.even() in _limbs_mod_limb_at_least_1_leading_zero
    // len < 4 in _limbs_mod_limb_at_least_1_leading_zero
    test(&[6, 7], 1, 0);
    test(&[6, 7], 2, 0);
    // len.odd() && len != 1 in _limbs_mod_limb_at_least_1_leading_zero
    test(&[100, 101, 102], 10, 8);
    // remainder < divisor in _limbs_mod_limb_small_unnormalized
    test(&[123, 456], 789, 636);
    test(&[0xffff_ffff, 0xffff_ffff], 2, 1);
    test(&[0xffff_ffff, 0xffff_ffff], 3, 0);
    // len >= 4 in _limbs_mod_limb_at_least_1_leading_zero
    test(&[1, 2, 3, 4, 5], 6, 3);
    test(&[1, 2, 3, 4], 6, 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_small_unnormalized_fail_1() {
    _limbs_mod_limb_small_unnormalized(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_small_unnormalized_fail_2() {
    _limbs_mod_limb_small_unnormalized(&[10, 10], 0xffff_ffff);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_at_least_1_leading_zero_fail_1() {
    _limbs_mod_limb_at_least_1_leading_zero(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_at_least_1_leading_zero_fail_2() {
    _limbs_mod_limb_at_least_1_leading_zero(&[10, 10], 0xffff_ffff);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_limb_at_least_2_leading_zeros() {
    let test = |limbs: &[Limb], divisor: Limb, remainder: Limb| {
        assert_eq!(
            _limbs_mod_limb_at_least_2_leading_zeros(limbs, divisor),
            remainder
        );
    };
    test(&[0, 0], 2, 0);
    test(&[0], 2, 0);
    // len === 1 mod 4
    // len < 4
    test(&[6], 2, 0);
    test(&[6], 4, 2);
    // len === 2 mod 4
    test(&[6, 7], 1, 0);
    test(&[6, 7], 2, 0);
    // len === 3 mod 4
    test(&[100, 101, 102], 10, 8);
    test(&[123, 456], 789, 636);
    test(&[0xffff_ffff, 0xffff_ffff], 2, 1);
    test(&[0xffff_ffff, 0xffff_ffff], 3, 0);
    // len === 0 mod 4
    test(&[1, 2, 3, 4], 6, 1);
    // len >= 4
    test(&[1, 2, 3, 4, 5], 6, 3);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_at_least_2_leading_zeros_fail_1() {
    _limbs_mod_limb_at_least_2_leading_zeros(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn _limbs_mod_limb_at_least_2_leading_zeros_fail_2() {
    _limbs_mod_limb_at_least_2_leading_zeros(&[10, 10], 0x7fff_ffff);
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
        |&(ref limbs, divisor)| {
            let remainder = limbs_mod_limb(limbs, divisor);
            assert_eq!(Natural::from_limbs_asc(limbs) % divisor, remainder);
            assert_eq!(
                _limbs_mod_limb_any_leading_zeros_1(limbs, divisor),
                remainder
            );
            assert_eq!(
                _limbs_mod_limb_any_leading_zeros_2(limbs, divisor),
                remainder
            );
            assert_eq!(_limbs_mod_limb_alt_1(limbs, divisor), remainder);
            assert_eq!(_limbs_mod_limb_alt_2(limbs, divisor), remainder);
            assert_eq!(_limbs_mod_limb_alt_3(limbs, divisor), remainder);
        },
    );
}

#[test]
fn _limbs_mod_limb_small_normalized_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned_var_1,
        |&(ref limbs, divisor)| {
            let remainder = _limbs_mod_limb_small_normalized(limbs, divisor);
            assert_eq!(remainder, Natural::from_limbs_asc(limbs) % divisor);
            if limbs.len() == 1 {
                assert_eq!(remainder, limbs[0] % divisor);
            } else {
                assert_eq!(remainder, limbs_mod_limb(limbs, divisor));
            }
        },
    );
}

#[test]
fn _limbs_mod_limb_small_unnormalized_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, divisor)| {
            let remainder = _limbs_mod_limb_small_unnormalized(limbs, divisor);
            assert_eq!(
                remainder,
                _limbs_mod_limb_at_least_1_leading_zero(limbs, divisor)
            );
            assert_eq!(remainder, Natural::from_limbs_asc(limbs) % divisor);
            if limbs.len() == 1 {
                assert_eq!(remainder, limbs[0] % divisor);
            } else {
                assert_eq!(remainder, limbs_mod_limb(limbs, divisor));
            }
        },
    );
}

#[test]
fn _limbs_mod_limb_at_least_2_leading_zeros_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2,
        |&(ref limbs, divisor)| {
            let remainder = _limbs_mod_limb_at_least_2_leading_zeros(limbs, divisor);
            assert_eq!(remainder, Natural::from_limbs_asc(limbs) % divisor);
            if limbs.len() == 1 {
                assert_eq!(remainder, limbs[0] % divisor);
            } else {
                assert_eq!(remainder, limbs_mod_limb(limbs, divisor));
            }
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
            assert_eq!(x * y % u, Natural::from(x % u) * Natural::from(y % u) % u);
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

            if u != 0 && u < *n {
                assert_eq!(remainder, n - Natural::from(u));
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
