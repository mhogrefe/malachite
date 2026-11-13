// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Two, Zero};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::root::root_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen_var_3, big_exponent_float_unsigned_rounding_mode_triple_gen_var_6,
    float_unsigned_pair_gen_var_1,
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

fn cbrt_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cbrt_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.cbrt_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.cbrt_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cbrt_prec(prec), &r));
        assert!(same_pair(&x.cbrt_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.cbrt_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn check(x: &BigExponentFloat, prec: u64, rm: RoundingMode, out: &BigExponentFloat, o: Ordering) {
    let r = cbrt_prec_round_all(x, prec, rm);
    assert!(same(&r.0, out), "{x:?} {prec} {rm}: {:?} != {out:?}", r.0);
    assert_eq!(r.1, o);
    assert!(same_pair(&root_prec_round_generic(x, 3, prec, rm), &r));
}

#[test]
fn test_cbrt_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    check(&nan, 10, Floor, &nan, Equal);
    check(&inf, 10, Floor, &inf, Equal);
    check(&ninf, 10, Floor, &ninf, Equal);
    check(&zero, 10, Floor, &zero, Equal);
    check(
        &BigExponentFloat::from(27),
        2,
        Exact,
        &BigExponentFloat::from(3),
        Equal,
    );
    check(
        &BigExponentFloat::from(-27),
        2,
        Exact,
        &BigExponentFloat::from(-3),
        Equal,
    );
    check(
        &BigExponentFloat::from(0.125),
        1,
        Exact,
        &BigExponentFloat::from(0.5),
        Equal,
    );
    // Exponents beyond the range of a `Float`: cbrt(2^(3N)) = 2^N, and cbrt(2^(3N + 1)) = 2^N *
    // 2^(1/3), which rounds down to 2^N and up to 2^(N + 1) at 1 bit.
    let three = Integer::from(3);
    check(&p2(big() * &three), 1, Exact, &p2(big()), Equal);
    check(&p2(-big() * &three), 1, Exact, &p2(-big()), Equal);
    check(
        &p2(big() * &three + Integer::ONE),
        1,
        Floor,
        &p2(big()),
        Less,
    );
    check(
        &p2(big() * &three + Integer::ONE),
        1,
        Ceiling,
        &p2(big() + Integer::ONE),
        Greater,
    );
    // The cube of 2^N * 3 is 2^(3N) * 27.
    check(
        &(p2(big() * &three) * BigExponentFloat::from(27)),
        2,
        Exact,
        &(p2(big()) * BigExponentFloat::from(3)),
        Equal,
    );
    check(&pi(), 5, Floor, &hex("0x1.7"), Less);
    check(&pi(), 5, Ceiling, &hex("0x1.8"), Greater);
    check(&pi(), 20, Nearest, &hex("0x1.76ef8"), Greater);
}

#[test]
#[should_panic]
fn cbrt_prec_round_fail_1() {
    BigExponentFloat::ONE.cbrt_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cbrt_prec_round_fail_2() {
    BigExponentFloat::TWO.cbrt_prec_round(10, Exact);
}

#[test]
fn cbrt_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_6().test_properties(
        |(x, prec, rm)| {
            let r = cbrt_prec_round_all(&x, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            let g = root_prec_round_generic(&x, 3, prec, rm);
            assert!(same_pair(&g, &r), "{x:#x} {prec} {rm}: {g:?} {r:?}");
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert_eq!(r.0.is_sign_positive(), x.is_sign_positive());
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // Multiplying x by 8 doubles the cube root.
            let (s, o) = (&x << 3u32).cbrt_prec_round(prec, rm);
            assert!(same(&s, &(&r.0 << 1u32)));
            assert_eq!(o, r.1);
            // Negating x negates the cube root.
            let (s, o) = (-&x).cbrt_prec_round(prec, -rm);
            assert!(same(&s, &-&r.0));
            assert_eq!(o, r.1.reverse());
            if finite_nonzero(&x) {
                // The cube is increasing, so comparing cubes gives the ordering.
                assert_eq!((&r.0).pow(3).partial_cmp(&x), Some(r.1));
            }
        },
    );
}

#[test]
fn cbrt_prec_round_properties_2() {
    big_exponent_float_gen_var_3().test_properties(|x| {
        if finite_nonzero(&x) {
            // The cube root of an exact cube is exact.
            let prec = x.get_prec().unwrap();
            assert!(same_pair(
                &(&x).pow(3).cbrt_prec_round(prec, Exact),
                &(x, Equal)
            ));
        }
    });
}

#[test]
fn cbrt_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let (f, fo) = x.cbrt_prec_round_ref(prec, rm);
            if float_result_comparable(&f, fo) {
                let r = cbrt_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}
