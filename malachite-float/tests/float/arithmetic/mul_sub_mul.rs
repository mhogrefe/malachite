// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_quadruple_gen, primitive_float_triple_gen,
};
use malachite_base::{assert_panic, max};
use malachite_float::float::arithmetic::mul_sub_mul::{
    primitive_float_mul_sub_mul, primitive_float_mul_sub_mul_rational,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::mul_sub_mul::{
    mul_sub_mul_prec_round_naive, mul_sub_mul_rational_prec_round_naive, rug_mul_sub_mul_prec_round,
};
use malachite_float::test_util::generators::{
    float_float_float_float_rounding_mode_quintuple_gen_var_2,
    float_float_float_float_unsigned_quintuple_gen_var_1,
    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3,
    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_4,
    float_float_float_rational_quadruple_gen,
    float_float_float_rational_rounding_mode_quintuple_gen_var_2,
    float_float_float_rational_unsigned_quintuple_gen_var_1,
    float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3,
    float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_4, float_quadruple_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;
use std::str::FromStr;

// Whether rug/MPFR can be used as an oracle for these fmma/fmms inputs; see the mpfr_fmma bug note
// in the mul_add_mul test file.
fn rug_fmma_safe(a: &Float, b: &Float, c: &Float, d: &Float) -> bool {
    let out_of_range = |x: &Float, y: &Float| {
        if let (Some(ex), Some(ey)) = (x.get_exponent(), y.get_exponent()) {
            let e = i64::from(ex) + i64::from(ey);
            e > i64::from(Float::MAX_EXPONENT) || e <= i64::from(Float::MIN_EXPONENT)
        } else {
            false
        }
    };
    let zero = |x: &Float, y: &Float| *x == 0u32 || *y == 0u32;
    !(zero(a, b) && out_of_range(c, d) || zero(c, d) && out_of_range(a, b))
}

#[test]
fn test_mul_sub_mul_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                u_hex: &str,
                v: &str,
                v_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let a = parse_hex_string(s_hex);
        assert_eq!(a.to_string(), s);
        let b = parse_hex_string(t_hex);
        assert_eq!(b.to_string(), t);
        let c = parse_hex_string(u_hex);
        assert_eq!(c.to_string(), u);
        let d = parse_hex_string(v_hex);
        assert_eq!(d.to_string(), v);

        let (diff, o) = a
            .clone()
            .mul_sub_mul_prec_round(b.clone(), c.clone(), d.clone(), prec, rm);
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if rug_fmma_safe(&a, &b, &c, &d)
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        {
            let (rug_diff, rug_o) = rug_mul_sub_mul_prec_round(
                &rug::Float::exact_from(&a),
                &rug::Float::exact_from(&b),
                &rug::Float::exact_from(&c),
                &rug::Float::exact_from(&d),
                prec,
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_diff)),
                ComparableFloatRef(&diff)
            );
            assert_eq!(rug_o, o);
        }
    };
    // - a NaN in any operand position
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", "1.0", "0x1.0#1", 1, Nearest, "NaN",
        "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", "1.0", "0x1.0#1", "NaN", "NaN", 1, Nearest, "NaN",
        "NaN", Equal,
    );
    // - an infinity times a zero in either pair, in either order
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", "1.0", "0x1.0#1", "1.0", "0x1.0#1", 1, Nearest,
        "NaN", "NaN", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    // - two infinite products with opposite signs (for one of the two operations)
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "Infinity", "Infinity", "-2.0", "-0x2.0#1", 1,
        Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "Infinity", "Infinity", "2.0", "0x2.0#1", 1,
        Nearest, "NaN", "NaN", Equal,
    );
    // - one infinite product against a finite one, in either pair
    test(
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2.0#1",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - two zero products: the addition sign rules, including the Floor inversion
    test(
        "0.0", "0x0.0", "2.0", "0x2.0#1", "3.0", "0x3.0#2", "0.0", "0x0.0", 1, Nearest, "0.0",
        "0x0.0", Equal,
    );
    test(
        "-2.0", "-0x2.0#1", "0.0", "0x0.0", "-0.0", "-0x0.0", "3.0", "0x3.0#2", 1, Nearest, "0.0",
        "0x0.0", Equal,
    );
    test(
        "-2.0", "-0x2.0#1", "0.0", "0x0.0", "-0.0", "-0x0.0", "3.0", "0x3.0#2", 1, Floor, "-0.0",
        "-0x0.0", Equal,
    );
    // - a zero product against a nonzero product, in either pair, exact and rounded
    test(
        "0.0",
        "0x0.0",
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "-4.5000",
        "-0x4.80#10",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-2.0",
        "-0x2.0#1",
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "-4.5000",
        "-0x4.80#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        "0.0",
        "0x0.0",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "4.5000",
        "0x4.80#10",
        Equal,
    );
    // - finite nonzero products under all basic rounding modes
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "1.5", "0x1.8#2", 10, Nearest, "0.0",
        "0x0.0", Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "1.5", "0x1.8#2", 3, Floor, "-0.0",
        "-0x0.0", Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "1.5", "0x1.8#2", 3, Ceiling, "0.0",
        "0x0.0", Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "1.5", "0x1.8#2", 3, Down, "0.0",
        "0x0.0", Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "1.5", "0x1.8#2", 3, Up, "0.0",
        "0x0.0", Equal,
    );
    // - many-bit operands with a negative product
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-1.33333334",
        "-0x1.5555558#26",
        "1.2",
        "0x1.4#3",
        20,
        Floor,
        "8.7777710",
        "0x8.c71c#20",
        Less,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-1.33333334",
        "-0x1.5555558#26",
        "1.2",
        "0x1.4#3",
        20,
        Ceiling,
        "8.7777863",
        "0x8.c71d#20",
        Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-1.33333334",
        "-0x1.5555558#26",
        "1.2",
        "0x1.4#3",
        20,
        Nearest,
        "8.7777710",
        "0x8.c71c#20",
        Less,
    );
    // - Exact rounding with an exactly representable result
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "3.0", "0x3.0#2", "2.0", "0x2.0#1", 4, Exact, "0.0",
        "0x0.0", Equal,
    );
    // - exactly cancelling products: the zero is positive except under Floor
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "-3.0",
        "-0x3.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "12.000",
        "0xc.00#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "-3.0",
        "-0x3.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "12.000",
        "0xc.00#10",
        Equal,
    );
    // - both products overflow with the same sign: sure overflow saturation
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    // - both products overflow and cancel exactly, even though each is out of range (the only
    //   Float-Float route to the scaled core's cancellation branch)
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    // - one overflowing product against a finite one
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - an underflowing product against a normal one: clamped alignment
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "3.0",
        "0x3.0#2",
        "3.0",
        "0x3.0#2",
        10,
        Floor,
        "-9.0000",
        "-0x9.00#10",
        Less,
    );
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "3.0",
        "0x3.0#2",
        "3.0",
        "0x3.0#2",
        10,
        Ceiling,
        "-8.9844",
        "-0x8.fc#10",
        Greater,
    );
    // - both products underflow
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        2,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
}

