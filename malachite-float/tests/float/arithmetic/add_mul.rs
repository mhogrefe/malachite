// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::cmp::max as cmp_max;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{primitive_float_pair_gen, primitive_float_triple_gen};
use malachite_base::{assert_panic, max};
use malachite_float::float::arithmetic::add_mul::{
    primitive_float_add_mul, primitive_float_add_mul_rational,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::add_mul::{
    add_mul_prec_round_naive, add_mul_rational_prec_round_naive, rug_add_mul, rug_add_mul_prec,
    rug_add_mul_prec_round, rug_add_mul_round,
};
use malachite_float::test_util::generators::{
    float_float_float_rounding_mode_quadruple_gen_var_1,
    float_float_float_unsigned_quadruple_gen_var_1,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_1,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_2,
    float_float_rational_rounding_mode_quadruple_gen_var_1, float_float_rational_triple_gen,
    float_float_rational_unsigned_quadruple_gen_var_1,
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_1,
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_2, float_triple_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_add_mul_prec_round() {
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

        let (sum, o) = x.clone().add_mul_prec_round(y.clone(), z.clone(), prec, rm);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) = rug_add_mul_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                &rug::Float::exact_from(&z),
                prec,
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
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
        "NaN",
        "NaN",
        Equal,
    );
    // - infinite product and infinite addend with the same sign
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "3.0", "0x3.0#2", 1, Nearest, "Infinity",
        "Infinity", Equal,
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
        "NaN",
        "NaN",
        Equal,
    );
    // - an infinite product with a finite addend
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "3.0", "0x3.0#2", 1, Nearest, "Infinity",
        "Infinity", Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "Infinity",
        "Infinity",
        "-3.0",
        "-0x3.0#2",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
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
        "-0.0", "-0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", "3.0", "0x3.0#2", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", 1, Floor, "0.0", "0x0.0", Equal,
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
        "0.0", "0x0.0", "2.0", "0x2.0#1", "3.0", "0x3.0#2", 5, Nearest, "6.00", "0x6.0#5", Equal,
    );
    test(
        "-0.0", "-0x0.0", "2.0", "0x2.0#1", "-3.0", "-0x3.0#2", 5, Nearest, "-6.00", "-0x6.0#5",
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
        "14.000",
        "0xe.00#10",
        Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Floor, "12.0", "0xc.0#2", Less,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Ceiling, "16.0", "0x10.0#2",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Nearest, "16.0", "0x10.0#2",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 4, Exact, "14.0", "0xe.0#4", Equal,
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
        "9.3333282",
        "0x9.5555#20",
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
        Floor,
        "9.3333282",
        "0x9.5555#20",
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
        "9.3333435",
        "0x9.5556#20",
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
        "9.3333282",
        "0x9.5555#20",
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
        "9.3333435",
        "0x9.5556#20",
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
        "-36.500",
        "-0x24.80#12",
        Equal,
    );
    // - exact cancellation: the zero is positive except under Floor
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", 10, Floor, "-0.0", "-0x0.0", Equal,
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
        "Infinity",
        "Infinity",
        Greater,
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
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
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
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
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
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
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
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
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
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Equal,
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
        "1.57e323228496",
        "0x6.0E+268435455#5",
        Equal,
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
        "3.0000",
        "0x3.00#10",
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
        "3.0039",
        "0x3.01#10",
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
        Less,
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
        Greater,
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
        Less,
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
        Less,
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
        "-3.6e-323228497",
        "-0x1.8E-268435456#2",
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
        "0.0", "0x0.0", "3.0", "0x3.0#2", "3.0", "0x3.0#2", 2, Floor, "8.0", "0x8.0#2", Less,
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
        "Infinity",
        "Infinity",
        Greater,
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
        "Infinity",
        "Infinity",
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
        Less,
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
        "4.8e-323228497",
        "0x2.0E-268435456#2",
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
        Ceiling,
        "-3.6e-323228497",
        "-0x1.8E-268435456#2",
        Greater,
    );
}

