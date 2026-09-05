// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::cmp::max;
use gmp_mpfr_sys::mpfr::{self, rnd_t};
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{Abs, ModPowerOf2, NegAssign, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{primitive_float_gen, primitive_float_pair_gen};
use malachite_float::float::arithmetic::rem::{
    primitive_float_ieee_remainder, primitive_float_ieee_remainder_and_quotient_bits,
    primitive_float_ieee_remainder_rational,
    primitive_float_ieee_remainder_rational_and_quotient_bits,
    primitive_float_rational_ieee_remainder_float, primitive_float_rational_rem_float,
    primitive_float_rem, primitive_float_rem_and_quotient_bits, primitive_float_rem_rational,
    primitive_float_rem_rational_and_quotient_bits, primitive_float_rem_unsigned,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::rem::{
    rug_ieee_remainder, rug_ieee_remainder_prec_round, rug_rem, rug_rem_prec, rug_rem_prec_round,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_40, float_float_rounding_mode_triple_gen_var_41,
    float_float_unsigned_rounding_mode_quadruple_gen_var_18,
    float_float_unsigned_rounding_mode_quadruple_gen_var_19,
    float_float_unsigned_rounding_mode_quadruple_gen_var_20,
    float_float_unsigned_rounding_mode_quadruple_gen_var_21, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_2, float_pair_gen, float_pair_gen_var_10,
    float_rational_pair_gen, float_rational_rounding_mode_triple_gen_var_16,
    float_rational_rounding_mode_triple_gen_var_17, float_rational_rounding_mode_triple_gen_var_18,
    float_rational_rounding_mode_triple_gen_var_19,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_17,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_18,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_19,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_20,
    float_rational_unsigned_triple_gen_var_1, float_unsigned_pair_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use std::panic::catch_unwind;

const fn mpfr_rnd(rm: RoundingMode) -> rnd_t {
    match rm {
        Floor => rnd_t::RNDD,
        Ceiling => rnd_t::RNDU,
        Down => rnd_t::RNDZ,
        Up => rnd_t::RNDA,
        Nearest => rnd_t::RNDN,
        Exact => panic!(),
    }
}

const fn ternary_sign(o: Ordering) -> i32 {
    match o {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

// Raw-FFI oracles for the functions rug does not bind (the quotient-bits pair and the u64 modulus).
// `forbid(unsafe_code)` keeps these out of the crate's test_util, but this integration test crate
// may use unsafe, so they live here and are used inside the unit-test closures and property
// helpers, following the usual rug-helper pattern.
fn mpfr_fmodquo_oracle(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> (Float, i32, i64) {
    let bx = rug::Float::exact_from(x);
    let by = rug::Float::exact_from(y);
    let mut r = rug::Float::new(u32::exact_from(prec));
    let mut quo = 0i64;
    let t = unsafe {
        mpfr::fmodquo(
            r.as_raw_mut(),
            &raw mut quo,
            bx.as_raw(),
            by.as_raw(),
            mpfr_rnd(rm),
        )
    };
    (Float::from(&r), t.signum(), quo)
}

fn mpfr_remquo_oracle(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> (Float, i32, i64) {
    let bx = rug::Float::exact_from(x);
    let by = rug::Float::exact_from(y);
    let mut r = rug::Float::new(u32::exact_from(prec));
    let mut quo = 0i64;
    let t = unsafe {
        mpfr::remquo(
            r.as_raw_mut(),
            &raw mut quo,
            bx.as_raw(),
            by.as_raw(),
            mpfr_rnd(rm),
        )
    };
    (Float::from(&r), t.signum(), quo)
}

fn mpfr_fmod_ui_oracle(x: &Float, u: u64, prec: u64, rm: RoundingMode) -> (Float, i32) {
    let bx = rug::Float::exact_from(x);
    let mut r = rug::Float::new(u32::exact_from(prec));
    let t = unsafe { mpfr::fmod_ui(r.as_raw_mut(), bx.as_raw(), u, mpfr_rnd(rm)) };
    (Float::from(&r), t.signum())
}

// Checks a quotient-bits result against the corresponding MPFR function. The bits are compared
// modulo 2^63: when the low 63 bits are all ones and the nearest-quotient rounds away, the C code
// increments a long past LONG_MAX (undefined behavior, wrapping in practice) where we implement the
// documented low-63-bits contract, so on any mismatch our value must be the wrapped 0.
fn check_quo_vs_mpfr(quo: i64, mpfr_quo: i64) {
    assert_eq!(
        quo.unsigned_abs() & u64::low_mask(63),
        mpfr_quo.unsigned_abs() & u64::low_mask(63)
    );
    if quo != mpfr_quo {
        assert_eq!(quo, 0);
    }
}

// Value patterns spanning both remainder regimes: exponent gaps in both directions (including gaps
// large enough for the modular-exponentiation path), tiny-quotient cases, exact multiples, ties,
// single-bit and all-ones significands, and both signs.
fn sweep_values() -> Vec<Float> {
    let mut xs = Vec::new();
    for prec in [1u64, 10, 64] {
        let mut sigs = vec![Natural::power_of_2(prec - 1), Natural::low_mask(prec)];
        if prec > 2 {
            sigs.push(Natural::power_of_2(prec - 1) + Natural::ONE);
            sigs.push(Natural::power_of_2(prec - 1) + Natural::power_of_2(prec / 2));
        }
        sigs.sort_unstable();
        sigs.dedup();
        for sig in sigs {
            for exp in [-500i64, -50, -3, -1, 0, 1, 2, 7, 50, 500, 5000] {
                let x =
                    Float::from_natural_prec(sig.clone(), prec).0 << (exp - i64::exact_from(prec));
                xs.push(x.clone());
                xs.push(-x);
            }
        }
    }
    xs
}

#[allow(clippy::needless_pass_by_value)]
fn rem_prec_round_properties_helper(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (rem, o) = x.clone().rem_prec_round(y.clone(), prec, rm);
    assert!(rem.is_valid());
    let (rem_alt, o_alt) = x.clone().rem_prec_round_val_ref(&y, prec, rm);
    assert!(rem_alt.is_valid());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.rem_prec_round_ref_val(y.clone(), prec, rm);
    assert!(rem_alt.is_valid());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.rem_prec_round_ref_ref(&y, prec, rm);
    assert!(rem_alt.is_valid());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.rem_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.rem_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    // the quotient-bits variant computes the same remainder
    let (rem_alt, o_alt, quo) = x.rem_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_rem, rug_o) = rug_rem_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_rem)),
            ComparableFloatRef(&rem)
        );
        assert_eq!(rug_o, o);
    }
    if rm != Exact {
        let (mpfr_rem, mpfr_t, mpfr_quo) = mpfr_fmodquo_oracle(&x, &y, prec, rm);
        assert_eq!(ComparableFloatRef(&mpfr_rem), ComparableFloatRef(&rem));
        assert_eq!(mpfr_t, ternary_sign(o));
        check_quo_vs_mpfr(quo, mpfr_quo);
    }

    if rem.is_normal() {
        assert_eq!(rem.get_prec(), Some(prec));
    }

    if !extreme && x.is_finite() && y.is_finite() && x != 0u32 && y != 0u32 {
        let rx = Rational::exact_from(&x);
        let ry = Rational::exact_from(&y);
        let (q, _) = Integer::rounding_from(&rx / &ry, Down);
        let r_exact = rx - Rational::from(q) * &ry;
        // fmod: the remainder is zero or has the sign of x, and is smaller than y in magnitude
        if r_exact == 0u32 {
            assert_eq!(o, Equal);
            let expected = if x > 0u32 {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            };
            assert_eq!(ComparableFloatRef(&rem), ComparableFloatRef(&expected));
        } else {
            assert_eq!((r_exact > 0u32), (x > 0u32));
            assert!(r_exact.lt_abs(&ry));
            let (rem_alt, o_alt) = Float::from_rational_prec_round(r_exact.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            assert_eq!(rem.partial_cmp(&r_exact), Some(o));
            if o == Less {
                let mut next = rem.clone();
                next.increment();
                assert!(next > r_exact);
            } else if o == Greater {
                let mut next = rem.clone();
                next.decrement();
                assert!(next < r_exact);
            }
            match (r_exact >= 0u32, rm) {
                (_, Floor) | (true, Down) | (false, Up) => {
                    assert_ne!(o, Greater);
                }
                (_, Ceiling) | (true, Up) | (false, Down) => {
                    assert_ne!(o, Less);
                }
                (_, Exact) => assert_eq!(o, Equal),
                _ => {}
            }
        }
    }

    // rem(-x, y) = -rem(x, y)
    let (mut rem_alt, mut o_alt) = (-&x).rem_prec_round_val_ref(&y, prec, -rm);
    rem_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(rem_alt.abs_negative_zero()),
        ComparableFloat(rem.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    // rem(x, -y) = rem(x, y)
    let (rem_alt, o_alt) = x.rem_prec_round_ref_val(-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.rem_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(rem.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.rem_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn rem_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, y, prec, rm)| {
            rem_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );

    float_float_unsigned_rounding_mode_quadruple_gen_var_19().test_properties(
        |(x, y, prec, rm)| {
            rem_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_float_unsigned_rounding_mode_quadruple_gen_var_18().test_properties_with_config(
        &config,
        |(x, y, prec, rm)| {
            rem_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn ieee_remainder_prec_round_properties_helper(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (rem, o) = x.clone().ieee_remainder_prec_round(y.clone(), prec, rm);
    assert!(rem.is_valid());
    let (rem_alt, o_alt) = x.clone().ieee_remainder_prec_round_val_ref(&y, prec, rm);
    assert!(rem_alt.is_valid());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_val(y.clone(), prec, rm);
    assert!(rem_alt.is_valid());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_ref(&y, prec, rm);
    assert!(rem_alt.is_valid());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ieee_remainder_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ieee_remainder_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt, quo) = x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_rem, rug_o) = rug_ieee_remainder_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_rem)),
            ComparableFloatRef(&rem)
        );
        assert_eq!(rug_o, o);
    }
    if rm != Exact {
        let (mpfr_rem, mpfr_t, mpfr_quo) = mpfr_remquo_oracle(&x, &y, prec, rm);
        assert_eq!(ComparableFloatRef(&mpfr_rem), ComparableFloatRef(&rem));
        assert_eq!(mpfr_t, ternary_sign(o));
        check_quo_vs_mpfr(quo, mpfr_quo);
    }

    if rem.is_normal() {
        assert_eq!(rem.get_prec(), Some(prec));
    }

    if !extreme && x.is_finite() && y.is_finite() && x != 0u32 && y != 0u32 {
        let rx = Rational::exact_from(&x);
        let ry = Rational::exact_from(&y);
        let (q, _) = Integer::rounding_from(&rx / &ry, Nearest);
        let r_exact = rx - Rational::from(q) * &ry;
        // remainder: |r| <= |y|/2, and r = 0 takes the sign of x
        assert!((&r_exact).abs() << 1u32 <= ry.abs());
        if r_exact == 0u32 {
            assert_eq!(o, Equal);
            let expected = if x > 0u32 {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            };
            assert_eq!(ComparableFloatRef(&rem), ComparableFloatRef(&expected));
        } else {
            let (rem_alt, o_alt) = Float::from_rational_prec_round(r_exact.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            assert_eq!(rem.partial_cmp(&r_exact), Some(o));
        }
    }

    // ieee_remainder(-x, y) = -ieee_remainder(x, y)
    let (mut rem_alt, mut o_alt) = (-&x).ieee_remainder_prec_round_val_ref(&y, prec, -rm);
    rem_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(rem_alt.abs_negative_zero()),
        ComparableFloat(rem.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    // ieee_remainder(x, -y) = ieee_remainder(x, y)
    let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_val(-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ieee_remainder_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(rem.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ieee_remainder_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn ieee_remainder_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_20().test_properties(
        |(x, y, prec, rm)| {
            ieee_remainder_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );

    float_float_unsigned_rounding_mode_quadruple_gen_var_21().test_properties(
        |(x, y, prec, rm)| {
            ieee_remainder_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_float_unsigned_rounding_mode_quadruple_gen_var_20().test_properties_with_config(
        &config,
        |(x, y, prec, rm)| {
            ieee_remainder_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn rem_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (rem, o) = x.clone().rem_prec(y.clone(), prec);
    assert!(rem.is_valid());
    let (rem_alt, o_alt) = x.clone().rem_prec_val_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.rem_prec_ref_val(y.clone(), prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.rem_prec_ref_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.rem_prec_assign(y.clone(), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.rem_prec_assign_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt) = x.rem_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt, _) = x.rem_and_quotient_bits_prec_ref_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
}

#[test]
fn rem_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        rem_prec_properties_helper(x, y, prec);
    });

    float_float_unsigned_triple_gen_var_2().test_properties(|(x, y, prec)| {
        rem_prec_properties_helper(x, y, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ieee_remainder_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (rem, o) = x.clone().ieee_remainder_prec(y.clone(), prec);
    assert!(rem.is_valid());
    let (rem_alt, o_alt) = x.clone().ieee_remainder_prec_val_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.ieee_remainder_prec_ref_val(y.clone(), prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.ieee_remainder_prec_ref_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ieee_remainder_prec_assign(y.clone(), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.ieee_remainder_prec_assign_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt, _) = x.ieee_remainder_and_quotient_bits_prec_ref_ref(&y, prec);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
}

#[test]
fn ieee_remainder_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        ieee_remainder_prec_properties_helper(x, y, prec);
    });

    float_float_unsigned_triple_gen_var_2().test_properties(|(x, y, prec)| {
        ieee_remainder_prec_properties_helper(x, y, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn rem_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (rem, o) = x.clone().rem_round(y.clone(), rm);
    assert!(rem.is_valid());
    let (rem_alt, o_alt) = x.clone().rem_round_val_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.rem_round_ref_val(y.clone(), rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.rem_round_ref_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.rem_round_assign(y.clone(), rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.rem_round_assign_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let prec = max(x.significant_bits(), y.significant_bits());
    let (rem_alt, o_alt) = x.rem_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt, _) = x.rem_and_quotient_bits_round_ref_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
}

#[test]
fn rem_round_properties() {
    float_float_rounding_mode_triple_gen_var_40().test_properties(|(x, y, rm)| {
        rem_round_properties_helper(x, y, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ieee_remainder_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (rem, o) = x.clone().ieee_remainder_round(y.clone(), rm);
    assert!(rem.is_valid());
    let (rem_alt, o_alt) = x.clone().ieee_remainder_round_val_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.ieee_remainder_round_ref_val(y.clone(), rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let (rem_alt, o_alt) = x.ieee_remainder_round_ref_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ieee_remainder_round_assign(y.clone(), rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.ieee_remainder_round_assign_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let prec = max(x.significant_bits(), y.significant_bits());
    let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);

    let (rem_alt, o_alt, _) = x.ieee_remainder_and_quotient_bits_round_ref_ref(&y, rm);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(o_alt, o);
}

#[test]
fn ieee_remainder_round_properties() {
    float_float_rounding_mode_triple_gen_var_41().test_properties(|(x, y, rm)| {
        ieee_remainder_round_properties_helper(x, y, rm);
    });
}

// The expected low 63 bits of the quotient, with the quotient's sign: quo = q mod 2^63, quo*q >= 0.
#[allow(clippy::needless_pass_by_value)]
fn expected_quotient_bits(q: &Integer) -> i64 {
    let low = i64::exact_from(&q.unsigned_abs_ref().mod_power_of_2(63));
    if *q >= 0u32 { low } else { -low }
}

#[allow(clippy::needless_pass_by_value)]
fn rem_properties_helper(x: Float, y: Float, extreme: bool) {
    let rem = &x % &y;
    assert!(rem.is_valid());
    let rem_alt = &x % y.clone();
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    let rem_alt = x.clone() % &y;
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    let rem_alt = x.clone() % y.clone();
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));

    let mut x_alt = x.clone();
    x_alt %= y.clone();
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
    let mut x_alt = x.clone();
    x_alt %= &y;
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));

    let (rem_alt, _) = x.rem_round_ref_ref(&y, Nearest);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));

    let (rem_alt, _, quo) = x.rem_and_quotient_bits_ref_ref(&y);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    let (rem_alt, _, quo_alt) = x.rem_and_quotient_bits_ref_val(y.clone());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(quo_alt, quo);
    let (rem_alt, _, quo_alt) = x.clone().rem_and_quotient_bits_val_ref(&y);
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(quo_alt, quo);
    let (rem_alt, _, quo_alt) = x.clone().rem_and_quotient_bits(y.clone());
    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    assert_eq!(quo_alt, quo);

    let ieee = x.ieee_remainder_ref_ref(&y);
    assert!(ieee.is_valid());
    let ieee_alt = x.ieee_remainder_ref_val(y.clone());
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    let ieee_alt = x.clone().ieee_remainder_val_ref(&y);
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    let ieee_alt = x.clone().ieee_remainder(y.clone());
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    let mut x_alt = x.clone();
    x_alt.ieee_remainder_assign(y.clone());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ieee));
    let mut x_alt = x.clone();
    x_alt.ieee_remainder_assign_ref(&y);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ieee));

    let (ieee_alt, _) = x.ieee_remainder_round_ref_ref(&y, Nearest);
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));

    let (ieee_alt, _, iquo) = x.ieee_remainder_and_quotient_bits_ref_ref(&y);
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    let (ieee_alt, _, iquo_alt) = x.ieee_remainder_and_quotient_bits_ref_val(y.clone());
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    assert_eq!(iquo_alt, iquo);
    let (ieee_alt, _, iquo_alt) = x.clone().ieee_remainder_and_quotient_bits_val_ref(&y);
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    assert_eq!(iquo_alt, iquo);
    let (ieee_alt, _, iquo_alt) = x.clone().ieee_remainder_and_quotient_bits(y.clone());
    assert_eq!(ComparableFloatRef(&ieee_alt), ComparableFloatRef(&ieee));
    assert_eq!(iquo_alt, iquo);

    // the quotient bits are exactly q mod 2^63 with the sign of q
    if !extreme && x.is_finite() && y.is_finite() && x != 0u32 && y != 0u32 {
        let rx = Rational::exact_from(&x);
        let ry = Rational::exact_from(&y);
        let (q, _) = Integer::rounding_from(&rx / &ry, Down);
        assert_eq!(quo, expected_quotient_bits(&q));
        let (q, _) = Integer::rounding_from(&rx / &ry, Nearest);
        assert_eq!(iquo, expected_quotient_bits(&q));
    }

    // quo(-x, y) = -quo(x, y) and quo(x, -y) = -quo(x, y)
    let (_, _, quo_alt) = (-&x).rem_and_quotient_bits_val_ref(&y);
    assert_eq!(quo_alt, quo.wrapping_neg());
    let (_, _, quo_alt) = x.rem_and_quotient_bits_ref_val(-&y);
    assert_eq!(quo_alt, quo.wrapping_neg());
}

#[test]
fn rem_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        rem_properties_helper(x, y, false);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        rem_properties_helper(x, y, true);
    });
}

#[test]
fn rem_unsigned_properties() {
    float_unsigned_pair_gen::<u64>().test_properties(|(x, u)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (rem, o) = x.rem_unsigned_round_ref(u, rm);
            assert!(rem.is_valid());
            let (rem_alt, o_alt) = x.clone().rem_unsigned_round(u, rm);
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            if u == 0 {
                assert!(rem.is_nan());
                assert_eq!(o, Equal);
            } else {
                let (rem_alt, o_alt) =
                    x.rem_prec_round_ref_val(Float::from(u), x.significant_bits(), rm);
                assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
                assert_eq!(o_alt, o);
            }
        }
        for prec in [1u64, 32, 64] {
            for rm in [Floor, Down, Nearest] {
                let (rem, o) = x.rem_unsigned_prec_round_ref(u, prec, rm);
                let (rem_alt, o_alt) = x.clone().rem_unsigned_prec_round(u, prec, rm);
                assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
                assert_eq!(o_alt, o);
                let (mpfr_rem, mpfr_t) = mpfr_fmod_ui_oracle(&x, u, prec, rm);
                assert_eq!(ComparableFloatRef(&mpfr_rem), ComparableFloatRef(&rem));
                assert_eq!(mpfr_t, ternary_sign(o));
                if u == 0 {
                    assert!(rem.is_nan());
                } else {
                    let (rem_alt, o_alt) = x.rem_prec_round_ref_val(Float::from(u), prec, rm);
                    assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
                    assert_eq!(o_alt, o);
                }
            }
            let (rem, o) = x.rem_unsigned_prec_ref(u, prec);
            let (rem_alt, o_alt) = x.clone().rem_unsigned_prec(u, prec);
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let (rem_alt, o_alt) = x.rem_unsigned_prec_round_ref(u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
        }
        let rem = x.rem_unsigned_ref(u);
        let rem_alt = x.clone().rem_unsigned(u);
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        let (rem_alt, _) = x.rem_unsigned_round_ref(u, Nearest);
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
    });
}

// The C99 F.9.7.1 specials, and the sign of a zero remainder.
#[test]
fn test_rem_special_values() {
    let three = Float::from(3u32);
    let six = Float::from(6u32);
    let x = Float::from(1.5f64);
    // any NaN, an infinite x, or a zero y makes the remainder NaN, with quotient bits 0
    for (a, b) in [
        (Float::NAN, Float::NAN),
        (Float::NAN, x.clone()),
        (x.clone(), Float::NAN),
        (Float::INFINITY, x.clone()),
        (Float::NEGATIVE_INFINITY, x.clone()),
        (Float::INFINITY, Float::INFINITY),
        (x.clone(), Float::ZERO),
        (x.clone(), Float::NEGATIVE_ZERO),
        (Float::ZERO, Float::ZERO),
        (Float::INFINITY, Float::ZERO),
    ] {
        let (r, o) = a.rem_prec_round_ref_ref(&b, 10, Nearest);
        assert!(r.is_nan(), "{a} {b}");
        assert_eq!(o, Equal);
        let (r, o, quo) = a.rem_and_quotient_bits_prec_round_ref_ref(&b, 10, Nearest);
        assert!(r.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(quo, 0);
        let (r, _) = a.ieee_remainder_prec_round_ref_ref(&b, 10, Nearest);
        assert!(r.is_nan());
        let (r, _, quo) = a.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&b, 10, Nearest);
        assert!(r.is_nan());
        assert_eq!(quo, 0);
    }
    // an infinite y or a zero x returns x, with quotient bits 0
    for (a, b) in [
        (x.clone(), Float::INFINITY),
        (x.clone(), Float::NEGATIVE_INFINITY),
        (-&x, Float::INFINITY),
        (Float::ZERO, x.clone()),
        (Float::NEGATIVE_ZERO, x.clone()),
        (Float::ZERO, Float::INFINITY),
    ] {
        for f in [
            |a: &Float, b: &Float| {
                let (r, o, quo) = a.rem_and_quotient_bits_prec_round_ref_ref(b, 10, Nearest);
                assert_eq!(quo, 0);
                (r, o)
            },
            |a: &Float, b: &Float| {
                let (r, o, quo) =
                    a.ieee_remainder_and_quotient_bits_prec_round_ref_ref(b, 10, Nearest);
                assert_eq!(quo, 0);
                (r, o)
            },
        ] {
            let (r, o) = f(&a, &b);
            // the result is x, re-rounded to the requested precision
            let (expected, expected_o) = Float::from_float_prec_round_ref(&a, 10, Nearest);
            assert_eq!(ComparableFloat(r), ComparableFloat(expected), "{a} {b}");
            assert_eq!(o, expected_o);
        }
    }
    // a zero remainder takes the sign of x
    for (a, b, sign) in [
        (six.clone(), three.clone(), true),
        (-&six, three.clone(), false),
        (six.clone(), -&three, true),
        (-&six, -&three, false),
    ] {
        let (r, o) = a.rem_prec_round_ref_ref(&b, 10, Nearest);
        assert_eq!(
            ComparableFloat(r),
            ComparableFloat(if sign {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            })
        );
        assert_eq!(o, Equal);
        let (r, o) = a.ieee_remainder_prec_round_ref_ref(&b, 10, Nearest);
        assert_eq!(
            ComparableFloat(r),
            ComparableFloat(if sign {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            })
        );
        assert_eq!(o, Equal);
    }
}

// The quotient-bits increment corner: when the low 63 bits are all ones and the nearest quotient
// rounds away, the C code overflows a long (undefined behavior); we wrap modulo 2^63 per the
// documented contract. x = 2^63 - 1/2, y = 1: the truncated quotient is 2^63 - 1 (bits all ones),
// and the nearest quotient is 2^63 (bits all zeros).
#[test]
fn test_quotient_bits_wrap_corner() {
    let x = Float::from_natural_prec(Natural::low_mask(64), 64).0 >> 1u32;
    let y = Float::ONE;
    let (r, o, quo) = x.rem_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
    assert_eq!(quo, i64::MAX);
    assert_eq!(o, Equal);
    assert_eq!(Rational::exact_from(&r), Rational::from_signeds(1, 2));
    let (r, o, quo) = x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
    assert_eq!(quo, 0);
    assert_eq!(o, Equal);
    assert_eq!(Rational::exact_from(&r), Rational::from_signeds(-1, 2));
    // and the negated variants, exercising the sign flip on the wrapped value
    let (_, _, quo) = (-&x).rem_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
    assert_eq!(quo, -i64::MAX);
    let (_, _, quo) = (-&x).ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
    assert_eq!(quo, 0);
}

// Directed underflow: the remainder's granularity can lie far below the minimum positive Float even
// though neither input is extreme in magnitude, so the final rounding can underflow. Here x =
// 3*2^(MIN_EXPONENT-2) and y = 2^(MIN_EXPONENT-1), so the exact remainder is 2^(MIN_EXPONENT-2),
// halfway between 0 and the minimum positive value 2^(MIN_EXPONENT-1).
#[test]
fn test_rem_underflow() {
    let min_exp = i64::from(Float::MIN_EXPONENT);
    let x = Float::from_rational_prec(Rational::from_unsigneds(3u32, 2u32) << (min_exp - 1), 2).0;
    let y = Float::power_of_2_prec(min_exp - 1, 1).0;
    // Floor/Down: to zero; Up/Ceiling: to the minimum positive value; Nearest: tie to even = zero
    for (rm, expected_zero, o) in [
        (Floor, true, Less),
        (Down, true, Less),
        (Nearest, true, Less),
        (Up, false, Greater),
        (Ceiling, false, Greater),
    ] {
        let (r, oo) = x.rem_prec_round_ref_ref(&y, 1, rm);
        if expected_zero {
            assert_eq!(ComparableFloat(r), ComparableFloat(Float::ZERO), "{rm}");
        } else {
            assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&y), "{rm}");
        }
        assert_eq!(oo, o, "{rm}");
    }
}

// The oracle comparisons woven through this file's tests and property suites were observed (via
// temporary first-hit marks) to cover every branch of rem1_helper:
// - the NaN and copy special arms (test_rem_special_values)
// - ex <= ey with a zero quotient (tiny) and with a real division, each with and without the
//   quotient-bits request
// - ex > ey under all three moduli (Y << 63 for quotient bits, 2Y for plain nearest, Y for fmod)
// - both sides of the modular-exponentiation threshold d > 3 * my.significant_bits()
// - the nearest low-quotient-bit subtraction taken and not taken
// - a zero remainder of each sign
// - in the nearest assembly: the tiny size short-circuit, the tiny shifted comparison, and the
//   ordinary comparison; the round-away branch taken (including on an exact tie) and not taken; the
//   quotient-bits increment, including the all-ones wrap (test_quotient_bits_wrap_corner)
// - the negative-x remainder negation
#[test]
fn test_rem() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let rem = x.clone() % y.clone();
        assert!(rem.is_valid());
        assert_eq!(rem.to_string(), out);
        assert_eq!(to_hex_string(&rem), out_hex);

        let rem_alt = x.clone() % &y;
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        let rem_alt = &x % y.clone();
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        let rem_alt = &x % &y;
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));

        let mut x_alt = x.clone();
        x_alt %= y.clone();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
        let mut x_alt = x.clone();
        x_alt %= &y;
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_rem(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&rem)
        );
    };
    // - NaN, infinite x, or zero y: NaN
    test("NaN", "NaN", "3.0", "0x3.0#2", "NaN", "NaN");
    test("3.0", "0x3.0#2", "NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "3.0", "0x3.0#2", "NaN", "NaN");
    test("-Infinity", "-Infinity", "3.0", "0x3.0#2", "NaN", "NaN");
    test("3.0", "0x3.0#2", "0.0", "0x0.0", "NaN", "NaN");
    test("3.0", "0x3.0#2", "-0.0", "-0x0.0", "NaN", "NaN");
    // - infinite y or zero x: x
    test("3.0", "0x3.0#2", "Infinity", "Infinity", "3.0", "0x3.0#2");
    test(
        "-3.0",
        "-0x3.0#2",
        "-Infinity",
        "-Infinity",
        "-3.0",
        "-0x3.0#2",
    );
    test("0.0", "0x0.0", "3.0", "0x3.0#2", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "3.0", "0x3.0#2", "-0.0", "-0x0.0");
    // - a zero remainder takes the sign of x
    test("6.0", "0x6.0#2", "3.0", "0x3.0#2", "0.0", "0x0.0");
    test("-6.0", "-0x6.0#2", "3.0", "0x3.0#2", "-0.0", "-0x0.0");
    test("6.0", "0x6.0#2", "-3.0", "-0x3.0#2", "0.0", "0x0.0");
    test("-6.0", "-0x6.0#2", "-3.0", "-0x3.0#2", "-0.0", "-0x0.0");
    // - a nonzero remainder takes the sign of x, regardless of y's sign
    test("10.0", "0xa.0#3", "3.0", "0x3.0#2", "1.0", "0x1.0#3");
    test("-10.0", "-0xa.0#3", "3.0", "0x3.0#2", "-1.0", "-0x1.0#3");
    test("10.0", "0xa.0#3", "-3.0", "-0x3.0#2", "1.0", "0x1.0#3");
    test("-10.0", "-0xa.0#3", "-3.0", "-0x3.0#2", "-1.0", "-0x1.0#3");
    test("14.0", "0xe.0#3", "3.0", "0x3.0#2", "2.0", "0x2.0#3");
    // - ex <= ey with a real division
    test("3.0", "0x3.0#2", "2.0", "0x2.0#1", "1.0", "0x1.0#2");
    // - ex <= ey with a zero quotient (tiny)
    test("1.0", "0x1.0#1", "8.0", "0x8.0#1", "1.0", "0x1.0#1");
    test("3.0", "0x3.0#2", "7.0", "0x7.0#3", "3.0", "0x3.0#3");
    test("3.0", "0x3.0#2", "4.0", "0x4.0#1", "3.0", "0x3.0#2");
    // - ex > ey, exact power of 2
    test("100.0", "0x64.0#5", "7.0", "0x7.0#3", "2.00", "0x2.0#5");
    test("10.0", "0xa.0#3", "7.0", "0x7.0#3", "3.0", "0x3.0#3");
    // - ex > ey, modular exponentiation (d > 3 * my.significant_bits())
    test("1.6e60", "0x1.0E+50#1", "3.0", "0x3.0#2", "1.0", "0x1.0#2");
    // - fractional operands
    test("10.5", "0xa.8#5", "3.25", "0x3.4#4", "0.750", "0x0.c0#5");
    test(
        "-10.5",
        "-0xa.8#5",
        "3.25",
        "0x3.4#4",
        "-0.750",
        "-0x0.c0#5",
    );
    // - the all-ones-quotient values (see test_quotient_bits_wrap_corner)
    test(
        "9223372036854775807.50",
        "0x7fffffffffffffff.8#64",
        "1.0",
        "0x1.0#1",
        "0.500000000000000000000",
        "0x0.8000000000000000#64",
    );
    // - remainder underflow (granularity below the minimum positive Float; Nearest ties to zero)
    test(
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "0.0",
        "0x0.0",
    );
}

