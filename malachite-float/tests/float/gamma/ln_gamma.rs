// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Two, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::gamma::traits::{LnGamma, LnGammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::gamma::ln_gamma::{
    primitive_float_ln_gamma, primitive_float_ln_gamma_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, exponent_in_gate, parse_hex_string,
    rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::gamma::ln_gamma::{
    rug_ln_gamma_prec_round, rug_ln_gamma_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;

#[test]
fn test_ln_gamma() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let ln_gamma = x.clone().ln_gamma();
        assert!(ln_gamma.is_valid());

        assert_eq!(ln_gamma.to_string(), out);
        assert_eq!(to_hex_string(&ln_gamma), out_hex);

        let ln_gamma_alt = (&x).ln_gamma();
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );

        let mut ln_gamma_alt = x.clone();
        ln_gamma_alt.ln_gamma_assign();
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "Infinity", "Infinity");
    test("-0.0", "-0x0.0", "Infinity", "Infinity");
    test("1.0", "0x1.0#1", "0.0", "0x0.0");
    test("2.0", "0x2.0#1", "0.0", "0x0.0");
    test("3.0", "0x3.0#2", "0.75", "0x0.c#2");
    test("4.0", "0x4.0#1", "2.0", "0x2.0#1");
    test("5.0", "0x5.0#3", "3.0", "0x3.0#3");
    test("10.0", "0xa.0#3", "12.0", "0xc.0#3");
    test("25.0", "0x19.0#5", "54.0", "0x36.0#5");
    test("171.0", "0xab.0#8", "708.0", "0x2c4.0#8");
    test("0.50", "0x0.8#1", "0.50", "0x0.8#1");
    test("1.5", "0x1.8#2", "-0.12", "-0x0.2#2");
    test("0.25", "0x0.4#1", "1.0", "0x1.0#1");
    test("0.75", "0x0.c#2", "0.19", "0x0.3#2");
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        "2.2527126517342060",
        "0x2.40b1c6be7bddc#52",
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        "0.98709857789473432",
        "0x0.fcb27e0dfc6a0#52",
    );
    test("-0.50", "-0x0.8#1", "NaN", "NaN");
    test("-1.5", "-0x1.8#2", "NaN", "NaN");
    test("-2.5", "-0x2.8#3", "NaN", "NaN");
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        "NaN",
        "NaN",
    );
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test("-2.0", "-0x2.0#1", "NaN", "NaN");
    test("-100.5", "-0x64.8#8", "NaN", "NaN");
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        "-5.7721558299185070e-8",
        "-0xf.7e9864728c5c8E-7#53",
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        "-4.2278430309861296e-8",
        "-0xb.595a02fd42d18E-7#53",
    );
    test("-2.0000000999999998", "-0x2.000001ad7f29a#53", "NaN", "NaN");
    test("-2.4570247382208006", "-0x2.74ff92c01f0d8#51", "NaN", "NaN");
    test("-2.7476826467274127", "-0x2.bf6821437b202#53", "NaN", "NaN");
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        "0.82769459232343710152957855845238",
        "0x0.d3e3caf51aef5edf2aa6dabf8#100",
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        "0.98542064692776706918717403697768",
        "0x0.fc44870b5ba0ce071ed58f220#100",
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        "NaN",
        "NaN",
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        "23.025850929882736",
        "0x17.069e2aa26ae4#53",
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        "NaN",
        "NaN",
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        "690.77552789821368",
        "0x2b2.c688ff0ff6a#53",
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        "NaN",
        "NaN",
    );
    test("1000.5", "0x3e8.8#11", "5908.0", "0x1714.0#11");
    test("1.0000e5", "0x1.86aE+4#12", "1.0511e6", "0x1.00aE+5#12");
    test("1000000.5", "0xf4240.8#21", "12815512.0", "0xc38c98.0#21");
    test("-1000000.5", "-0xf4240.8#21", "NaN", "NaN");
    test("6.7e7", "0x4.0E+6#1", "1.1e9", "0x4.0E+7#1");
    test(
        "67108864.50",
        "0x4000000.8#28",
        "1142315456.0",
        "0x441659c0.0#28",
    );
    test("-67108864.50", "-0x4000000.8#28", "NaN", "NaN");
    test("-67108865.50", "-0x4000001.8#28", "NaN", "NaN");
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        "6.8977552789821359e302",
        "0x4.05fce39e86adE+251#51",
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        "NaN",
        "NaN",
    );
    test("1.0e-301030", "0x1.0E-250000#1", "5.2e5", "0x8.0E+4#1");
    test("-1.0e-301030", "-0x1.0E-250000#1", "NaN", "NaN");
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "7.4449e8",
        "0x2.c6E+7#10",
    );
    test("-2.3826e-323228497", "-0x1.000E-268435456#10", "NaN", "NaN");
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        "Infinity",
        "Infinity",
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        "5.5e161614256",
        "0x1.0E+134217735#1",
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        "-3.5920219013030161753354382453583799958567583443853280143520578e-61",
        "-0x9.3c467e37db0c7a4d1be3f810152cb56a1cecc3af65cc0190b0E-51#201",
    );
}

#[test]
fn test_ln_gamma_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln_gamma, o) = x.clone().ln_gamma_prec(prec);
        assert!(ln_gamma.is_valid());

        assert_eq!(ln_gamma.to_string(), out);
        assert_eq!(to_hex_string(&ln_gamma), out_hex);
        assert_eq!(o, o_out);

        let (ln_gamma_alt, o_alt) = x.ln_gamma_prec_ref(prec);
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut ln_gamma_alt = x.clone();
        let o_alt = ln_gamma_alt.ln_gamma_prec_assign(prec);
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 10, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 1, "0.50", "0x0.8#1", Less);
    test("3.0", "0x3.0#2", 10, "0.69336", "0x0.b18#10", Greater);
    test(
        "3.0",
        "0x3.0#2",
        100,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test("4.0", "0x4.0#1", 1, "2.0", "0x2.0#1", Greater);
    test("4.0", "0x4.0#1", 10, "1.7910", "0x1.ca8#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        "1.7917594692280550008124773583802",
        "0x1.cab0bfa2a20023226dfd40ed0#100",
        Less,
    );
    test("5.0", "0x5.0#3", 1, "4.0", "0x4.0#1", Greater);
    test("5.0", "0x5.0#3", 10, "3.1797", "0x3.2e#10", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test("10.0", "0xa.0#3", 1, "16.0", "0x1.0E+1#1", Greater);
    test("10.0", "0xa.0#3", 10, "12.797", "0xc.cc#10", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61#100",
        Less,
    );
    test("25.0", "0x19.0#5", 1, "64.0", "0x4.0E+1#1", Greater);
    test("25.0", "0x19.0#5", 10, "54.812", "0x36.d#10", Greater);
    test(
        "25.0",
        "0x19.0#5",
        100,
        "54.784729398112319190093344083609",
        "0x36.c8e4069d1a2824fcb11da738#100",
        Greater,
    );
    test("171.0", "0xab.0#8", 1, "5.1e2", "0x2.0E+2#1", Less);
    test("171.0", "0xab.0#8", 10, "707.00", "0x2c3.0#10", Greater);
    test(
        "171.0",
        "0xab.0#8",
        100,
        "706.57306224578734711072226272161",
        "0x2c2.92b435143a9dc09fad3414c#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 10, "0.57227", "0x0.928#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, "-0.12", "-0x0.2#1", Less);
    test("1.5", "0x1.8#2", 10, "-0.12073", "-0x0.1ee8#10", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, "1.0", "0x1.0#1", Less);
    test("0.25", "0x0.4#1", 10, "1.2871", "0x1.498#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, "0.25", "0x0.4#1", Greater);
    test("0.75", "0x0.c#2", 10, "0.20337", "0x0.341#10", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84#100",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        "2.2539",
        "0x2.41#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        100,
        "2.2527126517342059020062379568968",
        "0x2.40b1c6be7bddba149de9373e8#100",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        "0.98730",
        "0x0.fcc#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        100,
        "0.98709857789473440405727868798982",
        "0x0.fcb27e0dfc6a061d0c9f26df8#100",
        Less,
    );
    test("-0.50", "-0x0.8#1", 1, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 100, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 1, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 100, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 1, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 100, "NaN", "NaN", Equal);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 1, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, "NaN", "NaN", Equal);
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        1,
        "-6.0e-8",
        "-0x1.0E-6#1",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        "-5.7742e-8",
        "-0xf.80E-7#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        1,
        "-3.0e-8",
        "-0x8.0E-7#1",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        "-4.2259e-8",
        "-0xb.58E-7#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        "0.82812",
        "0x0.d40#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        "0.98535",
        "0x0.fc4#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        "16.0",
        "0x1.0E+1#1",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        "23.031",
        "0x17.08#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        1,
        "5.1e2",
        "0x2.0E+2#1",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        "691.00",
        "0x2b3.0#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test("1000.5", "0x3e8.8#11", 1, "4.1e3", "0x1.0E+3#1", Less);
    test("1000.5", "0x3e8.8#11", 10, "5912.0", "0x1718.0#10", Greater);
    test("1.0000e5", "0x1.86aE+4#12", 1, "1.0e6", "0x1.0E+5#1", Less);
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        "1.0506e6",
        "0x1.008E+5#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        1,
        "1.7e7",
        "0x1.0E+6#1",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        "1.2812e7",
        "0xc.38E+5#10",
        Less,
    );
    test("-1000000.5", "-0xf4240.8#21", 1, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8#21", 10, "NaN", "NaN", Equal);
    test("6.7e7", "0x4.0E+6#1", 1, "1.1e9", "0x4.0E+7#1", Less);
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        1,
        "1.1e9",
        "0x4.0E+7#1",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test("-67108864.50", "-0x4000000.8#28", 1, "NaN", "NaN", Equal);
    test("-67108864.50", "-0x4000000.8#28", 10, "NaN", "NaN", Equal);
    test("-67108865.50", "-0x4000001.8#28", 1, "NaN", "NaN", Equal);
    test("-67108865.50", "-0x4000001.8#28", 10, "NaN", "NaN", Equal);
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        1,
        "6.9e302",
        "0x4.0E+251#1",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        "6.8978e302",
        "0x4.06E+251#10",
        Greater,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        1,
        "5.2e5",
        "0x8.0E+4#1",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        "6.9325e5",
        "0xa.94E+4#10",
        Greater,
    );
    test("-1.0e-301030", "-0x1.0E-250000#1", 1, "NaN", "NaN", Equal);
    test("-1.0e-301030", "-0x1.0E-250000#1", 10, "NaN", "NaN", Equal);
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        1,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        1,
        "5.5e161614256",
        "0x1.0E+134217735#1",
        Less,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        "7.6262e161614256",
        "0x1.630E+134217735#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        1,
        "-3.1e-61",
        "-0x8.0E-51#1",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        10,
        "-3.5916e-61",
        "-0x9.3cE-51#10",
        Greater,
    );
}

