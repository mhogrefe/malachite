// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::{FromStringBase, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::pow::pow_prec_round_float_kernels;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1,
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    big_exponent_float_unsigned_pair_gen_var_2,
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    float_integer_unsigned_rounding_mode_quadruple_gen_var_1,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_11,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9,
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

fn pow_integer_prec_round_all(
    x: &BigExponentFloat,
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().pow_integer_prec_round(n.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().pow_integer_prec_round_val_ref(n, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.pow_integer_prec_round_ref_val(n.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.pow_integer_prec_round_ref_ref(n, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.pow_integer_prec_round_assign(n.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.pow_integer_prec_round_assign_ref(n, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().pow_integer_prec(n.clone(), prec), &r));
        assert!(same_pair(&x.clone().pow_integer_prec_val_ref(n, prec), &r));
        assert!(same_pair(&x.pow_integer_prec_ref_val(n.clone(), prec), &r));
        assert!(same_pair(&x.pow_integer_prec_ref_ref(n, prec), &r));
        let mut z = x.clone();
        let o = z.pow_integer_prec_assign(n.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.pow_integer_prec_assign_ref(n, prec);
        assert!(same_pair(&(z, o), &r));
    }
    if let Ok(n) = i64::try_from(n) {
        let mut z = x.clone();
        let o = z.pow_s_prec_round_assign(n, prec, rm);
        assert!(same_pair(&(z, o), &r));
        assert!(same_pair(&x.clone().pow_s_prec_round(n, prec, rm), &r));
        assert!(same_pair(&x.pow_s_prec_round_ref(n, prec, rm), &r));
        if rm == Nearest {
            assert!(same_pair(&x.clone().pow_s_prec(n, prec), &r));
            assert!(same_pair(&x.pow_s_prec_ref(n, prec), &r));
            let mut z = x.clone();
            let o = z.pow_s_prec_assign(n, prec);
            assert!(same_pair(&(z, o), &r));
        }
    }
    if let Ok(n) = u64::try_from(n) {
        let mut z = x.clone();
        let o = z.pow_u_prec_round_assign(n, prec, rm);
        assert!(same_pair(&(z, o), &r));
        assert!(same_pair(&x.clone().pow_u_prec_round(n, prec, rm), &r));
        assert!(same_pair(&x.pow_u_prec_round_ref(n, prec, rm), &r));
        if rm == Nearest {
            assert!(same_pair(&x.clone().pow_u_prec(n, prec), &r));
            assert!(same_pair(&x.pow_u_prec_ref(n, prec), &r));
            let mut z = x.clone();
            let o = z.pow_u_prec_assign(n, prec);
            assert!(same_pair(&(z, o), &r));
        }
    }
    r
}

fn check(
    x: &BigExponentFloat,
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
    out: &BigExponentFloat,
    o: Ordering,
) {
    let r = pow_integer_prec_round_all(x, n, prec, rm);
    assert!(
        same(&r.0, out),
        "{x:?} {n} {prec} {rm}: {:?} != {out:?}",
        r.0
    );
    assert_eq!(r.1, o);
    let g = pow_prec_round_float_kernels(x, n, prec, rm);
    assert!(same_pair(&g, &r), "{x:#x} {n} {prec} {rm}: {g:?} {r:?}");
}

fn check_pi(
    op: &str,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
    out: &BigExponentFloat,
    o: Ordering,
) {
    let n = match op {
        "pow_u" => Integer::from(3),
        "pow_s" => Integer::from(-3),
        _ => Integer::from(-10),
    };
    check(x, &n, prec, rm, out, o);
}

#[test]
fn test_pow_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    let i = Integer::from;
    check(&nan, &i(0), 10, Floor, &one, Equal);
    check(&nan, &i(3), 10, Floor, &nan, Equal);
    check(&inf, &i(3), 10, Floor, &inf, Equal);
    check(&inf, &i(-3), 10, Floor, &zero, Equal);
    check(&ninf, &i(3), 10, Floor, &ninf, Equal);
    check(&ninf, &i(4), 10, Floor, &inf, Equal);
    check(&ninf, &i(-3), 10, Floor, &zero, Equal);
    check(&zero, &i(3), 10, Floor, &zero, Equal);
    // Without a negative zero, 0^n for negative odd n could be either infinity.
    check(&zero, &i(-3), 10, Floor, &nan, Equal);
    check(&zero, &i(-1), 10, Floor, &nan, Equal);
    check(&zero, &i(-4), 10, Floor, &inf, Equal);
    check(
        &BigExponentFloat::from(3),
        &i(4),
        7,
        Exact,
        &BigExponentFloat::from(81),
        Equal,
    );
    check(
        &BigExponentFloat::from(-3),
        &i(3),
        5,
        Exact,
        &BigExponentFloat::from(-27),
        Equal,
    );
    check(
        &BigExponentFloat::from(-2),
        &i(-3),
        1,
        Exact,
        &BigExponentFloat::from(-0.125),
        Equal,
    );
    // 3^4 = 81 = 0b1010001 rounds to 0b1010000 at 3 bits.
    check(
        &BigExponentFloat::from(3),
        &i(4),
        3,
        Floor,
        &BigExponentFloat::from(80),
        Less,
    );
    // Powers of powers of 2 beyond `Float`'s exponent range: (2^N)^N = 2^(N^2).
    check(&p2(big()), &big(), 1, Exact, &p2(big() * big()), Equal);
    check(
        &-p2(big()),
        &(big() + Integer::ONE),
        1,
        Exact,
        &-p2(big() * (big() + Integer::ONE)),
        Equal,
    );
    check(&p2(-big()), &-big(), 1, Exact, &p2(big() * big()), Equal);
    // A huge exponent of another value: (1 + 2^-10)^(2^70) is far beyond `Float`'s range. Its
    // binary exponent is floor(2^70 * log2(1 + 2^-10)) + 1, and its floor and ceiling at 100 bits
    // are adjacent.
    let x = BigExponentFloat::from(1.0 + 1.0 / 1024.0);
    let n = Integer::from(1u128 << 70);
    let (floor, o) = pow_integer_prec_round_all(&x, &n, 100, Floor);
    assert_eq!(o, Less);
    let (ceiling, o) = pow_integer_prec_round_all(&x, &n, 100, Ceiling);
    assert_eq!(o, Greater);
    let log = Float::from(1.0 + 1.0 / 1024.0).log_base_2_prec(200).0 << 70u32;
    let exponent = Integer::rounding_from(&log, Floor).0 + Integer::ONE;
    assert_eq!(floor.get_exponent().unwrap(), &exponent);
    let ulp = p2(exponent - Integer::from(100));
    assert!(same(&ceiling, &(floor + ulp)));
    // The scaling of x by a power of 2 is undone exactly: (2^N * 3)^5 = 2^(5N) * 243.
    check(
        &(p2(big()) * BigExponentFloat::from(3)),
        &i(5),
        8,
        Exact,
        &(p2(big() * Integer::from(5)) * BigExponentFloat::from(243)),
        Equal,
    );
    check_pi("pow_u", &pi(), 5, Floor, &hex("0x1f.0"), Less);
    check_pi("pow_u", &pi(), 5, Ceiling, &hex("0x2.0E+1"), Greater);
    check_pi("pow_u", &pi(), 20, Nearest, &hex("0x1f.019c"), Greater);
    check_pi("pow_s", &pi(), 5, Floor, &hex("0x0.08"), Less);
    check_pi("pow_s", &pi(), 5, Ceiling, &hex("0x0.088"), Greater);
    check_pi("pow_s", &pi(), 20, Nearest, &hex("0x0.0841a3"), Greater);
    check_pi("pow_integer", &pi(), 5, Floor, &hex("0x0.0000b"), Less);
    check_pi(
        "pow_integer",
        &pi(),
        5,
        Ceiling,
        &hex("0x0.0000b8"),
        Greater,
    );
    check_pi(
        "pow_integer",
        &pi(),
        20,
        Nearest,
        &hex("0x0.0000b326e"),
        Greater,
    );
}

// Exponents too large for `Float`'s exponent range unless the base is very close to 1, as these
// are; the results agree with `Float`'s.
#[test]
fn test_pow_large_exponent_float_agreement() {
    for (x, n) in [
        (1.0 + f64::EPSILON, 1i64 << 40),
        (1.0 - f64::EPSILON / 2.0, 1 << 45),
        (1.0 + 1.0 / 1048576.0, 1 << 30),
        (-1.0 - f64::EPSILON, (1 << 41) + 1),
        (1.0 + f64::EPSILON, -(1 << 40) - 1),
        (-1.0 + f64::EPSILON / 2.0, -(1 << 44) - 1),
    ] {
        for prec in [1, 10, 100] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let (f, fo) = Float::from(x).pow_s_prec_round(n, prec, rm);
                let r = pow_integer_prec_round_all(
                    &BigExponentFloat::from(x),
                    &Integer::from(n),
                    prec,
                    rm,
                );
                assert!(
                    same(&r.0, &float_to_big_exponent_float(&f)),
                    "{x} {n} {prec} {rm}"
                );
                assert_eq!(r.1, fo);
            }
        }
    }
}

#[test]
#[should_panic]
fn pow_prec_round_fail_1() {
    BigExponentFloat::ONE.pow_u_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn pow_prec_round_fail_2() {
    BigExponentFloat::from(3).pow_u_prec_round(3, 3, Exact);
}

#[test]
#[should_panic]
fn pow_prec_round_fail_3() {
    BigExponentFloat::from(3).pow_s_prec_round(-1, 10, Exact);
}

fn pow_properties_helper(x: &BigExponentFloat, n: &Integer, prec: u64, rm: RoundingMode) {
    let r = pow_integer_prec_round_all(x, n, prec, rm);
    // Binary exponentiation agrees with the implementation using `Float`'s kernels.
    let g = pow_prec_round_float_kernels(x, n, prec, rm);
    assert!(same_pair(&g, &r), "{x:#x} {n} {prec} {rm}: {g:?} {r:?}");
    if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if let Ok(shift) = i64::try_from(n) {
        // Multiplying x by 2 multiplies the power by 2^n.
        let (s, o) = (x << 1u32).pow_integer_prec_round_ref_ref(n, prec, rm);
        assert!(same(&s, &(&r.0 << shift)));
        assert_eq!(o, r.1);
    }
    // Negating x negates odd powers and leaves even ones unchanged.
    let odd = n.odd();
    let (s, o) = (-x).pow_integer_prec_round_ref_ref(n, prec, if odd { -rm } else { rm });
    if odd {
        assert!(same(&s, &-&r.0));
        assert_eq!(o, r.1.reverse());
    } else {
        assert!(same_pair(&(s, o), &r));
    }
    if finite_nonzero(x) && *n != 0u32 && *n.unsigned_abs_ref() <= 64u32 {
        let n_abs = u64::try_from(n.unsigned_abs_ref()).unwrap();
        let power = x.pow(n_abs);
        if *n > 0u32 {
            assert_eq!(r.0.partial_cmp(&power), Some(r.1));
        } else {
            // x^n = 1/x^|n|, so r * x^|n| is close to 1, and multiplying by x^|n| preserves the
            // order if x^|n| is positive and reverses it otherwise.
            let product = &r.0 * &power;
            assert_eq!(
                if power.is_sign_positive() {
                    product.partial_cmp(&BigExponentFloat::ONE)
                } else {
                    BigExponentFloat::ONE.partial_cmp(&product)
                },
                Some(r.1)
            );
        }
    }
}

#[test]
fn pow_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, n, prec, rm)| {
            pow_properties_helper(&x, &Integer::from(n), prec, rm);
        },
    );
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, n, prec, rm)| {
            pow_properties_helper(&x, &Integer::from(n), prec, rm);
        },
    );
    big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, n, prec, rm)| {
            pow_properties_helper(&x, &n, prec, rm);
        },
    );
}