#[test]
fn test_rem_prec() {
    let test = |s, s_hex, t, t_hex, prec, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (rem, o) = x.clone().rem_prec(y.clone(), prec);
        assert!(rem.is_valid());
        assert_eq!(rem.to_string(), out);
        assert_eq!(to_hex_string(&rem), out_hex);
        assert_eq!(o, o_out);

        let (rem_alt, o_alt) = x.clone().rem_prec_val_ref(&y, prec);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        let (rem_alt, o_alt) = x.rem_prec_ref_val(y.clone(), prec);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        let (rem_alt, o_alt) = x.rem_prec_ref_ref(&y, prec);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.rem_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.rem_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);

        let (rug_rem, rug_o) = rug_rem_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_rem)),
            ComparableFloatRef(&rem)
        );
        assert_eq!(rug_o, o);
    };
    // - the exact remainder 3 rounds at precision 1, to nearest-even 4
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, "4.0", "0x4.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 2, "3.0", "0x3.0#2", Equal,
    );
    test(
        "-10.0", "-0xa.0#3", "7.0", "0x7.0#3", 1, "-4.0", "-0x4.0#1", Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        "7.0",
        "0x7.0#3",
        10,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "3.0", "0x3.0#2", "2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Equal,
    );
}

#[test]
fn test_rem_prec_round() {
    let test =
        |s, s_hex, t, t_hex, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let y = parse_hex_string(t_hex);
            assert_eq!(y.to_string(), t);

            let (rem, o) = x.clone().rem_prec_round(y.clone(), prec, rm);
            assert!(rem.is_valid());
            assert_eq!(rem.to_string(), out);
            assert_eq!(to_hex_string(&rem), out_hex);
            assert_eq!(o, o_out);

            let (rem_alt, o_alt) = x.clone().rem_prec_round_val_ref(&y, prec, rm);
            assert!(rem_alt.is_valid());
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let (rem_alt, o_alt) = x.rem_prec_round_ref_val(y.clone(), prec, rm);
            assert!(rem_alt.is_valid());
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let (rem_alt, o_alt) = x.rem_prec_round_ref_ref(&y, prec, rm);
            assert!(rem_alt.is_valid());
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.rem_prec_round_assign(y.clone(), prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.rem_prec_round_assign_ref(&y, prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_rem, rug_o) = rug_rem_prec_round(
                    &rug::Float::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_rem)),
                    ComparableFloatRef(&rem)
                );
                assert_eq!(rug_o, o);
            }
        };
    // - NaN and copy special arms
    test(
        "NaN", "NaN", "3.0", "0x3.0#2", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "3.0", "0x3.0#2", "0.0", "0x0.0", 10, Nearest, "NaN", "NaN", Equal,
    );
    // - y infinite: x, re-rounded to the requested precision
    test(
        "3.0",
        "0x3.0#2",
        "Infinity",
        "Infinity",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "0.0", "0x0.0", "3.0", "0x3.0#2", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    // - a zero remainder takes the sign of x
    test(
        "6.0", "0x6.0#2", "3.0", "0x3.0#2", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-6.0", "-0x6.0#2", "3.0", "0x3.0#2", 10, Nearest, "-0.0", "-0x0.0", Equal,
    );
    // - ex <= ey with a zero quotient (tiny)
    test(
        "1.0",
        "0x1.0#1",
        "8.0",
        "0x8.0#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "7.0",
        "0x7.0#3",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    // - ex > ey, exact power of 2
    test(
        "100.0",
        "0x64.0#5",
        "7.0",
        "0x7.0#3",
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    // - ex > ey, modular exponentiation
    test(
        "1.6e60",
        "0x1.0E+50#1",
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    // - the exact remainder 3 under every rounding mode at precision 1
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Down, "2.0", "0x2.0#1", Less,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Up, "4.0", "0x4.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );
    // - rounding a negative remainder: Floor moves away from zero, Ceiling toward it
    test(
        "-10.0", "-0xa.0#3", "7.0", "0x7.0#3", 1, Floor, "-4.0", "-0x4.0#1", Less,
    );
    test(
        "-10.0", "-0xa.0#3", "7.0", "0x7.0#3", 1, Ceiling, "-2.0", "-0x2.0#1", Greater,
    );
    // - Exact is allowed when the remainder is exactly representable
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 2, Exact, "3.0", "0x3.0#2", Equal,
    );
    // - fractional operands
    test(
        "10.5", "0xa.8#5", "3.25", "0x3.4#4", 4, Nearest, "0.750", "0x0.c#4", Equal,
    );
    // - remainder underflow: Floor to zero, Up to the minimum positive Float
    test(
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
}

#[test]
fn test_ieee_remainder() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let rem = x.clone().ieee_remainder(y.clone());
        assert!(rem.is_valid());
        assert_eq!(rem.to_string(), out);
        assert_eq!(to_hex_string(&rem), out_hex);

        let rem_alt = x.clone().ieee_remainder_val_ref(&y);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        let rem_alt = x.ieee_remainder_ref_val(y.clone());
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        let rem_alt = x.ieee_remainder_ref_ref(&y);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));

        let mut x_alt = x.clone();
        x_alt.ieee_remainder_assign(y.clone());
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
        let mut x_alt = x.clone();
        x_alt.ieee_remainder_assign_ref(&y);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ieee_remainder(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&rem)
        );
    };
    // - NaN, infinite x, or zero y: NaN
    test("NaN", "NaN", "3.0", "0x3.0#2", "NaN", "NaN");
    test("3.0", "0x3.0#2", "NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "3.0", "0x3.0#2", "NaN", "NaN");
    test("3.0", "0x3.0#2", "0.0", "0x0.0", "NaN", "NaN");
    // - infinite y or zero x: x
    test("3.0", "0x3.0#2", "Infinity", "Infinity", "3.0", "0x3.0#2");
    test("0.0", "0x0.0", "3.0", "0x3.0#2", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "3.0", "0x3.0#2", "-0.0", "-0x0.0");
    // - a zero remainder takes the sign of x
    test("6.0", "0x6.0#2", "3.0", "0x3.0#2", "0.0", "0x0.0");
    test("-6.0", "-0x6.0#2", "3.0", "0x3.0#2", "-0.0", "-0x0.0");
    // - the nearest quotient leaves a remainder of either sign
    test("10.0", "0xa.0#3", "3.0", "0x3.0#2", "1.0", "0x1.0#3");
    test("14.0", "0xe.0#3", "3.0", "0x3.0#2", "-1.0", "-0x1.0#3");
    test("-10.0", "-0xa.0#3", "3.0", "0x3.0#2", "-1.0", "-0x1.0#3");
    // - a tie (x/y halfway between integers) rounds the quotient to even
    test("3.0", "0x3.0#2", "2.0", "0x2.0#1", "-1.0", "-0x1.0#2");
    // - tiny with the size short-circuit (|x| < |y|/2)
    test("1.0", "0x1.0#1", "8.0", "0x8.0#1", "1.0", "0x1.0#1");
    // - tiny with a full comparison, quotient stays 0
    test("3.0", "0x3.0#2", "7.0", "0x7.0#3", "3.0", "0x3.0#3");
    // - tiny with a full comparison, quotient rounds to 1
    test("3.0", "0x3.0#2", "4.0", "0x4.0#1", "-1.0", "-0x1.0#2");
    // - ex > ey with the low quotient bit subtracted and not
    test("100.0", "0x64.0#5", "7.0", "0x7.0#3", "2.00", "0x2.0#5");
    test("10.0", "0xa.0#3", "7.0", "0x7.0#3", "3.0", "0x3.0#3");
    // - ex > ey, modular exponentiation
    test("1.6e60", "0x1.0E+50#1", "3.0", "0x3.0#2", "1.0", "0x1.0#2");
    // - fractional operands
    test("10.5", "0xa.8#5", "3.25", "0x3.4#4", "0.750", "0x0.c0#5");
    // - the wrap corner's remainder (see test_quotient_bits_wrap_corner)
    test(
        "9223372036854775807.50",
        "0x7fffffffffffffff.8#64",
        "1.0",
        "0x1.0#1",
        "-0.500000000000000000000",
        "-0x0.8000000000000000#64",
    );
    // - remainder underflow with a negative exact value (the quotient rounds up)
    test(
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "-0.0",
        "-0x0.0",
    );
}

