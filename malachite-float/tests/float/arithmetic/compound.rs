// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Compound, CompoundAssign, Pow, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::primitive_float_signed_pair_gen_var_4;
use malachite_base::{apply_fn_to_primitive_floats, assert_panic};
use malachite_float::float::arithmetic::compound::primitive_float_compound;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::compound::{
    rug_compound, rug_compound_prec, rug_compound_prec_round, rug_compound_round,
};
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_unsigned_rounding_mode_quadruple_gen_var_17,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_18,
    float_signed_unsigned_triple_gen_var_1,
};
use malachite_float::{ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_compound() {
    let test = |s, s_hex, n: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (c, o) = x.compound_prec_round_ref(n, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);
    };
    // - n == 0
    test(
        "3.0",
        "0x3.0#2",
        0,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    // - x < -1: NaN (even with n == 0)
    test("-2.0", "-0x2.0#1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 4, 10, Floor, "NaN", "NaN", Equal);
    // - n == 1: 1 + x
    test(
        "3.0",
        "0x3.0#2",
        1,
        10,
        Nearest,
        "4.0000",
        "0x4.00#10",
        Equal,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        1,
        10,
        Floor,
        "1.0996",
        "0x1.198#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        1,
        10,
        Ceiling,
        "1.1016",
        "0x1.1a0#10",
        Greater,
    );
    // - x == -1
    test("-1.0", "-0x1.0#1", 3, 10, Nearest, "0.0", "0x0.0", Equal);
    test(
        "-1.0", "-0x1.0#1", -3, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    // - exact small cases
    test(
        "0.50",
        "0x0.8#1",
        2,
        10,
        Nearest,
        "2.2500",
        "0x2.40#10",
        Equal,
    );
    test("0.50", "0x0.8#1", 2, 4, Nearest, "2.25", "0x2.4#4", Equal);
    test(
        "0.50",
        "0x0.8#1",
        3,
        10,
        Nearest,
        "3.3750",
        "0x3.60#10",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        2,
        10,
        Nearest,
        "0.25000",
        "0x0.400#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        -3,
        10,
        Nearest,
        "64.000",
        "0x40.0#10",
        Equal,
    );
    // - general inexact, all rounding modes
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        10,
        53,
        Floor,
        "2.5937424601000001",
        "0x2.97ff818060472#53",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        10,
        53,
        Ceiling,
        "2.5937424601000005",
        "0x2.97ff818060474#53",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        10,
        53,
        Down,
        "2.5937424601000001",
        "0x2.97ff818060472#53",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        10,
        53,
        Up,
        "2.5937424601000005",
        "0x2.97ff818060474#53",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        10,
        53,
        Nearest,
        "2.5937424601000001",
        "0x2.97ff818060472#53",
        Less,
    );
    // - negative n
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        -10,
        53,
        Floor,
        "0.38554328942953170",
        "0x0.62b2f70b4ac724#53",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        -10,
        53,
        Ceiling,
        "0.38554328942953175",
        "0x0.62b2f70b4ac728#53",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        -10,
        53,
        Nearest,
        "0.38554328942953175",
        "0x0.62b2f70b4ac728#53",
        Greater,
    );
    // - x in (-1, 0)
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#53",
        7,
        30,
        Nearest,
        "0.47829690017",
        "0x0.7a71aa68#30",
        Greater,
    );
    test(
        "-0.90000000000000002",
        "-0x0.e6666666666668#53",
        -5,
        30,
        Nearest,
        "100000.00000",
        "0x186a0.0000#30",
        Less,
    );
    // - prec 1
    test("2.5", "0x2.8#3", 3, 1, Nearest, "32.0", "0x2.0E+1#1", Less);
    test("2.5", "0x2.8#3", 3, 1, Floor, "32.0", "0x2.0E+1#1", Less);
    // - near-one (tiny x)
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        3,
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        3,
        20,
        Ceiling,
        "1.0000019",
        "0x1.00002#20",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        -3,
        20,
        Floor,
        "0.99999905",
        "0x0.fffff#20",
        Less,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        3,
        20,
        Floor,
        "0.99999905",
        "0x0.fffff#20",
        Less,
    );
    // - x^n-fits escape (x a large even integer)
    test(
        "1.0e6",
        "0x1.0E+5#1",
        3,
        10,
        Nearest,
        "1.1529e18",
        "0x1.000E+15#10",
        Less,
    );
    test(
        "1.0e6",
        "0x1.0E+5#1",
        3,
        10,
        Ceiling,
        "1.1552e18",
        "0x1.008E+15#10",
        Greater,
    );
    // - Exact (valid)
    test(
        "0.50",
        "0x0.8#1",
        2,
        10,
        Exact,
        "2.2500",
        "0x2.40#10",
        Equal,
    );
    // - x^n-fits escape, all inner branches. x = 3*2^20, so x^2 = 9*2^40 has a 4-bit significand
    //   and (1+x)^2 is within 2^-20 of it
    // - x^n fits and needs exactly p bits with Nearest: round up
    test(
        "3.1e6",
        "0x3.0E+5#2",
        2,
        3,
        Nearest,
        "1.1e13",
        "0xa.0E+10#3",
        Greater,
    );
    // - x^n fits, directed rounding up
    test(
        "3.1e6",
        "0x3.0E+5#2",
        2,
        5,
        Up,
        "1.04e13",
        "0x9.8E+10#5",
        Greater,
    );
    // - x^n fits, Nearest with x^n at fewer than p bits: round down
    test(
        "3.1e6",
        "0x3.0E+5#2",
        2,
        5,
        Nearest,
        "9.90e12",
        "0x9.0E+10#5",
        Less,
    );
    // - x^n does not fit into p bits (pow_u inexact): fall through to the Ziv loop
    test(
        "3.1e6",
        "0x3.0E+5#2",
        2,
        3,
        Floor,
        "8.8e12",
        "0x8.0E+10#3",
        Less,
    );
    // - x^n fits but (1+1/x)^n - 1 is not below 2^-py: fall through to the exact-1+x escape
    test(
        "3.1e6",
        "0x3.0E+5#2",
        2,
        30,
        Nearest,
        "9.8956109414e12",
        "0x9.0000600E+10#30",
        Less,
    );
    // - x is a large odd integer (kx == ex): the x^n-fits check does not apply, and the Ziv loop
    //   converges on its second iteration
    test(
        "1048577.0",
        "0x100001.0#21",
        2,
        4,
        Nearest,
        "1.10e12",
        "0x1.0E+10#4",
        Less,
    );
    // - near-one, Nearest with a negative n*log2(1+x): 1 with ternary Greater
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        -3,
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    // - near-one, rounding toward 1 from above (Down with positive n*log2(1+x))
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        3,
        20,
        Down,
        "1.0000000",
        "0x1.00000#20",
        Less,
    );
    // - near-one, rounding toward 1 from below (Up with negative n*log2(1+x))
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        -3,
        20,
        Up,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    // - overflow
    test(
        "1000.0",
        "0x3e8.0#7",
        1073741824,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#7",
        1073741824,
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    // - regression: MPFR 4.2.2's inverted log2p1 rounding direction for negative n made its
    //   min_prec escape step the wrong way here, giving 0xf.fffffffcE-99; the correct Down rounding
    //   is 0x1.000000000E-98, since (1 + x)^-4 > 2^-392 exactly
    test(
        "316912650057056787424759234560.000003695491614053019065",
        "0x3fffffffffffe00001fffc000.00003e0003ffffffe00#174",
        -4,
        34,
        Down,
        "9.91383530201e-119",
        "0x1.000000000E-98#34",
        Less,
    );
    // - regression: minimal reproducer for the same MPFR 4.2.2 bug. 1/127 = 0.007874..., whose
    //   2-bit neighbors are 2^-7 = 0.0078125 and 3*2^-8 = 0.0117. 4.2.2 returns 2^-7 for Up (below
    //   the true value, an illegal upward rounding) and (2^-7, Greater) for Nearest (correct value,
    //   wrong ternary)
    test("126.0", "0x7e.0#7", -1, 2, Up, "0.012", "0x0.03#2", Greater);
    test(
        "126.0", "0x7e.0#7", -1, 2, Nearest, "0.0078", "0x0.02#2", Less,
    );
    // - regression: the same MPFR bug can produce the correct value with the wrong ternary (MPFR
    //   says Greater here; exact rational arithmetic confirms Less)
    test(
        "7.27595761418342590332031250000000280259692864963414184745916657983226252757951518e-12",
        "0x8.000000000000000000000000000fffffffffffffffffffffffffffffffc0000000E-10#265",
        -1,
        113,
        Nearest,
        "0.999999999992724042385869513655882696",
        "0x0.fffffffff8000000003ffffffffe0#113",
        Less,
    );
    // - underflow
    test(
        "1000.0",
        "0x3e8.0#7",
        -1073741824,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#7",
        -1073741824,
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
}

#[test]
fn test_compound_special_values() {
    let test = |x: Float, n: i64, out: &str, out_hex: &str, o_out| {
        let (c, o) = x.compound_prec_round(n, 1, Nearest);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);
    };
    test(Float::NAN, 0, "1.0", "0x1.0#1", Equal);
    test(Float::NAN, 2, "NaN", "NaN", Equal);
    test(Float::NAN, -2, "NaN", "NaN", Equal);
    test(Float::INFINITY, 0, "1.0", "0x1.0#1", Equal);
    test(Float::INFINITY, 2, "Infinity", "Infinity", Equal);
    test(Float::INFINITY, -2, "0.0", "0x0.0", Equal);
    test(Float::NEGATIVE_INFINITY, 0, "NaN", "NaN", Equal);
    test(Float::NEGATIVE_INFINITY, 2, "NaN", "NaN", Equal);
    test(Float::NEGATIVE_INFINITY, -2, "NaN", "NaN", Equal);
    test(Float::ZERO, 0, "1.0", "0x1.0#1", Equal);
    test(Float::ZERO, 2, "1.0", "0x1.0#1", Equal);
    test(Float::ZERO, -2, "1.0", "0x1.0#1", Equal);
    test(Float::NEGATIVE_ZERO, 0, "1.0", "0x1.0#1", Equal);
    test(Float::NEGATIVE_ZERO, 2, "1.0", "0x1.0#1", Equal);
    test(Float::NEGATIVE_ZERO, -2, "1.0", "0x1.0#1", Equal);
    test(Float::ONE, 0, "1.0", "0x1.0#1", Equal);
    test(Float::ONE, 2, "4.0", "0x4.0#1", Equal);
    test(Float::ONE, -2, "0.25", "0x0.4#1", Equal);
    test(Float::NEGATIVE_ONE, 0, "1.0", "0x1.0#1", Equal);
    test(Float::NEGATIVE_ONE, 2, "0.0", "0x0.0", Equal);
    test(Float::NEGATIVE_ONE, -2, "Infinity", "Infinity", Equal);
}