#[test]
fn add_mul_prec_round_fail() {
    assert_panic!(Float::from(1u32).add_mul_prec_round(Float::ONE, Float::ONE, 0, Nearest));
    assert_panic!(Float::from(1u32).add_mul_prec_round_ref_ref_ref(
        &Float::ONE,
        &Float::ONE,
        0,
        Nearest
    ));
    // Exact with an inexact sum: 1 + 3*3 = 10 is not representable with 2 bits
    assert_panic!(Float::from(1u32).add_mul_prec_round(
        Float::from(3u32),
        Float::from(3u32),
        2,
        Exact
    ));
    // Exact with an overflowing product of the addend's sign
    assert_panic!((Float::ONE).add_mul_prec_round(
        Float::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Float::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        2,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn add_mul_prec_round_properties_helper(
    x: Float,
    y: Float,
    z: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_mul_prec_round(y.clone(), z.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if sum.is_normal() {
        assert_eq!(sum.get_prec(), Some(prec));
    }

    // the product's factors commute
    let (sum_alt, o_alt) = x.add_mul_prec_round_ref_ref_ref(&z, &y, prec, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    // Rational-based single-rounding oracle; skipped for extreme inputs, whose exact values would
    // have exponent-sized (multi-hundred-megabyte) integer representations
    if !extreme {
        let (sum_alt, o_alt) = add_mul_prec_round_naive(&x, &y, &z, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sum, rug_o) = rug_add_mul_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            &rug::Float::exact_from(&z),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    }

    // -(x + y * z) = (-x) - y * z, with the rounding direction reversed
    let (neg_sum, neg_o) = (-&x).sub_mul_prec_round_ref_ref_ref(&y, &z, prec, -rm);
    // only up to the sign of zero: exact cancellation gives +0 under Nearest in both directions, so
    // negation flips it
    assert_eq!(
        ComparableFloat((-neg_sum).abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );
    assert_eq!(neg_o.reverse(), o);

    // fusing an exact product of one changes nothing: x + y * 1 is x + y in a single rounding
    if rm != Exact {
        let (sum_alt, o_alt) = x.add_mul_prec_round_ref_ref_ref(&y, &Float::ONE, prec, rm);
        let (sum_add, o_add) = x.add_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum_add));
        assert_eq!(o_alt, o_add);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, Exact));
    }
}

#[test]
fn add_mul_prec_round_properties() {
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_1().test_properties(
        |(x, y, z, prec, rm)| {
            add_mul_prec_round_properties_helper(x, y, z, prec, rm, false);
        },
    );

    float_float_float_unsigned_rounding_mode_quintuple_gen_var_2().test_properties(
        |(x, y, z, prec, rm)| {
            add_mul_prec_round_properties_helper(x, y, z, prec, rm, true);
        },
    );
}

#[test]
fn test_add_mul() {
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

        let sum = x.clone().add_mul(y.clone(), z.clone());
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let sum_alt = (&x).add_mul(&y, &z);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let mut sum_alt = x.clone();
        sum_alt.add_mul_assign(&y, &z);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let rug_sum = rug_add_mul(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            &rug::Float::exact_from(&z),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
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
        "2.0",
        "0x2.0#1",
        "Infinity",
        "Infinity",
        "-3.0",
        "-0x3.0#2",
        "-Infinity",
        "-Infinity",
    );
    test(
        "2.0", "0x2.0#1", "Infinity", "Infinity", "0.0", "0x0.0", "NaN", "NaN",
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", "3.0", "0x3.0#2", "0.0", "0x0.0",
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", "3.0", "0x3.0#2", "-0.0", "-0x0.0",
    );
    test(
        "2.0", "0x2.0#1", "0.0", "0x0.0", "3.0", "0x3.0#2", "2.0", "0x2.0#2",
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "16.0", "0x10.0#2",
    );
    test(
        "-2.0", "-0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", "8.0", "0x8.0#2",
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        "9.33333325",
        "0x9.555554#26",
    );
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", "0.0", "0x0.0",
    );
}

#[test]
fn test_add_mul_prec() {
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

        let (sum, o) = x.clone().add_mul_prec(y.clone(), z.clone(), prec);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.add_mul_prec_ref_ref_ref(&y, &z, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
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
        "14.000",
        "0xe.00#10",
        Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, "16.0", "0x10.0#2", Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        20,
        "9.3333282",
        "0x9.5555#20",
        Less,
    );
    test(
        "-2.0", "-0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", 4, "10.0", "0xa.0#4", Equal,
    );
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", 10, "0.0", "0x0.0", Equal,
    );
}