#[test]
fn test_ieee_remainder_prec_round() {
    let test =
        |s, s_hex, t, t_hex, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let y = parse_hex_string(t_hex);
            assert_eq!(y.to_string(), t);

            let (rem, o) = x.clone().ieee_remainder_prec_round(y.clone(), prec, rm);
            assert!(rem.is_valid());
            assert_eq!(rem.to_string(), out);
            assert_eq!(to_hex_string(&rem), out_hex);
            assert_eq!(o, o_out);

            let (rem_alt, o_alt) = x.clone().ieee_remainder_prec_round_val_ref(&y, prec, rm);
            assert!(rem_alt.is_valid());
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_val(y.clone(), prec, rm);
            assert!(rem_alt.is_valid());
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let (rem_alt, o_alt) = x.ieee_remainder_prec_round_ref_ref(&y, prec, rm);
            assert!(rem_alt.is_valid());
            assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.ieee_remainder_prec_round_assign(y.clone(), prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.ieee_remainder_prec_round_assign_ref(&y, prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&rem));
            assert_eq!(o_alt, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_rem, rug_o) = rug_ieee_remainder_prec_round(
                    &rug::Float::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_rem)),
                    ComparableFloatRef(&rem)
                );
                assert_eq!(rug_o, o);
            }
        };
    // - specials
    test(
        "NaN", "NaN", "3.0", "0x3.0#2", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "Infinity",
        "Infinity",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    // - a zero remainder
    test(
        "6.0", "0x6.0#2", "3.0", "0x3.0#2", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    // - remainders of both signs
    test(
        "10.0",
        "0xa.0#3",
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "14.0",
        "0xe.0#3",
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    // - ties round the quotient to even (from either side)
    test(
        "3.0",
        "0x3.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    // - the tiny paths
    test(
        "1.0",
        "0x1.0#1",
        "8.0",
        "0x8.0#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "7.0",
        "0x7.0#3",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    // - ex > ey paths
    test(
        "100.0",
        "0x64.0#5",
        "7.0",
        "0x7.0#3",
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.6e60",
        "0x1.0E+50#1",
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    // - rounding the exact remainder 3 at precision 1
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );
    test(
        "-10.0", "-0xa.0#3", "7.0", "0x7.0#3", 1, Floor, "-4.0", "-0x4.0#1", Less,
    );
    // - Exact is allowed when the remainder is exactly representable
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 2, Exact, "3.0", "0x3.0#2", Equal,
    );
}

#[test]
fn test_rem_and_quotient_bits() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str, o_out: Ordering, quo_out: i64| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (rem, o, quo) = x.clone().rem_and_quotient_bits(y.clone());
        assert!(rem.is_valid());
        assert_eq!(rem.to_string(), out);
        assert_eq!(to_hex_string(&rem), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(quo, quo_out);

        let (rem_alt, o_alt, quo_alt) = x.clone().rem_and_quotient_bits_val_ref(&y);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        assert_eq!(quo_alt, quo);
        let (rem_alt, o_alt, quo_alt) = x.rem_and_quotient_bits_ref_val(y.clone());
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        assert_eq!(quo_alt, quo);
        let (rem_alt, o_alt, quo_alt) = x.rem_and_quotient_bits_ref_ref(&y);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        assert_eq!(quo_alt, quo);

        let prec = max(x.significant_bits(), y.significant_bits());
        let (mpfr_rem, mpfr_t, mpfr_quo) = mpfr_fmodquo_oracle(&x, &y, prec, Nearest);
        assert_eq!(ComparableFloatRef(&mpfr_rem), ComparableFloatRef(&rem));
        assert_eq!(mpfr_t, ternary_sign(o));
        check_quo_vs_mpfr(quo, mpfr_quo);
    };
    // - unspecified in MPFR; our quotient bits are 0 for every special case
    test("NaN", "NaN", "3.0", "0x3.0#2", "NaN", "NaN", Equal, 0);
    test(
        "Infinity", "Infinity", "3.0", "0x3.0#2", "NaN", "NaN", Equal, 0,
    );
    test("3.0", "0x3.0#2", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test(
        "3.0", "0x3.0#2", "Infinity", "Infinity", "3.0", "0x3.0#2", Equal, 0,
    );
    test("0.0", "0x0.0", "3.0", "0x3.0#2", "0.0", "0x0.0", Equal, 0);
    // - the quotient bits have the sign of x/y, even when the remainder is zero
    test("6.0", "0x6.0#2", "3.0", "0x3.0#2", "0.0", "0x0.0", Equal, 2);
    test(
        "-6.0", "-0x6.0#2", "3.0", "0x3.0#2", "-0.0", "-0x0.0", Equal, -2,
    );
    test(
        "6.0", "0x6.0#2", "-3.0", "-0x3.0#2", "0.0", "0x0.0", Equal, -2,
    );
    test(
        "-6.0", "-0x6.0#2", "-3.0", "-0x3.0#2", "-0.0", "-0x0.0", Equal, 2,
    );
    // - the truncated quotient, all four sign combinations
    test(
        "10.0", "0xa.0#3", "3.0", "0x3.0#2", "1.0", "0x1.0#3", Equal, 3,
    );
    test(
        "-10.0", "-0xa.0#3", "3.0", "0x3.0#2", "-1.0", "-0x1.0#3", Equal, -3,
    );
    test(
        "10.0", "0xa.0#3", "-3.0", "-0x3.0#2", "1.0", "0x1.0#3", Equal, -3,
    );
    test(
        "-10.0", "-0xa.0#3", "-3.0", "-0x3.0#2", "-1.0", "-0x1.0#3", Equal, 3,
    );
    // - a zero quotient in the tiny cases
    test(
        "1.0", "0x1.0#1", "8.0", "0x8.0#1", "1.0", "0x1.0#1", Equal, 0,
    );
    test(
        "3.0", "0x3.0#2", "7.0", "0x7.0#3", "3.0", "0x3.0#3", Equal, 0,
    );
    // - ex > ey: the low 63 bits recovered via the shifted modulus
    test(
        "100.0", "0x64.0#5", "7.0", "0x7.0#3", "2.00", "0x2.0#5", Equal, 14,
    );
    // - the low 63 bits of a quotient too large for an i64 (2^200/3 in binary is 01 repeating)
    test(
        "1.6e60",
        "0x1.0E+50#1",
        "3.0",
        "0x3.0#2",
        "1.0",
        "0x1.0#2",
        Equal,
        6148914691236517205,
    );
    // - the all-ones quotient (no increment for the truncated quotient)
    test(
        "9223372036854775807.50",
        "0x7fffffffffffffff.8#64",
        "1.0",
        "0x1.0#1",
        "0.500000000000000000000",
        "0x0.8000000000000000#64",
        Equal,
        9223372036854775807,
    );
    test(
        "-9223372036854775807.50",
        "-0x7fffffffffffffff.8#64",
        "1.0",
        "0x1.0#1",
        "-0.500000000000000000000",
        "-0x0.8000000000000000#64",
        Equal,
        -9223372036854775807,
    );
}

#[test]
fn test_ieee_remainder_and_quotient_bits() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str, o_out: Ordering, quo_out: i64| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (rem, o, quo) = x.clone().ieee_remainder_and_quotient_bits(y.clone());
        assert!(rem.is_valid());
        assert_eq!(rem.to_string(), out);
        assert_eq!(to_hex_string(&rem), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(quo, quo_out);

        let (rem_alt, o_alt, quo_alt) = x.clone().ieee_remainder_and_quotient_bits_val_ref(&y);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        assert_eq!(quo_alt, quo);
        let (rem_alt, o_alt, quo_alt) = x.ieee_remainder_and_quotient_bits_ref_val(y.clone());
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        assert_eq!(quo_alt, quo);
        let (rem_alt, o_alt, quo_alt) = x.ieee_remainder_and_quotient_bits_ref_ref(&y);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));
        assert_eq!(o_alt, o);
        assert_eq!(quo_alt, quo);

        let prec = max(x.significant_bits(), y.significant_bits());
        let (mpfr_rem, mpfr_t, mpfr_quo) = mpfr_remquo_oracle(&x, &y, prec, Nearest);
        assert_eq!(ComparableFloatRef(&mpfr_rem), ComparableFloatRef(&rem));
        assert_eq!(mpfr_t, ternary_sign(o));
        check_quo_vs_mpfr(quo, mpfr_quo);
    };
    // - specials
    test("NaN", "NaN", "3.0", "0x3.0#2", "NaN", "NaN", Equal, 0);
    test(
        "3.0", "0x3.0#2", "Infinity", "Infinity", "3.0", "0x3.0#2", Equal, 0,
    );
    // - the nearest quotient, remainders of both signs
    test(
        "10.0", "0xa.0#3", "3.0", "0x3.0#2", "1.0", "0x1.0#3", Equal, 3,
    );
    test(
        "14.0", "0xe.0#3", "3.0", "0x3.0#2", "-1.0", "-0x1.0#3", Equal, 5,
    );
    test(
        "-10.0", "-0xa.0#3", "3.0", "0x3.0#2", "-1.0", "-0x1.0#3", Equal, -3,
    );
    // - a tie rounds the quotient to even (2, not 1)
    test(
        "3.0", "0x3.0#2", "2.0", "0x2.0#1", "-1.0", "-0x1.0#2", Equal, 2,
    );
    // - tiny: the quotient rounds to 0 or 1
    test(
        "1.0", "0x1.0#1", "8.0", "0x8.0#1", "1.0", "0x1.0#1", Equal, 0,
    );
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", "-1.0", "-0x1.0#2", Equal, 1,
    );
    // - ex > ey
    test(
        "100.0", "0x64.0#5", "7.0", "0x7.0#3", "2.00", "0x2.0#5", Equal, 14,
    );
    test(
        "1.6e60",
        "0x1.0E+50#1",
        "3.0",
        "0x3.0#2",
        "1.0",
        "0x1.0#2",
        Equal,
        6148914691236517205,
    );
    // - the all-ones wrap: the incremented quotient's low 63 bits are 0 (the C code overflows a
    //   long here)
    test(
        "9223372036854775807.50",
        "0x7fffffffffffffff.8#64",
        "1.0",
        "0x1.0#1",
        "-0.500000000000000000000",
        "-0x0.8000000000000000#64",
        Equal,
        0,
    );
    test(
        "-9223372036854775807.50",
        "-0x7fffffffffffffff.8#64",
        "1.0",
        "0x1.0#1",
        "0.500000000000000000000",
        "0x0.8000000000000000#64",
        Equal,
        0,
    );
    // - underflow with the quotient rounded up: quotient bits 2, remainder negative zero
    test(
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "-0.0",
        "-0x0.0",
        Greater,
        2,
    );
}