#[test]
#[should_panic]
fn ln_gamma_prec_fail() {
    Float::NAN.ln_gamma_prec(0);
}

#[test]
fn test_ln_gamma_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln_gamma, o) = x.clone().ln_gamma_round(rm);
        assert!(ln_gamma.is_valid());

        assert_eq!(ln_gamma.to_string(), out);
        assert_eq!(to_hex_string(&ln_gamma), out_hex);
        assert_eq!(o, o_out);

        let (ln_gamma_alt, o_alt) = x.ln_gamma_round_ref(rm);
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut ln_gamma_alt = x.clone();
        let o_alt = ln_gamma_alt.ln_gamma_round_assign(rm);
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", Exact, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Down, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Down, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Up, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Exact, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Ceiling, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Down, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Up, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Nearest, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Exact, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", Up, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", Exact, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", Floor, "0.50", "0x0.8#2", Less);
    test("3.0", "0x3.0#2", Ceiling, "0.75", "0x0.c#2", Greater);
    test("3.0", "0x3.0#2", Down, "0.50", "0x0.8#2", Less);
    test("3.0", "0x3.0#2", Up, "0.75", "0x0.c#2", Greater);
    test("3.0", "0x3.0#2", Nearest, "0.75", "0x0.c#2", Greater);
    test("4.0", "0x4.0#1", Floor, "1.0", "0x1.0#1", Less);
    test("4.0", "0x4.0#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("4.0", "0x4.0#1", Down, "1.0", "0x1.0#1", Less);
    test("4.0", "0x4.0#1", Up, "2.0", "0x2.0#1", Greater);
    test("4.0", "0x4.0#1", Nearest, "2.0", "0x2.0#1", Greater);
    test("5.0", "0x5.0#3", Floor, "3.0", "0x3.0#3", Less);
    test("5.0", "0x5.0#3", Ceiling, "3.5", "0x3.8#3", Greater);
    test("5.0", "0x5.0#3", Down, "3.0", "0x3.0#3", Less);
    test("5.0", "0x5.0#3", Up, "3.5", "0x3.8#3", Greater);
    test("5.0", "0x5.0#3", Nearest, "3.0", "0x3.0#3", Less);
    test("10.0", "0xa.0#3", Floor, "12.0", "0xc.0#3", Less);
    test("10.0", "0xa.0#3", Ceiling, "14.0", "0xe.0#3", Greater);
    test("10.0", "0xa.0#3", Down, "12.0", "0xc.0#3", Less);
    test("10.0", "0xa.0#3", Up, "14.0", "0xe.0#3", Greater);
    test("10.0", "0xa.0#3", Nearest, "12.0", "0xc.0#3", Less);
    test("25.0", "0x19.0#5", Floor, "54.0", "0x36.0#5", Less);
    test("25.0", "0x19.0#5", Ceiling, "56.0", "0x38.0#5", Greater);
    test("25.0", "0x19.0#5", Down, "54.0", "0x36.0#5", Less);
    test("25.0", "0x19.0#5", Up, "56.0", "0x38.0#5", Greater);
    test("25.0", "0x19.0#5", Nearest, "54.0", "0x36.0#5", Less);
    test("171.0", "0xab.0#8", Floor, "704.0", "0x2c0.0#8", Less);
    test("171.0", "0xab.0#8", Ceiling, "708.0", "0x2c4.0#8", Greater);
    test("171.0", "0xab.0#8", Down, "704.0", "0x2c0.0#8", Less);
    test("171.0", "0xab.0#8", Up, "708.0", "0x2c4.0#8", Greater);
    test("171.0", "0xab.0#8", Nearest, "708.0", "0x2c4.0#8", Greater);
    test("0.50", "0x0.8#1", Floor, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Down, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Up, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Nearest, "0.50", "0x0.8#1", Less);
    test("1.5", "0x1.8#2", Floor, "-0.12", "-0x0.2#2", Less);
    test("1.5", "0x1.8#2", Ceiling, "-0.094", "-0x0.18#2", Greater);
    test("1.5", "0x1.8#2", Down, "-0.094", "-0x0.18#2", Greater);
    test("1.5", "0x1.8#2", Up, "-0.12", "-0x0.2#2", Less);
    test("1.5", "0x1.8#2", Nearest, "-0.12", "-0x0.2#2", Less);
    test("0.25", "0x0.4#1", Floor, "1.0", "0x1.0#1", Less);
    test("0.25", "0x0.4#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.25", "0x0.4#1", Down, "1.0", "0x1.0#1", Less);
    test("0.25", "0x0.4#1", Up, "2.0", "0x2.0#1", Greater);
    test("0.25", "0x0.4#1", Nearest, "1.0", "0x1.0#1", Less);
    test("0.75", "0x0.c#2", Floor, "0.19", "0x0.3#2", Less);
    test("0.75", "0x0.c#2", Ceiling, "0.25", "0x0.4#2", Greater);
    test("0.75", "0x0.c#2", Down, "0.19", "0x0.3#2", Less);
    test("0.75", "0x0.c#2", Up, "0.25", "0x0.4#2", Greater);
    test("0.75", "0x0.c#2", Nearest, "0.19", "0x0.3#2", Less);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Floor,
        "2.2527126517342051",
        "0x2.40b1c6be7bdd8#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Ceiling,
        "2.2527126517342060",
        "0x2.40b1c6be7bddc#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Down,
        "2.2527126517342051",
        "0x2.40b1c6be7bdd8#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Up,
        "2.2527126517342060",
        "0x2.40b1c6be7bddc#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Nearest,
        "2.2527126517342060",
        "0x2.40b1c6be7bddc#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Floor,
        "0.98709857789473432",
        "0x0.fcb27e0dfc6a0#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Ceiling,
        "0.98709857789473454",
        "0x0.fcb27e0dfc6a1#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Down,
        "0.98709857789473432",
        "0x0.fcb27e0dfc6a0#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Up,
        "0.98709857789473454",
        "0x0.fcb27e0dfc6a1#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Nearest,
        "0.98709857789473432",
        "0x0.fcb27e0dfc6a0#52",
        Less,
    );
    test("-0.50", "-0x0.8#1", Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", Down, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", Up, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", Exact, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", Down, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", Up, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", Exact, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", Floor, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", Ceiling, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", Down, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", Up, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", Nearest, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", Exact, "NaN", "NaN", Equal);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("-1.0", "-0x1.0#1", Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Down, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Up, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Exact, "NaN", "NaN", Equal);
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Floor,
        "23.025850929882733",
        "0x17.069e2aa26ae3#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Ceiling,
        "23.025850929882736",
        "0x17.069e2aa26ae4#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Down,
        "23.025850929882733",
        "0x17.069e2aa26ae3#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Up,
        "23.025850929882736",
        "0x17.069e2aa26ae4#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Nearest,
        "23.025850929882736",
        "0x17.069e2aa26ae4#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Floor,
        "690.77552789821368",
        "0x2b2.c688ff0ff6a#53",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Ceiling,
        "690.77552789821380",
        "0x2b2.c688ff0ff6c#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Down,
        "690.77552789821368",
        "0x2b2.c688ff0ff6a#53",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Up,
        "690.77552789821380",
        "0x2b2.c688ff0ff6c#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Nearest,
        "690.77552789821368",
        "0x2b2.c688ff0ff6a#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
