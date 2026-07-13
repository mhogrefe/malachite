// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{
    CheckedRoot, IsPowerOf2, Parity, Pow, PowerOf2, Reciprocal, Root, RootAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Two, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_signed_pair_gen, primitive_float_unsigned_pair_gen_var_1,
};
use malachite_float::arithmetic::root::{
    primitive_float_root_s, primitive_float_root_s_rational, primitive_float_root_u,
    primitive_float_root_u_rational,
};
use malachite_float::test_util::arithmetic::root::{
    rug_root_s, rug_root_s_prec, rug_root_s_prec_round, rug_root_s_round, rug_root_u,
    rug_root_u_prec, rug_root_u_prec_round, rug_root_u_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_unsigned_rounding_mode_quadruple_gen_var_13,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_14,
    float_signed_unsigned_triple_gen_var_1, float_unsigned_pair_gen,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_14,
    float_unsigned_unsigned_triple_gen_var_1,
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

#[test]
fn test_root_u() {
    let test = |s, s_hex, n: u64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (p, o) = x.root_u_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - n == 0 fast path: x^0 = 1
    test("3.0", "0x3.0#2", 0, 10, Nearest, "NaN", "NaN", Equal);
    // - n == 1 fast path: x^1 = x
    test("3.0", "0x3.0#2", 1, 10, Nearest, "3.0", "0x3.00#10", Equal);
    // - n == 1 fast path with rounding
    test("1.2", "0x1.4#3", 1, 2, Nearest, "1.0", "0x1.0#2", Less);
    // - n == 2 fast path: x^2 = sqr(x)
    test(
        "3.0",
        "0x3.0#2",
        2,
        10,
        Nearest,
        "1.732",
        "0x1.bb8#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 2, 2, Nearest, "1.5", "0x1.8#2", Less);
    // - k >= 3 (the integer-root path)
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "1.245731",
        "0x1.3ee84#20",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 2, Floor, "1.0", "0x1.0#2", Less);
    test("3.0", "0x3.0#2", 5, 2, Ceiling, "1.5", "0x1.8#2", Greater);
    test(
        "1.5",
        "0x1.8#2",
        3,
        10,
        Nearest,
        "1.145",
        "0x1.250#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#4",
        10,
        20,
        Nearest,
        "1.04138",
        "0x1.0a97e#20",
        Greater,
    );
    // - negative argument, odd root order gives a negative result
    test(
        "-2.0",
        "-0x2.0#1",
        3,
        10,
        Nearest,
        "-1.26",
        "-0x1.428#10",
        Greater,
    );
    // - negative argument, even root order gives NaN
    test("-2.0", "-0x2.0#1", 4, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-3.0",
        "-0x3.0#2",
        3,
        10,
        Nearest,
        "-1.441",
        "-0x1.710#10",
        Greater,
    );
    // - inexact argument: the root of the rounded value is computed
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        2,
        53,
        Nearest,
        "1.189207115002721",
        "0x1.306fe0a31b715#53",
        Less,
    );
    // - overflow
    test(
        "2.0",
        "0x2.0#3",
        100000000000,
        5,
        Nearest,
        "1.0",
        "0x1.0#5",
        Less,
    );
    // - overflow with Down gives the largest finite value
    test(
        "2.0",
        "0x2.0#3",
        100000000000,
        5,
        Down,
        "1.0",
        "0x1.0#5",
        Less,
    );
    // - underflow to zero
    test(
        "0.5",
        "0x0.8#3",
        100000000000,
        5,
        Nearest,
        "1.0",
        "0x1.0#5",
        Greater,
    );
    // - underflow with Up gives the smallest positive value
    test(
        "0.5",
        "0x0.8#3",
        100000000000,
        5,
        Up,
        "1.0",
        "0x1.0#5",
        Greater,
    );
    // - an inexact root that the integer-root path rounds directly
    test(
        "-269104312292334.303",
        "-0xf4bfbaf113ee.4d8#57",
        11,
        123,
        Floor,
        "-20.502695835577102605036400705029461424",
        "-0x14.80b0ac9da398b6b075525e74e54a44#123",
        Less,
    );
    // - large k (the exp(ln/k) path with its Ziv loop)
    test(
        "-0.0078124999999999999999999999999999",
        "-0x0.01ffffffffffffffffffffffffff8#106",
        72,
        5,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn test_root_u_special_values() {
    let test = |base: Float, n: u64, out: &str, out_hex: &str| {
        let (p, o) = base.root_u_prec_round(n, 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    test(Float::NAN, 0, "NaN", "NaN");
    test(Float::NAN, 1, "NaN", "NaN");
    test(Float::NAN, 2, "NaN", "NaN");
    test(Float::NAN, 3, "NaN", "NaN");
    test(Float::NAN, 4, "NaN", "NaN");

    test(Float::INFINITY, 0, "NaN", "NaN");
    test(Float::INFINITY, 1, "Infinity", "Infinity");
    test(Float::INFINITY, 2, "Infinity", "Infinity");
    test(Float::INFINITY, 3, "Infinity", "Infinity");
    test(Float::INFINITY, 4, "Infinity", "Infinity");

    test(Float::NEGATIVE_INFINITY, 0, "NaN", "NaN");
    test(Float::NEGATIVE_INFINITY, 1, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, 2, "NaN", "NaN");
    test(Float::NEGATIVE_INFINITY, 3, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, 4, "NaN", "NaN");

    test(Float::ZERO, 0, "NaN", "NaN");
    test(Float::ZERO, 1, "0.0", "0x0.0");
    test(Float::ZERO, 2, "0.0", "0x0.0");
    test(Float::ZERO, 3, "0.0", "0x0.0");
    test(Float::ZERO, 4, "0.0", "0x0.0");

    test(Float::NEGATIVE_ZERO, 0, "NaN", "NaN");
    test(Float::NEGATIVE_ZERO, 1, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, 2, "0.0", "0x0.0");
    test(Float::NEGATIVE_ZERO, 3, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, 4, "0.0", "0x0.0");

    test(Float::ONE, 0, "NaN", "NaN");
    test(Float::ONE, 1, "1.0", "0x1.0#1");
    test(Float::ONE, 2, "1.0", "0x1.0#1");
    test(Float::ONE, 3, "1.0", "0x1.0#1");
    test(Float::ONE, 4, "1.0", "0x1.0#1");

    test(Float::NEGATIVE_ONE, 0, "NaN", "NaN");
    test(Float::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, 2, "NaN", "NaN");
    test(Float::NEGATIVE_ONE, 3, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, 4, "NaN", "NaN");
}

#[test]
fn test_root_u_extreme() {
    let max_e = i64::from(Float::MAX_EXPONENT);
    let min_e = i64::from(Float::MIN_EXPONENT);
    let test = |base: Float, n: u64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (p, o) = base.root_u_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // 2^(MAX_EXPONENT - 1) is the largest finite power of 2; its 1st root is itself. Unlike pow, a
    // root of order k >= 1 can never overflow or underflow: the result's exponent is about
    // EXP(x)/k.
    test(
        Float::power_of_2(max_e - 1),
        1,
        10,
        Nearest,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    // its square root is an exact power of 2 (the exponent is even)
    test(
        Float::power_of_2(max_e - 1),
        2,
        10,
        Nearest,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    // the same square root at a smaller precision, with Down (still exact)
    test(
        Float::power_of_2(max_e - 1),
        2,
        5,
        Down,
        "too_big",
        "0x8.0E+134217727#5",
        Equal,
    );
    test(
        Float::power_of_2(max_e - 1),
        3,
        10,
        Ceiling,
        "too_big",
        "0x1.968E+89478485#10",
        Greater,
    );
    // the square root of the smallest positive value: its exponent is odd, so the root is sqrt(2) *
    // 2^((MIN_EXPONENT - 1)/2), inexact
    test(
        Float::power_of_2(min_e),
        2,
        10,
        Nearest,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    // the same at a smaller precision, with Up
    test(
        Float::power_of_2(min_e),
        2,
        5,
        Up,
        "too_small",
        "0x1.7E-134217728#5",
        Greater,
    );
    test(
        Float::power_of_2(min_e),
        3,
        10,
        Nearest,
        "too_small",
        "0x8.00E-89478486#10",
        Equal,
    );
    // negative argument, odd root order: a huge negative root
    test(
        -Float::power_of_2(max_e - 1),
        3,
        10,
        Nearest,
        "-too_big",
        "-0x1.968E+89478485#10",
        Less,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn root_u_prec_round_properties_helper(
    x: Float,
    n: u64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (p, o) = x.root_u_prec_round_ref(n, prec, Nearest);
        if o == Equal {
            let (pe, oe) = x.root_u_prec_round_ref(n, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.root_u_prec_round_ref(n, prec, Exact));
        }
        return;
    }
    let (p, o) = x.clone().root_u_prec_round(n, prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = x.root_u_prec_round_ref(n, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.root_u_prec_round_assign(n, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // root_u (mpfr_rootn_ui) must agree with root_s (mpfr_rootn_si) for nonnegative k.
    if let Ok(k_s) = i64::try_from(n) {
        let (pi, oi) = x.root_s_prec_round_ref(k_s, prec, rm);
        assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
        assert_eq!(oi, o);
    }

    if let (Ok(rug_rm), true) = (
        rug_round_try_from_rounding_mode(rm),
        u32::try_from(n).is_ok(),
    ) {
        let (rug_p, rug_o) = rug_root_u_prec_round(&rug::Float::exact_from(&x), n, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn root_u_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, n, prec, rm)| {
            root_u_prec_round_properties_helper(x, n, prec, rm, false);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_14().test_properties(
        |(x, n, prec, rm)| {
            root_u_prec_round_properties_helper(x, n, prec, rm, true);
        },
    );
}

#[test]
fn root_u_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, n, prec)| {
        let (p, o) = x.clone().root_u_prec(n, prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = x.root_u_prec_ref(n, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.root_u_prec_round_ref(n, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        if let Ok(k_s) = i64::try_from(n) {
            let (pi, oi) = x.root_s_prec_ref(k_s, prec);
            assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
            assert_eq!(oi, o);
        }
        if u32::try_from(n).is_err() {
            return;
        }
        let (rug_p, rug_o) = rug_root_u_prec(&rug::Float::exact_from(&x), n, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn root_u_round_properties() {
    float_unsigned_pair_gen::<u64>().test_properties(|(x, n)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = x.clone().root_u_round(n, rm);
            assert!(p.is_valid());
            let (p_alt, o_alt) = x.root_u_round_ref(n, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.root_u_round_assign(n, rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            if let (Ok(rug_rm), true) = (
                rug_round_try_from_rounding_mode(rm),
                u32::try_from(n).is_ok(),
            ) {
                let (rug_p, rug_o) = rug_root_u_round(&rug::Float::exact_from(&x), n, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p)
                );
                assert_eq!(rug_o, o);
            }
        }
    });
}

#[test]
fn root_u_properties() {
    float_unsigned_pair_gen::<u64>().test_properties(|(x, n)| {
        let p = x.clone().root(n);
        assert!(p.is_valid());
        let p_alt = (&x).root(n);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let mut x_alt = x.clone();
        x_alt.root_assign(n);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

        // The trait rounds to the nearest value at the base's precision.
        let (p_alt, _) = x.root_u_round_ref(n, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        if u32::try_from(n).is_err() {
            return;
        }
        let rug_p = rug_root_u(&rug::Float::exact_from(&x), n);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_root_u() {
    fn test<T: PrimitiveFloat>(x: T, n: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_root_u(x, n)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, 0, f32::NAN);
    test::<f32>(f32::NAN, 1, f32::NAN);
    test::<f32>(f32::NAN, 2, f32::NAN);
    test::<f32>(f32::NAN, 3, f32::NAN);
    test::<f32>(f32::NAN, 4, f32::NAN);

    test::<f32>(f32::INFINITY, 0, f32::NAN);
    test::<f32>(f32::INFINITY, 1, f32::INFINITY);
    test::<f32>(f32::INFINITY, 2, f32::INFINITY);
    test::<f32>(f32::INFINITY, 3, f32::INFINITY);
    test::<f32>(f32::INFINITY, 4, f32::INFINITY);

    test::<f32>(f32::NEGATIVE_INFINITY, 0, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 1, f32::NEGATIVE_INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 2, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 3, f32::NEGATIVE_INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 4, f32::NAN);

    test::<f32>(0.0, 0, f32::NAN);
    test::<f32>(0.0, 1, 0.0);
    test::<f32>(0.0, 2, 0.0);
    test::<f32>(0.0, 3, 0.0);
    test::<f32>(0.0, 4, 0.0);

    test::<f32>(-0.0, 0, f32::NAN);
    test::<f32>(-0.0, 1, -0.0);
    test::<f32>(-0.0, 2, 0.0);
    test::<f32>(-0.0, 3, -0.0);
    test::<f32>(-0.0, 4, 0.0);

    test::<f32>(1.0, 0, f32::NAN);
    test::<f32>(1.0, 1, 1.0);
    test::<f32>(1.0, 2, 1.0);
    test::<f32>(1.0, 3, 1.0);
    test::<f32>(1.0, 4, 1.0);

    test::<f32>(-1.0, 0, f32::NAN);
    test::<f32>(-1.0, 1, -1.0);
    test::<f32>(-1.0, 2, f32::NAN);
    test::<f32>(-1.0, 3, -1.0);
    test::<f32>(-1.0, 4, f32::NAN);

    test::<f32>(3.0, 5, 1.245731);
    test::<f32>(2.0, 10, 1.0717734);
    test::<f32>(-2.0, 3, -1.2599211);
    test::<f32>(1.1, 2, 1.0488088);
    test::<f32>(2.0, 200, 1.0034717);
    test::<f32>(0.5, 200, 0.99654025);

    test::<f64>(3.0, 5, 1.2457309396155174);
    test::<f64>(-2.0, 3, -1.2599210498948732);
    test::<f64>(1.1, 2, 1.0488088481701516);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_root_u_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, n)| {
        primitive_float_root_u::<T>(x, n);
    });
}

#[test]
fn primitive_float_root_u_properties() {
    apply_fn_to_primitive_floats!(primitive_float_root_u_properties_helper);
}

#[test]
fn test_root_s() {
    let test = |s, s_hex, n: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (p, o) = x.root_s_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - n == 0: x^0 = 1
    test("3.0", "0x3.0#2", 0, 10, Nearest, "NaN", "NaN", Equal);
    // - n == 1: x^1 = x
    test("3.0", "0x3.0#2", 1, 10, Nearest, "3.0", "0x3.00#10", Equal);
    // - k >= 0 delegates to root_u
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "1.245731",
        "0x1.3ee84#20",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 2, Floor, "1.0", "0x1.0#2", Less);
    test("3.0", "0x3.0#2", 5, 2, Ceiling, "1.5", "0x1.8#2", Greater);
    test(
        "-2.0",
        "-0x2.0#1",
        3,
        10,
        Nearest,
        "-1.26",
        "-0x1.428#10",
        Greater,
    );
    test("-2.0", "-0x2.0#1", 4, 10, Nearest, "NaN", "NaN", Equal);
    // - k < 0 takes the reciprocal of the |k|th root
    test(
        "2.0",
        "0x2.0#1",
        -3,
        10,
        Nearest,
        "0.794",
        "0x0.cb4#10",
        Greater,
    );
    test("3.0", "0x3.0#2", -2, 10, Floor, "0.577", "0x0.93c#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        -2,
        10,
        Ceiling,
        "0.578",
        "0x0.940#10",
        Greater,
    );
    // - negative argument, odd negative root order gives a negative result
    test(
        "-2.0",
        "-0x2.0#2",
        -3,
        10,
        Nearest,
        "-0.794",
        "-0x0.cb4#10",
        Less,
    );
    // - negative argument, even negative root order gives NaN
    test("-2.0", "-0x2.0#2", -4, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "1.5",
        "0x1.8#4",
        -10,
        20,
        Nearest,
        "0.960264",
        "0x0.f5d3e#20",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        -1,
        10,
        Nearest,
        "-0.3335",
        "-0x0.556#10",
        Less,
    );
    // - a power-of-2 argument whose exponent k divides has an exact reciprocal root
    test("4.0", "0x4.0#1", -3, 5, Nearest, "0.62", "0x0.a0#5", Less);
    // - a reciprocal root of a huge value is tiny but in range
    test(
        "2.0",
        "0x2.0#3",
        -100000000000,
        5,
        Nearest,
        "1.0",
        "0x1.0#5",
        Greater,
    );
    // - a reciprocal root of a tiny value is huge but in range
    test(
        "0.5",
        "0x0.8#3",
        -100000000000,
        5,
        Nearest,
        "1.0",
        "0x1.0#5",
        Less,
    );
    // - overflow with Down gives the largest finite value
    test(
        "0.5",
        "0x0.8#3",
        -100000000000,
        5,
        Down,
        "1.0",
        "0x1.0#5",
        Less,
    );
}

#[test]
fn test_root_s_special_values() {
    let test = |base: Float, n: i64, out: &str, out_hex: &str| {
        let (p, o) = base.root_s_prec_round(n, 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    test(Float::NAN, 0, "NaN", "NaN");
    test(Float::NAN, 1, "NaN", "NaN");
    test(Float::NAN, 2, "NaN", "NaN");
    test(Float::NAN, 3, "NaN", "NaN");
    test(Float::NAN, -1, "NaN", "NaN");
    test(Float::NAN, -2, "NaN", "NaN");
    test(Float::NAN, -3, "NaN", "NaN");

    test(Float::INFINITY, 0, "NaN", "NaN");
    test(Float::INFINITY, 1, "Infinity", "Infinity");
    test(Float::INFINITY, 2, "Infinity", "Infinity");
    test(Float::INFINITY, 3, "Infinity", "Infinity");
    test(Float::INFINITY, -1, "0.0", "0x0.0");
    test(Float::INFINITY, -2, "0.0", "0x0.0");
    test(Float::INFINITY, -3, "0.0", "0x0.0");

    test(Float::NEGATIVE_INFINITY, 0, "NaN", "NaN");
    test(Float::NEGATIVE_INFINITY, 1, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, 2, "NaN", "NaN");
    test(Float::NEGATIVE_INFINITY, 3, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, -1, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_INFINITY, -2, "NaN", "NaN");
    test(Float::NEGATIVE_INFINITY, -3, "-0.0", "-0x0.0");

    test(Float::ZERO, 0, "NaN", "NaN");
    test(Float::ZERO, 1, "0.0", "0x0.0");
    test(Float::ZERO, 2, "0.0", "0x0.0");
    test(Float::ZERO, 3, "0.0", "0x0.0");
    test(Float::ZERO, -1, "Infinity", "Infinity");
    test(Float::ZERO, -2, "Infinity", "Infinity");
    test(Float::ZERO, -3, "Infinity", "Infinity");

    test(Float::NEGATIVE_ZERO, 0, "NaN", "NaN");
    test(Float::NEGATIVE_ZERO, 1, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, 2, "0.0", "0x0.0");
    test(Float::NEGATIVE_ZERO, 3, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, -1, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_ZERO, -2, "Infinity", "Infinity");
    test(Float::NEGATIVE_ZERO, -3, "-Infinity", "-Infinity");

    test(Float::ONE, 0, "NaN", "NaN");
    test(Float::ONE, 1, "1.0", "0x1.0#1");
    test(Float::ONE, 2, "1.0", "0x1.0#1");
    test(Float::ONE, 3, "1.0", "0x1.0#1");
    test(Float::ONE, -1, "1.0", "0x1.0#1");
    test(Float::ONE, -2, "1.0", "0x1.0#1");
    test(Float::ONE, -3, "1.0", "0x1.0#1");

    test(Float::NEGATIVE_ONE, 0, "NaN", "NaN");
    test(Float::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, 2, "NaN", "NaN");
    test(Float::NEGATIVE_ONE, 3, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, -1, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, -2, "NaN", "NaN");
    test(Float::NEGATIVE_ONE, -3, "-1.0", "-0x1.0#1");
}

#[test]
fn test_root_s_extreme() {
    let max_e = i64::from(Float::MAX_EXPONENT);
    let min_e = i64::from(Float::MIN_EXPONENT);
    let test = |base: Float, n: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (p, o) = base.root_s_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // the -1st root (reciprocal) of the smallest positive value overflows: this is the only root
    // order whose result can overflow or underflow
    test(
        Float::power_of_2(min_e),
        -1,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // the -2nd root (reciprocal square root) of the largest power of 2 is tiny but in range
    test(
        Float::power_of_2(max_e - 1),
        -2,
        10,
        Nearest,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    // the same at a smaller precision, with Up
    test(
        Float::power_of_2(max_e - 1),
        -2,
        5,
        Up,
        "too_small",
        "0x2.0E-134217728#5",
        Equal,
    );
    // negative argument, odd negative root order: a tiny negative result
    test(
        -Float::power_of_2(min_e),
        -1,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        Float::power_of_2(min_e),
        -3,
        10,
        Nearest,
        "too_big",
        "0x2.00E+89478485#10",
        Equal,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn root_s_prec_round_properties_helper(
    x: Float,
    n: i64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        let (p, o) = x.root_s_prec_round_ref(n, prec, Nearest);
        if o == Equal {
            let (pe, oe) = x.root_s_prec_round_ref(n, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.root_s_prec_round_ref(n, prec, Exact));
        }
        return;
    }
    let (p, o) = x.clone().root_s_prec_round(n, prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = x.root_s_prec_round_ref(n, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.root_s_prec_round_assign(n, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // root_s specializes k = -1 to the reciprocal and k = -2 to the reciprocal square root.
    if n == -1 {
        let (pi, oi) = x.reciprocal_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
        assert_eq!(oi, o);
    } else if n == -2 {
        let (pi, oi) = x.reciprocal_sqrt_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
        assert_eq!(oi, o);
    }

    let rug_safe = i32::try_from(n).is_ok()
        && (n >= -2 || !x.is_normal() || !x.significand_ref().unwrap().is_power_of_2());
    if let (Ok(rug_rm), true) = (rug_round_try_from_rounding_mode(rm), rug_safe) {
        let (rug_p, rug_o) = rug_root_s_prec_round(&rug::Float::exact_from(&x), n, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn root_s_prec_round_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, n, prec, rm)| {
            root_s_prec_round_properties_helper(x, n, prec, rm, false);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_14().test_properties(
        |(x, n, prec, rm)| {
            root_s_prec_round_properties_helper(x, n, prec, rm, true);
        },
    );
}

#[test]
fn root_s_prec_properties() {
    float_signed_unsigned_triple_gen_var_1::<i64, u64>().test_properties(|(x, n, prec)| {
        let (p, o) = x.clone().root_s_prec(n, prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = x.root_s_prec_ref(n, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.root_s_prec_round_ref(n, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let rug_safe = i32::try_from(n).is_ok()
            && (n >= -2 || !x.is_normal() || !x.significand_ref().unwrap().is_power_of_2());
        if !rug_safe {
            return;
        }
        let (rug_p, rug_o) = rug_root_s_prec(&rug::Float::exact_from(&x), n, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn root_s_round_properties() {
    float_signed_pair_gen::<i64>().test_properties(|(x, n)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = x.clone().root_s_round(n, rm);
            assert!(p.is_valid());
            let (p_alt, o_alt) = x.root_s_round_ref(n, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.root_s_round_assign(n, rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let rug_safe = i32::try_from(n).is_ok()
                && (n >= -2 || !x.is_normal() || !x.significand_ref().unwrap().is_power_of_2());
            if let (Ok(rug_rm), true) = (rug_round_try_from_rounding_mode(rm), rug_safe) {
                let (rug_p, rug_o) = rug_root_s_round(&rug::Float::exact_from(&x), n, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p)
                );
                assert_eq!(rug_o, o);
            }
        }
    });
}

#[test]
fn root_s_properties() {
    float_signed_pair_gen::<i64>().test_properties(|(x, n)| {
        let p = x.clone().root(n);
        assert!(p.is_valid());
        let p_alt = (&x).root(n);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let mut x_alt = x.clone();
        x_alt.root_assign(n);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

        let (p_alt, _) = x.root_s_round_ref(n, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let rug_safe = i32::try_from(n).is_ok()
            && (n >= -2 || !x.is_normal() || !x.significand_ref().unwrap().is_power_of_2());
        if !rug_safe {
            return;
        }
        let rug_p = rug_root_s(&rug::Float::exact_from(&x), n);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn root_u_rational_prec_round_properties_helper(x: Rational, k: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (r, o) = Float::root_u_rational_prec_round_ref(&x, k, prec, Nearest);
        if o == Equal {
            let (re, oe) = Float::root_u_rational_prec_round_ref(&x, k, prec, Exact);
            assert_eq!(ComparableFloatRef(&re), ComparableFloatRef(&r));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::root_u_rational_prec_round_ref(&x, k, prec, Exact));
        }
        return;
    }
    let (r, o) = Float::root_u_rational_prec_round(x.clone(), k, prec, rm);
    assert!(r.is_valid());
    let (r_alt, o_alt) = Float::root_u_rational_prec_round_ref(&x, k, prec, rm);
    assert!(r_alt.is_valid());
    assert_eq!(ComparableFloatRef(&r_alt), ComparableFloatRef(&r));
    assert_eq!(o_alt, o);

    if r.is_normal() {
        assert_eq!(r.get_prec(), Some(prec));
    }

    // An exact rational root is detected and rounded directly. (The 0th root, and even roots of
    // negative values, panic in `checked_root`; the function itself returns NaN for those.)
    if k != 0
        && (x >= 0u32 || k.odd())
        && let Some(root) = (&x).checked_root(k)
    {
        let (re, oe) = Float::from_rational_prec_round(root, prec, rm);
        assert_eq!(ComparableFloatRef(&re), ComparableFloatRef(&r));
        assert_eq!(oe, o);
    }

    // If x is exactly a Float, the rational path must agree with the Float path.
    if let Ok(xf) = Float::try_from(x.clone()) {
        let (rf, of) = xf.root_u_prec_round_ref(k, prec, rm);
        assert_eq!(ComparableFloatRef(&rf), ComparableFloatRef(&r));
        assert_eq!(of, o);
    }

    if o == Equal && r.is_normal() {
        // An exact result is rounding-mode-invariant.
        for rm2 in exhaustive_rounding_modes() {
            let (r2, o2) = Float::root_u_rational_prec_round_ref(&x, k, prec, rm2);
            assert_eq!(ComparableFloat(r2), ComparableFloat(r.clone()));
            assert_eq!(o2, Equal);
        }
    }
}

#[test]
fn root_u_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(x, k, prec, rm)| {
            root_u_rational_prec_round_properties_helper(x, k, prec, rm);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn root_s_rational_prec_round_properties_helper(x: Rational, k: i64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        let (r, o) = Float::root_s_rational_prec_round_ref(&x, k, prec, Nearest);
        if o == Equal {
            let (re, oe) = Float::root_s_rational_prec_round_ref(&x, k, prec, Exact);
            assert_eq!(ComparableFloatRef(&re), ComparableFloatRef(&r));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::root_s_rational_prec_round_ref(&x, k, prec, Exact));
        }
        return;
    }
    let (r, o) = Float::root_s_rational_prec_round(x.clone(), k, prec, rm);
    assert!(r.is_valid());
    let (r_alt, o_alt) = Float::root_s_rational_prec_round_ref(&x, k, prec, rm);
    assert!(r_alt.is_valid());
    assert_eq!(ComparableFloatRef(&r_alt), ComparableFloatRef(&r));
    assert_eq!(o_alt, o);

    if r.is_normal() {
        assert_eq!(r.get_prec(), Some(prec));
    }

    // Nonnegative k agrees with the unsigned function.
    if let Ok(ku) = u64::try_from(k) {
        let (ru, ou) = Float::root_u_rational_prec_round_ref(&x, ku, prec, rm);
        assert_eq!(ComparableFloatRef(&ru), ComparableFloatRef(&r));
        assert_eq!(ou, o);
    } else if x != 0u32 {
        // Negative k inverts the argument: x^(1/k) = (1/x)^(1/-k).
        let (ru, ou) =
            Float::root_u_rational_prec_round((&x).reciprocal(), k.unsigned_abs(), prec, rm);
        assert_eq!(ComparableFloatRef(&ru), ComparableFloatRef(&r));
        assert_eq!(ou, o);
    }

    if o == Equal && r.is_normal() {
        for rm2 in exhaustive_rounding_modes() {
            let (r2, o2) = Float::root_s_rational_prec_round_ref(&x, k, prec, rm2);
            assert_eq!(ComparableFloat(r2), ComparableFloat(r.clone()));
            assert_eq!(o2, Equal);
        }
    }
}

#[test]
fn root_s_rational_prec_round_properties() {
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, k, prec, rm)| {
            root_s_rational_prec_round_properties_helper(x, k, prec, rm);
        },
    );
}

#[test]
fn test_root_u_rational() {
    let test = |s, k, prec, rm, out: &str, out_hex: &str, out_o: Ordering| {
        let x = Rational::from_str(s).unwrap();
        let (r, o) = Float::root_u_rational_prec_round(x.clone(), k, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, out_o);
        let (r, o) = Float::root_u_rational_prec_round_ref(&x, k, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, out_o);
        if rm == Nearest {
            let (r, o) = Float::root_u_rational_prec(x.clone(), k, prec);
            assert_eq!(r.to_string(), out);
            assert_eq!(o, out_o);
            let (r, o) = Float::root_u_rational_prec_ref(&x, k, prec);
            assert_eq!(r.to_string(), out);
            assert_eq!(o, out_o);
        }
    };
    // - k = 0: NaN
    test("2", 0, 10, Nearest, "NaN", "NaN", Equal);
    // - k = 1: the value itself
    test("1/3", 1, 10, Nearest, "0.3335", "0x0.556#10", Greater);
    // - exact rational roots
    test("0", 2, 10, Nearest, "0.0", "0x0.0", Equal);
    test("8", 3, 10, Nearest, "2.0", "0x2.00#10", Equal);
    test("27/64", 3, 10, Exact, "0.75", "0x0.c00#10", Equal);
    test("-32", 5, 10, Nearest, "-2.0", "-0x2.00#10", Equal);
    // - negative x with even k: NaN
    test("-4", 2, 10, Nearest, "NaN", "NaN", Equal);
    // - irrational roots
    test(
        "2",
        2,
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(
        "2",
        3,
        53,
        Nearest,
        "1.2599210498948732",
        "0x1.428a2f98d728b#53",
        Greater,
    );
    test("3/5", 2, 20, Floor, "0.774596", "0x0.c64bf#20", Less);
    test("3/5", 2, 20, Ceiling, "0.774597", "0x0.c64c0#20", Greater);
    test(
        "-3/5",
        3,
        20,
        Nearest,
        "-0.843432",
        "-0x0.d7eb3#20",
        Greater,
    );
    // - large k (the exp(ln/k) path)
    test(
        "2",
        101,
        53,
        Nearest,
        "1.0068864466457506",
        "0x1.01c34f67210fd#53",
        Greater,
    );
}

#[test]
fn test_root_s_rational() {
    let test = |s, k, prec, rm, out: &str, out_hex: &str, out_o: Ordering| {
        let x = Rational::from_str(s).unwrap();
        let (r, o) = Float::root_s_rational_prec_round(x.clone(), k, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, out_o);
        let (r, o) = Float::root_s_rational_prec_round_ref(&x, k, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, out_o);
        if rm == Nearest {
            let (r, o) = Float::root_s_rational_prec(x.clone(), k, prec);
            assert_eq!(r.to_string(), out);
            assert_eq!(o, out_o);
            let (r, o) = Float::root_s_rational_prec_ref(&x, k, prec);
            assert_eq!(r.to_string(), out);
            assert_eq!(o, out_o);
        }
    };
    // - k = 0: NaN
    test("2", 0, 10, Nearest, "NaN", "NaN", Equal);
    // - x = 0 with negative k: +Infinity
    test("0", -2, 10, Nearest, "Infinity", "Infinity", Equal);
    // - negative x with even k: NaN
    test("-4", -2, 10, Nearest, "NaN", "NaN", Equal);
    // - exact reciprocal roots
    test("8", -3, 10, Nearest, "0.5", "0x0.800#10", Equal);
    test("4/9", -1, 10, Nearest, "2.25", "0x2.40#10", Equal);
    test("-32", -5, 10, Nearest, "-0.5", "-0x0.800#10", Equal);
    // - irrational reciprocal roots
    test(
        "2",
        -2,
        53,
        Nearest,
        "0.7071067811865476",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "2",
        -3,
        53,
        Nearest,
        "0.7937005259840998",
        "0x0.cb2ff529eb71e8#53",
        Greater,
    );
    test("3/5", -2, 20, Floor, "1.290993", "0x1.4a7e8#20", Less);
}

#[test]
fn root_u_rational_prec_round_fail() {
    assert_panic!(Float::root_u_rational_prec_round(
        Rational::from(2),
        2,
        0,
        Floor
    ));
    assert_panic!(Float::root_u_rational_prec_round(
        Rational::from(2),
        2,
        10,
        Exact
    ));
}

#[test]
fn root_s_rational_prec_round_fail() {
    assert_panic!(Float::root_s_rational_prec_round(
        Rational::from(2),
        -2,
        0,
        Floor
    ));
    assert_panic!(Float::root_s_rational_prec_round(
        Rational::from(2),
        -2,
        10,
        Exact
    ));
}

#[test]
fn test_primitive_float_root_u_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, k: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_root_u_rational(&u, k)),
            NiceFloat(out)
        );
    }
    test::<f32>("8/27", 3, 0.6666667);
    test::<f64>("2", 2, core::f64::consts::SQRT_2);
    test::<f64>("0", 5, 0.0);
    test::<f32>("-27", 3, -3.0);
}

#[test]
fn test_primitive_float_root_s_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, k: i64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_root_s_rational(&u, k)),
            NiceFloat(out)
        );
    }
    test::<f32>("8", -3, 0.5);
    test::<f64>("2", -2, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>("0", -5, f64::INFINITY);
    test::<f32>("-27", -3, -0.33333334);
}

#[test]
fn primitive_float_root_u_rational_properties() {
    // covered by root_u_rational_prec_round_properties and the emulate layer's own tests; spot
    // consistency with the direct primitive function
    primitive_float_unsigned_pair_gen_var_1::<f64, u64>().test_properties(|(x, k)| {
        if x.is_finite() && x > 0.0 {
            assert_eq!(
                NiceFloat(primitive_float_root_u_rational::<f64>(
                    &Rational::exact_from(x),
                    k
                )),
                NiceFloat(primitive_float_root_u::<f64>(x, k))
            );
        }
    });
}

#[test]
fn test_primitive_float_root_s() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, k: i64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_root_s(x, k)), NiceFloat(out));
    }
    test::<f64>(8.0, 3, 2.0);
    test::<f64>(8.0, -3, 0.5);
    test::<f64>(2.0, -2, core::f64::consts::FRAC_1_SQRT_2);
    test::<f32>(-27.0, -3, -0.33333334);
    test::<f64>(0.0, -2, f64::INFINITY);
    test::<f64>(f64::INFINITY, -2, 0.0);
    test::<f64>(2.0, 0, f64::NAN);
    test::<f64>(2.0, 1, 2.0);
}

#[test]
fn primitive_float_root_s_properties() {
    primitive_float_signed_pair_gen::<f64, i64>().test_properties(|(x, k)| {
        let root = primitive_float_root_s::<f64>(x, k);
        // consistency with the unsigned function for nonnegative k
        if let Ok(ku) = u64::try_from(k) {
            assert_eq!(
                NiceFloat(root),
                NiceFloat(primitive_float_root_u::<f64>(x, ku))
            );
        }
    });
}

// The `Exact` rounding mode for k > 100 uses an exact decomposition rather than the exp(ln/k) path,
// which could never certify an exact result.
#[test]
fn test_root_u_exact_large_k() {
    // 2^101 has an exact 101st root, 2
    let x = Float::power_of_2(101i64);
    let (root, o) = x.root_u_prec_round_ref(101, 10, Exact);
    assert_eq!(root.to_string(), "2.0");
    assert_eq!(o, Equal);
    // (-2)^101 = -2^101 has an exact 101st root, -2
    let x = -Float::power_of_2(101i64);
    let (root, o) = x.root_u_prec_round_ref(101, 10, Exact);
    assert_eq!(root.to_string(), "-2.0");
    assert_eq!(o, Equal);
    // 3^101 is exactly representable (161 bits), and its 101st root is 3
    let x = Float::exact_from(Rational::from(3u32).pow(101u64));
    let (root, o) = x.root_u_prec_round_ref(101, 10, Exact);
    assert_eq!(root.to_string(), "3.0");
    assert_eq!(o, Equal);
    // 2^102 has no exact 101st root (the exponent is not divisible by 101)
    assert_panic!(Float::power_of_2(102i64).root_u_prec_round_ref(101, 10, Exact));
    // 3 * 2^101 has no exact 101st root (the odd part is not a perfect power)
    assert_panic!((Float::exact_from(3u32) << 101u32).root_u_prec_round_ref(101, 10, Exact));
}

// The `Exact` rounding mode for k < -2 requires |x| to be 2 raised to a multiple of k.
#[test]
fn test_root_s_exact_negative_k() {
    // (2^-15)^(-1/5)... rather: the -5th root of 2^-15 is 2^3 = 8, exactly
    let x = Float::power_of_2(-15i64);
    let (root, o) = x.root_s_prec_round_ref(-5, 10, Exact);
    assert_eq!(root.to_string(), "8.0");
    assert_eq!(o, Equal);
    // the -5th root of 2^-16 is not exactly representable
    assert_panic!(Float::power_of_2(-16i64).root_s_prec_round_ref(-5, 10, Exact));
    // nor is any root of a non-power-of-2
    assert_panic!(Float::exact_from(3u32).root_s_prec_round_ref(-5, 10, Exact));
}

// Rational roots with exponents far outside the Float range: the exponent is reduced by a multiple
// of k (or, for huge k, the root is assembled from the mantissa root and 2^(e/k)).
#[test]
fn test_root_rational_extreme() {
    let test = |x: Rational, k: u64, prec: u64, rm, out: &str, out_hex: &str, out_o: Ordering| {
        let (root, o) = Float::root_u_rational_prec_round_ref(&x, k, prec, rm);
        assert!(root.is_valid());
        assert_eq!(root.to_string(), out);
        assert_eq!(to_hex_string(&root), out_hex);
        assert_eq!(o, out_o);
    };
    let e31 = 1i64 << 31;
    // - exponent 2^31, cube root: exponent ~ 2^31 / 3, in range
    test(
        Rational::from(3u32) * Rational::power_of_2(e31),
        3,
        53,
        Nearest,
        "too_big",
        "0x9.285ff0d8417a8E+178956970#53",
        Greater,
    );
    // - negative huge exponent
    test(
        Rational::from(3u32) * Rational::power_of_2(-e31),
        3,
        53,
        Nearest,
        "too_small",
        "0x3.a25da15e344feE-178956971#53",
        Greater,
    );
    // - overflow: the square root's exponent ~ 2^32 exceeds the maximum
    test(
        Rational::from(3u32) * Rational::power_of_2(1i64 << 33),
        2,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        Rational::from(3u32) * Rational::power_of_2(1i64 << 33),
        2,
        10,
        Floor,
        "too_big",
        "0x7.feE+268435455#10",
        Less,
    );
    // - underflow
    test(
        Rational::from(3u32) * Rational::power_of_2(-(1i64 << 33)),
        2,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        Rational::from(3u32) * Rational::power_of_2(-(1i64 << 33)),
        2,
        10,
        Ceiling,
        "too_small",
        "0x1.000E-268435456#10",
        Greater,
    );
    // - huge k and huge exponent (the 2^(e/k) product path): root(3 * 2^(2^31), 2^31 + 1) ~ 2
    test(
        Rational::from(3u32) * Rational::power_of_2(e31),
        (1u64 << 31) + 1,
        53,
        Nearest,
        "2.0000000003776188",
        "0x2.000000019f324#53",
        Greater,
    );
    // - the product path with a hard-to-round case exercising its Ziv loop is not constructed here;
    //   the error bound keeps the first iteration sufficient for these inputs
}

// The integer-root path truncates inputs wider than k * n bits (adopted from mpfr_cbrt and
// generalized to any k <= 100): rounding breakpoints at n bits have kth powers of at most k * n
// bits, so lower bits of the input only feed the inexact flag. This keeps the cost of a root
// independent of the input precision (formerly linear in it).
#[test]
fn test_root_u_wide_input_truncation() {
    // sqrt(2) to 10000 bits: the cube root at prec 53 uses only ~160 of those bits
    let x = Float::TWO.sqrt_prec(10000).0;
    let test = |v: (Float, Ordering), out: &str, out_hex: &str, o_out| {
        assert!(v.0.is_valid());
        assert_eq!(v.0.to_string(), out);
        assert_eq!(to_hex_string(&v.0), out_hex);
        assert_eq!(v.1, o_out);
    };
    test(
        x.root_u_prec_round_ref(3, 53, Floor),
        "1.1224620483093728",
        "0x1.1f59ac3c7d6bf#53",
        Less,
    );
    test(
        x.root_u_prec_round_ref(3, 53, Ceiling),
        "1.122462048309373",
        "0x1.1f59ac3c7d6c0#53",
        Greater,
    );
    test(
        x.root_u_prec_round_ref(3, 53, Nearest),
        "1.122462048309373",
        "0x1.1f59ac3c7d6c0#53",
        Greater,
    );
    test(
        x.root_u_prec_round_ref(100, 24, Nearest),
        "1.0034717",
        "0x1.00e386#24",
        Less,
    );
    // a wide representation of an exact cube: the dropped bits are all zero, so the root is exact
    let x = Float::from_rational_prec(Rational::from(8u32), 5000).0;
    let (v, o) = x.root_u_prec_round_ref(3, 10, Exact);
    assert_eq!(v.to_string(), "2.0");
    assert_eq!(o, Equal);
}
