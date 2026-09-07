// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Cos, PowerOf2, Sin, SinCos, SinCosAssign};
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
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::cos::{
    primitive_float_cos, primitive_float_cos_rational, primitive_float_cos_with_period,
    primitive_float_cos_with_period_rational,
};
use malachite_float::float::arithmetic::sin::{
    primitive_float_sin, primitive_float_sin_rational, primitive_float_sin_with_period,
    primitive_float_sin_with_period_rational,
};
use malachite_float::float::arithmetic::sin_cos::{
    primitive_float_sin_cos, primitive_float_sin_cos_pi, primitive_float_sin_cos_pi_rational,
    primitive_float_sin_cos_rational, primitive_float_sin_cos_with_period,
    primitive_float_sin_cos_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sin_cos::{
    rug_sin_cos, rug_sin_cos_pi_prec_round, rug_sin_cos_pi_rational_prec_round, rug_sin_cos_prec,
    rug_sin_cos_prec_round, rug_sin_cos_rational_prec, rug_sin_cos_rational_prec_round,
    rug_sin_cos_round, rug_sin_cos_with_period_prec, rug_sin_cos_with_period_prec_round,
    rug_sin_cos_with_period_rational_prec, rug_sin_cos_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36, float_unsigned_rounding_mode_triple_gen_var_37,
    float_unsigned_rounding_mode_triple_gen_var_39,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_10,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sin_cos_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (sin, cos, o_s, o_c) = x.clone().sin_cos_prec_round(prec, rm);
        assert!(sin.is_valid());
        assert!(cos.is_valid());
        assert_eq!(sin.to_string(), out_s);
        assert_eq!(to_hex_string(&sin), out_s_hex);
        assert_eq!(cos.to_string(), out_c);
        assert_eq!(to_hex_string(&cos), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (sin_alt, cos_alt, o_s_alt, o_c_alt) = x.sin_cos_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let mut sin_alt = x.clone();
        let mut cos_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = sin_alt.sin_cos_prec_round_assign(&mut cos_alt, prec, rm);
        assert!(sin_alt.is_valid());
        assert!(cos_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        // the two results are those of the separate functions
        let (sin_alt, o_s_alt) = x.sin_prec_round_ref(prec, rm);
        let (cos_alt, o_c_alt) = x.cos_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&sin)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&cos)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    };
    test(
        "NaN", "NaN", 1, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 1, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        Equal,
        Equal,
    );
    test(
        "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0.0",
        "0x0.0",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    // Rows reuse the sine test's inputs (minus its `Exact` rows), so they cover the small-input
    // shortcut, the reduced and unreduced Ziv loop, its retries, the cancellation bumps, and the
    // sine's underflow at 2^(-2^30).
    test(
        "NaN", "NaN", 1, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 1, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        Equal,
        Equal,
    );
    test(
        "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Floor,
        "0.12",
        "0x0.2#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Ceiling,
        "0.25",
        "0x0.4#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        2,
        Nearest,
        "0.25",
        "0x0.4#2",
        "1.0",
        "0x1.0#2",
        Greater,
        Greater,
    );
    test(
        "0.25", "0x0.4#1", 1, Floor, "0.12", "0x0.2#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "0.25", "0x0.4#1", 1, Nearest, "0.25", "0x0.4#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "0.25", "0x0.4#1", 1, Ceiling, "0.25", "0x0.4#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "NaN", "NaN", 1, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        Equal,
        Equal,
    );
    test(
        "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0.0",
        "0x0.0",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Floor,
        "0.84082",
        "0x0.d74#10",
        "0.54004",
        "0x0.8a4#10",
        Less,
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.84180",
        "0x0.d78#10",
        "0.54102",
        "0x0.8a8#10",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.84180",
        "0x0.d78#10",
        "0.54004",
        "0x0.8a4#10",
        Greater,
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        "0.54030230586813971740093660744256",
        "0x0.8a51407da8345c91c2466d976#100",
        Less,
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.84147098480789650665250232163084",
        "0x0.d76aa47848677020c6e9e909d#100",
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Less,
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.84180",
        "-0x0.d78#10",
        "0.54004",
        "0x0.8a4#10",
        Less,
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "0.90918",
        "0x0.e8c#10",
        "-0.41602",
        "-0x0.6a8#10",
        Less,
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "0.14111",
        "0x0.242#10",
        "-0.99023",
        "-0x0.fd8#10",
        Less,
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-0.75684",
        "-0x0.c1c#10",
        "-0.65332",
        "-0x0.a74#10",
        Less,
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "-0.75680249530792825137263909451172",
        "-0x0.c1bdceeee0f5738674c02ad07#100",
        "-0.65364362086361191463916818309786",
        "-0x0.a7553036d926062336d0e16e4#100",
        Greater,
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "-0.50684",
        "-0x0.81c#10",
        "0.86230",
        "0x0.dcc#10",
        Less,
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "-0.50636564110975879365655761046048",
        "-0x0.81a12dbc626dc03847b0aae85#100",
        "0.86231887228768393410193851395020",
        "0x0.dcc0edfb32fefb1fa19b9b30b#100",
        Less,
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "-0.50636564110975879365655761045969",
        "-0x0.81a12dbc626dc03847b0aae84#100",
        "0.86231887228768393410193851395099",
        "0x0.dcc0edfb32fefb1fa19b9b30c#100",
        Greater,
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "-0.349993502171292952130",
        "-0x0.59992c95a3619d268#64",
        "0.936752127533144786917",
        "0x0.efcefcc836996357#64",
        Less,
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "0.47942553860420301",
        "0x0.7abba1d12c17c#50",
        "0.87758256189037276",
        "0x0.e0a94032dbea8#50",
        Greater,
        Greater,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "0.10138798815552963",
        "0x0.19f4902d55d1f8#50",
        "0.99484696102354064",
        "0x0.feae4a5a1f000#50",
        Less,
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        50,
        Nearest,
        "1.0004441719502211e-10",
        "0x6.e00000000000E-9#50",
        "1.0000000000000000",
        "0x1.0000000000000#50",
        Greater,
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Floor,
        "9.9931e-11",
        "0x6.deE-9#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Ceiling,
        "1.0004e-10",
        "0x6.e0E-9#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1.570796326794896600",
        "0x1.921fb54442d183#57",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "1.9598976533924290e-17",
        "0x1.69898cc51701cE-14#53",
        Greater,
        Greater,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "3.4450928483976660e-16",
        "0x1.8d313198a2e03E-13#53",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
        Less,
    );
    test(
        "6.28318530717958579",
        "0x6.487ed5110b45e#54",
        53,
        Nearest,
        "-6.8901856967953321e-16",
        "-0x3.1a62633145c06E-13#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "9.979e99",
        "0x1.24E+83#7",
        53,
        Nearest,
        "-0.90292261046853339",
        "-0x0.e725efaac80328#53",
        "-0.42980316367459315",
        "-0x0.6e079483b32060#53",
        Less,
        Greater,
    );
    test(
        "3.0", "0x3.0#2", 1, Nearest, "0.12", "0x0.2#1", "-1.0", "-0x1.0#1", Less, Less,
    );
    test(
        "3.0", "0x3.0#2", 1, Floor, "0.12", "0x0.2#1", "-1.0", "-0x1.0#1", Less, Less,
    );
    test(
        "3.0", "0x3.0#2", 1, Ceiling, "0.25", "0x0.4#1", "-0.50", "-0x0.8#1", Greater, Greater,
    );
    test(
        "3.0", "0x3.0#2", 2, Nearest, "0.12", "0x0.2#2", "-1.0", "-0x1.0#2", Less, Less,
    );
    test(
        "0.25", "0x0.4#1", 1, Down, "0.12", "0x0.2#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1.0", "0x1.0#1", 1, Down, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "2.0", "0x2.0#1", 1, Down, "0.50", "0x0.8#1", "-0.25", "-0x0.4#1", Less, Greater,
    );
    test(
        "4.0", "0x4.0#1", 1, Down, "-0.50", "-0x0.8#1", "-0.50", "-0x0.8#1", Greater, Greater,
    );
    test(
        "-3.495934488151859089160804055e56",
        "-0xe.41ed086a5791d9e5b2924E+46#87",
        2,
        Down,
        "0.12",
        "0x0.2#2",
        "0.75",
        "0x0.c#2",
        Less,
        Less,
    );
    test(
        "6.28318536",
        "0x6.487ed6#26",
        10,
        Nearest,
        "5.5647e-8",
        "0xe.f0E-7#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "6.1232339957367660e-17",
        "0x4.69898cc51701cE-14#53",
        Greater,
        Greater,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        "5.7099684971243490026437400060191e-62",
        "0x1.77d4c76273644a29410f31c68E-51#100",
        Less,
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "-0.99999999999999999999999999999999999925",
        "-0x0.ffffffffffffffffffffffffffffff#120",
        "2.3305317247657495256428231620634736190e-36",
        "0x3.1909f22bccc1913547f3b9881a9d8cE-30#120",
        Greater,
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "-5.01655761266833202345e-20",
        "-0xe.ce675d1fc8f8cbbE-17#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Greater,
        Less,
    );
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "0.59863",
        "0x0.994#10",
        "-0.80078",
        "-0x0.cd0#10",
        Greater,
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-0.59863",
        "-0x0.994#10",
        "-0.80176",
        "-0x0.cd4#10",
        Less,
        Less,
    );
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "0.14136171",
        "0x0.243048#20",
        "-0.98995781",
        "-0x0.fd6de#20",
        Greater,
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "0.14111",
        "0x0.242#10",
        "-0.99023",
        "-0x0.fd8#10",
        Less,
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "-0.14111",
        "-0x0.242#10",
        "-0.98926",
        "-0x0.fd4#10",
        Greater,
        Greater,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Nearest,
        "1.7486e-7",
        "0x2.efE-6#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Floor,
        "1.7462e-7",
        "0x2.eeE-6#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "6.283185307179586476925286766559005788",
        "0x6.487ed5110b4611a62633145c06e10#117",
        100,
        Nearest,
        "1.9156791836247964809656913737618e-35",
        "0x1.976b7ed8fbbacc19c5fefa20aE-29#100",
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Less,
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "0.0",
        "0x0.0",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Down,
        "0.0",
        "0x0.0",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        "0.50",
        "0x0.8#1",
        Greater,
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Floor,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Ceiling,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Down,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Up,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Floor,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Down,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "0.50",
        "0x0.8#1",
        Greater,
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Up,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Nearest,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "0.0",
        "0x0.0",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_cos_prec_round_fail() {
    Float::ONE.sin_cos_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn sin_cos_prec_round_exact_fail() {
    Float::ONE.sin_cos_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_prec_fail() {
    Float::ONE.sin_cos_prec(0);
}

#[test]
#[should_panic]
fn sin_cos_round_fail() {
    Float::ONE.sin_cos_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_cos_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (s, c, o_s, o_c) = x.clone().sin_cos_prec_round(prec, rm);
    assert!(s.is_valid());
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o_s);
    assert_rounding_ordering_consistent(&c, rm, o_c);

    let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    let mut s_alt = x.clone();
    let mut c_alt = Float::NAN;
    let (o_s_alt, o_c_alt) = s_alt.sin_cos_prec_round_assign(&mut c_alt, prec, rm);
    assert!(s_alt.is_valid());
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    // the two results are those of the separate functions
    let (s_alt, o_s_alt) = x.sin_prec_round_ref(prec, rm);
    let (c_alt, o_c_alt) = x.cos_prec_round_ref(prec, rm);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_c, rug_o_s, rug_o_c) =
            rug_sin_cos_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    }

    // sin is odd and cos is even
    let (s_neg, _, o_s_neg, _) = (-&x).sin_cos_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
    assert_eq!(o_s_neg, o_s.reverse());
    let (_, c_neg, _, o_c_neg) = (-&x).sin_cos_prec_round(prec, rm);
    assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&c));
    assert_eq!(o_c_neg, o_c);

    if s.is_finite() {
        assert!(s.le_abs(&1u32));
        assert!(c.le_abs(&1u32));
    }
    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o_s == Equal {
        // only x = 0 (and NaN, and ±inf) is exact, and then both results are
        assert_eq!(o_c, Equal);
        for rm2 in exhaustive_rounding_modes() {
            let (s2, c2, o_s2, o_c2) = x.sin_cos_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(o_s2, Equal);
            assert_eq!(o_c2, Equal);
        }
    } else {
        assert_ne!(o_c, Equal);
        assert_panic!(x.sin_cos_prec_round_ref(prec, Exact));
    }
}

#[test]
fn sin_cos_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        sin_cos_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, c, o_s, o_c) = Float::NAN.sin_cos_prec_round(prec, rm);
        assert!(s.is_nan());
        assert!(c.is_nan());
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);

        let (s, c, o_s, o_c) = Float::INFINITY.sin_cos_prec_round(prec, rm);
        assert!(s.is_nan());
        assert!(c.is_nan());
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);

        let (s, c, o_s, o_c) = Float::NEGATIVE_INFINITY.sin_cos_prec_round(prec, rm);
        assert!(s.is_nan());
        assert!(c.is_nan());
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);

        let (s, c, o_s, o_c) = Float::ZERO.sin_cos_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);

        let (s, c, o_s, o_c) = Float::NEGATIVE_ZERO.sin_cos_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
    });
}

#[test]
fn sin_cos_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (s, c, o_s, o_c) = x.clone().sin_cos_round(rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o_s);
        assert_rounding_ordering_consistent(&c, rm, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_round_ref(rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_round_assign(&mut c_alt, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    });
}

#[test]
fn sin_cos_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, c, o_s, o_c) = x.clone().sin_cos_prec(prec);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, Nearest, o_s);
        assert_rounding_ordering_consistent(&c, Nearest, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_prec_assign(&mut c_alt, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let (rug_s, rug_c, rug_o_s, rug_o_c) = rug_sin_cos_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    });
}

#[test]
fn sin_cos_properties() {
    float_gen().test_properties(|x| {
        let (s, c) = x.clone().sin_cos();
        assert!(s.is_valid());
        assert!(c.is_valid());
        let (s_alt, c_alt) = (&x).sin_cos();
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        s_alt.sin_cos_assign(&mut c_alt);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        let (s_alt, c_alt, _, _) = x.sin_cos_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        assert_eq!(ComparableFloatRef(&(&x).sin()), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&(&x).cos()), ComparableFloatRef(&c));

        let (rug_s, rug_c) = rug_sin_cos(&rug::Float::exact_from(&x));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
    });
}

// n * pi, with pi rounded to the nearest `prec` bits and the product exact
fn multiple_of_pi(n: i64, prec: u64) -> Float {
    Float::pi_prec(prec)
        .0
        .mul_prec_round(Float::from(n), prec + 64, Exact)
        .0
}

// n * pi / 2, with pi rounded to the nearest `prec` bits and the product exact
fn odd_multiple_of_half_pi(n: i64, prec: u64) -> Float {
    multiple_of_pi(n, prec) >> 1u32
}

