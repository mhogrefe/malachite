// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::pow::{
    pow_prec_round_float_kernels, pow_prec_round_generic, pow_rational_prec_round_generic,
    rational_pow_prec_round_generic, rational_pow_rational_prec_round_generic,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::exhaustive::big_exponent_float_pow_exponent_is_moderate;
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18,
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18_float,
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19,
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19_float,
    big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21_float,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22_float,
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    big_exponent_float_unsigned_pair_gen_var_2,
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    float_integer_unsigned_rounding_mode_quadruple_gen_var_1,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_11,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11,
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_3,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
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
    x: &str,
    x_hex: &str,
    n: &Integer,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
    let x = &parse_big_exponent_float_hex(x_hex);
    assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
    let out = &parse_big_exponent_float_hex(out_hex);
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
    x: &str,
    x_hex: &str,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    o: Ordering,
) {
    let n = match op {
        "pow_u" => Integer::from(3),
        "pow_s" => Integer::from(-3),
        _ => Integer::from(-10),
    };
    check(x, x_hex, &n, prec, rm, out, out_hex, o);
}

#[test]
fn test_pow_prec_round() {
    let i = Integer::from;
    check("NaN", "NaN", &i(0), 10, Floor, "1.0", "0x1.0", Equal);
    check("NaN", "NaN", &i(3), 10, Floor, "NaN", "NaN", Equal);
    check(
        "Infinity",
        "Infinity",
        &i(3),
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(
        "Infinity",
        "Infinity",
        &i(-3),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        &i(3),
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        &i(4),
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        &i(-3),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    check("0.0", "0x0.0", &i(3), 10, Floor, "0.0", "0x0.0", Equal);
    // Without a negative zero, 0^n for negative odd n could be either infinity.
    check("0.0", "0x0.0", &i(-3), 10, Floor, "NaN", "NaN", Equal);
    check("0.0", "0x0.0", &i(-1), 10, Floor, "NaN", "NaN", Equal);
    check(
        "0.0",
        "0x0.0",
        &i(-4),
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    check("3.0", "0x3.0", &i(4), 7, Exact, "81.00", "0x51.0", Equal);
    check("-3.0", "-0x3.0", &i(3), 5, Exact, "-27.0", "-0x1b.0", Equal);
    check("-2.0", "-0x2.0", &i(-3), 1, Exact, "-0.12", "-0x0.2", Equal);
    // 3^4 = 81 = 0b1010001 rounds to 0b1010000 at 3 bits.
    check("3.0", "0x3.0", &i(4), 3, Floor, "80.0", "0x5.0E+1", Less);
    // Powers of powers of 2 beyond `Float`'s exponent range: (2^N)^N = 2^(N^2).
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        &big(),
        1,
        Exact,
        "7.9e3010299956639811952137388947244930267681",
        "0x1.0E+2500000000000000000000000000000000000000",
        Equal,
    );
    check(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        &(big() + Integer::ONE),
        1,
        Exact,
        "-1.9e3010299956639811952167491946811328387203",
        "-0x1.0E+2500000000000000000025000000000000000000",
        Equal,
    );
    check(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        &-big(),
        1,
        Exact,
        "7.9e3010299956639811952137388947244930267681",
        "0x1.0E+2500000000000000000000000000000000000000",
        Equal,
    );
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
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        &i(5),
        8,
        Exact,
        "1.799e150514997831990597609",
        "0xf.3E+125000000000000000001",
        Equal,
    );
    check_pi(
        "pow_u",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "31.0",
        "0x1f.0",
        Less,
    );
    check_pi(
        "pow_u",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "32.0",
        "0x2.0E+1",
        Greater,
    );
    check_pi(
        "pow_u",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "31.00629",
        "0x1f.019c",
        Greater,
    );
    check_pi(
        "pow_s",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "0.031",
        "0x0.08",
        Less,
    );
    check_pi(
        "pow_s",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "0.0332",
        "0x0.088",
        Greater,
    );
    check_pi(
        "pow_s",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "0.032251537",
        "0x0.0841a3",
        Greater,
    );
    check_pi(
        "pow_integer",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "0.0000105",
        "0x0.0000b",
        Less,
    );
    check_pi(
        "pow_integer",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "0.0000110",
        "0x0.0000b8",
        Greater,
    );
    check_pi(
        "pow_integer",
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "0.00001067828",
        "0x0.0000b326e",
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
    let test = |x: &str, x_hex: &str, n: u64, out: &str, out_hex: &str| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        assert!(same(&pow_all(x, n), out));
    };
    test("NaN", "NaN", 0, "1.0", "0x1.0");
    test("NaN", "NaN", 1, "NaN", "NaN");
    test("-Infinity", "-Infinity", 3, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 2, "Infinity", "Infinity");
    test("0.0", "0x0.0", 2, "0.0", "0x0.0");
    test("-3.0", "-0x3.0", 3, "-27.0", "-0x1b.0");
    test("1.5", "0x1.8", 2, "2.25", "0x2.4");
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        3,
        "1.3e90308998699194358564",
        "0x1.0E+75000000000000000000",
    );
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

fn pow_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().pow_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().pow_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.pow_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.pow_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.pow_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.pow_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().pow_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().pow_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.pow_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.pow_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.pow_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.pow_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_general_pow_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 y_hex: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = pow_prec_round_all(x, y, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("NaN", "NaN", "0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    check("1.0", "0x1", "NaN", "NaN", 10, Exact, "1.0", "0x1.0", Equal);
    check("NaN", "NaN", "1.0", "0x1", 10, Exact, "NaN", "NaN", Equal);
    check(
        "2.0", "0x2", "Infinity", "Infinity", 10, Exact, "Infinity", "Infinity", Equal,
    );
    check(
        "0.50", "0x0.8", "Infinity", "Infinity", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check(
        "2.0",
        "0x2",
        "-Infinity",
        "-Infinity",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "-1.0", "-0x1", "Infinity", "Infinity", 10, Exact, "1.0", "0x1.0", Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "3.0",
        "0x3",
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2",
        10,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "-3.0",
        "-0x3",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    check("0.0", "0x0", "-3.0", "-0x3", 10, Exact, "NaN", "NaN", Equal);
    check(
        "0.0", "0x0", "-2.0", "-0x2", 10, Exact, "Infinity", "Infinity", Equal,
    );
    check(
        "0.0", "0x0", "0.50", "0x0.8", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check(
        "-2.0", "-0x2", "0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal,
    );
    check(
        "-1.0", "-0x1", "3.0", "0x3", 10, Exact, "-1.0", "-0x1.0", Equal,
    );
    check(
        "9.00", "0x9", "0.50", "0x0.8", 10, Exact, "3.0", "0x3.0", Equal,
    );
    check(
        "4.0", "0x4", "-1.5", "-0x1.8", 10, Exact, "0.12", "0x0.2", Equal,
    );
    check(
        "81.00", "0x51", "0.75", "0x0.c", 10, Exact, "27.0", "0x1b.0", Equal,
    );
    check(
        "-2.0", "-0x2", "3.0", "0x3", 10, Exact, "-8.0", "-0x8.0", Equal,
    );
    check(
        "2.0",
        "0x2",
        "2.6e120",
        "0x1.0E+100",
        10,
        Exact,
        "4.0e7773346696038180627022555141287253630435120583191353102738697952724854402489468094777\
        61032229707657848729620784479815843",
        "0x1.0E+6455624695217271474139797930007529685824264482073058782076648391351619055042102986\
        57411338320034457858975792993186873344",
        Equal,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        10,
        Exact,
        "4.9e15051499783199059760",
        "0x1.0E+12500000000000000000",
        Equal,
    );
    check("3.0", "0x3", "2.0", "0x2", 2, Floor, "8.0", "0x8.0", Less);
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "22.44",
        "0x16.7",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "22.469",
        "0x16.78",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        100,
        Nearest,
        "22.45915771836103937687044715549",
        "0x16.758b5c38110f8164776b6948",
        Less,
    );
    check(
        "3.0", "0x3", "0.50", "0x0.8", 1, Nearest, "2.0", "0x2.0", Greater,
    );
    check(
        "3.0", "0x3", "0.50", "0x0.8", 10, Floor, "1.730", "0x1.bb", Less,
    );
    check(
        "3.0", "0x3", "0.50", "0x0.8", 10, Ceiling, "1.7324", "0x1.bb8", Greater,
    );
    check(
        "3.0",
        "0x3",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
    check(
        "0.50", "0x0.8", "-3.5", "-0x3.8", 1, Nearest, "8.0", "0x8.0", Less,
    );
    check(
        "0.50", "0x0.8", "-3.5", "-0x3.8", 10, Floor, "11.31", "0xb.5", Less,
    );
    check(
        "0.50", "0x0.8", "-3.5", "-0x3.8", 10, Ceiling, "11.328", "0xb.54", Greater,
    );
    check(
        "0.50",
        "0x0.8",
        "-3.5",
        "-0x3.8",
        100,
        Nearest,
        "11.313708498984760390413509793673",
        "0xb.504f333f9de6484597d89b37",
        Less,
    );
    check(
        "-3.0",
        "-0x3",
        "5.0",
        "0x5",
        1,
        Nearest,
        "-2.6e2",
        "-0x1.0E+2",
        Less,
    );
    check(
        "-3.0", "-0x3", "5.0", "0x5", 10, Floor, "-243.0", "-0xf3.0", Equal,
    );
    check(
        "-3.0", "-0x3", "5.0", "0x5", 10, Ceiling, "-243.0", "-0xf3.0", Equal,
    );
    check(
        "-3.0", "-0x3", "5.0", "0x5", 100, Nearest, "-243.0", "-0xf3.0", Equal,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        1,
        Nearest,
        "4.3e1232046301832521188248816884509488269094799407862261951888989957685502942707007817667\
        416108728889355235202946801893073832",
        "0x4.0E+1023192306064881829293264426990178577948889515462474161657524648219603931872948393\
        215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        10,
        Floor,
        "3.488e12320463018325211882488168845094882690947994078622619518889899576855029427070078176\
        67416108728889355235202946801893073832",
        "0x3.46E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Less,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        10,
        Ceiling,
        "3.4921e1232046301832521188248816884509488269094799407862261951888989957685502942707007817\
        667416108728889355235202946801893073832",
        "0x3.47E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        100,
        Nearest,
        "3.4918642785658923420597798454191e1232046301832521188248816884509488269094799407862261951\
        888989957685502942707007817667416108728889355235202946801893073832",
        "0x3.46f0082fa3a39df5f323f01dcE+1023192306064881829293264426990178577948889515462474161657\
        524648219603931872948393215416341841720759856647377005463516754",
        Less,
    );
    check(
        "-3.0",
        "-0x3",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        1,
        Nearest,
        "-8.5e123204630183252118824881688450948826909479940786226195188898995768550294270700781766\
        7416108728889355235202946801893073832",
        "-0x8.0E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "-3.0",
        "-0x3",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        10,
        Floor,
        "-1.049e1232046301832521188248816884509488269094799407862261951888989957685502942707007817\
        667416108728889355235202946801893073833",
        "-0x9.d8E+10231923060648818292932644269901785779488895154624741616575246482196039318729483\
        93215416341841720759856647377005463516754",
        Less,
    );
    check(
        "-3.0",
        "-0x3",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        10,
        Ceiling,
        "-1.0472e123204630183252118824881688450948826909479940786226195188898995768550294270700781\
        7667416108728889355235202946801893073833",
        "-0x9.d4E+10231923060648818292932644269901785779488895154624741616575246482196039318729483\
        93215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "-3.0",
        "-0x3",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        100,
        Nearest,
        "-1.0475592835697677026179339536254e123204630183252118824881688450948826909479940786226195\
        1888989957685502942707007817667416108728889355235202946801893073833",
        "-0x9.d4d0188eeaead9e1d96bd059E+1023192306064881829293264426990178577948889515462474161657\
        524648219603931872948393215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "9.7e15051499783199059760",
        "0x2.0E+12500000000000000000",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "8.416e15051499783199059760",
        "0x1.bbE+12500000000000000000",
        Less,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "8.4256e15051499783199059760",
        "0x1.bb8E+12500000000000000000",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "8.423747947952630271019807450647e15051499783199059760",
        "0x1.bb67ae8584caa73b25742d708E+12500000000000000000",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        1,
        Nearest,
        "2.9e45154499349597179281",
        "0x4.0E+37499999999999999999",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        10,
        Floor,
        "2.213e45154499349597179281",
        "0x3.14E+37499999999999999999",
        Less,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        10,
        Ceiling,
        "2.2159e45154499349597179281",
        "0x3.15E+37499999999999999999",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        100,
        Nearest,
        "2.213871077534927462789491423339e45154499349597179281",
        "0x3.14468b980884b78597eafb728E+37499999999999999999",
        Less,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "2.3e122243029165053",
        "0x4.0E+101520638246881",
        Greater,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        10,
        Floor,
        "1.8203e122243029165053",
        "0x3.23E+101520638246881",
        Less,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "1.823e122243029165053",
        "0x3.24E+101520638246881",
        Greater,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "1.820793516006083507583278508504e122243029165053",
        "0x3.233644bc9d80c2259689ccda8E+101520638246881",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        1,
        Nearest,
        "9.2e-576804203917528170433550",
        "0x2.0E-479025522560694005262339",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Floor,
        "8.7656e-576804203917528170433550",
        "0x1.e78E-479025522560694005262339",
        Less,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "8.77e-576804203917528170433550",
        "0x1.e8E-479025522560694005262339",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "8.767243622672027935008315333632e-576804203917528170433550",
        "0x1.e797e32cf5b62e0d7350a4b84E-479025522560694005262339",
        Less,
    );
}

#[test]
#[should_panic]
fn general_pow_prec_round_fail_1() {
    BigExponentFloat::from(3u32).pow_prec_round(BigExponentFloat::from(0.5), 0, Floor);
}

#[test]
#[should_panic]
fn general_pow_prec_round_fail_2() {
    BigExponentFloat::from(3u32).pow_prec_round(BigExponentFloat::from(0.5), 10, Exact);
}

#[test]
fn general_pow_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18()
        .test_properties(|(x, y, prec, rm)| {
            let r = pow_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(
                &pow_prec_round_generic(&x, &y, false, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // An integer exponent with a small absolute value gives `pow_integer`'s result.
            if let Ok(n) = Integer::try_from(&y)
                && n.unsigned_abs_ref().significant_bits() <= 64
            {
                assert!(same_pair(
                    &x.pow_integer_prec_round_ref_ref(&n, prec, rm),
                    &r
                ));
            }
        });
}

#[test]
fn general_pow_prec_round_float_agreement_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18_float()
        .test_properties(|((xf, yf, prec, rm), (x, y, _, _))| {
            let (f, fo) = xf.pow_prec_round_ref_ref(&yf, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = pow_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}

fn powr_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().powr_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().powr_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.powr_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.powr_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.powr_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.powr_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().powr_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().powr_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.powr_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.powr_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.powr_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.powr_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_powr_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 y_hex: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = powr_prec_round_all(x, y, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("1.0", "0x1", "NaN", "NaN", 10, Exact, "1.0", "0x1.0", Equal);
    check(
        "2.0", "0x2", "Infinity", "Infinity", 10, Exact, "Infinity", "Infinity", Equal,
    );
    check(
        "0.50", "0x0.8", "Infinity", "Infinity", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check(
        "2.0",
        "0x2",
        "-Infinity",
        "-Infinity",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    check(
        "0.0", "0x0", "-3.0", "-0x3", 10, Exact, "Infinity", "Infinity", Equal,
    );
    check(
        "0.0", "0x0", "-2.0", "-0x2", 10, Exact, "Infinity", "Infinity", Equal,
    );
    check(
        "0.0", "0x0", "0.50", "0x0.8", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check("-2.0", "-0x2", "2.0", "0x2", 10, Exact, "NaN", "NaN", Equal);
    check("0.0", "0x0", "0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    check(
        "Infinity", "Infinity", "0.0", "0x0", 10, Exact, "NaN", "NaN", Equal,
    );
    check(
        "1.0", "0x1", "Infinity", "Infinity", 10, Exact, "NaN", "NaN", Equal,
    );
    check("1.0", "0x1", "NaN", "NaN", 10, Exact, "1.0", "0x1.0", Equal);
    check(
        "9.00", "0x9", "0.50", "0x0.8", 10, Exact, "3.0", "0x3.0", Equal,
    );
    check(
        "4.0", "0x4", "-1.5", "-0x1.8", 10, Exact, "0.12", "0x0.2", Equal,
    );
    check(
        "81.00", "0x51", "0.75", "0x0.c", 10, Exact, "27.0", "0x1b.0", Equal,
    );
    check(
        "2.0",
        "0x2",
        "2.6e120",
        "0x1.0E+100",
        10,
        Exact,
        "4.0e7773346696038180627022555141287253630435120583191353102738697952724854402489468094777\
        61032229707657848729620784479815843",
        "0x1.0E+6455624695217271474139797930007529685824264482073058782076648391351619055042102986\
        57411338320034457858975792993186873344",
        Equal,
    );
    check(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        10,
        Exact,
        "4.9e15051499783199059760",
        "0x1.0E+12500000000000000000",
        Equal,
    );
    check("3.0", "0x3", "2.0", "0x2", 2, Floor, "8.0", "0x8.0", Less);
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "22.44",
        "0x16.7",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "22.469",
        "0x16.78",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        100,
        Nearest,
        "22.45915771836103937687044715549",
        "0x16.758b5c38110f8164776b6948",
        Less,
    );
    check(
        "3.0", "0x3", "0.50", "0x0.8", 1, Nearest, "2.0", "0x2.0", Greater,
    );
    check(
        "3.0", "0x3", "0.50", "0x0.8", 10, Floor, "1.730", "0x1.bb", Less,
    );
    check(
        "3.0", "0x3", "0.50", "0x0.8", 10, Ceiling, "1.7324", "0x1.bb8", Greater,
    );
    check(
        "3.0",
        "0x3",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
    check(
        "0.50", "0x0.8", "-3.5", "-0x3.8", 1, Nearest, "8.0", "0x8.0", Less,
    );
    check(
        "0.50", "0x0.8", "-3.5", "-0x3.8", 10, Floor, "11.31", "0xb.5", Less,
    );
    check(
        "0.50", "0x0.8", "-3.5", "-0x3.8", 10, Ceiling, "11.328", "0xb.54", Greater,
    );
    check(
        "0.50",
        "0x0.8",
        "-3.5",
        "-0x3.8",
        100,
        Nearest,
        "11.313708498984760390413509793673",
        "0xb.504f333f9de6484597d89b37",
        Less,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        1,
        Nearest,
        "4.3e1232046301832521188248816884509488269094799407862261951888989957685502942707007817667\
        416108728889355235202946801893073832",
        "0x4.0E+1023192306064881829293264426990178577948889515462474161657524648219603931872948393\
        215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        10,
        Floor,
        "3.488e12320463018325211882488168845094882690947994078622619518889899576855029427070078176\
        67416108728889355235202946801893073832",
        "0x3.46E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Less,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        10,
        Ceiling,
        "3.4921e1232046301832521188248816884509488269094799407862261951888989957685502942707007817\
        667416108728889355235202946801893073832",
        "0x3.47E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "2.6e120",
        "0x1.0E+100",
        100,
        Nearest,
        "3.4918642785658923420597798454191e1232046301832521188248816884509488269094799407862261951\
        888989957685502942707007817667416108728889355235202946801893073832",
        "0x3.46f0082fa3a39df5f323f01dcE+1023192306064881829293264426990178577948889515462474161657\
        524648219603931872948393215416341841720759856647377005463516754",
        Less,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        1,
        Nearest,
        "9.7e15051499783199059760",
        "0x2.0E+12500000000000000000",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        10,
        Floor,
        "8.416e15051499783199059760",
        "0x1.bbE+12500000000000000000",
        Less,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        10,
        Ceiling,
        "8.4256e15051499783199059760",
        "0x1.bb8E+12500000000000000000",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "8.423747947952630271019807450647e15051499783199059760",
        "0x1.bb67ae8584caa73b25742d708E+12500000000000000000",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        1,
        Nearest,
        "2.9e45154499349597179281",
        "0x4.0E+37499999999999999999",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        10,
        Floor,
        "2.213e45154499349597179281",
        "0x3.14E+37499999999999999999",
        Less,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        10,
        Ceiling,
        "2.2159e45154499349597179281",
        "0x3.15E+37499999999999999999",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-1.5",
        "-0x1.8",
        100,
        Nearest,
        "2.213871077534927462789491423339e45154499349597179281",
        "0x3.14468b980884b78597eafb728E+37499999999999999999",
        Less,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "2.3e122243029165053",
        "0x4.0E+101520638246881",
        Greater,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        10,
        Floor,
        "1.8203e122243029165053",
        "0x3.23E+101520638246881",
        Less,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "1.823e122243029165053",
        "0x3.24E+101520638246881",
        Greater,
    );
    check(
        "1.0000000002",
        "0x1.00000001",
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "1.820793516006083507583278508504e122243029165053",
        "0x3.233644bc9d80c2259689ccda8E+101520638246881",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        1,
        Nearest,
        "9.2e-576804203917528170433550",
        "0x2.0E-479025522560694005262339",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Floor,
        "8.7656e-576804203917528170433550",
        "0x1.e78E-479025522560694005262339",
        Less,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "8.77e-576804203917528170433550",
        "0x1.e8E-479025522560694005262339",
        Greater,
    );
    check(
        "3.0",
        "0x3",
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "8.767243622672027935008315333632e-576804203917528170433550",
        "0x1.e797e32cf5b62e0d7350a4b84E-479025522560694005262339",
        Less,
    );
}

#[test]
#[should_panic]
fn powr_prec_round_fail_1() {
    BigExponentFloat::from(3u32).powr_prec_round(BigExponentFloat::from(0.5), 0, Floor);
}

#[test]
#[should_panic]
fn powr_prec_round_fail_2() {
    BigExponentFloat::from(3u32).powr_prec_round(BigExponentFloat::from(0.5), 10, Exact);
}

#[test]
fn powr_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19()
        .test_properties(|(x, y, prec, rm)| {
            let r = powr_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(
                &pow_prec_round_generic(&x, &y, true, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // For a finite positive x and a finite y, `powr` is `pow`.
            if x.is_finite() && x > 0u32 && y.is_finite() {
                assert!(same_pair(&x.pow_prec_round_ref_ref(&y, prec, rm), &r));
            }
        });
}

#[test]
fn powr_prec_round_float_agreement_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19_float()
        .test_properties(|((xf, yf, prec, rm), (x, y, _, _))| {
            let (f, fo) = xf.powr_prec_round_ref_ref(&yf, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = powr_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}

fn pow_rational_prec_round_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().pow_rational_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().pow_rational_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.pow_rational_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.pow_rational_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.pow_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.pow_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().pow_rational_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().pow_rational_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.pow_rational_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.pow_rational_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.pow_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.pow_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_pow_rational_prec_round() {
    let check = |x: &str,
                 x_hex: &str,
                 y: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = pow_rational_prec_round_all(x, &Rational::from_str(y).unwrap(), prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("NaN", "NaN", "1/3", 10, Exact, "NaN", "NaN", Equal);
    check("0.0", "0x0", "-3", 10, Exact, "NaN", "NaN", Equal);
    check(
        "0.0", "0x0", "-1/3", 10, Exact, "Infinity", "Infinity", Equal,
    );
    check(
        "-Infinity",
        "-Infinity",
        "3",
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    check("-2.0", "-0x2", "1/3", 10, Exact, "NaN", "NaN", Equal);
    check("8.0", "0x8", "1/3", 10, Exact, "2.0", "0x2.0", Equal);
    check("16.0", "0x10", "-3/4", 10, Exact, "0.12", "0x0.2", Equal);
    check("27.0", "0x1b", "2/3", 10, Exact, "9.00", "0x9.0", Equal);
    check("-8.0", "-0x8", "3", 10, Exact, "-5.1e2", "-0x2.0E+2", Equal);
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1/3",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1/3",
        10,
        Floor,
        "1.4629",
        "0x1.768",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1/3",
        10,
        Ceiling,
        "1.465",
        "0x1.77",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "1/3",
        100,
        Nearest,
        "1.464591887561523243989373053169",
        "0x1.76ef7e73104b7750833131288",
        Greater,
    );
    check("2.0", "0x2", "1/3", 1, Nearest, "1.0", "0x1.0", Less);
    check("2.0", "0x2", "1/3", 10, Floor, "1.2598", "0x1.428", Less);
    check("2.0", "0x2", "1/3", 10, Ceiling, "1.262", "0x1.43", Greater);
    check(
        "2.0",
        "0x2",
        "1/3",
        100,
        Nearest,
        "1.259921049894873164767210607278",
        "0x1.428a2f98d728ae223ddab715c",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "1/3",
        1,
        Nearest,
        "9.8e10034333188799373173",
        "0x4.0E+8333333333333333333",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "1/3",
        10,
        Floor,
        "8.916e10034333188799373173",
        "0x3.a2E+8333333333333333333",
        Less,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "1/3",
        10,
        Ceiling,
        "8.9255e10034333188799373173",
        "0x3.a3E+8333333333333333333",
        Greater,
    );
    check(
        "7.1e30102999566398119521",
        "0x3.0E+25000000000000000000",
        "1/3",
        100,
        Nearest,
        "8.9194260478930974353679998050217e10034333188799373173",
        "0x3.a25da15e344fde7b575c26984E+8333333333333333333",
        Less,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-22/7",
        1,
        Nearest,
        "2.8e94609427208679804208",
        "0x2.0E+78571428571428571427",
        Less,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-22/7",
        10,
        Floor,
        "3.408e94609427208679804208",
        "0x2.78E+78571428571428571427",
        Less,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-22/7",
        10,
        Ceiling,
        "3.4135e94609427208679804208",
        "0x2.79E+78571428571428571427",
        Greater,
    );
    check(
        "1.3e-30102999566398119521",
        "0x3.0E-25000000000000000000",
        "-22/7",
        100,
        Nearest,
        "3.4095739390130045931356823492171e94609427208679804208",
        "0x2.7845dc9bf32ef98a02552b964E+78571428571428571427",
        Greater,
    );
    check(
        "-3.0", "-0x3", "-3", 1, Nearest, "-0.031", "-0x0.08", Greater,
    );
    check(
        "-3.0",
        "-0x3",
        "-3",
        10,
        Floor,
        "-0.037048",
        "-0x0.097c",
        Less,
    );
    check(
        "-3.0",
        "-0x3",
        "-3",
        10,
        Ceiling,
        "-0.03699",
        "-0x0.0978",
        Greater,
    );
    check(
        "-3.0",
        "-0x3",
        "-3",
        100,
        Nearest,
        "-0.037037037037037037037037037037046",
        "-0x0.097b425ed097b425ed097b425f",
        Less,
    );
}

#[test]
#[should_panic]
fn pow_rational_prec_round_fail_1() {
    BigExponentFloat::from(3u32).pow_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn pow_rational_prec_round_fail_2() {
    BigExponentFloat::from(3u32).pow_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        10,
        Exact,
    );
}

#[test]
fn pow_rational_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21().test_properties(
        |(x, y, prec, rm)| {
            let r = pow_rational_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(
                &pow_rational_prec_round_generic(&x, &y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // A dyadic exponent gives the power with a `BigExponentFloat` exponent.
            if let Ok(yb) = BigExponentFloat::try_from(&y) {
                assert!(same_pair(&x.pow_prec_round_ref_ref(&yb, prec, rm), &r));
            }
        },
    );
}

#[test]
fn pow_rational_prec_round_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21_float()
        .test_properties(|((xf, y, prec, rm), (x, _, _, _))| {
            let (f, fo) = xf.pow_rational_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = pow_rational_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}

fn rational_pow_prec_round_all(
    x: &Rational,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::rational_pow_prec_round(x.clone(), y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::rational_pow_prec_round_val_ref(x.clone(), y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::rational_pow_prec_round_ref_val(x, y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::rational_pow_prec_round_ref_ref(x, y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::rational_pow_prec(x.clone(), y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_pow_prec_val_ref(x.clone(), y, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_pow_prec_ref_val(x, y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_pow_prec_ref_ref(x, y, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_rational_pow_prec_round() {
    let check = |x: &str,
                 y: &str,
                 y_hex: &str,
                 prec: u64,
                 rm: RoundingMode,
                 out: &str,
                 out_hex: &str,
                 o| {
        assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
        let y = &parse_big_exponent_float_hex(y_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = rational_pow_prec_round_all(&Rational::from_str(x).unwrap(), y, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("1/3", "0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    check("1/3", "NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    check(
        "1/3", "Infinity", "Infinity", 10, Exact, "0.0", "0x0.0", Equal,
    );
    check("-1/3", "0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    check("1/9", "-0.50", "-0x0.8", 10, Exact, "3.0", "0x3.0", Equal);
    check("1/3", "-5.0", "-0x5", 10, Exact, "243.0", "0xf3.0", Equal);
    check("0", "-3.0", "-0x3", 10, Exact, "NaN", "NaN", Equal);
    check("1", "NaN", "NaN", 10, Exact, "1.0", "0x1.0", Equal);
    check(
        "1/3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "0.062",
        "0x0.1",
        Greater,
    );
    check(
        "1/3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "0.05042",
        "0x0.0ce8",
        Less,
    );
    check(
        "1/3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "0.050476",
        "0x0.0cec",
        Greater,
    );
    check(
        "1/3",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        100,
        Nearest,
        "0.050471935956387496808833789831301",
        "0x0.0cebba924c69e44ef85cb15df9",
        Greater,
    );
    check("4/9", "0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    check("4/9", "0.50", "0x0.8", 10, Floor, "0.6660", "0x0.aa8", Less);
    check(
        "4/9", "0.50", "0x0.8", 10, Ceiling, "0.66699", "0x0.aac", Greater,
    );
    check(
        "4/9",
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.66666666666666666666666666666693",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab",
        Greater,
    );
    check(
        "-1/3", "3.0", "0x3", 1, Nearest, "-0.031", "-0x0.08", Greater,
    );
    check(
        "-1/3",
        "3.0",
        "0x3",
        10,
        Floor,
        "-0.037048",
        "-0x0.097c",
        Less,
    );
    check(
        "-1/3",
        "3.0",
        "0x3",
        10,
        Ceiling,
        "-0.03699",
        "-0x0.0978",
        Greater,
    );
    check(
        "-1/3",
        "3.0",
        "0x3",
        100,
        Nearest,
        "-0.037037037037037037037037037037046",
        "-0x0.097b425ed097b425ed097b425f",
        Less,
    );
    check(
        "22/7",
        "2.6e120",
        "0x1.0E+100",
        1,
        Nearest,
        "2.1e1284216493095942956275188731592890422016977952660366949176348798716083707556869209508\
        278193502798719635083116538341438399",
        "0x2.0E+1066518712083283826589841711525311091405667569223469427433679994606291376724847329\
        591153369425170774523637202909639293167",
        Greater,
    );
    check(
        "22/7",
        "2.6e120",
        "0x1.0E+100",
        10,
        Floor,
        "1.642e12842164930959429562751887315928904220169779526603669491763487987160837075568692095\
        08278193502798719635083116538341438399",
        "0x1.99E+106651871208328382658984171152531109140566756922346942743367999460629137672484732\
        9591153369425170774523637202909639293167",
        Less,
    );
    check(
        "22/7",
        "2.6e120",
        "0x1.0E+100",
        10,
        Ceiling,
        "1.6439e1284216493095942956275188731592890422016977952660366949176348798716083707556869209\
        508278193502798719635083116538341438399",
        "0x1.998E+10665187120832838265898417115253110914056675692234694274336799946062913767248473\
        29591153369425170774523637202909639293167",
        Greater,
    );
    check(
        "22/7",
        "2.6e120",
        "0x1.0E+100",
        100,
        Nearest,
        "1.6428011973678294077958218379872e1284216493095942956275188731592890422016977952660366949\
        176348798716083707556869209508278193502798719635083116538341438399",
        "0x1.99393e37e3091279a5e7a569eE+1066518712083283826589841711525311091405667569223469427433\
        679994606291376724847329591153369425170774523637202909639293167",
        Less,
    );
    check(
        "1/3",
        "-2.6e120",
        "-0x1.0E+100",
        1,
        Nearest,
        "4.3e1232046301832521188248816884509488269094799407862261951888989957685502942707007817667\
        416108728889355235202946801893073832",
        "0x4.0E+1023192306064881829293264426990178577948889515462474161657524648219603931872948393\
        215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "1/3",
        "-2.6e120",
        "-0x1.0E+100",
        10,
        Floor,
        "3.488e12320463018325211882488168845094882690947994078622619518889899576855029427070078176\
        67416108728889355235202946801893073832",
        "0x3.46E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Less,
    );
    check(
        "1/3",
        "-2.6e120",
        "-0x1.0E+100",
        10,
        Ceiling,
        "3.4921e1232046301832521188248816884509488269094799407862261951888989957685502942707007817\
        667416108728889355235202946801893073832",
        "0x3.47E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Greater,
    );
    check(
        "1/3",
        "-2.6e120",
        "-0x1.0E+100",
        100,
        Nearest,
        "3.4918642785658923420597798454191e1232046301832521188248816884509488269094799407862261951\
        888989957685502942707007817667416108728889355235202946801893073832",
        "0x3.46f0082fa3a39df5f323f01dcE+1023192306064881829293264426990178577948889515462474161657\
        524648219603931872948393215416341841720759856647377005463516754",
        Less,
    );
    check(
        "-22/7",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        1,
        Nearest,
        "-4.1e128421649309594295627518873159289042201697795266036694917634879871608370755686920950\
        8278193502798719635083116538341438399",
        "-0x4.0E+106651871208328382658984171152531109140566756922346942743367999460629137672484732\
        9591153369425170774523637202909639293167",
        Greater,
    );
    check(
        "-22/7",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        10,
        Floor,
        "-5.171e1284216493095942956275188731592890422016977952660366949176348798716083707556869209\
        508278193502798719635083116538341438399",
        "-0x5.08E+10665187120832838265898417115253110914056675692234694274336799946062913767248473\
        29591153369425170774523637202909639293167",
        Less,
    );
    check(
        "-22/7",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        10,
        Ceiling,
        "-5.1626e128421649309594295627518873159289042201697795266036694917634879871608370755686920\
        9508278193502798719635083116538341438399",
        "-0x5.06E+10665187120832838265898417115253110914056675692234694274336799946062913767248473\
        29591153369425170774523637202909639293167",
        Greater,
    );
    check(
        "-22/7",
        "25822498780869085896559191720030118743297057928292235128306593565406476220168411946296453\
        53280137831435903171972747493377.0",
        "0x100000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000001",
        100,
        Nearest,
        "-5.163089477441749567358297205104e1284216493095942956275188731592890422016977952660366949\
        176348798716083707556869209508278193502798719635083116538341438399",
        "-0x5.06219ef8c98a3a109bb37596E+1066518712083283826589841711525311091405667569223469427433\
        679994606291376724847329591153369425170774523637202909639293167",
        Less,
    );
}

#[test]
#[should_panic]
fn rational_pow_prec_round_fail_1() {
    BigExponentFloat::rational_pow_prec_round(
        Rational::from_unsigneds(1u8, 3),
        BigExponentFloat::from(0.5),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn rational_pow_prec_round_fail_2() {
    BigExponentFloat::rational_pow_prec_round(
        Rational::from_unsigneds(1u8, 3),
        BigExponentFloat::from(0.5),
        10,
        Exact,
    );
}

#[test]
fn rational_pow_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22().test_properties(
        |(y, x, prec, rm)| {
            let r = rational_pow_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(
                &rational_pow_prec_round_generic(&x, &y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // A dyadic base gives the power with a `BigExponentFloat` base.
            if let Ok(xb) = BigExponentFloat::try_from(&x) {
                assert!(same_pair(&xb.pow_prec_round_ref_ref(&y, prec, rm), &r));
            }
        },
    );
}

#[test]
fn rational_pow_prec_round_float_agreement_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22_float()
        .test_properties(|((yf, x, prec, rm), (y, _, _, _))| {
            let (f, fo) = Float::rational_pow_prec_round_ref_ref(&x, &yf, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = rational_pow_prec_round_all(&x, &y, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        });
}

fn rational_pow_rational_prec_round_all(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::rational_pow_rational_prec_round(x.clone(), y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::rational_pow_rational_prec_round_ref(x, y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::rational_pow_rational_prec(x.clone(), y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::rational_pow_rational_prec_ref(x, y, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_rational_pow_rational_prec_round() {
    let check = |x: &str, y: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o| {
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = rational_pow_rational_prec_round_all(
            &Rational::from_str(x).unwrap(),
            &Rational::from_str(y).unwrap(),
            prec,
            rm,
        );
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("0", "-3", 10, Exact, "NaN", "NaN", Equal);
    check("0", "-1/3", 10, Exact, "Infinity", "Infinity", Equal);
    check("1/27", "-1/3", 10, Exact, "3.0", "0x3.0", Equal);
    check("-1/3", "1/3", 10, Exact, "NaN", "NaN", Equal);
    check("1/3", "-5", 10, Exact, "243.0", "0xf3.0", Equal);
    check("1/4", "1/2", 10, Exact, "0.50", "0x0.8", Equal);
    check("1/3", "1/2", 1, Nearest, "0.50", "0x0.8", Less);
    check("1/3", "1/2", 10, Floor, "0.57715", "0x0.93c", Less);
    check("1/3", "1/2", 10, Ceiling, "0.578", "0x0.94", Greater);
    check(
        "1/3",
        "1/2",
        100,
        Nearest,
        "0.5773502691896257645091487805021",
        "0x0.93cd3a2c8198e2690c7c0f258",
        Greater,
    );
    check("8/27", "2/3", 1, Nearest, "0.50", "0x0.8", Greater);
    check("8/27", "2/3", 10, Floor, "0.4443", "0x0.71c", Less);
    check("8/27", "2/3", 10, Ceiling, "0.44482", "0x0.71e", Greater);
    check(
        "8/27",
        "2/3",
        100,
        Nearest,
        "0.4444444444444444444444444444444",
        "0x0.71c71c71c71c71c71c71c71c7",
        Less,
    );
    check("-1/3", "3", 1, Nearest, "-0.031", "-0x0.08", Greater);
    check("-1/3", "3", 10, Floor, "-0.037048", "-0x0.097c", Less);
    check("-1/3", "3", 10, Ceiling, "-0.03699", "-0x0.0978", Greater);
    check(
        "-1/3",
        "3",
        100,
        Nearest,
        "-0.037037037037037037037037037037046",
        "-0x0.097b425ed097b425ed097b425f",
        Less,
    );
    check("22/7", "-1/3", 1, Nearest, "0.50", "0x0.8", Less);
    check("22/7", "-1/3", 10, Floor, "0.68262", "0x0.aec", Less);
    check("22/7", "-1/3", 10, Ceiling, "0.6836", "0x0.af", Greater);
    check(
        "22/7",
        "-1/3",
        100,
        Nearest,
        "0.68269248109553167539365300197794",
        "0x0.aec4ef3787c8bd690b4465a77",
        Greater,
    );
}

#[test]
#[should_panic]
fn rational_pow_rational_prec_round_fail_1() {
    BigExponentFloat::rational_pow_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        Rational::from_unsigneds(1u8, 3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn rational_pow_rational_prec_round_fail_2() {
    BigExponentFloat::rational_pow_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        Rational::from_unsigneds(1u8, 3),
        10,
        Exact,
    );
}

#[test]
fn rational_pow_rational_prec_round_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(x, y, prec, rm)| {
            let r = rational_pow_rational_prec_round_all(&x, &y, prec, rm);
            assert!(same_pair(
                &rational_pow_rational_prec_round_generic(&x, &y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // A dyadic exponent gives the power with a `BigExponentFloat` exponent.
            if let Ok(yb) = BigExponentFloat::try_from(&y) {
                assert!(same_pair(
                    &BigExponentFloat::rational_pow_prec_round_ref_ref(&x, &yb, prec, rm),
                    &r
                ));
            }
            if x != 0u32 {
                let (f, fo) = Float::rational_pow_rational_prec_round_ref(&x, &y, prec, rm);
                if float_result_comparable(&f, fo) {
                    assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                    assert_eq!(r.1, fo);
                }
            }
        },
    );
}

fn unsigned_pow_prec_round_all(
    x: u64,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::unsigned_pow_prec_round(x, y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::unsigned_pow_prec_round_ref(x, y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::unsigned_pow_prec(x, y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::unsigned_pow_prec_ref(x, y, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_unsigned_pow_prec_round() {
    let check =
        |x: u64, y: &str, y_hex: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o| {
            assert_eq!(parse_big_exponent_float_hex(y_hex).to_string(), y);
            let y = &parse_big_exponent_float_hex(y_hex);
            assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
            let out = &parse_big_exponent_float_hex(out_hex);
            let r = unsigned_pow_prec_round_all(x, y, prec, rm);
            assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
            assert_eq!(r.1, o);
        };
    check(0, "-3.0", "-0x3", 10, Exact, "NaN", "NaN", Equal);
    check(1, "NaN", "NaN", 10, Exact, "1.0", "0x1.0", Equal);
    check(3, "NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    check(
        0,
        "-Infinity",
        "-Infinity",
        10,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    check(9, "0.50", "0x0.8", 10, Exact, "3.0", "0x3.0", Equal);
    check(
        3,
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    check(
        3,
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Floor,
        "19.81",
        "0x13.d",
        Less,
    );
    check(
        3,
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Ceiling,
        "19.844",
        "0x13.d8",
        Greater,
    );
    check(
        3,
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        100,
        Nearest,
        "19.81299074527464375657864579515",
        "0x13.d0202956e7bf3742f4cd40ec",
        Less,
    );
    check(
        3,
        "2.6e120",
        "0x1.0E+100",
        1,
        Nearest,
        "4.3e1232046301832521188248816884509488269094799407862261951888989957685502942707007817667\
        416108728889355235202946801893073832",
        "0x4.0E+1023192306064881829293264426990178577948889515462474161657524648219603931872948393\
        215416341841720759856647377005463516754",
        Greater,
    );
    check(
        3,
        "2.6e120",
        "0x1.0E+100",
        10,
        Floor,
        "3.488e12320463018325211882488168845094882690947994078622619518889899576855029427070078176\
        67416108728889355235202946801893073832",
        "0x3.46E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Less,
    );
    check(
        3,
        "2.6e120",
        "0x1.0E+100",
        10,
        Ceiling,
        "3.4921e1232046301832521188248816884509488269094799407862261951888989957685502942707007817\
        667416108728889355235202946801893073832",
        "0x3.47E+102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516754",
        Greater,
    );
    check(
        3,
        "2.6e120",
        "0x1.0E+100",
        100,
        Nearest,
        "3.4918642785658923420597798454191e1232046301832521188248816884509488269094799407862261951\
        888989957685502942707007817667416108728889355235202946801893073832",
        "0x3.46f0082fa3a39df5f323f01dcE+1023192306064881829293264426990178577948889515462474161657\
        524648219603931872948393215416341841720759856647377005463516754",
        Less,
    );
    check(
        3,
        "-2.6e120",
        "-0x1.0E+100",
        1,
        Nearest,
        "2.3e-123204630183252118824881688450948826909479940786226195188898995768550294270700781766\
        7416108728889355235202946801893073833",
        "0x4.0E-1023192306064881829293264426990178577948889515462474161657524648219603931872948393\
        215416341841720759856647377005463516755",
        Less,
    );
    check(
        3,
        "-2.6e120",
        "-0x1.0E+100",
        10,
        Floor,
        "2.86e-12320463018325211882488168845094882690947994078622619518889899576855029427070078176\
        67416108728889355235202946801893073833",
        "0x4.eE-1023192306064881829293264426990178577948889515462474161657524648219603931872948393\
        215416341841720759856647377005463516755",
        Less,
    );
    check(
        3,
        "-2.6e120",
        "-0x1.0E+100",
        10,
        Ceiling,
        "2.8641e-123204630183252118824881688450948826909479940786226195188898995768550294270700781\
        7667416108728889355235202946801893073833",
        "0x4.e2E-102319230606488182929326442699017857794888951546247416165752464821960393187294839\
        3215416341841720759856647377005463516755",
        Greater,
    );
    check(
        3,
        "-2.6e120",
        "-0x1.0E+100",
        100,
        Nearest,
        "2.8637997362563579168009432583480e-123204630183252118824881688450948826909479940786226195\
        1888989957685502942707007817667416108728889355235202946801893073833",
        "0x4.e1e2b24122276b84da32de508E-1023192306064881829293264426990178577948889515462474161657\
        524648219603931872948393215416341841720759856647377005463516755",
        Greater,
    );
}

#[test]
#[should_panic]
fn unsigned_pow_prec_round_fail_1() {
    BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(0.5), 0, Floor);
}

#[test]
#[should_panic]
fn unsigned_pow_prec_round_fail_2() {
    BigExponentFloat::unsigned_pow_prec_round(3, BigExponentFloat::from(0.5), 10, Exact);
}

#[test]
fn unsigned_pow_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(yf, x, prec, rm)| {
            let y = float_to_big_exponent_float(&yf);
            if !big_exponent_float_pow_exponent_is_moderate(&y) {
                return;
            }
            let r = unsigned_pow_prec_round_all(x, &y, prec, rm);
            assert!(same_pair(
                &BigExponentFloat::from(x).pow_prec_round_ref_ref(&y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            if x != 0 {
                let (f, fo) = Float::unsigned_pow_prec_round_ref(x, &yf, prec, rm);
                if float_result_comparable(&f, fo) {
                    assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                    assert_eq!(r.1, fo);
                }
            }
        },
    );
}

fn unsigned_pow_rational_prec_round_all(
    x: u64,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::unsigned_pow_rational_prec_round(x, y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::unsigned_pow_rational_prec_round_ref(x, y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::unsigned_pow_rational_prec(x, y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::unsigned_pow_rational_prec_ref(x, y, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_unsigned_pow_rational_prec_round() {
    let check = |x: u64, y: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o| {
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = unsigned_pow_rational_prec_round_all(x, &Rational::from_str(y).unwrap(), prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check(0, "-3", 10, Exact, "NaN", "NaN", Equal);
    check(8, "1/3", 10, Exact, "2.0", "0x2.0", Equal);
    check(4, "-1/2", 10, Exact, "0.50", "0x0.8", Equal);
    check(3, "1/2", 1, Nearest, "2.0", "0x2.0", Greater);
    check(3, "1/2", 10, Floor, "1.730", "0x1.bb", Less);
    check(3, "1/2", 10, Ceiling, "1.7324", "0x1.bb8", Greater);
    check(
        3,
        "1/2",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708",
        Greater,
    );
    check(3, "-22/7", 1, Nearest, "0.031", "0x0.08", Less);
    check(3, "-22/7", 10, Floor, "0.03162", "0x0.0818", Less);
    check(3, "-22/7", 10, Ceiling, "0.031677", "0x0.081c", Greater);
    check(
        3,
        "-22/7",
        100,
        Nearest,
        "0.031657459255820452603747737955225",
        "0x0.081ab4082d9e9a66319bbbe775",
        Greater,
    );
}

#[test]
#[should_panic]
fn unsigned_pow_rational_prec_round_fail_1() {
    BigExponentFloat::unsigned_pow_rational_prec_round(
        3,
        Rational::from_unsigneds(1u8, 3),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn unsigned_pow_rational_prec_round_fail_2() {
    BigExponentFloat::unsigned_pow_rational_prec_round(
        3,
        Rational::from_unsigneds(1u8, 3),
        10,
        Exact,
    );
}

#[test]
fn unsigned_pow_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(y, x, prec, rm)| {
            let r = unsigned_pow_rational_prec_round_all(x, &y, prec, rm);
            assert!(same_pair(
                &BigExponentFloat::from(x).pow_rational_prec_round_ref_ref(&y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            if x != 0 {
                let (f, fo) = Float::unsigned_pow_rational_prec_round_ref(x, &y, prec, rm);
                if float_result_comparable(&f, fo) {
                    assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                    assert_eq!(r.1, fo);
                }
            }
        },
    );
}

fn unsigned_pow_unsigned_prec_round_all(
    x: u64,
    y: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::unsigned_pow_unsigned_prec_round(x, y, prec, rm);
    assert!(r.0.is_valid());
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::unsigned_pow_unsigned_prec(x, y, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_unsigned_pow_unsigned_prec_round() {
    let check = |x: u64, y: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o| {
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = unsigned_pow_unsigned_prec_round_all(x, y, prec, rm);
        assert!(same(&r.0, out), "{:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check(0, 0, 10, Exact, "1.0", "0x1.0", Equal);
    check(3, 4, 7, Exact, "81.00", "0x51.0", Equal);
    check(3, 100, 1, Nearest, "3.7e47", "0x4.0E+39", Less);
    check(3, 100, 10, Floor, "5.152e47", "0x5.a4E+39", Less);
    check(3, 100, 10, Ceiling, "5.1595e47", "0x5.a6E+39", Greater);
    check(
        3,
        100,
        100,
        Nearest,
        "5.1537752073201133103646112976572e47",
        "0x5.a4653ca673768565b41f775d8E+39",
        Greater,
    );
    check(
        3,
        1000000000000,
        1,
        Nearest,
        "4.1e477121254719",
        "0x2.0E+396240625180",
        Less,
    );
    check(
        3,
        1000000000000,
        10,
        Floor,
        "4.592e477121254719",
        "0x2.3aE+396240625180",
        Less,
    );
    check(
        3,
        1000000000000,
        10,
        Ceiling,
        "4.6003e477121254719",
        "0x2.3bE+396240625180",
        Greater,
    );
    check(
        3,
        1000000000000,
        100,
        Nearest,
        "4.5966061631523272210219172829282e477121254719",
        "0x2.3a89f598a33f1063911441d84E+396240625180",
        Less,
    );
}

#[test]
#[should_panic]
fn unsigned_pow_unsigned_prec_round_fail_1() {
    BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 0, Floor);
}

#[test]
#[should_panic]
fn unsigned_pow_unsigned_prec_round_fail_2() {
    BigExponentFloat::unsigned_pow_unsigned_prec_round(3, 100, 10, Exact);
}

#[test]
fn unsigned_pow_unsigned_prec_round_properties() {
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, prec, rm)| {
            let r = unsigned_pow_unsigned_prec_round_all(x, y, prec, rm);
            assert!(same_pair(
                &BigExponentFloat::from(x).pow_u_prec_round(y, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            let (f, fo) = Float::unsigned_pow_unsigned_prec_round(x, y, prec, rm);
            if float_result_comparable(&f, fo) {
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