#[should_panic]
fn ln_gamma_round_fail() {
    Float::from(0.5).ln_gamma_round(Exact);
}

#[test]
fn test_ln_gamma_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln_gamma, o) = x.clone().ln_gamma_prec_round(prec, rm);
        assert!(ln_gamma.is_valid());

        assert_eq!(ln_gamma.to_string(), out);
        assert_eq!(to_hex_string(&ln_gamma), out_hex);
        assert_eq!(o, o_out);

        let (ln_gamma_alt, o_alt) = x.ln_gamma_prec_round_ref(prec, rm);
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut ln_gamma_alt = x.clone();
        let o_alt = ln_gamma_alt.ln_gamma_prec_round_assign(prec, rm);
        assert!(ln_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_gamma),
            ComparableFloatRef(&ln_gamma_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    // The expected values agree with Azurite's logarithm of the gamma function.
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Exact, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Down, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Up, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Exact, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Down, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Up, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Up, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 10, Floor, "0.69238", "0x0.b14#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        10,
        Ceiling,
        "0.69336",
        "0x0.b18#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 10, Down, "0.69238", "0x0.b14#10", Less);
    test("3.0", "0x3.0#2", 10, Up, "0.69336", "0x0.b18#10", Greater);
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "0.69336",
        "0x0.b18#10",
        Greater,
    );
    test("4.0", "0x4.0#1", 10, Floor, "1.7910", "0x1.ca8#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        10,
        Ceiling,
        "1.7930",
        "0x1.cb0#10",
        Greater,
    );
    test("4.0", "0x4.0#1", 10, Down, "1.7910", "0x1.ca8#10", Less);
    test("4.0", "0x4.0#1", 10, Up, "1.7930", "0x1.cb0#10", Greater);
    test("4.0", "0x4.0#1", 10, Nearest, "1.7910", "0x1.ca8#10", Less);
    test("5.0", "0x5.0#3", 10, Floor, "3.1758", "0x3.2d#10", Less);
    test(
        "5.0",
        "0x5.0#3",
        10,
        Ceiling,
        "3.1797",
        "0x3.2e#10",
        Greater,
    );
    test("5.0", "0x5.0#3", 10, Down, "3.1758", "0x3.2d#10", Less);
    test("5.0", "0x5.0#3", 10, Up, "3.1797", "0x3.2e#10", Greater);
    test(
        "5.0",
        "0x5.0#3",
        10,
        Nearest,
        "3.1797",
        "0x3.2e#10",
        Greater,
    );
    test("10.0", "0xa.0#3", 10, Floor, "12.797", "0xc.cc#10", Less);
    test(
        "10.0",
        "0xa.0#3",
        10,
        Ceiling,
        "12.812",
        "0xc.d0#10",
        Greater,
    );
    test("10.0", "0xa.0#3", 10, Down, "12.797", "0xc.cc#10", Less);
    test("10.0", "0xa.0#3", 10, Up, "12.812", "0xc.d0#10", Greater);
    test("10.0", "0xa.0#3", 10, Nearest, "12.797", "0xc.cc#10", Less);
    test("25.0", "0x19.0#5", 10, Floor, "54.750", "0x36.c#10", Less);
    test(
        "25.0",
        "0x19.0#5",
        10,
        Ceiling,
        "54.812",
        "0x36.d#10",
        Greater,
    );
    test("25.0", "0x19.0#5", 10, Down, "54.750", "0x36.c#10", Less);
    test("25.0", "0x19.0#5", 10, Up, "54.812", "0x36.d#10", Greater);
    test(
        "25.0",
        "0x19.0#5",
        10,
        Nearest,
        "54.812",
        "0x36.d#10",
        Greater,
    );
    test("171.0", "0xab.0#8", 10, Floor, "706.00", "0x2c2.0#10", Less);
    test(
        "171.0",
        "0xab.0#8",
        10,
        Ceiling,
        "707.00",
        "0x2c3.0#10",
        Greater,
    );
    test("171.0", "0xab.0#8", 10, Down, "706.00", "0x2c2.0#10", Less);
    test("171.0", "0xab.0#8", 10, Up, "707.00", "0x2c3.0#10", Greater);
    test(
        "171.0",
        "0xab.0#8",
        10,
        Nearest,
        "707.00",
        "0x2c3.0#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Floor, "0.57227", "0x0.928#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.57324",
        "0x0.92c#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Down, "0.57227", "0x0.928#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.57324", "0x0.92c#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.57227",
        "0x0.928#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Floor,
        "-0.12085",
        "-0x0.1ef0#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "-0.12073",
        "-0x0.1ee8#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Down,
        "-0.12073",
        "-0x0.1ee8#10",
        Greater,
    );
    test("1.5", "0x1.8#2", 10, Up, "-0.12085", "-0x0.1ef0#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "-0.12073",
        "-0x0.1ee8#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Floor, "1.2871", "0x1.498#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "1.2891",
        "0x1.4a0#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Down, "1.2871", "0x1.498#10", Less);
    test("0.25", "0x0.4#1", 10, Up, "1.2891", "0x1.4a0#10", Greater);
    test("0.25", "0x0.4#1", 10, Nearest, "1.2871", "0x1.498#10", Less);
    test("0.75", "0x0.c#2", 10, Floor, "0.20312", "0x0.340#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "0.20337",
        "0x0.341#10",
        Greater,
    );
    test("0.75", "0x0.c#2", 10, Down, "0.20312", "0x0.340#10", Less);
    test("0.75", "0x0.c#2", 10, Up, "0.20337", "0x0.341#10", Greater);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Nearest,
        "0.20337",
        "0x0.341#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "2.2500",
        "0x2.40#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "2.2539",
        "0x2.41#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Down,
        "2.2500",
        "0x2.40#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Up,
        "2.2539",
        "0x2.41#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "2.2539",
        "0x2.41#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Floor,
        "0.98633",
        "0x0.fc8#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Ceiling,
        "0.98730",
        "0x0.fcc#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Down,
        "0.98633",
        "0x0.fc8#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Up,
        "0.98730",
        "0x0.fcc#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Nearest,
        "0.98730",
        "0x0.fcc#10",
        Greater,
    );
    test("-0.50", "-0x0.8#1", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Down, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Up, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Exact, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Down, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Up, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Exact, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, Floor, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, Down, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, Up, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.5", "-0x2.8#3", 10, Exact, "NaN", "NaN", Equal);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Down, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Up, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Exact, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, Floor, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, Down, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, Up, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, Nearest, "NaN", "NaN", Equal);
    test("-100.5", "-0x64.8#8", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Floor,
        "-5.7742e-8",
        "-0xf.80E-7#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Ceiling,
        "-5.7684e-8",
        "-0xf.7cE-7#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Down,
        "-5.7684e-8",
        "-0xf.7cE-7#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Up,
        "-5.7742e-8",
        "-0xf.80E-7#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Nearest,
        "-5.7742e-8",
        "-0xf.80E-7#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Floor,
        "-4.2317e-8",
        "-0xb.5cE-7#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Ceiling,
        "-4.2259e-8",
        "-0xb.58E-7#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Down,
        "-4.2259e-8",
        "-0xb.58E-7#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Up,
        "-4.2317e-8",
        "-0xb.5cE-7#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Nearest,
        "-4.2259e-8",
        "-0xb.58E-7#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4570247382208006",
        "-0x2.74ff92c01f0d8#51",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.7476826467274127",
        "-0x2.bf6821437b202#53",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Floor,
        "0.82715",
        "0x0.d3c#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Ceiling,
        "0.82812",
        "0x0.d40#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Down,
        "0.82715",
        "0x0.d3c#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Up,
        "0.82812",
        "0x0.d40#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Nearest,
        "0.82812",
        "0x0.d40#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "0.98535",
        "0x0.fc4#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "0.98633",
        "0x0.fc8#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Down,
        "0.98535",
        "0x0.fc4#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Up,
        "0.98633",
        "0x0.fc8#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "0.98535",
        "0x0.fc4#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "23.000",
        "0x17.00#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "23.031",
        "0x17.08#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "23.000",
        "0x17.00#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "23.031",
        "0x17.08#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "23.031",
        "0x17.08#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Floor,
        "690.00",
        "0x2b2.0#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Ceiling,
        "691.00",
        "0x2b3.0#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Down,
        "690.00",
        "0x2b2.0#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Up,
        "691.00",
        "0x2b3.0#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Nearest,
        "691.00",
        "0x2b3.0#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Floor,
        "5904.0",
        "0x1710.0#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Ceiling,
        "5912.0",
        "0x1718.0#10",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Down,
        "5904.0",
        "0x1710.0#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Up,
        "5912.0",
        "0x1718.0#10",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Nearest,
        "5912.0",
        "0x1718.0#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Floor,
        "1.0506e6",
        "0x1.008E+5#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Ceiling,
        "1.0527e6",
        "0x1.010E+5#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Down,
        "1.0506e6",
        "0x1.008E+5#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Up,
        "1.0527e6",
        "0x1.010E+5#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Nearest,
        "1.0506e6",
        "0x1.008E+5#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Floor,
        "1.2812e7",
        "0xc.38E+5#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Ceiling,
        "1.2829e7",
        "0xc.3cE+5#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Down,
        "1.2812e7",
        "0xc.38E+5#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Up,
        "1.2829e7",
        "0xc.3cE+5#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Nearest,
        "1.2812e7",
        "0xc.38E+5#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test("-1000000.5", "-0xf4240.8#21", 10, Down, "NaN", "NaN", Equal);
    test("-1000000.5", "-0xf4240.8#21", 10, Up, "NaN", "NaN", Equal);
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Floor,
        "1.1409e9",
        "0x4.40E+7#10",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Ceiling,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Down,
        "1.1409e9",
        "0x4.40E+7#10",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Up,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Nearest,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Floor,
        "1.1409e9",
        "0x4.40E+7#10",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Ceiling,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Down,
        "1.1409e9",
        "0x4.40E+7#10",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Up,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Nearest,
        "1.1429e9",
        "0x4.42E+7#10",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Floor,
        "6.8844e302",
        "0x4.04E+251#10",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Ceiling,
        "6.8978e302",
        "0x4.06E+251#10",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Down,
        "6.8844e302",
        "0x4.04E+251#10",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Up,
        "6.8978e302",
        "0x4.06E+251#10",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Nearest,
        "6.8978e302",
        "0x4.06E+251#10",
        Greater,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Floor,
        "6.9222e5",
        "0xa.90E+4#10",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Ceiling,
        "6.9325e5",
        "0xa.94E+4#10",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Down,
        "6.9222e5",
        "0xa.90E+4#10",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Up,
        "6.9325e5",
        "0xa.94E+4#10",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Nearest,
        "6.9325e5",
        "0xa.94E+4#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Down,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Up,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        Floor,
        "7.6154e161614256",
        "0x1.628E+134217735#10",
        Less,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        Ceiling,
        "7.6262e161614256",
        "0x1.630E+134217735#10",
        Greater,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        Down,
        "7.6154e161614256",
        "0x1.628E+134217735#10",
        Less,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        Up,
        "7.6262e161614256",
        "0x1.630E+134217735#10",
        Greater,
    );
    test(
        "2.0e161614248",
        "0x1.0E+134217728#1",
        10,
        Nearest,
        "7.6262e161614256",
        "0x1.630E+134217735#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        10,
        Floor,
        "-3.5977e-61",
        "-0x9.40E-51#10",
        Less,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        10,
        Ceiling,
        "-3.5916e-61",
        "-0x9.3cE-51#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        10,
        Down,
        "-3.5916e-61",
        "-0x9.3cE-51#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        10,
        Up,
        "-3.5977e-61",
        "-0x9.40E-51#10",
        Less,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000000000000000000000001#201",
        10,
        Nearest,
        "-3.5916e-61",
        "-0x9.3cE-51#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, Down, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Up, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, Up, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 100, Exact, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Floor,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Ceiling,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test("3.0", "0x3.0#2", 1, Down, "0.50", "0x0.8#1", Less);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Down,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Up, "1.0", "0x1.0#1", Greater);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Up,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Nearest,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test("4.0", "0x4.0#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Floor,
        "1.7917594692280550008124773583802",
        "0x1.cab0bfa2a20023226dfd40ed0#100",
        Less,
    );
    test("4.0", "0x4.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Ceiling,
        "1.7917594692280550008124773583818",
        "0x1.cab0bfa2a20023226dfd40ed2#100",
        Greater,
    );
    test("4.0", "0x4.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Down,
        "1.7917594692280550008124773583802",
        "0x1.cab0bfa2a20023226dfd40ed0#100",
        Less,
    );
    test("4.0", "0x4.0#1", 1, Up, "2.0", "0x2.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Up,
        "1.7917594692280550008124773583818",
        "0x1.cab0bfa2a20023226dfd40ed2#100",
        Greater,
    );
    test("4.0", "0x4.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "1.7917594692280550008124773583802",
        "0x1.cab0bfa2a20023226dfd40ed0#100",
        Less,
    );
    test("5.0", "0x5.0#3", 1, Floor, "2.0", "0x2.0#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Floor,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test("5.0", "0x5.0#3", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Ceiling,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4#100",
        Greater,
    );
    test("5.0", "0x5.0#3", 1, Down, "2.0", "0x2.0#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Down,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test("5.0", "0x5.0#3", 1, Up, "4.0", "0x4.0#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Up,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4#100",
        Greater,
    );
    test("5.0", "0x5.0#3", 1, Nearest, "4.0", "0x4.0#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Nearest,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test("10.0", "0xa.0#3", 1, Floor, "8.0", "0x8.0#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Floor,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61#100",
        Less,
    );
    test("10.0", "0xa.0#3", 1, Ceiling, "16.0", "0x1.0E+1#1", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Ceiling,
        "12.801827480081469611207717874575",
        "0xc.cd4490d3fbe7a583ebac5a62#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, Down, "8.0", "0x8.0#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Down,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61#100",
        Less,
    );
    test("10.0", "0xa.0#3", 1, Up, "16.0", "0x1.0E+1#1", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Up,
        "12.801827480081469611207717874575",
        "0xc.cd4490d3fbe7a583ebac5a62#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, Nearest, "16.0", "0x1.0E+1#1", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Nearest,
        "12.801827480081469611207717874562",
        "0xc.cd4490d3fbe7a583ebac5a61#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Down, "0.50", "0x0.8#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Down,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Up, "1.0", "0x1.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Up,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Floor, "-0.12", "-0x0.2#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Floor,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Ceiling, "-0.062", "-0x0.1#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Ceiling,
        "-0.12078223763524522234551844578156",
        "-0x0.1eeb95b094c1914d1a18fd62c0#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Down, "-0.062", "-0x0.1#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Down,
        "-0.12078223763524522234551844578156",
        "-0x0.1eeb95b094c1914d1a18fd62c0#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Up, "-0.12", "-0x0.2#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Up,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Nearest, "-0.12", "-0x0.2#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Nearest,
        "-0.12078223763524522234551844578166",
        "-0x0.1eeb95b094c1914d1a18fd62c2#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Floor,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Ceiling,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Down,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Up, "2.0", "0x2.0#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Up,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Nearest,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Floor, "0.12", "0x0.2#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Floor,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Ceiling,
        "0.20328095143129537148143297186260",
        "0x0.340a386e4c13dbb02d73eb7d88#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Down, "0.12", "0x0.2#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Down,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Up, "0.25", "0x0.4#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Up,
        "0.20328095143129537148143297186260",
        "0x0.340a386e4c13dbb02d73eb7d88#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Nearest, "0.25", "0x0.4#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Nearest,
        "0.20328095143129537148143297186241",
        "0x0.340a386e4c13dbb02d73eb7d84#100",
        Less,
    );
}