// Inputs close to a zero of the sine (n * pi) or of the cosine (n * pi / 2 with n odd), with pi
// rounded to `prec_x` bits, where that result is tiny and takes its near-zero path, while the other
// is within 2^-2cancel of ±1 and rounds from ±1 alone.
#[test]
fn test_sin_cos_near_zero() {
    let test = |half: bool,
                n: i64,
                prec_x: u64,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = if half {
            odd_multiple_of_half_pi(n, prec_x)
        } else {
            multiple_of_pi(n, prec_x)
        };
        let (s, c, o_s, o_c) = x.sin_cos_prec_round_ref(prec, rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_eq!(s.to_string(), out_s);
        assert_eq!(to_hex_string(&s), out_s_hex);
        assert_eq!(c.to_string(), out_c);
        assert_eq!(to_hex_string(&c), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (s_alt, o_s_alt) = x.sin_prec_round_ref(prec, rm);
        let (c_alt, o_c_alt) = x.cos_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let (rug_s, rug_c, rug_o_s, rug_o_c) = rug_sin_cos_prec_round(
            &rug::Float::exact_from(&x),
            prec,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    };
    test(
        false,
        1,
        150,
        1,
        Up,
        "-1.4e-45",
        "-0x8.0E-38#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        150,
        1,
        Floor,
        "-1.4e-45",
        "-0x8.0E-38#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        150,
        1,
        Ceiling,
        "-7.0e-46",
        "-0x4.0E-38#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        150,
        1,
        Nearest,
        "-1.4e-45",
        "-0x8.0E-38#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        150,
        10,
        Down,
        "-1.3767e-45",
        "-0x7.dcE-38#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        150,
        10,
        Up,
        "-1.3780e-45",
        "-0x7.deE-38#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1,
        150,
        10,
        Floor,
        "-1.3780e-45",
        "-0x7.deE-38#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1,
        150,
        10,
        Nearest,
        "-1.3780e-45",
        "-0x7.deE-38#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1,
        190,
        1,
        Down,
        "5.0e-60",
        "0x8.0E-50#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        false,
        1,
        190,
        1,
        Up,
        "1.0e-59",
        "0x1.0E-49#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        1,
        190,
        1,
        Floor,
        "5.0e-60",
        "0x8.0E-50#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        190,
        1,
        Ceiling,
        "1.0e-59",
        "0x1.0E-49#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        190,
        1,
        Nearest,
        "5.0e-60",
        "0x8.0E-50#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        190,
        10,
        Down,
        "5.0854e-60",
        "0x8.2cE-50#10",
        "-0.99902",
        "-0x0.ffc#10",
        Less,
        Greater,
    );
    test(
        false,
        1,
        190,
        10,
        Up,
        "5.0951e-60",
        "0x8.30E-50#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        1,
        190,
        10,
        Floor,
        "5.0854e-60",
        "0x8.2cE-50#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1,
        190,
        10,
        Ceiling,
        "5.0951e-60",
        "0x8.30E-50#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        190,
        10,
        Nearest,
        "5.0951e-60",
        "0x8.30E-50#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        1,
        200,
        1,
        Down,
        "7.8e-62",
        "0x2.0E-51#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        false,
        1,
        200,
        1,
        Up,
        "1.6e-61",
        "0x4.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        1,
        200,
        1,
        Floor,
        "7.8e-62",
        "0x2.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        200,
        1,
        Ceiling,
        "1.6e-61",
        "0x4.0E-51#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        200,
        1,
        Nearest,
        "7.8e-62",
        "0x2.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        200,
        10,
        Down,
        "1.1410e-61",
        "0x2.efE-51#10",
        "-0.99902",
        "-0x0.ffc#10",
        Less,
        Greater,
    );
    test(
        false,
        1,
        200,
        10,
        Up,
        "1.1425e-61",
        "0x2.f0E-51#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        1,
        200,
        10,
        Floor,
        "1.1410e-61",
        "0x2.efE-51#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1,
        200,
        10,
        Ceiling,
        "1.1425e-61",
        "0x2.f0E-51#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        200,
        10,
        Nearest,
        "1.1425e-61",
        "0x2.f0E-51#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        3,
        200,
        1,
        Down,
        "3.1e-61",
        "0x8.0E-51#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        false,
        3,
        200,
        1,
        Up,
        "6.2e-61",
        "0x1.0E-50#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        3,
        200,
        1,
        Floor,
        "3.1e-61",
        "0x8.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        3,
        200,
        1,
        Ceiling,
        "6.2e-61",
        "0x1.0E-50#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        3,
        200,
        1,
        Nearest,
        "3.1e-61",
        "0x8.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        3,
        200,
        10,
        Down,
        "3.4214e-61",
        "0x8.ccE-51#10",
        "-0.99902",
        "-0x0.ffc#10",
        Less,
        Greater,
    );
    test(
        false,
        3,
        200,
        10,
        Up,
        "3.4275e-61",
        "0x8.d0E-51#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        3,
        200,
        10,
        Floor,
        "3.4214e-61",
        "0x8.ccE-51#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        3,
        200,
        10,
        Nearest,
        "3.4275e-61",
        "0x8.d0E-51#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        -1,
        200,
        1,
        Down,
        "-7.8e-62",
        "-0x2.0E-51#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        -1,
        200,
        1,
        Up,
        "-1.6e-61",
        "-0x4.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        -1,
        200,
        1,
        Floor,
        "-1.6e-61",
        "-0x4.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        -1,
        200,
        1,
        Ceiling,
        "-7.8e-62",
        "-0x2.0E-51#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        -1,
        200,
        1,
        Nearest,
        "-7.8e-62",
        "-0x2.0E-51#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        -1,
        200,
        10,
        Down,
        "-1.1410e-61",
        "-0x2.efE-51#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        false,
        -1,
        200,
        10,
        Up,
        "-1.1425e-61",
        "-0x2.f0E-51#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        -1,
        200,
        10,
        Floor,
        "-1.1425e-61",
        "-0x2.f0E-51#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        5,
        300,
        1,
        Down,
        "2.0e-90",
        "0x4.0E-75#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        false,
        5,
        300,
        1,
        Up,
        "3.9e-90",
        "0x8.0E-75#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        5,
        300,
        1,
        Floor,
        "2.0e-90",
        "0x4.0E-75#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        5,
        300,
        1,
        Ceiling,
        "3.9e-90",
        "0x8.0E-75#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        5,
        300,
        1,
        Nearest,
        "2.0e-90",
        "0x4.0E-75#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        5,
        300,
        10,
        Down,
        "2.4661e-90",
        "0x5.06E-75#10",
        "-0.99902",
        "-0x0.ffc#10",
        Less,
        Greater,
    );
    test(
        false,
        5,
        300,
        10,
        Up,
        "2.4699e-90",
        "0x5.08E-75#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        5,
        300,
        10,
        Floor,
        "2.4661e-90",
        "0x5.06E-75#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        5,
        300,
        10,
        Ceiling,
        "2.4699e-90",
        "0x5.08E-75#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        false,
        5,
        300,
        10,
        Nearest,
        "2.4661e-90",
        "0x5.06E-75#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        -3,
        300,
        1,
        Down,
        "-9.8e-91",
        "-0x2.0E-75#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        -3,
        300,
        1,
        Up,
        "-2.0e-90",
        "-0x4.0E-75#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        -3,
        300,
        1,
        Floor,
        "-2.0e-90",
        "-0x4.0E-75#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        -3,
        300,
        1,
        Ceiling,
        "-9.8e-91",
        "-0x2.0E-75#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        -3,
        300,
        1,
        Nearest,
        "-2.0e-90",
        "-0x4.0E-75#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        -3,
        300,
        10,
        Down,
        "-1.4785e-90",
        "-0x3.03E-75#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        false,
        -3,
        300,
        10,
        Up,
        "-1.4804e-90",
        "-0x3.04E-75#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        -3,
        300,
        10,
        Floor,
        "-1.4804e-90",
        "-0x3.04E-75#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        -3,
        300,
        10,
        Nearest,
        "-1.4804e-90",
        "-0x3.04E-75#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        7,
        1000,
        1,
        Down,
        "7.5e-301",
        "0x8.0E-250#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        false,
        7,
        1000,
        1,
        Up,
        "1.5e-300",
        "0x1.0E-249#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        7,
        1000,
        1,
        Floor,
        "7.5e-301",
        "0x8.0E-250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        7,
        1000,
        1,
        Ceiling,
        "1.5e-300",
        "0x1.0E-249#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        7,
        1000,
        1,
        Nearest,
        "7.5e-301",
        "0x8.0E-250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        7,
        1000,
        10,
        Up,
        "8.7639e-301",
        "0x9.64E-250#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        false,
        7,
        1000,
        10,
        Floor,
        "8.7493e-301",
        "0x9.60E-250#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1,
        5000,
        1,
        Down,
        "-7.1e-1506",
        "-0x1.0E-1250#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        5000,
        1,
        Up,
        "-1.4e-1505",
        "-0x2.0E-1250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        5000,
        1,
        Floor,
        "-1.4e-1505",
        "-0x2.0E-1250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1,
        5000,
        1,
        Ceiling,
        "-7.1e-1506",
        "-0x1.0E-1250#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        false,
        1,
        5000,
        1,
        Nearest,
        "-7.1e-1506",
        "-0x1.0E-1250#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        1,
        5000,
        10,
        Up,
        "-9.7071e-1506",
        "-0x1.5f0E-1250#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        false,
        1048577,
        1000,
        1,
        Down,
        "9.8e-296",
        "0x1.0E-245#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        false,
        1048577,
        1000,
        1,
        Up,
        "2.0e-295",
        "0x2.0E-245#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        1048577,
        1000,
        1,
        Floor,
        "9.8e-296",
        "0x1.0E-245#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1048577,
        1000,
        1,
        Nearest,
        "9.8e-296",
        "0x1.0E-245#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        false,
        1048579,
        2000,
        1,
        Up,
        "9.1e-597",
        "0x1.0E-495#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        false,
        1048579,
        2000,
        1,
        Floor,
        "4.6e-597",
        "0x8.0E-496#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        150,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "-7.0e-46",
        "-0x4.0E-38#1",
        Greater,
        Less,
    );
    test(
        true,
        1,
        150,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "-7.0e-46",
        "-0x4.0E-38#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        150,
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "-3.5e-46",
        "-0x2.0E-38#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        150,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "-7.0e-46",
        "-0x4.0E-38#1",
        Greater,
        Less,
    );
    test(
        true,
        1,
        150,
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        "-6.8833e-46",
        "-0x3.eeE-38#10",
        Less,
        Greater,
    );
    test(
        true,
        1,
        150,
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "-6.8902e-46",
        "-0x3.efE-38#10",
        Greater,
        Less,
    );
    test(
        true,
        1,
        150,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "-6.8902e-46",
        "-0x3.efE-38#10",
        Less,
        Less,
    );
    test(
        true,
        1,
        150,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "-6.8902e-46",
        "-0x3.efE-38#10",
        Greater,
        Less,
    );
    test(
        true,
        1,
        190,
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "2.5e-60",
        "0x4.0E-50#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        190,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "5.0e-60",
        "0x8.0E-50#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        190,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "2.5e-60",
        "0x4.0E-50#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        190,
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "5.0e-60",
        "0x8.0E-50#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        190,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "2.5e-60",
        "0x4.0E-50#1",
        Greater,
        Less,
    );
    test(
        true,
        1,
        190,
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        "2.5427e-60",
        "0x4.16E-50#10",
        Less,
        Less,
    );
    test(
        true,
        1,
        190,
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "2.5475e-60",
        "0x4.18E-50#10",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        190,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "2.5427e-60",
        "0x4.16E-50#10",
        Less,
        Less,
    );
    test(
        true,
        1,
        190,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "2.5475e-60",
        "0x4.18E-50#10",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        190,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "2.5475e-60",
        "0x4.18E-50#10",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        200,
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "3.9e-62",
        "0x1.0E-51#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        200,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "7.8e-62",
        "0x2.0E-51#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        200,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "3.9e-62",
        "0x1.0E-51#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        200,
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "7.8e-62",
        "0x2.0E-51#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        200,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "3.9e-62",
        "0x1.0E-51#1",
        Greater,
        Less,
    );
    test(
        true,
        1,
        200,
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        "5.7049e-62",
        "0x1.778E-51#10",
        Less,
        Less,
    );
    test(
        true,
        1,
        200,
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "5.7125e-62",
        "0x1.780E-51#10",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        200,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "5.7049e-62",
        "0x1.778E-51#10",
        Less,
        Less,
    );
    test(
        true,
        1,
        200,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "5.7125e-62",
        "0x1.780E-51#10",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        200,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "5.7125e-62",
        "0x1.780E-51#10",
        Greater,
        Greater,
    );
    test(
        true,
        3,
        200,
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        "-1.6e-61",
        "-0x4.0E-51#1",
        Greater,
        Greater,
    );
    test(
        true,
        3,
        200,
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "-3.1e-61",
        "-0x8.0E-51#1",
        Less,
        Less,
    );
    test(
        true,
        3,
        200,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-3.1e-61",
        "-0x8.0E-51#1",
        Less,
        Less,
    );
    test(
        true,
        3,
        200,
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        "-1.6e-61",
        "-0x4.0E-51#1",
        Greater,
        Greater,
    );
    test(
        true,
        3,
        200,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.6e-61",
        "-0x4.0E-51#1",
        Less,
        Greater,
    );
    test(
        true,
        3,
        200,
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        "-1.7107e-61",
        "-0x4.66E-51#10",
        Greater,
        Greater,
    );
    test(
        true,
        3,
        200,
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        "-1.7138e-61",
        "-0x4.68E-51#10",
        Less,
        Less,
    );
    test(
        true,
        3,
        200,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.7138e-61",
        "-0x4.68E-51#10",
        Less,
        Less,
    );
    test(
        true,
        3,
        200,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.7138e-61",
        "-0x4.68E-51#10",
        Less,
        Less,
    );
    test(
        true,
        -1,
        200,
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        "3.9e-62",
        "0x1.0E-51#1",
        Greater,
        Less,
    );
    test(
        true,
        -1,
        200,
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "7.8e-62",
        "0x2.0E-51#1",
        Less,
        Greater,
    );
    test(
        true,
        -1,
        200,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "3.9e-62",
        "0x1.0E-51#1",
        Less,
        Less,
    );
    test(
        true,
        -1,
        200,
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        "7.8e-62",
        "0x2.0E-51#1",
        Greater,
        Greater,
    );
    test(
        true,
        -1,
        200,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "3.9e-62",
        "0x1.0E-51#1",
        Less,
        Less,
    );
    test(
        true,
        -1,
        200,
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        "5.7049e-62",
        "0x1.778E-51#10",
        Greater,
        Less,
    );
    test(
        true,
        -1,
        200,
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        "5.7125e-62",
        "0x1.780E-51#10",
        Less,
        Greater,
    );
    test(
        true,
        -1,
        200,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "5.7049e-62",
        "0x1.778E-51#10",
        Less,
        Less,
    );
    test(
        true,
        5,
        300,
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "9.8e-91",
        "0x2.0E-75#1",
        Less,
        Less,
    );
    test(
        true,
        5,
        300,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "2.0e-90",
        "0x4.0E-75#1",
        Greater,
        Greater,
    );
    test(
        true,
        5,
        300,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "9.8e-91",
        "0x2.0E-75#1",
        Less,
        Less,
    );
    test(
        true,
        5,
        300,
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "2.0e-90",
        "0x4.0E-75#1",
        Greater,
        Greater,
    );
    test(
        true,
        5,
        300,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "9.8e-91",
        "0x2.0E-75#1",
        Greater,
        Less,
    );
    test(
        true,
        5,
        300,
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        "1.2330e-90",
        "0x2.83E-75#10",
        Less,
        Less,
    );
    test(
        true,
        5,
        300,
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "1.2349e-90",
        "0x2.84E-75#10",
        Greater,
        Greater,
    );
    test(
        true,
        5,
        300,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "1.2330e-90",
        "0x2.83E-75#10",
        Less,
        Less,
    );
    test(
        true,
        5,
        300,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "1.2349e-90",
        "0x2.84E-75#10",
        Greater,
        Greater,
    );
    test(
        true,
        5,
        300,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "1.2330e-90",
        "0x2.83E-75#10",
        Greater,
        Less,
    );
    test(
        true,
        -3,
        300,
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "-4.9e-91",
        "-0x1.0E-75#1",
        Less,
        Greater,
    );
    test(
        true,
        -3,
        300,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "-9.8e-91",
        "-0x2.0E-75#1",
        Greater,
        Less,
    );
    test(
        true,
        -3,
        300,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "-9.8e-91",
        "-0x2.0E-75#1",
        Less,
        Less,
    );
    test(
        true,
        -3,
        300,
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "-4.9e-91",
        "-0x1.0E-75#1",
        Greater,
        Greater,
    );
    test(
        true,
        -3,
        300,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "-9.8e-91",
        "-0x2.0E-75#1",
        Greater,
        Less,
    );
    test(
        true,
        -3,
        300,
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        "-7.3924e-91",
        "-0x1.818E-75#10",
        Less,
        Greater,
    );
    test(
        true,
        -3,
        300,
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "-7.4020e-91",
        "-0x1.820E-75#10",
        Greater,
        Less,
    );
    test(
        true,
        -3,
        300,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "-7.4020e-91",
        "-0x1.820E-75#10",
        Less,
        Less,
    );
    test(
        true,
        -3,
        300,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "-7.4020e-91",
        "-0x1.820E-75#10",
        Greater,
        Less,
    );
    test(
        true,
        7,
        1000,
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        "-3.7e-301",
        "-0x4.0E-250#1",
        Greater,
        Greater,
    );
    test(
        true,
        7,
        1000,
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "-7.5e-301",
        "-0x8.0E-250#1",
        Less,
        Less,
    );
    test(
        true,
        7,
        1000,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-7.5e-301",
        "-0x8.0E-250#1",
        Less,
        Less,
    );
    test(
        true,
        7,
        1000,
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        "-3.7e-301",
        "-0x4.0E-250#1",
        Greater,
        Greater,
    );
    test(
        true,
        7,
        1000,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-3.7e-301",
        "-0x4.0E-250#1",
        Less,
        Greater,
    );
    test(
        true,
        7,
        1000,
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        "-4.3820e-301",
        "-0x4.b2E-250#10",
        Less,
        Less,
    );
    test(
        true,
        7,
        1000,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-4.3820e-301",
        "-0x4.b2E-250#10",
        Less,
        Less,
    );
    test(
        true,
        1,
        5000,
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "-3.5e-1506",
        "-0x8.0E-1251#1",
        Less,
        Greater,
    );
    test(
        true,
        1,
        5000,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "-7.1e-1506",
        "-0x1.0E-1250#1",
        Greater,
        Less,
    );
    test(
        true,
        1,
        5000,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "-7.1e-1506",
        "-0x1.0E-1250#1",
        Less,
        Less,
    );
    test(
        true,
        1,
        5000,
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "-3.5e-1506",
        "-0x8.0E-1251#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        5000,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "-3.5e-1506",
        "-0x8.0E-1251#1",
        Greater,
        Greater,
    );
    test(
        true,
        1,
        5000,
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "-4.8535e-1506",
        "-0xa.f8E-1251#10",
        Greater,
        Less,
    );
    test(
        true,
        1048577,
        1000,
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "4.9e-296",
        "0x8.0E-246#1",
        Less,
        Less,
    );
    test(
        true,
        1048577,
        1000,
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "9.8e-296",
        "0x1.0E-245#1",
        Greater,
        Greater,
    );
    test(
        true,
        1048577,
        1000,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "4.9e-296",
        "0x8.0E-246#1",
        Less,
        Less,
    );
    test(
        true,
        1048577,
        1000,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "4.9e-296",
        "0x8.0E-246#1",
        Greater,
        Less,
    );
    test(
        true,
        1048579,
        2000,
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "-4.6e-597",
        "-0x8.0E-496#1",
        Less,
        Less,
    );
    test(
        true,
        1048579,
        2000,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-4.6e-597",
        "-0x8.0E-496#1",
        Less,
        Less,
    );
}

// Inputs within 2^(-2^30) of pi and of pi/2, where the sine or the cosine underflows. Each call
// computes pi to about 2^30 bits, so this test is slow even in release mode.
#[test]
fn test_sin_cos_underflow() {
    let p = (1u64 << 30) + 64;
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    // pi rounded down: the sine is positive and tiny, and the cosine is just above -1
    let pi = Float::pi_prec_round(p, Floor).0;
    let (s, c, o_s, o_c) = pi.sin_cos_prec_round_ref(10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o_c, Greater);
    // pi/2 rounded down: the cosine is positive and tiny, and the sine is just below 1
    let half_pi = &pi >> 1u32;
    let (s, c, o_s, o_c) = half_pi.sin_cos_prec_round_ref(10, Floor);
    assert_eq!(s.to_string(), "0.99902");
    assert_eq!(o_s, Less);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o_c, Less);
    let _ = Rational::ZERO;
}

#[test]
fn test_sin_cos_rational_prec_round() {
    let test = |s: &str,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (sin, cos, o_s, o_c) = Float::sin_cos_rational_prec_round(x.clone(), prec, rm);
        assert!(sin.is_valid());
        assert!(cos.is_valid());
        assert_eq!(sin.to_string(), out_s);
        assert_eq!(to_hex_string(&sin), out_s_hex);
        assert_eq!(cos.to_string(), out_c);
        assert_eq!(to_hex_string(&cos), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if rm == Nearest {
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) = Float::sin_cos_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
        }

        // the two results are those of the separate functions
        let (sin_alt, o_s_alt) = Float::sin_rational_prec_round_ref(&x, prec, rm);
        let (cos_alt, o_c_alt) = Float::cos_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    };
    test(
        "0", 1, Floor, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    // Rows reuse the sine's `Rational` inputs, covering the cosine-rounds-to-1 delegation, the
    // exactly representable inputs, the general bracket, and the huge-input reduction.
    test("0", 1, Down, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal);
    test("0", 1, Up, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal);
    test(
        "0", 1, Floor, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0", 1, Ceiling, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0", 1, Nearest, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0", 5, Nearest, "0.0", "0x0.0", "1.00", "0x1.0#5", Equal, Equal,
    );
    test(
        "0",
        10,
        Down,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Up,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        20,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000",
        "0x1.00000#20",
        Equal,
        Equal,
    );
    test(
        "0",
        53,
        Down,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "0",
        53,
        Up,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "0",
        53,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "0",
        53,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "0",
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "0",
        100,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
        Equal,
    );
    test(
        "1", 1, Down, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1", 1, Up, "1.0", "0x1.0#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1", 1, Floor, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1", 1, Ceiling, "1.0", "0x1.0#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1", 1, Nearest, "1.0", "0x1.0#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "1", 5, Nearest, "0.844", "0x0.d8#5", "0.531", "0x0.88#5", Greater, Less,
    );
    test(
        "1",
        10,
        Down,
        "0.84082",
        "0x0.d74#10",
        "0.54004",
        "0x0.8a4#10",
        Less,
        Less,
    );
    test(
        "1",
        10,
        Up,
        "0.84180",
        "0x0.d78#10",
        "0.54102",
        "0x0.8a8#10",
        Greater,
        Greater,
    );
    test(
        "1",
        10,
        Floor,
        "0.84082",
        "0x0.d74#10",
        "0.54004",
        "0x0.8a4#10",
        Less,
        Less,
    );
    test(
        "1",
        10,
        Ceiling,
        "0.84180",
        "0x0.d78#10",
        "0.54102",
        "0x0.8a8#10",
        Greater,
        Greater,
    );
    test(
        "1",
        10,
        Nearest,
        "0.84180",
        "0x0.d78#10",
        "0.54004",
        "0x0.8a4#10",
        Greater,
        Less,
    );
    test(
        "1",
        20,
        Nearest,
        "0.84147072",
        "0x0.d76aa#20",
        "0.54030228",
        "0x0.8a514#20",
        Less,
        Less,
    );
    test(
        "1",
        53,
        Down,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
        Less,
    );
    test(
        "1",
        53,
        Up,
        "0.84147098480789662",
        "0x0.d76aa478486778#53",
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "0.84147098480789662",
        "0x0.d76aa478486778#53",
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Less,
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Less,
        Greater,
    );
    test(
        "-1", 1, Down, "-0.50", "-0x0.8#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "-1", 1, Up, "-1.0", "-0x1.0#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "-1", 1, Floor, "-1.0", "-0x1.0#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "-1", 1, Ceiling, "-0.50", "-0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "-1", 1, Nearest, "-1.0", "-0x1.0#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "-1",
        5,
        Nearest,
        "-0.844",
        "-0x0.d8#5",
        "0.531",
        "0x0.88#5",
        Less,
        Less,
    );
    test(
        "-1",
        10,
        Down,
        "-0.84082",
        "-0x0.d74#10",
        "0.54004",
        "0x0.8a4#10",
        Greater,
        Less,
    );
    test(
        "-1",
        10,
        Up,
        "-0.84180",
        "-0x0.d78#10",
        "0.54102",
        "0x0.8a8#10",
        Less,
        Greater,
    );
    test(
        "-1",
        10,
        Floor,
        "-0.84180",
        "-0x0.d78#10",
        "0.54004",
        "0x0.8a4#10",
        Less,
        Less,
    );
    test(
        "-1",
        10,
        Ceiling,
        "-0.84082",
        "-0x0.d74#10",
        "0.54102",
        "0x0.8a8#10",
        Greater,
        Greater,
    );
    test(
        "-1",
        10,
        Nearest,
        "-0.84180",
        "-0x0.d78#10",
        "0.54004",
        "0x0.8a4#10",
        Less,
        Less,
    );
    test(
        "-1",
        20,
        Nearest,
        "-0.84147072",
        "-0x0.d76aa#20",
        "0.54030228",
        "0x0.8a514#20",
        Greater,
        Less,
    );
    test(
        "-1",
        53,
        Down,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Greater,
        Less,
    );
    test(
        "-1",
        53,
        Up,
        "-0.84147098480789662",
        "-0x0.d76aa478486778#53",
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Less,
        Greater,
    );
    test(
        "-1",
        53,
        Floor,
        "-0.84147098480789662",
        "-0x0.d76aa478486778#53",
        "0.54030230586813965",
        "0x0.8a51407da83458#53",
        Less,
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        "0.54030230586813977",
        "0x0.8a51407da83460#53",
        Greater,
        Greater,
    );
    test(
        "-1",
        100,
        Nearest,
        "-0.84147098480789650665250232163005",
        "-0x0.d76aa47848677020c6e9e909c#100",
        "0.54030230586813971740093660744335",
        "0x0.8a51407da8345c91c2466d977#100",
        Greater,
        Greater,
    );
    test(
        "1/2", 1, Down, "0.25", "0x0.4#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1/2", 1, Up, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1/2", 1, Floor, "0.25", "0x0.4#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1/2", 1, Ceiling, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1/2", 1, Nearest, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1/2", 5, Nearest, "0.484", "0x0.7c#5", "0.875", "0x0.e0#5", Greater, Less,
    );
    test(
        "1/2",
        10,
        Down,
        "0.47900",
        "0x0.7aa#10",
        "0.87695",
        "0x0.e08#10",
        Less,
        Less,
    );
    test(
        "1/2",
        10,
        Up,
        "0.47949",
        "0x0.7ac#10",
        "0.87793",
        "0x0.e0c#10",
        Greater,
        Greater,
    );
    test(
        "1/2",
        10,
        Floor,
        "0.47900",
        "0x0.7aa#10",
        "0.87695",
        "0x0.e08#10",
        Less,
        Less,
    );
    test(
        "1/2",
        10,
        Ceiling,
        "0.47949",
        "0x0.7ac#10",
        "0.87793",
        "0x0.e0c#10",
        Greater,
        Greater,
    );
    test(
        "1/2",
        10,
        Nearest,
        "0.47949",
        "0x0.7ac#10",
        "0.87793",
        "0x0.e0c#10",
        Greater,
        Greater,
    );
    test(
        "1/2",
        20,
        Nearest,
        "0.47942543",
        "0x0.7abba0#20",
        "0.87758255",
        "0x0.e0a94#20",
        Less,
        Less,
    );
    test(
        "1/2",
        53,
        Down,
        "0.47942553860420295",
        "0x0.7abba1d12c17bc#53",
        "0.87758256189037265",
        "0x0.e0a94032dbea78#53",
        Less,
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "0.47942553860420295",
        "0x0.7abba1d12c17bc#53",
        "0.87758256189037265",
        "0x0.e0a94032dbea78#53",
        Less,
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        "0.87758256189037276",
        "0x0.e0a94032dbea80#53",
        Greater,
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.47942553860420300027328793521567",
        "0x0.7abba1d12c17bfa1d92f0d93f8#100",
        "0.87758256189037271611628158260408",
        "0x0.e0a94032dbea7cedbddd9da30#100",
        Greater,
        Greater,
    );
    test(
        "1/3", 1, Down, "0.25", "0x0.4#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1/3", 1, Up, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1/3", 1, Floor, "0.25", "0x0.4#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1/3", 1, Ceiling, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1/3", 1, Nearest, "0.25", "0x0.4#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "1/3", 5, Nearest, "0.328", "0x0.54#5", "0.938", "0x0.f0#5", Greater, Less,
    );
    test(
        "1/3",
        10,
        Down,
        "0.32715",
        "0x0.53c#10",
        "0.94434",
        "0x0.f1c#10",
        Less,
        Less,
    );
    test(
        "1/3",
        10,
        Up,
        "0.32764",
        "0x0.53e#10",
        "0.94531",
        "0x0.f20#10",
        Greater,
        Greater,
    );
    test(
        "1/3",
        10,
        Floor,
        "0.32715",
        "0x0.53c#10",
        "0.94434",
        "0x0.f1c#10",
        Less,
        Less,
    );
    test(
        "1/3",
        10,
        Ceiling,
        "0.32764",
        "0x0.53e#10",
        "0.94531",
        "0x0.f20#10",
        Greater,
        Greater,
    );
    test(
        "1/3",
        10,
        Nearest,
        "0.32715",
        "0x0.53c#10",
        "0.94531",
        "0x0.f20#10",
        Less,
        Greater,
    );
    test(
        "1/3",
        20,
        Nearest,
        "0.32719469",
        "0x0.53c308#20",
        "0.94495678",
        "0x0.f1e8b#20",
        Less,
        Less,
    );
    test(
        "1/3",
        53,
        Down,
        "0.32719469679615221",
        "0x0.53c3081a2a0318#53",
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "0.32719469679615221",
        "0x0.53c3081a2a0318#53",
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.32719469679615224417334408526753",
        "0x0.53c3081a2a031ab144c484c790#100",
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d#100",
        Less,
        Less,
    );
    test(
        "-1/3", 1, Down, "-0.25", "-0x0.4#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "-1/3", 1, Up, "-0.50", "-0x0.8#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "-1/3", 1, Floor, "-0.50", "-0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "-1/3", 1, Ceiling, "-0.25", "-0x0.4#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "-1/3", 1, Nearest, "-0.25", "-0x0.4#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "-1/3",
        5,
        Nearest,
        "-0.328",
        "-0x0.54#5",
        "0.938",
        "0x0.f0#5",
        Less,
        Less,
    );
    test(
        "-1/3",
        10,
        Down,
        "-0.32715",
        "-0x0.53c#10",
        "0.94434",
        "0x0.f1c#10",
        Greater,
        Less,
    );
    test(
        "-1/3",
        10,
        Up,
        "-0.32764",
        "-0x0.53e#10",
        "0.94531",
        "0x0.f20#10",
        Less,
        Greater,
    );
    test(
        "-1/3",
        10,
        Floor,
        "-0.32764",
        "-0x0.53e#10",
        "0.94434",
        "0x0.f1c#10",
        Less,
        Less,
    );
    test(
        "-1/3",
        10,
        Ceiling,
        "-0.32715",
        "-0x0.53c#10",
        "0.94531",
        "0x0.f20#10",
        Greater,
        Greater,
    );
    test(
        "-1/3",
        10,
        Nearest,
        "-0.32715",
        "-0x0.53c#10",
        "0.94531",
        "0x0.f20#10",
        Greater,
        Greater,
    );
    test(
        "-1/3",
        20,
        Nearest,
        "-0.32719469",
        "-0x0.53c308#20",
        "0.94495678",
        "0x0.f1e8b#20",
        Greater,
        Less,
    );
    test(
        "-1/3",
        53,
        Down,
        "-0.32719469679615221",
        "-0x0.53c3081a2a0318#53",
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Greater,
        Less,
    );
    test(
        "-1/3",
        53,
        Up,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Less,
        Greater,
    );
    test(
        "-1/3",
        53,
        Floor,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        "0.94495694631473759",
        "0x0.f1e8b2cc8cc160#53",
        Less,
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-0.32719469679615221",
        "-0x0.53c3081a2a0318#53",
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Greater,
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        "0.94495694631473770",
        "0x0.f1e8b2cc8cc168#53",
        Less,
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.32719469679615224417334408526753",
        "-0x0.53c3081a2a031ab144c484c790#100",
        "0.94495694631473766438828400767583",
        "0x0.f1e8b2cc8cc1656b6998d964d#100",
        Greater,
        Less,
    );
    test(
        "3/5", 1, Down, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "3/5", 1, Up, "1.0", "0x1.0#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "3/5", 1, Floor, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "3/5", 1, Ceiling, "1.0", "0x1.0#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "3/5", 1, Nearest, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "3/5",
        10,
        Down,
        "0.56445",
        "0x0.908#10",
        "0.82520",
        "0x0.d34#10",
        Less,
        Less,
    );
    test(
        "3/5",
        10,
        Up,
        "0.56543",
        "0x0.90c#10",
        "0.82617",
        "0x0.d38#10",
        Greater,
        Greater,
    );
    test(
        "3/5",
        10,
        Floor,
        "0.56445",
        "0x0.908#10",
        "0.82520",
        "0x0.d34#10",
        Less,
        Less,
    );
    test(
        "3/5",
        10,
        Ceiling,
        "0.56543",
        "0x0.90c#10",
        "0.82617",
        "0x0.d38#10",
        Greater,
        Greater,
    );
    test(
        "3/5",
        10,
        Nearest,
        "0.56445",
        "0x0.908#10",
        "0.82520",
        "0x0.d34#10",
        Less,
        Less,
    );
    test(
        "3/5",
        53,
        Down,
        "0.56464247339503526",
        "0x0.908c68bd2a0ac0#53",
        "0.82533561490967822",
        "0x0.d34931e242d8a0#53",
        Less,
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "0.56464247339503526",
        "0x0.908c68bd2a0ac0#53",
        "0.82533561490967822",
        "0x0.d34931e242d8a0#53",
        Less,
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        "0.82533561490967833",
        "0x0.d34931e242d8a8#53",
        Greater,
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "0.56464247339503535720094544565865",
        "0x0.908c68bd2a0ac6fa881b0a82b#100",
        "0.82533561490967829724095249895546",
        "0x0.d34931e242d8a5cb448dccbdf#100",
        Less,
        Greater,
    );
    test(
        "22/7",
        1,
        Down,
        "-0.00098",
        "-0x0.004#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "22/7",
        1,
        Up,
        "-0.0020",
        "-0x0.008#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "22/7",
        1,
        Floor,
        "-0.0020",
        "-0x0.008#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "22/7",
        1,
        Ceiling,
        "-0.00098",
        "-0x0.004#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "22/7",
        1,
        Nearest,
        "-0.00098",
        "-0x0.004#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        "22/7",
        5,
        Nearest,
        "-0.00128",
        "-0x0.0054#5",
        "-1.00",
        "-0x1.0#5",
        Less,
        Less,
    );
    test(
        "22/7",
        10,
        Down,
        "-0.0012627",
        "-0x0.0052c#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "22/7",
        10,
        Up,
        "-0.0012646",
        "-0x0.0052e#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "22/7",
        10,
        Floor,
        "-0.0012646",
        "-0x0.0052e#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "22/7",
        10,
        Ceiling,
        "-0.0012627",
        "-0x0.0052c#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "22/7",
        10,
        Nearest,
        "-0.0012646",
        "-0x0.0052e#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "22/7",
        20,
        Nearest,
        "-0.0012644883",
        "-0x0.0052de98#20",
        "-0.99999905",
        "-0x0.fffff#20",
        Greater,
        Greater,
    );
    test(
        "22/7",
        53,
        Down,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
        Greater,
    );
    test(
        "22/7",
        53,
        Up,
        "-0.0012644889303773535",
        "-0x0.0052de9a9a24d910#53",
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
        Less,
    );
    test(
        "22/7",
        53,
        Floor,
        "-0.0012644889303773535",
        "-0x0.0052de9a9a24d910#53",
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Greater,
        Less,
    );
    test(
        "22/7",
        100,
        Nearest,
        "-0.0012644889303773534003603504756467",
        "-0x0.0052de9a9a24d90da7cdffe30130#100",
        "-0.99999920053355290326833573965634",
        "-0x0.fffff29651586c28bc01f9fb2#100",
        Less,
        Greater,
    );
    test(
        "-22/7",
        1,
        Down,
        "0.00098",
        "0x0.004#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        "-22/7",
        1,
        Up,
        "0.0020",
        "0x0.008#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        "-22/7",
        1,
        Floor,
        "0.00098",
        "0x0.004#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "-22/7",
        1,
        Ceiling,
        "0.0020",
        "0x0.008#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "-22/7",
        1,
        Nearest,
        "0.00098",
        "0x0.004#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "-22/7",
        5,
        Nearest,
        "0.00128",
        "0x0.0054#5",
        "-1.00",
        "-0x1.0#5",
        Greater,
        Less,
    );
    test(
        "-22/7",
        10,
        Down,
        "0.0012627",
        "0x0.0052c#10",
        "-0.99902",
        "-0x0.ffc#10",
        Less,
        Greater,
    );
    test(
        "-22/7",
        10,
        Up,
        "0.0012646",
        "0x0.0052e#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        "-22/7",
        10,
        Floor,
        "0.0012627",
        "0x0.0052c#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "-22/7",
        10,
        Ceiling,
        "0.0012646",
        "0x0.0052e#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "-22/7",
        10,
        Nearest,
        "0.0012646",
        "0x0.0052e#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        "-22/7",
        20,
        Nearest,
        "0.0012644883",
        "0x0.0052de98#20",
        "-0.99999905",
        "-0x0.fffff#20",
        Less,
        Greater,
    );
    test(
        "-22/7",
        53,
        Down,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Less,
        Greater,
    );
    test(
        "-22/7",
        53,
        Up,
        "0.0012644889303773535",
        "0x0.0052de9a9a24d910#53",
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Greater,
        Less,
    );
    test(
        "-22/7",
        53,
        Floor,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "0.0012644889303773535",
        "0x0.0052de9a9a24d910#53",
        "-0.99999920053355285",
        "-0x0.fffff296515868#53",
        Greater,
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        "-0.99999920053355296",
        "-0x0.fffff296515870#53",
        Less,
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "0.0012644889303773534003603504756467",
        "0x0.0052de9a9a24d90da7cdffe30130#100",
        "-0.99999920053355290326833573965634",
        "-0x0.fffff29651586c28bc01f9fb2#100",
        Greater,
        Greater,
    );
    test(
        "355/113",
        1,
        Down,
        "-2.4e-7",
        "-0x4.0E-6#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "355/113",
        1,
        Up,
        "-4.8e-7",
        "-0x8.0E-6#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "355/113",
        1,
        Floor,
        "-4.8e-7",
        "-0x8.0E-6#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "355/113",
        1,
        Ceiling,
        "-2.4e-7",
        "-0x4.0E-6#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "355/113",
        1,
        Nearest,
        "-2.4e-7",
        "-0x4.0E-6#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        "355/113",
        5,
        Nearest,
        "-2.68e-7",
        "-0x4.8E-6#5",
        "-1.00",
        "-0x1.0#5",
        Less,
        Less,
    );
    test(
        "355/113",
        10,
        Down,
        "-2.6636e-7",
        "-0x4.78E-6#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "355/113",
        10,
        Up,
        "-2.6682e-7",
        "-0x4.7aE-6#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "355/113",
        10,
        Floor,
        "-2.6682e-7",
        "-0x4.7aE-6#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "355/113",
        10,
        Ceiling,
        "-2.6636e-7",
        "-0x4.78E-6#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "355/113",
        10,
        Nearest,
        "-2.6682e-7",
        "-0x4.7aE-6#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "355/113",
        20,
        Nearest,
        "-2.6676435e-7",
        "-0x4.79be8E-6#20",
        "-1.0000000",
        "-0x1.00000#20",
        Less,
        Less,
    );
    test(
        "355/113",
        53,
        Down,
        "-2.6676418906241912e-7",
        "-0x4.79be53e7511d4E-6#53",
        "-0.99999999999996436",
        "-0x0.fffffffffff5f8#53",
        Greater,
        Greater,
    );
    test(
        "355/113",
        53,
        Up,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
        Less,
    );
    test(
        "355/113",
        53,
        Floor,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "-2.6676418906241912e-7",
        "-0x4.79be53e7511d4E-6#53",
        "-0.99999999999996436",
        "-0x0.fffffffffff5f8#53",
        Greater,
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        "-0.99999999999996447",
        "-0x0.fffffffffff600#53",
        Less,
        Less,
    );
    test(
        "355/113",
        100,
        Nearest,
        "-2.6676418906241914840637452887346e-7",
        "-0x4.79be53e7511d64791a3045970E-6#100",
        "-0.99999999999996441843371693431264",
        "-0x0.fffffffffff5fc13f3fa12934#100",
        Greater,
        Greater,
    );
    test(
        "3", 1, Down, "0.12", "0x0.2#1", "-0.50", "-0x0.8#1", Less, Greater,
    );
    test(
        "3", 1, Up, "0.25", "0x0.4#1", "-1.0", "-0x1.0#1", Greater, Less,
    );
    test(
        "3", 1, Floor, "0.12", "0x0.2#1", "-1.0", "-0x1.0#1", Less, Less,
    );
    test(
        "3", 1, Ceiling, "0.25", "0x0.4#1", "-0.50", "-0x0.8#1", Greater, Greater,
    );
    test(
        "3", 1, Nearest, "0.12", "0x0.2#1", "-1.0", "-0x1.0#1", Less, Less,
    );
    test(
        "3", 5, Nearest, "0.141", "0x0.24#5", "-1.00", "-0x1.0#5", Less, Less,
    );
    test(
        "3",
        10,
        Down,
        "0.14111",
        "0x0.242#10",
        "-0.98926",
        "-0x0.fd4#10",
        Less,
        Greater,
    );
    test(
        "3",
        10,
        Up,
        "0.14136",
        "0x0.243#10",
        "-0.99023",
        "-0x0.fd8#10",
        Greater,
        Less,
    );
    test(
        "3",
        10,
        Floor,
        "0.14111",
        "0x0.242#10",
        "-0.99023",
        "-0x0.fd8#10",
        Less,
        Less,
    );
    test(
        "3",
        10,
        Ceiling,
        "0.14136",
        "0x0.243#10",
        "-0.98926",
        "-0x0.fd4#10",
        Greater,
        Greater,
    );
    test(
        "3",
        10,
        Nearest,
        "0.14111",
        "0x0.242#10",
        "-0.99023",
        "-0x0.fd8#10",
        Less,
        Less,
    );
    test(
        "3",
        20,
        Nearest,
        "0.14111996",
        "0x0.242070#20",
        "-0.98999214",
        "-0x0.fd702#20",
        Less,
        Greater,
    );
    test(
        "3",
        53,
        Down,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Less,
        Greater,
    );
    test(
        "3",
        53,
        Up,
        "0.14112000805986724",
        "0x0.242070db6daab8#53",
        "-0.98999249660044553",
        "-0x0.fd7025f42f2e98#53",
        Greater,
        Less,
    );
    test(
        "3",
        53,
        Floor,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        "-0.98999249660044553",
        "-0x0.fd7025f42f2e98#53",
        Less,
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "0.14112000805986724",
        "0x0.242070db6daab8#53",
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Greater,
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        "-0.98999249660044542",
        "-0x0.fd7025f42f2e90#53",
        Less,
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "0.14112000805986722210074480280816",
        "0x0.242070db6daab69e3902e84684#100",
        "-0.98999249660044545727157279473154",
        "-0x0.fd7025f42f2e9307dff82fdf7#100",
        Greater,
        Less,
    );
    test(
        "100", 1, Down, "-0.50", "-0x0.8#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "100", 1, Up, "-1.0", "-0x1.0#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "100", 1, Floor, "-1.0", "-0x1.0#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "100", 1, Ceiling, "-0.50", "-0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "100", 1, Nearest, "-0.50", "-0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "100",
        5,
        Nearest,
        "-0.500",
        "-0x0.80#5",
        "0.875",
        "0x0.e0#5",
        Greater,
        Greater,
    );
    test(
        "100",
        10,
        Down,
        "-0.50586",
        "-0x0.818#10",
        "0.86230",
        "0x0.dcc#10",
        Greater,
        Less,
    );
    test(
        "100",
        10,
        Up,
        "-0.50684",
        "-0x0.81c#10",
        "0.86328",
        "0x0.dd0#10",
        Less,
        Greater,
    );
    test(
        "100",
        10,
        Floor,
        "-0.50684",
        "-0x0.81c#10",
        "0.86230",
        "0x0.dcc#10",
        Less,
        Less,
    );
    test(
        "100",
        10,
        Ceiling,
        "-0.50586",
        "-0x0.818#10",
        "0.86328",
        "0x0.dd0#10",
        Greater,
        Greater,
    );
    test(
        "100",
        10,
        Nearest,
        "-0.50684",
        "-0x0.81c#10",
        "0.86230",
        "0x0.dcc#10",
        Less,
        Less,
    );
    test(
        "100",
        20,
        Nearest,
        "-0.50636578",
        "-0x0.81a13#20",
        "0.86231899",
        "0x0.dcc0f#20",
        Less,
        Greater,
    );
    test(
        "100",
        53,
        Down,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Greater,
        Less,
    );
    test(
        "100",
        53,
        Up,
        "-0.50636564110975890",
        "-0x0.81a12dbc626dc8#53",
        "0.86231887228768400",
        "0x0.dcc0edfb32ff00#53",
        Less,
        Greater,
    );
    test(
        "100",
        53,
        Floor,
        "-0.50636564110975890",
        "-0x0.81a12dbc626dc8#53",
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Less,
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        "0.86231887228768400",
        "0x0.dcc0edfb32ff00#53",
        Greater,
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        "0.86231887228768389",
        "0x0.dcc0edfb32fef8#53",
        Greater,
        Less,
    );
    test(
        "100",
        100,
        Nearest,
        "-0.50636564110975879365655761045969",
        "-0x0.81a12dbc626dc03847b0aae84#100",
        "0.86231887228768393410193851395099",
        "0x0.dcc0edfb32fefb1fa19b9b30c#100",
        Greater,
        Greater,
    );
    test(
        "1000000", 1, Down, "-0.25", "-0x0.4#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "1000000", 1, Up, "-0.50", "-0x0.8#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "1000000", 1, Floor, "-0.50", "-0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "1000000", 1, Ceiling, "-0.25", "-0x0.4#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1000000", 1, Nearest, "-0.25", "-0x0.4#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "1000000",
        5,
        Nearest,
        "-0.344",
        "-0x0.58#5",
        "0.938",
        "0x0.f0#5",
        Greater,
        Greater,
    );
    test(
        "1000000",
        10,
        Down,
        "-0.34961",
        "-0x0.598#10",
        "0.93652",
        "0x0.efc#10",
        Greater,
        Less,
    );
    test(
        "1000000",
        10,
        Up,
        "-0.35010",
        "-0x0.59a#10",
        "0.93750",
        "0x0.f00#10",
        Less,
        Greater,
    );
    test(
        "1000000",
        10,
        Floor,
        "-0.35010",
        "-0x0.59a#10",
        "0.93652",
        "0x0.efc#10",
        Less,
        Less,
    );
    test(
        "1000000",
        10,
        Ceiling,
        "-0.34961",
        "-0x0.598#10",
        "0.93750",
        "0x0.f00#10",
        Greater,
        Greater,
    );
    test(
        "1000000",
        10,
        Nearest,
        "-0.35010",
        "-0x0.59a#10",
        "0.93652",
        "0x0.efc#10",
        Less,
        Less,
    );
    test(
        "1000000",
        20,
        Nearest,
        "-0.34999371",
        "-0x0.599930#20",
        "0.93675232",
        "0x0.efcf0#20",
        Less,
        Greater,
    );
    test(
        "1000000",
        53,
        Down,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Greater,
        Less,
    );
    test(
        "1000000",
        53,
        Up,
        "-0.34999350217129299",
        "-0x0.59992c95a361a0#53",
        "0.93675212753314485",
        "0x0.efcefcc8369968#53",
        Less,
        Greater,
    );
    test(
        "1000000",
        53,
        Floor,
        "-0.34999350217129299",
        "-0x0.59992c95a361a0#53",
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Less,
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        "0.93675212753314485",
        "0x0.efcefcc8369968#53",
        Greater,
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        "0.93675212753314474",
        "0x0.efcefcc8369960#53",
        Greater,
        Less,
    );
    test(
        "1000000",
        100,
        Nearest,
        "-0.34999350217129295211765248678059",
        "-0x0.59992c95a3619d264732d26e98#100",
        "0.93675212753314478693853253507492",
        "0x0.efcefcc836996357644d418cd#100",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        1,
        Down,
        "9.5e-7",
        "0x0.00001#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "1/1000000",
        1,
        Up,
        "1.9e-6",
        "0x0.00002#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        1,
        Floor,
        "9.5e-7",
        "0x0.00001#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "1/1000000",
        1,
        Ceiling,
        "1.9e-6",
        "0x0.00002#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        1,
        Nearest,
        "9.5e-7",
        "0x0.00001#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "1/1000000",
        5,
        Nearest,
        "1.01e-6",
        "0x0.000011#5",
        "1.00",
        "0x1.0#5",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        10,
        Down,
        "9.9838e-7",
        "0x0.000010c0#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1000000",
        10,
        Up,
        "1.0002e-6",
        "0x0.000010c8#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        10,
        Floor,
        "9.9838e-7",
        "0x0.000010c0#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1000000",
        10,
        Ceiling,
        "1.0002e-6",
        "0x0.000010c8#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        10,
        Nearest,
        "1.0002e-6",
        "0x0.000010c8#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        20,
        Nearest,
        "1.0000003e-6",
        "0x0.000010c6f8#20",
        "1.0000000",
        "0x1.00000#20",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        53,
        Down,
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "9.9999999999983351e-7",
        "0x0.000010c6f7a0b5ea7b#53",
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "9.9999999999983351e-7",
        "0x0.000010c6f7a0b5ea7b#53",
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
        Less,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "9.9999999999983333333333334166635e-7",
        "0x0.000010c6f7a0b5ea7a2711cf92d7e2#100",
        "0.99999999999950000000000004166665",
        "0x0.ffffffffff734333f690bc5c6#100",
        Less,
        Less,
    );
    test(
        "-1/1000000",
        1,
        Down,
        "-9.5e-7",
        "-0x0.00001#1",
        "0.50",
        "0x0.8#1",
        Greater,
        Less,
    );
    test(
        "-1/1000000",
        1,
        Up,
        "-1.9e-6",
        "-0x0.00002#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "-1/1000000",
        1,
        Floor,
        "-1.9e-6",
        "-0x0.00002#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "-1/1000000",
        1,
        Ceiling,
        "-9.5e-7",
        "-0x0.00001#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "-1/1000000",
        1,
        Nearest,
        "-9.5e-7",
        "-0x0.00001#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "-1/1000000",
        5,
        Nearest,
        "-1.01e-6",
        "-0x0.000011#5",
        "1.00",
        "0x1.0#5",
        Less,
        Greater,
    );
    test(
        "-1/1000000",
        10,
        Down,
        "-9.9838e-7",
        "-0x0.000010c0#10",
        "0.99902",
        "0x0.ffc#10",
        Greater,
        Less,
    );
    test(
        "-1/1000000",
        10,
        Up,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "-1/1000000",
        10,
        Floor,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "-1/1000000",
        10,
        Ceiling,
        "-9.9838e-7",
        "-0x0.000010c0#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "-1/1000000",
        10,
        Nearest,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "-1/1000000",
        20,
        Nearest,
        "-1.0000003e-6",
        "-0x0.000010c6f8#20",
        "1.0000000",
        "0x1.00000#20",
        Less,
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Down,
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Greater,
        Less,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "-9.9999999999983351e-7",
        "-0x0.000010c6f7a0b5ea7b#53",
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Less,
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "-9.9999999999983351e-7",
        "-0x0.000010c6f7a0b5ea7b#53",
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Less,
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        "0.99999999999950007",
        "0x0.ffffffffff7348#53",
        Greater,
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        "0.99999999999949996",
        "0x0.ffffffffff7340#53",
        Greater,
        Less,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-9.9999999999983333333333334166635e-7",
        "-0x0.000010c6f7a0b5ea7a2711cf92d7e2#100",
        "0.99999999999950000000000004166665",
        "0x0.ffffffffff734333f690bc5c6#100",
        Greater,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Down,
        "8.3e-25",
        "0x1.0E-20#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Up,
        "1.7e-24",
        "0x2.0E-20#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Floor,
        "8.3e-25",
        "0x1.0E-20#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Ceiling,
        "1.7e-24",
        "0x2.0E-20#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Nearest,
        "8.3e-25",
        "0x1.0E-20#1",
        "1.0",
        "0x1.0#1",
        Less,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        Nearest,
        "9.82e-25",
        "0x1.3E-20#5",
        "1.00",
        "0x1.0#5",
        Less,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Down,
        "9.9843e-25",
        "0x1.350E-20#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Up,
        "1.0000e-24",
        "0x1.358E-20#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Floor,
        "9.9843e-25",
        "0x1.350E-20#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Ceiling,
        "1.0000e-24",
        "0x1.358E-20#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Nearest,
        "1.0000e-24",
        "0x1.358E-20#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        20,
        Nearest,
        "9.9999953e-25",
        "0x1.357c2E-20#20",
        "1.0000000",
        "0x1.00000#20",
        Less,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Down,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Up,
        "1.0000000000000001e-24",
        "0x1.357c299a88ea8E-20#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Floor,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Ceiling,
        "1.0000000000000001e-24",
        "0x1.357c299a88ea8E-20#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Nearest,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        100,
        Nearest,
        "9.9999999999999999999999999999980e-25",
        "0x1.357c299a88ea76a58924d52ceE-20#100",
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Less,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        "8.63e-32",
        "0x1.cE-26#5",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "8.4741e-32",
        "0x1.b80E-26#10",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        "-2.0e-31",
        "-0x4.0E-26#1",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "-3.9e-31",
        "-0x8.0E-26#1",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-3.9e-31",
        "-0x8.0E-26#1",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        "-2.0e-31",
        "-0x4.0E-26#1",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-2.0e-31",
        "-0x4.0E-26#1",
        Less,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        "-2.59e-31",
        "-0x5.4E-26#5",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        "-2.5461e-31",
        "-0x5.2aE-26#10",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-2.5461e-31",
        "-0x5.2aE-26#10",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-2.5422e-31",
        "-0x5.28E-26#10",
        Less,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        "-2.5435290e-31",
        "-0x5.28ad0E-26#20",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        "-2.5435282981106695e-31",
        "-0x5.28ace6f60dd40E-26#53",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        "-2.5435282981106695e-31",
        "-0x5.28ace6f60dd40E-26#53",
        Greater,
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "-2.5435282981106700e-31",
        "-0x5.28ace6f60dd44E-26#53",
        Less,
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        "-2.5435282981106698931876104408174e-31",
        "-0x5.28ace6f60dd4333e6ecab80c8E-26#100",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "9.9e-32",
        "0x2.0E-26#1",
        "-0.50",
        "-0x0.8#1",
        Less,
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "2.0e-31",
        "0x4.0E-26#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "9.9e-32",
        "0x2.0E-26#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "2.0e-31",
        "0x4.0E-26#1",
        "-0.50",
        "-0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "2.0e-31",
        "0x4.0E-26#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "1.73e-31",
        "0x3.8E-26#5",
        "-1.00",
        "-0x1.0#5",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "1.6948e-31",
        "0x3.70E-26#10",
        "-0.99902",
        "-0x0.ffc#10",
        Less,
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "1.6967e-31",
        "0x3.71E-26#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "1.6948e-31",
        "0x3.70E-26#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "1.6967e-31",
        "0x3.71E-26#10",
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "1.6948e-31",
        "0x3.70E-26#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "1.6956854e-31",
        "0x3.70734E-26#20",
        "-1.0000000",
        "-0x1.00000#20",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Less,
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "1.6956855320737801e-31",
        "0x3.707344a409384E-26#53",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "1.6956855320737801e-31",
        "0x3.707344a409384E-26#53",
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "1.6956855320737799287917402938778e-31",
        "0x3.707344a4093822299f31d0084E-26#100",
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Greater,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        "4.9e-32",
        "0x1.0E-26#1",
        Greater,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "9.9e-32",
        "0x2.0E-26#1",
        Less,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "9.9e-32",
        "0x2.0E-26#1",
        Less,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        "8.63e-32",
        "0x1.cE-26#5",
        Less,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        "8.4741e-32",
        "0x1.b80E-26#10",
        Greater,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        "8.4837e-32",
        "0x1.b88E-26#10",
        Less,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Less,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Greater,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Less,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        "0.25",
        "0x0.4#1",
        Greater,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        "0.50",
        "0x0.8#1",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "0.25",
        "0x0.4#1",
        Less,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        "0.50",
        "0x0.8#1",
        Greater,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "0.50",
        "0x0.8#1",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "-0.875",
        "-0x0.e0#5",
        "0.484",
        "0x0.7c#5",
        Less,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "-0.87207",
        "-0x0.df4#10",
        "0.48877",
        "0x0.7d2#10",
        Greater,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "-0.87305",
        "-0x0.df8#10",
        "0.48926",
        "0x0.7d4#10",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "-0.87305",
        "-0x0.df8#10",
        "0.48877",
        "0x0.7d2#10",
        Less,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "-0.87207",
        "-0x0.df4#10",
        "0.48926",
        "0x0.7d4#10",
        Greater,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "-0.87207",
        "-0x0.df4#10",
        "0.48926",
        "0x0.7d4#10",
        Greater,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "-0.87218380",
        "-0x0.df477#20",
        "0.48917866",
        "0x0.7d3ad0#20",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "-0.87218360541826723",
        "-0x0.df476cbd60fac0#53",
        "0.48917865697472140",
        "0x0.7d3acffd9b8db0#53",
        Greater,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        "0.48917865697472140",
        "0x0.7d3acffd9b8db0#53",
        Less,
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "-0.87218360541826723",
        "-0x0.df476cbd60fac0#53",
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Greater,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        "0.48917865697472146",
        "0x0.7d3acffd9b8db4#53",
        Less,
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "-0.87218360541826730978071977821350",
        "-0x0.df476cbd60fac5f54ce60f245#100",
        "0.48917865697472144990578930875139",
        "0x0.7d3acffd9b8db34b71d86c7ff0#100",
        Less,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "3.9e-31",
        "0x8.0E-26#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "7.9e-31",
        "0x1.0E-25#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "3.9e-31",
        "0x8.0E-26#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "7.9e-31",
        "0x1.0E-25#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "7.89e-31",
        "0x1.0E-25#5",
        "1.00",
        "0x1.0#5",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "7.8809e-31",
        "0xf.fcE-26#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "7.8886e-31",
        "0x1.000E-25#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "7.8809e-31",
        "0xf.fcE-26#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "7.8886e-31",
        "0x1.000E-25#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "7.8886091e-31",
        "0x1.00000E-25#20",
        "1.0000000",
        "0x1.00000#20",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "7.8886090522101172e-31",
        "0xf.ffffffffffff8E-26#53",
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "7.8886090522101172e-31",
        "0xf.ffffffffffff8E-26#53",
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_cos_rational_prec_round_fail_1() {
    Float::sin_cos_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn sin_cos_rational_prec_round_fail_2() {
    Float::sin_cos_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_rational_prec_round_ref_fail() {
    Float::sin_cos_rational_prec_round_ref(&Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_rational_prec_fail() {
    Float::sin_cos_rational_prec(Rational::ONE, 0);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_cos_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, c, o_s, o_c) = Float::sin_cos_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o_s);
    assert_rounding_ordering_consistent(&c, rm, o_c);

    let (s_alt, c_alt, o_s_alt, o_c_alt) = Float::sin_cos_rational_prec_round_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    // the two results are those of the separate functions
    let (s_alt, o_s_alt) = Float::sin_rational_prec_round_ref(&x, prec, rm);
    let (c_alt, o_c_alt) = Float::cos_rational_prec_round_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_c, rug_o_s, rug_o_c) = rug_sin_cos_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    }

    assert!(s.le_abs(&1u32));
    assert!(c.le_abs(&1u32));
    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o_s == Equal {
        // only sin(0) = 0 and cos(0) = 1 are exact
        assert_eq!(x, 0u32);
        assert_eq!(o_c, Equal);
        for rm in exhaustive_rounding_modes() {
            let (s2, c2, o_s2, o_c2) = Float::sin_cos_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(o_s2, Equal);
            assert_eq!(o_c2, Equal);
        }
    } else {
        assert_ne!(o_c, Equal);
        assert_panic!(Float::sin_cos_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn sin_cos_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        sin_cos_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, c, o_s, o_c) = Float::sin_cos_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
    });
}

#[test]
fn sin_cos_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (s, c, o_s, o_c) = Float::sin_cos_rational_prec(x.clone(), prec);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, Nearest, o_s);
        assert_rounding_ordering_consistent(&c, Nearest, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = Float::sin_cos_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_rational_prec_round_ref(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (rug_s, rug_c, rug_o_s, rug_o_c) = rug_sin_cos_rational_prec(&x, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    });
}

// An input too large to be a `Float`, reduced modulo 2 pi once for both results, with pi to about
// 2^30 bits; slow even in release mode. The values are those of the separate functions.
#[test]
fn test_sin_cos_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (s, c, o_s, o_c) = Float::sin_cos_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(s.to_string(), "0.62793");
    assert_eq!(to_hex_string(&s), "0x0.a0c#10");
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-0.77832");
    assert_eq!(to_hex_string(&c), "-0x0.c74#10");
    assert_eq!(o_c, Greater);
}