#[test]
fn test_rem_unsigned() {
    let test = |s, s_hex, u: u64, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let rem = x.clone().rem_unsigned(u);
        assert!(rem.is_valid());
        assert_eq!(rem.to_string(), out);
        assert_eq!(to_hex_string(&rem), out_hex);

        let rem_alt = x.rem_unsigned_ref(u);
        assert!(rem_alt.is_valid());
        assert_eq!(ComparableFloatRef(&rem_alt), ComparableFloatRef(&rem));

        let (mpfr_rem, _) = mpfr_fmod_ui_oracle(&x, u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&mpfr_rem), ComparableFloatRef(&rem));
    };
    // - a fractional remainder, both signs
    test("10.5", "0xa.8#5", 3, "1.50", "0x1.8#5");
    test("-10.5", "-0xa.8#5", 3, "-1.50", "-0x1.8#5");
    // - a zero modulus is NaN, matching mpfr_fmod_ui
    test("10.0", "0xa.0#3", 0, "NaN", "NaN");
    // - specials
    test("NaN", "NaN", 3, "NaN", "NaN");
    test("Infinity", "Infinity", 3, "NaN", "NaN");
    test("0.0", "0x0.0", 3, "0.0", "0x0.0");
    // - |x| smaller than the modulus
    test("1.5", "0x1.8#2", 7, "1.5", "0x1.8#2");
    // - a large modulus: 2^100 mod (2^64 - 1) = 2^36
    test(
        "1.3e30",
        "0x1.0E+25#1",
        18446744073709551615,
        "6.9e10",
        "0x1.0E+9#1",
    );
    // - an exact multiple
    test("100.0", "0x64.0#5", 1, "0.0", "0x0.0");
}