#[test]
fn mul_sub_mul_prec_round_fail() {
    assert_panic!(Float::from(1u32).mul_sub_mul_prec_round(
        Float::ONE,
        Float::ONE,
        Float::ONE,
        0,
        Nearest
    ));
    // Exact with an inexact result: the diff needs more than 2 bits
    assert_panic!(Float::from(3u32).mul_sub_mul_prec_round(
        Float::from(3u32),
        Float::from(5u32),
        Float::from(7u32),
        2,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn mul_sub_mul_prec_round_properties_helper(
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (diff, o) = a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
    assert!(diff.is_valid());
    for (diff_alt, o_alt) in [
        a.clone()
            .mul_sub_mul_prec_round(b.clone(), c.clone(), d.clone(), prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_val_val_ref(b.clone(), c.clone(), &d, prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_val_ref_val(b.clone(), &c, d.clone(), prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_val_ref_ref(b.clone(), &c, &d, prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_ref_val_val(&b, c.clone(), d.clone(), prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_ref_val_ref(&b, c.clone(), &d, prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_ref_ref_val(&b, &c, d.clone(), prec, rm),
        a.clone()
            .mul_sub_mul_prec_round_val_ref_ref_ref(&b, &c, &d, prec, rm),
    ] {
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }
    for (diff_alt, o_alt) in [
        {
            let mut a_alt = a.clone();
            let o = a_alt.mul_sub_mul_prec_round_assign(b.clone(), c.clone(), d.clone(), prec, rm);
            (a_alt, o)
        },
        {
            let mut a_alt = a.clone();
            let o = a_alt.mul_sub_mul_prec_round_assign_ref_ref_ref(&b, &c, &d, prec, rm);
            (a_alt, o)
        },
    ] {
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    if diff.is_normal() {
        assert_eq!(diff.get_prec(), Some(prec));
    }

    // the products may be swapped, with the second's sign flipped
    let (diff_alt, o_alt) = (-&c).mul_add_mul_prec_round_ref_ref_ref_ref(&d, &a, &b, prec, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    // Rational-based single-rounding oracle; skipped for extreme inputs
    if !extreme {
        let (diff_alt, o_alt) = mul_sub_mul_prec_round_naive(&a, &b, &c, &d, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    if rug_fmma_safe(&a, &b, &c, &d)
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_diff, rug_o) = rug_mul_sub_mul_prec_round(
            &rug::Float::exact_from(&a),
            &rug::Float::exact_from(&b),
            &rug::Float::exact_from(&c),
            &rug::Float::exact_from(&d),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_diff)),
            ComparableFloatRef(&diff)
        );
        assert_eq!(rug_o, o);
    }

    // a multiplier of one reduces to the three-operand fused operation
    if rm != Exact {
        let (diff_alt, o_alt) =
            a.mul_sub_mul_prec_round_ref_ref_ref_ref(&Float::ONE, &c, &d, prec, rm);
        let (diff_fma, o_fma) = a.sub_mul_prec_round_ref_ref_ref(&c, &d, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff_fma));
        assert_eq!(o_alt, o_fma);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, Exact));
    }
}

#[test]
fn mul_sub_mul_prec_round_properties() {
    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_3().test_properties(
        |(a, b, c, d, prec, rm)| {
            mul_sub_mul_prec_round_properties_helper(a, b, c, d, prec, rm, false);
        },
    );

    float_float_float_float_unsigned_rounding_mode_sextuple_gen_var_4().test_properties(
        |(a, b, c, d, prec, rm)| {
            mul_sub_mul_prec_round_properties_helper(a, b, c, d, prec, rm, true);
        },
    );
}

#[test]
fn mul_sub_mul_shorthand_properties() {
    float_float_float_float_unsigned_quintuple_gen_var_1().test_properties(|(a, b, c, d, prec)| {
        let (diff, o) = a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, Nearest);
        let (diff_alt, o_alt) = a.mul_sub_mul_prec_ref_ref_ref_ref(&b, &c, &d, prec);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = a
            .clone()
            .mul_sub_mul_prec(b.clone(), c.clone(), d.clone(), prec);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let mut a_alt = a.clone();
        let o_alt = a_alt.mul_sub_mul_prec_assign(b.clone(), c.clone(), d.clone(), prec);
        assert_eq!(ComparableFloatRef(&a_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    });

    float_float_float_float_rounding_mode_quintuple_gen_var_2().test_properties(
        |(a, b, c, d, rm)| {
            let prec = max!(
                a.significant_bits(),
                b.significant_bits(),
                c.significant_bits(),
                d.significant_bits()
            );
            let (diff, o) = a.mul_sub_mul_prec_round_ref_ref_ref_ref(&b, &c, &d, prec, rm);
            let (diff_alt, o_alt) = a.mul_sub_mul_round_ref_ref_ref_ref(&b, &c, &d, rm);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let (diff_alt, o_alt) =
                a.clone()
                    .mul_sub_mul_round(b.clone(), c.clone(), d.clone(), rm);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let mut a_alt = a.clone();
            let o_alt = a_alt.mul_sub_mul_round_assign(b.clone(), c.clone(), d.clone(), rm);
            assert_eq!(ComparableFloatRef(&a_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
        },
    );

    float_quadruple_gen().test_properties(|(a, b, c, d)| {
        let prec = max!(
            a.significant_bits(),
            b.significant_bits(),
            c.significant_bits(),
            d.significant_bits()
        );
        let (diff, _) = a.mul_sub_mul_prec_ref_ref_ref_ref(&b, &c, &d, prec);
        let diff_alt = a.clone().mul_sub_mul(b.clone(), c.clone(), d.clone());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = (&a).mul_sub_mul(&b, &c, &d);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let mut a_alt = a.clone();
        a_alt.mul_sub_mul_assign(b.clone(), c.clone(), d.clone());
        assert_eq!(ComparableFloatRef(&a_alt), ComparableFloatRef(&diff));
        let mut a_alt = a.clone();
        a_alt.mul_sub_mul_assign(&b, &c, &d);
        assert_eq!(ComparableFloatRef(&a_alt), ComparableFloatRef(&diff));
    });
}

// The emulated primitive-float version agrees with a direct correctly-rounded conversion of the
// exact value for normal-range results.
#[test]
fn primitive_float_mul_sub_mul_properties() {
    primitive_float_quadruple_gen::<f64>().test_properties(|(a, b, c, d)| {
        let s = primitive_float_mul_sub_mul(a, b, c, d);
        if a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan() {
            assert!(s.is_nan());
        } else if [a, b, c, d].iter().all(|x| x.is_finite() && *x != 0.0) {
            let exact = Rational::exact_from(a) * Rational::exact_from(b)
                - Rational::exact_from(c) * Rational::exact_from(d);
            if exact != 0u32 {
                let approx = f64::rounding_from(&exact, Nearest).0;
                if approx.is_normal() {
                    assert_eq!(NiceFloat(s), NiceFloat(approx));
                }
            }
        }
    });
}

#[test]
fn test_mul_sub_mul_rational_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                u_hex: &str,
                v: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let z = parse_hex_string(u_hex);
        assert_eq!(z.to_string(), u);
        let w = Rational::from_str(v).unwrap();

        let (diff, o) =
            x.clone()
                .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), prec, rm);
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) =
            x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    // - a NaN in any Float position
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", "1/3", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", "NaN", "NaN", "1/3", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - an infinity times a zero in the Float pair, and an infinite Float times a zero Rational
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", "1.0", "0x1.0#1", "1/3", 1, Nearest, "NaN", "NaN",
        Equal,
    );
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", "Infinity", "Infinity", "0", 1, Nearest, "NaN", "NaN",
        Equal,
    );
    // - two infinite products with opposite signs (for one of the two operations)
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "Infinity", "Infinity", "-1/3", 1, Nearest,
        "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "Infinity", "Infinity", "1/3", 1, Nearest, "NaN",
        "NaN", Equal,
    );
    // - one infinite product against a finite one, on either side
    test(
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "22/7",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        "-Infinity",
        "-Infinity",
        "1/3",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - two zero products, from a zero Rational and from zero Floats, with the sign rules of
    //   addition including the Floor inversion; a zero Rational counts as positive
    test(
        "0.0", "0x0.0", "2.0", "0x2.0#1", "3.0", "0x3.0#2", "0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-2.0", "-0x2.0#1", "0.0", "0x0.0", "-3.0", "-0x3.0#2", "0", 1, Nearest, "0.0", "0x0.0",
        Equal,
    );
    test(
        "-2.0", "-0x2.0#1", "0.0", "0x0.0", "-3.0", "-0x3.0#2", "0", 1, Floor, "-0.0", "-0x0.0",
        Equal,
    );
    // - a zero first product against a nonzero second product, exact and rounded
    test(
        "0.0",
        "0x0.0",
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "1/3",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-2.0",
        "-0x2.0#1",
        "3.0",
        "0x3.0#2",
        "-22/7",
        10,
        Nearest,
        "9.4219",
        "0x9.6c#10",
        Less,
    );
    // - a zero second product (from a zero Float or a zero Rational) against a nonzero first
    test(
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        "0.0",
        "0x0.0",
        "22/7",
        10,
        Floor,
        "4.5000",
        "0x4.80#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "1.5",
        "0x1.8#2",
        "-2.0",
        "-0x2.0#1",
        "0",
        10,
        Ceiling,
        "4.5000",
        "0x4.80#10",
        Equal,
    );
    // - finite nonzero products under all basic rounding modes
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        "1/3",
        10,
        Nearest,
        "4.6641",
        "0x4.aa#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        "22/7",
        10,
        Floor,
        "-6.5781",
        "-0x6.94#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        "22/7",
        10,
        Ceiling,
        "-6.5703",
        "-0x6.92#10",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "22/7", 2, Down, "-6.0", "-0x6.0#2",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "22/7", 2, Up, "-8.0", "-0x8.0#2",
        Less,
    );
    // - many-bit operands with a negative Rational
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-1.33333334",
        "-0x1.5555558#26",
        "-355/113",
        20,
        Floor,
        "2.9223175",
        "0x2.ec1d0#20",
        Less,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-1.33333334",
        "-0x1.5555558#26",
        "-355/113",
        20,
        Ceiling,
        "2.9223213",
        "0x2.ec1d4#20",
        Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-1.33333334",
        "-0x1.5555558#26",
        "-355/113",
        20,
        Nearest,
        "2.9223213",
        "0x2.ec1d4#20",
        Greater,
    );
    // - Exact rounding with an exactly representable result
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "3/2", 4, Exact, "0.0", "0x0.0",
        Equal,
    );
    // - exactly cancelling products: the zero is positive except under Floor
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "2.0",
        "0x2.0#1",
        "-3",
        10,
        Nearest,
        "12.000",
        "0xc.00#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "2.0",
        "0x2.0#1",
        "-3",
        10,
        Floor,
        "12.000",
        "0xc.00#10",
        Equal,
    );
    // - an integer Rational, whose denominator of 1 takes the integer assembly path
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        "5",
        10,
        Floor,
        "-14.000",
        "-0xe.00#10",
        Equal,
    );
    // - both products overflow; the huge Rational reinforces or opposes
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "3.0",
        "0x3.0#2",
        "1208925819614629174706176",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "-3.0",
        "-0x3.0#2",
        "1208925819614629174706176",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    // - operands at the bottom of the exponent range with a tiny Rational
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1/3626777458843887524118528",
        2,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "1/3626777458843887524118528",
        2,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#2",
        Greater,
    );
    // - a cancelling first product against an overflowing second product
    // The huge Rational, 2^1073741821, is constructed directly; its decimal expansion has over 300
    // million digits.
    let x = parse_hex_string("0x4.0E+268435455#1");
    let y = parse_hex_string("-0x4.0E+268435455#1");
    let z = parse_hex_string("0x4.0E+268435455#1");
    let w = Rational::power_of_2(1073741821u64);
    let (diff, o) =
        x.clone()
            .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 10, Nearest);
    assert!(diff.is_valid());
    assert_eq!(diff.to_string(), "-Infinity");
    assert_eq!(to_hex_string(&diff), "-Infinity");
    assert_eq!(o, Less);
    let (diff_alt, o_alt) =
        x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 10, Nearest);
    assert!(diff_alt.is_valid());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
}