#[test]
#[should_panic]
fn ln_gamma_prec_round_fail_1() {
    Float::ONE.ln_gamma_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_gamma_prec_round_fail_2() {
    Float::from(0.5).ln_gamma_prec_round(10, Exact);
}

// Checks a result against MPFR's `mpfr_lngamma`, which differs for a negative argument: it is the
// logarithm of the gamma function wherever that function is positive.
fn check_rounding(x: &Float, ln_gamma: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && (x.is_nan() || *x >= 0u32)
    {
        let (rug_ln_gamma, rug_o) =
            rug_ln_gamma_prec_round(&rug::Float::exact_from(x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_gamma)),
            ComparableFloatRef(ln_gamma)
        );
        assert_eq!(rug_o, o);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn ln_gamma_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (ln_gamma, o) = x.clone().ln_gamma_prec_round(prec, rm);
    assert!(ln_gamma.is_valid());
    assert_rounding_ordering_consistent(&ln_gamma, rm, o);

    let (ln_gamma_alt, o_alt) = x.ln_gamma_prec_round_ref(prec, rm);
    assert!(ln_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_gamma_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_gamma));
    assert_eq!(o_alt, o);

    check_rounding(&x, &ln_gamma, o, prec, rm);

    if ln_gamma.is_normal() {
        assert_eq!(ln_gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The function is negative between its zeros 1 and 2, and positive on the rest of the
        // positive reals; a result next to a zero may underflow.
        if x > 1u32 && x < 2u32 {
            assert!(ln_gamma <= 0u32);
        } else {
            assert!(ln_gamma >= 0u32);
        }
    } else if x < 0u32 {
        assert!(ln_gamma.is_nan());
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ln_gamma_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(ln_gamma.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ln_gamma_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) = Float::ln_gamma_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&ln_gamma));
        assert_eq!(ro, o);
    }
}

