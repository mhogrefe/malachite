// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Cos, CosAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::cos::{primitive_float_cos, primitive_float_cos_rational};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::cos::{
    rug_cos, rug_cos_prec, rug_cos_prec_round, rug_cos_rational_prec, rug_cos_rational_prec_round,
    rug_cos_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_cos_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().cos_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.cos_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.cos_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_cos_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0.0", "0x0.0", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 10, Floor, "0.54004", "0x0.8a4#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.54102",
        "0x0.8a8#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Nearest, "0.54004", "0x0.8a4#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.54030230586813971740093660744256",
        "0x0.8a51407da8345c91c2466d976#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "0.54004",
        "0x0.8a4#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-0.41602",
        "-0x0.6a8#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "-0.99023",
        "-0x0.fd8#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-0.65332",
        "-0x0.a74#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "-0.65364362086361191463916818309786",
        "-0x0.a7553036d926062336d0e16e4#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "0.86230",
        "0x0.dcc#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "0.86231887228768393410193851395020",
        "0x0.dcc0edfb32fefb1fa19b9b30b#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "0.86231887228768393410193851395099",
        "0x0.dcc0edfb32fefb1fa19b9b30c#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "0.936752127533144786917",
        "0x0.efcefcc836996357#64",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "0.87758256189037276",
        "0x0.e0a94032dbea8#50",
        Greater,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "0.99484696102354064",
        "0x0.feae4a5a1f000#50",
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        50,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#50",
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.570796326794896600",
        "0x1.921fb54442d183#57",
        53,
        Nearest,
        "1.9598976533924290e-17",
        "0x1.69898cc51701cE-14#53",
        Greater,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "6.28318530717958579",
        "0x6.487ed5110b45e#54",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "9.979e99",
        "0x1.24E+83#7",
        53,
        Nearest,
        "-0.42980316367459315",
        "-0x0.6e079483b32060#53",
        Greater,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("3.0", "0x3.0#2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "-1.0", "-0x1.0#2", Less);
    // - tiny x: the small-input shortcut rounds 1 - x^2/2 directly
    test("0.25", "0x0.4#1", 1, Down, "0.50", "0x0.8#1", Less);
    // - general path: the first working precision can round
    test("1.0", "0x1.0#1", 1, Down, "0.50", "0x0.8#1", Less);
    // - cancellation: cos(x) is much smaller than 1, so the working precision grows by the lost
    //   bits
    test("2.0", "0x2.0#1", 1, Down, "-0.25", "-0x0.4#1", Greater);
    // - |x| >= 4: argument reduction modulo 2 pi
    test("4.0", "0x4.0#1", 1, Down, "-0.50", "-0x0.8#1", Greater);
    // - the sum rounds to exactly 1 at the working precision, and the error bound alone decides the
    //   rounding
    test(
        "-3.495934488151859089160804055e56",
        "-0xe.41ed086a5791d9e5b2924E+46#87",
        2,
        Down,
        "0.75",
        "0x0.c#2",
        Less,
    );
    // - x equals the working-precision 2 pi, so the reduced argument is zero and the loop retries
    //   at a higher precision
    test(
        "6.28318536",
        "0x6.487ed6#26",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    // - x = pi / 2 rounded to 53 bits: cos(x) is about 2^-54, found after growing the precision
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        53,
        Nearest,
        "6.1232339957367660e-17",
        "0x4.69898cc51701cE-14#53",
        Greater,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "5.7099684971243490026437400060191e-62",
        "0x1.77d4c76273644a29410f31c68E-51#100",
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "2.3305317247657495256428231620634736190e-36",
        "0x3.1909f22bccc1913547f3b9881a9d8cE-30#120",
        Greater,
    );
    // - x = pi rounded to 64 bits: cos(x) is within 2^-128 of -1
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
    );
}

#[test]
#[should_panic]
fn cos_prec_round_fail() {
    Float::ONE.cos_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn cos_prec_round_exact_fail() {
    Float::ONE.cos_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn cos_prec_fail() {
    Float::ONE.cos_prec(0);
}

#[test]
#[should_panic]
fn cos_round_fail() {
    Float::ONE.cos_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn cos_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().cos_prec_round(prec, rm);
    assert!(c.is_valid());

    let (c_alt, o_alt) = x.cos_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.cos_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_cos_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // cos is even
    let (c_neg, o_neg) = (-&x).cos_prec_round(prec, rm);
    assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&c));
    assert_eq!(o_neg, o);

    // |cos x| <= 1
    if c.is_finite() {
        assert!(c.le_abs(&1u32));
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // cos is exact only for x = 0 (and NaN, and ±inf): the result is rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.cos_prec_round_ref(prec, rm2);
            assert_eq!(
                ComparableFloat(c2.abs_negative_zero_ref()),
                ComparableFloat(c.abs_negative_zero_ref())
            );
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.cos_prec_round_ref(prec, Exact));
    }
}

#[test]
fn cos_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        cos_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::NAN.cos_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);

        let (c, o) = Float::INFINITY.cos_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);

        let (c, o) = Float::NEGATIVE_INFINITY.cos_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);

        let (c, o) = Float::ZERO.cos_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);

        let (c, o) = Float::NEGATIVE_ZERO.cos_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
    });
}

#[test]
fn cos_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().cos_round(rm);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.cos_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.cos_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.cos_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_cos_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn cos_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().cos_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.cos_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.cos_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.cos_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_cos_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn cos_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().cos();
        assert!(c.is_valid());
        let c_alt = (&x).cos();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        let mut x_alt = x.clone();
        x_alt.cos_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

        let (c_alt, _) = x.cos_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_cos(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&c)
        );

        assert_eq!(ComparableFloatRef(&(-&x).cos()), ComparableFloatRef(&c));
        if c.is_finite() {
            assert!(c.le_abs(&1u32));
        }
    });
}

// n * pi / 2, with pi rounded to the nearest `prec` bits and the product exact
fn odd_multiple_of_half_pi(n: i64, prec: u64) -> Float {
    Float::pi_prec(prec)
        .0
        .mul_prec_round(Float::from(n), prec + 64, Exact)
        .0
        >> 1u32
}