#[test]
fn rem_prec_round_fail() {
    assert_panic!(Float::from(1u32).rem_prec_round(Float::from(3u32), 0, Nearest));
    assert_panic!(Float::from(1u32).rem_prec_round_val_ref(&Float::from(3u32), 0, Nearest));
    assert_panic!(Float::from(1u32).rem_prec_round_ref_val(Float::from(3u32), 0, Nearest));
    assert_panic!(Float::from(1u32).rem_prec_round_ref_ref(&Float::from(3u32), 0, Nearest));
    // Exact with an inexact remainder
    assert_panic!(Float::from(10u32).rem_prec_round(Float::from(7u32), 1, Exact));
}

#[test]
fn ieee_remainder_prec_round_fail() {
    assert_panic!(Float::from(1u32).ieee_remainder_prec_round(Float::from(3u32), 0, Nearest));
    assert_panic!(Float::from(10u32).ieee_remainder_prec_round(Float::from(7u32), 1, Exact));
}

#[test]
fn rem_and_quotient_bits_prec_round_fail() {
    assert_panic!(Float::from(1u32).rem_and_quotient_bits_prec_round(
        Float::from(3u32),
        0,
        Nearest
    ));
    assert_panic!(
        Float::from(1u32).ieee_remainder_and_quotient_bits_prec_round(
            Float::from(3u32),
            0,
            Nearest
        )
    );
}

#[test]
fn rem_unsigned_prec_round_fail() {
    assert_panic!(Float::from(1u32).rem_unsigned_prec_round(3, 0, Nearest));
    assert_panic!(Float::from(1u32).rem_unsigned_prec_round_ref(3, 0, Nearest));
}

fn test_rationals() -> Vec<Rational> {
    let mut ys = vec![
        Rational::from_signeds(1, 3),
        Rational::from_signeds(-1, 3),
        Rational::from_signeds(3, 7),
        Rational::from_signeds(10, 3),
        Rational::from_signeds(7, 1),
        Rational::from_signeds(3, 8),
        Rational::from_signeds(-3, 8),
        Rational::from_signeds(355, 113),
        Rational::from_signeds(1, 3) >> 50u32,
        Rational::from_signeds(1, 3) << 100u32,
    ];
    ys.extend(ys.clone().into_iter().map(|q| -q));
    ys.sort_unstable();
    ys.dedup();
    ys
}

// All four mixed Float-Rational remainder directions versus the exact Rational identity: q = x/y
// rounded to an integer (toward zero or to nearest-even), r = x - qy computed exactly, then rounded
// once.
#[test]
fn test_rem_rational_vs_exact() {
    let rationals = test_rationals();
    for x in sweep_values() {
        if !x.is_finite() || x == 0u32 {
            continue;
        }
        let xr = Rational::exact_from(&x);
        for y in &rationals {
            for prec in [1u64, 10, 64] {
                for rm in [Floor, Down, Nearest] {
                    for nearest_quotient in [false, true] {
                        let (r, o, quo) = if nearest_quotient {
                            x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
                                y, prec, rm,
                            )
                        } else {
                            x.rem_rational_and_quotient_bits_prec_round_ref_ref(y, prec, rm)
                        };
                        let (rr, oo) = if nearest_quotient {
                            x.ieee_remainder_rational_prec_round_ref_ref(y, prec, rm)
                        } else {
                            x.rem_rational_prec_round_ref_ref(y, prec, rm)
                        };
                        assert_eq!(ComparableFloatRef(&rr), ComparableFloatRef(&r));
                        assert_eq!(oo, o);
                        let (q, _) = Integer::rounding_from(
                            &xr / y,
                            if nearest_quotient { Nearest } else { Down },
                        );
                        assert_eq!(quo, expected_quotient_bits(&q), "{x} {y} {prec} {rm}");
                        let r_exact = &xr - Rational::from(q) * y;
                        if r_exact == 0u32 {
                            let expected = if x > 0u32 {
                                Float::ZERO
                            } else {
                                Float::NEGATIVE_ZERO
                            };
                            assert_eq!(
                                ComparableFloat(r),
                                ComparableFloat(expected),
                                "{x} {y} {prec} {rm}"
                            );
                            assert_eq!(o, Equal);
                        } else {
                            let (expected, expected_o) =
                                Float::from_rational_prec_round(r_exact, prec, rm);
                            assert_eq!(
                                ComparableFloat(r),
                                ComparableFloat(expected),
                                "{x} {y} {prec} {rm} {nearest_quotient}"
                            );
                            assert_eq!(o, expected_o);
                        }
                    }
                }
            }
        }
    }
}

// The reversed direction versus the same identity.
#[test]
fn test_rational_rem_float_vs_exact() {
    type MixedQuoFn = fn(&Rational, &Float, u64, RoundingMode) -> (Float, Ordering, i64);
    let quo_fns: [MixedQuoFn; 2] = [
        Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref,
        Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref,
    ];
    let rationals = test_rationals();
    for y in sweep_values() {
        if !y.is_finite() || y == 0u32 {
            continue;
        }
        let yr = Rational::exact_from(&y);
        for x in &rationals {
            for prec in [1u64, 10, 64] {
                for rm in [Floor, Down, Nearest] {
                    for nearest_quotient in [false, true] {
                        let (r, o, quo) = quo_fns[usize::from(nearest_quotient)](x, &y, prec, rm);
                        let (rr, oo) = if nearest_quotient {
                            Float::rational_ieee_remainder_float_prec_round_ref_ref(x, &y, prec, rm)
                        } else {
                            Float::rational_rem_float_prec_round_ref_ref(x, &y, prec, rm)
                        };
                        assert_eq!(ComparableFloatRef(&rr), ComparableFloatRef(&r));
                        assert_eq!(oo, o);
                        let (q, _) = Integer::rounding_from(
                            x / &yr,
                            if nearest_quotient { Nearest } else { Down },
                        );
                        assert_eq!(quo, expected_quotient_bits(&q), "{x} {y} {prec} {rm}");
                        let r_exact = x - Rational::from(q) * &yr;
                        if r_exact == 0u32 {
                            let expected = if *x > 0u32 {
                                Float::ZERO
                            } else {
                                Float::NEGATIVE_ZERO
                            };
                            assert_eq!(
                                ComparableFloat(r),
                                ComparableFloat(expected),
                                "{x} {y} {prec} {rm}"
                            );
                            assert_eq!(o, Equal);
                        } else {
                            let (expected, expected_o) =
                                Float::from_rational_prec_round(r_exact, prec, rm);
                            assert_eq!(
                                ComparableFloat(r),
                                ComparableFloat(expected),
                                "{x} {y} {prec} {rm} {nearest_quotient}"
                            );
                            assert_eq!(o, expected_o);
                        }
                    }
                }
            }
        }
    }
}

// With a dyadic Rational modulus, the mixed functions agree with the Float-Float functions on the
// exactly-converted modulus.
#[test]
fn test_rem_rational_dyadic_consistency() {
    for x in sweep_values() {
        for y in [
            Rational::from_signeds(3, 8),
            Rational::from_signeds(-3, 8),
            Rational::from_signeds(7, 1),
            Rational::from_signeds(1, 4) << 60u32,
        ] {
            let yf = Float::from_rational_prec_round_ref(&y, 3, Floor).0;
            assert_eq!(Rational::exact_from(&yf), y);
            for prec in [1u64, 10] {
                for rm in [Floor, Nearest] {
                    let (r, o) = x.rem_rational_prec_round_ref_ref(&y, prec, rm);
                    let (expected, expected_o) = x.rem_prec_round_ref_ref(&yf, prec, rm);
                    assert_eq!(ComparableFloat(r), ComparableFloat(expected), "{x} {y}");
                    assert_eq!(o, expected_o);
                    let (r, o) = x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm);
                    let (expected, expected_o) = x.ieee_remainder_prec_round_ref_ref(&yf, prec, rm);
                    assert_eq!(ComparableFloat(r), ComparableFloat(expected), "{x} {y}");
                    assert_eq!(o, expected_o);
                }
            }
        }
    }
}

// Special values for the mixed remainder functions.
#[test]
fn test_rem_rational_special_values() {
    let third = Rational::from_signeds(1, 3);
    // NaN or infinite Float dividend, or zero Rational modulus: NaN
    for (x, y) in [
        (Float::NAN, third.clone()),
        (Float::INFINITY, third.clone()),
        (Float::NEGATIVE_INFINITY, third.clone()),
        (Float::from(3u32), Rational::ZERO),
        (Float::NAN, Rational::ZERO),
        (Float::ZERO, Rational::ZERO),
    ] {
        let (r, o, quo) = x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
        assert!(r.is_nan(), "{x} {y}");
        assert_eq!(o, Equal);
        assert_eq!(quo, 0);
        let (r, _, quo) =
            x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
        assert!(r.is_nan());
        assert_eq!(quo, 0);
    }
    // zero Float dividend: x, with its sign
    for x in [Float::ZERO, Float::NEGATIVE_ZERO] {
        let (r, o, quo) = x.rem_rational_and_quotient_bits_prec_round_ref_ref(&third, 10, Nearest);
        assert_eq!(ComparableFloat(r), ComparableFloat(x.clone()));
        assert_eq!(o, Equal);
        assert_eq!(quo, 0);
    }
    // reversed direction: NaN or zero Float modulus is NaN; an infinite modulus returns the
    // Rational rounded; a zero Rational dividend is a positive zero
    for y in [Float::NAN, Float::ZERO, Float::NEGATIVE_ZERO] {
        let (r, o, quo) =
            Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(&third, &y, 10, Nearest);
        assert!(r.is_nan(), "{y}");
        assert_eq!(o, Equal);
        assert_eq!(quo, 0);
    }
    let (r, o, quo) = Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(
        &third,
        &Float::INFINITY,
        10,
        Nearest,
    );
    let (expected, expected_o) = Float::from_rational_prec_round_ref(&third, 10, Nearest);
    assert_eq!(ComparableFloat(r), ComparableFloat(expected));
    assert_eq!(o, expected_o);
    assert_eq!(quo, 0);
    let (r, o, quo) = Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(
        &Rational::ZERO,
        &Float::from(3u32),
        10,
        Nearest,
    );
    assert_eq!(ComparableFloat(r), ComparableFloat(Float::ZERO));
    assert_eq!(o, Equal);
    assert_eq!(quo, 0);
}