#[test]
fn test_add_mul_round() {
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

        let (sum, o) = x.clone().add_mul_round(y.clone(), z.clone(), rm);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.add_mul_round_ref_ref_ref(&y, &z, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) = rug_add_mul_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                &rug::Float::exact_from(&z),
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }
    };
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", "1.0", "0x1.0#1", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "4.0", "0x4.0#1", Nearest, "16.0", "0x10.0#2", Greater,
    );
    test(
        "2.00", "0x2.0#4", "3.0", "0x3.0#2", "4.0", "0x4.0#1", Exact, "14.0", "0xe.0#4", Equal,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        Floor,
        "9.33333325",
        "0x9.555554#26",
        Less,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        Ceiling,
        "9.33333349",
        "0x9.555558#26",
        Greater,
    );
    test(
        "5.33333325",
        "0x5.555554#25",
        "1.33333334",
        "0x1.5555558#26",
        "3.0",
        "0x3.0#2",
        Nearest,
        "9.33333325",
        "0x9.555554#26",
        Less,
    );
    test(
        "4.0", "0x4.0#3", "-2.0", "-0x2.0#3", "2.0", "0x2.0#3", Floor, "-0.0", "-0x0.0", Equal,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn add_mul_prec_properties_helper(x: Float, y: Float, z: Float, prec: u64) {
    let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, Nearest);
    let (sum_alt, o_alt) = x.add_mul_prec_ref_ref_ref(&y, &z, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_prec(y.clone(), z.clone(), prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_prec_val_val_ref(y.clone(), &z, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_prec_val_ref_val(&y, z.clone(), prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_prec_val_ref_ref(&y, &z, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_mul_prec_ref_val_val(y.clone(), z.clone(), prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_mul_prec_ref_val_ref(y.clone(), &z, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_mul_prec_ref_ref_val(&y, z.clone(), prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_prec_assign(y.clone(), z.clone(), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_prec_assign_val_ref(y.clone(), &z, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_prec_assign_ref_val(&y, z.clone(), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_prec_assign_ref_ref(&y, &z, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let (rug_sum, rug_o) = rug_add_mul_prec(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        &rug::Float::exact_from(&z),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum)
    );
    assert_eq!(rug_o, o);
}

#[test]
fn add_mul_prec_properties() {
    float_float_float_unsigned_quadruple_gen_var_1().test_properties(|(x, y, z, prec)| {
        add_mul_prec_properties_helper(x, y, z, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_mul_round_properties_helper(x: Float, y: Float, z: Float, rm: RoundingMode) {
    let prec = max!(
        x.significant_bits(),
        y.significant_bits(),
        z.significant_bits()
    );
    let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
    let (sum_alt, o_alt) = x.add_mul_round_ref_ref_ref(&y, &z, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_round(y.clone(), z.clone(), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_round_val_val_ref(y.clone(), &z, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_round_val_ref_val(&y, z.clone(), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.clone().add_mul_round_val_ref_ref(&y, &z, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_mul_round_ref_val_val(y.clone(), z.clone(), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_mul_round_ref_val_ref(y.clone(), &z, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_mul_round_ref_ref_val(&y, z.clone(), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_round_assign(y.clone(), z.clone(), rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_round_assign_val_ref(y.clone(), &z, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_round_assign_ref_val(&y, z.clone(), rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.add_mul_round_assign_ref_ref(&y, &z, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sum, rug_o) = rug_add_mul_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            &rug::Float::exact_from(&z),
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
fn add_mul_round_properties() {
    float_float_float_rounding_mode_quadruple_gen_var_1().test_properties(|(x, y, z, rm)| {
        add_mul_round_properties_helper(x, y, z, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_mul_properties_helper(x: Float, y: Float, z: Float) {
    let prec = max!(
        x.significant_bits(),
        y.significant_bits(),
        z.significant_bits()
    );
    let (sum, _) = x.add_mul_prec_ref_ref_ref(&y, &z, prec);
    let sum_alt = x.clone().add_mul(y.clone(), z.clone());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x.clone().add_mul(y.clone(), &z);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x.clone().add_mul(&y, z.clone());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x.clone().add_mul(&y, &z);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = (&x).add_mul(y.clone(), z.clone());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = (&x).add_mul(y.clone(), &z);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = (&x).add_mul(&y, z.clone());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = (&x).add_mul(&y, &z);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let mut x_alt = x.clone();
    x_alt.add_mul_assign(y.clone(), z.clone());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    let mut x_alt = x.clone();
    x_alt.add_mul_assign(y.clone(), &z);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    let mut x_alt = x.clone();
    x_alt.add_mul_assign(&y, z.clone());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    let mut x_alt = x.clone();
    x_alt.add_mul_assign(&y, &z);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    let rug_sum = rug_add_mul(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        &rug::Float::exact_from(&z),
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum)
    );
}

#[test]
fn add_mul_properties() {
    float_triple_gen().test_properties(|(x, y, z)| {
        add_mul_properties_helper(x, y, z);
    });
}

#[test]
fn add_mul_prec_fail() {
    assert_panic!(Float::from(1u32).add_mul_prec(Float::ONE, Float::ONE, 0));
    assert_panic!(Float::from(1u32).add_mul_prec_ref_ref_ref(&Float::ONE, &Float::ONE, 0));
}

#[test]
fn add_mul_round_fail() {
    // Exact with an inexact result at the natural precision
    assert_panic!(Float::from(1u32).add_mul_round(Float::from(3u32), Float::from(3u32), Exact));
    assert_panic!(Float::from(1u32).add_mul_round_ref_ref_ref(
        &Float::from(3u32),
        &Float::from(3u32),
        Exact
    ));
}

// The emulated primitive-float fused multiply-add agrees bit-for-bit with the standard library's
// hardware-backed `mul_add` (which is also correctly rounded), up to argument order.
#[test]
fn primitive_float_add_mul_properties() {
    primitive_float_triple_gen::<f64>().test_properties(|(x, y, z)| {
        assert_eq!(
            NiceFloat(primitive_float_add_mul(x, y, z)),
            NiceFloat(y.mul_add(z, x))
        );
    });

    primitive_float_triple_gen::<f32>().test_properties(|(x, y, z)| {
        assert_eq!(
            NiceFloat(primitive_float_add_mul(x, y, z)),
            NiceFloat(y.mul_add(z, x))
        );
    });
}

#[test]
fn test_add_mul_rational_prec_round() {
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

        let (sum, o) = x
            .clone()
            .add_mul_rational_prec_round(y.clone(), z.clone(), prec, rm);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
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
        "Infinity", "Infinity", "Infinity", "Infinity", "-1/3", 1, Nearest, "NaN", "NaN", Equal,
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
        "2.0",
        "0x2.0#1",
        "Infinity",
        "Infinity",
        "-1/3",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        "-Infinity",
        "-Infinity",
        "1/3",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
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
        "-0.0", "-0x0.0", "-2.0", "-0x2.0#1", "0", 1, Nearest, "-0.0", "-0x0.0", Equal,
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
        "0.0", "0x0.0", "3.0", "0x3.0#2", "1/3", 5, Nearest, "1.00", "0x1.0#5", Equal,
    );
    test(
        "-0.0", "-0x0.0", "3.0", "0x3.0#2", "-1/3", 5, Nearest, "-1.00", "-0x1.0#5", Equal,
    );
    // - finite nonzero values; a dyadic-summing case and general rounding under all basic modes
    test(
        "2.0",
        "0x2.0#1",
        "3.0",
        "0x3.0#2",
        "1/3",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
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
        "11.422",
        "0xb.6c#10",
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
        "11.438",
        "0xb.70#10",
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
        "11.422",
        "0xb.6c#10",
        Less,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "22/7", 2, Down, "8.0", "0x8.0#2", Less,
    );
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "22/7", 2, Up, "12.0", "0xc.0#2", Greater,
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
        "1.1445408",
        "0x1.2500a#20",
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
        "1.1445427",
        "0x1.2500c#20",
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
        "1.1445427",
        "0x1.2500c#20",
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
        "16.500",
        "0x10.80#10",
        Equal,
    );
    // - exact cancellation, x = -y * z: the zero is positive except under Floor (this is the only
    //   reachable route to the scaled core's cancellation branch)
    test(
        "2.0", "0x2.0#1", "1.0", "0x1.0#1", "-2", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "2.0", "0x2.0#1", "1.0", "0x1.0#1", "-2", 10, Floor, "-0.0", "-0x0.0", Equal,
    );
    // - Exact rounding with an exactly representable result
    test(
        "2.0", "0x2.0#1", "3.0", "0x3.0#2", "2/3", 4, Exact, "4.00", "0x4.0#4", Equal,
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
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "1208925819614629174706176",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
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
        Less,
    );
    test(
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1/3626777458843887524118528",
        2,
        Ceiling,
        "-3.6e-323228497",
        "-0x1.8E-268435456#2",
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
        "0.0",
        "0x0.0",
        Equal,
    );
}

#[test]
fn add_mul_rational_prec_round_fail() {
    assert_panic!(Float::from(1u32).add_mul_rational_prec_round(
        Float::ONE,
        Rational::from_signeds(1i32, 3i32),
        0,
        Nearest
    ));
    // Exact with an inexact result
    assert_panic!(Float::from(1u32).add_mul_rational_prec_round(
        Float::ONE,
        Rational::from_signeds(1i32, 3i32),
        2,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn add_mul_rational_prec_round_properties_helper(
    x: Float,
    y: Float,
    z: Rational,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (sum, o) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
    assert!(sum.is_valid());
    for (sum_alt, o_alt) in [
        x.clone()
            .add_mul_rational_prec_round(y.clone(), z.clone(), prec, rm),
        x.clone()
            .add_mul_rational_prec_round_val_val_ref(y.clone(), &z, prec, rm),
        x.clone()
            .add_mul_rational_prec_round_val_ref_val(&y, z.clone(), prec, rm),
        x.clone()
            .add_mul_rational_prec_round_val_ref_ref(&y, &z, prec, rm),
        x.add_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm),
        x.add_mul_rational_prec_round_ref_val_ref(y.clone(), &z, prec, rm),
        x.add_mul_rational_prec_round_ref_ref_val(&y, z.clone(), prec, rm),
    ] {
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }
    for (x_alt, o_alt) in [
        {
            let mut x_alt = x.clone();
            let o = x_alt.add_mul_rational_prec_round_assign(y.clone(), z.clone(), prec, rm);
            (x_alt, o)
        },
        {
            let mut x_alt = x.clone();
            let o = x_alt.add_mul_rational_prec_round_assign_val_ref(y.clone(), &z, prec, rm);
            (x_alt, o)
        },
        {
            let mut x_alt = x.clone();
            let o = x_alt.add_mul_rational_prec_round_assign_ref_val(&y, z.clone(), prec, rm);
            (x_alt, o)
        },
        {
            let mut x_alt = x.clone();
            let o = x_alt.add_mul_rational_prec_round_assign_ref_ref(&y, &z, prec, rm);
            (x_alt, o)
        },
    ] {
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    if sum.is_normal() {
        assert_eq!(sum.get_prec(), Some(prec));
    }

    // Rational-based single-rounding oracle; skipped for extreme inputs, whose exact values would
    // have exponent-sized integer representations
    if !extreme {
        let (sum_alt, o_alt) = add_mul_rational_prec_round_naive(&x, &y, &z, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    // a dyadic Rational multiplicand must agree with the Float-Float fused operation
    if let Ok(zf) = Float::try_from(z.clone()) {
        let (sum_alt, o_alt) = x.add_mul_prec_round_ref_ref_ref(&y, &zf, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    // -(x + y * z) = (-x) - y * z with the rounding direction reversed, up to the sign of zero
    let (neg_sum, neg_o) = (-&x).sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, -rm);
    assert_eq!(
        ComparableFloat((-neg_sum).abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );
    assert_eq!(neg_o.reverse(), o);

    // multiplying by an exact 1 is a plain addition
    if rm != Exact {
        let (sum_alt, o_alt) =
            x.add_mul_rational_prec_round_ref_ref_val(&y, Rational::ONE, prec, rm);
        let (sum_add, o_add) = x.add_prec_round_ref_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum_add));
        assert_eq!(o_alt, o_add);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, Exact));
    }
}

#[test]
fn add_mul_rational_prec_round_properties() {
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_1().test_properties(
        |(x, y, z, prec, rm)| {
            add_mul_rational_prec_round_properties_helper(x, y, z, prec, rm, false);
        },
    );

    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_2().test_properties(
        |(x, y, z, prec, rm)| {
            add_mul_rational_prec_round_properties_helper(x, y, z, prec, rm, true);
        },
    );
}

#[test]
fn add_mul_rational_shorthand_properties() {
    float_float_rational_unsigned_quadruple_gen_var_1().test_properties(|(x, y, z, prec)| {
        let (sum, o) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, Nearest);
        let (sum_alt, o_alt) = x.add_mul_rational_prec_ref_ref_ref(&y, &z, prec);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.clone().add_mul_rational_prec(y.clone(), z.clone(), prec);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.add_mul_rational_prec_assign(y.clone(), z.clone(), prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    });

    float_float_rational_rounding_mode_quadruple_gen_var_1().test_properties(|(x, y, z, rm)| {
        let prec = cmp_max(x.significant_bits(), y.significant_bits());
        let (sum, o) = x.add_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        let (sum_alt, o_alt) = x.add_mul_rational_round_ref_ref_ref(&y, &z, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.clone().add_mul_rational_round(y.clone(), z.clone(), rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.add_mul_rational_round_assign(y.clone(), z.clone(), rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    });

    float_float_rational_triple_gen().test_properties(|(x, y, z)| {
        let prec = cmp_max(x.significant_bits(), y.significant_bits());
        let (sum, _) = x.add_mul_rational_prec_ref_ref_ref(&y, &z, prec);
        let sum_alt = x.clone().add_mul(y.clone(), z.clone());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = x.clone().add_mul(y.clone(), &z);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = x.clone().add_mul(&y, z.clone());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = x.clone().add_mul(&y, &z);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = (&x).add_mul(y.clone(), z.clone());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = (&x).add_mul(&y, &z);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let mut x_alt = x.clone();
        x_alt.add_mul_assign(y.clone(), z.clone());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        let mut x_alt = x.clone();
        x_alt.add_mul_assign(&y, &z);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    });
}

// The emulated mixed fused multiply-add: for a dyadic Rational that fits the primitive type, it
// agrees with the hardware fused multiply-add.
#[test]
fn primitive_float_add_mul_rational_properties() {
    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        for z in [
            Rational::from_signeds(1i64, 3i64),
            Rational::from_signeds(-22i64, 7i64),
            Rational::from_signeds(3i64, 4i64),
        ] {
            let s = primitive_float_add_mul_rational(x, y, &z);
            if x.is_nan() || y.is_nan() {
                assert!(s.is_nan());
            }
            if let Ok(zf) = f64::try_from(z.clone()) {
                assert_eq!(NiceFloat(s), NiceFloat(primitive_float_add_mul(x, y, zf)));
            }
        }
    });
}