#[test]
fn ln_gamma_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        ln_gamma_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            ln_gamma_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.ln_gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.ln_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.ln_gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.ln_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.ln_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        // The function is not defined at negative arguments.
        let (s, o) = Float::NEGATIVE_ONE.ln_gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // The zeros of the function
        let (s, o) = Float::ONE.ln_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::TWO.ln_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_gamma_prec_properties_helper(x: Float, prec: u64) {
    let (ln_gamma, o) = x.clone().ln_gamma_prec(prec);
    assert!(ln_gamma.is_valid());

    let (ln_gamma_alt, o_alt) = x.ln_gamma_prec_ref(prec);
    assert!(ln_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_gamma_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_gamma));
    assert_eq!(o_alt, o);

    let (ln_gamma_alt, o_alt) = x.ln_gamma_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &ln_gamma, o, prec, Nearest);

    if ln_gamma.is_normal() {
        assert_eq!(ln_gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The function is negative between its zeros 1 and 2, and positive on the rest of the
        // positive reals; a result next to a zero may underflow.
        if x > 1u32 && x < 2u32 {
            assert!(ln_gamma <= 0u32);
        } else {
            assert!(ln_gamma >= 0u32);
        }
    } else if x < 0u32 {
        assert!(ln_gamma.is_nan());
    }
}

#[test]
fn ln_gamma_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        ln_gamma_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        ln_gamma_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        ln_gamma_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.ln_gamma_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_gamma_round_properties_helper(x: Float, rm: RoundingMode) {
    let (ln_gamma, o) = x.clone().ln_gamma_round(rm);
    assert!(ln_gamma.is_valid());
    assert_rounding_ordering_consistent(&ln_gamma, rm, o);

    let (ln_gamma_alt, o_alt) = x.ln_gamma_round_ref(rm);
    assert!(ln_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_gamma_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_gamma));
    assert_eq!(o_alt, o);

    let (ln_gamma_alt, o_alt) = x.ln_gamma_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &ln_gamma, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ln_gamma_round_ref(rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(ln_gamma.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ln_gamma_round_ref(Exact));
    }
}

#[test]
fn ln_gamma_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        ln_gamma_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_47().test_properties_with_config(&config, |(x, rm)| {
        ln_gamma_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_gamma_properties_helper(x: Float) {
    let ln_gamma = x.clone().ln_gamma();
    assert!(ln_gamma.is_valid());

    let ln_gamma_alt = (&x).ln_gamma();
    assert!(ln_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );

    let mut x_alt = x.clone();
    x_alt.ln_gamma_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_gamma));

    let ln_gamma_alt = x.ln_gamma_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    let ln_gamma_alt = x.ln_gamma_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    let ln_gamma_alt = x.ln_gamma_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    if x > 0u32 {
        // The function is negative between its zeros 1 and 2, and positive on the rest of the
        // positive reals; a result next to a zero may underflow.
        if x > 1u32 && x < 2u32 {
            assert!(ln_gamma <= 0u32);
        } else {
            assert!(ln_gamma >= 0u32);
        }
    } else if x < 0u32 {
        assert!(ln_gamma.is_nan());
    }
}

#[test]
fn ln_gamma_properties() {
    float_gen().test_properties(|x| {
        ln_gamma_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        ln_gamma_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        ln_gamma_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_ln_gamma() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_ln_gamma(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::INFINITY);
    test::<f32>(1.0, 0.0);
    test::<f32>(2.0, 0.0);
    test::<f32>(5.0, 3.1780539);
    test::<f32>(0.5, 0.5723649);
    test::<f32>(-0.5, f32::NAN);
    test::<f32>(-1.0, f32::NAN);
    test::<f32>(1.5, -0.12078224);
    test::<f32>(3.3, 0.9870985);
    test::<f32>(-3.3, f32::NAN);
    test::<f32>(35.0, 88.580826);
    test::<f32>(36.0, 92.13618);
    test::<f32>(171.0, 706.57306);
    test::<f32>(172.0, 711.7147);
    test::<f32>(1.0e-30, 69.07755);
    test::<f32>(-1.0e-30, f32::NAN);
    test::<f32>(1.0e-40, 92.10341);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::INFINITY);
    test::<f64>(1.0, 0.0);
    test::<f64>(2.0, 0.0);
    test::<f64>(5.0, 3.1780538303479458);
    test::<f64>(0.5, 0.5723649429247001);
    test::<f64>(-0.5, f64::NAN);
    test::<f64>(-1.0, f64::NAN);
    test::<f64>(1.5, -0.12078223763524522);
    test::<f64>(3.3, 0.9870985778947344);
    test::<f64>(-3.3, f64::NAN);
    test::<f64>(35.0, 88.58082754219768);
    test::<f64>(36.0, 92.1361756036871);
    test::<f64>(171.0, 706.5730622457874);
    test::<f64>(172.0, 711.71472580229);
    test::<f64>(1.0e-30, 69.07755278982137);
    test::<f64>(-1.0e-30, f64::NAN);
    test::<f64>(1.0e-40, 92.10340371976183);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_ln_gamma_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_ln_gamma(x);
    });
}