// Inputs within 2^(-2^30) of pi and of pi/2, where the sine or the cosine underflows while the
// other result is just short of ±1. Each call computes pi to about 2^30 bits, so this test is slow
// even in release mode.
#[test]
fn test_sin_cos_rational_underflow() {
    let p = (1u64 << 30) + 64;
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    let pi = Rational::exact_from(&Float::pi_prec_round(p, Floor).0);
    // just below pi: the sine is positive and tiny, and the cosine is just above -1
    let (s, c, o_s, o_c) = Float::sin_cos_rational_prec_round_ref(&pi, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o_c, Greater);
    // just below pi/2: the cosine is positive and tiny, and the sine is just below 1
    let half_pi = pi >> 1u32;
    let (s, c, o_s, o_c) = Float::sin_cos_rational_prec_round_ref(&half_pi, 10, Floor);
    assert_eq!(s.to_string(), "0.99902");
    assert_eq!(o_s, Less);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o_c, Less);
}

// The inputs are drawn from the `sin_with_period` and `cos_with_period` tests, so they cover both
// sets of exact and closed-form cases (some of which only one function has, sending the pair
// through the loop), both near-zero paths, and the tiny x/u whose cosine rounds from 1 alone.
#[test]
fn test_sin_cos_with_period_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (sin, cos, o_s, o_c) = x.clone().sin_cos_with_period_prec_round(u, prec, rm);
        assert!(sin.is_valid());
        assert!(cos.is_valid());
        assert_eq!(sin.to_string(), out_s);
        assert_eq!(to_hex_string(&sin), out_s_hex);
        assert_eq!(cos.to_string(), out_c);
        assert_eq!(to_hex_string(&cos), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
            x.sin_cos_with_period_prec_round_ref(u, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let mut sin_alt = x.clone();
        let mut cos_alt = Float::NAN;
        let (o_s_alt, o_c_alt) =
            sin_alt.sin_cos_with_period_prec_round_assign(&mut cos_alt, u, prec, rm);
        assert!(sin_alt.is_valid());
        assert!(cos_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        // the two results are those of the separate functions
        let (sin_alt, o_s_alt) = x.sin_with_period_prec_round_ref(u, prec, rm);
        let (cos_alt, o_c_alt) = x.cos_with_period_prec_round_ref(u, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u32::try_from(u).is_ok()
        {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&sin)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&cos)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    };
    test(
        "NaN", "NaN", 4, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 4, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        4,
        10,
        Nearest,
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        4,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        4,
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.0", "0x1.0#1", 0, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "0.0", "0x0.0", 0, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "NaN", "NaN", 0, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "0.0",
        "0x0.0",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        360,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        360,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        1,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        "0.86523",
        "0x0.dd8#10",
        Equal,
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Floor,
        "6.2808e-6",
        "0x0.0000696#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        1,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        3,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Ceiling,
        "0.43408",
        "0x0.6f2#10",
        "0.90137",
        "0x0.e6c#10",
        Greater,
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        360,
        53,
        Nearest,
        "0.0087265354983739347",
        "0x0.023be6f892a9842#53",
        "0.99996192306417131",
        "0x0.fffd812cce4e68#53",
        Less,
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1099511627776,
        53,
        Nearest,
        "2.8572618735686711e-12",
        "0x3.243f6a8885a30E-10#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Floor,
        "0.38232",
        "0x0.61e#10",
        "0.92383",
        "0x0.ec8#10",
        Less,
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        7,
        10,
        Nearest,
        "0.22241",
        "0x0.38f#10",
        "0.97461",
        "0x0.f98#10",
        Less,
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Floor,
        "0.0043564",
        "0x0.011d8#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        10,
        Floor,
        "1.4282e-12",
        "0x1.920E-10#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        2,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        3,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        6,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        1000000,
        10,
        Ceiling,
        "9.4324e-6",
        "0x0.00009e4#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        12,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        1000000,
        10,
        Nearest,
        "0.000012562",
        "0x0.0000d2c#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        1,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        2,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        4,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Floor,
        "0.43359",
        "0x0.6f0#10",
        "-0.90137",
        "-0x0.e6c#10",
        Less,
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Ceiling,
        "0.052368",
        "0x0.0d68#10",
        "0.99902",
        "0x0.ffc#10",
        Greater,
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Ceiling,
        "1.7167e-11",
        "0x1.2e0E-9#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        2,
        10,
        Floor,
        "-0.0",
        "-0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        6,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Greater,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Nearest,
        "-0.017456",
        "-0x0.0478#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1099511627776,
        10,
        Nearest,
        "-5.7128e-12",
        "-0x6.48E-10#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        3,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Floor,
        "0.00062752",
        "0x0.00292#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Floor,
        "0.27295",
        "0x0.45e#10",
        "-0.96289",
        "-0x0.f68#10",
        Less,
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        3,
        10,
        Nearest,
        "0.81641",
        "0x0.d10#10",
        "0.57715",
        "0x0.93c#10",
        Greater,
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        53,
        Nearest,
        "-0.75425138073610065",
        "-0x0.c1169e55397210#53",
        "0.65658575575296008",
        "0x0.a816010bfa78e8#53",
        Less,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        7,
        10,
        Ceiling,
        "-0.75586",
        "-0x0.c18#10",
        "-0.65332",
        "-0x0.a74#10",
        Greater,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        360,
        53,
        Nearest,
        "0.83430943331480656",
        "0x0.d5954d92d4d5c8#53",
        "-0.55129644428558244",
        "-0x0.8d21c3869b8fc8#53",
        Less,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        53,
        Nearest,
        "7.0549224372658779e-10",
        "0x3.07b269b202f0eE-8#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        4,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        7,
        10,
        Nearest,
        "-0.43408",
        "-0x0.6f2#10",
        "-0.90137",
        "-0x0.e6c#10",
        Less,
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        10,
        Floor,
        "-0.98535",
        "-0x0.fc4#10",
        "0.17358",
        "0x0.2c7#10",
        Less,
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Floor,
        "0.057068",
        "0x0.0e9c#10",
        "0.99805",
        "0x0.ff8#10",
        Less,
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Greater,
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        3,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Ceiling,
        "-0.70703",
        "-0x0.b50#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        12,
        10,
        Ceiling,
        "-0.38232",
        "-0x0.61e#10",
        "0.92480",
        "0x0.ecc#10",
        Greater,
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1000000,
        10,
        Ceiling,
        "-4.7088e-6",
        "-0x0.00004f0#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        10,
        Ceiling,
        "6.2846e-10",
        "0x2.b3E-8#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        10,
        Floor,
        "2.0941e-10",
        "0xe.64E-9#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        6,
        10,
        Nearest,
        "1.0471e-10",
        "0x7.32E-9#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        12,
        10,
        Nearest,
        "5.2353e-11",
        "0x3.99E-9#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1000000,
        10,
        Nearest,
        "6.2797e-16",
        "0x2.d4E-13#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        2,
        53,
        Nearest,
        "0.38268343236508978",
        "0x0.61f78a9abaa58c#53",
        "0.92387953251128674",
        "0x0.ec835e79946a30#53",
        Greater,
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "0.19531",
        "0x0.320#10",
        "0.98145",
        "0x0.fb4#10",
        Greater,
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Floor,
        "0.11194",
        "0x0.1ca8#10",
        "0.99316",
        "0x0.fe4#10",
        Less,
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        10,
        Ceiling,
        "0.0021820",
        "0x0.008f0#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1099511627776,
        10,
        Ceiling,
        "7.1498e-13",
        "0xc.94E-11#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "90.000000000000000000000000000808",
        "0x5a.000000000000000000000040#100",
        360,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "-1.4098657419642452e-29",
        "-0x1.1df46a2529d39E-24#53",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Greater,
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Greater,
        Greater,
    );
    test(
        "7.0", "0x7.0#3", 8, 1, Nearest, "-0.50", "-0x0.8#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        53,
        Nearest,
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Less,
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        8,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Down,
        "0.50000",
        "0x0.800#10",
        "0.86523",
        "0x0.dd8#10",
        Equal,
        Less,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        "-0.86523",
        "-0x0.dd8#10",
        Equal,
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Floor,
        "-0.50000",
        "-0x0.800#10",
        "-0.86621",
        "-0x0.ddc#10",
        Equal,
        Less,
    );
    test(
        "-11.0",
        "-0xb.0#4",
        12,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        "0.86621",
        "0x0.ddc#10",
        Equal,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "2.0", "0x2.0#1", 5, 1, Nearest, "0.50", "0x0.8#1", "-1.0", "-0x1.0#1", Less, Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        "-0.80901699437494745",
        "-0x0.cf1bbcdcbfa540#53",
        Greater,
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Up,
        "-0.58789",
        "-0x0.968#10",
        "-0.80957",
        "-0x0.cf4#10",
        Less,
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Down,
        "-0.95020",
        "-0x0.f34#10",
        "0.30859",
        "0x0.4f0#10",
        Greater,
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        "0.80859",
        "0x0.cf0#10",
        Less,
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        "-0.30908",
        "-0x0.4f2#10",
        Less,
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        "-0.30908",
        "-0x0.4f2#10",
        Less,
        Less,
    );
    test(
        "9.00", "0x9.0#4", 10, 1, Nearest, "-0.50", "-0x0.8#1", "1.0", "0x1.0#1", Greater, Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        53,
        Nearest,
        "-0.58778525229247314",
        "-0x0.96791823aad2f0#53",
        "0.80901699437494745",
        "0x0.cf1bbcdcbfa540#53",
        Less,
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Less,
        Greater,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        "0.86621",
        "0x0.ddc#10",
        Equal,
        Greater,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        "-0.86621",
        "-0x0.ddc#10",
        Equal,
        Less,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Less,
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        "-0.80859",
        "-0x0.cf0#10",
        Greater,
        Greater,
    );
    test(
        "36.0", "0x24.0#4", 360, 1, Nearest, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        "0.80901699437494745",
        "0x0.cf1bbcdcbfa540#53",
        Greater,
        Greater,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        "-0.30908",
        "-0x0.4f2#10",
        Greater,
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        20,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        "0.30859",
        "0x0.4f0#10",
        Less,
        Less,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        60,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        "0.80859",
        "0x0.cf0#10",
        Less,
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        20,
        30,
        Ceiling,
        "0.30901699467",
        "0x0.4f1bbcde#30",
        "0.95105651673",
        "0x0.f378709c#30",
        Greater,
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        20,
        30,
        Nearest,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        "-0.58778525237",
        "-0x0.96791824#30",
        Less,
        Less,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Floor,
        "-0.80901699513",
        "-0x0.cf1bbce0#30",
        "-0.58778525237",
        "-0x0.96791824#30",
        Less,
        Less,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        "0.95105651673",
        "0x0.f378709c#30",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Nearest,
        "0.86602540407",
        "0x0.ddb3d744#30",
        "0.50000000000",
        "0x0.80000000#30",
        Greater,
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        30,
        Nearest,
        "-0.70710678119",
        "-0x0.b504f334#30",
        "-0.70710678119",
        "-0x0.b504f334#30",
        Less,
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        4,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        2,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        10,
        Down,
        "0.0",
        "0x0.0",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        4,
        1,
        Up,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        "720.0",
        "0x2.dE+2#6",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        2,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        4,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        1099511627776,
        10,
        Nearest,
        "5.7128e-12",
        "0x6.48E-10#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        2,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Nearest,
        "0.25879",
        "0x0.424#10",
        "0.96582",
        "0x0.f74#10",
        Less,
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Ceiling,
        "3.1441e-6",
        "0x0.000034c#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        7,
        10,
        Ceiling,
        "0.22266",
        "0x0.390#10",
        "0.97559",
        "0x0.f9c#10",
        Greater,
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        1000000,
        10,
        Floor,
        "1.5702e-6",
        "0x0.00001a58#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        4,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Less,
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        7,
        10,
        Floor,
        "0.97461",
        "0x0.f98#10",
        "0.22241",
        "0x0.38f#10",
        Less,
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1000000,
        10,
        Nearest,
        "9.4175e-6",
        "0x0.00009e0#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        "-0.50000000000000000",
        "-0x0.80000000000000#53",
        Greater,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Nearest,
        "0.97461",
        "0x0.f98#10",
        "-0.22241",
        "-0x0.38f#10",
        Less,
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        53,
        Nearest,
        "0.034899496702500969",
        "0x0.08ef2c64fbee138#53",
        "0.99939082701909576",
        "0x0.ffd813c5f82b38#53",
        Less,
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        1,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        3,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        6,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        6,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        "0.50000",
        "0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Floor,
        "-0.017456",
        "-0x0.0478#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1099511627776,
        53,
        Nearest,
        "-5.7145237471373423e-12",
        "-0x6.487ed5110b460E-10#53",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        3,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        360,
        10,
        Nearest,
        "0.98438",
        "0x0.fc0#10",
        "-0.17358",
        "-0x0.2c7#10",
        Less,
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1099511627776,
        10,
        Ceiling,
        "5.7207e-10",
        "0x2.75E-8#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        53,
        Nearest,
        "-0.99046142569665252",
        "-0x0.fd8ee147511068#53",
        "-0.13779029068462850",
        "-0x0.23463978326254#53",
        Greater,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Nearest,
        "-0.45947",
        "-0x0.75a#10",
        "-0.88770",
        "-0x0.e34#10",
        Greater,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Ceiling,
        "0.97168",
        "0x0.f8c#10",
        "-0.23633",
        "-0x0.3c8#10",
        Greater,
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        10,
        Floor,
        "7.0486e-10",
        "0x3.07E-8#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        4,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Nearest,
        "0.057129",
        "0x0.0ea0#10",
        "0.99805",
        "0x0.ff8#10",
        Greater,
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Floor,
        "-0.70801",
        "-0x0.b54#10",
        "-0.70801",
        "-0x0.b54#10",
        Less,
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        4,
        10,
        Ceiling,
        "-0.92383",
        "-0x0.ec8#10",
        "0.38281",
        "0x0.620#10",
        Greater,
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        12,
        10,
        Nearest,
        "-0.38281",
        "-0x0.620#10",
        "0.92383",
        "0x0.ec8#10",
        Less,
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        2,
        10,
        Nearest,
        "3.1423e-10",
        "0x1.598E-8#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        10,
        Floor,
        "1.5689e-10",
        "0xa.c8E-9#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        7,
        10,
        Ceiling,
        "8.9813e-11",
        "0x6.2cE-9#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1000000,
        10,
        Floor,
        "6.2797e-16",
        "0x2.d4E-13#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Nearest,
        "0.19507",
        "0x0.31f#10",
        "0.98047",
        "0x0.fb0#10",
        Less,
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Nearest,
        "7.8510e-7",
        "0xd.2cE-6#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        "3.8e30",
        "0x3.0E+25#2",
        7,
        53,
        Nearest,
        "-0.78183148246802980",
        "-0x0.c8261ba82ef258#53",
        "0.62348980185873348",
        "0x0.9f9d07145f6e88#53",
        Greater,
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Greater,
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        8,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "-0.70801",
        "-0x0.b54#10",
        Less,
        Less,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        10,
        Ceiling,
        "-0.70703",
        "-0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Greater,
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Down,
        "-0.70703",
        "-0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Greater,
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Equal,
        Less,
    );
    test(
        "7.0", "0x7.0#3", 12, 1, Nearest, "-0.50", "-0x0.8#1", "-1.0", "-0x1.0#1", Equal, Less,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        "0.86621",
        "0x0.ddc#10",
        Equal,
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        5,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Less,
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        "-0.80957",
        "-0x0.cf4#10",
        Less,
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Ceiling,
        "-0.58691",
        "-0x0.964#10",
        "-0.80859",
        "-0x0.cf0#10",
        Greater,
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        "0.80901699437494745",
        "0x0.cf1bbcdcbfa540#53",
        Greater,
        Greater,
    );
    test(
        "7.0", "0x7.0#3", 10, 1, Nearest, "-1.0", "-0x1.0#1", "-0.25", "-0x0.4#1", Less, Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        "0.80859",
        "0x0.cf0#10",
        Less,
        Less,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        360,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Greater,
        Less,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "-0.70801",
        "-0x0.b54#10",
        Less,
        Less,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Down,
        "0.50000",
        "0x0.800#10",
        "-0.86523",
        "-0x0.dd8#10",
        Equal,
        Greater,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        "-0.80901699437494745",
        "-0x0.cf1bbcdcbfa540#53",
        Greater,
        Less,
    );
    test(
        "108.0", "0x6c.0#5", 360, 1, Nearest, "1.0", "0x1.0#1", "-0.25", "-0x0.4#1", Greater,
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        24,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Greater,
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        "0.80957",
        "0x0.cf4#10",
        Greater,
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_cos_with_period_prec_round_fail_1() {
    Float::ONE.sin_cos_with_period_prec_round(7, 0, Nearest);
}

#[test]
#[should_panic]
fn sin_cos_with_period_prec_round_fail_2() {
    Float::ONE.sin_cos_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_with_period_prec_round_fail_3() {
    // the sine of a twelfth of a turn is exact, but the cosine is not
    Float::from(30u32).sin_cos_with_period_prec_round(360, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_with_period_prec_round_ref_fail() {
    Float::ONE.sin_cos_with_period_prec_round_ref(7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_with_period_prec_fail() {
    Float::ONE.sin_cos_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn sin_cos_with_period_round_fail() {
    Float::ONE.sin_cos_with_period_round(7, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_cos_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when both results are exactly representable; otherwise panic.
        let (s, c, o_s, o_c) = x.sin_cos_with_period_prec_round_ref(u, prec, Nearest);
        if o_s == Equal && o_c == Equal {
            let (se, ce, o_se, o_ce) = x.sin_cos_with_period_prec_round_ref(u, prec, Exact);
            assert_eq!(ComparableFloatRef(&se), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&ce), ComparableFloatRef(&c));
            assert_eq!(o_se, Equal);
            assert_eq!(o_ce, Equal);
        } else {
            assert_panic!(x.sin_cos_with_period_prec_round_ref(u, prec, Exact));
        }
        return;
    }
    let (s, c, o_s, o_c) = x.clone().sin_cos_with_period_prec_round(u, prec, rm);
    assert!(s.is_valid());
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o_s);
    assert_rounding_ordering_consistent(&c, rm, o_c);

    let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_prec_round_ref(u, prec, rm);
    assert!(s_alt.is_valid());
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    let mut s_alt = x.clone();
    let mut c_alt = Float::NAN;
    let (o_s_alt, o_c_alt) = s_alt.sin_cos_with_period_prec_round_assign(&mut c_alt, u, prec, rm);
    assert!(s_alt.is_valid());
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    // the two results are those of the separate functions
    let (s_alt, o_s_alt) = x.sin_with_period_prec_round_ref(u, prec, rm);
    let (c_alt, o_c_alt) = x.cos_with_period_prec_round_ref(u, prec, rm);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u32::try_from(u).is_ok()
    {
        let (rug_s, rug_c, rug_o_s, rug_o_c) =
            rug_sin_cos_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    }

    // both results are NaN exactly for u = 0 and non-finite x, and otherwise lie in [-1, 1]
    assert_eq!(s.is_nan(), u == 0 || !x.is_finite());
    assert_eq!(c.is_nan(), s.is_nan());
    if !s.is_nan() {
        assert!(s.le_abs(&1u32));
        assert!(c.le_abs(&1u32));
        if s.is_normal() {
            assert_eq!(s.get_prec(), Some(prec));
        }
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
        // the sine is odd and the cosine is even
        let (s_neg, _, o_s_neg, _) = (-&x).sin_cos_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_s_neg, o_s.reverse());
        let (_, c_neg, _, o_c_neg) = (-&x).sin_cos_with_period_prec_round(u, prec, rm);
        assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&c));
        assert_eq!(o_c_neg, o_c);
        // both have period u
        if x.is_finite() && u != 0 {
            let (shifted, os) =
                x.add_prec_round_ref_val(Float::from(u), x.significant_bits() + 64, Nearest);
            if os == Equal {
                let (s_shifted, c_shifted, o_s_shifted, o_c_shifted) =
                    shifted.sin_cos_with_period_prec_round(u, prec, rm);
                assert_eq!(
                    ComparableFloat(s_shifted.abs_negative_zero()),
                    ComparableFloat(s.abs_negative_zero_ref())
                );
                assert_eq!(ComparableFloatRef(&c_shifted), ComparableFloatRef(&c));
                assert_eq!(o_s_shifted, o_s);
                assert_eq!(o_c_shifted, o_c);
            }
        }
    }

    if o_s == Equal && o_c == Equal {
        for rm2 in exhaustive_rounding_modes() {
            let (s2, c2, o_s2, o_c2) = x.sin_cos_with_period_prec_round_ref(u, prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(o_s2, Equal);
            assert_eq!(o_c2, Equal);
        }
    } else {
        assert_panic!(x.sin_cos_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn sin_cos_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            sin_cos_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            sin_cos_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // sin_cos_with_period(±0) = (±0, 1) and sin_cos_with_period(x, 0) = (NaN, NaN), exactly
        let (s, c, o_s, o_c) = Float::ZERO.sin_cos_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
        let (s, c, o_s, o_c) = Float::NEGATIVE_ZERO.sin_cos_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
        let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(0, prec, rm);
        assert!(s.is_nan());
        assert!(c.is_nan());
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
        // exact cases: quarter turns, with the sine's zeros taking the sign of x
        let one = Float::one_prec(prec);
        for (k, expected_s, expected_c) in [
            (0u32, Float::ZERO, one.clone()),
            (1, one.clone(), Float::ZERO),
            (2, Float::ZERO, -&one),
            (3, -&one, Float::ZERO),
            (4, Float::ZERO, one.clone()),
        ] {
            let (s, c, o_s, o_c) = Float::from(k).sin_cos_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(expected_s));
            assert_eq!(ComparableFloat(c), ComparableFloat(expected_c));
            assert_eq!(o_s, Equal);
            assert_eq!(o_c, Equal);
        }
        for (k, expected_s, expected_c) in [
            (1i32, -&one, Float::ZERO),
            (2, Float::NEGATIVE_ZERO, -&one),
            (3, one.clone(), Float::ZERO),
            (4, Float::NEGATIVE_ZERO, one.clone()),
        ] {
            let (s, c, o_s, o_c) = Float::from(-k).sin_cos_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(expected_s));
            assert_eq!(ComparableFloat(c), ComparableFloat(expected_c));
            assert_eq!(o_s, Equal);
            assert_eq!(o_c, Equal);
        }
        if rm != Exact {
            // twelfths of a turn: the sine is exactly ±1/2, and the cosine is ±sqrt(3)/2
            let half = &one >> 1u32;
            for (k, expected_s) in
                [(1u32, half.clone()), (5, half.clone()), (7, -&half), (11, -&half)]
            {
                let (s, c, o_s, o_c) = Float::from(k).sin_cos_with_period_prec_round(12, prec, rm);
                assert_eq!(ComparableFloat(s), ComparableFloat(expected_s));
                assert_eq!(o_s, Equal);
                assert_ne!(o_c, Equal);
                assert_eq!(
                    ComparableFloat(c),
                    ComparableFloat(Float::from(k).cos_with_period_prec_round(12, prec, rm).0)
                );
            }
            // sixths of a turn: the cosine is exactly ±1/2, and the sine is ±sqrt(3)/2
            for (k, expected_c) in
                [(1u32, half.clone()), (5, half.clone()), (2, -&half), (4, -&half)]
            {
                let (s, c, o_s, o_c) = Float::from(k).sin_cos_with_period_prec_round(6, prec, rm);
                assert_eq!(ComparableFloat(c), ComparableFloat(expected_c));
                assert_eq!(o_c, Equal);
                assert_ne!(o_s, Equal);
                assert_eq!(
                    ComparableFloat(s),
                    ComparableFloat(Float::from(k).sin_with_period_prec_round(6, prec, rm).0)
                );
            }
        }
    });
}

#[test]
fn sin_cos_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (s, c, o_s, o_c) = x.clone().sin_cos_with_period_prec(u, prec);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, Nearest, o_s);
        assert_rounding_ordering_consistent(&c, Nearest, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_prec_ref(u, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) =
            x.sin_cos_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_with_period_prec_assign(&mut c_alt, u, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        if u32::try_from(u).is_ok() {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_with_period_prec(&rug::Float::exact_from(&x), u, prec);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    });
}

#[test]
fn sin_cos_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        if rm == Exact {
            // The generator allows Exact when the sine is exactly representable, which the cosine
            // need not be (a twelfth of a turn); the pair panics unless both are.
            let (s, c, o_s, o_c) = x.sin_cos_with_period_round_ref(u, Nearest);
            if o_s == Equal && o_c == Equal {
                let (se, ce, o_se, o_ce) = x.sin_cos_with_period_round_ref(u, Exact);
                assert_eq!(ComparableFloatRef(&se), ComparableFloatRef(&s));
                assert_eq!(ComparableFloatRef(&ce), ComparableFloatRef(&c));
                assert_eq!(o_se, Equal);
                assert_eq!(o_ce, Equal);
            } else {
                assert_panic!(x.sin_cos_with_period_round_ref(u, Exact));
            }
            return;
        }
        let (s, c, o_s, o_c) = x.clone().sin_cos_with_period_round(u, rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o_s);
        assert_rounding_ordering_consistent(&c, rm, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) =
            x.sin_cos_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_with_period_round_assign(&mut c_alt, u, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    });
}

