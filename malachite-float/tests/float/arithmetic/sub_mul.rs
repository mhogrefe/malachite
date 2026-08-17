// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::cmp::max as cmp_max;
use malachite_base::num::arithmetic::traits::{PowerOf2, SubMul, SubMulAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{primitive_float_pair_gen, primitive_float_triple_gen};
use malachite_base::{assert_panic, max};
use malachite_float::float::arithmetic::sub_mul::{
    primitive_float_sub_mul, primitive_float_sub_mul_rational,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sub_mul::{
    rug_sub_mul, rug_sub_mul_prec, rug_sub_mul_prec_round, rug_sub_mul_round,
    sub_mul_prec_round_naive, sub_mul_rational_prec_round_naive,
};
use malachite_float::test_util::generators::{
    float_float_float_rounding_mode_quadruple_gen_var_2,
    float_float_float_unsigned_quadruple_gen_var_1,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_3,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_4,
    float_float_rational_rounding_mode_quadruple_gen_var_2, float_float_rational_triple_gen,
    float_float_rational_unsigned_quadruple_gen_var_1,
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3,
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_4, float_triple_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sub_mul_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                u_hex: &str,
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

        let (diff, o) = x.clone().sub_mul_prec_round(y.clone(), z.clone(), prec, rm);
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_diff, rug_o) = rug_sub_mul_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                &rug::Float::exact_from(&z),
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
    // - NaN operands propagate
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - an infinite addend with a finite nonzero product
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "3.0", "0x3.0#2", 1, Nearest, "Infinity",
        "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-2.0", "-0x2.0#1", "3.0", "0x3.0#2", 1, Nearest, "Infinity",
        "Infinity", Equal,
    );
    // - infinite product and infinite addend with opposite signs (as in mpfr_fma_singular)
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "3.0",
        "0x3.0#2",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - infinite product and infinite addend with the same sign
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "3.0", "0x3.0#2", 1, Nearest, "NaN", "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "3.0",
        "0x3.0#2",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    // - an infinite product with a finite addend
    test(
        "2.0",
        "0x2.0#1",
        "Infinity",
        "Infinity",
        "3.0",
        "0x3.0#2",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "-3.0", "-0x3.0#2", 1, Nearest, "Infinity",
        "Infinity", Equal,
    );
    // - an infinite multiplicand times zero
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - zero plus zero: positive unless both are negative, except under Floor, where it is negative
    //   unless both are positive
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", "3.0", "0x3.0#2", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    // - a zero product leaves the rounded addend
    test(
        "2.0", "0x2.0#1", "0.0", "0x0.0", "3.0", "0x3.0#2", 5, Nearest, "2.00", "0x2.0#5", Equal,
    );
    // - a zero addend leaves the rounded product
    test(
        "-2.0", "-0x2.0#1", "0.0", "0x0.0", "-3.0", "-0x3.0#2", 5, Nearest, "-2.00", "-0x2.0#5",
        Equal,
    );
    test(
        "0.0", "0x0.0", "2.0", "0x2.0#1", "3.0", "0x3.0#2", 5, Nearest, "-6.00", "-0x6.0#5", Equal,
    );
    test(
        "-0.0", "-0x0.0", "2.0", "0x2.0#1", "-3.0", "-0x3.0#2", 5, Nearest, "6.00", "0x6.0#5",
        Equal,
    );
    // - finite nonzero values, exact and directed-rounding cases
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-10.000",
        "-0xa.00#10",
        Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Floor, "-12.0", "-0xc.0#2", Less,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Ceiling, "-8.0", "-0x8.0#2",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Nearest, "-8.0", "-0x8.0#2",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 4, Exact, "-10.0", "-0xa.0#4", Equal,
    );
    // - inexact values under all five basic rounding modes
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        Nearest,
        "1.3333340",
        "0x1.55556#20",
        Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        Floor,
        "1.3333321",
        "0x1.55554#20",
        Less,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        Ceiling,
        "1.3333340",
        "0x1.55556#20",
        Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        Down,
        "1.3333321",
        "0x1.55554#20",
        Less,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        Up,
        "1.3333340",
        "0x1.55556#20",
        Greater,
    );
    // - mixed signs, exact
    test(
        "-1.5",
        "-0x1.8#3",
        "5.00",
        "0x5.0#4",
        "-7.00",
        "-0x7.0#5",
        12,
        Nearest,
        "33.500",
        "0x21.80#12",
        Equal,
    );
    // - exact cancellation: the zero is positive except under Floor
    test(
        "4.0",
        "0x4.0#3",
        "-2.0",
        "-0x2.0#3",
        "2.0",
        "0x2.0#3",
        10,
        Nearest,
        "8.0000",
        "0x8.00#10",
        Equal,
    );
    test(
        "4.0",
        "0x4.0#3",
        "-2.0",
        "-0x2.0#3",
        "2.0",
        "0x2.0#3",
        10,
        Floor,
        "8.0000",
        "0x8.00#10",
        Equal,
    );
    // - the product overflows with the addend's sign: saturation per the rounding mode
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    // - the product overflows against the addend's sign; the exact scaled sum is representable
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    // - the product overflows against a smaller addend of opposite sign (integer-level scaled path
    //   at the exponent-range edge)
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1.0e161614248",
        "0x8.0E+134217727#1",
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1.0e161614248",
        "0x8.0E+134217727#1",
        "2.0e161614248",
        "0x1.0E+134217728#1",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-1.6e323228496",
        "-0x6.0E+268435455#2",
        "1.5e161614248",
        "0xc.0E+134217727#2",
        "2.0e161614248",
        "0x1.0E+134217728#1",
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    // - the product underflows far below the addend: minimal-value sentinel
    test(
        "3.0",
        "0x3.0#2",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        10,
        Floor,
        "2.9961",
        "0x2.ff#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "-3.1e-161614250",
        "-0x1.0E-134217729#1",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Less,
    );
    // - the addend is at the bottom of the exponent range: scaled underflow path
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        2,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#2",
        Greater,
    );
    test(
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        2,
        Nearest,
        "-4.8e-323228497",
        "-0x2.0E-268435456#2",
        Greater,
    );
    test(
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        "3.1e-161614250",
        "0x1.0E-134217729#1",
        2,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#2",
        Greater,
    );
    // - a NaN in either multiplicand position
    test(
        "1.0", "0x1.0#1", "NaN", "NaN", "1.0", "0x1.0#1", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - zero times infinity in either order
    test(
        "2.0", "0x2.0#1", "0.0", "0x0.0", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - a negative infinite addend with a finite product
    test(
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    // - a zero product against an addend that does not fit the requested precision
    test(
        "5.33333325",
        "0x5.555554#25",
        "0.0",
        "0x0.0",
        "3.0",
        "0x3.0#2",
        4,
        Floor,
        "5.00",
        "0x5.0#4",
        Less,
    );
    // - a zero addend against a product that does not fit the requested precision
    test(
        "0.0", "0x0.0", "3.0", "0x3.0#2", "3.0", "0x3.0#2", 2, Floor, "-12.0", "-0xc.0#2", Less,
    );
    // - the product's exponent is at least MAX_EXPONENT + 3 with the addend's sign opposite: still
    //   a sure overflow
    test(
        "-1.0",
        "-0x1.0#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Ceiling,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    // - the product is below the addend's stored bits and rounding window: sticky shortcut, both
    //   product signs relative to the addend
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        2,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#2",
        Greater,
    );
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        2,
        Floor,
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        Less,
    );
    test(
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        2,
        Nearest,
        "-4.8e-323228497",
        "-0x2.0E-268435456#2",
        Greater,
    );
    test(
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        "8.9e-161614261",
        "0x2.0E-134217738#1",
        2,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#2",
        Greater,
    );
}

#[test]
fn sub_mul_prec_round_fail() {
    assert_panic!(Float::from(1u32).sub_mul_prec_round(Float::ONE, Float::ONE, 0, Nearest));
    assert_panic!(Float::from(1u32).sub_mul_prec_round_ref_ref_ref(
        &Float::ONE,
        &Float::ONE,
        0,
        Nearest
    ));
    // Exact with an inexact difference: 1 - 3*3 = -8 is representable, but 1 - 3*5 = -14 is not
    // representable with 2 bits
    assert_panic!(Float::from(1u32).sub_mul_prec_round(
        Float::from(3u32),
        Float::from(5u32),
        2,
        Exact
    ));
    // Exact with an overflowing product of the addend's sign
    assert_panic!((-Float::ONE).sub_mul_prec_round(
        Float::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Float::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        2,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn sub_mul_prec_round_properties_helper(
    x: Float,
    y: Float,
    z: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
    assert!(diff.is_valid());
    let (diff_alt, o_alt) = x.clone().sub_mul_prec_round(y.clone(), z.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    if diff.is_normal() {
        assert_eq!(diff.get_prec(), Some(prec));
    }

    // the product's factors commute
    let (diff_alt, o_alt) = x.sub_mul_prec_round_ref_ref_ref(&z, &y, prec, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    // subtracting a product is adding the product of the negated multiplicand
    let (diff_alt, o_alt) = x.add_mul_prec_round_ref_ref_ref(&-&y, &z, prec, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    // Rational-based single-rounding oracle; skipped for extreme inputs, whose exact values would
    // have exponent-sized (multi-hundred-megabyte) integer representations
    if !extreme {
        let (diff_alt, o_alt) = sub_mul_prec_round_naive(&x, &y, &z, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_diff, rug_o) = rug_sub_mul_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            &rug::Float::exact_from(&z),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_diff)),
            ComparableFloatRef(&diff)
        );
        assert_eq!(rug_o, o);
    }

    // -(x - y * z) = (-x) + y * z, with the rounding direction reversed
    let (neg_diff, neg_o) = (-&x).add_mul_prec_round_ref_ref_ref(&y, &z, prec, -rm);
    // only up to the sign of zero: exact cancellation gives +0 under Nearest in both directions, so
    // negation flips it
    assert_eq!(
        ComparableFloat((-neg_diff).abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero_ref())
    );
    assert_eq!(neg_o.reverse(), o);

    // fusing an exact product of one changes nothing: x - y * 1 is x - y in a single rounding
    if rm != Exact {
        let (diff_alt, o_alt) = x.sub_mul_prec_round_ref_ref_ref(&y, &Float::ONE, prec, rm);
        let (diff_sub, o_sub) = x.sub_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff_sub));
        assert_eq!(o_alt, o_sub);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (d, oo) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
            assert_eq!(
                ComparableFloat(d.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, Exact));
    }
}

#[test]
fn sub_mul_prec_round_properties() {
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_3().test_properties(
        |(x, y, z, prec, rm)| {
            sub_mul_prec_round_properties_helper(x, y, z, prec, rm, false);
        },
    );

    float_float_float_unsigned_rounding_mode_quintuple_gen_var_4().test_properties(
        |(x, y, z, prec, rm)| {
            sub_mul_prec_round_properties_helper(x, y, z, prec, rm, true);
        },
    );
}
#[test]
fn test_sub_mul() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                u_hex: &str,
                out: &str,
                out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let z = parse_hex_string(u_hex);
        assert_eq!(z.to_string(), u);

        let diff = x.clone().sub_mul(y.clone(), z.clone());
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);

        let diff_alt = (&x).sub_mul(&y, &z);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        let mut diff_alt = x.clone();
        diff_alt.sub_mul_assign(&y, &z);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        let rug_diff = rug_sub_mul(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            &rug::Float::exact_from(&z),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_diff)),
            ComparableFloatRef(&diff)
        );
    };
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", "NaN", "NaN",
    );
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "3.0", "0x3.0#2", "Infinity", "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "-Infinity",
        "-Infinity",
    );
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "-3.0", "-0x3.0#2", "Infinity", "Infinity",
    );
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "0.0", "0x0.0", "NaN", "NaN",
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", "0.0", "0x0.0",
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", "3.0", "0x3.0#2", "0.0", "0x0.0",
    );
    test(
        "2.0", "0x2.0#1", "0.0", "0x0.0", "3.0", "0x3.0#2", "2.0", "0x2.0#2",
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "-8.0", "-0x8.0#2",
    );
    test(
        "-2.0",
        "-0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        "-16.0",
        "-0x10.0#2",
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        "1.33333322",
        "0x1.5555538#26",
    );
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", "8.0", "0x8.0#3",
    );
}

