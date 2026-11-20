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
use malachite_base::num::gamma::traits::{Gamma, GammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::gamma::gamma::{primitive_float_gamma, primitive_float_gamma_rational};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, exponent_in_gate, parse_hex_string,
    rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::gamma::gamma::{
    rug_gamma_prec_round, rug_gamma_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;

#[test]
fn test_gamma() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let gamma = x.clone().gamma();
        assert!(gamma.is_valid());

        assert_eq!(gamma.to_string(), out);
        assert_eq!(to_hex_string(&gamma), out_hex);

        let gamma_alt = (&x).gamma();
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));

        let mut gamma_alt = x.clone();
        gamma_alt.gamma_assign();
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "Infinity", "Infinity");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("2.0", "0x2.0#1", "1.0", "0x1.0#1");
    test("3.0", "0x3.0#2", "2.0", "0x2.0#2");
    test("4.0", "0x4.0#1", "8.0", "0x8.0#1");
    test("5.0", "0x5.0#3", "24.0", "0x18.0#3");
    test("10.0", "0xa.0#3", "3.9e5", "0x6.0E+4#3");
    test("25.0", "0x19.0#5", "6.04e23", "0x8.0E+19#5");
    test("171.0", "0xab.0#8", "7.242e306", "0xa.5E+254#8");
    test("0.50", "0x0.8#1", "2.0", "0x2.0#1");
    test("1.5", "0x1.8#2", "1.0", "0x1.0#2");
    test("0.25", "0x0.4#1", "4.0", "0x4.0#1");
    test("0.75", "0x0.c#2", "1.0", "0x1.0#2");
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        "9.5135076986687324",
        "0x9.83753d9406c4#52",
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        "2.6834373819557680",
        "0x2.aef5c0945d278#52",
    );
    test("-0.50", "-0x0.8#1", "-4.0", "-0x4.0#1");
    test("-1.5", "-0x1.8#2", "2.0", "0x2.0#2");
    test("-2.5", "-0x2.8#3", "-1.0", "-0x1.0#3");
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        "-10.686287021193191",
        "-0xa.afb08197b1ab#52",
    );
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test("-2.0", "-0x2.0#1", "NaN", "NaN");
    test("-100.5", "-0x64.8#8", "-3.361e-159", "-0x2.f4E-132#8");
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        "0.99999994227844335",
        "0x0.ffffff08167a30#53",
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        "0.99999995772157058",
        "0x0.ffffff4a6a6010#53",
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        "-4999999.5467908205",
        "-0x4c4b3f.8bfa7bb4#53",
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        "2.2880377953400324179595889090588",
        "0x2.49bcd84eff5452fcc6c8e2150#100",
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc#100",
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        "-1.30575658444688611664",
        "-0x1.4e461042bc6cfe86#64",
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        "9999999999.4227848",
        "0x2540be3ff.6c3ba#53",
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        "-10000000000.577215",
        "-0x2540be400.93c46#53",
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        "9.9999999999999990e299",
        "0x1.7e43c8800759bE+249#53",
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        "-9.9999999999999990e299",
        "-0x1.7e43c8800759bE+249#53",
    );
    test("1000.5", "0x3e8.8#11", "1.2721e2566", "0x1.554E+2131#11");
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        "2.8240e456568",
        "0xb.d4E+379171#12",
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        "8.2639326e5565705",
        "0x7.3b1f0E+4622218#21",
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        "-3.8015706e-5565712",
        "-0x7.4a038E-4622224#21",
    );
    test("6.7e7", "0x4.0E+6#1", "Infinity", "Infinity");
    test("67108864.50", "0x4000000.8#28", "Infinity", "Infinity");
    test("-67108864.50", "-0x4000000.8#28", "-0.0", "-0x0.0");
    test("-67108865.50", "-0x4000001.8#28", "0.0", "0x0.0");
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        "Infinity",
        "Infinity",
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        "NaN",
        "NaN",
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        "9.9e301029",
        "0x1.0E+250000#1",
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        "-9.9e301029",
        "-0x1.0E+250000#1",
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "Infinity",
        "Infinity",
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        "-Infinity",
        "-Infinity",
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        "Infinity",
        "Infinity",
    );
}