#[test]
fn mul_sub_mul_rational_prec_round_fail() {
    assert_panic!(Float::from(1u32).mul_sub_mul_rational_prec_round(
        Float::ONE,
        Float::ONE,
        Rational::from_signeds(1i32, 3i32),
        0,
        Nearest
    ));
    // Exact with an inexact result
    assert_panic!(Float::from(1u32).mul_sub_mul_rational_prec_round(
        Float::ONE,
        Float::ONE,
        Rational::from_signeds(1i32, 3i32),
        2,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn mul_sub_mul_rational_prec_round_properties_helper(
    x: Float,
    y: Float,
    z: Float,
    w: Rational,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm);
    assert!(diff.is_valid());
    for (diff_alt, o_alt) in [
        x.clone()
            .mul_sub_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), prec, rm),
        x.clone().mul_sub_mul_rational_prec_round_val_val_val_ref(
            y.clone(),
            z.clone(),
            &w,
            prec,
            rm,
        ),
        x.clone().mul_sub_mul_rational_prec_round_val_val_ref_val(
            y.clone(),
            &z,
            w.clone(),
            prec,
            rm,
        ),
        x.clone()
            .mul_sub_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, prec, rm),
        x.clone().mul_sub_mul_rational_prec_round_val_ref_val_val(
            &y,
            z.clone(),
            w.clone(),
            prec,
            rm,
        ),
        x.clone()
            .mul_sub_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, prec, rm),
        x.clone()
            .mul_sub_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), prec, rm),
        x.clone()
            .mul_sub_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, prec, rm),
    ] {
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }
    let mut x_alt = x.clone();
    let o_alt =
        x_alt.mul_sub_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.mul_sub_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    if diff.is_normal() {
        assert_eq!(diff.get_prec(), Some(prec));
    }

    // the Float pair's factors commute
    let (diff_alt, o_alt) = y.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&x, &z, &w, prec, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    // Rational-based single-rounding oracle; skipped for extreme inputs
    if !extreme {
        let (diff_alt, o_alt) = mul_sub_mul_rational_prec_round_naive(&x, &y, &z, &w, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    // a dyadic Rational must agree with the Float-Float fused operation
    if let Ok(wf) = Float::try_from(w.clone()) {
        let (diff_alt, o_alt) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &wf, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    // a second factor of one in the first product reduces to the three-operand mixed fused
    // operation: x * 1 - z * w = x - z * w
    if rm != Exact {
        let (diff_alt, o_alt) =
            x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&Float::ONE, &z, &w, prec, rm);
        let (diff_fma, o_fma) = x.sub_mul_rational_prec_round_ref_ref_ref(&z, &w, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff_fma));
        assert_eq!(o_alt, o_fma);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, Exact));
    }
}