fn pow_all(x: &BigExponentFloat, n: u64) -> BigExponentFloat {
    let r = x.clone().pow(n);
    assert!(r.is_valid());
    assert!(same(&x.pow(n), &r));
    let mut z = x.clone();
    z.pow_assign(n);
    assert!(same(&z, &r));
    r
}

#[test]
fn test_pow() {
    let i = |x: i64| BigExponentFloat::from(x);
    assert!(same(
        &pow_all(&BigExponentFloat::NAN, 0),
        &BigExponentFloat::ONE
    ));
    assert!(same(
        &pow_all(&BigExponentFloat::NAN, 1),
        &BigExponentFloat::NAN
    ));
    assert!(same(
        &pow_all(&BigExponentFloat::NEGATIVE_INFINITY, 3),
        &BigExponentFloat::NEGATIVE_INFINITY
    ));
    assert!(same(
        &pow_all(&BigExponentFloat::NEGATIVE_INFINITY, 2),
        &BigExponentFloat::INFINITY
    ));
    assert!(same(
        &pow_all(&BigExponentFloat::ZERO, 2),
        &BigExponentFloat::ZERO
    ));
    assert!(same(&pow_all(&i(-3), 3), &i(-27)));
    assert!(same(
        &pow_all(&BigExponentFloat::from(1.5), 2),
        &BigExponentFloat::from(2.25)
    ));
    assert!(same(&pow_all(&p2(big()), 3), &p2(big() * Integer::from(3))));
}