// n quarter turns plus or minus 2^-k, exactly: inputs close to a zero of the sine (n even) or of
// the cosine (n odd), where that result is tiny and, for a cancellation of at least 64 bits and
// half the precision, takes its near-zero path while the other is within 2^-2k of ±1 and rounds
// from ±1 alone; the smaller cancellations are resolved by the Ziv loop.
#[test]
fn test_sin_cos_with_period_near_zero() {
    let test = |n: i64,
                k: u64,
                above: bool,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let eps = Rational::power_of_2(-i64::exact_from(k));
        let q = if above {
            Rational::from(n) + eps
        } else {
            Rational::from(n) - eps
        };
        let x = Float::from_rational_prec_round(q, k + 3, Exact).0;
        let (s, c, o_s, o_c) = x.sin_cos_with_period_prec_round_ref(4, prec, rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_eq!(s.to_string(), out_s);
        assert_eq!(to_hex_string(&s), out_s_hex);
        assert_eq!(c.to_string(), out_c);
        assert_eq!(to_hex_string(&c), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (s_alt, o_s_alt) = x.sin_with_period_prec_round_ref(4, prec, rm);
        let (c_alt, o_c_alt) = x.cos_with_period_prec_round_ref(4, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let (rug_s, rug_c, rug_o_s, rug_o_c) = rug_sin_cos_with_period_prec_round(
            &rug::Float::exact_from(&x),
            4,
            prec,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    };
    test(
        1,
        20,
        true,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "-1.9e-6",
        "-0x0.00002#1",
        Greater,
        Less,
    );
    test(
        1,
        20,
        true,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "-1.9e-6",
        "-0x0.00002#1",
        Less,
        Less,
    );
    test(
        1,
        20,
        true,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "-1.4976e-6",
        "-0x0.00001920#10",
        Greater,
        Greater,
    );
    test(
        1,
        20,
        true,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "-1.4994e-6",
        "-0x0.00001928#10",
        Less,
        Less,
    );
    test(
        1,
        20,
        true,
        64,
        Nearest,
        "0.999999999998877955865",
        "0x0.fffffffffec42c33#64",
        "-1.49802811316901122884e-6",
        "-0x0.00001921fb544422c2682#64",
        Less,
        Greater,
    );
    test(
        1,
        20,
        true,
        64,
        Floor,
        "0.999999999998877955865",
        "0x0.fffffffffec42c33#64",
        "-1.49802811316901122894e-6",
        "-0x0.00001921fb544422c2684#64",
        Less,
        Less,
    );
    test(
        1,
        20,
        false,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "1.9e-6",
        "0x0.00002#1",
        Greater,
        Greater,
    );
    test(
        1,
        20,
        false,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "9.5e-7",
        "0x0.00001#1",
        Less,
        Less,
    );
    test(
        1,
        20,
        false,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "1.4976e-6",
        "0x0.00001920#10",
        Greater,
        Less,
    );
    test(
        1,
        20,
        false,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
        Less,
    );
    test(
        1,
        20,
        false,
        64,
        Nearest,
        "0.999999999998877955865",
        "0x0.fffffffffec42c33#64",
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        Less,
        Less,
    );
    test(
        1,
        20,
        false,
        64,
        Floor,
        "0.999999999998877955865",
        "0x0.fffffffffec42c33#64",
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        Less,
        Less,
    );
    test(
        1,
        70,
        true,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "-1.7e-21",
        "-0x8.0E-18#1",
        Greater,
        Less,
    );
    test(
        1,
        70,
        true,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "-1.7e-21",
        "-0x8.0E-18#1",
        Less,
        Less,
    );
    test(
        1,
        70,
        true,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "-1.3301e-21",
        "-0x6.48E-18#10",
        Greater,
        Greater,
    );
    test(
        1,
        70,
        true,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        Less,
        Less,
    );
    test(
        1,
        70,
        true,
        64,
        Nearest,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Greater,
        Less,
    );
    test(
        1,
        70,
        true,
        64,
        Floor,
        "0.999999999999999999946",
        "0x0.ffffffffffffffff#64",
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
        Less,
    );
    test(
        1,
        70,
        false,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "1.7e-21",
        "0x8.0E-18#1",
        Greater,
        Greater,
    );
    test(
        1,
        70,
        false,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "8.5e-22",
        "0x4.0E-18#1",
        Less,
        Less,
    );
    test(
        1,
        70,
        false,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "1.3301e-21",
        "0x6.48E-18#10",
        Greater,
        Less,
    );
    test(
        1,
        70,
        false,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
        Less,
    );
    test(
        1,
        70,
        false,
        64,
        Nearest,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        Greater,
        Greater,
    );
    test(
        1,
        70,
        false,
        64,
        Floor,
        "0.999999999999999999946",
        "0x0.ffffffffffffffff#64",
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        Less,
        Less,
    );
    test(
        1,
        1000,
        true,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "-1.9e-301",
        "-0x2.0E-250#1",
        Greater,
        Less,
    );
    test(
        1,
        1000,
        true,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
        Less,
    );
    test(
        1,
        1000,
        true,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "-1.4655e-301",
        "-0x1.920E-250#10",
        Greater,
        Greater,
    );
    test(
        1,
        1000,
        true,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "-1.4673e-301",
        "-0x1.928E-250#10",
        Less,
        Less,
    );
    test(
        1,
        1000,
        true,
        64,
        Nearest,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Greater,
        Less,
    );
    test(
        1,
        1000,
        true,
        64,
        Floor,
        "0.999999999999999999946",
        "0x0.ffffffffffffffff#64",
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
        Less,
    );
    test(
        1,
        1000,
        false,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "1.9e-301",
        "0x2.0E-250#1",
        Greater,
        Greater,
    );
    test(
        1,
        1000,
        false,
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        "9.3e-302",
        "0x1.0E-250#1",
        Less,
        Less,
    );
    test(
        1,
        1000,
        false,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "1.4655e-301",
        "0x1.920E-250#10",
        Greater,
        Less,
    );
    test(
        1,
        1000,
        false,
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
        Less,
    );
    test(
        1,
        1000,
        false,
        64,
        Nearest,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        Greater,
        Greater,
    );
    test(
        1,
        1000,
        false,
        64,
        Floor,
        "0.999999999999999999946",
        "0x0.ffffffffffffffff#64",
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        Less,
        Less,
    );
    test(
        2,
        20,
        true,
        1,
        Nearest,
        "-1.9e-6",
        "-0x0.00002#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        20,
        true,
        1,
        Floor,
        "-1.9e-6",
        "-0x0.00002#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        20,
        true,
        10,
        Nearest,
        "-1.4976e-6",
        "-0x0.00001920#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        2,
        20,
        true,
        10,
        Floor,
        "-1.4994e-6",
        "-0x0.00001928#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        20,
        true,
        64,
        Nearest,
        "-1.49802811316901122884e-6",
        "-0x0.00001921fb544422c2682#64",
        "-0.999999999998877955865",
        "-0x0.fffffffffec42c33#64",
        Greater,
        Greater,
    );
    test(
        2,
        20,
        true,
        64,
        Floor,
        "-1.49802811316901122894e-6",
        "-0x0.00001921fb544422c2684#64",
        "-0.999999999998877955919",
        "-0x0.fffffffffec42c34#64",
        Less,
        Less,
    );
    test(
        2,
        20,
        false,
        1,
        Nearest,
        "1.9e-6",
        "0x0.00002#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        2,
        20,
        false,
        1,
        Floor,
        "9.5e-7",
        "0x0.00001#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        20,
        false,
        10,
        Nearest,
        "1.4976e-6",
        "0x0.00001920#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        20,
        false,
        10,
        Floor,
        "1.4976e-6",
        "0x0.00001920#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        20,
        false,
        64,
        Nearest,
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        "-0.999999999998877955865",
        "-0x0.fffffffffec42c33#64",
        Less,
        Greater,
    );
    test(
        2,
        20,
        false,
        64,
        Floor,
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        "-0.999999999998877955919",
        "-0x0.fffffffffec42c34#64",
        Less,
        Less,
    );
    test(
        2,
        70,
        true,
        1,
        Nearest,
        "-1.7e-21",
        "-0x8.0E-18#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        70,
        true,
        1,
        Floor,
        "-1.7e-21",
        "-0x8.0E-18#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        70,
        true,
        10,
        Nearest,
        "-1.3301e-21",
        "-0x6.48E-18#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        2,
        70,
        true,
        10,
        Floor,
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        70,
        true,
        64,
        Nearest,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
        Less,
    );
    test(
        2,
        70,
        true,
        64,
        Floor,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
        Less,
    );
    test(
        2,
        70,
        false,
        1,
        Nearest,
        "1.7e-21",
        "0x8.0E-18#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        2,
        70,
        false,
        1,
        Floor,
        "8.5e-22",
        "0x4.0E-18#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        70,
        false,
        10,
        Nearest,
        "1.3301e-21",
        "0x6.48E-18#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        70,
        false,
        10,
        Floor,
        "1.3301e-21",
        "0x6.48E-18#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        70,
        false,
        64,
        Nearest,
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Greater,
        Less,
    );
    test(
        2,
        70,
        false,
        64,
        Floor,
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
        Less,
    );
    test(
        2,
        1000,
        true,
        1,
        Nearest,
        "-1.9e-301",
        "-0x2.0E-250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        1000,
        true,
        1,
        Floor,
        "-1.9e-301",
        "-0x2.0E-250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        1000,
        true,
        10,
        Nearest,
        "-1.4655e-301",
        "-0x1.920E-250#10",
        "-1.0000",
        "-0x1.000#10",
        Greater,
        Less,
    );
    test(
        2,
        1000,
        true,
        10,
        Floor,
        "-1.4673e-301",
        "-0x1.928E-250#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        1000,
        true,
        64,
        Nearest,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
        Less,
    );
    test(
        2,
        1000,
        true,
        64,
        Floor,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
        Less,
    );
    test(
        2,
        1000,
        false,
        1,
        Nearest,
        "1.9e-301",
        "0x2.0E-250#1",
        "-1.0",
        "-0x1.0#1",
        Greater,
        Less,
    );
    test(
        2,
        1000,
        false,
        1,
        Floor,
        "9.3e-302",
        "0x1.0E-250#1",
        "-1.0",
        "-0x1.0#1",
        Less,
        Less,
    );
    test(
        2,
        1000,
        false,
        10,
        Nearest,
        "1.4655e-301",
        "0x1.920E-250#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        1000,
        false,
        10,
        Floor,
        "1.4655e-301",
        "0x1.920E-250#10",
        "-1.0000",
        "-0x1.000#10",
        Less,
        Less,
    );
    test(
        2,
        1000,
        false,
        64,
        Nearest,
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Greater,
        Less,
    );
    test(
        2,
        1000,
        false,
        64,
        Floor,
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Less,
        Less,
    );
    test(
        3,
        20,
        true,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "1.9e-6",
        "0x0.00002#1",
        Less,
        Greater,
    );
    test(
        3,
        20,
        true,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "9.5e-7",
        "0x0.00001#1",
        Less,
        Less,
    );
    test(
        3,
        20,
        true,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
        Less,
    );
    test(
        3,
        20,
        true,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
        Less,
    );
    test(
        3,
        20,
        true,
        64,
        Nearest,
        "-0.999999999998877955865",
        "-0x0.fffffffffec42c33#64",
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        Greater,
        Less,
    );
    test(
        3,
        20,
        true,
        64,
        Floor,
        "-0.999999999998877955919",
        "-0x0.fffffffffec42c34#64",
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        Less,
        Less,
    );
    test(
        3,
        20,
        false,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-6",
        "-0x0.00002#1",
        Less,
        Less,
    );
    test(
        3,
        20,
        false,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-6",
        "-0x0.00002#1",
        Less,
        Less,
    );
    test(
        3,
        20,
        false,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.4976e-6",
        "-0x0.00001920#10",
        Less,
        Greater,
    );
    test(
        3,
        20,
        false,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.4994e-6",
        "-0x0.00001928#10",
        Less,
        Less,
    );
    test(
        3,
        20,
        false,
        64,
        Nearest,
        "-0.999999999998877955865",
        "-0x0.fffffffffec42c33#64",
        "-1.49802811316901122884e-6",
        "-0x0.00001921fb544422c2682#64",
        Greater,
        Greater,
    );
    test(
        3,
        20,
        false,
        64,
        Floor,
        "-0.999999999998877955919",
        "-0x0.fffffffffec42c34#64",
        "-1.49802811316901122894e-6",
        "-0x0.00001921fb544422c2684#64",
        Less,
        Less,
    );
    test(
        3,
        70,
        true,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "1.7e-21",
        "0x8.0E-18#1",
        Less,
        Greater,
    );
    test(
        3,
        70,
        true,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "8.5e-22",
        "0x4.0E-18#1",
        Less,
        Less,
    );
    test(
        3,
        70,
        true,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
        Less,
    );
    test(
        3,
        70,
        true,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
        Less,
    );
    test(
        3,
        70,
        true,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        Less,
        Greater,
    );
    test(
        3,
        70,
        true,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        Less,
        Less,
    );
    test(
        3,
        70,
        false,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.7e-21",
        "-0x8.0E-18#1",
        Less,
        Less,
    );
    test(
        3,
        70,
        false,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-1.7e-21",
        "-0x8.0E-18#1",
        Less,
        Less,
    );
    test(
        3,
        70,
        false,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.3301e-21",
        "-0x6.48E-18#10",
        Less,
        Greater,
    );
    test(
        3,
        70,
        false,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        Less,
        Less,
    );
    test(
        3,
        70,
        false,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
        Less,
    );
    test(
        3,
        70,
        false,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
        Less,
    );
    test(
        3,
        1000,
        true,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "1.9e-301",
        "0x2.0E-250#1",
        Less,
        Greater,
    );
    test(
        3,
        1000,
        true,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        Less,
        Less,
    );
    test(
        3,
        1000,
        true,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
        Less,
    );
    test(
        3,
        1000,
        true,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
        Less,
    );
    test(
        3,
        1000,
        true,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        Less,
        Greater,
    );
    test(
        3,
        1000,
        true,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        Less,
        Less,
    );
    test(
        3,
        1000,
        false,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
        Less,
    );
    test(
        3,
        1000,
        false,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
        Less,
    );
    test(
        3,
        1000,
        false,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.4655e-301",
        "-0x1.920E-250#10",
        Less,
        Greater,
    );
    test(
        3,
        1000,
        false,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.4673e-301",
        "-0x1.928E-250#10",
        Less,
        Less,
    );
    test(
        3,
        1000,
        false,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
        Less,
    );
    test(
        3,
        1000,
        false,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
        Less,
    );
    test(
        -1,
        20,
        true,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "1.9e-6",
        "0x0.00002#1",
        Less,
        Greater,
    );
    test(
        -1,
        20,
        true,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "9.5e-7",
        "0x0.00001#1",
        Less,
        Less,
    );
    test(
        -1,
        20,
        true,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
        Less,
    );
    test(
        -1,
        20,
        true,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
        Less,
    );
    test(
        -1,
        20,
        true,
        64,
        Nearest,
        "-0.999999999998877955865",
        "-0x0.fffffffffec42c33#64",
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        Greater,
        Less,
    );
    test(
        -1,
        20,
        true,
        64,
        Floor,
        "-0.999999999998877955919",
        "-0x0.fffffffffec42c34#64",
        "1.49802811316901122884e-6",
        "0x0.00001921fb544422c2682#64",
        Less,
        Less,
    );
    test(
        -1,
        20,
        false,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-6",
        "-0x0.00002#1",
        Less,
        Less,
    );
    test(
        -1,
        20,
        false,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-6",
        "-0x0.00002#1",
        Less,
        Less,
    );
    test(
        -1,
        20,
        false,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.4976e-6",
        "-0x0.00001920#10",
        Less,
        Greater,
    );
    test(
        -1,
        20,
        false,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.4994e-6",
        "-0x0.00001928#10",
        Less,
        Less,
    );
    test(
        -1,
        20,
        false,
        64,
        Nearest,
        "-0.999999999998877955865",
        "-0x0.fffffffffec42c33#64",
        "-1.49802811316901122884e-6",
        "-0x0.00001921fb544422c2682#64",
        Greater,
        Greater,
    );
    test(
        -1,
        20,
        false,
        64,
        Floor,
        "-0.999999999998877955919",
        "-0x0.fffffffffec42c34#64",
        "-1.49802811316901122894e-6",
        "-0x0.00001921fb544422c2684#64",
        Less,
        Less,
    );
    test(
        -1,
        70,
        true,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "1.7e-21",
        "0x8.0E-18#1",
        Less,
        Greater,
    );
    test(
        -1,
        70,
        true,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "8.5e-22",
        "0x4.0E-18#1",
        Less,
        Less,
    );
    test(
        -1,
        70,
        true,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
        Less,
    );
    test(
        -1,
        70,
        true,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
        Less,
    );
    test(
        -1,
        70,
        true,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        Less,
        Greater,
    );
    test(
        -1,
        70,
        true,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        Less,
        Less,
    );
    test(
        -1,
        70,
        false,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.7e-21",
        "-0x8.0E-18#1",
        Less,
        Less,
    );
    test(
        -1,
        70,
        false,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-1.7e-21",
        "-0x8.0E-18#1",
        Less,
        Less,
    );
    test(
        -1,
        70,
        false,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.3301e-21",
        "-0x6.48E-18#10",
        Less,
        Greater,
    );
    test(
        -1,
        70,
        false,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        Less,
        Less,
    );
    test(
        -1,
        70,
        false,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
        Less,
    );
    test(
        -1,
        70,
        false,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        true,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "1.9e-301",
        "0x2.0E-250#1",
        Less,
        Greater,
    );
    test(
        -1,
        1000,
        true,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        true,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        true,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        true,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        Less,
        Greater,
    );
    test(
        -1,
        1000,
        true,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        false,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        false,
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        false,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "-1.4655e-301",
        "-0x1.920E-250#10",
        Less,
        Greater,
    );
    test(
        -1,
        1000,
        false,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "-1.4673e-301",
        "-0x1.928E-250#10",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        false,
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
        Less,
    );
    test(
        -1,
        1000,
        false,
        64,
        Floor,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
        Less,
    );
}

// Inputs within 2^(-2^30) of a half turn and of a quarter turn, where the sine or the cosine
// underflows: cheap, since the near-zero path works with the exact distance to the zero rather than
// with pi to 2^30 bits.
#[test]
fn test_sin_cos_with_period_underflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    // 180 +- 2^-(2^30 + 70) and 90 +- 2^-(2^30 + 70), which need 2^30 + 78 bits (the offset alone
    // is below the exponent range, so the sums are built from `Rational`s)
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let p = (1u64 << 30) + 78;
    let above = Float::from_rational_prec_round(Rational::from(180u32) + &eps, p, Exact).0;
    let below = Float::from_rational_prec_round(Rational::from(180u32) - &eps, p, Exact).0;
    // just past a half turn: the sine is negative and tiny, and the cosine is just above -1
    let (s, c, o_s, o_c) = above.sin_cos_with_period_prec_round_ref(360, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-1.0000");
    assert_eq!(o_c, Less);
    let (s, c, o_s, o_c) = above.sin_cos_with_period_prec_round_ref(360, 10, Floor);
    assert_eq!(ComparableFloat(s), ComparableFloat(-&min_positive));
    assert_eq!(o_s, Less);
    assert_eq!(c.to_string(), "-1.0000");
    assert_eq!(o_c, Less);
    // just short of it: positive and tiny
    let (s, c, o_s, o_c) = below.sin_cos_with_period_prec_round_ref(360, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o_c, Greater);
    let (s, c, o_s, o_c) = below.sin_cos_with_period_prec_round_ref(360, 10, Down);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
    assert_eq!(o_s, Less);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o_c, Greater);
    let above = Float::from_rational_prec_round(Rational::from(90u32) + &eps, p, Exact).0;
    let below = Float::from_rational_prec_round(Rational::from(90u32) - eps, p, Exact).0;
    // just past a quarter turn: the cosine is negative and tiny, and the sine is just below 1
    let (s, c, o_s, o_c) = above.sin_cos_with_period_prec_round_ref(360, 10, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o_c, Greater);
    assert_eq!(s.to_string(), "1.0000");
    assert_eq!(o_s, Greater);
    let (s, c, o_s, o_c) = above.sin_cos_with_period_prec_round_ref(360, 10, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(-&min_positive));
    assert_eq!(o_c, Less);
    assert_eq!(s.to_string(), "0.99902");
    assert_eq!(o_s, Less);
    // just short of it: positive and tiny
    let (s, c, o_s, o_c) = below.sin_cos_with_period_prec_round_ref(360, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min_positive));
    assert_eq!(o_c, Greater);
    assert_eq!(s.to_string(), "1.0000");
    assert_eq!(o_s, Greater);
    let (s, c, o_s, o_c) = below.sin_cos_with_period_prec_round_ref(360, 10, Down);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o_c, Less);
    assert_eq!(s.to_string(), "0.99902");
    assert_eq!(o_s, Less);
}