#[test]
fn test_compound_extreme() {
    let max_e = i64::from(Float::MAX_EXPONENT);
    let min_e = i64::from(Float::MIN_EXPONENT);
    let test = |x: Float, n: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (c, o) = x.compound_prec_round(n, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);
    };
    // x = -(1 - 2^-100), so 1 + x = 2^-100 exactly
    let (x, o_c) = Float::NEGATIVE_ONE.add_prec(Float::power_of_2(-100i64), 101);
    assert_eq!(o_c, Equal);
    test(
        Float::power_of_2(max_e - 1),
        2,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        Float::power_of_2(max_e - 1),
        1,
        10,
        Nearest,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Less,
    );
    test(
        Float::power_of_2(max_e - 1),
        -1,
        10,
        Nearest,
        "9.5303e-323228497",
        "0x4.00E-268435456#10",
        Greater,
    );
    test(
        Float::power_of_2(min_e),
        5,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        Float::power_of_2(min_e),
        5,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        -Float::power_of_2(min_e),
        5,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        x.clone(),
        100000,
        10,
        Nearest,
        "1.1050e-3010300",
        "0x1.000E-2500000#10",
        Equal,
    );
    test(
        x.clone(),
        -100000,
        10,
        Nearest,
        "9.0498e3010299",
        "0x1.000E+2500000#10",
        Equal,
    );
    test(x.clone(), 33554432, 10, Nearest, "0.0", "0x0.0", Less);
    test(
        x.clone(),
        33554432,
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        x.clone(),
        -33554432,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        x.clone(),
        -33554432,
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    // With x = 2^(2^29), u = -2 * log2(1+x) truncates to exactly MIN_EXPONENT - 1 and the
    // intermediate t is the minimum positive Float, so the min_prec escape cannot step below it and
    // resolves the rounding directly. (The results are too precise to compare as strings.)
    let x = Float::power_of_2(536870912i64);
    let prec = 1u64 << 28;
    for (rm, o_out) in [(Nearest, Greater), (Ceiling, Greater), (Up, Greater)] {
        let (c, o) = x.compound_prec_round_ref(-2, prec, rm);
        assert!(c.is_valid());
        assert_eq!(
            ComparableFloatRef(&c),
            ComparableFloatRef(&Float::min_positive_value_prec(prec))
        );
        assert_eq!(o, o_out);
    }
    for rm in [Floor, Down] {
        let (c, o) = x.compound_prec_round_ref(-2, prec, rm);
        assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Less);
    }
}

