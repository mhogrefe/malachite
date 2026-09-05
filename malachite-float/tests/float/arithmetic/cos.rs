// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Cos, CosAssign};
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::cos::{
    rug_cos, rug_cos_prec, rug_cos_prec_round, rug_cos_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

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
