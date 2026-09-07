// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Cos, Sin, SinCos, SinCosAssign};
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
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sin_cos::{
    rug_sin_cos, rug_sin_cos_prec, rug_sin_cos_prec_round, rug_sin_cos_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

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