#[test]
fn compound_prec_round_fail() {
    assert_panic!(Float::ONE.compound_prec_round(2, 0, Nearest));
    assert_panic!(Float::ONE.compound_prec_round_ref(2, 0, Nearest));
    assert_panic!({
        let mut x = Float::ONE;
        x.compound_prec_round_assign(2, 0, Nearest);
    });
    // (1 + 1/2)^2 = 9/4 is not exactly representable with precision 2
    assert_panic!(
        Float::from_primitive_float_prec(0.5, 1)
            .0
            .compound_prec_round(2, 2, Exact)
    );
    // Exact rounding requested, but the result overflows
    assert_panic!(Float::from(1000u32).compound_prec_round(1 << 30, 5, Exact));
    // Exact rounding requested, but the result underflows
    assert_panic!(Float::from(1000u32).compound_prec_round(-(1 << 30), 5, Exact));
    // Exact rounding requested, but the result is within a quarter-ulp of 1
    assert_panic!(Float::power_of_2(-100i64).compound_prec_round(3, 20, Exact));
}

// Compares a compound result against MPFR's, via rug. MPFR 4.2.2's mpfr_compound_si rounds log2p1
// in a direction that is backwards for negative n (fixed in the MPFR development sources and in our
// port), so for n < 0 MPFR may occasionally return the neighboring value; in that case its result
// must be exactly one step away from ours, and the ternary values are incomparable.
fn check_rug_compound(
    x: &Float,
    c: &Float,
    o: Ordering,
    rug_c: &rug::Float,
    rug_o: Ordering,
    n: i64,
) {
    let rug_f = Float::from(rug_c);
    if ComparableFloatRef(&rug_f) == ComparableFloatRef(c) {
        // The same bug can also produce the correct value with the wrong ternary (verified against
        // exact rational arithmetic on our side).
        if rug_o != o {
            assert!(
                n < 0,
                "ternary disagreement for x = {x:#x}, n = {n}, c = {c:#x}"
            );
            assert_ne!(rug_o, Equal);
            assert_ne!(o, Equal);
        }
    } else {
        assert!(n < 0);
        if c.is_finite() && !c.is_zero() && rug_f.is_finite() && !rug_f.is_zero() {
            let mut stepped = rug_f;
            if stepped < *c {
                stepped.increment();
            } else {
                stepped.decrement();
            }
            assert_eq!(ComparableFloatRef(&stepped), ComparableFloatRef(c));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn compound_prec_round_properties_helper(
    x: Float,
    n: i64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        let (c, o) = x.compound_prec_round_ref(n, prec, Nearest);
        if o == Equal {
            let (ce, oe) = x.compound_prec_round_ref(n, prec, Exact);
            assert_eq!(ComparableFloatRef(&ce), ComparableFloatRef(&c));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.compound_prec_round_ref(n, prec, Exact));
        }
        return;
    }
    let (c, o) = x.clone().compound_prec_round(n, prec, rm);
    assert!(c.is_valid());
    let (c_alt, o_alt) = x.compound_prec_round_ref(n, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.compound_prec_round_assign(n, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if c.is_normal() && !extreme {
        assert_eq!(c.get_prec(), Some(prec));
    }

    // compound (mpfr_compound_si) must agree with MPFR via rug.
    if i32::convertible_from(n)
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_c, rug_o) = rug_compound_prec_round(
            &rug::Float::exact_from(&x),
            i32::exact_from(n),
            prec,
            rug_rm,
        );
        check_rug_compound(&x, &c, o, &rug_c, rug_o, n);
    }

    // When 1 + x is exactly representable, compound agrees with pow_s applied to 1 + x.
    if x.is_finite()
        && !x.is_zero()
        && x.partial_cmp(&-1i32) == Some(Greater)
        && x.get_exponent().unwrap().unsigned_abs() <= 4096
        && x.get_prec().unwrap() <= 4096
    {
        let (s, o_s) = x.add_prec_round_ref_val(Float::ONE, x.get_prec().unwrap() + 4200, Floor);
        if o_s == Equal && !s.is_zero() {
            let (p, o_p) = s.pow_s_prec_round(n, prec, rm);
            assert_eq!(ComparableFloatRef(&p), ComparableFloatRef(&c));
            assert_eq!(o_p, o);
        }
    }

    // Exact-Rational cross-check for small cases: compound(x, n) = (1 + x)^n.
    if x.is_finite()
        && !x.is_zero()
        && n != 0
        && n.unsigned_abs() <= 32
        && x.get_exponent().unwrap().unsigned_abs() <= 2048
        && x.get_prec().unwrap() <= 1000
        && x.partial_cmp(&-1i32) == Some(Greater)
    {
        let exact = (Rational::ONE + Rational::exact_from(&x)).pow(n);
        let (cr, or) = Float::from_rational_prec_round(exact, prec, rm);
        assert_eq!(ComparableFloatRef(&cr), ComparableFloatRef(&c));
        assert_eq!(or, o);
    }
}

#[test]
fn compound_prec_round_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, n, prec, rm)| {
            compound_prec_round_properties_helper(x, n, prec, rm, false);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, n, prec, rm)| {
            compound_prec_round_properties_helper(x, n, prec, rm, true);
        },
    );
}