// The exact-oracle check shared by the mixed-remainder property tests: q and r = x - qy computed
// exactly as Rationals, then compared with the function's rounded output and quotient bits.
fn check_mixed_rem_exact(
    xr: &Rational,
    yr: &Rational,
    nearest_quotient: bool,
    r: &Float,
    o: Ordering,
    quo: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let (q, _) = Integer::rounding_from(xr / yr, if nearest_quotient { Nearest } else { Down });
    assert_eq!(quo, expected_quotient_bits(&q));
    let r_exact = xr - Rational::from(q) * yr;
    if r_exact == 0u32 {
        let expected = if *xr > 0u32 {
            Float::ZERO
        } else {
            Float::NEGATIVE_ZERO
        };
        assert_eq!(ComparableFloatRef(r), ComparableFloatRef(&expected));
        assert_eq!(o, Equal);
    } else {
        let (expected, expected_o) = Float::from_rational_prec_round(r_exact, prec, rm);
        assert_eq!(ComparableFloatRef(r), ComparableFloatRef(&expected));
        assert_eq!(o, expected_o);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn rem_rational_prec_round_properties_helper(
    x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
    nearest_quotient: bool,
) {
    type F = fn(&Float, &Rational, u64, RoundingMode) -> (Float, Ordering, i64);
    let quo_fn: F = if nearest_quotient {
        Float::ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref
    } else {
        Float::rem_rational_and_quotient_bits_prec_round_ref_ref
    };
    let (rem, o, quo) = quo_fn(&x, &y, prec, rm);
    assert!(rem.is_valid());

    // all ownership variants and assigns agree
    if nearest_quotient {
        let (r2, o2) = x
            .clone()
            .ieee_remainder_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x
            .clone()
            .ieee_remainder_rational_prec_round_val_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.ieee_remainder_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.ieee_remainder_rational_prec_round_assign(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.ieee_remainder_rational_prec_round_assign_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2, quo2) = x
            .clone()
            .ieee_remainder_rational_and_quotient_bits_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);
    } else {
        let (r2, o2) = x.clone().rem_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().rem_rational_prec_round_val_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.rem_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.rem_rational_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.rem_rational_prec_round_assign(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.rem_rational_prec_round_assign_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2, quo2) =
            x.clone()
                .rem_rational_and_quotient_bits_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);
    }

    if rem.is_normal() {
        assert_eq!(rem.get_prec(), Some(prec));
    }

    if x.is_finite() && x != 0u32 && y != 0u32 {
        check_mixed_rem_exact(
            &Rational::exact_from(&x),
            &y,
            nearest_quotient,
            &rem,
            o,
            quo,
            prec,
            rm,
        );
    }

    // rem(-x, y) = -rem(x, y); rem(x, -y) = rem(x, y)
    let (mut r2, mut o2, quo2) = quo_fn(&-&x, &y, prec, -rm);
    r2.neg_assign();
    o2 = o2.reverse();
    assert_eq!(
        ComparableFloat(r2.abs_negative_zero()),
        ComparableFloat(rem.abs_negative_zero_ref())
    );
    assert_eq!(o2, o);
    assert_eq!(quo2, quo.wrapping_neg());
    let (r2, o2, quo2) = quo_fn(&x, &-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
    assert_eq!(o2, o);
    assert_eq!(quo2, quo.wrapping_neg());

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo, _) = quo_fn(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(rem.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(quo_fn(&x, &y, prec, Exact));
    }
}

#[test]
fn rem_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, y, prec, rm)| {
            rem_rational_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );
}

#[test]
fn ieee_remainder_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, y, prec, rm)| {
            rem_rational_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn rational_rem_float_prec_round_properties_helper(
    x: Rational,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    nearest_quotient: bool,
) {
    type F = fn(&Rational, &Float, u64, RoundingMode) -> (Float, Ordering, i64);
    let quo_fn: F = if nearest_quotient {
        Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref
    } else {
        Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref
    };
    let (rem, o, quo) = quo_fn(&x, &y, prec, rm);
    assert!(rem.is_valid());

    if nearest_quotient {
        let (r2, o2) =
            Float::rational_ieee_remainder_float_prec_round(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) =
            Float::rational_ieee_remainder_float_prec_round_val_ref(x.clone(), &y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) =
            Float::rational_ieee_remainder_float_prec_round_ref_val(&x, y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = Float::rational_ieee_remainder_float_prec_round_ref_ref(&x, &y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2, quo2) = Float::rational_ieee_remainder_float_and_quotient_bits_prec_round(
            x.clone(),
            y.clone(),
            prec,
            rm,
        );
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);
    } else {
        let (r2, o2) = Float::rational_rem_float_prec_round(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = Float::rational_rem_float_prec_round_val_ref(x.clone(), &y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = Float::rational_rem_float_prec_round_ref_val(&x, y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = Float::rational_rem_float_prec_round_ref_ref(&x, &y, prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2, quo2) =
            Float::rational_rem_float_and_quotient_bits_prec_round(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);
    }

    if rem.is_normal() {
        assert_eq!(rem.get_prec(), Some(prec));
    }

    if y.is_finite() && y != 0u32 && x != 0u32 {
        check_mixed_rem_exact(
            &x,
            &Rational::exact_from(&y),
            nearest_quotient,
            &rem,
            o,
            quo,
            prec,
            rm,
        );
    }

    // rem(-x, y) = -rem(x, y); rem(x, -y) = rem(x, y)
    let (mut r2, mut o2, quo2) = quo_fn(&-&x, &y, prec, -rm);
    r2.neg_assign();
    o2 = o2.reverse();
    assert_eq!(
        ComparableFloat(r2.abs_negative_zero()),
        ComparableFloat(rem.abs_negative_zero_ref())
    );
    assert_eq!(o2, o);
    assert_eq!(quo2, quo.wrapping_neg());
    let (r2, o2, quo2) = quo_fn(&x, &-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
    assert_eq!(o2, o);
    assert_eq!(quo2, quo.wrapping_neg());

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo, _) = quo_fn(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(rem.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(quo_fn(&x, &y, prec, Exact));
    }
}

#[test]
fn rational_rem_float_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_19().test_properties(
        |(x, y, prec, rm)| {
            rational_rem_float_prec_round_properties_helper(y, x, prec, rm, false);
        },
    );
}

#[test]
fn rational_ieee_remainder_float_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_20().test_properties(
        |(x, y, prec, rm)| {
            rational_rem_float_prec_round_properties_helper(y, x, prec, rm, true);
        },
    );
}

// The shorthand levels and operators are consistent with prec_round.
#[test]
fn rem_rational_shorthand_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (rem, o) = x.rem_rational_prec_round_ref_ref(&y, prec, Nearest);
        let (r2, o2) = x.rem_rational_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().rem_rational_prec(y.clone(), prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.rem_rational_prec_assign(y.clone(), prec);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2, _) = x.rem_rational_and_quotient_bits_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);

        let (rem, o) = x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, Nearest);
        let (r2, o2) = x.ieee_remainder_rational_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = Float::rational_rem_float_prec_ref_ref(&y, &x, prec);
        let (r3, o3) = Float::rational_rem_float_prec_round_ref_ref(&y, &x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r3));
        assert_eq!(o2, o3);
    });
}

#[test]
fn rem_rational_round_properties() {
    float_rational_rounding_mode_triple_gen_var_16().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (rem, o) = x.rem_rational_prec_round_ref_ref(&y, prec, rm);
        let (r2, o2) = x.rem_rational_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2) = x.clone().rem_rational_round(y.clone(), rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.rem_rational_round_assign_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
        let (r2, o2, _) = x.rem_rational_and_quotient_bits_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_17().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (rem, o) = x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm);
        let (r2, o2) = x.ieee_remainder_rational_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_18().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (rem, o) = Float::rational_rem_float_prec_round_ref_ref(&y, &x, prec, rm);
        let (r2, o2) = Float::rational_rem_float_round_ref_ref(&y, &x, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_19().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (rem, o) = Float::rational_ieee_remainder_float_prec_round_ref_ref(&y, &x, prec, rm);
        let (r2, o2) = Float::rational_ieee_remainder_float_round_ref_ref(&y, &x, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        assert_eq!(o2, o);
    });
}

#[test]
fn rem_rational_operator_properties() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        let rem = &x % &y;
        assert!(rem.is_valid());
        let r2 = &x % y.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        let r2 = x.clone() % &y;
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        let r2 = x.clone() % y.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        let mut x2 = x.clone();
        x2 %= y.clone();
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        let mut x2 = x.clone();
        x2 %= &y;
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&rem));
        let (r2, _) = x.rem_rational_round_ref_ref(&y, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));

        let rem = &y % &x;
        assert!(rem.is_valid());
        let r2 = &y % x.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        let r2 = y.clone() % &x;
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        let r2 = y.clone() % x.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));
        let (r2, _) = Float::rational_rem_float_round_ref_ref(&y, &x, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&rem));

        let ieee = x.ieee_remainder_rational_ref_ref(&y);
        let r2 = x.clone().ieee_remainder_rational(y.clone());
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&ieee));
        let r2 = x.clone().ieee_remainder_rational_val_ref(&y);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&ieee));
        let r2 = x.ieee_remainder_rational_ref_val(y.clone());
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&ieee));
        let mut x2 = x.clone();
        x2.ieee_remainder_rational_assign_ref(&y);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&ieee));

        let ieee = Float::rational_ieee_remainder_float_ref_ref(&y, &x);
        let r2 = Float::rational_ieee_remainder_float(y.clone(), x.clone());
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&ieee));

        let (r2, _, quo) = x.rem_rational_and_quotient_bits_ref_ref(&y);
        let (r3, _, quo2) = x.clone().rem_rational_and_quotient_bits(y.clone());
        assert_eq!(ComparableFloatRef(&r3), ComparableFloatRef(&r2));
        assert_eq!(quo2, quo);
        let (r2, _, quo) = Float::rational_rem_float_and_quotient_bits_ref_ref(&y, &x);
        let (r3, _, quo2) = Float::rational_rem_float_and_quotient_bits(y.clone(), x.clone());
        assert_eq!(ComparableFloatRef(&r3), ComparableFloatRef(&r2));
        assert_eq!(quo2, quo);
    });
}