#[test]
fn test_gamma_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (gamma, o) = x.clone().gamma_prec(prec);
        assert!(gamma.is_valid());

        assert_eq!(gamma.to_string(), out);
        assert_eq!(to_hex_string(&gamma), out_hex);
        assert_eq!(o, o_out);

        let (gamma_alt, o_alt) = x.gamma_prec_ref(prec);
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
        assert_eq!(o_alt, o_out);

        let mut gamma_alt = x.clone();
        let o_alt = gamma_alt.gamma_prec_assign(prec);
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
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
    test("-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 10, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, "2.0", "0x2.0#1", Equal);
    test("3.0", "0x3.0#2", 10, "2.0000", "0x2.00#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, "8.0", "0x8.0#1", Greater);
    test("4.0", "0x4.0#1", 10, "6.0000", "0x6.00#10", Equal);
    test(
        "4.0",
        "0x4.0#1",
        100,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test("5.0", "0x5.0#3", 1, "32.0", "0x2.0E+1#1", Greater);
    test("5.0", "0x5.0#3", 10, "24.000", "0x18.00#10", Equal);
    test(
        "5.0",
        "0x5.0#3",
        100,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, "2.6e5", "0x4.0E+4#1", Less);
    test("10.0", "0xa.0#3", 10, "3.6301e5", "0x5.8aE+4#10", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test("25.0", "0x19.0#5", 1, "6.0e23", "0x8.0E+19#1", Less);
    test(
        "25.0",
        "0x19.0#5",
        10,
        "6.2099e23",
        "0x8.38E+19#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        100,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test("171.0", "0xab.0#8", 1, "5.6e306", "0x8.0E+254#1", Less);
    test(
        "171.0",
        "0xab.0#8",
        10,
        "7.2527e306",
        "0xa.54E+254#10",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        100,
        "7.2574156153079989673967282111251e306",
        "0xa.55bc3220c31c79101dd27c41E+254#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, "2.0", "0x2.0#1", Greater);
    test("0.50", "0x0.8#1", 10, "1.7715", "0x1.c58#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, "1.0", "0x1.0#1", Greater);
    test("1.5", "0x1.8#2", 10, "0.88574", "0x0.e2c#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", 10, "3.6250", "0x3.a0#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, "1.0", "0x1.0#1", Less);
    test("0.75", "0x0.c#2", 10, "1.2246", "0x1.398#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        "1.2254167024651776451290983033636",
        "0x1.39b4e8b50f62c027af524acd4#100",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        "9.5156",
        "0x9.84#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        100,
        "9.5135076986687312858079798958231",
        "0x9.83753d9406c3b29e53ad31da#100",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        "2.6836",
        "0x2.af#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        100,
        "2.6834373819557683003231093395824",
        "0x2.aef5c0945d27976cf2b85fe4c#100",
        Less,
    );
    test("-0.50", "-0x0.8#1", 1, "-4.0", "-0x4.0#1", Less);
    test("-0.50", "-0x0.8#1", 10, "-3.5430", "-0x3.8b#10", Greater);
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac#100",
        Less,
    );
    test("-1.5", "-0x1.8#2", 1, "2.0", "0x2.0#1", Less);
    test("-1.5", "-0x1.8#2", 10, "2.3633", "0x2.5d#10", Greater);
    test(
        "-1.5",
        "-0x1.8#2",
        100,
        "2.3632718012073547030642233111227",
        "0x2.5cff6179be9e38a25a406d674#100",
        Greater,
    );
    test("-2.5", "-0x2.8#3", 1, "-1.0", "-0x1.0#1", Less);
    test("-2.5", "-0x2.8#3", 10, "-0.94531", "-0x0.f20#10", Less);
    test(
        "-2.5",
        "-0x2.8#3",
        100,
        "-0.94530872048294188122568932444893",
        "-0x0.f1ffc097190c16a7574cf88fb#100",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        "-8.0",
        "-0x8.0#1",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        "-10.688",
        "-0xa.b0#10",
        Less,
    );
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, "NaN", "NaN", Equal);
    test(
        "-100.5",
        "-0x64.8#8",
        1,
        "-2.3e-159",
        "-0x2.0E-132#1",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        "-3.3519e-159",
        "-0x2.f2E-132#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        1,
        "-4.2e6",
        "-0x4.0E+5#1",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        "-4.9971e6",
        "-0x4.c4E+5#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        "2.2891",
        "0x2.4a#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        "2.6797",
        "0x2.ae#10",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        1,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        "-1.3066",
        "-0x1.4e8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        "8.6e9",
        "0x2.0E+8#1",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        1,
        "-8.6e9",
        "-0x2.0E+8#1",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        "-9.9992e9",
        "-0x2.54E+8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        1,
        "6.7e299",
        "0x1.0E+249#1",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        "1.0006e300",
        "0x1.7e8E+249#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        1,
        "-6.7e299",
        "-0x1.0E+249#1",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        "-1.0006e300",
        "-0x1.7e8E+249#10",
        Less,
    );
    test("1000.5", "0x3e8.8#11", 1, "9.5e2565", "0x1.0E+2131#1", Less);
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        "1.2730e2566",
        "0x1.558E+2131#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        1,
        "1.9e456568",
        "0x8.0E+379171#1",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        "2.8240e456568",
        "0xb.d4E+379171#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        1,
        "9.1e5565705",
        "0x8.0E+4622218#1",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        "8.2679e5565705",
        "0x7.3cE+4622218#10",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        1,
        "-4.2e-5565712",
        "-0x8.0E-4622224#1",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224#10",
        Greater,
    );
    test("6.7e7", "0x4.0E+6#1", 1, "Infinity", "Infinity", Greater);
    test("6.7e7", "0x4.0E+6#1", 10, "Infinity", "Infinity", Greater);
    test(
        "67108864.50",
        "0x4000000.8#28",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        1,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test("-67108865.50", "-0x4000001.8#28", 1, "0.0", "0x0.0", Less);
    test("-67108865.50", "-0x4000001.8#28", 10, "0.0", "0x0.0", Less);
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        "Infinity",
        "Infinity",
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
        "9.9e301029",
        "0x1.0E+250000#1",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        "9.9007e301029",
        "0x1.000E+250000#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        1,
        "-9.9e301029",
        "-0x1.0E+250000#1",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        "-9.9007e301029",
        "-0x1.000E+250000#10",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        "-Infinity",
        "-Infinity",
        Less,
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
}

#[test]
#[should_panic]
fn gamma_prec_fail() {
    Float::NAN.gamma_prec(0);
}

#[test]
fn test_gamma_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (gamma, o) = x.clone().gamma_round(rm);
        assert!(gamma.is_valid());

        assert_eq!(gamma.to_string(), out);
        assert_eq!(to_hex_string(&gamma), out_hex);
        assert_eq!(o, o_out);

        let (gamma_alt, o_alt) = x.gamma_round_ref(rm);
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
        assert_eq!(o_alt, o_out);

        let mut gamma_alt = x.clone();
        let o_alt = gamma_alt.gamma_round_assign(rm);
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
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
    test("-0.0", "-0x0.0", Floor, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Down, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Nearest, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Exact, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Exact, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Exact, "1.0", "0x1.0#1", Equal);
    test("3.0", "0x3.0#2", Floor, "2.0", "0x2.0#2", Equal);
    test("3.0", "0x3.0#2", Ceiling, "2.0", "0x2.0#2", Equal);
    test("3.0", "0x3.0#2", Down, "2.0", "0x2.0#2", Equal);
    test("3.0", "0x3.0#2", Up, "2.0", "0x2.0#2", Equal);
    test("3.0", "0x3.0#2", Nearest, "2.0", "0x2.0#2", Equal);
    test("3.0", "0x3.0#2", Exact, "2.0", "0x2.0#2", Equal);
    test("4.0", "0x4.0#1", Floor, "4.0", "0x4.0#1", Less);
    test("4.0", "0x4.0#1", Ceiling, "8.0", "0x8.0#1", Greater);
    test("4.0", "0x4.0#1", Down, "4.0", "0x4.0#1", Less);
    test("4.0", "0x4.0#1", Up, "8.0", "0x8.0#1", Greater);
    test("4.0", "0x4.0#1", Nearest, "8.0", "0x8.0#1", Greater);
    test("5.0", "0x5.0#3", Floor, "24.0", "0x18.0#3", Equal);
    test("5.0", "0x5.0#3", Ceiling, "24.0", "0x18.0#3", Equal);
    test("5.0", "0x5.0#3", Down, "24.0", "0x18.0#3", Equal);
    test("5.0", "0x5.0#3", Up, "24.0", "0x18.0#3", Equal);
    test("5.0", "0x5.0#3", Nearest, "24.0", "0x18.0#3", Equal);
    test("5.0", "0x5.0#3", Exact, "24.0", "0x18.0#3", Equal);
    test("10.0", "0xa.0#3", Floor, "3.3e5", "0x5.0E+4#3", Less);
    test("10.0", "0xa.0#3", Ceiling, "3.9e5", "0x6.0E+4#3", Greater);
    test("10.0", "0xa.0#3", Down, "3.3e5", "0x5.0E+4#3", Less);
    test("10.0", "0xa.0#3", Up, "3.9e5", "0x6.0E+4#3", Greater);
    test("10.0", "0xa.0#3", Nearest, "3.9e5", "0x6.0E+4#3", Greater);
    test("25.0", "0x19.0#5", Floor, "6.04e23", "0x8.0E+19#5", Less);
    test(
        "25.0",
        "0x19.0#5",
        Ceiling,
        "6.42e23",
        "0x8.8E+19#5",
        Greater,
    );
    test("25.0", "0x19.0#5", Down, "6.04e23", "0x8.0E+19#5", Less);
    test("25.0", "0x19.0#5", Up, "6.42e23", "0x8.8E+19#5", Greater);
    test("25.0", "0x19.0#5", Nearest, "6.04e23", "0x8.0E+19#5", Less);
    test(
        "171.0",
        "0xab.0#8",
        Floor,
        "7.242e306",
        "0xa.5E+254#8",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        Ceiling,
        "7.286e306",
        "0xa.6E+254#8",
        Greater,
    );
    test("171.0", "0xab.0#8", Down, "7.242e306", "0xa.5E+254#8", Less);
    test(
        "171.0",
        "0xab.0#8",
        Up,
        "7.286e306",
        "0xa.6E+254#8",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        Nearest,
        "7.242e306",
        "0xa.5E+254#8",
        Less,
    );
    test("0.50", "0x0.8#1", Floor, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.50", "0x0.8#1", Down, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", Up, "2.0", "0x2.0#1", Greater);
    test("0.50", "0x0.8#1", Nearest, "2.0", "0x2.0#1", Greater);
    test("1.5", "0x1.8#2", Floor, "0.75", "0x0.c#2", Less);
    test("1.5", "0x1.8#2", Ceiling, "1.0", "0x1.0#2", Greater);
    test("1.5", "0x1.8#2", Down, "0.75", "0x0.c#2", Less);
    test("1.5", "0x1.8#2", Up, "1.0", "0x1.0#2", Greater);
    test("1.5", "0x1.8#2", Nearest, "1.0", "0x1.0#2", Greater);
    test("0.25", "0x0.4#1", Floor, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", Ceiling, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", Down, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", Up, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", Nearest, "4.0", "0x4.0#1", Greater);
    test("0.75", "0x0.c#2", Floor, "1.0", "0x1.0#2", Less);
    test("0.75", "0x0.c#2", Ceiling, "1.5", "0x1.8#2", Greater);
    test("0.75", "0x0.c#2", Down, "1.0", "0x1.0#2", Less);
    test("0.75", "0x0.c#2", Up, "1.5", "0x1.8#2", Greater);
    test("0.75", "0x0.c#2", Nearest, "1.0", "0x1.0#2", Less);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Floor,
        "9.5135076986687288",
        "0x9.83753d9406c3#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Ceiling,
        "9.5135076986687324",
        "0x9.83753d9406c4#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Down,
        "9.5135076986687288",
        "0x9.83753d9406c3#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Up,
        "9.5135076986687324",
        "0x9.83753d9406c4#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Nearest,
        "9.5135076986687324",
        "0x9.83753d9406c4#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Floor,
        "2.6834373819557680",
        "0x2.aef5c0945d278#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Ceiling,
        "2.6834373819557689",
        "0x2.aef5c0945d27c#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Down,
        "2.6834373819557680",
        "0x2.aef5c0945d278#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Up,
        "2.6834373819557689",
        "0x2.aef5c0945d27c#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Nearest,
        "2.6834373819557680",
        "0x2.aef5c0945d278#52",
        Less,
    );
    test("-0.50", "-0x0.8#1", Floor, "-4.0", "-0x4.0#1", Less);
    test("-0.50", "-0x0.8#1", Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("-0.50", "-0x0.8#1", Down, "-2.0", "-0x2.0#1", Greater);
    test("-0.50", "-0x0.8#1", Up, "-4.0", "-0x4.0#1", Less);
    test("-0.50", "-0x0.8#1", Nearest, "-4.0", "-0x4.0#1", Less);
    test("-1.5", "-0x1.8#2", Floor, "2.0", "0x2.0#2", Less);
    test("-1.5", "-0x1.8#2", Ceiling, "3.0", "0x3.0#2", Greater);
    test("-1.5", "-0x1.8#2", Down, "2.0", "0x2.0#2", Less);
    test("-1.5", "-0x1.8#2", Up, "3.0", "0x3.0#2", Greater);
    test("-1.5", "-0x1.8#2", Nearest, "2.0", "0x2.0#2", Less);
    test("-2.5", "-0x2.8#3", Floor, "-1.0", "-0x1.0#3", Less);
    test("-2.5", "-0x2.8#3", Ceiling, "-0.88", "-0x0.e#3", Greater);
    test("-2.5", "-0x2.8#3", Down, "-0.88", "-0x0.e#3", Greater);
    test("-2.5", "-0x2.8#3", Up, "-1.0", "-0x1.0#3", Less);
    test("-2.5", "-0x2.8#3", Nearest, "-1.0", "-0x1.0#3", Less);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Floor,
        "-10.686287021193195",
        "-0xa.afb08197b1ac#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Ceiling,
        "-10.686287021193191",
        "-0xa.afb08197b1ab#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Down,
        "-10.686287021193191",
        "-0xa.afb08197b1ab#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Up,
        "-10.686287021193195",
        "-0xa.afb08197b1ac#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Nearest,
        "-10.686287021193191",
        "-0xa.afb08197b1ab#52",
        Greater,
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
        "9999999999.4227829",
        "0x2540be3ff.6c3b8#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Ceiling,
        "9999999999.4227848",
        "0x2540be3ff.6c3ba#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Down,
        "9999999999.4227829",
        "0x2540be3ff.6c3b8#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Up,
        "9999999999.4227848",
        "0x2540be3ff.6c3ba#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Nearest,
        "9999999999.4227848",
        "0x2540be3ff.6c3ba#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Floor,
        "-10000000000.577217",
        "-0x2540be400.93c48#53",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Ceiling,
        "-10000000000.577215",
        "-0x2540be400.93c46#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Down,
        "-10000000000.577215",
        "-0x2540be400.93c46#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Up,
        "-10000000000.577217",
        "-0x2540be400.93c48#53",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Nearest,
        "-10000000000.577215",
        "-0x2540be400.93c46#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Floor,
        "9.9999999999999990e299",
        "0x1.7e43c8800759bE+249#53",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Ceiling,
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Down,
        "9.9999999999999990e299",
        "0x1.7e43c8800759bE+249#53",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Up,
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Nearest,
        "9.9999999999999990e299",
        "0x1.7e43c8800759bE+249#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Floor,
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Ceiling,
        "-9.9999999999999990e299",
        "-0x1.7e43c8800759bE+249#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Down,
        "-9.9999999999999990e299",
        "-0x1.7e43c8800759bE+249#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Up,
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Nearest,
        "-9.9999999999999990e299",
        "-0x1.7e43c8800759bE+249#53",
        Greater,
    );
}

#[test]
#[should_panic]
fn gamma_round_fail() {
    Float::from(0.5).gamma_round(Exact);
}

#[test]
fn test_gamma_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (gamma, o) = x.clone().gamma_prec_round(prec, rm);
        assert!(gamma.is_valid());

        assert_eq!(gamma.to_string(), out);
        assert_eq!(to_hex_string(&gamma), out_hex);
        assert_eq!(o, o_out);

        let (gamma_alt, o_alt) = x.gamma_prec_round_ref(prec, rm);
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
        assert_eq!(o_alt, o_out);

        let mut gamma_alt = x.clone();
        let o_alt = gamma_alt.gamma_prec_round_assign(prec, rm);
        assert!(gamma_alt.is_valid());
        assert_eq!(ComparableFloatRef(&gamma), ComparableFloatRef(&gamma_alt));
        assert_eq!(o_alt, o_out);
    };
    // The expected values agree with Azurite's gamma function.
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
    test("-0.0", "-0x0.0", 10, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 10, Down, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 10, Up, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("3.0", "0x3.0#2", 10, Floor, "2.0000", "0x2.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Ceiling, "2.0000", "0x2.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Down, "2.0000", "0x2.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Up, "2.0000", "0x2.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Nearest, "2.0000", "0x2.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Exact, "2.0000", "0x2.00#10", Equal);
    test("4.0", "0x4.0#1", 10, Floor, "6.0000", "0x6.00#10", Equal);
    test("4.0", "0x4.0#1", 10, Ceiling, "6.0000", "0x6.00#10", Equal);
    test("4.0", "0x4.0#1", 10, Down, "6.0000", "0x6.00#10", Equal);
    test("4.0", "0x4.0#1", 10, Up, "6.0000", "0x6.00#10", Equal);
    test("4.0", "0x4.0#1", 10, Nearest, "6.0000", "0x6.00#10", Equal);
    test("4.0", "0x4.0#1", 10, Exact, "6.0000", "0x6.00#10", Equal);
    test("5.0", "0x5.0#3", 10, Floor, "24.000", "0x18.00#10", Equal);
    test("5.0", "0x5.0#3", 10, Ceiling, "24.000", "0x18.00#10", Equal);
    test("5.0", "0x5.0#3", 10, Down, "24.000", "0x18.00#10", Equal);
    test("5.0", "0x5.0#3", 10, Up, "24.000", "0x18.00#10", Equal);
    test("5.0", "0x5.0#3", 10, Nearest, "24.000", "0x18.00#10", Equal);
    test("5.0", "0x5.0#3", 10, Exact, "24.000", "0x18.00#10", Equal);
    test(
        "10.0",
        "0xa.0#3",
        10,
        Floor,
        "3.6250e5",
        "0x5.88E+4#10",
        Less,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Ceiling,
        "3.6301e5",
        "0x5.8aE+4#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Down,
        "3.6250e5",
        "0x5.88E+4#10",
        Less,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Up,
        "3.6301e5",
        "0x5.8aE+4#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Nearest,
        "3.6301e5",
        "0x5.8aE+4#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Floor,
        "6.1981e23",
        "0x8.34E+19#10",
        Less,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Ceiling,
        "6.2099e23",
        "0x8.38E+19#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Down,
        "6.1981e23",
        "0x8.34E+19#10",
        Less,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Up,
        "6.2099e23",
        "0x8.38E+19#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Nearest,
        "6.2099e23",
        "0x8.38E+19#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Floor,
        "7.2527e306",
        "0xa.54E+254#10",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Ceiling,
        "7.2636e306",
        "0xa.58E+254#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Down,
        "7.2527e306",
        "0xa.54E+254#10",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Up,
        "7.2636e306",
        "0xa.58E+254#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Nearest,
        "7.2527e306",
        "0xa.54E+254#10",
        Less,
    );
    test("0.50", "0x0.8#1", 10, Floor, "1.7715", "0x1.c58#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.7734",
        "0x1.c60#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Down, "1.7715", "0x1.c58#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "1.7734", "0x1.c60#10", Greater);
    test("0.50", "0x0.8#1", 10, Nearest, "1.7715", "0x1.c58#10", Less);
    test("1.5", "0x1.8#2", 10, Floor, "0.88574", "0x0.e2c#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "0.88672",
        "0x0.e30#10",
        Greater,
    );
    test("1.5", "0x1.8#2", 10, Down, "0.88574", "0x0.e2c#10", Less);
    test("1.5", "0x1.8#2", 10, Up, "0.88672", "0x0.e30#10", Greater);
    test("1.5", "0x1.8#2", 10, Nearest, "0.88574", "0x0.e2c#10", Less);
    test("0.25", "0x0.4#1", 10, Floor, "3.6250", "0x3.a0#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "3.6289",
        "0x3.a1#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Down, "3.6250", "0x3.a0#10", Less);
    test("0.25", "0x0.4#1", 10, Up, "3.6289", "0x3.a1#10", Greater);
    test("0.25", "0x0.4#1", 10, Nearest, "3.6250", "0x3.a0#10", Less);
    test("0.75", "0x0.c#2", 10, Floor, "1.2246", "0x1.398#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "1.2266",
        "0x1.3a0#10",
        Greater,
    );
    test("0.75", "0x0.c#2", 10, Down, "1.2246", "0x1.398#10", Less);
    test("0.75", "0x0.c#2", 10, Up, "1.2266", "0x1.3a0#10", Greater);
    test("0.75", "0x0.c#2", 10, Nearest, "1.2246", "0x1.398#10", Less);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "9.5000",
        "0x9.80#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "9.5156",
        "0x9.84#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Down,
        "9.5000",
        "0x9.80#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Up,
        "9.5156",
        "0x9.84#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "9.5156",
        "0x9.84#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Floor,
        "2.6797",
        "0x2.ae#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Ceiling,
        "2.6836",
        "0x2.af#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Down,
        "2.6797",
        "0x2.ae#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Up,
        "2.6836",
        "0x2.af#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Nearest,
        "2.6836",
        "0x2.af#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Floor,
        "-3.5469",
        "-0x3.8c#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Ceiling,
        "-3.5430",
        "-0x3.8b#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Down,
        "-3.5430",
        "-0x3.8b#10",
        Greater,
    );
    test("-0.50", "-0x0.8#1", 10, Up, "-3.5469", "-0x3.8c#10", Less);
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "-3.5430",
        "-0x3.8b#10",
        Greater,
    );
    test("-1.5", "-0x1.8#2", 10, Floor, "2.3594", "0x2.5c#10", Less);
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Ceiling,
        "2.3633",
        "0x2.5d#10",
        Greater,
    );
    test("-1.5", "-0x1.8#2", 10, Down, "2.3594", "0x2.5c#10", Less);
    test("-1.5", "-0x1.8#2", 10, Up, "2.3633", "0x2.5d#10", Greater);
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Nearest,
        "2.3633",
        "0x2.5d#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-0.94531",
        "-0x0.f20#10",
        Less,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Ceiling,
        "-0.94434",
        "-0x0.f1c#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Down,
        "-0.94434",
        "-0x0.f1c#10",
        Greater,
    );
    test("-2.5", "-0x2.8#3", 10, Up, "-0.94531", "-0x0.f20#10", Less);
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Nearest,
        "-0.94531",
        "-0x0.f20#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-10.688",
        "-0xa.b0#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-10.672",
        "-0xa.ac#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Down,
        "-10.672",
        "-0xa.ac#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "-10.688",
        "-0xa.b0#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-10.688",
        "-0xa.b0#10",
        Less,
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
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Floor,
        "-3.3564e-159",
        "-0x2.f3E-132#10",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Ceiling,
        "-3.3519e-159",
        "-0x2.f2E-132#10",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Down,
        "-3.3519e-159",
        "-0x2.f2E-132#10",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Up,
        "-3.3564e-159",
        "-0x2.f3E-132#10",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Nearest,
        "-3.3519e-159",
        "-0x2.f2E-132#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Floor,
        "-5.0053e6",
        "-0x4.c6E+5#10",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Ceiling,
        "-4.9971e6",
        "-0x4.c4E+5#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Down,
        "-4.9971e6",
        "-0x4.c4E+5#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Up,
        "-5.0053e6",
        "-0x4.c6E+5#10",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Nearest,
        "-4.9971e6",
        "-0x4.c4E+5#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Floor,
        "2.2852",
        "0x2.49#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Ceiling,
        "2.2891",
        "0x2.4a#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Down,
        "2.2852",
        "0x2.49#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Up,
        "2.2891",
        "0x2.4a#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Nearest,
        "2.2891",
        "0x2.4a#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "2.6758",
        "0x2.ad#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "2.6797",
        "0x2.ae#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Down,
        "2.6758",
        "0x2.ad#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Up,
        "2.6797",
        "0x2.ae#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "2.6797",
        "0x2.ae#10",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Floor,
        "-1.3066",
        "-0x1.4e8#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Ceiling,
        "-1.3047",
        "-0x1.4e0#10",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Down,
        "-1.3047",
        "-0x1.4e0#10",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Up,
        "-1.3066",
        "-0x1.4e8#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Nearest,
        "-1.3066",
        "-0x1.4e8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "1.0016e10",
        "0x2.55E+8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "1.0016e10",
        "0x2.55E+8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "-1.0016e10",
        "-0x2.55E+8#10",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "-9.9992e9",
        "-0x2.54E+8#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "-9.9992e9",
        "-0x2.54E+8#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "-1.0016e10",
        "-0x2.55E+8#10",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "-9.9992e9",
        "-0x2.54E+8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Floor,
        "9.9931e299",
        "0x1.7e0E+249#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Ceiling,
        "1.0006e300",
        "0x1.7e8E+249#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Down,
        "9.9931e299",
        "0x1.7e0E+249#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Up,
        "1.0006e300",
        "0x1.7e8E+249#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Nearest,
        "1.0006e300",
        "0x1.7e8E+249#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Floor,
        "-1.0006e300",
        "-0x1.7e8E+249#10",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Ceiling,
        "-9.9931e299",
        "-0x1.7e0E+249#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Down,
        "-9.9931e299",
        "-0x1.7e0E+249#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Up,
        "-1.0006e300",
        "-0x1.7e8E+249#10",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Nearest,
        "-1.0006e300",
        "-0x1.7e8E+249#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Floor,
        "1.2712e2566",
        "0x1.550E+2131#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Ceiling,
        "1.2730e2566",
        "0x1.558E+2131#10",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Down,
        "1.2712e2566",
        "0x1.550E+2131#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Up,
        "1.2730e2566",
        "0x1.558E+2131#10",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Nearest,
        "1.2730e2566",
        "0x1.558E+2131#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Floor,
        "2.8240e456568",
        "0xb.d4E+379171#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Ceiling,
        "2.8277e456568",
        "0xb.d8E+379171#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Down,
        "2.8240e456568",
        "0xb.d4E+379171#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Up,
        "2.8277e456568",
        "0xb.d8E+379171#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Nearest,
        "2.8240e456568",
        "0xb.d4E+379171#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Floor,
        "8.2589e5565705",
        "0x7.3aE+4622218#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Ceiling,
        "8.2679e5565705",
        "0x7.3cE+4622218#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Down,
        "8.2589e5565705",
        "0x7.3aE+4622218#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Up,
        "8.2679e5565705",
        "0x7.3cE+4622218#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Nearest,
        "8.2679e5565705",
        "0x7.3cE+4622218#10",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Floor,
        "-3.8056e-5565712",
        "-0x7.4cE-4622224#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Ceiling,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224#10",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Down,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224#10",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Up,
        "-3.8056e-5565712",
        "-0x7.4cE-4622224#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Nearest,
        "-3.8015e-5565712",
        "-0x7.4aE-4622224#10",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Nearest,
        "Infinity",
        "Infinity",
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
        "9.8910e301029",
        "0xf.fcE+249999#10",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Ceiling,
        "9.9007e301029",
        "0x1.000E+250000#10",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Down,
        "9.8910e301029",
        "0xf.fcE+249999#10",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Up,
        "9.9007e301029",
        "0x1.000E+250000#10",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Nearest,
        "9.9007e301029",
        "0x1.000E+250000#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Floor,
        "-9.9200e301029",
        "-0x1.008E+250000#10",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Ceiling,
        "-9.9007e301029",
        "-0x1.000E+250000#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Down,
        "-9.9007e301029",
        "-0x1.000E+250000#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Up,
        "-9.9200e301029",
        "-0x1.008E+250000#10",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Nearest,
        "-9.9007e301029",
        "-0x1.000E+250000#10",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Ceiling,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Down,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
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
    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Down,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Exact,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Floor,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Down,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Exact,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Floor, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Floor,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Ceiling, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Ceiling,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Down, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Down,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Up, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Up,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Nearest,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Exact, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Exact,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, Floor, "4.0", "0x4.0#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Floor,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Ceiling,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, Down, "4.0", "0x4.0#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Down,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, Up, "8.0", "0x8.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Up,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Exact,
        "6.0000000000000000000000000000000",
        "0x6.0000000000000000000000000#100",
        Equal,
    );
    test("5.0", "0x5.0#3", 1, Floor, "16.0", "0x1.0E+1#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Floor,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("5.0", "0x5.0#3", 1, Ceiling, "32.0", "0x2.0E+1#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Ceiling,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("5.0", "0x5.0#3", 1, Down, "16.0", "0x1.0E+1#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Down,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("5.0", "0x5.0#3", 1, Up, "32.0", "0x2.0E+1#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Up,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("5.0", "0x5.0#3", 1, Nearest, "32.0", "0x2.0E+1#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Nearest,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        100,
        Exact,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, Floor, "2.6e5", "0x4.0E+4#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Floor,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        1,
        Ceiling,
        "5.2e5",
        "0x8.0E+4#1",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        100,
        Ceiling,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, Down, "2.6e5", "0x4.0E+4#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Down,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, Up, "5.2e5", "0x8.0E+4#1", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Up,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, Nearest, "2.6e5", "0x4.0E+4#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Nearest,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        100,
        Exact,
        "362880.00000000000000000000000000",
        "0x58980.000000000000000000000#100",
        Equal,
    );
    test("0.50", "0x0.8#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "1.7724538509055160272981674833397",
        "0x1.c5bf891b4ef6aa79c3b0520d4#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Down,
        "1.7724538509055160272981674833397",
        "0x1.c5bf891b4ef6aa79c3b0520d4#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Up, "2.0", "0x2.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Up,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Floor,
        "0.88622692545275801364908374166984",
        "0x0.e2dfc48da77b553ce1d82906a#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Ceiling,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Down, "0.50", "0x0.8#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Down,
        "0.88622692545275801364908374166984",
        "0x0.e2dfc48da77b553ce1d82906a#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Up, "1.0", "0x1.0#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Up,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Nearest,
        "0.88622692545275801364908374167063",
        "0x0.e2dfc48da77b553ce1d82906b#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Floor, "2.0", "0x2.0#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Floor,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Ceiling,
        "3.6256099082219083119306851558705",
        "0x3.a027f88fdddd479b38a50a518#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Down, "2.0", "0x2.0#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Down,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Up, "4.0", "0x4.0#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Up,
        "3.6256099082219083119306851558705",
        "0x3.a027f88fdddd479b38a50a518#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Nearest, "4.0", "0x4.0#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Nearest,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Floor, "1.0", "0x1.0#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Floor,
        "1.2254167024651776451290983033620",
        "0x1.39b4e8b50f62c027af524acd2#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Ceiling,
        "1.2254167024651776451290983033636",
        "0x1.39b4e8b50f62c027af524acd4#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Down,
        "1.2254167024651776451290983033620",
        "0x1.39b4e8b50f62c027af524acd2#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Up, "2.0", "0x2.0#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Up,
        "1.2254167024651776451290983033636",
        "0x1.39b4e8b50f62c027af524acd4#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Nearest,
        "1.2254167024651776451290983033636",
        "0x1.39b4e8b50f62c027af524acd4#100",
        Greater,
    );
    // - gamma_prec_round_helper huge positive
    test(
        "1.34e8",
        "0x8.0E+6#5",
        7,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - gamma_prec_round_helper huge negative integer
    test("-8.6e9", "-0x2.0E+8#1", 4, Up, "NaN", "NaN", Equal);
    // - gamma_prec_round_helper huge negative
    test(
        "-134283263.94",
        "-0x800ffff.f#32",
        9,
        Up,
        "2.383e-323228497",
        "0x1.00E-268435456#9",
        Greater,
    );
}

#[test]
#[should_panic]
fn gamma_prec_round_fail_1() {
    Float::ONE.gamma_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gamma_prec_round_fail_2() {
    Float::from(0.5).gamma_prec_round(10, Exact);
}

// Whether MPFR's gamma function, which is slow for a large argument, is run on `x`.
fn reference_is_cheap(x: &Float) -> bool {
    x.get_exponent().is_none_or(|e| e <= 16)
}

// Checks a result against MPFR's gamma function.
fn check_rounding(x: &Float, gamma: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && reference_is_cheap(x)
    {
        let (rug_gamma, rug_o) = rug_gamma_prec_round(&rug::Float::exact_from(x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_gamma)),
            ComparableFloatRef(gamma)
        );
        assert_eq!(rug_o, o);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn gamma_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (gamma, o) = x.clone().gamma_prec_round(prec, rm);
    assert!(gamma.is_valid());
    assert_rounding_ordering_consistent(&gamma, rm, o);

    let (gamma_alt, o_alt) = x.gamma_prec_round_ref(prec, rm);
    assert!(gamma_alt.is_valid());
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.gamma_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    check_rounding(&x, &gamma, o, prec, rm);

    if gamma.is_normal() {
        assert_eq!(gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The gamma function is positive on the positive reals.
        assert!(gamma > 0u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.gamma_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(gamma.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.gamma_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) = Float::gamma_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&gamma));
        assert_eq!(ro, o);
    }
}

#[test]
fn gamma_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        gamma_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            gamma_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.gamma_prec_round(prec, rm);
        assert_eq!(
            ComparableFloat(s),
            ComparableFloat(Float::NEGATIVE_INFINITY)
        );
        assert_eq!(o, Equal);

        // The gamma function has poles at the negative integers.
        let (s, o) = Float::NEGATIVE_ONE.gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ONE.gamma_prec_round(prec, rm);
        assert_eq!(s, 1u32);
        assert_eq!(o, Equal);

        let (s, o) = Float::TWO.gamma_prec_round(prec, rm);
        assert_eq!(s, 1u32);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gamma_prec_properties_helper(x: Float, prec: u64) {
    let (gamma, o) = x.clone().gamma_prec(prec);
    assert!(gamma.is_valid());

    let (gamma_alt, o_alt) = x.gamma_prec_ref(prec);
    assert!(gamma_alt.is_valid());
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.gamma_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    let (gamma_alt, o_alt) = x.gamma_prec_round_ref(prec, Nearest);
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    check_rounding(&x, &gamma, o, prec, Nearest);

    if gamma.is_normal() {
        assert_eq!(gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The gamma function is positive on the positive reals.
        assert!(gamma > 0u32);
    }
}

#[test]
fn gamma_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        gamma_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        gamma_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        gamma_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.gamma_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gamma_round_properties_helper(x: Float, rm: RoundingMode) {
    let (gamma, o) = x.clone().gamma_round(rm);
    assert!(gamma.is_valid());
    assert_rounding_ordering_consistent(&gamma, rm, o);

    let (gamma_alt, o_alt) = x.gamma_round_ref(rm);
    assert!(gamma_alt.is_valid());
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.gamma_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    let (gamma_alt, o_alt) = x.gamma_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    check_rounding(&x, &gamma, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.gamma_round_ref(rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(gamma.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.gamma_round_ref(Exact));
    }
}

#[test]
fn gamma_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        gamma_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_47().test_properties_with_config(&config, |(x, rm)| {
        gamma_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gamma_properties_helper(x: Float) {
    let gamma = x.clone().gamma();
    assert!(gamma.is_valid());

    let gamma_alt = (&x).gamma();
    assert!(gamma_alt.is_valid());
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));

    let mut x_alt = x.clone();
    x_alt.gamma_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&gamma));

    let gamma_alt = x.gamma_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    let gamma_alt = x.gamma_prec_ref(x.significant_bits()).0;
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    let gamma_alt = x.gamma_round_ref(Nearest).0;
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    if x > 0u32 {
        // The gamma function is positive on the positive reals.
        assert!(gamma > 0u32);
    }
}

#[test]
fn gamma_properties() {
    float_gen().test_properties(|x| {
        gamma_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        gamma_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        gamma_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_gamma() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_gamma(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 1.0);
    test::<f32>(2.0, 1.0);
    test::<f32>(5.0, 24.0);
    test::<f32>(0.5, 1.7724539);
    test::<f32>(-0.5, -3.5449078);
    test::<f32>(-1.0, f32::NAN);
    test::<f32>(1.5, 0.88622695);
    test::<f32>(3.3, 2.6834373);
    test::<f32>(-3.3, 0.43851748);
    test::<f32>(35.0, 2.952328e38);
    test::<f32>(36.0, f32::INFINITY);
    test::<f32>(171.0, f32::INFINITY);
    test::<f32>(172.0, f32::INFINITY);
    test::<f32>(1.0e-30, 1.0e30);
    test::<f32>(-1.0e-30, -1.0e30);
    test::<f32>(1.0e-40, f32::INFINITY);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 1.0);
    test::<f64>(2.0, 1.0);
    test::<f64>(5.0, 24.0);
    test::<f64>(0.5, 1.772453850905516);
    test::<f64>(-0.5, -3.544907701811032);
    test::<f64>(-1.0, f64::NAN);
    test::<f64>(1.5, 0.886226925452758);
    test::<f64>(3.3, 2.6834373819557684);
    test::<f64>(-3.3, 0.43851739219876307);
    test::<f64>(35.0, 2.9523279903960416e38);
    test::<f64>(36.0, 1.0333147966386145e40);
    test::<f64>(171.0, 7.257415615307999e306);
    test::<f64>(172.0, f64::INFINITY);
    test::<f64>(1.0e-30, 9.999999999999999e29);
    test::<f64>(-1.0e-30, -9.999999999999999e29);
    test::<f64>(1.0e-40, 1.0e40);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_gamma_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_gamma(x);
    });
}

#[test]
fn primitive_float_gamma_properties() {
    apply_fn_to_primitive_floats!(primitive_float_gamma_properties_helper);
}

#[test]
fn test_gamma_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::gamma_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::gamma_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "Infinity", "Infinity", Equal);
    test("0", 10, "Infinity", "Infinity", Equal);
    test("0", 100, "Infinity", "Infinity", Equal);
    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "1",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("2", 1, "1.0", "0x1.0#1", Equal);
    test("2", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "2",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("3", 1, "2.0", "0x2.0#1", Equal);
    test("3", 10, "2.0000", "0x2.00#10", Equal);
    test(
        "3",
        100,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test("5", 1, "32.0", "0x2.0E+1#1", Greater);
    test("5", 10, "24.000", "0x18.00#10", Equal);
    test(
        "5",
        100,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test("25", 1, "6.0e23", "0x8.0E+19#1", Less);
    test("25", 10, "6.2099e23", "0x8.38E+19#10", Greater);
    test(
        "25",
        100,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test("1/2", 1, "2.0", "0x2.0#1", Greater);
    test("1/2", 10, "1.7715", "0x1.c58#10", Less);
    test(
        "1/2",
        100,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test("1/3", 1, "2.0", "0x2.0#1", Less);
    test("1/3", 10, "2.6797", "0x2.ae#10", Greater);
    test(
        "1/3",
        100,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc#100",
        Less,
    );
    test("1/4", 1, "4.0", "0x4.0#1", Greater);
    test("1/4", 10, "3.6250", "0x3.a0#10", Less);
    test(
        "1/4",
        100,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test("1/6", 1, "4.0", "0x4.0#1", Less);
    test("1/6", 10, "5.5625", "0x5.90#10", Less);
    test(
        "1/6",
        100,
        "5.5663160017802352042500968952067",
        "0x5.90fa15e2d8fb86a4c6b7ee8e0#100",
        Less,
    );
    test("2/3", 1, "1.0", "0x1.0#1", Less);
    test("2/3", 10, "1.3535", "0x1.5a8#10", Less);
    test(
        "2/3",
        100,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702#100",
        Less,
    );
    test("-1/2", 1, "-4.0", "-0x4.0#1", Less);
    test("-1/2", 10, "-3.5430", "-0x3.8b#10", Greater);
    test(
        "-1/2",
        100,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac#100",
        Less,
    );
    test("-1", 1, "NaN", "NaN", Equal);
    test("-1", 10, "NaN", "NaN", Equal);
    test("-2", 1, "NaN", "NaN", Equal);
    test("-2", 10, "NaN", "NaN", Equal);
    test("-5/2", 1, "-1.0", "-0x1.0#1", Less);
    test("-5/2", 10, "-0.94531", "-0x0.f20#10", Less);
    test("-7/3", 1, "-1.0", "-0x1.0#1", Greater);
    test("-7/3", 10, "-1.3066", "-0x1.4e8#10", Less);
    test("22/7", 1, "2.0", "0x2.0#1", Less);
    test("22/7", 10, "2.2891", "0x2.4a#10", Less);
    test("1000001/1000", 1, "3.0e2564", "0x8.0E+2129#1", Less);
    test(
        "1000001/1000",
        10,
        "4.0539e2564",
        "0xa.e0E+2129#10",
        Greater,
    );
    test("-1000001/1000", 1, "-1.7e-2565", "-0x1.0E-2130#1", Greater);
    test(
        "-1000001/1000",
        10,
        "-2.4691e-2565",
        "-0x1.790E-2130#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        "1.3e30",
        "0x1.0E+25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        1,
        "-1.3e30",
        "-0x1.0E+25#1",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        "-1.2677e30",
        "-0x1.000E+25#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test("100000000", 1, "Infinity", "Infinity", Greater);
    test("100000000", 10, "Infinity", "Infinity", Greater);
    test("200000001/2", 1, "Infinity", "Infinity", Greater);
    test("200000001/2", 10, "Infinity", "Infinity", Greater);
    test("-200000001/2", 1, "-0.0", "-0x0.0", Greater);
    test("-200000001/2", 10, "-0.0", "-0x0.0", Greater);
    test("-200000003/2", 1, "0.0", "0x0.0", Less);
    test("-200000003/2", 10, "0.0", "0x0.0", Less);
}

#[test]
#[should_panic]
fn gamma_rational_prec_fail() {
    Float::gamma_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn gamma_rational_prec_ref_fail() {
    Float::gamma_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_gamma_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::gamma_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::gamma_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    // - gamma_rational_prec_round_helper zero
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("3", 10, Floor, "2.0000", "0x2.00#10", Equal);
    test("3", 10, Ceiling, "2.0000", "0x2.00#10", Equal);
    test("3", 10, Down, "2.0000", "0x2.00#10", Equal);
    test("3", 10, Up, "2.0000", "0x2.00#10", Equal);
    test("3", 10, Nearest, "2.0000", "0x2.00#10", Equal);
    test("3", 10, Exact, "2.0000", "0x2.00#10", Equal);
    test("5", 10, Floor, "24.000", "0x18.00#10", Equal);
    test("5", 10, Ceiling, "24.000", "0x18.00#10", Equal);
    test("5", 10, Down, "24.000", "0x18.00#10", Equal);
    test("5", 10, Up, "24.000", "0x18.00#10", Equal);
    test("5", 10, Nearest, "24.000", "0x18.00#10", Equal);
    test("5", 10, Exact, "24.000", "0x18.00#10", Equal);
    test("25", 10, Floor, "6.1981e23", "0x8.34E+19#10", Less);
    test("25", 10, Ceiling, "6.2099e23", "0x8.38E+19#10", Greater);
    test("25", 10, Down, "6.1981e23", "0x8.34E+19#10", Less);
    test("25", 10, Up, "6.2099e23", "0x8.38E+19#10", Greater);
    test("25", 10, Nearest, "6.2099e23", "0x8.38E+19#10", Greater);
    test("1/2", 10, Floor, "1.7715", "0x1.c58#10", Less);
    test("1/2", 10, Ceiling, "1.7734", "0x1.c60#10", Greater);
    test("1/2", 10, Down, "1.7715", "0x1.c58#10", Less);
    test("1/2", 10, Up, "1.7734", "0x1.c60#10", Greater);
    test("1/2", 10, Nearest, "1.7715", "0x1.c58#10", Less);
    test("1/3", 10, Floor, "2.6758", "0x2.ad#10", Less);
    test("1/3", 10, Ceiling, "2.6797", "0x2.ae#10", Greater);
    test("1/3", 10, Down, "2.6758", "0x2.ad#10", Less);
    test("1/3", 10, Up, "2.6797", "0x2.ae#10", Greater);
    test("1/3", 10, Nearest, "2.6797", "0x2.ae#10", Greater);
    test("1/4", 10, Floor, "3.6250", "0x3.a0#10", Less);
    test("1/4", 10, Ceiling, "3.6289", "0x3.a1#10", Greater);
    test("1/4", 10, Down, "3.6250", "0x3.a0#10", Less);
    test("1/4", 10, Up, "3.6289", "0x3.a1#10", Greater);
    test("1/4", 10, Nearest, "3.6250", "0x3.a0#10", Less);
    test("1/6", 10, Floor, "5.5625", "0x5.90#10", Less);
    test("1/6", 10, Ceiling, "5.5703", "0x5.92#10", Greater);
    test("1/6", 10, Down, "5.5625", "0x5.90#10", Less);
    test("1/6", 10, Up, "5.5703", "0x5.92#10", Greater);
    test("1/6", 10, Nearest, "5.5625", "0x5.90#10", Less);
    test("2/3", 10, Floor, "1.3535", "0x1.5a8#10", Less);
    test("2/3", 10, Ceiling, "1.3555", "0x1.5b0#10", Greater);
    test("2/3", 10, Down, "1.3535", "0x1.5a8#10", Less);
    test("2/3", 10, Up, "1.3555", "0x1.5b0#10", Greater);
    test("2/3", 10, Nearest, "1.3535", "0x1.5a8#10", Less);
    test("-1/2", 10, Floor, "-3.5469", "-0x3.8c#10", Less);
    test("-1/2", 10, Ceiling, "-3.5430", "-0x3.8b#10", Greater);
    test("-1/2", 10, Down, "-3.5430", "-0x3.8b#10", Greater);
    test("-1/2", 10, Up, "-3.5469", "-0x3.8c#10", Less);
    test("-1/2", 10, Nearest, "-3.5430", "-0x3.8b#10", Greater);
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
    test("-5/2", 10, Floor, "-0.94531", "-0x0.f20#10", Less);
    test("-5/2", 10, Ceiling, "-0.94434", "-0x0.f1c#10", Greater);
    test("-5/2", 10, Down, "-0.94434", "-0x0.f1c#10", Greater);
    test("-5/2", 10, Up, "-0.94531", "-0x0.f20#10", Less);
    test("-5/2", 10, Nearest, "-0.94531", "-0x0.f20#10", Less);
    test("-7/3", 10, Floor, "-1.3066", "-0x1.4e8#10", Less);
    test("-7/3", 10, Ceiling, "-1.3047", "-0x1.4e0#10", Greater);
    test("-7/3", 10, Down, "-1.3047", "-0x1.4e0#10", Greater);
    test("-7/3", 10, Up, "-1.3066", "-0x1.4e8#10", Less);
    test("-7/3", 10, Nearest, "-1.3066", "-0x1.4e8#10", Less);
    test("22/7", 10, Floor, "2.2891", "0x2.4a#10", Less);
    test("22/7", 10, Ceiling, "2.2930", "0x2.4b#10", Greater);
    test("22/7", 10, Down, "2.2891", "0x2.4a#10", Less);
    test("22/7", 10, Up, "2.2930", "0x2.4b#10", Greater);
    test("22/7", 10, Nearest, "2.2891", "0x2.4a#10", Less);
    test(
        "1000001/1000",
        10,
        Floor,
        "4.0481e2564",
        "0xa.dcE+2129#10",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Ceiling,
        "4.0539e2564",
        "0xa.e0E+2129#10",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Down,
        "4.0481e2564",
        "0xa.dcE+2129#10",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Up,
        "4.0539e2564",
        "0xa.e0E+2129#10",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Nearest,
        "4.0539e2564",
        "0xa.e0E+2129#10",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Floor,
        "-2.4691e-2565",
        "-0x1.790E-2130#10",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Ceiling,
        "-2.4658e-2565",
        "-0x1.788E-2130#10",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Down,
        "-2.4658e-2565",
        "-0x1.788E-2130#10",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Up,
        "-2.4691e-2565",
        "-0x1.790E-2130#10",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Nearest,
        "-2.4691e-2565",
        "-0x1.790E-2130#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "1.2664e30",
        "0xf.fcE+24#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "1.2664e30",
        "0xf.fcE+24#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Floor,
        "-1.2701e30",
        "-0x1.008E+25#10",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Ceiling,
        "-1.2677e30",
        "-0x1.000E+25#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Down,
        "-1.2677e30",
        "-0x1.000E+25#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Up,
        "-1.2701e30",
        "-0x1.008E+25#10",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Nearest,
        "-1.2677e30",
        "-0x1.000E+25#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "100000000",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test("100000000", 10, Ceiling, "Infinity", "Infinity", Greater);
    test(
        "100000000",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test("100000000", 10, Up, "Infinity", "Infinity", Greater);
    test("100000000", 10, Nearest, "Infinity", "Infinity", Greater);
    test(
        "200000001/2",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test("200000001/2", 10, Ceiling, "Infinity", "Infinity", Greater);
    test(
        "200000001/2",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test("200000001/2", 10, Up, "Infinity", "Infinity", Greater);
    test("200000001/2", 10, Nearest, "Infinity", "Infinity", Greater);
    test(
        "-200000001/2",
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test("-200000001/2", 10, Ceiling, "-0.0", "-0x0.0", Greater);
    test("-200000001/2", 10, Down, "-0.0", "-0x0.0", Greater);
    test(
        "-200000001/2",
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test("-200000001/2", 10, Nearest, "-0.0", "-0x0.0", Greater);
    test("-200000003/2", 10, Floor, "0.0", "0x0.0", Less);
    test(
        "-200000003/2",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("-200000003/2", 10, Down, "0.0", "0x0.0", Less);
    test(
        "-200000003/2",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    // - gamma_rational_prec_round_helper huge negative
    test("-200000003/2", 10, Nearest, "0.0", "0x0.0", Less);
    test("0", 100, Floor, "Infinity", "Infinity", Equal);
    test("0", 100, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 100, Down, "Infinity", "Infinity", Equal);
    test("0", 100, Up, "Infinity", "Infinity", Equal);
    test("0", 100, Nearest, "Infinity", "Infinity", Equal);
    test("0", 100, Exact, "Infinity", "Infinity", Equal);
    test(
        "1",
        100,
        Floor,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Down,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Exact,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Floor,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Down,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Exact,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Floor,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Ceiling,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Down,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Up,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Nearest,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Exact,
        "2.0000000000000000000000000000000",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Floor,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Ceiling,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Down,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Up,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Nearest,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Exact,
        "24.000000000000000000000000000000",
        "0x18.000000000000000000000000#100",
        Equal,
    );
    test(
        "25",
        100,
        Floor,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test(
        "25",
        100,
        Ceiling,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test(
        "25",
        100,
        Down,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test(
        "25",
        100,
        Up,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test(
        "25",
        100,
        Nearest,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test(
        "25",
        100,
        Exact,
        "620448401733239439360000.00000000",
        "0x83629343d3dcd1c00000.00000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Floor,
        "1.7724538509055160272981674833397",
        "0x1.c5bf891b4ef6aa79c3b0520d4#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "1.7724538509055160272981674833397",
        "0x1.c5bf891b4ef6aa79c3b0520d4#100",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.7724538509055160272981674833413",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Floor,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "2.6789385347077476336556929409768",
        "0x2.adceea72905e2ceec8d3e92d0#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "2.6789385347077476336556929409768",
        "0x2.adceea72905e2ceec8d3e92d0#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "2.6789385347077476336556929409736",
        "0x2.adceea72905e2ceec8d3e92cc#100",
        Less,
    );
    test(
        "1/4",
        100,
        Floor,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test(
        "1/4",
        100,
        Ceiling,
        "3.6256099082219083119306851558705",
        "0x3.a027f88fdddd479b38a50a518#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Down,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test(
        "1/4",
        100,
        Up,
        "3.6256099082219083119306851558705",
        "0x3.a027f88fdddd479b38a50a518#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Nearest,
        "3.6256099082219083119306851558673",
        "0x3.a027f88fdddd479b38a50a514#100",
        Less,
    );
    test(
        "1/6",
        100,
        Floor,
        "5.5663160017802352042500968952067",
        "0x5.90fa15e2d8fb86a4c6b7ee8e0#100",
        Less,
    );
    test(
        "1/6",
        100,
        Ceiling,
        "5.5663160017802352042500968952130",
        "0x5.90fa15e2d8fb86a4c6b7ee8e8#100",
        Greater,
    );
    test(
        "1/6",
        100,
        Down,
        "5.5663160017802352042500968952067",
        "0x5.90fa15e2d8fb86a4c6b7ee8e0#100",
        Less,
    );
    test(
        "1/6",
        100,
        Up,
        "5.5663160017802352042500968952130",
        "0x5.90fa15e2d8fb86a4c6b7ee8e8#100",
        Greater,
    );
    test(
        "1/6",
        100,
        Nearest,
        "5.5663160017802352042500968952067",
        "0x5.90fa15e2d8fb86a4c6b7ee8e0#100",
        Less,
    );
    test(
        "2/3",
        100,
        Floor,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702#100",
        Less,
    );
    test(
        "2/3",
        100,
        Ceiling,
        "1.3541179394264004169452880281555",
        "0x1.5aa77928c3678cab2f4feb704#100",
        Greater,
    );
    test(
        "2/3",
        100,
        Down,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702#100",
        Less,
    );
    test(
        "2/3",
        100,
        Up,
        "1.3541179394264004169452880281555",
        "0x1.5aa77928c3678cab2f4feb704#100",
        Greater,
    );
    test(
        "2/3",
        100,
        Nearest,
        "1.3541179394264004169452880281540",
        "0x1.5aa77928c3678cab2f4feb702#100",
        Less,
    );
    test(
        "-1/2",
        100,
        Floor,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac#100",
        Less,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-3.5449077018110320545963349666794",
        "-0x3.8b7f12369ded54f38760a41a8#100",
        Greater,
    );
    test(
        "-1/2",
        100,
        Down,
        "-3.5449077018110320545963349666794",
        "-0x3.8b7f12369ded54f38760a41a8#100",
        Greater,
    );
    test(
        "-1/2",
        100,
        Up,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac#100",
        Less,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-3.5449077018110320545963349666825",
        "-0x3.8b7f12369ded54f38760a41ac#100",
        Less,
    );
    // - gamma_rational_prec_round_helper huge negative integer or huge denominator
    test("-67108864", 3, Up, "NaN", "NaN", Equal);
    // - gamma_rational_prec_round_helper huge positive
    test("100663295", 2, Up, "Infinity", "Infinity", Greater);
}

#[test]
#[should_panic]
fn gamma_rational_prec_round_fail_1() {
    Float::gamma_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn gamma_rational_prec_round_fail_2() {
    Float::gamma_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn gamma_rational_prec_round_ref_fail() {
    Float::gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn gamma_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (gamma, o) = Float::gamma_rational_prec_round(x.clone(), prec, rm);
    assert!(gamma.is_valid());
    assert_rounding_ordering_consistent(&gamma, rm, o);

    let (gamma_alt, o_alt) = Float::gamma_rational_prec_round_ref(&x, prec, rm);
    assert!(gamma_alt.is_valid());
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    // MPFR's gamma function at a `Float` near x; see `rug_gamma_rational_prec_round`. The rounding
    // error of that `Float` matters less than the extra precision when x is not large and not
    // within 2^-64 of an integer.
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && x.lt_abs(&0x10000u32)
        && (&x - Rational::from(Integer::rounding_from(&x, Nearest).0))
            .ge_abs(&Rational::power_of_2(-64i64))
    {
        let (rug_gamma, rug_o) = rug_gamma_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_gamma)),
            ComparableFloatRef(&gamma)
        );
        assert_eq!(rug_o, o);
    }

    if gamma.is_normal() {
        assert_eq!(gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The gamma function is positive on the positive reals.
        assert!(gamma > 0u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::gamma_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(gamma.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::gamma_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn gamma_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        gamma_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gamma_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (gamma, o) = Float::gamma_rational_prec(x.clone(), prec);
    assert!(gamma.is_valid());

    let (gamma_alt, o_alt) = Float::gamma_rational_prec_ref(&x, prec);
    assert!(gamma_alt.is_valid());
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);

    let (gamma_alt, o_alt) = Float::gamma_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&gamma_alt), ComparableFloatRef(&gamma));
    assert_eq!(o_alt, o);
}

#[test]
fn gamma_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        gamma_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_gamma_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_gamma_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::INFINITY);
    test::<f32>("1", 1.0);
    test::<f32>("2", 1.0);
    test::<f32>("3", 2.0);
    test::<f32>("5", 24.0);
    test::<f32>("25", 6.204484e23);
    test::<f32>("1/2", 1.7724539);
    test::<f32>("1/3", 2.6789386);
    test::<f32>("1/4", 3.6256099);
    test::<f32>("1/6", 5.566316);
    test::<f32>("2/3", 1.354118);
    test::<f32>("-1/2", -3.5449078);
    test::<f32>("-1", f32::NAN);
    test::<f32>("-2", f32::NAN);
    test::<f32>("-5/2", -0.94530874);
    test::<f32>("-7/3", -1.3057566);
    test::<f32>("22/7", 2.2908676);
    test::<f32>("1000001/1000", f32::INFINITY);
    test::<f32>("-1000001/1000", -0.0);
    test::<f32>("1/1267650600228229401496703205376", 1.2676506e30);
    test::<f32>("-1/1267650600228229401496703205376", -1.2676506e30);
    test::<f32>("35", 2.952328e38);
    test::<f32>("36", f32::INFINITY);
    test::<f32>("171", f32::INFINITY);
    test::<f32>("172", f32::INFINITY);
    test::<f64>("0", f64::INFINITY);
    test::<f64>("1", 1.0);
    test::<f64>("2", 1.0);
    test::<f64>("3", 2.0);
    test::<f64>("5", 24.0);
    test::<f64>("25", 6.204484017332394e23);
    test::<f64>("1/2", 1.772453850905516);
    test::<f64>("1/3", 2.6789385347077475);
    test::<f64>("1/4", 3.625609908221908);
    test::<f64>("1/6", 5.566316001780235);
    test::<f64>("2/3", 1.3541179394264005);
    test::<f64>("-1/2", -3.544907701811032);
    test::<f64>("-1", f64::NAN);
    test::<f64>("-2", f64::NAN);
    test::<f64>("-5/2", -0.9453087204829419);
    test::<f64>("-7/3", -1.3057565844468861);
    test::<f64>("22/7", 2.290867500961338);
    test::<f64>("1000001/1000", f64::INFINITY);
    test::<f64>("-1000001/1000", -0.0);
    test::<f64>("1/1267650600228229401496703205376", 1.2676506002282294e30);
    test::<f64>("-1/1267650600228229401496703205376", -1.2676506002282294e30);
    test::<f64>("35", 2.9523279903960416e38);
    test::<f64>("36", 1.0333147966386145e40);
    test::<f64>("171", 7.257415615307999e306);
    test::<f64>("172", f64::INFINITY);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_gamma_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        primitive_float_gamma_rational::<T>(&x);
    });
}

#[test]
fn primitive_float_gamma_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_gamma_rational_properties_helper);
}