#[test]
fn mul_sub_mul_rational_prec_round_properties() {
    float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_3().test_properties(
        |(x, y, z, w, prec, rm)| {
            mul_sub_mul_rational_prec_round_properties_helper(x, y, z, w, prec, rm, false);
        },
    );

    float_float_float_rational_unsigned_rounding_mode_sextuple_gen_var_4().test_properties(
        |(x, y, z, w, prec, rm)| {
            mul_sub_mul_rational_prec_round_properties_helper(x, y, z, w, prec, rm, true);
        },
    );
}

#[test]
fn mul_sub_mul_rational_shorthand_properties() {
    float_float_float_rational_unsigned_quintuple_gen_var_1().test_properties(
        |(x, y, z, w, prec)| {
            let (diff, o) =
                x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, Nearest);
            let (diff_alt, o_alt) = x.mul_sub_mul_rational_prec_ref_ref_ref_ref(&y, &z, &w, prec);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let (diff_alt, o_alt) =
                x.clone()
                    .mul_sub_mul_rational_prec(y.clone(), z.clone(), w.clone(), prec);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt =
                x_alt.mul_sub_mul_rational_prec_assign(y.clone(), z.clone(), w.clone(), prec);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
        },
    );

    float_float_float_rational_rounding_mode_quintuple_gen_var_2().test_properties(
        |(x, y, z, w, rm)| {
            let prec = max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits()
            );
            let (diff, o) = x.mul_sub_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm);
            let (diff_alt, o_alt) = x.mul_sub_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, rm);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let (diff_alt, o_alt) =
                x.clone()
                    .mul_sub_mul_rational_round(y.clone(), z.clone(), w.clone(), rm);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt =
                x_alt.mul_sub_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
        },
    );

    float_float_float_rational_quadruple_gen().test_properties(|(x, y, z, w)| {
        let prec = max!(
            x.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        let (diff, _) = x.mul_sub_mul_rational_prec_ref_ref_ref_ref(&y, &z, &w, prec);
        let diff_alt = x.clone().mul_sub_mul(y.clone(), z.clone(), w.clone());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = (&x).mul_sub_mul(&y, &z, &w);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let mut x_alt = x.clone();
        x_alt.mul_sub_mul_assign(y.clone(), z.clone(), w.clone());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        let mut x_alt = x.clone();
        x_alt.mul_sub_mul_assign(&y, &z, &w);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    });
}

// The emulated mixed version: for a dyadic Rational that fits the primitive type, it agrees with
// the all-Float emulated version.
#[test]
fn primitive_float_mul_sub_mul_rational_properties() {
    primitive_float_triple_gen::<f64>().test_properties(|(x, y, z)| {
        for w in [
            Rational::from_signeds(1i64, 3i64),
            Rational::from_signeds(-22i64, 7i64),
            Rational::from_signeds(3i64, 4i64),
        ] {
            let s = primitive_float_mul_sub_mul_rational(x, y, z, &w);
            if x.is_nan() || y.is_nan() || z.is_nan() {
                assert!(s.is_nan());
            }
            if let Ok(wf) = f64::try_from(w.clone()) {
                assert_eq!(
                    NiceFloat(s),
                    NiceFloat(primitive_float_mul_sub_mul(x, y, z, wf))
                );
            }
        }
    });
}