#[test]
fn test_rem_rational() {
    let test = |s, s_hex, t: &str, out: &str, out_hex: &str, o_out: Ordering, quo_out: i64| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let (r, o, quo) = x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(quo, quo_out);

        let (r2, o2, quo2) =
            x.clone()
                .rem_rational_and_quotient_bits_prec_round(y.clone(), 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);
        let (r2, o2, quo2) = x
            .clone()
            .rem_rational_and_quotient_bits_prec_round_val_ref(&y, 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);
        let (r2, o2, quo2) =
            x.rem_rational_and_quotient_bits_prec_round_ref_val(y.clone(), 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        assert_eq!(quo2, quo);

        let (r2, o2) = x.rem_rational_prec_round_ref_ref(&y, 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = x.rem_rational_prec_ref_ref(&y, 10);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    };
    // NaN, infinite x, or a zero Rational modulus: NaN with quotient bits 0; a zero x or a zero
    // remainder keeps x's sign; otherwise the exact truncated-quotient remainder, rounded to
    // precision 10
    test("NaN", "NaN", "1/3", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "-1/3", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "22/7", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "7", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "3/8", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "0", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "1/3", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "-1/3", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "22/7", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "7", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "3/8", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "0", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "1/3", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "-1/3", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "22/7", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "7", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "3/8", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "0", "NaN", "NaN", Equal, 0);
    test("0.0", "0x0.0", "1/3", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "-1/3", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "22/7", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "7", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "3/8", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "0", "NaN", "NaN", Equal, 0);
    test("-0.0", "-0x0.0", "1/3", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "-1/3", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "22/7", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "7", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "3/8", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "0", "NaN", "NaN", Equal, 0);
    test("10.0", "0xa.0#3", "1/3", "0.0", "0x0.0", Equal, 30);
    test("10.0", "0xa.0#3", "-1/3", "0.0", "0x0.0", Equal, -30);
    test("10.0", "0xa.0#3", "22/7", "0.57129", "0x0.924#10", Less, 3);
    test("10.0", "0xa.0#3", "7", "3.0000", "0x3.00#10", Equal, 1);
    test("10.0", "0xa.0#3", "3/8", "0.25000", "0x0.400#10", Equal, 26);
    test("10.0", "0xa.0#3", "0", "NaN", "NaN", Equal, 0);
    test("-10.0", "-0xa.0#3", "1/3", "-0.0", "-0x0.0", Equal, -30);
    test("-10.0", "-0xa.0#3", "-1/3", "-0.0", "-0x0.0", Equal, 30);
    test(
        "-10.0",
        "-0xa.0#3",
        "22/7",
        "-0.57129",
        "-0x0.924#10",
        Greater,
        -3,
    );
    test("-10.0", "-0xa.0#3", "7", "-3.0000", "-0x3.00#10", Equal, -1);
    test(
        "-10.0",
        "-0xa.0#3",
        "3/8",
        "-0.25000",
        "-0x0.400#10",
        Equal,
        -26,
    );
    test("-10.0", "-0xa.0#3", "0", "NaN", "NaN", Equal, 0);
    test("3.0", "0x3.0#2", "1/3", "0.0", "0x0.0", Equal, 9);
    test("3.0", "0x3.0#2", "-1/3", "0.0", "0x0.0", Equal, -9);
    test("3.0", "0x3.0#2", "22/7", "3.0000", "0x3.00#10", Equal, 0);
    test("3.0", "0x3.0#2", "7", "3.0000", "0x3.00#10", Equal, 0);
    test("3.0", "0x3.0#2", "3/8", "0.0", "0x0.0", Equal, 8);
    test("3.0", "0x3.0#2", "0", "NaN", "NaN", Equal, 0);
    test(
        "10.5",
        "0xa.8#6",
        "1/3",
        "0.16675",
        "0x0.2ab#10",
        Greater,
        31,
    );
    test(
        "10.5",
        "0xa.8#6",
        "-1/3",
        "0.16675",
        "0x0.2ab#10",
        Greater,
        -31,
    );
    test(
        "10.5",
        "0xa.8#6",
        "22/7",
        "1.0723",
        "0x1.128#10",
        Greater,
        3,
    );
    test("10.5", "0xa.8#6", "7", "3.5000", "0x3.80#10", Equal, 1);
    test("10.5", "0xa.8#6", "3/8", "0.0", "0x0.0", Equal, 28);
    test("10.5", "0xa.8#6", "0", "NaN", "NaN", Equal, 0);
}

#[test]
fn test_ieee_remainder_rational() {
    let test = |s, s_hex, t: &str, out: &str, out_hex: &str, o_out: Ordering, quo_out: i64| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let (r, o, quo) =
            x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, 10, Nearest);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(quo, quo_out);

        let (r2, o2) = x.ieee_remainder_rational_prec_round_ref_ref(&y, 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = x.ieee_remainder_rational_prec_ref_ref(&y, 10);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    };
    // the nearest-even quotient variant of the previous test's rows
    test("NaN", "NaN", "1/3", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "-1/3", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "22/7", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "7", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "3/8", "NaN", "NaN", Equal, 0);
    test("NaN", "NaN", "0", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "1/3", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "-1/3", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "22/7", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "7", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "3/8", "NaN", "NaN", Equal, 0);
    test("Infinity", "Infinity", "0", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "1/3", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "-1/3", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "22/7", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "7", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "3/8", "NaN", "NaN", Equal, 0);
    test("-Infinity", "-Infinity", "0", "NaN", "NaN", Equal, 0);
    test("0.0", "0x0.0", "1/3", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "-1/3", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "22/7", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "7", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "3/8", "0.0", "0x0.0", Equal, 0);
    test("0.0", "0x0.0", "0", "NaN", "NaN", Equal, 0);
    test("-0.0", "-0x0.0", "1/3", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "-1/3", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "22/7", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "7", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "3/8", "-0.0", "-0x0.0", Equal, 0);
    test("-0.0", "-0x0.0", "0", "NaN", "NaN", Equal, 0);
    test("10.0", "0xa.0#3", "1/3", "0.0", "0x0.0", Equal, 30);
    test("10.0", "0xa.0#3", "-1/3", "0.0", "0x0.0", Equal, -30);
    test("10.0", "0xa.0#3", "22/7", "0.57129", "0x0.924#10", Less, 3);
    test("10.0", "0xa.0#3", "7", "3.0000", "0x3.00#10", Equal, 1);
    test(
        "10.0",
        "0xa.0#3",
        "3/8",
        "-0.12500",
        "-0x0.200#10",
        Equal,
        27,
    );
    test("10.0", "0xa.0#3", "0", "NaN", "NaN", Equal, 0);
    test("-10.0", "-0xa.0#3", "1/3", "-0.0", "-0x0.0", Equal, -30);
    test("-10.0", "-0xa.0#3", "-1/3", "-0.0", "-0x0.0", Equal, 30);
    test(
        "-10.0",
        "-0xa.0#3",
        "22/7",
        "-0.57129",
        "-0x0.924#10",
        Greater,
        -3,
    );
    test("-10.0", "-0xa.0#3", "7", "-3.0000", "-0x3.00#10", Equal, -1);
    test(
        "-10.0",
        "-0xa.0#3",
        "3/8",
        "0.12500",
        "0x0.200#10",
        Equal,
        -27,
    );
    test("-10.0", "-0xa.0#3", "0", "NaN", "NaN", Equal, 0);
    test("3.0", "0x3.0#2", "1/3", "0.0", "0x0.0", Equal, 9);
    test("3.0", "0x3.0#2", "-1/3", "0.0", "0x0.0", Equal, -9);
    test(
        "3.0",
        "0x3.0#2",
        "22/7",
        "-0.14282",
        "-0x0.249#10",
        Greater,
        1,
    );
    test("3.0", "0x3.0#2", "7", "3.0000", "0x3.00#10", Equal, 0);
    test("3.0", "0x3.0#2", "3/8", "0.0", "0x0.0", Equal, 8);
    test("3.0", "0x3.0#2", "0", "NaN", "NaN", Equal, 0);
    test(
        "10.5",
        "0xa.8#6",
        "1/3",
        "-0.16675",
        "-0x0.2ab#10",
        Less,
        32,
    );
    test(
        "10.5",
        "0xa.8#6",
        "-1/3",
        "-0.16675",
        "-0x0.2ab#10",
        Less,
        -32,
    );
    test(
        "10.5",
        "0xa.8#6",
        "22/7",
        "1.0723",
        "0x1.128#10",
        Greater,
        3,
    );
    test("10.5", "0xa.8#6", "7", "-3.5000", "-0x3.80#10", Equal, 2);
    test("10.5", "0xa.8#6", "3/8", "0.0", "0x0.0", Equal, 28);
    test("10.5", "0xa.8#6", "0", "NaN", "NaN", Equal, 0);
}