#[test]
fn test_sub_mul_prec() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                u_hex: &str,
                prec: u64,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let z = parse_hex_string(u_hex);
        assert_eq!(z.to_string(), u);

        let (diff, o) = x.clone().sub_mul_prec(y.clone(), z.clone(), prec);
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.sub_mul_prec_ref_ref_ref(&y, &z, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", 5, "NaN", "NaN", Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        "-10.000",
        "-0xa.00#10",
        Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, "-8.0", "-0x8.0#2", Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        "1.3333340",
        "0x1.55556#20",
        Greater,
    );
    test(
        "-2.0", "-0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 4, "-14.0", "-0xe.0#4", Equal,
    );
    test(
        "4.0",
        "0x4.0#3",
        "-2.0",
        "-0x2.0#3",
        "2.0",
        "0x2.0#3",
        10,
        "8.0000",
        "0x8.00#10",
        Equal,
    );
}

#[test]
fn test_sub_mul_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                u_hex: &str,
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

        let (diff, o) = x.clone().sub_mul_round(y.clone(), z.clone(), rm);
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.sub_mul_round_ref_ref_ref(&y, &z, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_diff, rug_o) = rug_sub_mul_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                &rug::Float::exact_from(&z),
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_diff)),
                ComparableFloatRef(&diff)
            );
            assert_eq!(rug_o, o);
        }
    };
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", Nearest, "-8.0", "-0x8.0#2", Greater,
    );
    test(
        "2.00", "0x2.0#4", "3.0", "0x3.0#2", "4.0", "0x4.0#1", Exact, "-10.0", "-0xa.0#4", Equal,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        Floor,
        "1.33333322",
        "0x1.5555538#26",
        Equal,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        Ceiling,
        "1.33333322",
        "0x1.5555538#26",
        Equal,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        Nearest,
        "1.33333322",
        "0x1.5555538#26",
        Equal,
    );
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", Floor, "8.0", "0x8.0#3", Equal,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn sub_mul_prec_properties_helper(x: Float, y: Float, z: Float, prec: u64) {
    let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, Nearest);
    let (diff_alt, o_alt) = x.sub_mul_prec_ref_ref_ref(&y, &z, prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_prec(y.clone(), z.clone(), prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_prec_val_val_ref(y.clone(), &z, prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_prec_val_ref_val(&y, z.clone(), prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_prec_val_ref_ref(&y, &z, prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.sub_mul_prec_ref_val_val(y.clone(), z.clone(), prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.sub_mul_prec_ref_val_ref(y.clone(), &z, prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.sub_mul_prec_ref_ref_val(&y, z.clone(), prec);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_prec_assign(y.clone(), z.clone(), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_prec_assign_val_ref(y.clone(), &z, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_prec_assign_ref_val(&y, z.clone(), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_prec_assign_ref_ref(&y, &z, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    let (rug_diff, rug_o) = rug_sub_mul_prec(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        &rug::Float::exact_from(&z),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_diff)),
        ComparableFloatRef(&diff)
    );
    assert_eq!(rug_o, o);
}

#[test]
fn sub_mul_prec_properties() {
    float_float_float_unsigned_quadruple_gen_var_1().test_properties(|(x, y, z, prec)| {
        sub_mul_prec_properties_helper(x, y, z, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sub_mul_round_properties_helper(x: Float, y: Float, z: Float, rm: RoundingMode) {
    let prec = max!(
        x.significant_bits(),
        y.significant_bits(),
        z.significant_bits()
    );
    let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
    let (diff_alt, o_alt) = x.sub_mul_round_ref_ref_ref(&y, &z, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_round(y.clone(), z.clone(), rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_round_val_val_ref(y.clone(), &z, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_round_val_ref_val(&y, z.clone(), rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.clone().sub_mul_round_val_ref_ref(&y, &z, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.sub_mul_round_ref_val_val(y.clone(), z.clone(), rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.sub_mul_round_ref_val_ref(y.clone(), &z, rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) = x.sub_mul_round_ref_ref_val(&y, z.clone(), rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_round_assign(y.clone(), z.clone(), rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_round_assign_val_ref(y.clone(), &z, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_round_assign_ref_val(&y, z.clone(), rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_mul_round_assign_ref_ref(&y, &z, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_diff, rug_o) = rug_sub_mul_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            &rug::Float::exact_from(&z),
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_diff)),
            ComparableFloatRef(&diff)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
fn sub_mul_round_properties() {
    float_float_float_rounding_mode_quadruple_gen_var_2().test_properties(|(x, y, z, rm)| {
        sub_mul_round_properties_helper(x, y, z, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sub_mul_properties_helper(x: Float, y: Float, z: Float) {
    let prec = max!(
        x.significant_bits(),
        y.significant_bits(),
        z.significant_bits()
    );
    let (diff, _) = x.sub_mul_prec_ref_ref_ref(&y, &z, prec);
    let diff_alt = x.clone().sub_mul(y.clone(), z.clone());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = x.clone().sub_mul(y.clone(), &z);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = x.clone().sub_mul(&y, z.clone());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = x.clone().sub_mul(&y, &z);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = (&x).sub_mul(y.clone(), z.clone());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = (&x).sub_mul(y.clone(), &z);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = (&x).sub_mul(&y, z.clone());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = (&x).sub_mul(&y, &z);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let mut x_alt = x.clone();
    x_alt.sub_mul_assign(y.clone(), z.clone());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    let mut x_alt = x.clone();
    x_alt.sub_mul_assign(y.clone(), &z);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    let mut x_alt = x.clone();
    x_alt.sub_mul_assign(&y, z.clone());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    let mut x_alt = x.clone();
    x_alt.sub_mul_assign(&y, &z);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));

    let rug_diff = rug_sub_mul(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        &rug::Float::exact_from(&z),
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_diff)),
        ComparableFloatRef(&diff)
    );
}

#[test]
fn sub_mul_properties() {
    float_triple_gen().test_properties(|(x, y, z)| {
        sub_mul_properties_helper(x, y, z);
    });
}

#[test]
fn sub_mul_prec_fail() {
    assert_panic!(Float::from(1u32).sub_mul_prec(Float::ONE, Float::ONE, 0));
    assert_panic!(Float::from(1u32).sub_mul_prec_ref_ref_ref(&Float::ONE, &Float::ONE, 0));
}

#[test]
fn sub_mul_round_fail() {
    // Exact with an inexact result at the natural precision: 1 - 5 * 7 = -34 needs 4 bits
    assert_panic!(Float::from(1u32).sub_mul_round(Float::from(5u32), Float::from(7u32), Exact));
    assert_panic!(Float::from(1u32).sub_mul_round_ref_ref_ref(
        &Float::from(5u32),
        &Float::from(7u32),
        Exact
    ));
}

// The emulated primitive-float fused multiply-subtract agrees bit-for-bit with the standard
// library's hardware-backed `mul_add` with the multiplicand negated (also correctly rounded), up to
// argument order.
#[test]
fn primitive_float_sub_mul_properties() {
    primitive_float_triple_gen::<f64>().test_properties(|(x, y, z)| {
        assert_eq!(
            NiceFloat(primitive_float_sub_mul(x, y, z)),
            NiceFloat((-y).mul_add(z, x))
        );
    });

    primitive_float_triple_gen::<f32>().test_properties(|(x, y, z)| {
        assert_eq!(
            NiceFloat(primitive_float_sub_mul(x, y, z)),
            NiceFloat((-y).mul_add(z, x))
        );
    });
}

#[test]
fn test_sub_mul_rational_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                u: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let z = Rational::from_str(u).unwrap();

        let (diff, o) = x
            .clone()
            .sub_mul_rational_prec_round(y.clone(), z.clone(), prec, rm);
        assert!(diff.is_valid());
        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    // - a NaN in either Float position
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1/3", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "NaN", "NaN", "1/3", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - an infinite addend with a finite product
    test(
        "Infinity", "Infinity", "2.0", "0x2.0#1", "1/3", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-2.0", "-0x2.0#1", "1/3", 1, Nearest, "Infinity", "Infinity",
        Equal,
    );
    // - an infinite product against an infinite addend of the opposite sign
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "-1/3", 1, Nearest, "Infinity", "Infinity",
        Equal,
    );
    // - a negative infinite addend
    test(
        "-Infinity",
        "-Infinity",
        "2.0",
        "0x2.0#1",
        "22/7",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    // - an infinite multiplier times a zero Rational
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "0", 1, Nearest, "NaN", "NaN", Equal,
    );
    // - an infinite multiplier with a finite addend, both product signs
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "-1/3", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "-Infinity",
        "-Infinity",
        "1/3",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - zero plus a zero product: the addition sign rules, a zero Rational counting as positive
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", "1/3", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", "1/3", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-2.0", "-0x2.0#1", "0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    // - a zero product against a nonzero addend, exact and rounded
    test(
        "2.0", "0x2.0#1", "0.0", "0x0.0", "22/7", 5, Nearest, "2.00", "0x2.0#5", Equal,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "0.0",
        "0x0.0",
        "1/3",
        4,
        Floor,
        "5.00",
        "0x5.0#4",
        Less,
    );
    // - a zero addend against a nonzero product; 3 * 1/3 is exactly 1
    test(
        "0.0", "0x0.0", "3.0", "0x3.0#2", "1/3", 5, Nearest, "-1.00", "-0x1.0#5", Equal,
    );
    test(
        "-0.0", "-0x0.0", "3.0", "0x3.0#2", "-1/3", 5, Nearest, "1.00", "0x1.0#5", Equal,
    );
    // - finite nonzero values; a dyadic-diffming case and general rounding under all basic modes
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "1/3",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "22/7",
        10,
        Floor,
        "-7.4297",
        "-0x7.6e#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "22/7",
        10,
        Ceiling,
        "-7.4219",
        "-0x7.6c#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "22/7",
        10,
        Nearest,
        "-7.4297",
        "-0x7.6e#10",
        Less,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "22/7", 2, Down, "-6.0", "-0x6.0#2", Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "22/7", 2, Up, "-8.0", "-0x8.0#2", Less,
    );
    // - negative Rational multiplicand with many-bit Floats
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-355/113",
        20,
        Floor,
        "9.5221100",
        "0x9.85a9#20",
        Less,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-355/113",
        20,
        Ceiling,
        "9.5221252",
        "0x9.85aa#20",
        Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "-355/113",
        20,
        Nearest,
        "9.5221252",
        "0x9.85aa#20",
        Greater,
    );
    // - an integer Rational, whose denominator of 1 takes the integer assembly path
    test(
        "1.5",
        "0x1.8#2",
        "3.0",
        "0x3.0#2",
        "5",
        10,
        Floor,
        "-13.500",
        "-0xd.80#10",
        Equal,
    );
    // - exact cancellation, x = -y * z: the zero is positive except under Floor (this is the only
    //   reachable route to the scaled core's cancellation branch)
    test(
        "2.0",
        "0x2.0#1",
        "1.0",
        "0x1.0#1",
        "-2",
        10,
        Nearest,
        "4.0000",
        "0x4.00#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "1.0",
        "0x1.0#1",
        "-2",
        10,
        Floor,
        "4.0000",
        "0x4.00#10",
        Equal,
    );
    // - Exact rounding with an exactly representable result
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "2/3", 4, Exact, "0.0", "0x0.0", Equal,
    );
    // - the product overflows: saturation per the rounding mode
    test(
        "3.0",
        "0x3.0#2",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1208925819614629174706176",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1208925819614629174706176",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    // - operands at the bottom of the exponent range: clamped alignment, both directions
    test(
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1/3626777458843887524118528",
        2,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#2",
        Greater,
    );
    test(
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1/3626777458843887524118528",
        2,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#2",
        Greater,
    );
    // - overflowing operands with an exactly cancelling integer Rational
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "-1",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
}

#[test]
fn sub_mul_rational_prec_round_fail() {
    assert_panic!(Float::from(1u32).sub_mul_rational_prec_round(
        Float::ONE,
        Rational::from_signeds(1i32, 3i32),
        0,
        Nearest
    ));
    // Exact with an inexact result
    assert_panic!(Float::from(1u32).sub_mul_rational_prec_round(
        Float::ONE,
        Rational::from_signeds(1i32, 3i32),
        2,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn sub_mul_rational_prec_round_properties_helper(
    x: Float,
    y: Float,
    z: Rational,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
    assert!(diff.is_valid());
    for (diff_alt, o_alt) in [
        x.clone()
            .sub_mul_rational_prec_round(y.clone(), z.clone(), prec, rm),
        x.clone()
            .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, prec, rm),
        x.clone()
            .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), prec, rm),
        x.clone()
            .sub_mul_rational_prec_round_val_ref_ref(&y, &z, prec, rm),
        x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm),
        x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, prec, rm),
        x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), prec, rm),
    ] {
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }
    for (x_alt, o_alt) in [
        {
            let mut x_alt = x.clone();
            let o = x_alt.sub_mul_rational_prec_round_assign(y.clone(), z.clone(), prec, rm);
            (x_alt, o)
        },
        {
            let mut x_alt = x.clone();
            let o = x_alt.sub_mul_rational_prec_round_assign_val_ref(y.clone(), &z, prec, rm);
            (x_alt, o)
        },
        {
            let mut x_alt = x.clone();
            let o = x_alt.sub_mul_rational_prec_round_assign_ref_val(&y, z.clone(), prec, rm);
            (x_alt, o)
        },
        {
            let mut x_alt = x.clone();
            let o = x_alt.sub_mul_rational_prec_round_assign_ref_ref(&y, &z, prec, rm);
            (x_alt, o)
        },
    ] {
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    if diff.is_normal() {
        assert_eq!(diff.get_prec(), Some(prec));
    }

    // Rational-based single-rounding oracle; skipped for extreme inputs, whose exact values would
    // have exponent-sized integer representations
    if !extreme {
        let (diff_alt, o_alt) = sub_mul_rational_prec_round_naive(&x, &y, &z, prec, rm);
        assert_eq!(
            ComparableFloatRef(&diff_alt),
            ComparableFloatRef(&diff),
            "INPUTS x={x:#x} y={y:#x} z={z} prec={prec} rm={rm}"
        );
        assert_eq!(o_alt, o);
    }

    // a dyadic Rational multiplicand must agree with the Float-Float fused operation
    if let Ok(zf) = Float::try_from(z.clone()) {
        let (diff_alt, o_alt) = x.sub_mul_prec_round_ref_ref_ref(&y, &zf, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    }

    // -(x - y * z) = (-x) + y * z with the rounding direction reversed, up to the sign of zero
    let (neg_diff, neg_o) = (-&x).add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, -rm);
    assert_eq!(
        ComparableFloat((-neg_diff).abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero_ref())
    );
    assert_eq!(neg_o.reverse(), o);

    // multiplying by an exact 1 is a plain subtraction
    if rm != Exact {
        let (diff_alt, o_alt) =
            x.sub_mul_rational_prec_round_ref_ref_val(&y, Rational::ONE, prec, rm);
        let (diff_add, o_add) = x.sub_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff_add));
        assert_eq!(o_alt, o_add);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, Exact));
    }
}

#[test]
fn sub_mul_rational_prec_round_properties() {
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3().test_properties(
        |(x, y, z, prec, rm)| {
            sub_mul_rational_prec_round_properties_helper(x, y, z, prec, rm, false);
        },
    );

    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_4().test_properties(
        |(x, y, z, prec, rm)| {
            sub_mul_rational_prec_round_properties_helper(x, y, z, prec, rm, true);
        },
    );
}

#[test]
fn sub_mul_rational_shorthand_properties() {
    float_float_rational_unsigned_quadruple_gen_var_1().test_properties(|(x, y, z, prec)| {
        let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, Nearest);
        let (diff_alt, o_alt) = x.sub_mul_rational_prec_ref_ref_ref(&y, &z, prec);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.clone().sub_mul_rational_prec(y.clone(), z.clone(), prec);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_mul_rational_prec_assign(y.clone(), z.clone(), prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    });

    float_float_rational_rounding_mode_quadruple_gen_var_2().test_properties(|(x, y, z, rm)| {
        let prec = cmp_max(x.significant_bits(), y.significant_bits());
        let (diff, o) = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        let (diff_alt, o_alt) = x.sub_mul_rational_round_ref_ref_ref(&y, &z, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.clone().sub_mul_rational_round(y.clone(), z.clone(), rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_mul_rational_round_assign(y.clone(), z.clone(), rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    });

    float_float_rational_triple_gen().test_properties(|(x, y, z)| {
        let prec = cmp_max(x.significant_bits(), y.significant_bits());
        let (diff, _) = x.sub_mul_rational_prec_ref_ref_ref(&y, &z, prec);
        let diff_alt = x.clone().sub_mul(y.clone(), z.clone());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = x.clone().sub_mul(y.clone(), &z);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = x.clone().sub_mul(&y, z.clone());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = x.clone().sub_mul(&y, &z);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = (&x).sub_mul(y.clone(), z.clone());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = (&x).sub_mul(&y, &z);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let mut x_alt = x.clone();
        x_alt.sub_mul_assign(y.clone(), z.clone());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        let mut x_alt = x.clone();
        x_alt.sub_mul_assign(&y, &z);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    });
}

// The emulated mixed fused multiply-subtract: for a dyadic Rational that fits the primitive type,
// it agrees with the hardware fused multiply-subtract.
#[test]
fn primitive_float_sub_mul_rational_properties() {
    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        for z in [
            Rational::from_signeds(1i64, 3i64),
            Rational::from_signeds(-22i64, 7i64),
            Rational::from_signeds(3i64, 4i64),
        ] {
            let s = primitive_float_sub_mul_rational(x, y, &z);
            if x.is_nan() || y.is_nan() {
                assert!(s.is_nan());
            }
            if let Ok(zf) = f64::try_from(z.clone()) {
                assert_eq!(NiceFloat(s), NiceFloat(primitive_float_sub_mul(x, y, zf)));
            }
        }
    });
}
