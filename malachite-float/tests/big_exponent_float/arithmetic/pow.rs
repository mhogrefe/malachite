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
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::pow::pow_prec_round_float_kernels;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float, parse_big_exponent_float_hex,
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
