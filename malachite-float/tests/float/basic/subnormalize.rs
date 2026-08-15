// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::{ComparableFloat, Float};
use malachite_nz::natural::Natural;

// Dense differential sweep against rug's subnormalize_round, which wraps mpfr_subnormalize with a
// temporarily narrowed exponent range and handles values below the smallest subnormal itself,
// exactly matching this function's extended domain.
#[test]
fn test_subnormalize_vs_rug() {
    let normal_exp_min = -20i64;
    for prec in [1u64, 2, 3, 5, 10, 24] {
        let sub_exp_min = normal_exp_min - i64::exact_from(prec) + 1;
        // exponents spanning below the smallest subnormal, the whole subnormal range, and normal
        // territory
        for exp in (sub_exp_min - 3)..=(normal_exp_min + 2) {
            // significand patterns: power of 2, all ones, a lone second bit (the q-rounding
            // midpoint shape for various q), and a mixed pattern
            let mut significands = vec![
                Natural::power_of_2(prec - 1),
                (Natural::power_of_2(prec)) - Natural::power_of_2(0u64),
            ];
            for tail in 0..prec {
                significands.push(Natural::power_of_2(prec - 1) + Natural::power_of_2(tail));
            }
            if prec >= 3 {
                significands.push(
                    Natural::power_of_2(prec - 1)
                        + Natural::power_of_2(prec - 3)
                        + Natural::power_of_2(0u64),
                );
                // - patterns whose kept mantissa at some q is odd but not all ones, with the
                //   discarded tail an exact midpoint: rounding to even then goes upward to a
                //   non-power, exercising the plain step-back in the double-rounding correction
                for tail in 1..prec - 1 {
                    significands.push(
                        Natural::power_of_2(prec - 1)
                            + Natural::power_of_2(prec - 1 - tail)
                            + Natural::power_of_2(prec - 2 - tail),
                    );
                }
            }
            for significand in significands {
                for sign in [false, true] {
                    for o in [Less, Equal, Greater] {
                        for rm in [Floor, Ceiling, Down, Up, Nearest] {
                            let x = Float::from_natural_prec(significand.clone(), prec).0
                                << (exp - i64::exact_from(significand.significant_bits()));
                            let x = if sign { -x } else { x };
                            let (ours, o_ours) = x.subnormalize_ref(o, normal_exp_min, rm);
                            // - the value, reference, and assignment variants agree
                            let (ours_val, o_val) = x.clone().subnormalize(o, normal_exp_min, rm);
                            assert_eq!(ComparableFloat(ours_val), ComparableFloat(ours.clone()));
                            assert_eq!(o_val, o_ours);
                            let mut ours_assign = x.clone();
                            let o_assign = ours_assign.subnormalize_assign(o, normal_exp_min, rm);
                            assert_eq!(ComparableFloat(ours_assign), ComparableFloat(ours.clone()));
                            assert_eq!(o_assign, o_ours);
                            let mut theirs = rug::Float::exact_from(&x);
                            let o_theirs = theirs.subnormalize_round(
                                i32::exact_from(normal_exp_min),
                                o,
                                rug_round_try_from_rounding_mode(rm).unwrap(),
                            );
                            assert_eq!(
                                ComparableFloat(
                                    Float::from_float_prec(Float::from(&theirs), prec).0
                                ),
                                ComparableFloat(ours.clone()),
                                "{prec} {exp} {significand} {sign} {o:?} {rm}"
                            );
                            assert_eq!(
                                o_theirs, o_ours,
                                "ternary: {prec} {exp} {significand} {sign} {o:?} {rm}"
                            );
                            assert!(ours.is_valid());
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn subnormalize_fail() {
    // - a value strictly inside the subnormal range that needs rounding is not exact
    let x = Float::from_natural_prec(Natural::from(0b101u32), 3).0 >> 25u64;
    assert!(std::panic::catch_unwind(|| x.subnormalize_ref(Equal, -20, Exact)).is_err());
}

#[test]
fn subnormalize_properties_like() {
    let normal_exp_min = -20i64;
    for prec in [1u64, 3, 10] {
        let sub_exp_min = normal_exp_min - i64::exact_from(prec) + 1;
        for exp in (sub_exp_min - 2)..=(normal_exp_min + 1) {
            for tail in 0..prec {
                let significand = Natural::power_of_2(prec - 1) + Natural::power_of_2(tail);
                for rm in [Floor, Ceiling, Down, Up, Nearest] {
                    let x = Float::from_natural_prec(significand.clone(), prec).0
                        << (exp - i64::exact_from(significand.significant_bits()));
                    let (y, o2) = x.subnormalize_ref(Less, normal_exp_min, rm);
                    // - the precision is preserved, except by zeros, which carry none
                    if y != 0u32 {
                        assert_eq!(y.get_prec(), Some(prec));
                    }
                    // - the result is idempotent: subnormalizing again changes nothing
                    let (z, o3) = y.subnormalize_ref(o2, normal_exp_min, rm);
                    assert_eq!(ComparableFloat(z), ComparableFloat(y.clone()));
                    assert_eq!(o3, o2);
                    // - the result is exactly representable in the emulated format: its min-prec
                    //   fits the available bits at its exponent
                    if let Some(e) = y.get_exponent() {
                        let avail = i64::from(e) - sub_exp_min + 1;
                        assert!(
                            i64::exact_from(y.get_min_prec().unwrap())
                                <= avail.min(i64::exact_from(prec))
                        );
                    }
                }
            }
        }
    }
}