#[test]
fn pow_properties() {
    big_exponent_float_unsigned_pair_gen_var_2().test_properties(|(x, n)| {
        let r = pow_all(&x, n);
        if n <= 16 {
            // The exact power is the product of n copies of x.
            let mut product = BigExponentFloat::ONE;
            for _ in 0..n {
                product *= &x;
            }
            if !x.is_nan() {
                assert!(same(&product, &r));
            }
        }
        if let Some(prec) = r.get_prec() {
            assert!(same_pair(
                &x.pow_u_prec_round_ref(n, prec, Exact),
                &(r.clone(), Equal)
            ));
        }
    });
}

#[test]
fn pow_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, n, prec, rm)| {
            let (f, fo) = x.pow_u_prec_round_ref(n, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = pow_integer_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &Integer::from(n),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
    float_signed_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(x, n, prec, rm)| {
            let (f, fo) = x.pow_s_prec_round_ref(n, prec, rm);
            // `Float` has a negative zero, so it gives an infinity for 0^n with negative odd n.
            if float_result_comparable(&f, fo) && !(x.is_zero() && n < 0 && n % 2 != 0) {
                let r = pow_integer_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &Integer::from(n),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
    float_integer_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, n, prec, rm)| {
            let (f, fo) = x.pow_integer_prec_round_ref_ref(&n, prec, rm);
            if float_result_comparable(&f, fo) && !(x.is_zero() && n < 0u32 && n.odd()) {
                let r = pow_integer_prec_round_all(&float_to_big_exponent_float(&x), &n, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
