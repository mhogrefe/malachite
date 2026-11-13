// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two, Zero};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::hypot::hypot_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6,
    float_float_unsigned_rounding_mode_quadruple_gen_var_24,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn pi() -> BigExponentFloat {
    BigExponentFloat::from(core::f64::consts::PI)
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn big() -> Integer {
    Integer::from_str("100000000000000000000").unwrap()
}

fn p2(e: Integer) -> BigExponentFloat {
    big_exponent_float_power_of_2(e)
}

// Whether `x` is finite and nonzero.
const fn finite_nonzero(x: &BigExponentFloat) -> bool {
    x.is_finite() && !x.is_zero()
}

fn hypot_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().hypot_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().hypot_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.hypot_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.hypot_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.hypot_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.hypot_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().hypot_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().hypot_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.hypot_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.hypot_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.hypot_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.hypot_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn check(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
    out: &BigExponentFloat,
    o: Ordering,
) {
    let r = hypot_prec_round_all(x, y, prec, rm);
    assert!(
        same(&r.0, out),
        "{x:?} {y:?} {prec} {rm}: {:?} != {out:?}",
        r.0
    );
    assert_eq!(r.1, o);
    assert!(same_pair(&hypot_prec_round_generic(x, y, prec, rm), &r));
}

fn check_pi(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
    out: &BigExponentFloat,
    o: Ordering,
) {
    check(
        x,
        &BigExponentFloat::from(core::f64::consts::E),
        prec,
        rm,
        out,
        o,
    );
}

#[test]
fn test_hypot_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    check(&nan, &ninf, 10, Floor, &inf, Equal);
    check(&ninf, &nan, 10, Floor, &inf, Equal);
    check(&nan, &one, 10, Floor, &nan, Equal);
    check(&one, &nan, 10, Floor, &nan, Equal);
    check(&zero, &zero, 10, Floor, &zero, Equal);
    check(
        &zero,
        &BigExponentFloat::from(-3),
        10,
        Floor,
        &BigExponentFloat::from(3),
        Equal,
    );
    check(
        &BigExponentFloat::from(-3),
        &zero,
        1,
        Floor,
        &BigExponentFloat::TWO,
        Less,
    );
    check(
        &BigExponentFloat::from(3),
        &BigExponentFloat::from(-4),
        3,
        Exact,
        &BigExponentFloat::from(5),
        Equal,
    );
    // Exponents beyond the range of a `Float`: hypot(3 * 2^N, 4 * 2^N) = 5 * 2^N.
    check(
        &(p2(big()) * BigExponentFloat::from(3)),
        &(p2(big()) * BigExponentFloat::from(4)),
        3,
        Exact,
        &(p2(big()) * BigExponentFloat::from(5)),
        Equal,
    );
    // y far below x: the hypotenuse is just above |x|.
    let tiny = p2(-big());
    check(&one, &tiny, 10, Floor, &one, Less);
    check(&one, &tiny, 10, Nearest, &one, Less);
    check(&tiny, &one, 10, Ceiling, &hex("0x1.008"), Greater);
    check(
        &p2(big()),
        &-one.clone(),
        1,
        Up,
        &p2(big() + Integer::ONE),
        Greater,
    );
    // 1 + 2^-9 has 10 bits; at 9 bits it is a midpoint, but the hypotenuse is just above it.
    let x = hex("0x1.008");
    check(&x, &tiny, 9, Nearest, &hex("0x1.01"), Greater);
    check(&x, &tiny, 10, Nearest, &x, Less);
    check_pi(&pi(), 5, Floor, &hex("0x4.0"), Less);
    check_pi(&pi(), 5, Ceiling, &hex("0x4.4"), Greater);
    check_pi(&pi(), 20, Nearest, &hex("0x4.2784"), Greater);
}

#[test]
#[should_panic]
fn hypot_prec_round_fail_1() {
    BigExponentFloat::ONE.hypot_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn hypot_prec_round_fail_2() {
    BigExponentFloat::ONE.hypot_prec_round(BigExponentFloat::ONE, 10, Exact);
}

#[test]
fn hypot_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
        .test_properties(|(x, y, prec, rm)| {
            let r = hypot_prec_round_all(&x, &y, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(&hypot_prec_round_generic(&x, &y, prec, rm), &r));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.is_sign_positive());
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            assert!(same_pair(&hypot_prec_round_all(&y, &x, prec, rm), &r));
            assert!(same_pair(&hypot_prec_round_all(&-&x, &y, prec, rm), &r));
            // Multiplying both arguments by 2 doubles the hypotenuse.
            let (s, o) = (&x << 1u32).hypot_prec_round(&y << 1u32, prec, rm);
            assert!(same(&s, &(&r.0 << 1u32)));
            assert_eq!(o, r.1);
            if finite_nonzero(&x)
                && finite_nonzero(&y)
                && big_exponent_float_exponent_is_small(&x)
                && big_exponent_float_exponent_is_small(&y)
            {
                // Comparing squares gives the ordering.
                assert_eq!(
                    (&r.0)
                        .square()
                        .partial_cmp(&((&x).square() + (&y).square())),
                    Some(r.1)
                );
            }
        });
}

#[test]
fn hypot_prec_round_float_agreement_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_24().test_properties(
        |(x, y, prec, rm)| {
            let (f, fo) = x.hypot_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = hypot_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &float_to_big_exponent_float(&y),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