// The inputs are drawn from the `sin_with_period_rational` and `cos_with_period_rational` tests,
// including the exact and closed-form cases hit directly (1/12, 1/3, 1/8, 1/20 of a turn) and
// non-dyadic inputs whose values MPFR cannot compute exactly, since it must round the input first.
#[test]
fn test_sin_cos_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (sin, cos, o_s, o_c) =
            Float::sin_cos_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(sin.is_valid());
        assert!(cos.is_valid());
        assert_eq!(sin.to_string(), out_s);
        assert_eq!(to_hex_string(&sin), out_s_hex);
        assert_eq!(cos.to_string(), out_c);
        assert_eq!(to_hex_string(&cos), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(sin_alt.is_valid());
        assert!(cos_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if rm == Nearest {
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
        }

        // the two results are those of the separate functions
        let (sin_alt, o_s_alt) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
        let (cos_alt, o_c_alt) = Float::cos_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        // MPFR rounds the input first, so it cannot see the exact cases of non-dyadic inputs
        if o_s != Equal
            && o_c != Equal
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u32::try_from(u).is_ok()
        {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&sin)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&cos)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    };
    test(
        "0", 4, 1, Nearest, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0",
        4,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        4,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        4,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        4,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        4,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test("0", 0, 1, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal);
    test(
        "0", 0, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test("0", 0, 10, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal);
    test(
        "0", 0, 10, Ceiling, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test("0", 0, 10, Exact, "NaN", "NaN", "NaN", "NaN", Equal, Equal);
    test(
        "0", 0, 53, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test("1", 0, 1, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal);
    test(
        "1", 0, 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test("1", 0, 10, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal);
    test(
        "1", 0, 10, Ceiling, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test("1", 0, 10, Exact, "NaN", "NaN", "NaN", "NaN", Equal, Equal);
    test(
        "1", 0, 53, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "90", 360, 1, Nearest, "1.0", "0x1.0#1", "0.0", "0x0.0", Equal, Equal,
    );
    test(
        "90",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "90",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "90",
        360,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "90",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "90",
        360,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "180", 360, 1, Nearest, "0.0", "0x0.0", "-1.0", "-0x1.0#1", Equal, Equal,
    );
    test(
        "180",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "180",
        360,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "180",
        360,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "180",
        360,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "180",
        360,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "270", 360, 1, Nearest, "-1.0", "-0x1.0#1", "0.0", "0x0.0", Equal, Equal,
    );
    test(
        "270",
        360,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "270",
        360,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "270",
        360,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "270",
        360,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "270",
        360,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "360", 360, 1, Nearest, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "360",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "360",
        360,
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "360",
        360,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "360",
        360,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "450",
        360,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "60",
        360,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "120",
        360,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "240",
        360,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "300",
        360,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "45",
        360,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "135",
        360,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "-0.70801",
        "-0x0.b54#10",
        Less,
        Less,
    );
    test(
        "30",
        360,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        "0.86523",
        "0x0.dd8#10",
        Equal,
        Less,
    );
    test(
        "150",
        360,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        "-0.86621",
        "-0x0.ddc#10",
        Equal,
        Less,
    );
    test(
        "72",
        360,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        "0.30859",
        "0x0.4f0#10",
        Less,
        Less,
    );
    test(
        "144",
        360,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        "-0.80957",
        "-0x0.cf4#10",
        Less,
        Less,
    );
    test(
        "36",
        360,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        "0.80859",
        "0x0.cf0#10",
        Less,
        Less,
    );
    test(
        "108",
        360,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        "-0.30908",
        "-0x0.4f2#10",
        Less,
        Less,
    );
    test(
        "1/3",
        1,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "-0.50000",
        "-0x0.800#10",
        Less,
        Equal,
    );
    test(
        "1/8",
        1,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "1/5",
        1,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        "0.30859",
        "0x0.4f0#10",
        Less,
        Less,
    );
    test(
        "-1/10",
        1,
        10,
        Floor,
        "-0.58789",
        "-0x0.968#10",
        "0.80859",
        "0x0.cf0#10",
        Less,
        Less,
    );
    test(
        "1/7",
        1,
        10,
        Floor,
        "0.78125",
        "0x0.c80#10",
        "0.62305",
        "0x0.9f8#10",
        Less,
        Less,
    );
    test(
        "2/7",
        1,
        10,
        Floor,
        "0.97461",
        "0x0.f98#10",
        "-0.22266",
        "-0x0.390#10",
        Less,
        Less,
    );
    test(
        "-3/7",
        1,
        10,
        Floor,
        "-0.43408",
        "-0x0.6f2#10",
        "-0.90137",
        "-0x0.e6c#10",
        Less,
        Less,
    );
    test(
        "22/7",
        1,
        10,
        Floor,
        "0.78125",
        "0x0.c80#10",
        "0.62305",
        "0x0.9f8#10",
        Less,
        Less,
    );
    test(
        "1/7",
        3,
        10,
        Floor,
        "0.29443",
        "0x0.4b6#10",
        "0.95508",
        "0x0.f48#10",
        Less,
        Less,
    );
    test(
        "355/113",
        360,
        10,
        Floor,
        "0.054749",
        "0x0.0e04#10",
        "0.99805",
        "0x0.ff8#10",
        Less,
        Less,
    );
    test(
        "1",
        7,
        10,
        Floor,
        "0.78125",
        "0x0.c80#10",
        "0.62305",
        "0x0.9f8#10",
        Less,
        Less,
    );
    test(
        "1000000",
        7,
        10,
        Floor,
        "0.78125",
        "0x0.c80#10",
        "0.62305",
        "0x0.9f8#10",
        Less,
        Less,
    );
    test(
        "1/1000000",
        1,
        10,
        Floor,
        "6.2808e-6",
        "0x0.0000696#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Floor,
        "6.2801e-30",
        "0x7.f6E-25#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "111414603535684224740921180161/1237940039285380274899124224",
        360,
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        "-1.4098657419642455e-29",
        "-0x1.1df46a2529d3aE-24#53",
        Less,
        Less,
    );
    test(
        "3/20",
        1,
        30,
        Nearest,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        "0.58778525237",
        "0x0.96791824#30",
        Less,
        Greater,
    );
    test(
        "11/20",
        1,
        30,
        Floor,
        "-0.30901699467",
        "-0x0.4f1bbcde#30",
        "-0.95105651673",
        "-0x0.f378709c#30",
        Less,
        Less,
    );
    test(
        "17/20",
        1,
        30,
        Nearest,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        "0.58778525237",
        "0x0.96791824#30",
        Greater,
        Greater,
    );
    test(
        "-1/3",
        1,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        "-0.50000000000",
        "-0x0.80000000#30",
        Less,
        Equal,
    );
    test(
        "3/8",
        1,
        30,
        Nearest,
        "0.70710678119",
        "0x0.b504f334#30",
        "-0.70710678119",
        "-0x0.b504f334#30",
        Greater,
        Less,
    );
    test(
        "1/2",
        1,
        10,
        Exact,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-5",
        1,
        10,
        Exact,
        "-0.0",
        "-0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-1/1000000000000000000000000000000",
        7,
        10,
        Floor,
        "-8.9825e-31",
        "-0x1.238E-25#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "450",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "120",
        360,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "300",
        360,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Greater,
        Equal,
    );
    test(
        "135",
        360,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "-0.70703",
        "-0x0.b50#10",
        Less,
        Greater,
    );
    test(
        "30",
        360,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        "0.86621",
        "0x0.ddc#10",
        Equal,
        Greater,
    );
    test(
        "72", 360, 1, Nearest, "1.0", "0x1.0#1", "0.25", "0x0.4#1", Greater, Less,
    );
    test(
        "36",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        "0.80901699437494745",
        "0x0.cf1bbcdcbfa540#53",
        Greater,
        Greater,
    );
    test(
        "1/3",
        1,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "1/5",
        1,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        "0.30901699437494745",
        "0x0.4f1bbcdcbfa540#53",
        Less,
        Greater,
    );
    test(
        "1/7",
        1,
        10,
        Nearest,
        "0.78223",
        "0x0.c84#10",
        "0.62305",
        "0x0.9f8#10",
        Greater,
        Less,
    );
    test(
        "2/7",
        1,
        10,
        Ceiling,
        "0.97559",
        "0x0.f9c#10",
        "-0.22241",
        "-0x0.38f#10",
        Greater,
        Greater,
    );
    test(
        "22/7", 1, 1, Nearest, "1.0", "0x1.0#1", "0.50", "0x0.8#1", Greater, Less,
    );
    test(
        "355/113",
        360,
        53,
        Nearest,
        "0.054803669797705817",
        "0x0.0e079d017b5fba0#53",
        "0.99849714960870273",
        "0x0.ff9d825ab7f600#53",
        Greater,
        Greater,
    );
    test(
        "1000000",
        7,
        10,
        Nearest,
        "0.78223",
        "0x0.c84#10",
        "0.62305",
        "0x0.9f8#10",
        Greater,
        Less,
    );
    test(
        "1/1000000",
        1,
        10,
        Ceiling,
        "6.2883e-6",
        "0x0.0000698#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
}

// Fractions of a turn near the bottom of the exponent range, where the sine underflows or nearly
// does while the cosine rounds from 1 alone.
#[test]
fn test_sin_cos_with_period_rational_tiny() {
    let test = |x: &Rational,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round_ref(x, u, prec, rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_eq!(s.to_string(), out_s);
        assert_eq!(to_hex_string(&s), out_s_hex);
        assert_eq!(c.to_string(), out_c);
        assert_eq!(to_hex_string(&c), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);
        // the two results are those of the separate functions
        let (s_alt, o_s_alt) = Float::sin_with_period_rational_prec_round_ref(x, u, prec, rm);
        let (c_alt, o_c_alt) = Float::cos_with_period_rational_prec_round_ref(x, u, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    };
    let min_exp = -(1i64 << 30);
    // 2 pi x = pi/2 * 2^(-2^30), about 1.57 times the smallest positive Float
    let x = Rational::power_of_2(min_exp - 2);
    test(
        &x,
        1,
        1,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        Greater,
        Greater,
    );
    test(
        &x,
        1,
        1,
        Floor,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "0.50",
        "0x0.8#1",
        Less,
        Less,
    );
    test(
        &x,
        1,
        10,
        Nearest,
        "3.7414e-323228497",
        "0x1.920E-268435456#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    // 2 pi x = pi/4 * 2^(-2^30), about 0.79 times it: below, but above half
    let x = Rational::power_of_2(min_exp - 3);
    test(
        &x,
        1,
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    test(
        &x,
        1,
        10,
        Down,
        "0.0",
        "0x0.0",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    let x = -Rational::power_of_2(min_exp - 3);
    test(
        &x,
        1,
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        &x,
        1,
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    // 2 pi x = pi/8 * 2^(-2^30), about 0.39 times it: below half
    let x = Rational::power_of_2(min_exp - 4);
    test(
        &x,
        1,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        &x,
        1,
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
    // and with a divisor
    let x = Rational::power_of_2(min_exp);
    test(
        &x,
        100,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Less,
        Greater,
    );
    test(
        &x,
        7,
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_cos_with_period_rational_prec_round_fail_1() {
    Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn sin_cos_with_period_rational_prec_round_fail_2() {
    Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_with_period_rational_prec_round_fail_3() {
    // the sine of a twelfth of a turn is exact, but the cosine is not
    Float::sin_cos_with_period_rational_prec_round(Rational::from_unsigneds(1u8, 12), 1, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_with_period_rational_prec_round_ref_fail() {
    Float::sin_cos_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_with_period_rational_prec_fail() {
    Float::sin_cos_with_period_rational_prec(Rational::ONE, 7, 0);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_cos_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact {
        // Exact is only allowed when both results are exactly representable; otherwise panic.
        let (s, c, o_s, o_c) =
            Float::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
        if o_s == Equal && o_c == Equal {
            let (se, ce, o_se, o_ce) =
                Float::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, Exact);
            assert_eq!(ComparableFloatRef(&se), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&ce), ComparableFloatRef(&c));
            assert_eq!(o_se, Equal);
            assert_eq!(o_ce, Equal);
        } else {
            assert_panic!(Float::sin_cos_with_period_rational_prec_round_ref(
                &x, u, prec, Exact
            ));
        }
        return;
    }
    let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(s.is_valid());
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o_s);
    assert_rounding_ordering_consistent(&c, rm, o_c);

    let (s_alt, c_alt, o_s_alt, o_c_alt) =
        Float::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(s_alt.is_valid());
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    // the two results are those of the separate functions
    let (s_alt, o_s_alt) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
    let (c_alt, o_c_alt) = Float::cos_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_s_alt, o_s);
    assert_eq!(o_c_alt, o_c);

    // MPFR rounds the input to a `Float` first, so it cannot see the exact cases of non-dyadic
    // inputs (1/12 of a turn, say); those are checked separately below.
    if o_s != Equal
        && o_c != Equal
        && let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        && u32::try_from(u).is_ok()
    {
        let (rug_s, rug_c, rug_o_s, rug_o_c) =
            rug_sin_cos_with_period_rational_prec_round(&x, u, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o_s, o_s);
        assert_eq!(rug_o_c, o_c);
    }

    // both NaN exactly for u = 0, and otherwise in [-1, 1]
    assert_eq!(s.is_nan(), u == 0);
    assert_eq!(c.is_nan(), u == 0);
    if u != 0 {
        assert!(s.le_abs(&1u32));
        assert!(c.le_abs(&1u32));
        if s.is_normal() {
            assert_eq!(s.get_prec(), Some(prec));
        }
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
        // the sine is odd and the cosine even (a `Rational` has no negative zero, so x = 0 is
        // excluded), and both are periodic with period u, up to the sign of a zero
        if x != 0u32 {
            let (s_neg, _, o_s_neg, _) =
                Float::sin_cos_with_period_rational_prec_round(-&x, u, prec, -rm);
            assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
            assert_eq!(o_s_neg, o_s.reverse());
            let (_, c_neg, _, o_c_neg) =
                Float::sin_cos_with_period_rational_prec_round(-&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&c));
            assert_eq!(o_c_neg, o_c);
        }
        let (s_shifted, c_shifted, o_s_shifted, o_c_shifted) =
            Float::sin_cos_with_period_rational_prec_round(&x + Rational::from(u), u, prec, rm);
        assert_eq!(
            ComparableFloat(s_shifted.abs_negative_zero()),
            ComparableFloat(s.abs_negative_zero_ref())
        );
        assert_eq!(ComparableFloatRef(&c_shifted), ComparableFloatRef(&c));
        assert_eq!(o_s_shifted, o_s);
        assert_eq!(o_c_shifted, o_c);
        // a Float input agrees with the Float version
        if let Ok(f) = Float::try_from(&x) {
            let (s_alt, c_alt, o_s_alt, o_c_alt) = f.sin_cos_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
        }
    }

    if o_s == Equal && o_c == Equal {
        for rm2 in exhaustive_rounding_modes() {
            let (s2, c2, o_s2, o_c2) =
                Float::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(o_s2, Equal);
            assert_eq!(o_c2, Equal);
        }
    } else {
        assert_panic!(Float::sin_cos_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn sin_cos_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, rm)| {
            sin_cos_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, c, o_s, o_c) =
            Float::sin_cos_with_period_rational_prec_round(Rational::ZERO, 4, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
        let (s, c, o_s, o_c) =
            Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert!(s.is_nan());
        assert!(c.is_nan());
        assert_eq!(o_s, Equal);
        assert_eq!(o_c, Equal);
        // exact cases, straight from a fraction of a turn
        let one = Float::one_prec(prec);
        for (s, expected_s, expected_c) in [
            ("1/4", one.clone(), Float::ZERO),
            ("1/2", Float::ZERO, -&one),
            ("-1/2", Float::NEGATIVE_ZERO, -&one),
            ("3/4", -&one, Float::ZERO),
            ("1", Float::ZERO, one.clone()),
            ("-1", Float::NEGATIVE_ZERO, one.clone()),
        ] {
            let (sin, cos, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round(
                Rational::from_str(s).unwrap(),
                1,
                prec,
                rm,
            );
            assert_eq!(ComparableFloat(sin), ComparableFloat(expected_s));
            assert_eq!(ComparableFloat(cos), ComparableFloat(expected_c));
            assert_eq!(o_s, Equal);
            assert_eq!(o_c, Equal);
        }
        if rm != Exact {
            // twelfths of a turn: the sine is exactly ±1/2, and the cosine is ±sqrt(3)/2
            let half = &one >> 1u32;
            for (s, expected_s) in [
                ("1/12", half.clone()),
                ("5/12", half.clone()),
                ("7/12", -&half),
                ("-1/12", -&half),
            ] {
                let x = Rational::from_str(s).unwrap();
                let (sin, cos, o_s, o_c) =
                    Float::sin_cos_with_period_rational_prec_round_ref(&x, 1, prec, rm);
                assert_eq!(ComparableFloat(sin), ComparableFloat(expected_s));
                assert_eq!(o_s, Equal);
                assert_ne!(o_c, Equal);
                assert_eq!(
                    ComparableFloat(cos),
                    ComparableFloat(Float::cos_with_period_rational_prec_round(x, 1, prec, rm).0)
                );
            }
        }
    });
}

#[test]
fn sin_cos_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, _)| {
            let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec(x.clone(), u, prec);
            assert!(s.is_valid());
            assert!(c.is_valid());
            assert_rounding_ordering_consistent(&s, Nearest, o_s);
            assert_rounding_ordering_consistent(&c, Nearest, o_c);
            let (s_alt, c_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
            let (s_alt, c_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
            if o_s != Equal && o_c != Equal && u32::try_from(u).is_ok() {
                let (rug_s, rug_c, rug_o_s, rug_o_c) =
                    rug_sin_cos_with_period_rational_prec(&x, u, prec);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_s)),
                    ComparableFloatRef(&s)
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_c)),
                    ComparableFloatRef(&c)
                );
                assert_eq!(rug_o_s, o_s);
                assert_eq!(rug_o_c, o_c);
            }
        },
    );
}

// Inputs within 2^(-2^30) of a half turn and of a quarter turn, where the sine or the cosine
// underflows, including non-dyadic ones that no `Float` could express.
#[test]
fn test_sin_cos_with_period_rational_underflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let above = Rational::from(180u32) + &eps;
    let below = Rational::from(180u32) - &eps;
    // just past a half turn: the sine is negative and tiny, and the cosine is just above -1
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&above, 360, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-1.0000");
    assert_eq!(o_c, Less);
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&above, 360, 10, Floor);
    assert_eq!(ComparableFloat(s), ComparableFloat(-&min_positive));
    assert_eq!(o_s, Less);
    assert_eq!(c.to_string(), "-1.0000");
    assert_eq!(o_c, Less);
    // just short of it: positive and tiny
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&below, 360, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o_c, Greater);
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&below, 360, 10, Down);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
    assert_eq!(o_s, Less);
    assert_eq!(c.to_string(), "-0.99902");
    assert_eq!(o_c, Greater);
    let above = Rational::from(90u32) + &eps;
    let below = Rational::from(90u32) - &eps;
    // just past a quarter turn: the cosine is negative and tiny, and the sine is just below 1
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&above, 360, 10, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o_c, Greater);
    assert_eq!(s.to_string(), "1.0000");
    assert_eq!(o_s, Greater);
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&above, 360, 10, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(-&min_positive));
    assert_eq!(o_c, Less);
    assert_eq!(s.to_string(), "0.99902");
    assert_eq!(o_s, Less);
    // just short of it: positive and tiny
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&below, 360, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min_positive));
    assert_eq!(o_c, Greater);
    assert_eq!(s.to_string(), "1.0000");
    assert_eq!(o_s, Greater);
    let (s, c, o_s, o_c) =
        Float::sin_cos_with_period_rational_prec_round_ref(&below, 360, 10, Down);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o_c, Less);
    assert_eq!(s.to_string(), "0.99902");
    assert_eq!(o_s, Less);
    // non-dyadic versions: 1/2 and 1/4 of a turn plus 1/(3 * 2^(2^30 + 70))
    let tiny = Rational::power_of_2(-((1i64 << 30) + 70)) / Rational::from(3u32);
    let x = Rational::from_unsigneds(1u32, 2u32) + &tiny;
    let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round_ref(&x, 1, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o_s, Greater);
    assert_eq!(c.to_string(), "-1.0000");
    assert_eq!(o_c, Less);
    let x = Rational::from_unsigneds(1u32, 4u32) + tiny;
    let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round_ref(&x, 1, 10, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o_c, Greater);
    assert_eq!(s.to_string(), "1.0000");
    assert_eq!(o_s, Greater);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_cos_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out_s: T, out_c: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let (s, c) = primitive_float_sin_cos_with_period(x, u);
        assert_eq!(NiceFloat(s), NiceFloat(out_s));
        assert_eq!(NiceFloat(c), NiceFloat(out_c));
    }
    test::<f32>(f32::NAN, 360, f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN, f32::NAN);
    test::<f32>(1.0, 0, f32::NAN, f32::NAN);
    test::<f32>(0.0, 360, 0.0, 1.0);
    test::<f32>(-0.0, 360, -0.0, 1.0);
    test::<f32>(90.0, 360, 1.0, 0.0);
    test::<f32>(180.0, 360, 0.0, -1.0);
    test::<f32>(-180.0, 360, -0.0, -1.0);
    test::<f32>(270.0, 360, -1.0, 0.0);
    test::<f32>(360.0, 360, 0.0, 1.0);
    test::<f32>(-360.0, 360, -0.0, 1.0);
    test::<f32>(30.0, 360, 0.5, 0.8660254);
    test::<f32>(150.0, 360, 0.5, -0.8660254);
    test::<f32>(210.0, 360, -0.5, -0.8660254);
    test::<f32>(45.0, 360, 0.70710677, 0.70710677);
    test::<f32>(60.0, 360, 0.8660254, 0.5);
    test::<f32>(18.0, 360, 0.309017, 0.95105654);
    test::<f32>(54.0, 360, 0.809017, 0.58778524);
    test::<f32>(1.0, 7, 0.7818315, 0.6234898);
    test::<f32>(-1.0, 7, -0.7818315, 0.6234898);
    test::<f32>(2.0, 7, 0.9749279, -0.22252093);
    test::<f32>(1.0, 360, 0.017452406, 0.9998477);
    test::<f32>(100.0, 360, 0.9848077, -0.17364818);
    test::<f32>(10000000000.0, 360, -0.9848077, 0.17364818);
    test::<f32>(1.0e30, 7, 0.7818315, 0.6234898);
    test::<f32>(1.0e-30, 7, 8.975979e-31, 1.0);
    test::<f32>(3.4028235e38, 360, 0.0, 1.0);
    test::<f32>(0.5, 1, 0.0, -1.0);
    test::<f32>(0.25, 1, 1.0, 0.0);
    test::<f32>(0.1, 1, 0.58778524, 0.809017);
    test::<f32>(1.0e-45, 1, 8.0e-45, 1.0);
    test::<f32>(1.0e-45, 360, 0.0, 1.0);
    test::<f32>(-1.0e-45, 1, -8.0e-45, 1.0);
    test::<f32>(120.0, 360, 0.8660254, -0.5);
    test::<f32>(72.0, 360, 0.95105654, 0.309017);
    test::<f32>(36.0, 360, 0.58778524, 0.809017);
    test::<f64>(f64::NAN, 360, f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN, f64::NAN);
    test::<f64>(1.0, 0, f64::NAN, f64::NAN);
    test::<f64>(0.0, 360, 0.0, 1.0);
    test::<f64>(-0.0, 360, -0.0, 1.0);
    test::<f64>(90.0, 360, 1.0, 0.0);
    test::<f64>(180.0, 360, 0.0, -1.0);
    test::<f64>(-180.0, 360, -0.0, -1.0);
    test::<f64>(270.0, 360, -1.0, 0.0);
    test::<f64>(360.0, 360, 0.0, 1.0);
    test::<f64>(-360.0, 360, -0.0, 1.0);
    test::<f64>(30.0, 360, 0.5, 0.8660254037844386);
    test::<f64>(150.0, 360, 0.5, -0.8660254037844386);
    test::<f64>(210.0, 360, -0.5, -0.8660254037844386);
    test::<f64>(
        45.0,
        360,
        core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    test::<f64>(60.0, 360, 0.8660254037844386, 0.5);
    test::<f64>(18.0, 360, 0.30901699437494745, 0.9510565162951535);
    test::<f64>(54.0, 360, 0.8090169943749475, 0.5877852522924731);
    test::<f64>(1.0, 7, 0.7818314824680298, 0.6234898018587335);
    test::<f64>(-1.0, 7, -0.7818314824680298, 0.6234898018587335);
    test::<f64>(2.0, 7, 0.9749279121818236, -0.2225209339563144);
    test::<f64>(1.0, 360, 0.01745240643728351, 0.9998476951563913);
    test::<f64>(100.0, 360, 0.984807753012208, -0.17364817766693036);
    test::<f64>(10000000000.0, 360, -0.984807753012208, 0.17364817766693036);
    test::<f64>(1.0e100, 7, 0.9749279121818236, -0.2225209339563144);
    test::<f64>(1.0e-100, 7, 8.975979010256552e-101, 1.0);
    test::<f64>(
        1.7976931348623157e308,
        360,
        0.7880107536067219,
        -0.6156614753256583,
    );
    test::<f64>(0.5, 1, 0.0, -1.0);
    test::<f64>(0.25, 1, 1.0, 0.0);
    test::<f64>(0.1, 1, 0.5877852522924731, 0.8090169943749475);
    test::<f64>(5.0e-324, 1, 3.0e-323, 1.0);
    test::<f64>(5.0e-324, 360, 0.0, 1.0);
    test::<f64>(-5.0e-324, 1, -3.0e-323, 1.0);
    test::<f64>(120.0, 360, 0.8660254037844386, -0.5);
    test::<f64>(72.0, 360, 0.9510565162951535, 0.30901699437494745);
    test::<f64>(36.0, 360, 0.5877852522924731, 0.8090169943749475);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_cos_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out_s: T, out_c: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        let (s, c) = primitive_float_sin_cos_with_period_rational::<T>(&x, u);
        assert_eq!(NiceFloat(s), NiceFloat(out_s));
        assert_eq!(NiceFloat(c), NiceFloat(out_c));
    }
    test::<f32>("0", 360, 0.0, 1.0);
    test::<f32>("1", 0, f32::NAN, f32::NAN);
    test::<f32>("90", 360, 1.0, 0.0);
    test::<f32>("180", 360, 0.0, -1.0);
    test::<f32>("-180", 360, -0.0, -1.0);
    test::<f32>("30", 360, 0.5, 0.8660254);
    test::<f32>("45", 360, 0.70710677, 0.70710677);
    test::<f32>("60", 360, 0.8660254, 0.5);
    test::<f32>("18", 360, 0.309017, 0.95105654);
    test::<f32>("54", 360, 0.809017, 0.58778524);
    test::<f32>("1/12", 1, 0.5, 0.8660254);
    test::<f32>("1/3", 1, 0.8660254, -0.5);
    test::<f32>("1/8", 1, 0.70710677, 0.70710677);
    test::<f32>("1/20", 1, 0.309017, 0.95105654);
    test::<f32>("1/5", 1, 0.95105654, 0.309017);
    test::<f32>("1/7", 1, 0.7818315, 0.6234898);
    test::<f32>("-2/7", 1, -0.9749279, -0.22252093);
    test::<f32>("22/7", 1, 0.7818315, 0.6234898);
    test::<f32>("1", 7, 0.7818315, 0.6234898);
    test::<f32>("1000000", 7, 0.7818315, 0.6234898);
    test::<f32>("1/1000000", 1, 0.0000062831855, 1.0);
    test::<f32>("355/113", 360, 0.05480367, 0.9984971);
    test::<f32>("36", 360, 0.58778524, 0.809017);
    test::<f64>("0", 360, 0.0, 1.0);
    test::<f64>("1", 0, f64::NAN, f64::NAN);
    test::<f64>("90", 360, 1.0, 0.0);
    test::<f64>("180", 360, 0.0, -1.0);
    test::<f64>("-180", 360, -0.0, -1.0);
    test::<f64>("30", 360, 0.5, 0.8660254037844386);
    test::<f64>(
        "45",
        360,
        core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    test::<f64>("60", 360, 0.8660254037844386, 0.5);
    test::<f64>("18", 360, 0.30901699437494745, 0.9510565162951535);
    test::<f64>("54", 360, 0.8090169943749475, 0.5877852522924731);
    test::<f64>("1/12", 1, 0.5, 0.8660254037844386);
    test::<f64>("1/3", 1, 0.8660254037844386, -0.5);
    test::<f64>(
        "1/8",
        1,
        core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    test::<f64>("1/20", 1, 0.30901699437494745, 0.9510565162951535);
    test::<f64>("1/5", 1, 0.9510565162951535, 0.30901699437494745);
    test::<f64>("1/7", 1, 0.7818314824680298, 0.6234898018587335);
    test::<f64>("-2/7", 1, -0.9749279121818236, -0.2225209339563144);
    test::<f64>("22/7", 1, 0.7818314824680298, 0.6234898018587335);
    test::<f64>("1", 7, 0.7818314824680298, 0.6234898018587335);
    test::<f64>("1000000", 7, 0.7818314824680298, 0.6234898018587335);
    test::<f64>("1/1000000", 1, 6.283185307138245e-6, 0.9999999999802608);
    test::<f64>("355/113", 360, 0.05480366979770582, 0.9984971496087027);
    test::<f64>("36", 360, 0.5877852522924731, 0.8090169943749475);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_cos_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // the two results are those of the separate functions
        let (s, c) = primitive_float_sin_cos_with_period(x, u);
        assert_eq!(
            NiceFloat(s),
            NiceFloat(primitive_float_sin_with_period(x, u))
        );
        assert_eq!(
            NiceFloat(c),
            NiceFloat(primitive_float_cos_with_period(x, u))
        );
        // and, for a nonzero x, those of the `Rational` path (a `Rational` cannot carry the sign of
        // a zero)
        if x != T::ZERO {
            let (s_alt, c_alt) =
                primitive_float_sin_cos_with_period_rational::<T>(&Rational::exact_from(x), u);
            assert_eq!(NiceFloat(s_alt), NiceFloat(s));
            assert_eq!(NiceFloat(c_alt), NiceFloat(c));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // NaN exactly for NaN and infinite inputs
        let (s, c) = primitive_float_sin_cos_with_period(x, 7);
        assert_eq!(s.is_nan(), !x.is_finite());
        assert_eq!(c.is_nan(), !x.is_finite());
    });

    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let (s, c) = primitive_float_sin_cos_with_period_rational::<T>(&x, u);
        assert_eq!(
            NiceFloat(s),
            NiceFloat(primitive_float_sin_with_period_rational::<T>(&x, u))
        );
        assert_eq!(
            NiceFloat(c),
            NiceFloat(primitive_float_cos_with_period_rational::<T>(&x, u))
        );
    });
}