// Inputs close to an odd multiple of pi/2, where the cosine is tiny and takes the near-zero path: n
// * pi / 2 with pi rounded to `prec_x` bits. The rows with `prec_x` at most 100 and a large output
// precision need several terms of the sin(delta) / delta series.
#[test]
fn test_cos_near_zero() {
    let test = |n: i64, prec_x: u64, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = odd_multiple_of_half_pi(n, prec_x);
        let (c, o) = x.cos_prec_round_ref(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (rug_c, rug_o) = rug_cos_prec_round(
            &rug::Float::exact_from(&x),
            prec,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    };
    test(1, 150, 1, Up, "-7.0e-46", "-0x4.0E-38#1", Less);
    test(1, 150, 1, Floor, "-7.0e-46", "-0x4.0E-38#1", Less);
    test(1, 150, 1, Ceiling, "-3.5e-46", "-0x2.0E-38#1", Greater);
    test(1, 150, 1, Nearest, "-7.0e-46", "-0x4.0E-38#1", Less);
    test(1, 150, 10, Down, "-6.8833e-46", "-0x3.eeE-38#10", Greater);
    test(1, 150, 10, Up, "-6.8902e-46", "-0x3.efE-38#10", Less);
    test(1, 150, 10, Floor, "-6.8902e-46", "-0x3.efE-38#10", Less);
    test(
        1,
        150,
        10,
        Ceiling,
        "-6.8833e-46",
        "-0x3.eeE-38#10",
        Greater,
    );
    test(1, 150, 10, Nearest, "-6.8902e-46", "-0x3.efE-38#10", Less);
    test(
        1,
        150,
        53,
        Down,
        "-6.8896173743304411e-46",
        "-0x3.eeeb306717fbeE-38#53",
        Greater,
    );
    test(
        1,
        150,
        53,
        Up,
        "-6.8896173743304419e-46",
        "-0x3.eeeb306717fc0E-38#53",
        Less,
    );
    test(
        1,
        150,
        53,
        Floor,
        "-6.8896173743304419e-46",
        "-0x3.eeeb306717fc0E-38#53",
        Less,
    );
    test(
        1,
        150,
        53,
        Ceiling,
        "-6.8896173743304411e-46",
        "-0x3.eeeb306717fbeE-38#53",
        Greater,
    );
    test(
        1,
        150,
        53,
        Nearest,
        "-6.8896173743304411e-46",
        "-0x3.eeeb306717fbeE-38#53",
        Greater,
    );
    test(
        1,
        150,
        100,
        Nearest,
        "-6.8896173743304413174294781085918e-46",
        "-0x3.eeeb306717fbe882b389d8c9cE-38#100",
        Less,
    );
    test(1, 190, 1, Down, "2.5e-60", "0x4.0E-50#1", Less);
    test(1, 190, 1, Up, "5.0e-60", "0x8.0E-50#1", Greater);
    test(1, 190, 1, Floor, "2.5e-60", "0x4.0E-50#1", Less);
    test(1, 190, 1, Ceiling, "5.0e-60", "0x8.0E-50#1", Greater);
    test(1, 190, 1, Nearest, "2.5e-60", "0x4.0E-50#1", Less);
    test(1, 190, 10, Down, "2.5427e-60", "0x4.16E-50#10", Less);
    test(1, 190, 10, Up, "2.5475e-60", "0x4.18E-50#10", Greater);
    test(1, 190, 10, Floor, "2.5427e-60", "0x4.16E-50#10", Less);
    test(1, 190, 10, Ceiling, "2.5475e-60", "0x4.18E-50#10", Greater);
    test(1, 190, 10, Nearest, "2.5475e-60", "0x4.18E-50#10", Greater);
    test(
        1,
        190,
        53,
        Down,
        "2.5463057961157001e-60",
        "0x4.177d4c7627364E-50#53",
        Less,
    );
    test(
        1,
        190,
        53,
        Up,
        "2.5463057961157007e-60",
        "0x4.177d4c7627368E-50#53",
        Greater,
    );
    test(
        1,
        190,
        53,
        Floor,
        "2.5463057961157001e-60",
        "0x4.177d4c7627364E-50#53",
        Less,
    );
    test(
        1,
        190,
        53,
        Ceiling,
        "2.5463057961157007e-60",
        "0x4.177d4c7627368E-50#53",
        Greater,
    );
    test(
        1,
        190,
        53,
        Nearest,
        "2.5463057961157001e-60",
        "0x4.177d4c7627364E-50#53",
        Less,
    );
    test(
        1,
        190,
        100,
        Nearest,
        "2.5463057961157001728840630215730e-60",
        "0x4.177d4c76273644a29410f31c8E-50#100",
        Greater,
    );
    test(1, 200, 1, Down, "3.9e-62", "0x1.0E-51#1", Less);
    test(1, 200, 1, Up, "7.8e-62", "0x2.0E-51#1", Greater);
    test(1, 200, 1, Floor, "3.9e-62", "0x1.0E-51#1", Less);
    test(1, 200, 1, Ceiling, "7.8e-62", "0x2.0E-51#1", Greater);
    test(1, 200, 1, Nearest, "3.9e-62", "0x1.0E-51#1", Less);
    test(1, 200, 10, Down, "5.7049e-62", "0x1.778E-51#10", Less);
    test(1, 200, 10, Up, "5.7125e-62", "0x1.780E-51#10", Greater);
    test(1, 200, 10, Floor, "5.7049e-62", "0x1.778E-51#10", Less);
    test(1, 200, 10, Ceiling, "5.7125e-62", "0x1.780E-51#10", Greater);
    test(1, 200, 10, Nearest, "5.7125e-62", "0x1.780E-51#10", Greater);
    test(
        1,
        200,
        53,
        Down,
        "5.7099684971243485e-62",
        "0x1.77d4c76273644E-51#53",
        Less,
    );
    test(
        1,
        200,
        53,
        Up,
        "5.7099684971243493e-62",
        "0x1.77d4c76273645E-51#53",
        Greater,
    );
    test(
        1,
        200,
        53,
        Floor,
        "5.7099684971243485e-62",
        "0x1.77d4c76273644E-51#53",
        Less,
    );
    test(
        1,
        200,
        53,
        Ceiling,
        "5.7099684971243493e-62",
        "0x1.77d4c76273645E-51#53",
        Greater,
    );
    test(
        1,
        200,
        53,
        Nearest,
        "5.7099684971243493e-62",
        "0x1.77d4c76273645E-51#53",
        Greater,
    );
    test(
        1,
        200,
        100,
        Nearest,
        "5.7099684971243490026437400060191e-62",
        "0x1.77d4c76273644a29410f31c68E-51#100",
        Less,
    );
    test(3, 200, 1, Down, "-1.6e-61", "-0x4.0E-51#1", Greater);
    test(3, 200, 1, Up, "-3.1e-61", "-0x8.0E-51#1", Less);
    test(3, 200, 1, Floor, "-3.1e-61", "-0x8.0E-51#1", Less);
    test(3, 200, 1, Ceiling, "-1.6e-61", "-0x4.0E-51#1", Greater);
    test(3, 200, 1, Nearest, "-1.6e-61", "-0x4.0E-51#1", Greater);
    test(3, 200, 10, Down, "-1.7107e-61", "-0x4.66E-51#10", Greater);
    test(3, 200, 10, Up, "-1.7138e-61", "-0x4.68E-51#10", Less);
    test(3, 200, 10, Floor, "-1.7138e-61", "-0x4.68E-51#10", Less);
    test(
        3,
        200,
        10,
        Ceiling,
        "-1.7107e-61",
        "-0x4.66E-51#10",
        Greater,
    );
    test(3, 200, 10, Nearest, "-1.7138e-61", "-0x4.68E-51#10", Less);
    test(
        3,
        200,
        53,
        Down,
        "-1.7129905491373045e-61",
        "-0x4.677e56275a2ccE-51#53",
        Greater,
    );
    test(
        3,
        200,
        53,
        Up,
        "-1.7129905491373049e-61",
        "-0x4.677e56275a2d0E-51#53",
        Less,
    );
    test(
        3,
        200,
        53,
        Floor,
        "-1.7129905491373049e-61",
        "-0x4.677e56275a2d0E-51#53",
        Less,
    );
    test(
        3,
        200,
        53,
        Ceiling,
        "-1.7129905491373045e-61",
        "-0x4.677e56275a2ccE-51#53",
        Greater,
    );
    test(
        3,
        200,
        53,
        Nearest,
        "-1.7129905491373045e-61",
        "-0x4.677e56275a2ccE-51#53",
        Greater,
    );
    test(
        3,
        200,
        100,
        Nearest,
        "-1.7129905491373047007931220018057e-61",
        "-0x4.677e56275a2cde7bc32d95538E-51#100",
        Greater,
    );
    test(-1, 200, 1, Down, "3.9e-62", "0x1.0E-51#1", Less);
    test(-1, 200, 1, Up, "7.8e-62", "0x2.0E-51#1", Greater);
    test(-1, 200, 1, Floor, "3.9e-62", "0x1.0E-51#1", Less);
    test(-1, 200, 1, Ceiling, "7.8e-62", "0x2.0E-51#1", Greater);
    test(-1, 200, 1, Nearest, "3.9e-62", "0x1.0E-51#1", Less);
    test(-1, 200, 10, Down, "5.7049e-62", "0x1.778E-51#10", Less);
    test(-1, 200, 10, Up, "5.7125e-62", "0x1.780E-51#10", Greater);
    test(-1, 200, 10, Floor, "5.7049e-62", "0x1.778E-51#10", Less);
    test(
        -1,
        200,
        10,
        Ceiling,
        "5.7125e-62",
        "0x1.780E-51#10",
        Greater,
    );
    test(
        -1,
        200,
        10,
        Nearest,
        "5.7125e-62",
        "0x1.780E-51#10",
        Greater,
    );
    test(
        -1,
        200,
        53,
        Down,
        "5.7099684971243485e-62",
        "0x1.77d4c76273644E-51#53",
        Less,
    );
    test(
        -1,
        200,
        53,
        Up,
        "5.7099684971243493e-62",
        "0x1.77d4c76273645E-51#53",
        Greater,
    );
    test(
        -1,
        200,
        53,
        Floor,
        "5.7099684971243485e-62",
        "0x1.77d4c76273644E-51#53",
        Less,
    );
    test(
        -1,
        200,
        53,
        Ceiling,
        "5.7099684971243493e-62",
        "0x1.77d4c76273645E-51#53",
        Greater,
    );
    test(
        -1,
        200,
        53,
        Nearest,
        "5.7099684971243493e-62",
        "0x1.77d4c76273645E-51#53",
        Greater,
    );
    test(
        -1,
        200,
        100,
        Nearest,
        "5.7099684971243490026437400060191e-62",
        "0x1.77d4c76273644a29410f31c68E-51#100",
        Less,
    );
    test(5, 300, 1, Down, "9.8e-91", "0x2.0E-75#1", Less);
    test(5, 300, 1, Up, "2.0e-90", "0x4.0E-75#1", Greater);
    test(5, 300, 1, Floor, "9.8e-91", "0x2.0E-75#1", Less);
    test(5, 300, 1, Ceiling, "2.0e-90", "0x4.0E-75#1", Greater);
    test(5, 300, 1, Nearest, "9.8e-91", "0x2.0E-75#1", Less);
    test(5, 300, 10, Down, "1.2330e-90", "0x2.83E-75#10", Less);
    test(5, 300, 10, Up, "1.2349e-90", "0x2.84E-75#10", Greater);
    test(5, 300, 10, Floor, "1.2330e-90", "0x2.83E-75#10", Less);
    test(5, 300, 10, Ceiling, "1.2349e-90", "0x2.84E-75#10", Greater);
    test(5, 300, 10, Nearest, "1.2330e-90", "0x2.83E-75#10", Less);
    test(
        5,
        300,
        53,
        Down,
        "1.2331064348210500e-90",
        "0x2.830ab5bd30106E-75#53",
        Less,
    );
    test(
        5,
        300,
        53,
        Up,
        "1.2331064348210502e-90",
        "0x2.830ab5bd30108E-75#53",
        Greater,
    );
    test(
        5,
        300,
        53,
        Floor,
        "1.2331064348210500e-90",
        "0x2.830ab5bd30106E-75#53",
        Less,
    );
    test(
        5,
        300,
        53,
        Ceiling,
        "1.2331064348210502e-90",
        "0x2.830ab5bd30108E-75#53",
        Greater,
    );
    test(
        5,
        300,
        53,
        Nearest,
        "1.2331064348210500e-90",
        "0x2.830ab5bd30106E-75#53",
        Less,
    );
    test(
        5,
        300,
        100,
        Nearest,
        "1.2331064348210500019313688799045e-90",
        "0x2.830ab5bd3010604469f0fe1b0E-75#100",
        Greater,
    );
    test(-3, 300, 1, Down, "-4.9e-91", "-0x1.0E-75#1", Greater);
    test(-3, 300, 1, Up, "-9.8e-91", "-0x2.0E-75#1", Less);
    test(-3, 300, 1, Floor, "-9.8e-91", "-0x2.0E-75#1", Less);
    test(-3, 300, 1, Ceiling, "-4.9e-91", "-0x1.0E-75#1", Greater);
    test(-3, 300, 1, Nearest, "-9.8e-91", "-0x2.0E-75#1", Less);
    test(-3, 300, 10, Down, "-7.3924e-91", "-0x1.818E-75#10", Greater);
    test(-3, 300, 10, Up, "-7.4020e-91", "-0x1.820E-75#10", Less);
    test(-3, 300, 10, Floor, "-7.4020e-91", "-0x1.820E-75#10", Less);
    test(
        -3,
        300,
        10,
        Ceiling,
        "-7.3924e-91",
        "-0x1.818E-75#10",
        Greater,
    );
    test(-3, 300, 10, Nearest, "-7.4020e-91", "-0x1.820E-75#10", Less);
    test(
        -3,
        300,
        53,
        Down,
        "-7.3986386089262991e-91",
        "-0x1.81d339d7e9a36E-75#53",
        Greater,
    );
    test(
        -3,
        300,
        53,
        Up,
        "-7.3986386089263002e-91",
        "-0x1.81d339d7e9a37E-75#53",
        Less,
    );
    test(
        -3,
        300,
        53,
        Floor,
        "-7.3986386089263002e-91",
        "-0x1.81d339d7e9a37E-75#53",
        Less,
    );
    test(
        -3,
        300,
        53,
        Ceiling,
        "-7.3986386089262991e-91",
        "-0x1.81d339d7e9a36E-75#53",
        Greater,
    );
    test(
        -3,
        300,
        53,
        Nearest,
        "-7.3986386089263002e-91",
        "-0x1.81d339d7e9a37E-75#53",
        Less,
    );
    test(
        -3,
        300,
        100,
        Nearest,
        "-7.3986386089263000115882132794221e-91",
        "-0x1.81d339d7e9a36cf5d92a32102E-75#100",
        Greater,
    );
    test(7, 1000, 1, Down, "-3.7e-301", "-0x4.0E-250#1", Greater);
    test(7, 1000, 1, Up, "-7.5e-301", "-0x8.0E-250#1", Less);
    test(7, 1000, 1, Floor, "-7.5e-301", "-0x8.0E-250#1", Less);
    test(7, 1000, 1, Ceiling, "-3.7e-301", "-0x4.0E-250#1", Greater);
    test(7, 1000, 1, Nearest, "-3.7e-301", "-0x4.0E-250#1", Greater);
    test(
        7,
        1000,
        10,
        Down,
        "-4.3747e-301",
        "-0x4.b0E-250#10",
        Greater,
    );
    test(7, 1000, 10, Up, "-4.3820e-301", "-0x4.b2E-250#10", Less);
    test(7, 1000, 10, Floor, "-4.3820e-301", "-0x4.b2E-250#10", Less);
    test(
        7,
        1000,
        10,
        Ceiling,
        "-4.3747e-301",
        "-0x4.b0E-250#10",
        Greater,
    );
    test(
        7,
        1000,
        10,
        Nearest,
        "-4.3820e-301",
        "-0x4.b2E-250#10",
        Less,
    );
    test(
        7,
        1000,
        53,
        Down,
        "-4.3804042091405903e-301",
        "-0x4.b19271bf377b0E-250#53",
        Greater,
    );
    test(
        7,
        1000,
        53,
        Up,
        "-4.3804042091405911e-301",
        "-0x4.b19271bf377b4E-250#53",
        Less,
    );
    test(
        7,
        1000,
        53,
        Floor,
        "-4.3804042091405911e-301",
        "-0x4.b19271bf377b4E-250#53",
        Less,
    );
    test(
        7,
        1000,
        53,
        Ceiling,
        "-4.3804042091405903e-301",
        "-0x4.b19271bf377b0E-250#53",
        Greater,
    );
    test(
        7,
        1000,
        53,
        Nearest,
        "-4.3804042091405911e-301",
        "-0x4.b19271bf377b4E-250#53",
        Less,
    );
    test(
        7,
        1000,
        100,
        Nearest,
        "-4.3804042091405911132891649396465e-301",
        "-0x4.b19271bf377b3dd80357392f8E-250#100",
        Greater,
    );
    test(1, 5000, 1, Down, "-3.5e-1506", "-0x8.0E-1251#1", Greater);
    test(1, 5000, 1, Up, "-7.1e-1506", "-0x1.0E-1250#1", Less);
    test(1, 5000, 1, Floor, "-7.1e-1506", "-0x1.0E-1250#1", Less);
    test(1, 5000, 1, Ceiling, "-3.5e-1506", "-0x8.0E-1251#1", Greater);
    test(1, 5000, 1, Nearest, "-3.5e-1506", "-0x8.0E-1251#1", Greater);
    test(
        1,
        5000,
        10,
        Down,
        "-4.8466e-1506",
        "-0xa.f4E-1251#10",
        Greater,
    );
    test(1, 5000, 10, Up, "-4.8535e-1506", "-0xa.f8E-1251#10", Less);
    test(
        1,
        5000,
        10,
        Floor,
        "-4.8535e-1506",
        "-0xa.f8E-1251#10",
        Less,
    );
    test(
        1,
        5000,
        10,
        Ceiling,
        "-4.8466e-1506",
        "-0xa.f4E-1251#10",
        Greater,
    );
    test(
        1,
        5000,
        10,
        Nearest,
        "-4.8535e-1506",
        "-0xa.f8E-1251#10",
        Less,
    );
    test(
        1,
        5000,
        53,
        Down,
        "-4.8535006557356868e-1506",
        "-0xa.f7f9cdfe9c6c0E-1251#53",
        Greater,
    );
    test(
        1,
        5000,
        53,
        Up,
        "-4.8535006557356876e-1506",
        "-0xa.f7f9cdfe9c6c8E-1251#53",
        Less,
    );
    test(
        1,
        5000,
        53,
        Floor,
        "-4.8535006557356876e-1506",
        "-0xa.f7f9cdfe9c6c8E-1251#53",
        Less,
    );
    test(
        1,
        5000,
        53,
        Ceiling,
        "-4.8535006557356868e-1506",
        "-0xa.f7f9cdfe9c6c0E-1251#53",
        Greater,
    );
    test(
        1,
        5000,
        53,
        Nearest,
        "-4.8535006557356868e-1506",
        "-0xa.f7f9cdfe9c6c0E-1251#53",
        Greater,
    );
    test(
        1,
        5000,
        100,
        Nearest,
        "-4.8535006557356870949617714368251e-1506",
        "-0xa.f7f9cdfe9c6c34c306d00827E-1251#100",
        Greater,
    );
    test(1048577, 1000, 1, Down, "4.9e-296", "0x8.0E-246#1", Less);
    test(1048577, 1000, 1, Up, "9.8e-296", "0x1.0E-245#1", Greater);
    test(1048577, 1000, 1, Floor, "4.9e-296", "0x8.0E-246#1", Less);
    test(
        1048577,
        1000,
        1,
        Ceiling,
        "9.8e-296",
        "0x1.0E-245#1",
        Greater,
    );
    test(1048577, 1000, 1, Nearest, "4.9e-296", "0x8.0E-246#1", Less);
    test(
        1048577,
        1000,
        10,
        Down,
        "6.5558e-296",
        "0xa.b8E-246#10",
        Less,
    );
    test(
        1048577,
        1000,
        10,
        Up,
        "6.5654e-296",
        "0xa.bcE-246#10",
        Greater,
    );
    test(
        1048577,
        1000,
        10,
        Floor,
        "6.5558e-296",
        "0xa.b8E-246#10",
        Less,
    );
    test(
        1048577,
        1000,
        10,
        Ceiling,
        "6.5654e-296",
        "0xa.bcE-246#10",
        Greater,
    );
    test(
        1048577,
        1000,
        10,
        Nearest,
        "6.5654e-296",
        "0xa.bcE-246#10",
        Greater,
    );
    test(
        1048577,
        1000,
        53,
        Down,
        "6.5617015777257331e-296",
        "0xa.ba73f8c9fcc70E-246#53",
        Less,
    );
    test(
        1048577,
        1000,
        53,
        Up,
        "6.5617015777257342e-296",
        "0xa.ba73f8c9fcc78E-246#53",
        Greater,
    );
    test(
        1048577,
        1000,
        53,
        Floor,
        "6.5617015777257331e-296",
        "0xa.ba73f8c9fcc70E-246#53",
        Less,
    );
    test(
        1048577,
        1000,
        53,
        Ceiling,
        "6.5617015777257342e-296",
        "0xa.ba73f8c9fcc78E-246#53",
        Greater,
    );
    test(
        1048577,
        1000,
        53,
        Nearest,
        "6.5617015777257342e-296",
        "0xa.ba73f8c9fcc78E-246#53",
        Greater,
    );
    test(
        1048577,
        1000,
        100,
        Nearest,
        "6.5617015777257337254277324355972e-296",
        "0xa.ba73f8c9fcc74c23ebe63a0bE-246#100",
        Less,
    );
    test(
        1048579,
        2000,
        1,
        Down,
        "-2.3e-597",
        "-0x4.0E-496#1",
        Greater,
    );
    test(1048579, 2000, 1, Up, "-4.6e-597", "-0x8.0E-496#1", Less);
    test(1048579, 2000, 1, Floor, "-4.6e-597", "-0x8.0E-496#1", Less);
    test(
        1048579,
        2000,
        1,
        Ceiling,
        "-2.3e-597",
        "-0x4.0E-496#1",
        Greater,
    );
    test(
        1048579,
        2000,
        1,
        Nearest,
        "-2.3e-597",
        "-0x4.0E-496#1",
        Greater,
    );
    test(
        1048579,
        2000,
        10,
        Down,
        "-2.3992e-597",
        "-0x4.34E-496#10",
        Greater,
    );
    test(
        1048579,
        2000,
        10,
        Up,
        "-2.4036e-597",
        "-0x4.36E-496#10",
        Less,
    );
    test(
        1048579,
        2000,
        10,
        Floor,
        "-2.4036e-597",
        "-0x4.36E-496#10",
        Less,
    );
    test(
        1048579,
        2000,
        10,
        Ceiling,
        "-2.3992e-597",
        "-0x4.34E-496#10",
        Greater,
    );
    test(
        1048579,
        2000,
        10,
        Nearest,
        "-2.4036e-597",
        "-0x4.36E-496#10",
        Less,
    );
    test(
        1048579,
        2000,
        53,
        Down,
        "-2.4024850074608730e-597",
        "-0x4.357cad9e8749cE-496#53",
        Greater,
    );
    test(
        1048579,
        2000,
        53,
        Up,
        "-2.4024850074608735e-597",
        "-0x4.357cad9e874a0E-496#53",
        Less,
    );
    test(
        1048579,
        2000,
        53,
        Floor,
        "-2.4024850074608735e-597",
        "-0x4.357cad9e874a0E-496#53",
        Less,
    );
    test(
        1048579,
        2000,
        53,
        Ceiling,
        "-2.4024850074608730e-597",
        "-0x4.357cad9e8749cE-496#53",
        Greater,
    );
    test(
        1048579,
        2000,
        53,
        Nearest,
        "-2.4024850074608735e-597",
        "-0x4.357cad9e874a0E-496#53",
        Less,
    );
    test(
        1048579,
        2000,
        100,
        Nearest,
        "-2.4024850074608735037918121034245e-597",
        "-0x4.357cad9e8749fdce2e7039980E-496#100",
        Less,
    );
    test(
        1,
        100,
        200,
        Nearest,
        "8.4784276603688996439587014693867018775520284138208363242266186e-32",
        "0x1.b839a252049c1114cf98e804177d4c76273644a29410f31c68E-26#200",
        Less,
    );
    test(
        3,
        70,
        300,
        Nearest,
        "-9.84601062862050162789118856206421377120257443264456283773667580368778297869791669066481\
        56997e-22",
        "-0x4.a64f450528ace6f60dd4333e6ecab80c4667247515c03d71a58ba059e79ebdf97f6caad8630E-18#300",
        Less,
    );
    test(
        -1,
        70,
        600,
        Nearest,
        "3.282003542873500542630396187354737923734191948911604340104315228439512001212817575867967\
        731372313070616956744414466712260078477462741735637350786933057556923781236738926841984996\
        8371e-22",
        "0x1.8cc51701b839a252049c1114cf98e804177cad9cc4d88b22f20ac197d6d5a48d3d31b7c01e756914c1809\
        275c428c08a2d0d186e3b4b633dc034a552759a2dcad5da7752b6a6b5ace917c6E-18#600",
        Greater,
    );
    test(
        1,
        90,
        1000,
        Nearest,
        "2.903855973979360333879556990387591995385154884042586741146238433982103151160615610356893\
        779558205091720450596615978798433445783511684248972731919117289147467475007779984454335221\
        093892643831504338347538685940013443056453075556384505237755060214145095216776145950987187\
        62762754249439112814650591948176017e-28",
        "0x1.701b839a252049c1114cf98e804177d4c76273644a294090580192590e69e98ca6b8d595da3bdb8f1b86c\
        a16b75da5809b6f06368aafa24eb458c65acd365c322501df79534d51ff0b1be994c00ab59e870160af8a68978\
        6d11c4f5de33f2de9ccd39f5e38cf702376a9c68d24864b49903a4060073c54b44eae67107cE-23#1000",
        Less,
    );
}

// Inputs within 2^(-2^30) of an odd multiple of pi/2, whose cosines underflow. Constructing an
// input and each call computes pi to about 2^30 bits (~6 minutes each), so this test is slow even
// in release mode; the two calls cover both signs, rounding to zero and rounding away from it, and
// `Nearest` and a directed mode.
#[test]
fn test_cos_underflow() {
    let p = (1u64 << 30) + 64;
    // 2^(-2^30), the smallest positive `Float`, at the output precision
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    assert_eq!(min_positive.get_exponent(), Some(Float::MIN_EXPONENT));
    // pi rounded down: x_lo < pi/2, so cos(x_lo) is positive, and at most half an ulp of x_lo,
    // 2^(-2^30 - 64), in magnitude
    let mut pi = Float::pi_prec_round(p, Floor).0;
    let x_lo = &pi >> 1u32;
    let (c, o) = x_lo.cos_prec_round_ref(10, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    // pi rounded up: x_hi > pi/2, so cos(x_hi) is negative
    pi.increment();
    let x_hi = pi >> 1u32;
    let (c, o) = x_hi.cos_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cos() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_cos(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 1.0);
    test::<f32>(-0.0, 1.0);
    test::<f32>(1.0, 0.5403023);
    test::<f32>(-1.0, 0.5403023);
    test::<f32>(0.5, 0.87758255);
    test::<f32>(-0.5, 0.87758255);
    test::<f32>(2.0, -0.41614684);
    test::<f32>(-2.0, -0.41614684);
    test::<f32>(core::f32::consts::PI, -1.0);
    test::<f32>(core::f32::consts::FRAC_PI_2, -4.371139e-8);
    test::<f32>(core::f32::consts::E, -0.91173387);
    test::<f32>(100.0, 0.8623189);
    test::<f32>(1.0e10, 0.87311965);
    test::<f32>(1.0e30, -0.6116048);
    test::<f32>(3.4028235e38, 0.853021);
    test::<f32>(1.1754944e-38, 1.0);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 1.0);
    test::<f64>(-0.0, 1.0);
    test::<f64>(1.0, 0.5403023058681398);
    test::<f64>(-1.0, 0.5403023058681398);
    test::<f64>(0.5, 0.8775825618903728);
    test::<f64>(-0.5, 0.8775825618903728);
    test::<f64>(2.0, -0.4161468365471424);
    test::<f64>(-2.0, -0.4161468365471424);
    test::<f64>(core::f64::consts::PI, -1.0);
    test::<f64>(core::f64::consts::FRAC_PI_2, 6.123233995736766e-17);
    test::<f64>(core::f64::consts::E, -0.9117339147869651);
    test::<f64>(100.0, 0.8623188722876839);
    test::<f64>(1.0e10, 0.873119622676856);
    test::<f64>(1.0e100, 0.9247242387519338);
    test::<f64>(1.0e300, -0.5753861119575491);
    test::<f64>(1.7976931348623157e308, -0.9999876894265599);
    test::<f64>(2.2250738585072014e-308, 1.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cos_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_cos(x);
        // cos is NaN exactly for NaN and infinite inputs, and otherwise lies in [-1, 1].
        assert_eq!(c.is_nan(), !x.is_finite());
        if x.is_finite() {
            assert!(c >= T::NEGATIVE_ONE && c <= T::ONE);
            // cos is even
            assert_eq!(NiceFloat(primitive_float_cos(-x)), NiceFloat(c));
            // the result is the correctly rounded cosine, as computed by MPFR at the same precision
            let rug_x = rug::Float::with_val(
                u32::exact_from(T::MANTISSA_WIDTH + 1),
                &rug::Float::exact_from(&Float::from(x)),
            );
            let rug_c: T = T::exact_from(&<Float as From<&rug::Float>>::from(&rug_x.cos()));
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_cos_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cos_properties_helper);
}

#[test]
fn test_cos_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::cos_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::cos_rational_prec_ref(&x, prec);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "1.0", "0x1.0#1", Equal);
    test("0", 5, "1.00", "0x1.0#5", Equal);
    test("0", 10, "1.0000", "0x1.000#10", Equal);
    test("0", 20, "1.0000000", "0x1.00000#20", Equal);
    test("0", 53, "1.0000000000000000", "0x1.0000000000000#53", Equal);
    test(
        "0",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1", 1, "0.50", "0x0.8#1", Less);
    test("1", 5, "0.531", "0x0.88#5", Less);
    test("1", 10, "0.54004", "0x0.8a4#10", Less);
    test("1", 20, "0.54030228", "0x0.8a514#20", Less);
    test(
        "1",
        53,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "1",
        100,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
    );
    test("-1", 1, "0.50", "0x0.8#1", Less);
    test("-1", 5, "0.531", "0x0.88#5", Less);
    test("-1", 10, "0.54004", "0x0.8a4#10", Less);
    test("-1", 20, "0.54030228", "0x0.8a514#20", Less);
    test(
        "-1",
        53,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "-1",
        100,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
    );
    test("1/2", 1, "1.0", "0x1.0#1", Greater);
    test("1/2", 5, "0.875", "0x0.e0#5", Less);
    test("1/2", 10, "0.87793", "0x0.e0c#10", Greater);
    test("1/2", 20, "0.87758255", "0x0.e0a94#20", Less);
    test(
        "1/2",
        53,
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
    );
    test(
        "1/2",
        100,
        "0.87758256189037271611628158260408",
        "0x0.e0a94032dbea7cedbddd9da30#100",
        Greater,
    );
    test("1/3", 1, "1.0", "0x1.0#1", Greater);
    test("1/3", 5, "0.938", "0x0.f0#5", Less);
    test("1/3", 10, "0.94531", "0x0.f20#10", Greater);
    test("1/3", 20, "0.94495678", "0x0.f1e8b#20", Less);
    test(
        "1/3",
        53,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "1/3",
        100,
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d#100",
        Less,
    );
    test("-1/3", 1, "1.0", "0x1.0#1", Greater);
    test("-1/3", 5, "0.938", "0x0.f0#5", Less);
    test("-1/3", 10, "0.94531", "0x0.f20#10", Greater);
    test("-1/3", 20, "0.94495678", "0x0.f1e8b#20", Less);
    test(
        "-1/3",
        53,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "-1/3",
        100,
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d#100",
        Less,
    );
    test("3/5", 1, "1.0", "0x1.0#1", Greater);
    test("3/5", 10, "0.82520", "0x0.d34#10", Less);
    test(
        "3/5",
        53,
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
    );
    test(
        "3/5",
        100,
        "0.82533561490967829724095249895546",
        "0x0.d34931e242d8a5cb448dccbdf#100",
        Greater,
    );
    test("22/7", 1, "-1.0", "-0x1.0#1", Less);
    test("22/7", 5, "-1.00", "-0x1.0#5", Less);
    test("22/7", 10, "-1.0000", "-0x1.000#10", Less);
    test("22/7", 20, "-0.99999905", "-0x0.fffff#20", Greater);
    test(
        "22/7",
        53,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "22/7",
        100,
        "-0.99999920053355290326833573965634",
        "-0x0.fffff29651586c28bc01f9fb2#100",
        Greater,
    );
    test("-22/7", 1, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 5, "-1.00", "-0x1.0#5", Less);
    test("-22/7", 10, "-1.0000", "-0x1.000#10", Less);
    test("-22/7", 20, "-0.99999905", "-0x0.fffff#20", Greater);
    test(
        "-22/7",
        53,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "-22/7",
        100,
        "-0.99999920053355290326833573965634",
        "-0x0.fffff29651586c28bc01f9fb2#100",
        Greater,
    );
    test("355/113", 1, "-1.0", "-0x1.0#1", Less);
    test("355/113", 5, "-1.00", "-0x1.0#5", Less);
    test("355/113", 10, "-1.0000", "-0x1.000#10", Less);
    test("355/113", 20, "-1.0000000", "-0x1.00000#20", Less);
    test(
        "355/113",
        53,
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
    );
    test(
        "355/113",
        100,
        "-0.99999999999996441843371693431264",
        "-0x0.fffffffffff5fc13f3fa12934#100",
        Greater,
    );
    test("3", 1, "-1.0", "-0x1.0#1", Less);
    test("3", 5, "-1.00", "-0x1.0#5", Less);
    test("3", 10, "-0.99023", "-0x0.fd8#10", Less);
    test("3", 20, "-0.98999214", "-0x0.fd702#20", Greater);
    test(
        "3",
        53,
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Greater,
    );
    test(
        "3",
        100,
        "-0.98999249660044545727157279473154",
        "-0x0.fd7025f42f2e9307dff82fdf7#100",
        Less,
    );
    test("100", 1, "1.0", "0x1.0#1", Greater);
    test("100", 5, "0.875", "0x0.e0#5", Greater);
    test("100", 10, "0.86230", "0x0.dcc#10", Less);
    test("100", 20, "0.86231899", "0x0.dcc0f#20", Greater);
    test(
        "100",
        53,
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Less,
    );
    test(
        "100",
        100,
        "0.86231887228768393410193851395099",
        "0x0.dcc0edfb32fefb1fa19b9b30c#100",
        Greater,
    );
    test("1000000", 1, "1.0", "0x1.0#1", Greater);
    test("1000000", 5, "0.938", "0x0.f0#5", Greater);
    test("1000000", 10, "0.93652", "0x0.efc#10", Less);
    test("1000000", 20, "0.93675232", "0x0.efcf0#20", Greater);
    test(
        "1000000",
        53,
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Less,
    );
    test(
        "1000000",
        100,
        "0.93675212753314478693853253507492",
        "0x0.efcefcc836996357644d418cd#100",
        Greater,
    );
    test("1/1000000", 1, "1.0", "0x1.0#1", Greater);
    test("1/1000000", 5, "1.00", "0x1.0#5", Greater);
    test("1/1000000", 10, "1.0000", "0x1.000#10", Greater);
    test("1/1000000", 20, "1.0000000", "0x1.00000#20", Greater);
    test(
        "1/1000000",
        53,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        "0.99999999999950000000000004166665",
        "0x0.ffffffffff734333f690bc5c6#100",
        Less,
    );
    test("-1/1000000", 1, "1.0", "0x1.0#1", Greater);
    test("-1/1000000", 5, "1.00", "0x1.0#5", Greater);
    test("-1/1000000", 10, "1.0000", "0x1.000#10", Greater);
    test("-1/1000000", 20, "1.0000000", "0x1.00000#20", Greater);
    test(
        "-1/1000000",
        53,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "-1/1000000",
        100,
        "0.99999999999950000000000004166665",
        "0x0.ffffffffff734333f690bc5c6#100",
        Less,
    );
    test("1/1000000000000000000000000", 1, "1.0", "0x1.0#1", Greater);
    test("1/1000000000000000000000000", 5, "1.00", "0x1.0#5", Greater);
    test(
        "1/1000000000000000000000000",
        10,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        20,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        "8.63e-32",
        "0x1.cE-26#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        "-2.59e-31",
        "-0x5.4E-26#5",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        "-2.5435290e-31",
        "-0x5.28ad0E-26#20",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        "-2.5435282981106698931876104408174e-31",
        "-0x5.28ace6f60dd4333e6ecab80c8E-26#100",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        "-1.00",
        "-0x1.0#5",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        "-1.0000000",
        "-0x1.00000#20",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        "8.63e-32",
        "0x1.cE-26#5",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        5,
        "0.484",
        "0x0.7c#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        "0.48926",
        "0x0.7d4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        "0.48917866",
        "0x0.7d3ad0#20",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        100,
        "0.48917865697472144990578930875139",
        "0x0.7d3acffd9b8db34b71d86c7ff0#100",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        "1.00",
        "0x1.0#5",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
}

// The inputs include dyadic approximations of pi/2, 3pi/2, and pi (from `Float::pi_prec(100)`),
// which take the near-zero path, as well as 2^100, which is reduced modulo 2 pi, and values small
// enough to round to 1.
#[test]
fn test_cos_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::cos_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::cos_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, Down, "1.0", "0x1.0#1", Equal);
    test("0", 1, Up, "1.0", "0x1.0#1", Equal);
    test("0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("0", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("0", 5, Nearest, "1.00", "0x1.0#5", Equal);
    test("0", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("0", 20, Nearest, "1.0000000", "0x1.00000#20", Equal);
    test(
        "0",
        53,
        Down,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Up,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Ceiling,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Exact,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1", 1, Down, "0.50", "0x0.8#1", Less);
    test("1", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1", 5, Nearest, "0.531", "0x0.88#5", Less);
    test("1", 10, Down, "0.54004", "0x0.8a4#10", Less);
    test("1", 10, Up, "0.54102", "0x0.8a8#10", Greater);
    test("1", 10, Floor, "0.54004", "0x0.8a4#10", Less);
    test("1", 10, Ceiling, "0.54102", "0x0.8a8#10", Greater);
    test("1", 10, Nearest, "0.54004", "0x0.8a4#10", Less);
    test("1", 20, Nearest, "0.54030228", "0x0.8a514#20", Less);
    test(
        "1",
        53,
        Down,
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
    );
    test("-1", 1, Down, "0.50", "0x0.8#1", Less);
    test("-1", 1, Up, "1.0", "0x1.0#1", Greater);
    test("-1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("-1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("-1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("-1", 5, Nearest, "0.531", "0x0.88#5", Less);
    test("-1", 10, Down, "0.54004", "0x0.8a4#10", Less);
    test("-1", 10, Up, "0.54102", "0x0.8a8#10", Greater);
    test("-1", 10, Floor, "0.54004", "0x0.8a4#10", Less);
    test("-1", 10, Ceiling, "0.54102", "0x0.8a8#10", Greater);
    test("-1", 10, Nearest, "0.54004", "0x0.8a4#10", Less);
    test("-1", 20, Nearest, "0.54030228", "0x0.8a514#20", Less);
    test(
        "-1",
        53,
        Down,
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
    );
    test(
        "-1",
        53,
        Up,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "-1",
        53,
        Floor,
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
    );
    test(
        "-1",
        100,
        Nearest,
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
    );
    test("1/2", 1, Down, "0.50", "0x0.8#1", Less);
    test("1/2", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/2", 5, Nearest, "0.875", "0x0.e0#5", Less);
    test("1/2", 10, Down, "0.87695", "0x0.e08#10", Less);
    test("1/2", 10, Up, "0.87793", "0x0.e0c#10", Greater);
    test("1/2", 10, Floor, "0.87695", "0x0.e08#10", Less);
    test("1/2", 10, Ceiling, "0.87793", "0x0.e0c#10", Greater);
    test("1/2", 10, Nearest, "0.87793", "0x0.e0c#10", Greater);
    test("1/2", 20, Nearest, "0.87758255", "0x0.e0a94#20", Less);
    test(
        "1/2",
        53,
        Down,
        "0.87758256189037265",
        "0x0.e0a94032dbea78#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "0.87758256189037265",
        "0x0.e0a94032dbea78#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.87758256189037271611628158260408",
        "0x0.e0a94032dbea7cedbddd9da30#100",
        Greater,
    );
    test("1/3", 1, Down, "0.50", "0x0.8#1", Less);
    test("1/3", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1/3", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1/3", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/3", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/3", 5, Nearest, "0.938", "0x0.f0#5", Less);
    test("1/3", 10, Down, "0.94434", "0x0.f1c#10", Less);
    test("1/3", 10, Up, "0.94531", "0x0.f20#10", Greater);
    test("1/3", 10, Floor, "0.94434", "0x0.f1c#10", Less);
    test("1/3", 10, Ceiling, "0.94531", "0x0.f20#10", Greater);
    test("1/3", 10, Nearest, "0.94531", "0x0.f20#10", Greater);
    test("1/3", 20, Nearest, "0.94495678", "0x0.f1e8b#20", Less);
    test(
        "1/3",
        53,
        Down,
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d#100",
        Less,
    );
    test("-1/3", 1, Down, "0.50", "0x0.8#1", Less);
    test("-1/3", 1, Up, "1.0", "0x1.0#1", Greater);
    test("-1/3", 1, Floor, "0.50", "0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("-1/3", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("-1/3", 5, Nearest, "0.938", "0x0.f0#5", Less);
    test("-1/3", 10, Down, "0.94434", "0x0.f1c#10", Less);
    test("-1/3", 10, Up, "0.94531", "0x0.f20#10", Greater);
    test("-1/3", 10, Floor, "0.94434", "0x0.f1c#10", Less);
    test("-1/3", 10, Ceiling, "0.94531", "0x0.f20#10", Greater);
    test("-1/3", 10, Nearest, "0.94531", "0x0.f20#10", Greater);
    test("-1/3", 20, Nearest, "0.94495678", "0x0.f1e8b#20", Less);
    test(
        "-1/3",
        53,
        Down,
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Up,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Floor,
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d#100",
        Less,
    );
    test("3/5", 1, Down, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Up, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("3/5", 10, Down, "0.82520", "0x0.d34#10", Less);
    test("3/5", 10, Up, "0.82617", "0x0.d38#10", Greater);
    test("3/5", 10, Floor, "0.82520", "0x0.d34#10", Less);
    test("3/5", 10, Ceiling, "0.82617", "0x0.d38#10", Greater);
    test("3/5", 10, Nearest, "0.82520", "0x0.d34#10", Less);
    test(
        "3/5",
        53,
        Down,
        "0.82533561490967822",
        "0x0.d34931e242d8a0#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "0.82533561490967822",
        "0x0.d34931e242d8a0#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "0.82533561490967829724095249895546",
        "0x0.d34931e242d8a5cb448dccbdf#100",
        Greater,
    );
    test("22/7", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("22/7", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("22/7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("22/7", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("22/7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("22/7", 5, Nearest, "-1.00", "-0x1.0#5", Less);
    test("22/7", 10, Down, "-0.99902", "-0x0.ffc#10", Greater);
    test("22/7", 10, Up, "-1.0000", "-0x1.000#10", Less);
    test("22/7", 10, Floor, "-1.0000", "-0x1.000#10", Less);
    test("22/7", 10, Ceiling, "-0.99902", "-0x0.ffc#10", Greater);
    test("22/7", 10, Nearest, "-1.0000", "-0x1.000#10", Less);
    test("22/7", 20, Nearest, "-0.99999905", "-0x0.fffff#20", Greater);
    test(
        "22/7",
        53,
        Down,
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Up,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "22/7",
        53,
        Floor,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "22/7",
        100,
        Nearest,
        "-0.99999920053355290326833573965634",
        "-0x0.fffff29651586c28bc01f9fb2#100",
        Greater,
    );
    test("-22/7", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("-22/7", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 5, Nearest, "-1.00", "-0x1.0#5", Less);
    test("-22/7", 10, Down, "-0.99902", "-0x0.ffc#10", Greater);
    test("-22/7", 10, Up, "-1.0000", "-0x1.000#10", Less);
    test("-22/7", 10, Floor, "-1.0000", "-0x1.000#10", Less);
    test("-22/7", 10, Ceiling, "-0.99902", "-0x0.ffc#10", Greater);
    test("-22/7", 10, Nearest, "-1.0000", "-0x1.000#10", Less);
    test(
        "-22/7",
        20,
        Nearest,
        "-0.99999905",
        "-0x0.fffff#20",
        Greater,
    );
    test(
        "-22/7",
        53,
        Down,
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Up,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Floor,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-0.99999920053355290326833573965634",
        "-0x0.fffff29651586c28bc01f9fb2#100",
        Greater,
    );
    test("355/113", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("355/113", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("355/113", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("355/113", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("355/113", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("355/113", 5, Nearest, "-1.00", "-0x1.0#5", Less);
    test("355/113", 10, Down, "-0.99902", "-0x0.ffc#10", Greater);
    test("355/113", 10, Up, "-1.0000", "-0x1.000#10", Less);
    test("355/113", 10, Floor, "-1.0000", "-0x1.000#10", Less);
    test("355/113", 10, Ceiling, "-0.99902", "-0x0.ffc#10", Greater);
    test("355/113", 10, Nearest, "-1.0000", "-0x1.000#10", Less);
    test("355/113", 20, Nearest, "-1.0000000", "-0x1.00000#20", Less);
    test(
        "355/113",
        53,
        Down,
        "-0.99999999999996436",
        "-0x0.fffffffffff5f8#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Up,
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
    );
    test(
        "355/113",
        53,
        Floor,
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "-0.99999999999996436",
        "-0x0.fffffffffff5f8#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
    );
    test(
        "355/113",
        100,
        Nearest,
        "-0.99999999999996441843371693431264",
        "-0x0.fffffffffff5fc13f3fa12934#100",
        Greater,
    );
    test("3", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("3", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("3", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("3", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("3", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("3", 5, Nearest, "-1.00", "-0x1.0#5", Less);
    test("3", 10, Down, "-0.98926", "-0x0.fd4#10", Greater);
    test("3", 10, Up, "-0.99023", "-0x0.fd8#10", Less);
    test("3", 10, Floor, "-0.99023", "-0x0.fd8#10", Less);
    test("3", 10, Ceiling, "-0.98926", "-0x0.fd4#10", Greater);
    test("3", 10, Nearest, "-0.99023", "-0x0.fd8#10", Less);
    test("3", 20, Nearest, "-0.98999214", "-0x0.fd702#20", Greater);
    test(
        "3",
        53,
        Down,
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Greater,
    );
    test(
        "3",
        53,
        Up,
        "-0.98999249660044553",
        "-0x0.fd7025f42f2e98#53",
        Less,
    );
    test(
        "3",
        53,
        Floor,
        "-0.98999249660044553",
        "-0x0.fd7025f42f2e98#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "-0.98999249660044545727157279473154",
        "-0x0.fd7025f42f2e9307dff82fdf7#100",
        Less,
    );
    test("100", 1, Down, "0.50", "0x0.8#1", Less);
    test("100", 1, Up, "1.0", "0x1.0#1", Greater);
    test("100", 1, Floor, "0.50", "0x0.8#1", Less);
    test("100", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("100", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("100", 5, Nearest, "0.875", "0x0.e0#5", Greater);
    test("100", 10, Down, "0.86230", "0x0.dcc#10", Less);
    test("100", 10, Up, "0.86328", "0x0.dd0#10", Greater);
    test("100", 10, Floor, "0.86230", "0x0.dcc#10", Less);
    test("100", 10, Ceiling, "0.86328", "0x0.dd0#10", Greater);
    test("100", 10, Nearest, "0.86230", "0x0.dcc#10", Less);
    test("100", 20, Nearest, "0.86231899", "0x0.dcc0f#20", Greater);
    test(
        "100",
        53,
        Down,
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Less,
    );
    test(
        "100",
        53,
        Up,
        "0.86231887228768400",
        "0x0.dcc0edfb32ff00#53",
        Greater,
    );
    test(
        "100",
        53,
        Floor,
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "0.86231887228768400",
        "0x0.dcc0edfb32ff00#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Less,
    );
    test(
        "100",
        100,
        Nearest,
        "0.86231887228768393410193851395099",
        "0x0.dcc0edfb32fefb1fa19b9b30c#100",
        Greater,
    );
    test("1000000", 1, Down, "0.50", "0x0.8#1", Less);
    test("1000000", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1000000", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1000000", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1000000", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1000000", 5, Nearest, "0.938", "0x0.f0#5", Greater);
    test("1000000", 10, Down, "0.93652", "0x0.efc#10", Less);
    test("1000000", 10, Up, "0.93750", "0x0.f00#10", Greater);
    test("1000000", 10, Floor, "0.93652", "0x0.efc#10", Less);
    test("1000000", 10, Ceiling, "0.93750", "0x0.f00#10", Greater);
    test("1000000", 10, Nearest, "0.93652", "0x0.efc#10", Less);
    test(
        "1000000",
        20,
        Nearest,
        "0.93675232",
        "0x0.efcf0#20",
        Greater,
    );
    test(
        "1000000",
        53,
        Down,
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Less,
    );
    test(
        "1000000",
        53,
        Up,
        "0.93675212753314485",
        "0x0.efcefcc8369968#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Floor,
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "0.93675212753314485",
        "0x0.efcefcc8369968#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Less,
    );
    test(
        "1000000",
        100,
        Nearest,
        "0.93675212753314478693853253507492",
        "0x0.efcefcc836996357644d418cd#100",
        Greater,
    );
    test("1/1000000", 1, Down, "0.50", "0x0.8#1", Less);
    test("1/1000000", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1/1000000", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1/1000000", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/1000000", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/1000000", 5, Nearest, "1.00", "0x1.0#5", Greater);
    test("1/1000000", 10, Down, "0.99902", "0x0.ffc#10", Less);
    test("1/1000000", 10, Up, "1.0000", "0x1.000#10", Greater);
    test("1/1000000", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test("1/1000000", 10, Ceiling, "1.0000", "0x1.000#10", Greater);
    test("1/1000000", 10, Nearest, "1.0000", "0x1.000#10", Greater);
    test(
        "1/1000000",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Down,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "0.99999999999950000000000004166665",
        "0x0.ffffffffff734333f690bc5c6#100",
        Less,
    );
    test("-1/1000000", 1, Down, "0.50", "0x0.8#1", Less);
    test("-1/1000000", 1, Up, "1.0", "0x1.0#1", Greater);
    test("-1/1000000", 1, Floor, "0.50", "0x0.8#1", Less);
    test("-1/1000000", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("-1/1000000", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("-1/1000000", 5, Nearest, "1.00", "0x1.0#5", Greater);
    test("-1/1000000", 10, Down, "0.99902", "0x0.ffc#10", Less);
    test("-1/1000000", 10, Up, "1.0000", "0x1.000#10", Greater);
    test("-1/1000000", 10, Floor, "0.99902", "0x0.ffc#10", Less);
    test("-1/1000000", 10, Ceiling, "1.0000", "0x1.000#10", Greater);
    test("-1/1000000", 10, Nearest, "1.0000", "0x1.000#10", Greater);
    test(
        "-1/1000000",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Down,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "0.99999999999950000000000004166665",
        "0x0.ffffffffff734333f690bc5c6#100",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Down,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Up,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Ceiling,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "8.63e-32",
        "0x1.cE-26#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "-3.9e-31",
        "-0x8.0E-26#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "-3.9e-31",
        "-0x8.0E-26#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "-2.59e-31",
        "-0x5.4E-26#5",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "-2.5461e-31",
        "-0x5.2aE-26#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "-2.5461e-31",
        "-0x5.2aE-26#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "-2.5435290e-31",
        "-0x5.28ad0E-26#20",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "-2.5435282981106695e-31",
        "-0x5.28ace6f60dd40E-26#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "-2.5435282981106695e-31",
        "-0x5.28ace6f60dd40E-26#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "-2.5435282981106698931876104408174e-31",
        "-0x5.28ace6f60dd4333e6ecab80c8E-26#100",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "8.63e-32",
        "0x1.cE-26#5",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Down,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Up,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Ceiling,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "0.484",
        "0x0.7c#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "0.48877",
        "0x0.7d2#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "0.48926",
        "0x0.7d4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "0.48877",
        "0x0.7d2#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "0.48926",
        "0x0.7d4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "0.48926",
        "0x0.7d4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "0.48917866",
        "0x0.7d3ad0#20",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "0.48917865697472140",
        "0x0.7d3acffd9b8db0#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "0.48917865697472140",
        "0x0.7d3acffd9b8db0#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "0.48917865697472144990578930875139",
        "0x0.7d3acffd9b8db34b71d86c7ff0#100",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn cos_rational_prec_fail() {
    Float::cos_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn cos_rational_prec_ref_fail() {
    Float::cos_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
#[should_panic]
fn cos_rational_prec_round_fail_1() {
    Float::cos_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn cos_rational_prec_round_fail_2() {
    Float::cos_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn cos_rational_prec_round_ref_fail() {
    Float::cos_rational_prec_round_ref(&Rational::ONE, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn cos_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::cos_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::cos_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // |cos x| <= 1, and cos is even
    assert!(c.le_abs(&1u32));
    let (c_neg, o_neg) = Float::cos_rational_prec_round(-&x, prec, rm);
    assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&c));
    assert_eq!(o_neg, o);

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_cos_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::cos_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::cos_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn cos_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        cos_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::cos_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn cos_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::cos_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::cos_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::cos_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    assert!(c.le_abs(&1u32));

    let (rug_c, rug_o) = rug_cos_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);

    // the cosine of an exactly representable rational is the Float cosine
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.cos_prec(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn cos_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        cos_rational_prec_properties_helper(x, prec);
    });
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about
// 2^30 bits (~14 minutes in release mode; MPFR cannot serve as an oracle here, so the value was
// cross-checked by temporarily routing moderate inputs through the same reduction).
#[test]
fn test_cos_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (c, o) = Float::cos_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(c.to_string(), "-0.77832");
    assert_eq!(to_hex_string(&c), "-0x0.c74#10");
    assert_eq!(o, Greater);
}

// Dyadic rationals within 2^(-2^30) of pi/2, whose cosines underflow: the near-zero path computes
// the distance to pi/2 exactly and rounds the bracket in `Rational` arithmetic, so the underflow
// decision and the `Ordering` are exact. Constructing the inputs and each call computes pi to about
// 2^30 bits (~6 minutes each), so this test is slow even in release mode.
#[test]
fn test_cos_rational_underflow() {
    let p = (1u64 << 30) + 64;
    // 2^(-2^30), the smallest positive `Float`, at the output precision
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    // pi rounded down: x_lo < pi/2, so cos(x_lo) is positive, and at most half an ulp of x_lo,
    // 2^(-2^30 - 64), in magnitude
    let mut pi = Float::pi_prec_round(p, Floor).0;
    let x_lo = Rational::exact_from(&(&pi >> 1u32));
    let (c, o) = Float::cos_rational_prec_round_ref(&x_lo, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    // pi rounded up: x_hi > pi/2, so cos(x_hi) is negative
    pi.increment();
    let x_hi = Rational::exact_from(&(pi >> 1u32));
    let (c, o) = Float::cos_rational_prec_round_ref(&x_hi, 10, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cos_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_cos_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 1.0);
    test::<f32>("1", 0.5403023);
    test::<f32>("1/2", 0.87758255);
    test::<f32>("1/3", 0.94495696);
    test::<f32>("3/5", 0.8253356);
    test::<f32>("22/7", -0.9999992);
    test::<f32>("355/113", -1.0);
    test::<f32>("1000000", 0.93675214);
    test::<f32>("1/1000000", 1.0);
    test::<f32>("1/1000000000000000000000000", 1.0);
    test::<f32>("-1", 0.5403023);
    test::<f32>("-1/2", 0.87758255);
    test::<f32>("-1/3", 0.94495696);
    test::<f32>("-22/7", -0.9999992);
    test::<f32>("-1000000", 0.93675214);
    test::<f32>("10000", -0.95215535);

    test::<f64>("0", 1.0);
    test::<f64>("1", 0.5403023058681398);
    test::<f64>("1/2", 0.8775825618903728);
    test::<f64>("1/3", 0.9449569463147377);
    test::<f64>("3/5", 0.8253356149096783);
    test::<f64>("22/7", -0.999999200533553);
    test::<f64>("355/113", -0.9999999999999645);
    test::<f64>("1000000", 0.9367521275331447);
    test::<f64>("1/1000000", 0.9999999999995);
    test::<f64>("1/1000000000000000000000000", 1.0);
    test::<f64>("-1", 0.5403023058681398);
    test::<f64>("-1/2", 0.8775825618903728);
    test::<f64>("-1/3", 0.9449569463147377);
    test::<f64>("-22/7", -0.999999200533553);
    test::<f64>("-1000000", 0.9367521275331447);
    test::<f64>("10000", -0.9521553682590148);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cos_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_cos_rational::<T>(&x);
        // the cosine of a rational is never NaN, and lies in [-1, 1]
        assert!(c >= T::NEGATIVE_ONE && c <= T::ONE);
        // cos is even
        assert_eq!(
            NiceFloat(primitive_float_cos_rational::<T>(&-&x)),
            NiceFloat(c)
        );
        // the result is the correctly rounded cosine, as computed by MPFR at the same precision
        let rug_c = rug_cos_rational_prec(&x, T::MANTISSA_WIDTH + 1).0;
        let rug_c: T = T::exact_from(&<Float as From<&rug::Float>>::from(&rug_c));
        assert_eq!(NiceFloat(rug_c), NiceFloat(c));
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The cosine of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float cosine.
        if x.is_finite() {
            assert_eq!(
                NiceFloat(primitive_float_cos_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_cos(x))
            );
        }
    });
}

#[test]
fn primitive_float_cos_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cos_rational_properties_helper);
}