#[test]
fn test_rational_rem_float() {
    let test = |t: &str, s, s_hex, out: &str, out_hex: &str, o_out: Ordering, quo_out: i64| {
        let x = t.parse::<Rational>().unwrap();
        let y = parse_hex_string(s_hex);
        assert_eq!(y.to_string(), s);

        let (r, o, quo) =
            Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(&x, &y, 10, Nearest);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(quo, quo_out);

        let (r2, o2) = Float::rational_rem_float_prec_round_ref_ref(&x, &y, 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let (r2, o2) = Float::rational_rem_float_prec_ref_ref(&x, &y, 10);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    };
    // a NaN or zero Float modulus is NaN; an infinite modulus returns the Rational rounded; a zero
    // Rational dividend is a positive zero
    test("1/3", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("-1/3", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("22/7", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("7", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("3/8", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("0", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test(
        "1/3",
        "Infinity",
        "Infinity",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "Infinity",
        "Infinity",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test(
        "22/7",
        "Infinity",
        "Infinity",
        "3.1445",
        "0x3.25#10",
        Greater,
        0,
    );
    test("7", "Infinity", "Infinity", "7.0000", "0x7.00#10", Equal, 0);
    test(
        "3/8",
        "Infinity",
        "Infinity",
        "0.37500",
        "0x0.600#10",
        Equal,
        0,
    );
    test("0", "Infinity", "Infinity", "0.0", "0x0.0", Equal, 0);
    test(
        "1/3",
        "-Infinity",
        "-Infinity",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "-Infinity",
        "-Infinity",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test(
        "22/7",
        "-Infinity",
        "-Infinity",
        "3.1445",
        "0x3.25#10",
        Greater,
        0,
    );
    test(
        "7",
        "-Infinity",
        "-Infinity",
        "7.0000",
        "0x7.00#10",
        Equal,
        0,
    );
    test(
        "3/8",
        "-Infinity",
        "-Infinity",
        "0.37500",
        "0x0.600#10",
        Equal,
        0,
    );
    test("0", "-Infinity", "-Infinity", "0.0", "0x0.0", Equal, 0);
    test("1/3", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("-1/3", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("22/7", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("7", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("3/8", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("0", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("1/3", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("-1/3", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("22/7", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("7", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("3/8", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("0", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test(
        "1/3",
        "10.0",
        "0xa.0#3",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "10.0",
        "0xa.0#3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test("22/7", "10.0", "0xa.0#3", "3.1445", "0x3.25#10", Greater, 0);
    test("7", "10.0", "0xa.0#3", "7.0000", "0x7.00#10", Equal, 0);
    test("3/8", "10.0", "0xa.0#3", "0.37500", "0x0.600#10", Equal, 0);
    test("0", "10.0", "0xa.0#3", "0.0", "0x0.0", Equal, 0);
    test(
        "1/3",
        "-10.0",
        "-0xa.0#3",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "-10.0",
        "-0xa.0#3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test(
        "22/7",
        "-10.0",
        "-0xa.0#3",
        "3.1445",
        "0x3.25#10",
        Greater,
        0,
    );
    test("7", "-10.0", "-0xa.0#3", "7.0000", "0x7.00#10", Equal, 0);
    test(
        "3/8",
        "-10.0",
        "-0xa.0#3",
        "0.37500",
        "0x0.600#10",
        Equal,
        0,
    );
    test("0", "-10.0", "-0xa.0#3", "0.0", "0x0.0", Equal, 0);
    test("1/3", "3.0", "0x3.0#2", "0.33350", "0x0.556#10", Greater, 0);
    test("-1/3", "3.0", "0x3.0#2", "-0.33350", "-0x0.556#10", Less, 0);
    test("22/7", "3.0", "0x3.0#2", "0.14282", "0x0.249#10", Less, 1);
    test("7", "3.0", "0x3.0#2", "1.0000", "0x1.000#10", Equal, 2);
    test("3/8", "3.0", "0x3.0#2", "0.37500", "0x0.600#10", Equal, 0);
    test("0", "3.0", "0x3.0#2", "0.0", "0x0.0", Equal, 0);
    test(
        "1/3",
        "10.5",
        "0xa.8#6",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "10.5",
        "0xa.8#6",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test("22/7", "10.5", "0xa.8#6", "3.1445", "0x3.25#10", Greater, 0);
    test("7", "10.5", "0xa.8#6", "7.0000", "0x7.00#10", Equal, 0);
    test("3/8", "10.5", "0xa.8#6", "0.37500", "0x0.600#10", Equal, 0);
    test("0", "10.5", "0xa.8#6", "0.0", "0x0.0", Equal, 0);
}

#[test]
fn test_rational_ieee_remainder_float() {
    let test = |t: &str, s, s_hex, out: &str, out_hex: &str, o_out: Ordering, quo_out: i64| {
        let x = t.parse::<Rational>().unwrap();
        let y = parse_hex_string(s_hex);
        assert_eq!(y.to_string(), s);

        let (r, o, quo) = Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref(
            &x, &y, 10, Nearest,
        );
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
        assert_eq!(quo, quo_out);

        let (r2, o2) = Float::rational_ieee_remainder_float_prec_round_ref_ref(&x, &y, 10, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    };
    // the nearest-even quotient variant of the previous test's rows
    test("1/3", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("-1/3", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("22/7", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("7", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("3/8", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test("0", "NaN", "NaN", "NaN", "NaN", Equal, 0);
    test(
        "1/3",
        "Infinity",
        "Infinity",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "Infinity",
        "Infinity",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test(
        "22/7",
        "Infinity",
        "Infinity",
        "3.1445",
        "0x3.25#10",
        Greater,
        0,
    );
    test("7", "Infinity", "Infinity", "7.0000", "0x7.00#10", Equal, 0);
    test(
        "3/8",
        "Infinity",
        "Infinity",
        "0.37500",
        "0x0.600#10",
        Equal,
        0,
    );
    test("0", "Infinity", "Infinity", "0.0", "0x0.0", Equal, 0);
    test(
        "1/3",
        "-Infinity",
        "-Infinity",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "-Infinity",
        "-Infinity",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test(
        "22/7",
        "-Infinity",
        "-Infinity",
        "3.1445",
        "0x3.25#10",
        Greater,
        0,
    );
    test(
        "7",
        "-Infinity",
        "-Infinity",
        "7.0000",
        "0x7.00#10",
        Equal,
        0,
    );
    test(
        "3/8",
        "-Infinity",
        "-Infinity",
        "0.37500",
        "0x0.600#10",
        Equal,
        0,
    );
    test("0", "-Infinity", "-Infinity", "0.0", "0x0.0", Equal, 0);
    test("1/3", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("-1/3", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("22/7", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("7", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("3/8", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("0", "0.0", "0x0.0", "NaN", "NaN", Equal, 0);
    test("1/3", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("-1/3", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("22/7", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("7", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("3/8", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test("0", "-0.0", "-0x0.0", "NaN", "NaN", Equal, 0);
    test(
        "1/3",
        "10.0",
        "0xa.0#3",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "10.0",
        "0xa.0#3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test("22/7", "10.0", "0xa.0#3", "3.1445", "0x3.25#10", Greater, 0);
    test("7", "10.0", "0xa.0#3", "-3.0000", "-0x3.00#10", Equal, 1);
    test("3/8", "10.0", "0xa.0#3", "0.37500", "0x0.600#10", Equal, 0);
    test("0", "10.0", "0xa.0#3", "0.0", "0x0.0", Equal, 0);
    test(
        "1/3",
        "-10.0",
        "-0xa.0#3",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "-10.0",
        "-0xa.0#3",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test(
        "22/7",
        "-10.0",
        "-0xa.0#3",
        "3.1445",
        "0x3.25#10",
        Greater,
        0,
    );
    test("7", "-10.0", "-0xa.0#3", "-3.0000", "-0x3.00#10", Equal, -1);
    test(
        "3/8",
        "-10.0",
        "-0xa.0#3",
        "0.37500",
        "0x0.600#10",
        Equal,
        0,
    );
    test("0", "-10.0", "-0xa.0#3", "0.0", "0x0.0", Equal, 0);
    test("1/3", "3.0", "0x3.0#2", "0.33350", "0x0.556#10", Greater, 0);
    test("-1/3", "3.0", "0x3.0#2", "-0.33350", "-0x0.556#10", Less, 0);
    test("22/7", "3.0", "0x3.0#2", "0.14282", "0x0.249#10", Less, 1);
    test("7", "3.0", "0x3.0#2", "1.0000", "0x1.000#10", Equal, 2);
    test("3/8", "3.0", "0x3.0#2", "0.37500", "0x0.600#10", Equal, 0);
    test("0", "3.0", "0x3.0#2", "0.0", "0x0.0", Equal, 0);
    test(
        "1/3",
        "10.5",
        "0xa.8#6",
        "0.33350",
        "0x0.556#10",
        Greater,
        0,
    );
    test(
        "-1/3",
        "10.5",
        "0xa.8#6",
        "-0.33350",
        "-0x0.556#10",
        Less,
        0,
    );
    test("22/7", "10.5", "0xa.8#6", "3.1445", "0x3.25#10", Greater, 0);
    test("7", "10.5", "0xa.8#6", "-3.5000", "-0x3.80#10", Equal, 1);
    test("3/8", "10.5", "0xa.8#6", "0.37500", "0x0.600#10", Equal, 0);
    test("0", "10.5", "0xa.8#6", "0.0", "0x0.0", Equal, 0);
}

#[test]
fn test_rem_rational_prec_round() {
    let test = |s, s_hex, t: &str, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let (r, o) = x.rem_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);

        let (r2, o2) = x.clone().rem_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.rem_rational_prec_round_assign(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.rem_rational_prec_round_assign_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&r));
        assert_eq!(o2, o);
    };
    // 10 mod 22/7 = 4/7, rounded at precision 1 in each direction; exact multiples of 1/3 and 3/8
    // (Exact is allowed when the remainder is exactly representable)
    test("10.0", "0xa.0#3", "22/7", 1, Floor, "0.50", "0x0.8#1", Less);
    test(
        "10.0", "0xa.0#3", "22/7", 1, Ceiling, "1.0", "0x1.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "22/7", 1, Nearest, "0.50", "0x0.8#1", Less,
    );
    test(
        "-10.0", "-0xa.0#3", "22/7", 1, Floor, "-1.0", "-0x1.0#1", Less,
    );
    test("10.0", "0xa.0#3", "1/3", 2, Floor, "0.0", "0x0.0", Equal);
    test("10.0", "0xa.0#3", "1/3", 2, Ceiling, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", "3/8", 4, Exact, "0.0", "0x0.0", Equal);
}

#[test]
fn test_rem_rational_operators() {
    let test = |s, s_hex, t: &str, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let r = &x % &y;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        let r2 = &x % y.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        let r2 = x.clone() % &y;
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        let r2 = x.clone() % y.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        let mut x2 = x.clone();
        x2 %= y.clone();
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&r));
        let mut x2 = x.clone();
        x2 %= &y;
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&r));
    };
    // the Float-mod-Rational operator, at the Float's precision
    test("10.0", "0xa.0#3", "22/7", "0.62", "0x0.a#3");
    test("-10.0", "-0xa.0#3", "22/7", "-0.62", "-0x0.a#3");
    test("3.0", "0x3.0#2", "1/3", "0.0", "0x0.0");
}

#[test]
fn rem_rational_fail() {
    assert_panic!(Float::from(1u32).rem_rational_prec_round(
        Rational::from_signeds(1i32, 3i32),
        0,
        Nearest
    ));
    assert_panic!(Float::rational_rem_float_prec_round(
        Rational::from_signeds(1i32, 3i32),
        Float::from(1u32),
        0,
        Nearest
    ));
    // Exact with an inexact remainder
    assert_panic!(Float::from(10u32).rem_rational_prec_round(
        Rational::from_signeds(22i32, 7i32),
        1,
        Exact
    ));
    assert_panic!(Float::from(10u32).ieee_remainder_rational_prec_round(
        Rational::from_signeds(22i32, 7i32),
        1,
        Exact
    ));
}

#[test]
fn test_rational_rem_float_operator() {
    let test = |t: &str, s, s_hex, out: &str, out_hex: &str| {
        let x = t.parse::<Rational>().unwrap();
        let y = parse_hex_string(s_hex);
        assert_eq!(y.to_string(), s);

        let r = &x % &y;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        let r2 = &x % y.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        let r2 = x.clone() % &y;
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        let r2 = x.clone() % y.clone();
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
    };
    // the Rational-mod-Float operator, at the Float's precision
    test("22/7", "3.0", "0x3.0#2", "0.12", "0x0.2#2");
    test("-22/7", "3.0", "0x3.0#2", "-0.12", "-0x0.2#2");
    test("1/3", "4.0", "0x4.0#1", "0.25", "0x0.4#1");
}

// The emulated primitive-float remainders: the truncated-quotient form must agree bit-for-bit with
// the primitive `%` operator (the floating-point remainder is always exactly representable), and
// the others are checked against exact Rational arithmetic.
#[test]
fn primitive_float_rem_properties() {
    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        let r = primitive_float_rem(x, y);
        assert_eq!(NiceFloat(r), NiceFloat(x % y));
        let (r2, quo) = primitive_float_rem_and_quotient_bits(x, y);
        assert_eq!(NiceFloat(r2), NiceFloat(r));
        let ieee = primitive_float_ieee_remainder(x, y);
        let (ieee2, iquo) = primitive_float_ieee_remainder_and_quotient_bits(x, y);
        assert_eq!(NiceFloat(ieee2), NiceFloat(ieee));
        if x.is_finite() && y.is_finite() && x != 0.0 && y != 0.0 {
            let xr = Rational::exact_from(x);
            let yr = Rational::exact_from(y);
            let (q, _) = Integer::rounding_from(&xr / &yr, Down);
            assert_eq!(quo, expected_quotient_bits(&q));
            let r_exact = &xr - Rational::from(q) * &yr;
            if r_exact != 0u32 {
                assert_eq!(Rational::exact_from(r), r_exact);
            } else {
                assert_eq!(r, 0.0);
            }
            let (q, _) = Integer::rounding_from(&xr / &yr, Nearest);
            assert_eq!(iquo, expected_quotient_bits(&q));
            let r_exact = xr - Rational::from(q) * yr;
            if r_exact != 0u32 {
                assert_eq!(Rational::exact_from(ieee), r_exact);
            } else {
                assert_eq!(ieee, 0.0);
            }
        }
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        for y in test_rationals() {
            let r = primitive_float_rem_rational(x, &y);
            let (r2, quo) = primitive_float_rem_rational_and_quotient_bits(x, &y);
            assert_eq!(NiceFloat(r2), NiceFloat(r));
            let ieee = primitive_float_ieee_remainder_rational(x, &y);
            let (ieee2, iquo) = primitive_float_ieee_remainder_rational_and_quotient_bits(x, &y);
            assert_eq!(NiceFloat(ieee2), NiceFloat(ieee));
            let rev = primitive_float_rational_rem_float(&y, x);
            let rev_ieee = primitive_float_rational_ieee_remainder_float(&y, x);
            if x.is_finite() && x != 0.0 {
                let xr = Rational::exact_from(x);
                let (q, _) = Integer::rounding_from(&xr / &y, Down);
                assert_eq!(quo, expected_quotient_bits(&q));
                let r_exact = &xr - Rational::from(q) * &y;
                if r_exact != 0u32 {
                    assert_eq!(
                        NiceFloat(r),
                        NiceFloat(f64::rounding_from(&r_exact, Nearest).0)
                    );
                }
                let (q, _) = Integer::rounding_from(&xr / &y, Nearest);
                assert_eq!(iquo, expected_quotient_bits(&q));
                // reversed direction: y is the dividend
                let (q, _) = Integer::rounding_from(&y / &xr, Down);
                let r_exact = &y - Rational::from(q) * &xr;
                if r_exact != 0u32 {
                    assert_eq!(
                        NiceFloat(rev),
                        NiceFloat(f64::rounding_from(&r_exact, Nearest).0)
                    );
                }
                let _ = rev_ieee;
            }
        }
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        for u in [0u64, 1, 3, 7, u64::MAX >> 1, u64::MAX] {
            let r = primitive_float_rem_unsigned(x, u);
            if u == 0 {
                assert!(r.is_nan());
            } else if x.is_finite() && x != 0.0 {
                let xr = Rational::exact_from(x);
                let ur = Rational::from(u);
                let (q, _) = Integer::rounding_from(&xr / &ur, Down);
                let r_exact = xr - Rational::from(q) * ur;
                if r_exact != 0u32 {
                    assert_eq!(
                        NiceFloat(r),
                        NiceFloat(f64::rounding_from(&r_exact, Nearest).0)
                    );
                }
            }
        }
    });

    primitive_float_pair_gen::<f32>().test_properties(|(x, y)| {
        let r = primitive_float_rem(x, y);
        assert_eq!(NiceFloat(r), NiceFloat(x % y));
    });
}

// Regression test: with a divisor of more than 2^30 bits whose mantissa is odd, the integer
// remainder has more than 2^30 bits, and the fast path used to round it to a `Float` before
// applying the shift, overflowing the intermediate to infinity even though the result is tiny. This
// is the shape of `cos` reducing a near-maximal-exponent argument modulo 2 pi.
#[test]
fn test_ieee_remainder_huge_precision_divisor() {
    let x = Float::from_unsigned_prec(3u32, 64).0 << ((1u64 << 30) - 3);
    let c = (Float::ONE << 2u32)
        .add_prec_round(Float::ONE >> ((1u64 << 30) + 70), (1u64 << 30) + 73, Exact)
        .0;
    let (r, o) = x.ieee_remainder_prec_round_ref_ref(&c, 77, Nearest);
    assert!(r.is_finite());
    assert!(r.le_abs(&(&c >> 1u32)));
    let x_q = Rational::exact_from(&x);
    let c_q = Rational::exact_from(&c);
    let q = Integer::rounding_from(&x_q / &c_q, Nearest).0;
    let r_exact = x_q - c_q * Rational::from(q);
    let (r_expected, o_expected) = Float::from_rational_prec_round(r_exact, 77, Nearest);
    assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&r_expected));
    assert_eq!(o, o_expected);
}