#[test]
fn primitive_float_sin_cos_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_cos_with_period_properties_helper);
}

#[test]
fn test_sin_cos_pi_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (sin, cos, o_s, o_c) = x.clone().sin_cos_pi_prec_round(prec, rm);
        assert!(sin.is_valid());
        assert!(cos.is_valid());
        assert_eq!(sin.to_string(), out_s);
        assert_eq!(to_hex_string(&sin), out_s_hex);
        assert_eq!(cos.to_string(), out_c);
        assert_eq!(to_hex_string(&cos), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (sin_alt, cos_alt, o_s_alt, o_c_alt) = x.sin_cos_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        let mut sin_alt = x.clone();
        let mut cos_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = sin_alt.sin_cos_pi_prec_round_assign(&mut cos_alt, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if rm == Nearest {
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) = x.sin_cos_pi_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
        }

        // the two results are those of the separate functions
        let (sin_alt, o_s_alt) = x.sin_pi_prec_round_ref(prec, rm);
        let (cos_alt, o_c_alt) = x.cos_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_pi_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&sin)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&cos)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    };
    test(
        "NaN", "NaN", 1, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "NaN", "NaN", 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "NaN", "NaN", 10, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "NaN", "NaN", 10, Ceiling, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "NaN", "NaN", 10, Exact, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "NaN", "NaN", 53, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 10, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 10, Floor, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 10, Ceiling, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 10, Exact, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "Infinity", "Infinity", 53, Nearest, "NaN", "NaN", "NaN", "NaN", Equal, Equal,
    );
    test(
        "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0.0",
        "0x0.0",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "0.50", "0x0.8#1", 1, Nearest, "1.0", "0x1.0#1", "0.0", "0x0.0", Equal, Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5", "0x1.8#2", 1, Nearest, "-1.0", "-0x1.0#1", "0.0", "0x0.0", Equal, Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", "-1.0", "-0x1.0#1", Equal, Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.25", "0x0.4#1", 1, Nearest, "0.50", "0x0.8#1", "0.50", "0x0.8#1", Less, Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "0.30908",
        "0x0.4f2#10",
        "0.95117",
        "0x0.f38#10",
        Greater,
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        "0.30859",
        "0x0.4f0#10",
        Less,
        Less,
    );
    test(
        "100.2",
        "0x64.4#9",
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0",
        "0x1.0#1",
        Equal,
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Floor,
        "3.1400e-30",
        "0x3.fbE-25#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-0.30908",
        "-0x0.4f2#10",
        "0.95117",
        "0x0.f38#10",
        Less,
        Greater,
    );
    test(
        "2.0",
        "0x2.0#2",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        20,
        Nearest,
        "0.70710659",
        "0x0.b504f#20",
        "0.70710659",
        "0x0.b504f#20",
        Less,
        Less,
    );
    test(
        "1.2",
        "0x1.4#3",
        20,
        Nearest,
        "-0.70710659",
        "-0x0.b504f#20",
        "-0.70710659",
        "-0x0.b504f#20",
        Greater,
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "100.2",
        "0x64.4#9",
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        "0.70801",
        "0x0.b54#10",
        Greater,
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-0.30908",
        "-0x0.4f2#10",
        "0.95020",
        "0x0.f34#10",
        Less,
        Less,
    );
}