#[test]
fn compound_prec_properties() {
    float_signed_unsigned_triple_gen_var_1::<i64, u64>().test_properties(|(x, n, prec)| {
        let (c, o) = x.clone().compound_prec(n, prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.compound_prec_ref(n, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.compound_prec_round_ref(n, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.compound_prec_assign(n, prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        if i32::convertible_from(n) {
            let (rug_c, rug_o) =
                rug_compound_prec(&rug::Float::exact_from(&x), i32::exact_from(n), prec);
            check_rug_compound(&x, &c, o, &rug_c, rug_o, n);
        }
    });
}

#[test]
fn compound_round_properties() {
    float_signed_pair_gen::<i64>().test_properties(|(x, n)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (c, o) = x.clone().compound_round(n, rm);
            assert!(c.is_valid());
            let (c_alt, o_alt) = x.compound_round_ref(n, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.compound_round_assign(n, rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            if i32::convertible_from(n)
                && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            {
                let (rug_c, rug_o) =
                    rug_compound_round(&rug::Float::exact_from(&x), i32::exact_from(n), rug_rm);
                check_rug_compound(&x, &c, o, &rug_c, rug_o, n);
            }
        }
    });
}

#[test]
fn compound_properties() {
    float_signed_pair_gen::<i64>().test_properties(|(x, n)| {
        let c = x.clone().compound(n);
        assert!(c.is_valid());
        let c_alt = (&x).compound(n);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        let mut x_alt = x.clone();
        x_alt.compound_assign(n);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

        let (c_alt, _) = x.compound_round_ref(n, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        if i32::convertible_from(n) {
            let rug_c = rug_compound(&rug::Float::exact_from(&x), i32::exact_from(n));
            check_rug_compound(&x, &c, Equal, &rug_c, Equal, n);
        }

        // compound(x, 0) = 1 unless x < -1 or x == -Inf
        if n == 0 && !(x == Float::NEGATIVE_INFINITY || x.partial_cmp(&-1i32) == Some(Less)) {
            assert_eq!(
                ComparableFloatRef(&c),
                ComparableFloatRef(&Float::one_prec(x.significant_bits()))
            );
        }

        // compound(x, 1) = 1 + x, unless x < -1 or x == -Inf (where compound is NaN)
        if n == 1 && x != Float::NEGATIVE_INFINITY && x.partial_cmp(&-1i32) != Some(Less) {
            let prec = x.significant_bits();
            let (s, o_s) = x.add_prec_round_ref_val(Float::ONE, prec, Nearest);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            let _ = o_s;
        }

        // compound(0, n) = 1
        if x.is_zero() {
            assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&Float::ONE));
        }
    });
}

#[test]
fn test_primitive_float_compound() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, n: i64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_compound(x, n)), NiceFloat(out));
    }
    // - special cases, mirroring the compound table
    test::<f32>(f32::NAN, 0, 1.0);
    test::<f32>(f32::NAN, 2, f32::NAN);
    test::<f32>(f32::NAN, -2, f32::NAN);
    test::<f32>(f32::INFINITY, 0, 1.0);
    test::<f32>(f32::INFINITY, 2, f32::INFINITY);
    test::<f32>(f32::INFINITY, -2, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 0, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 2, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, -2, f32::NAN);
    test::<f32>(0.0, 0, 1.0);
    test::<f32>(0.0, 2, 1.0);
    test::<f32>(0.0, -2, 1.0);
    test::<f32>(-0.0, 0, 1.0);
    test::<f32>(-0.0, 2, 1.0);
    test::<f32>(-0.0, -2, 1.0);
    test::<f32>(-1.0, 0, 1.0);
    test::<f32>(-1.0, 2, 0.0);
    test::<f32>(-1.0, -2, f32::INFINITY);
    // - x < -1 is NaN, even for n == 0
    test::<f32>(-2.0, 0, f32::NAN);
    test::<f32>(-2.0, 2, f32::NAN);
    test::<f32>(-2.0, -2, f32::NAN);
    // - finite cases
    test::<f32>(0.5, 2, 2.25);
    test::<f32>(0.1, 10, 2.5937426);
    test::<f32>(2.0, -3, 0.037037037);
    // - overflow and underflow
    test::<f32>(1.0, 130, f32::INFINITY);
    test::<f32>(1.0, -150, 0.0);
    test::<f64>(0.5, 2, 2.25);
    test::<f64>(0.1, 10, 2.5937424601);
    test::<f64>(0.1, -10, 0.38554328942953175);
    test::<f64>(-0.5, -2, 4.0);
    test::<f64>(6.0, 500, f64::INFINITY);
    test::<f64>(-0.99999, 100000, 0.0);
    // - (1+1)^-1074 = 2^-1074, the smallest positive subnormal
    test::<f64>(1.0, -1074, 5.0e-324);
    test::<f64>(1.0, -1080, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_compound_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    T: RoundingFrom<Rational>,
    Rational: ExactFrom<T>,
{
    primitive_float_signed_pair_gen_var_4::<T, i64>().test_properties(|(x, n)| {
        let c = primitive_float_compound::<T>(x, n);
        if x.is_finite() {
            if x <= T::NEGATIVE_ONE {
                if x < T::NEGATIVE_ONE {
                    assert!(c.is_nan());
                }
            } else if n.unsigned_abs() <= 24 {
                let exact = (Rational::ONE + Rational::exact_from(x)).pow(n);
                let (c_alt, _) = T::rounding_from(exact, Nearest);
                assert_eq!(NiceFloat(c_alt), NiceFloat(c));
            }
        }
    });
}

#[test]
fn primitive_float_compound_properties() {
    apply_fn_to_primitive_floats!(primitive_float_compound_properties_helper);
}