#[test]
fn primitive_float_ln_gamma_properties() {
    apply_fn_to_primitive_floats!(primitive_float_ln_gamma_properties_helper);
}

#[test]
fn test_ln_gamma_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::ln_gamma_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::ln_gamma_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "Infinity", "Infinity", Equal);
    test("0", 10, "Infinity", "Infinity", Equal);
    test("0", 100, "Infinity", "Infinity", Equal);
    test("1", 1, "0.0", "0x0.0", Equal);
    test("1", 10, "0.0", "0x0.0", Equal);
    test("1", 100, "0.0", "0x0.0", Equal);
    test("2", 1, "0.0", "0x0.0", Equal);
    test("2", 10, "0.0", "0x0.0", Equal);
    test("2", 100, "0.0", "0x0.0", Equal);
    test("3", 1, "0.50", "0x0.8#1", Less);
    test("3", 10, "0.69336", "0x0.b18#10", Greater);
    test(
        "3",
        100,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test("5", 1, "4.0", "0x4.0#1", Greater);
    test("5", 10, "3.1797", "0x3.2e#10", Greater);
    test(
        "5",
        100,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test("25", 1, "64.0", "0x4.0E+1#1", Greater);
    test("25", 10, "54.812", "0x36.d#10", Greater);
    test(
        "25",
        100,
        "54.784729398112319190093344083609",
        "0x36.c8e4069d1a2824fcb11da738#100",
        Greater,
    );
    test("1/2", 1, "0.50", "0x0.8#1", Less);
    test("1/2", 10, "0.57227", "0x0.928#10", Less);
    test(
        "1/2",
        100,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test("1/3", 1, "1.0", "0x1.0#1", Greater);
    test("1/3", 10, "0.98535", "0x0.fc4#10", Less);
    test(
        "1/3",
        100,
        "0.98542064692776706918717403697768",
        "0x0.fc44870b5ba0ce071ed58f220#100",
        Less,
    );
    test("1/4", 1, "1.0", "0x1.0#1", Less);
    test("1/4", 10, "1.2871", "0x1.498#10", Less);
    test(
        "1/4",
        100,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test("1/6", 1, "2.0", "0x2.0#1", Greater);
    test("1/6", 10, "1.7168", "0x1.b78#10", Greater);
    test(
        "1/6",
        100,
        "1.7167334350782404605278463095874",
        "0x1.b77bd7a79c5c35319ca148314#100",
        Less,
    );
    test("2/3", 1, "0.25", "0x0.4#1", Less);
    test("2/3", 10, "0.30322", "0x0.4da#10", Greater);
    test(
        "2/3",
        100,
        "0.30315027514752356867586281737202",
        "0x0.4d9b41a5883227a6b896ca3600#100",
        Greater,
    );
    test("-1/2", 1, "NaN", "NaN", Equal);
    test("-1/2", 10, "NaN", "NaN", Equal);
    test("-1/2", 100, "NaN", "NaN", Equal);
    test("-1", 1, "NaN", "NaN", Equal);
    test("-1", 10, "NaN", "NaN", Equal);
    test("-2", 1, "NaN", "NaN", Equal);
    test("-2", 10, "NaN", "NaN", Equal);
    test("-5/2", 1, "NaN", "NaN", Equal);
    test("-5/2", 10, "NaN", "NaN", Equal);
    test("-7/3", 1, "NaN", "NaN", Equal);
    test("-7/3", 10, "NaN", "NaN", Equal);
    test("22/7", 1, "1.0", "0x1.0#1", Greater);
    test("22/7", 10, "0.82910", "0x0.d44#10", Greater);
    test("1000001/1000", 1, "4.1e3", "0x1.0E+3#1", Less);
    test("1000001/1000", 10, "5904.0", "0x1710.0#10", Less);
    test("-1000001/1000", 1, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, "NaN", "NaN", Equal);
    test(
        "1/1267650600228229401496703205376",
        1,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        "69.375",
        "0x45.6#10",
        Greater,
    );
    test("-1/1267650600228229401496703205376", 1, "NaN", "NaN", Equal);
    test(
        "-1/1267650600228229401496703205376",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        1,
        "-3.9e-31",
        "-0x8.0E-26#1",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        "-4.5529e-31",
        "-0x9.3cE-26#10",
        Greater,
    );
    test("100000000", 1, "2.1e9", "0x8.0E+7#1", Greater);
    test("100000000", 10, "1.7427e9", "0x6.7eE+7#10", Greater);
    test("200000001/2", 1, "2.1e9", "0x8.0E+7#1", Greater);
    test("200000001/2", 10, "1.7427e9", "0x6.7eE+7#10", Greater);
    test("-200000001/2", 1, "NaN", "NaN", Equal);
    test("-200000001/2", 10, "NaN", "NaN", Equal);
    test("-200000003/2", 1, "NaN", "NaN", Equal);
    test("-200000003/2", 10, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_fail() {
    Float::ln_gamma_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_ref_fail() {
    Float::ln_gamma_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_ln_gamma_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::ln_gamma_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::ln_gamma_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Down, "0.0", "0x0.0", Equal);
    test("1", 10, Up, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("2", 10, Floor, "0.0", "0x0.0", Equal);
    test("2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2", 10, Down, "0.0", "0x0.0", Equal);
    test("2", 10, Up, "0.0", "0x0.0", Equal);
    test("2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("2", 10, Exact, "0.0", "0x0.0", Equal);
    test("3", 10, Floor, "0.69238", "0x0.b14#10", Less);
    test("3", 10, Ceiling, "0.69336", "0x0.b18#10", Greater);
    test("3", 10, Down, "0.69238", "0x0.b14#10", Less);
    test("3", 10, Up, "0.69336", "0x0.b18#10", Greater);
    test("3", 10, Nearest, "0.69336", "0x0.b18#10", Greater);
    test("5", 10, Floor, "3.1758", "0x3.2d#10", Less);
    test("5", 10, Ceiling, "3.1797", "0x3.2e#10", Greater);
    test("5", 10, Down, "3.1758", "0x3.2d#10", Less);
    test("5", 10, Up, "3.1797", "0x3.2e#10", Greater);
    test("5", 10, Nearest, "3.1797", "0x3.2e#10", Greater);
    test("25", 10, Floor, "54.750", "0x36.c#10", Less);
    test("25", 10, Ceiling, "54.812", "0x36.d#10", Greater);
    test("25", 10, Down, "54.750", "0x36.c#10", Less);
    test("25", 10, Up, "54.812", "0x36.d#10", Greater);
    test("25", 10, Nearest, "54.812", "0x36.d#10", Greater);
    test("1/2", 10, Floor, "0.57227", "0x0.928#10", Less);
    test("1/2", 10, Ceiling, "0.57324", "0x0.92c#10", Greater);
    test("1/2", 10, Down, "0.57227", "0x0.928#10", Less);
    test("1/2", 10, Up, "0.57324", "0x0.92c#10", Greater);
    test("1/2", 10, Nearest, "0.57227", "0x0.928#10", Less);
    test("1/3", 10, Floor, "0.98535", "0x0.fc4#10", Less);
    test("1/3", 10, Ceiling, "0.98633", "0x0.fc8#10", Greater);
    test("1/3", 10, Down, "0.98535", "0x0.fc4#10", Less);
    test("1/3", 10, Up, "0.98633", "0x0.fc8#10", Greater);
    test("1/3", 10, Nearest, "0.98535", "0x0.fc4#10", Less);
    test("1/4", 10, Floor, "1.2871", "0x1.498#10", Less);
    test("1/4", 10, Ceiling, "1.2891", "0x1.4a0#10", Greater);
    test("1/4", 10, Down, "1.2871", "0x1.498#10", Less);
    test("1/4", 10, Up, "1.2891", "0x1.4a0#10", Greater);
    test("1/4", 10, Nearest, "1.2871", "0x1.498#10", Less);
    test("1/6", 10, Floor, "1.7148", "0x1.b70#10", Less);
    test("1/6", 10, Ceiling, "1.7168", "0x1.b78#10", Greater);
    test("1/6", 10, Down, "1.7148", "0x1.b70#10", Less);
    test("1/6", 10, Up, "1.7168", "0x1.b78#10", Greater);
    test("1/6", 10, Nearest, "1.7168", "0x1.b78#10", Greater);
    test("2/3", 10, Floor, "0.30273", "0x0.4d8#10", Less);
    test("2/3", 10, Ceiling, "0.30322", "0x0.4da#10", Greater);
    test("2/3", 10, Down, "0.30273", "0x0.4d8#10", Less);
    test("2/3", 10, Up, "0.30322", "0x0.4da#10", Greater);
    test("2/3", 10, Nearest, "0.30322", "0x0.4da#10", Greater);
    test("-1/2", 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 10, Down, "NaN", "NaN", Equal);
    test("-1/2", 10, Up, "NaN", "NaN", Equal);
    test("-1/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 10, Exact, "NaN", "NaN", Equal);
    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Exact, "NaN", "NaN", Equal);
    test("-2", 10, Floor, "NaN", "NaN", Equal);
    test("-2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2", 10, Down, "NaN", "NaN", Equal);
    test("-2", 10, Up, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Exact, "NaN", "NaN", Equal);
    test("-5/2", 10, Floor, "NaN", "NaN", Equal);
    test("-5/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-5/2", 10, Down, "NaN", "NaN", Equal);
    test("-5/2", 10, Up, "NaN", "NaN", Equal);
    test("-5/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-5/2", 10, Exact, "NaN", "NaN", Equal);
    test("-7/3", 10, Floor, "NaN", "NaN", Equal);
    test("-7/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-7/3", 10, Down, "NaN", "NaN", Equal);
    test("-7/3", 10, Up, "NaN", "NaN", Equal);
    test("-7/3", 10, Nearest, "NaN", "NaN", Equal);
    test("-7/3", 10, Exact, "NaN", "NaN", Equal);
    test("22/7", 10, Floor, "0.82812", "0x0.d40#10", Less);
    test("22/7", 10, Ceiling, "0.82910", "0x0.d44#10", Greater);
    test("22/7", 10, Down, "0.82812", "0x0.d40#10", Less);
    test("22/7", 10, Up, "0.82910", "0x0.d44#10", Greater);
    test("22/7", 10, Nearest, "0.82910", "0x0.d44#10", Greater);
    test("1000001/1000", 10, Floor, "5904.0", "0x1710.0#10", Less);
    test(
        "1000001/1000",
        10,
        Ceiling,
        "5912.0",
        "0x1718.0#10",
        Greater,
    );
    test("1000001/1000", 10, Down, "5904.0", "0x1710.0#10", Less);
    test("1000001/1000", 10, Up, "5912.0", "0x1718.0#10", Greater);
    test("1000001/1000", 10, Nearest, "5904.0", "0x1710.0#10", Less);
    test("-1000001/1000", 10, Floor, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Down, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Up, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Nearest, "NaN", "NaN", Equal);
    test("-1000001/1000", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "69.250",
        "0x45.4#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "69.375",
        "0x45.6#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "69.250",
        "0x45.4#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "69.375",
        "0x45.6#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "69.375",
        "0x45.6#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "-4.5606e-31",
        "-0x9.40E-26#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "-4.5529e-31",
        "-0x9.3cE-26#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Down,
        "-4.5529e-31",
        "-0x9.3cE-26#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Up,
        "-4.5606e-31",
        "-0x9.40E-26#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "-4.5529e-31",
        "-0x9.3cE-26#10",
        Greater,
    );
    test("100000000", 10, Floor, "1.7406e9", "0x6.7cE+7#10", Less);
    test(
        "100000000",
        10,
        Ceiling,
        "1.7427e9",
        "0x6.7eE+7#10",
        Greater,
    );
    test("100000000", 10, Down, "1.7406e9", "0x6.7cE+7#10", Less);
    test("100000000", 10, Up, "1.7427e9", "0x6.7eE+7#10", Greater);
    test(
        "100000000",
        10,
        Nearest,
        "1.7427e9",
        "0x6.7eE+7#10",
        Greater,
    );
    test("200000001/2", 10, Floor, "1.7406e9", "0x6.7cE+7#10", Less);
    test(
        "200000001/2",
        10,
        Ceiling,
        "1.7427e9",
        "0x6.7eE+7#10",
        Greater,
    );
    test("200000001/2", 10, Down, "1.7406e9", "0x6.7cE+7#10", Less);
    test("200000001/2", 10, Up, "1.7427e9", "0x6.7eE+7#10", Greater);
    test(
        "200000001/2",
        10,
        Nearest,
        "1.7427e9",
        "0x6.7eE+7#10",
        Greater,
    );
    test("-200000001/2", 10, Floor, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Down, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Up, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-200000001/2", 10, Exact, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Floor, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Down, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Up, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-200000003/2", 10, Exact, "NaN", "NaN", Equal);
    test("0", 100, Floor, "Infinity", "Infinity", Equal);
    test("0", 100, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 100, Down, "Infinity", "Infinity", Equal);
    test("0", 100, Up, "Infinity", "Infinity", Equal);
    test("0", 100, Nearest, "Infinity", "Infinity", Equal);
    test("0", 100, Exact, "Infinity", "Infinity", Equal);
    test("1", 100, Floor, "0.0", "0x0.0", Equal);
    test("1", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 100, Down, "0.0", "0x0.0", Equal);
    test("1", 100, Up, "0.0", "0x0.0", Equal);
    test("1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", 100, Exact, "0.0", "0x0.0", Equal);
    test("2", 100, Floor, "0.0", "0x0.0", Equal);
    test("2", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("2", 100, Down, "0.0", "0x0.0", Equal);
    test("2", 100, Up, "0.0", "0x0.0", Equal);
    test("2", 100, Nearest, "0.0", "0x0.0", Equal);
    test("2", 100, Exact, "0.0", "0x0.0", Equal);
    test(
        "3",
        100,
        Floor,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test(
        "3",
        100,
        Ceiling,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test(
        "3",
        100,
        Down,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test(
        "3",
        100,
        Up,
        "0.69314718055994530941723212145877",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "0.69314718055994530941723212145798",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test(
        "5",
        100,
        Floor,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test(
        "5",
        100,
        Ceiling,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4#100",
        Greater,
    );
    test(
        "5",
        100,
        Down,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test(
        "5",
        100,
        Up,
        "3.1780538303479456196469416012994",
        "0x3.2d94ef92459f167a01c4a81d4#100",
        Greater,
    );
    test(
        "5",
        100,
        Nearest,
        "3.1780538303479456196469416012962",
        "0x3.2d94ef92459f167a01c4a81d0#100",
        Less,
    );
    test(
        "25",
        100,
        Floor,
        "54.784729398112319190093344083558",
        "0x36.c8e4069d1a2824fcb11da734#100",
        Less,
    );
    test(
        "25",
        100,
        Ceiling,
        "54.784729398112319190093344083609",
        "0x36.c8e4069d1a2824fcb11da738#100",
        Greater,
    );
    test(
        "25",
        100,
        Down,
        "54.784729398112319190093344083558",
        "0x36.c8e4069d1a2824fcb11da734#100",
        Less,
    );
    test(
        "25",
        100,
        Up,
        "54.784729398112319190093344083609",
        "0x36.c8e4069d1a2824fcb11da738#100",
        Greater,
    );
    test(
        "25",
        100,
        Nearest,
        "54.784729398112319190093344083609",
        "0x36.c8e4069d1a2824fcb11da738#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Floor,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "0.57236494292470008707171367567721",
        "0x0.928682473d0de85eafcab6355#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.57236494292470008707171367567642",
        "0x0.928682473d0de85eafcab6354#100",
        Less,
    );
    test(
        "1/3",
        100,
        Floor,
        "0.98542064692776706918717403697768",
        "0x0.fc44870b5ba0ce071ed58f220#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.98542064692776706918717403697846",
        "0x0.fc44870b5ba0ce071ed58f221#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.98542064692776706918717403697768",
        "0x0.fc44870b5ba0ce071ed58f220#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.98542064692776706918717403697846",
        "0x0.fc44870b5ba0ce071ed58f221#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.98542064692776706918717403697768",
        "0x0.fc44870b5ba0ce071ed58f220#100",
        Less,
    );
    test(
        "1/4",
        100,
        Floor,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test(
        "1/4",
        100,
        Ceiling,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Down,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test(
        "1/4",
        100,
        Up,
        "1.2880225246980774573706104402212",
        "0x1.49bbd81c16efb1e317135ab92#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Nearest,
        "1.2880225246980774573706104402196",
        "0x1.49bbd81c16efb1e317135ab90#100",
        Less,
    );
    test(
        "1/6",
        100,
        Floor,
        "1.7167334350782404605278463095874",
        "0x1.b77bd7a79c5c35319ca148314#100",
        Less,
    );
    test(
        "1/6",
        100,
        Ceiling,
        "1.7167334350782404605278463095890",
        "0x1.b77bd7a79c5c35319ca148316#100",
        Greater,
    );
    test(
        "1/6",
        100,
        Down,
        "1.7167334350782404605278463095874",
        "0x1.b77bd7a79c5c35319ca148314#100",
        Less,
    );
    test(
        "1/6",
        100,
        Up,
        "1.7167334350782404605278463095890",
        "0x1.b77bd7a79c5c35319ca148316#100",
        Greater,
    );
    test(
        "1/6",
        100,
        Nearest,
        "1.7167334350782404605278463095874",
        "0x1.b77bd7a79c5c35319ca148314#100",
        Less,
    );
    test(
        "2/3",
        100,
        Floor,
        "0.30315027514752356867586281737163",
        "0x0.4d9b41a5883227a6b896ca35f8#100",
        Less,
    );
    test(
        "2/3",
        100,
        Ceiling,
        "0.30315027514752356867586281737202",
        "0x0.4d9b41a5883227a6b896ca3600#100",
        Greater,
    );
    test(
        "2/3",
        100,
        Down,
        "0.30315027514752356867586281737163",
        "0x0.4d9b41a5883227a6b896ca35f8#100",
        Less,
    );
    test(
        "2/3",
        100,
        Up,
        "0.30315027514752356867586281737202",
        "0x0.4d9b41a5883227a6b896ca3600#100",
        Greater,
    );
    test(
        "2/3",
        100,
        Nearest,
        "0.30315027514752356867586281737202",
        "0x0.4d9b41a5883227a6b896ca3600#100",
        Greater,
    );
    test("-1/2", 100, Floor, "NaN", "NaN", Equal);
    test("-1/2", 100, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 100, Down, "NaN", "NaN", Equal);
    test("-1/2", 100, Up, "NaN", "NaN", Equal);
    test("-1/2", 100, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 100, Exact, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_round_fail_1() {
    Float::ln_gamma_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_round_fail_2() {
    Float::ln_gamma_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn ln_gamma_rational_prec_round_ref_fail() {
    Float::ln_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn ln_gamma_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (ln_gamma, o) = Float::ln_gamma_rational_prec_round(x.clone(), prec, rm);
    assert!(ln_gamma.is_valid());
    assert_rounding_ordering_consistent(&ln_gamma, rm, o);

    let (ln_gamma_alt, o_alt) = Float::ln_gamma_rational_prec_round_ref(&x, prec, rm);
    assert!(ln_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    // MPFR's `mpfr_lngamma` at a `Float` near x; see `rug_ln_gamma_rational_prec_round`. The
    // rounding error of that `Float` matters less than the extra precision when x is not large and
    // not within 2^-64 of a zero of the function.
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && x.lt_abs(&0x10000u32)
        && x > 0u32
        && (&x - Rational::ONE).ge_abs(&Rational::power_of_2(-64i64))
        && (&x - Rational::TWO).ge_abs(&Rational::power_of_2(-64i64))
    {
        let (rug_ln_gamma, rug_o) = rug_ln_gamma_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_gamma)),
            ComparableFloatRef(&ln_gamma)
        );
        assert_eq!(rug_o, o);
    }

    if ln_gamma.is_normal() {
        assert_eq!(ln_gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The function is negative between its zeros 1 and 2, and positive on the rest of the
        // positive reals; a result next to a zero may underflow.
        if x > 1u32 && x < 2u32 {
            assert!(ln_gamma <= 0u32);
        } else {
            assert!(ln_gamma >= 0u32);
        }
    } else if x < 0u32 {
        assert!(ln_gamma.is_nan());
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::ln_gamma_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(ln_gamma.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::ln_gamma_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn ln_gamma_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        ln_gamma_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_gamma_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (ln_gamma, o) = Float::ln_gamma_rational_prec(x.clone(), prec);
    assert!(ln_gamma.is_valid());

    let (ln_gamma_alt, o_alt) = Float::ln_gamma_rational_prec_ref(&x, prec);
    assert!(ln_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);

    let (ln_gamma_alt, o_alt) = Float::ln_gamma_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&ln_gamma_alt),
        ComparableFloatRef(&ln_gamma)
    );
    assert_eq!(o_alt, o);
}

#[test]
fn ln_gamma_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        ln_gamma_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_ln_gamma_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_ln_gamma_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::INFINITY);
    test::<f32>("1", 0.0);
    test::<f32>("2", 0.0);
    test::<f32>("3", core::f32::consts::LN_2);
    test::<f32>("5", 3.1780539);
    test::<f32>("25", 54.78473);
    test::<f32>("1/2", 0.5723649);
    test::<f32>("1/3", 0.98542064);
    test::<f32>("1/4", 1.2880225);
    test::<f32>("1/6", 1.7167335);
    test::<f32>("2/3", 0.30315027);
    test::<f32>("-1/2", f32::NAN);
    test::<f32>("-1", f32::NAN);
    test::<f32>("-2", f32::NAN);
    test::<f32>("-5/2", f32::NAN);
    test::<f32>("-7/3", f32::NAN);
    test::<f32>("22/7", 0.82893056);
    test::<f32>("1000001/1000", 5905.2275);
    test::<f32>("-1000001/1000", f32::NAN);
    test::<f32>("1/1267650600228229401496703205376", 69.31472);
    test::<f32>("-1/1267650600228229401496703205376", f32::NAN);
    test::<f32>("35", 88.580826);
    test::<f32>("36", 92.13618);
    test::<f32>("171", 706.57306);
    test::<f32>("172", 711.7147);
    test::<f64>("0", f64::INFINITY);
    test::<f64>("1", 0.0);
    test::<f64>("2", 0.0);
    test::<f64>("3", core::f64::consts::LN_2);
    test::<f64>("5", 3.1780538303479458);
    test::<f64>("25", 54.78472939811232);
    test::<f64>("1/2", 0.5723649429247001);
    test::<f64>("1/3", 0.9854206469277671);
    test::<f64>("1/4", 1.2880225246980774);
    test::<f64>("1/6", 1.7167334350782404);
    test::<f64>("2/3", 0.3031502751475236);
    test::<f64>("-1/2", f64::NAN);
    test::<f64>("-1", f64::NAN);
    test::<f64>("-2", f64::NAN);
    test::<f64>("-5/2", f64::NAN);
    test::<f64>("-7/3", f64::NAN);
    test::<f64>("22/7", 0.8289305672119419);
    test::<f64>("1000001/1000", 5905.227330464877);
    test::<f64>("-1000001/1000", f64::NAN);
    test::<f64>("1/1267650600228229401496703205376", 69.31471805599453);
    test::<f64>("-1/1267650600228229401496703205376", f64::NAN);
    test::<f64>("35", 88.58082754219768);
    test::<f64>("36", 92.1361756036871);
    test::<f64>("171", 706.5730622457874);
    test::<f64>("172", 711.71472580229);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_ln_gamma_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        primitive_float_ln_gamma_rational::<T>(&x);
    });
}

#[test]
fn primitive_float_ln_gamma_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_ln_gamma_rational_properties_helper);
}