#[test]
fn test_sin_cos_pi_rational_prec_round() {
    let test = |s: &str,
                prec: u64,
                rm: RoundingMode,
                out_s: &str,
                out_s_hex: &str,
                out_c: &str,
                out_c_hex: &str,
                o_s_out: Ordering,
                o_c_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (sin, cos, o_s, o_c) = Float::sin_cos_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(sin.is_valid());
        assert!(cos.is_valid());
        assert_eq!(sin.to_string(), out_s);
        assert_eq!(to_hex_string(&sin), out_s_hex);
        assert_eq!(cos.to_string(), out_c);
        assert_eq!(to_hex_string(&cos), out_c_hex);
        assert_eq!(o_s, o_s_out);
        assert_eq!(o_c, o_c_out);

        let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        if rm == Nearest {
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_pi_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
            let (sin_alt, cos_alt, o_s_alt, o_c_alt) =
                Float::sin_cos_pi_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
            assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
            assert_eq!(o_s_alt, o_s);
            assert_eq!(o_c_alt, o_c);
        }

        // the two results are those of the separate functions
        let (sin_alt, o_s_alt) = Float::sin_pi_rational_prec_round_ref(&x, prec, rm);
        let (cos_alt, o_c_alt) = Float::cos_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&sin_alt), ComparableFloatRef(&sin));
        assert_eq!(ComparableFloatRef(&cos_alt), ComparableFloatRef(&cos));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);

        // MPFR rounds the input first, so it cannot see the exact cases of non-dyadic inputs
        if o_s != Equal
            && o_c != Equal
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_pi_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&sin)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&cos)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    };
    test(
        "0", 1, Nearest, "0.0", "0x0.0", "1.0", "0x1.0#1", Equal, Equal,
    );
    test(
        "0",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Floor,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "0",
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "1/2", 1, Nearest, "1.0", "0x1.0#1", "0.0", "0x0.0", Equal, Equal,
    );
    test(
        "1/2",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1/2",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1/2",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1/2",
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1/2",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1", 1, Nearest, "0.0", "0x0.0", "-1.0", "-0x1.0#1", Equal, Equal,
    );
    test(
        "1",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1",
        10,
        Floor,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1",
        10,
        Exact,
        "0.0",
        "0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "1",
        53,
        Nearest,
        "0.0",
        "0x0.0",
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
        Equal,
    );
    test(
        "1/3", 1, Nearest, "1.0", "0x1.0#1", "0.50", "0x0.8#1", Greater, Equal,
    );
    test(
        "1/3",
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        "0.50000",
        "0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "1/3",
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        "0.50000",
        "0x0.800#10",
        Less,
        Equal,
    );
    test(
        "2/3", 1, Nearest, "1.0", "0x1.0#1", "-0.50", "-0x0.8#1", Greater, Equal,
    );
    test(
        "2/3",
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        "-0.50000",
        "-0x0.800#10",
        Greater,
        Equal,
    );
    test(
        "1/4",
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "1/4",
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
        Greater,
    );
    test(
        "1/6",
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        "0.86523",
        "0x0.dd8#10",
        Equal,
        Less,
    );
    test(
        "1/5", 1, Nearest, "0.50", "0x0.8#1", "1.0", "0x1.0#1", Less, Greater,
    );
    test(
        "1/5",
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        "0.80957",
        "0x0.cf4#10",
        Greater,
        Greater,
    );
    test(
        "2/5",
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "2/5",
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        "0.30901699437494745",
        "0x0.4f1bbcdcbfa540#53",
        Less,
        Greater,
    );
    test(
        "1/7",
        10,
        Floor,
        "0.43359",
        "0x0.6f0#10",
        "0.90039",
        "0x0.e68#10",
        Less,
        Less,
    );
    test(
        "-3/7", 1, Nearest, "-1.0", "-0x1.0#1", "0.25", "0x0.4#1", Less, Greater,
    );
    test(
        "-3/7",
        10,
        Ceiling,
        "-0.97461",
        "-0x0.f98#10",
        "0.22266",
        "0x0.390#10",
        Greater,
        Greater,
    );
    test(
        "22/7",
        10,
        Nearest,
        "-0.43408",
        "-0x0.6f2#10",
        "-0.90137",
        "-0x0.e6c#10",
        Less,
        Less,
    );
    test(
        "22/7",
        53,
        Nearest,
        "-0.43388373911755812",
        "-0x0.6f130135c6af04#53",
        "-0.90096886790241915",
        "-0x0.e6a5e54e5ae388#53",
        Greater,
        Less,
    );
    test(
        "1/1000000",
        10,
        Floor,
        "3.1404e-6",
        "0x0.000034b#10",
        "0.99902",
        "0x0.ffc#10",
        Less,
        Less,
    );
    test(
        "-1",
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        "-1.0000",
        "-0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "-2",
        10,
        Exact,
        "-0.0",
        "-0x0.0",
        "1.0000",
        "0x1.000#10",
        Equal,
        Equal,
    );
    test(
        "3/2",
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        "0.0",
        "0x0.0",
        Equal,
        Equal,
    );
    test(
        "1/6",
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        "0.86621",
        "0x0.ddc#10",
        Equal,
        Greater,
    );
    test(
        "1/4",
        20,
        Nearest,
        "0.70710659",
        "0x0.b504f#20",
        "0.70710659",
        "0x0.b504f#20",
        Less,
        Less,
    );
    test(
        "3/10",
        20,
        Nearest,
        "0.80901718",
        "0x0.cf1bc#20",
        "0.58778572",
        "0x0.96792#20",
        Greater,
        Greater,
    );
    test(
        "1/4",
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        "0.70703",
        "0x0.b50#10",
        Less,
        Less,
    );
    test(
        "1/5",
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        "0.80901699437494745",
        "0x0.cf1bbcdcbfa540#53",
        Greater,
        Greater,
    );
    test(
        "2/5",
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        "0.30908",
        "0x0.4f2#10",
        Greater,
        Greater,
    );
    test(
        "-3/7",
        10,
        Nearest,
        "-0.97461",
        "-0x0.f98#10",
        "0.22241",
        "0x0.38f#10",
        Greater,
        Less,
    );
    test(
        "22/7", 1, Nearest, "-0.50", "-0x0.8#1", "-1.0", "-0x1.0#1", Less, Less,
    );
    test(
        "1/1000000",
        10,
        Ceiling,
        "3.1441e-6",
        "0x0.000034c#10",
        "1.0000",
        "0x1.000#10",
        Greater,
        Greater,
    );
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_cos_pi() {
    fn test<T: PrimitiveFloat>(x: T, out_s: T, out_c: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let (s, c) = primitive_float_sin_cos_pi(x);
        assert_eq!(NiceFloat(s), NiceFloat(out_s));
        assert_eq!(NiceFloat(c), NiceFloat(out_c));
    }
    test::<f32>(f32::NAN, f32::NAN, f32::NAN);
    test::<f32>(0.5, 1.0, 0.0);
    test::<f32>(1.0, 0.0, -1.0);
    test::<f32>(-1.0, -0.0, -1.0);
    test::<f32>(
        0.25,
        core::f32::consts::FRAC_1_SQRT_2,
        core::f32::consts::FRAC_1_SQRT_2,
    );
    test::<f32>(0.1, 0.309017, 0.95105654);
    test::<f32>(-0.1, -0.309017, 0.95105654);
    test::<f32>(
        100.25,
        core::f32::consts::FRAC_1_SQRT_2,
        core::f32::consts::FRAC_1_SQRT_2,
    );
    test::<f32>(10000000000.0, 0.0, 1.0);
    test::<f32>(1.0e-45, 4.0e-45, 1.0);
    test::<f32>(10000000000.0, 0.0, 1.0);
    test::<f64>(f64::NAN, f64::NAN, f64::NAN);
    test::<f64>(0.5, 1.0, 0.0);
    test::<f64>(1.0, 0.0, -1.0);
    test::<f64>(-1.0, -0.0, -1.0);
    test::<f64>(
        0.25,
        core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    test::<f64>(0.1, 0.30901699437494745, 0.9510565162951535);
    test::<f64>(-0.1, -0.30901699437494745, 0.9510565162951535);
    test::<f64>(
        100.25,
        core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    test::<f64>(10000000000.0, 0.0, 1.0);
    test::<f64>(5.0e-324, 1.5e-323, 1.0);
    test::<f64>(10000000000.0, 0.0, 1.0);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_cos_pi_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out_s: T, out_c: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        let (s, c) = primitive_float_sin_cos_pi_rational::<T>(&x);
        assert_eq!(NiceFloat(s), NiceFloat(out_s));
        assert_eq!(NiceFloat(c), NiceFloat(out_c));
    }
    test::<f32>("1/2", 1.0, 0.0);
    test::<f32>("1", 0.0, -1.0);
    test::<f32>("-1", -0.0, -1.0);
    test::<f32>("1/6", 0.5, 0.8660254);
    test::<f32>("1/3", 0.8660254, 0.5);
    test::<f32>(
        "1/4",
        core::f32::consts::FRAC_1_SQRT_2,
        core::f32::consts::FRAC_1_SQRT_2,
    );
    test::<f32>("1/7", 0.43388373, 0.90096885);
    test::<f32>("-3/7", -0.9749279, 0.22252093);
    test::<f32>("22/7", -0.43388373, -0.90096885);
    test::<f64>("1/2", 1.0, 0.0);
    test::<f64>("1", 0.0, -1.0);
    test::<f64>("-1", -0.0, -1.0);
    test::<f64>("1/6", 0.5, 0.8660254037844386);
    test::<f64>("1/3", 0.8660254037844386, 0.5);
    test::<f64>(
        "1/4",
        core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    test::<f64>("1/7", 0.4338837391175581, 0.9009688679024191);
    test::<f64>("-3/7", -0.9749279121818236, 0.2225209339563144);
    test::<f64>("22/7", -0.4338837391175581, -0.9009688679024191);
}

#[test]
#[should_panic]
fn sin_cos_pi_prec_round_fail_1() {
    Float::from(0.1f64).sin_cos_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sin_cos_pi_prec_round_fail_2() {
    Float::from(0.1f64).sin_cos_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_pi_prec_round_fail_3() {
    // the sine of a sixth of a half-turn is exact, but the cosine is not
    Float::from_unsigned_prec(1u32, 10)
        .0
        .div_prec_round(Float::from(6u32), 10, Nearest)
        .0
        .sin_cos_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_pi_rational_prec_round_fail_1() {
    Float::sin_cos_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Exact);
}

#[test]
#[should_panic]
fn sin_cos_pi_rational_prec_round_fail_2() {
    Float::sin_cos_pi_rational_prec_round(Rational::from_unsigneds(1u8, 6), 10, Exact);
}

// Every `sin_cos_pi` variant is `sin_cos_with_period` with a period of 2.
#[test]
fn sin_cos_pi_properties() {
    // The borrowed generators admit `Exact` for inputs whose results are not both exact, so `Exact`
    // is checked against the exactness of the results.
    let exact_ok = |x: &Float, prec: u64, rm: RoundingMode| {
        rm != Exact || {
            let (_, _, o_s, o_c) = x.sin_cos_with_period_prec_round_ref(2, prec, Nearest);
            o_s == Equal && o_c == Equal
        }
    };
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            assert_panic!(x.sin_cos_pi_prec_round_ref(prec, Exact));
            return;
        }
        let (s, c, o_s, o_c) = x.clone().sin_cos_pi_prec_round(prec, rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o_s);
        assert_rounding_ordering_consistent(&c, rm, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_pi_prec_round_assign(&mut c_alt, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_pi_prec_round(&rug::Float::exact_from(&x), prec, rrm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_37().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            return;
        }
        let (s, c, o_s, o_c) = x.sin_cos_pi_prec_round_ref(prec, rm);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, c, o_s, o_c) = x.clone().sin_cos_pi_prec(prec);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_prec_ref(2, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_pi_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_pi_prec_assign(&mut c_alt, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    });

    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        if !exact_ok(&x, x.significant_bits(), rm) {
            return;
        }
        let (s, c, o_s, o_c) = x.clone().sin_cos_pi_round(rm);
        assert_rounding_ordering_consistent(&s, rm, o_s);
        assert_rounding_ordering_consistent(&c, rm, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_with_period_round_ref(2, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = x.sin_cos_pi_round_ref(rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let mut s_alt = x.clone();
        let mut c_alt = Float::NAN;
        let (o_s_alt, o_c_alt) = s_alt.sin_cos_pi_round_assign(&mut c_alt, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    });

    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            let (_, _, o_s, o_c) = Float::sin_cos_with_period_rational_prec_ref(&x, 2, prec);
            if o_s != Equal || o_c != Equal {
                assert_panic!(Float::sin_cos_pi_rational_prec_round_ref(&x, prec, Exact));
                return;
            }
        }
        let (s, c, o_s, o_c) = Float::sin_cos_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(s.is_valid());
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o_s);
        assert_rounding_ordering_consistent(&c, rm, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_with_period_rational_prec_round_ref(&x, 2, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        // MPFR agrees, except that it cannot see the exact cases of non-dyadic inputs
        if o_s != Equal
            && o_c != Equal
            && let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        {
            let (rug_s, rug_c, rug_o_s, rug_o_c) =
                rug_sin_cos_pi_rational_prec_round(&x, prec, rrm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o_s, o_s);
            assert_eq!(rug_o_c, o_c);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (s, c, o_s, o_c) = Float::sin_cos_pi_rational_prec(x.clone(), prec);
        let (s_alt, c_alt, o_s_alt, o_c_alt) =
            Float::sin_cos_with_period_rational_prec_ref(&x, 2, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
        let (s_alt, c_alt, o_s_alt, o_c_alt) = Float::sin_cos_pi_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_s_alt, o_s);
        assert_eq!(o_c_alt, o_c);
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_cos_pi_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let (s, c) = primitive_float_sin_cos_pi(x);
        let (s_alt, c_alt) = primitive_float_sin_cos_with_period(x, 2);
        assert_eq!(NiceFloat(s), NiceFloat(s_alt));
        assert_eq!(NiceFloat(c), NiceFloat(c_alt));
    });
    rational_gen().test_properties(|x| {
        let (s, c) = primitive_float_sin_cos_pi_rational::<T>(&x);
        let (s_alt, c_alt) = primitive_float_sin_cos_with_period_rational::<T>(&x, 2);
        assert_eq!(NiceFloat(s), NiceFloat(s_alt));
        assert_eq!(NiceFloat(c), NiceFloat(c_alt));
    });
}

#[test]
fn primitive_float_sin_cos_pi_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_cos_pi_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_cos() {
    fn test<T: PrimitiveFloat>(x: T, out_s: T, out_c: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let (s, c) = primitive_float_sin_cos(x);
        assert_eq!(NiceFloat(s), NiceFloat(out_s));
        assert_eq!(NiceFloat(c), NiceFloat(out_c));
    }
    test::<f32>(f32::NAN, f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN, f32::NAN);
    test::<f32>(0.0, 0.0, 1.0);
    test::<f32>(-0.0, -0.0, 1.0);
    test::<f32>(1.0, 0.84147096, 0.5403023);
    test::<f32>(-1.0, -0.84147096, 0.5403023);
    test::<f32>(0.5, 0.47942555, 0.87758255);
    test::<f32>(2.0, 0.9092974, -0.41614684);
    test::<f32>(core::f32::consts::PI, -8.742278e-8, -1.0);
    test::<f32>(core::f32::consts::FRAC_PI_2, 1.0, -4.371139e-8);
    test::<f32>(100.0, -0.50636566, 0.8623189);
    test::<f32>(1.0e10, -0.48750603, 0.87311965);
    test::<f32>(3.4028235e38, -0.5218765, 0.853021);
    test::<f32>(1.0e-45, 1.0e-45, 1.0);

    test::<f64>(f64::NAN, f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN, f64::NAN);
    test::<f64>(0.0, 0.0, 1.0);
    test::<f64>(-0.0, -0.0, 1.0);
    test::<f64>(1.0, 0.8414709848078965, 0.5403023058681398);
    test::<f64>(-1.0, -0.8414709848078965, 0.5403023058681398);
    test::<f64>(0.5, 0.479425538604203, 0.8775825618903728);
    test::<f64>(2.0, 0.9092974268256817, -0.4161468365471424);
    test::<f64>(core::f64::consts::PI, 1.2246467991473532e-16, -1.0);
    test::<f64>(core::f64::consts::FRAC_PI_2, 1.0, 6.123233995736766e-17);
    test::<f64>(100.0, -0.5063656411097588, 0.8623188722876839);
    test::<f64>(1.0e100, -0.3806377310050287, 0.9247242387519338);
    test::<f64>(
        1.7976931348623157e308,
        0.004961954789184062,
        -0.9999876894265599,
    );
    test::<f64>(5.0e-324, 5.0e-324, 1.0);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_cos_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out_s: T, out_c: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        let (sin, cos) = primitive_float_sin_cos_rational::<T>(&x);
        assert_eq!(NiceFloat(sin), NiceFloat(out_s));
        assert_eq!(NiceFloat(cos), NiceFloat(out_c));
    }
    test::<f32>("0", 0.0, 1.0);
    test::<f32>("1", 0.84147096, 0.5403023);
    test::<f32>("1/2", 0.47942555, 0.87758255);
    test::<f32>("1/3", 0.3271947, 0.94495696);
    test::<f32>("22/7", -0.0012644889, -0.9999992);
    test::<f32>("355/113", -2.6676418e-7, -1.0);
    test::<f32>("-1000000", 0.3499935, 0.93675214);
    test::<f32>("1/1000000", 0.000001, 1.0);
    test::<f32>("10000", -0.30561438, -0.95215535);
    test::<f64>("0", 0.0, 1.0);
    test::<f64>("1", 0.8414709848078965, 0.5403023058681398);
    test::<f64>("1/2", 0.479425538604203, 0.8775825618903728);
    test::<f64>("1/3", 0.32719469679615226, 0.9449569463147377);
    test::<f64>("22/7", -0.0012644889303773533, -0.999999200533553);
    test::<f64>("355/113", -2.6676418906241917e-7, -0.9999999999999645);
    test::<f64>("-1000000", 0.34999350217129294, 0.9367521275331447);
    test::<f64>("1/1000000", 9.999999999998333e-7, 0.9999999999995);
    test::<f64>("10000", -0.30561438888825215, -0.9521553682590148);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_cos_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        // the two results are those of the separate functions
        let (s, c) = primitive_float_sin_cos(x);
        assert_eq!(NiceFloat(s), NiceFloat(primitive_float_sin(x)));
        assert_eq!(NiceFloat(c), NiceFloat(primitive_float_cos(x)));
    });
    rational_gen().test_properties(|x| {
        let (s, c) = primitive_float_sin_cos_rational::<T>(&x);
        assert_eq!(
            NiceFloat(s),
            NiceFloat(primitive_float_sin_rational::<T>(&x))
        );
        assert_eq!(
            NiceFloat(c),
            NiceFloat(primitive_float_cos_rational::<T>(&x))
        );
    });
}

#[test]
fn primitive_float_sin_cos_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_cos_properties_helper);
}
