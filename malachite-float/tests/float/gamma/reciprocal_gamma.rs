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
use malachite_base::num::gamma::traits::{ReciprocalGamma, ReciprocalGammaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::gamma::reciprocal_gamma::{
    primitive_float_reciprocal_gamma, primitive_float_reciprocal_gamma_rational,
};
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, reciprocal_gamma_prec_round_naive,
    reciprocal_gamma_rational_prec_round_naive,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, exponent_in_gate, parse_hex_string, to_hex_string,
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

// Whether a result did not overflow or underflow, so that it is the correct rounding of the exact
// value with an unbounded exponent.
fn result_in_range(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

#[test]
fn test_reciprocal_gamma() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let reciprocal_gamma = x.clone().reciprocal_gamma();
        assert!(reciprocal_gamma.is_valid());

        assert_eq!(reciprocal_gamma.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_gamma), out_hex);

        let reciprocal_gamma_alt = (&x).reciprocal_gamma();
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );

        let mut reciprocal_gamma_alt = x.clone();
        reciprocal_gamma_alt.reciprocal_gamma_assign();
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "0.0", "0x0.0");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("2.0", "0x2.0#1", "1.0", "0x1.0#1");
    test("3.0", "0x3.0#2", "0.50", "0x0.8#2");
    test("4.0", "0x4.0#1", "0.12", "0x0.2#1");
    test("5.0", "0x5.0#3", "0.039", "0x0.0a#3");
    test("10.0", "0xa.0#3", "2.9e-6", "0x0.000030#3");
    test("25.0", "0x19.0#5", "1.60e-24", "0x1.fE-20#5");
    test("171.0", "0xab.0#8", "1.377e-307", "0x1.8cE-255#8");
    test("0.50", "0x0.8#1", "0.50", "0x0.8#1");
    test("1.5", "0x1.8#2", "1.0", "0x1.0#2");
    test("0.25", "0x0.4#1", "0.25", "0x0.4#1");
    test("0.75", "0x0.c#2", "0.75", "0x0.c#2");
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        "0.10511370061117778",
        "0x0.1ae8bb427c8e22#52",
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        "0.37265635737367964",
        "0x0.5f666833910260#52",
    );
    test("-0.50", "-0x0.8#1", "-0.25", "-0x0.4#1");
    test("-1.5", "-0x1.8#2", "0.38", "0x0.6#2");
    test("-2.5", "-0x2.8#3", "-1.0", "-0x1.0#3");
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        "-0.093577872091287290",
        "-0x0.17f4b82c42e7d6#52",
    );
    test("-1.0", "-0x1.0#1", "0.0", "0x0.0");
    test("-2.0", "-0x2.0#1", "0.0", "0x0.0");
    test("-100.5", "-0x64.8#8", "-2.986e158", "-0x5.70E+131#8");
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        "1.0000000577215600",
        "0x1.000000f7e986c#53",
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        "1.0000000422784312",
        "0x1.000000b595a07#53",
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        "-2.0000001812836883e-7",
        "-0x3.5afe5871df4a6E-6#53",
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        "0.43705571736475047504248119570299",
        "0x0.6fe2e22c9c8630eb26eaf052e0#100",
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8#100",
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        "-0.765839523163190842884",
        "-0x0.c40e0f19f85b09ec#64",
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        "1.0000000000577216e-10",
        "0x6.df37f677a3908E-9#53",
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        "-9.9999999994227847e-11",
        "-0x6.df37f6743b4d0E-9#53",
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
    );
    test("1000.5", "0x3e8.8#11", "7.8592e-2567", "0xc.00E-2132#11");
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        "3.5407e-456569",
        "0x1.5a4E-379172#12",
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        "1.2100777e-5565706",
        "0x2.36746E-4622219#21",
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        "-2.6304912e5565711",
        "-0x2.31ef0E+4622223#21",
    );
    test("6.7e7", "0x4.0E+6#1", "0.0", "0x0.0");
    test("67108864.50", "0x4000000.8#28", "0.0", "0x0.0");
    test("-67108864.50", "-0x4000000.8#28", "-Infinity", "-Infinity");
    test("-67108865.50", "-0x4000001.8#28", "Infinity", "Infinity");
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        "0.0",
        "0x0.0",
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        "0.0",
        "0x0.0",
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        "1.0e-301030",
        "0x1.0E-250000#1",
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        "-1.0e-301030",
        "-0x1.0E-250000#1",
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
    );
    test("2.0965e323228496", "0x7.feE+268435455#10", "0.0", "0x0.0");
}

#[test]
fn test_reciprocal_gamma_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal_gamma, o) = x.clone().reciprocal_gamma_prec(prec);
        assert!(reciprocal_gamma.is_valid());

        assert_eq!(reciprocal_gamma.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_gamma), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_prec_ref(prec);
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_gamma_alt = x.clone();
        let o_alt = reciprocal_gamma_alt.reciprocal_gamma_prec_assign(prec);
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0", Equal);
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
    test("3.0", "0x3.0#2", 1, "0.50", "0x0.8#1", Equal);
    test("3.0", "0x3.0#2", 10, "0.50000", "0x0.800#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, "0.12", "0x0.2#1", Less);
    test("4.0", "0x4.0#1", 10, "0.16675", "0x0.2ab#10", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac#100",
        Greater,
    );
    test("5.0", "0x5.0#3", 1, "0.031", "0x0.08#1", Less);
    test("5.0", "0x5.0#3", 10, "0.041687", "0x0.0aac#10", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, "1.9e-6", "0x0.00002#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        10,
        "2.7567e-6",
        "0x0.00002e4#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        100,
        "2.7557319223985890652557319223986e-6",
        "0x0.00002e3bc74aad8e671f5583911ca0#100",
        Less,
    );
    test("25.0", "0x19.0#5", 1, "1.7e-24", "0x2.0E-20#1", Greater);
    test(
        "25.0",
        "0x19.0#5",
        10,
        "1.6124e-24",
        "0x1.f30E-20#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        100,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20#100",
        Greater,
    );
    test("171.0", "0xab.0#8", 1, "1.8e-307", "0x2.0E-255#1", Greater);
    test(
        "171.0",
        "0xab.0#8",
        10,
        "1.3785e-307",
        "0x1.8c8E-255#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        100,
        "1.3779009677917705867455934866037e-307",
        "0x1.8c53af9080a2c005d70793c8aE-255#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 10, "0.56445", "0x0.908#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, "1.0", "0x1.0#1", Less);
    test("1.5", "0x1.8#2", 10, "1.1289", "0x1.210#10", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, "0.25", "0x0.4#1", Less);
    test("0.25", "0x0.4#1", 10, "0.27588", "0x0.46a#10", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, "1.0", "0x1.0#1", Greater);
    test("0.75", "0x0.c#2", 10, "0.81641", "0x0.d10#10", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        "0.81604893909826298107708594735056",
        "0x0.d0e895515ccf1bee40557ccfc#100",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        "0.12",
        "0x0.2#1",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        "0.10510",
        "0x0.1ae8#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        100,
        "0.10511370061117778682794439847226",
        "0x0.1ae8bb427c8e2288d99c5d6c8c#100",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        1,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        "0.37256",
        "0x0.5f6#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        100,
        "0.37265635737367961585698616281040",
        "0x0.5f66683391025e849912df0768#100",
        Less,
    );
    test("-0.50", "-0x0.8#1", 1, "-0.25", "-0x0.4#1", Greater);
    test("-0.50", "-0x0.8#1", 10, "-0.28223", "-0x0.484#10", Less);
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8#100",
        Less,
    );
    test("-1.5", "-0x1.8#2", 1, "0.50", "0x0.8#1", Greater);
    test("-1.5", "-0x1.8#2", 10, "0.42334", "0x0.6c6#10", Greater);
    test(
        "-1.5",
        "-0x1.8#2",
        100,
        "0.42314218766081721521105958867053",
        "0x0.6c530be18fa48e6a155f67df90#100",
        Less,
    );
    test("-2.5", "-0x2.8#3", 1, "-1.0", "-0x1.0#1", Greater);
    test("-2.5", "-0x2.8#3", 10, "-1.0586", "-0x1.0f0#10", Less);
    test(
        "-2.5",
        "-0x2.8#3",
        100,
        "-1.0578554691520430380276489716759",
        "-0x1.0ecf9db3e71b6409356e83aee#100",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        "-0.062",
        "-0x0.1#1",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        "-0.093628",
        "-0x0.17f8#10",
        Less,
    );
    test("-1.0", "-0x1.0#1", 1, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 1, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, "0.0", "0x0.0", Equal);
    test(
        "-100.5",
        "-0x64.8#8",
        1,
        "-2.2e158",
        "-0x4.0E+131#1",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        "-2.9819e158",
        "-0x5.6eE+131#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        1,
        "-2.4e-7",
        "-0x4.0E-6#1",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        "-2.0000e-7",
        "-0x3.5bE-6#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        1,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        "0.43701",
        "0x0.6fe#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        "0.37305",
        "0x0.5f8#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        "-0.76562",
        "-0x0.c40#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        "1.2e-10",
        "0x8.0E-9#1",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        1,
        "-1.2e-10",
        "-0x8.0E-9#1",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        "-1.0004e-10",
        "-0x6.e0E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        1,
        "7.5e-301",
        "0x8.0E-250#1",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        "1.0003e-300",
        "0xa.b8E-250#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        1,
        "-7.5e-301",
        "-0x8.0E-250#1",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        "-1.0003e-300",
        "-0xa.b8E-250#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        1,
        "1.0e-2566",
        "0x1.0E-2131#1",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        "7.8592e-2567",
        "0xc.00E-2132#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        1,
        "2.6e-456569",
        "0x1.0E-379172#1",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        "3.5432e-456569",
        "0x1.5a8E-379172#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        1,
        "1.1e-5565706",
        "0x2.0E-4622219#1",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        "1.2091e-5565706",
        "0x2.36E-4622219#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        1,
        "-2.4e5565711",
        "-0x2.0E+4622223#1",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        "-2.6308e5565711",
        "-0x2.32E+4622223#10",
        Less,
    );
    test("6.7e7", "0x4.0E+6#1", 1, "0.0", "0x0.0", Less);
    test("6.7e7", "0x4.0E+6#1", 10, "0.0", "0x0.0", Less);
    test("67108864.50", "0x4000000.8#28", 1, "0.0", "0x0.0", Less);
    test("67108864.50", "0x4000000.8#28", 10, "0.0", "0x0.0", Less);
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        1,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        1,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        1,
        "1.0e-301030",
        "0x1.0E-250000#1",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        "1.0100e-301030",
        "0x1.000E-250000#10",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        1,
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        "-1.0100e-301030",
        "-0x1.000E-250000#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        "0.0",
        "0x0.0",
        Less,
    );
}

#[test]
#[should_panic]
fn reciprocal_gamma_prec_fail() {
    Float::NAN.reciprocal_gamma_prec(0);
}

#[test]
fn test_reciprocal_gamma_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal_gamma, o) = x.clone().reciprocal_gamma_round(rm);
        assert!(reciprocal_gamma.is_valid());

        assert_eq!(reciprocal_gamma.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_gamma), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_round_ref(rm);
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_gamma_alt = x.clone();
        let o_alt = reciprocal_gamma_alt.reciprocal_gamma_round_assign(rm);
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", Exact, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Down, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Up, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Nearest, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Exact, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Exact, "-0.0", "-0x0.0", Equal);
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
    test("3.0", "0x3.0#2", Floor, "0.50", "0x0.8#2", Equal);
    test("3.0", "0x3.0#2", Ceiling, "0.50", "0x0.8#2", Equal);
    test("3.0", "0x3.0#2", Down, "0.50", "0x0.8#2", Equal);
    test("3.0", "0x3.0#2", Up, "0.50", "0x0.8#2", Equal);
    test("3.0", "0x3.0#2", Nearest, "0.50", "0x0.8#2", Equal);
    test("3.0", "0x3.0#2", Exact, "0.50", "0x0.8#2", Equal);
    test("4.0", "0x4.0#1", Floor, "0.12", "0x0.2#1", Less);
    test("4.0", "0x4.0#1", Ceiling, "0.25", "0x0.4#1", Greater);
    test("4.0", "0x4.0#1", Down, "0.12", "0x0.2#1", Less);
    test("4.0", "0x4.0#1", Up, "0.25", "0x0.4#1", Greater);
    test("4.0", "0x4.0#1", Nearest, "0.12", "0x0.2#1", Less);
    test("5.0", "0x5.0#3", Floor, "0.039", "0x0.0a#3", Less);
    test("5.0", "0x5.0#3", Ceiling, "0.047", "0x0.0c#3", Greater);
    test("5.0", "0x5.0#3", Down, "0.039", "0x0.0a#3", Less);
    test("5.0", "0x5.0#3", Up, "0.047", "0x0.0c#3", Greater);
    test("5.0", "0x5.0#3", Nearest, "0.039", "0x0.0a#3", Less);
    test("10.0", "0xa.0#3", Floor, "2.4e-6", "0x0.000028#3", Less);
    test(
        "10.0",
        "0xa.0#3",
        Ceiling,
        "2.9e-6",
        "0x0.000030#3",
        Greater,
    );
    test("10.0", "0xa.0#3", Down, "2.4e-6", "0x0.000028#3", Less);
    test("10.0", "0xa.0#3", Up, "2.9e-6", "0x0.000030#3", Greater);
    test(
        "10.0",
        "0xa.0#3",
        Nearest,
        "2.9e-6",
        "0x0.000030#3",
        Greater,
    );
    test("25.0", "0x19.0#5", Floor, "1.60e-24", "0x1.fE-20#5", Less);
    test(
        "25.0",
        "0x19.0#5",
        Ceiling,
        "1.65e-24",
        "0x2.0E-20#5",
        Greater,
    );
    test("25.0", "0x19.0#5", Down, "1.60e-24", "0x1.fE-20#5", Less);
    test("25.0", "0x19.0#5", Up, "1.65e-24", "0x2.0E-20#5", Greater);
    test("25.0", "0x19.0#5", Nearest, "1.60e-24", "0x1.fE-20#5", Less);
    test(
        "171.0",
        "0xab.0#8",
        Floor,
        "1.377e-307",
        "0x1.8cE-255#8",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        Ceiling,
        "1.384e-307",
        "0x1.8eE-255#8",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        Down,
        "1.377e-307",
        "0x1.8cE-255#8",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        Up,
        "1.384e-307",
        "0x1.8eE-255#8",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        Nearest,
        "1.377e-307",
        "0x1.8cE-255#8",
        Less,
    );
    test("0.50", "0x0.8#1", Floor, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Down, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Up, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Nearest, "0.50", "0x0.8#1", Less);
    test("1.5", "0x1.8#2", Floor, "1.0", "0x1.0#2", Less);
    test("1.5", "0x1.8#2", Ceiling, "1.5", "0x1.8#2", Greater);
    test("1.5", "0x1.8#2", Down, "1.0", "0x1.0#2", Less);
    test("1.5", "0x1.8#2", Up, "1.5", "0x1.8#2", Greater);
    test("1.5", "0x1.8#2", Nearest, "1.0", "0x1.0#2", Less);
    test("0.25", "0x0.4#1", Floor, "0.25", "0x0.4#1", Less);
    test("0.25", "0x0.4#1", Ceiling, "0.50", "0x0.8#1", Greater);
    test("0.25", "0x0.4#1", Down, "0.25", "0x0.4#1", Less);
    test("0.25", "0x0.4#1", Up, "0.50", "0x0.8#1", Greater);
    test("0.25", "0x0.4#1", Nearest, "0.25", "0x0.4#1", Less);
    test("0.75", "0x0.c#2", Floor, "0.75", "0x0.c#2", Less);
    test("0.75", "0x0.c#2", Ceiling, "1.0", "0x1.0#2", Greater);
    test("0.75", "0x0.c#2", Down, "0.75", "0x0.c#2", Less);
    test("0.75", "0x0.c#2", Up, "1.0", "0x1.0#2", Greater);
    test("0.75", "0x0.c#2", Nearest, "0.75", "0x0.c#2", Less);
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Floor,
        "0.10511370061117778",
        "0x0.1ae8bb427c8e22#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Ceiling,
        "0.10511370061117781",
        "0x0.1ae8bb427c8e24#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Down,
        "0.10511370061117778",
        "0x0.1ae8bb427c8e22#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Up,
        "0.10511370061117781",
        "0x0.1ae8bb427c8e24#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Nearest,
        "0.10511370061117778",
        "0x0.1ae8bb427c8e22#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Floor,
        "0.37265635737367953",
        "0x0.5f666833910258#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Ceiling,
        "0.37265635737367964",
        "0x0.5f666833910260#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Down,
        "0.37265635737367953",
        "0x0.5f666833910258#52",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Up,
        "0.37265635737367964",
        "0x0.5f666833910260#52",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        Nearest,
        "0.37265635737367964",
        "0x0.5f666833910260#52",
        Greater,
    );
    test("-0.50", "-0x0.8#1", Floor, "-0.50", "-0x0.8#1", Less);
    test("-0.50", "-0x0.8#1", Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-0.50", "-0x0.8#1", Down, "-0.25", "-0x0.4#1", Greater);
    test("-0.50", "-0x0.8#1", Up, "-0.50", "-0x0.8#1", Less);
    test("-0.50", "-0x0.8#1", Nearest, "-0.25", "-0x0.4#1", Greater);
    test("-1.5", "-0x1.8#2", Floor, "0.38", "0x0.6#2", Less);
    test("-1.5", "-0x1.8#2", Ceiling, "0.50", "0x0.8#2", Greater);
    test("-1.5", "-0x1.8#2", Down, "0.38", "0x0.6#2", Less);
    test("-1.5", "-0x1.8#2", Up, "0.50", "0x0.8#2", Greater);
    test("-1.5", "-0x1.8#2", Nearest, "0.38", "0x0.6#2", Less);
    test("-2.5", "-0x2.8#3", Floor, "-1.2", "-0x1.4#3", Less);
    test("-2.5", "-0x2.8#3", Ceiling, "-1.0", "-0x1.0#3", Greater);
    test("-2.5", "-0x2.8#3", Down, "-1.0", "-0x1.0#3", Greater);
    test("-2.5", "-0x2.8#3", Up, "-1.2", "-0x1.4#3", Less);
    test("-2.5", "-0x2.8#3", Nearest, "-1.0", "-0x1.0#3", Greater);
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Floor,
        "-0.093577872091287290",
        "-0x0.17f4b82c42e7d6#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Ceiling,
        "-0.093577872091287262",
        "-0x0.17f4b82c42e7d4#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Down,
        "-0.093577872091287262",
        "-0x0.17f4b82c42e7d4#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Up,
        "-0.093577872091287290",
        "-0x0.17f4b82c42e7d6#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Nearest,
        "-0.093577872091287290",
        "-0x0.17f4b82c42e7d6#52",
        Less,
    );
    test("-1.0", "-0x1.0#1", Floor, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Down, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Up, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", Floor, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", Ceiling, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", Down, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", Up, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", Nearest, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", Exact, "0.0", "0x0.0", Equal);
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Floor,
        "1.0000000000577215e-10",
        "0x6.df37f677a3904E-9#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Ceiling,
        "1.0000000000577216e-10",
        "0x6.df37f677a3908E-9#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Down,
        "1.0000000000577215e-10",
        "0x6.df37f677a3904E-9#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Up,
        "1.0000000000577216e-10",
        "0x6.df37f677a3908E-9#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        Nearest,
        "1.0000000000577216e-10",
        "0x6.df37f677a3908E-9#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Floor,
        "-9.9999999994227860e-11",
        "-0x6.df37f6743b4d4E-9#53",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Ceiling,
        "-9.9999999994227847e-11",
        "-0x6.df37f6743b4d0E-9#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Down,
        "-9.9999999994227847e-11",
        "-0x6.df37f6743b4d0E-9#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Up,
        "-9.9999999994227860e-11",
        "-0x6.df37f6743b4d4E-9#53",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        Nearest,
        "-9.9999999994227847e-11",
        "-0x6.df37f6743b4d0E-9#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Floor,
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Ceiling,
        "1.0000000000000002e-300",
        "0xa.b70fe17c79ad0E-250#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Down,
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Up,
        "1.0000000000000002e-300",
        "0xa.b70fe17c79ad0E-250#53",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Nearest,
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Floor,
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Ceiling,
        "-9.9999999999999986e-301",
        "-0xa.b70fe17c79ac0E-250#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Down,
        "-9.9999999999999986e-301",
        "-0xa.b70fe17c79ac0E-250#53",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Up,
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Nearest,
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        Less,
    );
}

#[test]
#[should_panic]
fn reciprocal_gamma_round_fail() {
    Float::from(0.5).reciprocal_gamma_round(Exact);
}

#[test]
fn test_reciprocal_gamma_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal_gamma, o) = x.clone().reciprocal_gamma_prec_round(prec, rm);
        assert!(reciprocal_gamma.is_valid());

        assert_eq!(reciprocal_gamma.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_gamma), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_prec_round_ref(prec, rm);
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_gamma_alt = x.clone();
        let o_alt = reciprocal_gamma_alt.reciprocal_gamma_prec_round_assign(prec, rm);
        assert!(reciprocal_gamma_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_gamma),
            ComparableFloatRef(&reciprocal_gamma_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    // The expected values agree with Azurite's reciprocal gamma function.
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Down, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Up, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Nearest, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Exact, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "-0.0", "-0x0.0", Equal);
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
    test("3.0", "0x3.0#2", 10, Floor, "0.50000", "0x0.800#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test("3.0", "0x3.0#2", 10, Down, "0.50000", "0x0.800#10", Equal);
    test("3.0", "0x3.0#2", 10, Up, "0.50000", "0x0.800#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test("3.0", "0x3.0#2", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("4.0", "0x4.0#1", 10, Floor, "0.16650", "0x0.2aa#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        10,
        Ceiling,
        "0.16675",
        "0x0.2ab#10",
        Greater,
    );
    test("4.0", "0x4.0#1", 10, Down, "0.16650", "0x0.2aa#10", Less);
    test("4.0", "0x4.0#1", 10, Up, "0.16675", "0x0.2ab#10", Greater);
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "0.16675",
        "0x0.2ab#10",
        Greater,
    );
    test("5.0", "0x5.0#3", 10, Floor, "0.041626", "0x0.0aa8#10", Less);
    test(
        "5.0",
        "0x5.0#3",
        10,
        Ceiling,
        "0.041687",
        "0x0.0aac#10",
        Greater,
    );
    test("5.0", "0x5.0#3", 10, Down, "0.041626", "0x0.0aa8#10", Less);
    test("5.0", "0x5.0#3", 10, Up, "0.041687", "0x0.0aac#10", Greater);
    test(
        "5.0",
        "0x5.0#3",
        10,
        Nearest,
        "0.041687",
        "0x0.0aac#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Floor,
        "2.7530e-6",
        "0x0.00002e3#10",
        Less,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Ceiling,
        "2.7567e-6",
        "0x0.00002e4#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Down,
        "2.7530e-6",
        "0x0.00002e3#10",
        Less,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Up,
        "2.7567e-6",
        "0x0.00002e4#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Nearest,
        "2.7567e-6",
        "0x0.00002e4#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Floor,
        "1.6107e-24",
        "0x1.f28E-20#10",
        Less,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Ceiling,
        "1.6124e-24",
        "0x1.f30E-20#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Down,
        "1.6107e-24",
        "0x1.f28E-20#10",
        Less,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Up,
        "1.6124e-24",
        "0x1.f30E-20#10",
        Greater,
    );
    test(
        "25.0",
        "0x19.0#5",
        10,
        Nearest,
        "1.6124e-24",
        "0x1.f30E-20#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Floor,
        "1.3768e-307",
        "0x1.8c0E-255#10",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Ceiling,
        "1.3785e-307",
        "0x1.8c8E-255#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Down,
        "1.3768e-307",
        "0x1.8c0E-255#10",
        Less,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Up,
        "1.3785e-307",
        "0x1.8c8E-255#10",
        Greater,
    );
    test(
        "171.0",
        "0xab.0#8",
        10,
        Nearest,
        "1.3785e-307",
        "0x1.8c8E-255#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Floor, "0.56348", "0x0.904#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.56445",
        "0x0.908#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Down, "0.56348", "0x0.904#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.56445", "0x0.908#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.56445",
        "0x0.908#10",
        Greater,
    );
    test("1.5", "0x1.8#2", 10, Floor, "1.1270", "0x1.208#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "1.1289",
        "0x1.210#10",
        Greater,
    );
    test("1.5", "0x1.8#2", 10, Down, "1.1270", "0x1.208#10", Less);
    test("1.5", "0x1.8#2", 10, Up, "1.1289", "0x1.210#10", Greater);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "1.1289",
        "0x1.210#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Floor, "0.27539", "0x0.468#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "0.27588",
        "0x0.46a#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Down, "0.27539", "0x0.468#10", Less);
    test("0.25", "0x0.4#1", 10, Up, "0.27588", "0x0.46a#10", Greater);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Nearest,
        "0.27588",
        "0x0.46a#10",
        Greater,
    );
    test("0.75", "0x0.c#2", 10, Floor, "0.81543", "0x0.d0c#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "0.81641",
        "0x0.d10#10",
        Greater,
    );
    test("0.75", "0x0.c#2", 10, Down, "0.81543", "0x0.d0c#10", Less);
    test("0.75", "0x0.c#2", 10, Up, "0.81641", "0x0.d10#10", Greater);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Nearest,
        "0.81641",
        "0x0.d10#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "0.10510",
        "0x0.1ae8#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "0.10522",
        "0x0.1af0#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Down,
        "0.10510",
        "0x0.1ae8#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Up,
        "0.10522",
        "0x0.1af0#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "0.10510",
        "0x0.1ae8#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Floor,
        "0.37256",
        "0x0.5f6#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Ceiling,
        "0.37305",
        "0x0.5f8#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Down,
        "0.37256",
        "0x0.5f6#10",
        Less,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Up,
        "0.37305",
        "0x0.5f8#10",
        Greater,
    );
    test(
        "3.2999999999999998",
        "0x3.4cccccccccccc#52",
        10,
        Nearest,
        "0.37256",
        "0x0.5f6#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Floor,
        "-0.28223",
        "-0x0.484#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Ceiling,
        "-0.28174",
        "-0x0.482#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Down,
        "-0.28174",
        "-0x0.482#10",
        Greater,
    );
    test("-0.50", "-0x0.8#1", 10, Up, "-0.28223", "-0x0.484#10", Less);
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "-0.28223",
        "-0x0.484#10",
        Less,
    );
    test("-1.5", "-0x1.8#2", 10, Floor, "0.42285", "0x0.6c4#10", Less);
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Ceiling,
        "0.42334",
        "0x0.6c6#10",
        Greater,
    );
    test("-1.5", "-0x1.8#2", 10, Down, "0.42285", "0x0.6c4#10", Less);
    test("-1.5", "-0x1.8#2", 10, Up, "0.42334", "0x0.6c6#10", Greater);
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Nearest,
        "0.42334",
        "0x0.6c6#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-1.0586",
        "-0x1.0f0#10",
        Less,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Ceiling,
        "-1.0566",
        "-0x1.0e8#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Down,
        "-1.0566",
        "-0x1.0e8#10",
        Greater,
    );
    test("-2.5", "-0x2.8#3", 10, Up, "-1.0586", "-0x1.0f0#10", Less);
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Nearest,
        "-1.0586",
        "-0x1.0f0#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-0.093628",
        "-0x0.17f8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-0.093506",
        "-0x0.17f0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Down,
        "-0.093506",
        "-0x0.17f0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "-0.093628",
        "-0x0.17f8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-0.093628",
        "-0x0.17f8#10",
        Less,
    );
    test("-1.0", "-0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Down, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Up, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, Down, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, Up, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Floor,
        "-2.9819e158",
        "-0x5.6eE+131#10",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Ceiling,
        "-2.9776e158",
        "-0x5.6cE+131#10",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Down,
        "-2.9776e158",
        "-0x5.6cE+131#10",
        Greater,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Up,
        "-2.9819e158",
        "-0x5.6eE+131#10",
        Less,
    );
    test(
        "-100.5",
        "-0x64.8#8",
        10,
        Nearest,
        "-2.9819e158",
        "-0x5.6eE+131#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.0000001000000001",
        "0x1.000001ad7f29b#53",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.9999998999999999",
        "0x1.fffffe5280d65#53",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Floor,
        "-2.0000e-7",
        "-0x3.5bE-6#10",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Ceiling,
        "-1.9977e-7",
        "-0x3.5aE-6#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Down,
        "-1.9977e-7",
        "-0x3.5aE-6#10",
        Greater,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Up,
        "-2.0000e-7",
        "-0x3.5bE-6#10",
        Less,
    );
    test(
        "-2.0000000999999998",
        "-0x2.000001ad7f29a#53",
        10,
        Nearest,
        "-2.0000e-7",
        "-0x3.5bE-6#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Floor,
        "0.43701",
        "0x0.6fe#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Ceiling,
        "0.43750",
        "0x0.700#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Down,
        "0.43701",
        "0x0.6fe#10",
        Less,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Up,
        "0.43750",
        "0x0.700#10",
        Greater,
    );
    test(
        "3.1415926535897932384626433832793",
        "0x3.243f6a8885a308d313198a2e0#100",
        10,
        Nearest,
        "0.43701",
        "0x0.6fe#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "0.37305",
        "0x0.5f8#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "0.37354",
        "0x0.5fa#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Down,
        "0.37305",
        "0x0.5f8#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Up,
        "0.37354",
        "0x0.5fa#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "0.37305",
        "0x0.5f8#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Floor,
        "-0.76660",
        "-0x0.c44#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Ceiling,
        "-0.76562",
        "-0x0.c40#10",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Down,
        "-0.76562",
        "-0x0.c40#10",
        Greater,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Up,
        "-0.76660",
        "-0x0.c44#10",
        Less,
    );
    test(
        "-2.33333333333333333326",
        "-0x2.5555555555555554#64",
        10,
        Nearest,
        "-0.76562",
        "-0x0.c40#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "9.9931e-11",
        "0x6.deE-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "9.9931e-11",
        "0x6.deE-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Floor,
        "-1.0004e-10",
        "-0x6.e0E-9#10",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Ceiling,
        "-9.9931e-11",
        "-0x6.deE-9#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Down,
        "-9.9931e-11",
        "-0x6.deE-9#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Up,
        "-1.0004e-10",
        "-0x6.e0E-9#10",
        Less,
    );
    test(
        "-1.0000000000000000e-10",
        "-0x6.df37f675ef6ecE-9#53",
        10,
        Nearest,
        "-1.0004e-10",
        "-0x6.e0E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Floor,
        "9.9888e-301",
        "0xa.b4E-250#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Ceiling,
        "1.0003e-300",
        "0xa.b8E-250#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Down,
        "9.9888e-301",
        "0xa.b4E-250#10",
        Less,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Up,
        "1.0003e-300",
        "0xa.b8E-250#10",
        Greater,
    );
    test(
        "1.0000000000000000e-300",
        "0xa.b70fe17c79ac8E-250#53",
        10,
        Nearest,
        "1.0003e-300",
        "0xa.b8E-250#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Floor,
        "-1.0003e-300",
        "-0xa.b8E-250#10",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Ceiling,
        "-9.9888e-301",
        "-0xa.b4E-250#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Down,
        "-9.9888e-301",
        "-0xa.b4E-250#10",
        Greater,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Up,
        "-1.0003e-300",
        "-0xa.b8E-250#10",
        Less,
    );
    test(
        "-1.0000000000000000e-300",
        "-0xa.b70fe17c79ac8E-250#53",
        10,
        Nearest,
        "-1.0003e-300",
        "-0xa.b8E-250#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Floor,
        "7.8592e-2567",
        "0xc.00E-2132#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Ceiling,
        "7.8694e-2567",
        "0xc.04E-2132#10",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Down,
        "7.8592e-2567",
        "0xc.00E-2132#10",
        Less,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Up,
        "7.8694e-2567",
        "0xc.04E-2132#10",
        Greater,
    );
    test(
        "1000.5",
        "0x3e8.8#11",
        10,
        Nearest,
        "7.8592e-2567",
        "0xc.00E-2132#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Floor,
        "3.5381e-456569",
        "0x1.5a0E-379172#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Ceiling,
        "3.5432e-456569",
        "0x1.5a8E-379172#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Down,
        "3.5381e-456569",
        "0x1.5a0E-379172#10",
        Less,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Up,
        "3.5432e-456569",
        "0x1.5a8E-379172#10",
        Greater,
    );
    test(
        "1.0000e5",
        "0x1.86aE+4#12",
        10,
        Nearest,
        "3.5432e-456569",
        "0x1.5a8E-379172#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Floor,
        "1.2091e-5565706",
        "0x2.36E-4622219#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Ceiling,
        "1.2112e-5565706",
        "0x2.37E-4622219#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Down,
        "1.2091e-5565706",
        "0x2.36E-4622219#10",
        Less,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Up,
        "1.2112e-5565706",
        "0x2.37E-4622219#10",
        Greater,
    );
    test(
        "1000000.5",
        "0xf4240.8#21",
        10,
        Nearest,
        "1.2091e-5565706",
        "0x2.36E-4622219#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Floor,
        "-2.6308e5565711",
        "-0x2.32E+4622223#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Ceiling,
        "-2.6261e5565711",
        "-0x2.31E+4622223#10",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Down,
        "-2.6261e5565711",
        "-0x2.31E+4622223#10",
        Greater,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Up,
        "-2.6308e5565711",
        "-0x2.32E+4622223#10",
        Less,
    );
    test(
        "-1000000.5",
        "-0xf4240.8#21",
        10,
        Nearest,
        "-2.6308e5565711",
        "-0x2.32E+4622223#10",
        Less,
    );
    test("6.7e7", "0x4.0E+6#1", 10, Floor, "0.0", "0x0.0", Less);
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("6.7e7", "0x4.0E+6#1", 10, Down, "0.0", "0x0.0", Less);
    test(
        "6.7e7",
        "0x4.0E+6#1",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("6.7e7", "0x4.0E+6#1", 10, Nearest, "0.0", "0x0.0", Less);
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "67108864.50",
        "0x4000000.8#28",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Ceiling,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Down,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-67108864.50",
        "-0x4000000.8#28",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-67108865.50",
        "-0x4000001.8#28",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "1.0000000000000001e300",
        "0x1.7e43c8800759cE+249#51",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Down,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Up,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0000000000000001e300",
        "-0x1.7e43c8800759cE+249#51",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Floor,
        "1.0100e-301030",
        "0x1.000E-250000#10",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Ceiling,
        "1.0120e-301030",
        "0x1.008E-250000#10",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Down,
        "1.0100e-301030",
        "0x1.000E-250000#10",
        Less,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Up,
        "1.0120e-301030",
        "0x1.008E-250000#10",
        Greater,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        10,
        Nearest,
        "1.0100e-301030",
        "0x1.000E-250000#10",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Floor,
        "-1.0100e-301030",
        "-0x1.000E-250000#10",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Ceiling,
        "-1.0090e-301030",
        "-0xf.fcE-250001#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Down,
        "-1.0090e-301030",
        "-0xf.fcE-250001#10",
        Greater,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Up,
        "-1.0100e-301030",
        "-0x1.000E-250000#10",
        Less,
    );
    test(
        "-1.0e-301030",
        "-0x1.0E-250000#1",
        10,
        Nearest,
        "-1.0100e-301030",
        "-0x1.000E-250000#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Ceiling,
        "2.3872e-323228497",
        "0x1.008E-268435456#10",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Down,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Up,
        "2.3872e-323228497",
        "0x1.008E-268435456#10",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
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
    test("3.0", "0x3.0#2", 1, Floor, "0.50", "0x0.8#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Floor,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Ceiling, "0.50", "0x0.8#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Ceiling,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Down, "0.50", "0x0.8#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Down,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Up, "0.50", "0x0.8#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Up,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "0.50", "0x0.8#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Nearest,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("3.0", "0x3.0#2", 1, Exact, "0.50", "0x0.8#1", Equal);
    test(
        "3.0",
        "0x3.0#2",
        100,
        Exact,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("4.0", "0x4.0#1", 1, Floor, "0.12", "0x0.2#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Floor,
        "0.16666666666666666666666666666654",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaa8#100",
        Less,
    );
    test("4.0", "0x4.0#1", 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Ceiling,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac#100",
        Greater,
    );
    test("4.0", "0x4.0#1", 1, Down, "0.12", "0x0.2#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Down,
        "0.16666666666666666666666666666654",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaa8#100",
        Less,
    );
    test("4.0", "0x4.0#1", 1, Up, "0.25", "0x0.4#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Up,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac#100",
        Greater,
    );
    test("4.0", "0x4.0#1", 1, Nearest, "0.12", "0x0.2#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "0.16666666666666666666666666666673",
        "0x0.2aaaaaaaaaaaaaaaaaaaaaaaac#100",
        Greater,
    );
    test("5.0", "0x5.0#3", 1, Floor, "0.031", "0x0.08#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Floor,
        "0.041666666666666666666666666666634",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test("5.0", "0x5.0#3", 1, Ceiling, "0.062", "0x0.1#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Ceiling,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test("5.0", "0x5.0#3", 1, Down, "0.031", "0x0.08#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Down,
        "0.041666666666666666666666666666634",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test("5.0", "0x5.0#3", 1, Up, "0.062", "0x0.1#1", Greater);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Up,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test("5.0", "0x5.0#3", 1, Nearest, "0.031", "0x0.08#1", Less);
    test(
        "5.0",
        "0x5.0#3",
        100,
        Nearest,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, Floor, "1.9e-6", "0x0.00002#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Floor,
        "2.7557319223985890652557319223986e-6",
        "0x0.00002e3bc74aad8e671f5583911ca0#100",
        Less,
    );
    test(
        "10.0",
        "0xa.0#3",
        1,
        Ceiling,
        "3.8e-6",
        "0x0.00004#1",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        100,
        Ceiling,
        "2.7557319223985890652557319224016e-6",
        "0x0.00002e3bc74aad8e671f5583911ca4#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, Down, "1.9e-6", "0x0.00002#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Down,
        "2.7557319223985890652557319223986e-6",
        "0x0.00002e3bc74aad8e671f5583911ca0#100",
        Less,
    );
    test("10.0", "0xa.0#3", 1, Up, "3.8e-6", "0x0.00004#1", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Up,
        "2.7557319223985890652557319224016e-6",
        "0x0.00002e3bc74aad8e671f5583911ca4#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, Nearest, "1.9e-6", "0x0.00002#1", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        Nearest,
        "2.7557319223985890652557319223986e-6",
        "0x0.00002e3bc74aad8e671f5583911ca0#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "0.56418958354775628694807945156018",
        "0x0.906eba8214db688d71d48a7f6#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Down, "0.50", "0x0.8#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Down,
        "0.56418958354775628694807945156018",
        "0x0.906eba8214db688d71d48a7f6#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Up, "1.0", "0x1.0#1", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Up,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Floor, "1.0", "0x1.0#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Floor,
        "1.1283791670955125738961589031204",
        "0x1.20dd750429b6d11ae3a914fec#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Ceiling,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Down,
        "1.1283791670955125738961589031204",
        "0x1.20dd750429b6d11ae3a914fec#100",
        Less,
    );
    test("1.5", "0x1.8#2", 1, Up, "2.0", "0x2.0#1", Greater);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Up,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Nearest,
        "1.1283791670955125738961589031219",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Floor, "0.25", "0x0.4#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Floor,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Ceiling,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Down, "0.25", "0x0.4#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Down,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Up, "0.50", "0x0.8#1", Greater);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Up,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8#100",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Nearest, "0.25", "0x0.4#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        Nearest,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Floor,
        "0.81604893909826298107708594735056",
        "0x0.d0e895515ccf1bee40557ccfc#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Ceiling,
        "0.81604893909826298107708594735135",
        "0x0.d0e895515ccf1bee40557ccfd#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Down, "0.50", "0x0.8#1", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Down,
        "0.81604893909826298107708594735056",
        "0x0.d0e895515ccf1bee40557ccfc#100",
        Less,
    );
    test("0.75", "0x0.c#2", 1, Up, "1.0", "0x1.0#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Up,
        "0.81604893909826298107708594735135",
        "0x0.d0e895515ccf1bee40557ccfd#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Nearest,
        "0.81604893909826298107708594735056",
        "0x0.d0e895515ccf1bee40557ccfc#100",
        Less,
    );
    // - gamma_prec_round_helper huge positive
    test(
        "1.34e8",
        "0x8.0E+6#5",
        7,
        Up,
        "2.383e-323228497",
        "0x1.00E-268435456#7",
        Greater,
    );
    // - gamma_prec_round_helper huge negative integer
    test("-8.6e9", "-0x2.0E+8#1", 4, Up, "0.0", "0x0.0", Equal);
    // - gamma_prec_round_helper huge negative
    test(
        "-134283263.94",
        "-0x800ffff.f#32",
        9,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
}

#[test]
#[should_panic]
fn reciprocal_gamma_prec_round_fail_1() {
    Float::ONE.reciprocal_gamma_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn reciprocal_gamma_prec_round_fail_2() {
    Float::from(0.5).reciprocal_gamma_prec_round(10, Exact);
}

// Whether the reference, which takes twice as many bits as a tiny argument has leading zeros, and
// whose time grows with a large argument, is run on `x`.
fn reference_is_cheap(x: &Float) -> bool {
    x.get_exponent().is_some_and(|e| (-1000..=20).contains(&e))
}

// Checks an inexact result that is finite and nonzero and did not overflow or underflow against the
// reference, which computes the reciprocal of the gamma function.
fn check_rounding(x: &Float, reciprocal_gamma: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    if rm != Exact
        && o != Equal
        && reciprocal_gamma.is_normal()
        && result_in_range(reciprocal_gamma, o)
        && reference_is_cheap(x)
    {
        let (g, go) = reciprocal_gamma_prec_round_naive(&float_to_big_exponent_float(x), prec, rm);
        assert_eq!(float_to_big_exponent_float(reciprocal_gamma), g);
        assert_eq!(o, go);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_gamma_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (reciprocal_gamma, o) = x.clone().reciprocal_gamma_prec_round(prec, rm);
    assert!(reciprocal_gamma.is_valid());
    assert_rounding_ordering_consistent(&reciprocal_gamma, rm, o);

    let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_prec_round_ref(prec, rm);
    assert!(reciprocal_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_gamma_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &reciprocal_gamma, o, prec, rm);

    if reciprocal_gamma.is_normal() {
        assert_eq!(reciprocal_gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The gamma function is positive on the positive reals; its reciprocal may underflow.
        assert!(reciprocal_gamma >= 0u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.reciprocal_gamma_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s),
                ComparableFloat(reciprocal_gamma.clone())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.reciprocal_gamma_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) =
            Float::reciprocal_gamma_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&r),
            ComparableFloatRef(&reciprocal_gamma)
        );
        assert_eq!(ro, o);
    }
}

#[test]
fn reciprocal_gamma_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        reciprocal_gamma_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            reciprocal_gamma_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.reciprocal_gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.reciprocal_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.reciprocal_gamma_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.reciprocal_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.reciprocal_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        // The reciprocal gamma function is 0 at the poles of the gamma function.
        let (s, o) = Float::NEGATIVE_ONE.reciprocal_gamma_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::ONE.reciprocal_gamma_prec_round(prec, rm);
        assert_eq!(s, 1u32);
        assert_eq!(o, Equal);

        let (s, o) = Float::TWO.reciprocal_gamma_prec_round(prec, rm);
        assert_eq!(s, 1u32);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_gamma_prec_properties_helper(x: Float, prec: u64) {
    let (reciprocal_gamma, o) = x.clone().reciprocal_gamma_prec(prec);
    assert!(reciprocal_gamma.is_valid());

    let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_prec_ref(prec);
    assert!(reciprocal_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_gamma_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &reciprocal_gamma, o, prec, Nearest);

    if reciprocal_gamma.is_normal() {
        assert_eq!(reciprocal_gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The gamma function is positive on the positive reals; its reciprocal may underflow.
        assert!(reciprocal_gamma >= 0u32);
    }
}

#[test]
fn reciprocal_gamma_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        reciprocal_gamma_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        reciprocal_gamma_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        reciprocal_gamma_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.reciprocal_gamma_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_gamma_round_properties_helper(x: Float, rm: RoundingMode) {
    let (reciprocal_gamma, o) = x.clone().reciprocal_gamma_round(rm);
    assert!(reciprocal_gamma.is_valid());
    assert_rounding_ordering_consistent(&reciprocal_gamma, rm, o);

    let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_round_ref(rm);
    assert!(reciprocal_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_gamma_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    let (reciprocal_gamma_alt, o_alt) = x.reciprocal_gamma_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &reciprocal_gamma, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.reciprocal_gamma_round_ref(rm);
            assert_eq!(
                ComparableFloat(s),
                ComparableFloat(reciprocal_gamma.clone())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.reciprocal_gamma_round_ref(Exact));
    }
}

#[test]
fn reciprocal_gamma_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        reciprocal_gamma_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_47().test_properties_with_config(&config, |(x, rm)| {
        reciprocal_gamma_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_gamma_properties_helper(x: Float) {
    let reciprocal_gamma = x.clone().reciprocal_gamma();
    assert!(reciprocal_gamma.is_valid());

    let reciprocal_gamma_alt = (&x).reciprocal_gamma();
    assert!(reciprocal_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );

    let mut x_alt = x.clone();
    x_alt.reciprocal_gamma_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );

    let reciprocal_gamma_alt = x
        .reciprocal_gamma_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    let reciprocal_gamma_alt = x.reciprocal_gamma_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    let reciprocal_gamma_alt = x.reciprocal_gamma_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    if x > 0u32 {
        // The gamma function is positive on the positive reals; its reciprocal may underflow.
        assert!(reciprocal_gamma >= 0u32);
    }
}

#[test]
fn reciprocal_gamma_properties() {
    float_gen().test_properties(|x| {
        reciprocal_gamma_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        reciprocal_gamma_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        reciprocal_gamma_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_reciprocal_gamma() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_reciprocal_gamma(x)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 1.0);
    test::<f32>(2.0, 1.0);
    test::<f32>(5.0, 0.041666668);
    test::<f32>(0.5, 0.5641896);
    test::<f32>(-0.5, -0.2820948);
    test::<f32>(-1.0, 0.0);
    test::<f32>(1.5, core::f32::consts::FRAC_2_SQRT_PI);
    test::<f32>(3.3, 0.37265638);
    test::<f32>(-3.3, 2.2804108);
    test::<f32>(35.0, 3.387157e-39);
    test::<f32>(36.0, 9.6776e-41);
    test::<f32>(171.0, 0.0);
    test::<f32>(172.0, 0.0);
    test::<f32>(1.0e-30, 1.0e-30);
    test::<f32>(-1.0e-30, -1.0e-30);
    test::<f32>(1.0e-40, 1.0e-40);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 1.0);
    test::<f64>(2.0, 1.0);
    test::<f64>(5.0, 0.041666666666666664);
    test::<f64>(0.5, 0.5641895835477563);
    test::<f64>(-0.5, -0.28209479177387814);
    test::<f64>(-1.0, 0.0);
    test::<f64>(1.5, core::f64::consts::FRAC_2_SQRT_PI);
    test::<f64>(3.3, 0.37265635737367964);
    test::<f64>(-3.3, 2.2804112625634207);
    test::<f64>(35.0, 3.387157535521162e-39);
    test::<f64>(36.0, 9.67759295863189e-41);
    test::<f64>(171.0, 1.3779009677917706e-307);
    test::<f64>(172.0, 8.05790039644312e-310);
    test::<f64>(1.0e-30, 1.0e-30);
    test::<f64>(-1.0e-30, -1.0e-30);
    test::<f64>(1.0e-40, 1.0e-40);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_reciprocal_gamma_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_reciprocal_gamma(x);
    });
}

#[test]
fn primitive_float_reciprocal_gamma_properties() {
    apply_fn_to_primitive_floats!(primitive_float_reciprocal_gamma_properties_helper);
}

#[test]
fn test_reciprocal_gamma_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::reciprocal_gamma_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::reciprocal_gamma_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 100, "0.0", "0x0.0", Equal);
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
    test("3", 1, "0.50", "0x0.8#1", Equal);
    test("3", 10, "0.50000", "0x0.800#10", Equal);
    test(
        "3",
        100,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("5", 1, "0.031", "0x0.08#1", Less);
    test("5", 10, "0.041687", "0x0.0aac#10", Greater);
    test(
        "5",
        100,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test("25", 1, "1.7e-24", "0x2.0E-20#1", Greater);
    test("25", 10, "1.6124e-24", "0x1.f30E-20#10", Greater);
    test(
        "25",
        100,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20#100",
        Greater,
    );
    test("1/2", 1, "0.50", "0x0.8#1", Less);
    test("1/2", 10, "0.56445", "0x0.908#10", Greater);
    test(
        "1/2",
        100,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test("1/3", 1, "0.25", "0x0.4#1", Less);
    test("1/3", 10, "0.37305", "0x0.5f8#10", Less);
    test(
        "1/3",
        100,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8#100",
        Greater,
    );
    test("1/4", 1, "0.25", "0x0.4#1", Less);
    test("1/4", 10, "0.27588", "0x0.46a#10", Greater);
    test(
        "1/4",
        100,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test("1/6", 1, "0.12", "0x0.2#1", Less);
    test("1/6", 10, "0.17969", "0x0.2e0#10", Greater);
    test(
        "1/6",
        100,
        "0.17965203550789734572626719518281",
        "0x0.2dfdad012a8f59a314ad4a6878#100",
        Less,
    );
    test("2/3", 1, "0.50", "0x0.8#1", Less);
    test("2/3", 10, "0.73828", "0x0.bd0#10", Less);
    test(
        "2/3",
        100,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f#100",
        Greater,
    );
    test("-1/2", 1, "-0.25", "-0x0.4#1", Greater);
    test("-1/2", 10, "-0.28223", "-0x0.484#10", Less);
    test(
        "-1/2",
        100,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8#100",
        Less,
    );
    test("-1", 1, "0.0", "0x0.0", Equal);
    test("-1", 10, "0.0", "0x0.0", Equal);
    test("-2", 1, "0.0", "0x0.0", Equal);
    test("-2", 10, "0.0", "0x0.0", Equal);
    test("-5/2", 1, "-1.0", "-0x1.0#1", Greater);
    test("-5/2", 10, "-1.0586", "-0x1.0f0#10", Less);
    test("-7/3", 1, "-1.0", "-0x1.0#1", Less);
    test("-7/3", 10, "-0.76562", "-0x0.c40#10", Greater);
    test("22/7", 1, "0.50", "0x0.8#1", Greater);
    test("22/7", 10, "0.43652", "0x0.6fc#10", Greater);
    test("1000001/1000", 1, "1.7e-2565", "0x1.0E-2130#1", Less);
    test(
        "1000001/1000",
        10,
        "2.4691e-2565",
        "0x1.790E-2130#10",
        Greater,
    );
    test("-1000001/1000", 1, "-3.0e2564", "-0x8.0E+2129#1", Greater);
    test(
        "-1000001/1000",
        10,
        "-4.0539e2564",
        "-0xa.e0E+2129#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        1,
        "-7.9e-31",
        "-0x1.0E-25#1",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test("100000000", 1, "0.0", "0x0.0", Less);
    test("100000000", 10, "0.0", "0x0.0", Less);
    test("200000001/2", 1, "0.0", "0x0.0", Less);
    test("200000001/2", 10, "0.0", "0x0.0", Less);
    test("-200000001/2", 1, "-Infinity", "-Infinity", Less);
    test("-200000001/2", 10, "-Infinity", "-Infinity", Less);
    test("-200000003/2", 1, "Infinity", "Infinity", Greater);
    test("-200000003/2", 10, "Infinity", "Infinity", Greater);
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_fail() {
    Float::reciprocal_gamma_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_ref_fail() {
    Float::reciprocal_gamma_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_reciprocal_gamma_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::reciprocal_gamma_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::reciprocal_gamma_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    // - gamma_rational_prec_round_helper zero
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
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
    test("3", 10, Floor, "0.50000", "0x0.800#10", Equal);
    test("3", 10, Ceiling, "0.50000", "0x0.800#10", Equal);
    test("3", 10, Down, "0.50000", "0x0.800#10", Equal);
    test("3", 10, Up, "0.50000", "0x0.800#10", Equal);
    test("3", 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("3", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("5", 10, Floor, "0.041626", "0x0.0aa8#10", Less);
    test("5", 10, Ceiling, "0.041687", "0x0.0aac#10", Greater);
    test("5", 10, Down, "0.041626", "0x0.0aa8#10", Less);
    test("5", 10, Up, "0.041687", "0x0.0aac#10", Greater);
    test("5", 10, Nearest, "0.041687", "0x0.0aac#10", Greater);
    test("25", 10, Floor, "1.6107e-24", "0x1.f28E-20#10", Less);
    test("25", 10, Ceiling, "1.6124e-24", "0x1.f30E-20#10", Greater);
    test("25", 10, Down, "1.6107e-24", "0x1.f28E-20#10", Less);
    test("25", 10, Up, "1.6124e-24", "0x1.f30E-20#10", Greater);
    test("25", 10, Nearest, "1.6124e-24", "0x1.f30E-20#10", Greater);
    test("1/2", 10, Floor, "0.56348", "0x0.904#10", Less);
    test("1/2", 10, Ceiling, "0.56445", "0x0.908#10", Greater);
    test("1/2", 10, Down, "0.56348", "0x0.904#10", Less);
    test("1/2", 10, Up, "0.56445", "0x0.908#10", Greater);
    test("1/2", 10, Nearest, "0.56445", "0x0.908#10", Greater);
    test("1/3", 10, Floor, "0.37305", "0x0.5f8#10", Less);
    test("1/3", 10, Ceiling, "0.37354", "0x0.5fa#10", Greater);
    test("1/3", 10, Down, "0.37305", "0x0.5f8#10", Less);
    test("1/3", 10, Up, "0.37354", "0x0.5fa#10", Greater);
    test("1/3", 10, Nearest, "0.37305", "0x0.5f8#10", Less);
    test("1/4", 10, Floor, "0.27539", "0x0.468#10", Less);
    test("1/4", 10, Ceiling, "0.27588", "0x0.46a#10", Greater);
    test("1/4", 10, Down, "0.27539", "0x0.468#10", Less);
    test("1/4", 10, Up, "0.27588", "0x0.46a#10", Greater);
    test("1/4", 10, Nearest, "0.27588", "0x0.46a#10", Greater);
    test("1/6", 10, Floor, "0.17944", "0x0.2df#10", Less);
    test("1/6", 10, Ceiling, "0.17969", "0x0.2e0#10", Greater);
    test("1/6", 10, Down, "0.17944", "0x0.2df#10", Less);
    test("1/6", 10, Up, "0.17969", "0x0.2e0#10", Greater);
    test("1/6", 10, Nearest, "0.17969", "0x0.2e0#10", Greater);
    test("2/3", 10, Floor, "0.73828", "0x0.bd0#10", Less);
    test("2/3", 10, Ceiling, "0.73926", "0x0.bd4#10", Greater);
    test("2/3", 10, Down, "0.73828", "0x0.bd0#10", Less);
    test("2/3", 10, Up, "0.73926", "0x0.bd4#10", Greater);
    test("2/3", 10, Nearest, "0.73828", "0x0.bd0#10", Less);
    test("-1/2", 10, Floor, "-0.28223", "-0x0.484#10", Less);
    test("-1/2", 10, Ceiling, "-0.28174", "-0x0.482#10", Greater);
    test("-1/2", 10, Down, "-0.28174", "-0x0.482#10", Greater);
    test("-1/2", 10, Up, "-0.28223", "-0x0.484#10", Less);
    test("-1/2", 10, Nearest, "-0.28223", "-0x0.484#10", Less);
    test("-1", 10, Floor, "0.0", "0x0.0", Equal);
    test("-1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-1", 10, Down, "0.0", "0x0.0", Equal);
    test("-1", 10, Up, "0.0", "0x0.0", Equal);
    test("-1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-2", 10, Floor, "0.0", "0x0.0", Equal);
    test("-2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-2", 10, Down, "0.0", "0x0.0", Equal);
    test("-2", 10, Up, "0.0", "0x0.0", Equal);
    test("-2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-2", 10, Exact, "0.0", "0x0.0", Equal);
    test("-5/2", 10, Floor, "-1.0586", "-0x1.0f0#10", Less);
    test("-5/2", 10, Ceiling, "-1.0566", "-0x1.0e8#10", Greater);
    test("-5/2", 10, Down, "-1.0566", "-0x1.0e8#10", Greater);
    test("-5/2", 10, Up, "-1.0586", "-0x1.0f0#10", Less);
    test("-5/2", 10, Nearest, "-1.0586", "-0x1.0f0#10", Less);
    test("-7/3", 10, Floor, "-0.76660", "-0x0.c44#10", Less);
    test("-7/3", 10, Ceiling, "-0.76562", "-0x0.c40#10", Greater);
    test("-7/3", 10, Down, "-0.76562", "-0x0.c40#10", Greater);
    test("-7/3", 10, Up, "-0.76660", "-0x0.c44#10", Less);
    test("-7/3", 10, Nearest, "-0.76562", "-0x0.c40#10", Greater);
    test("22/7", 10, Floor, "0.43604", "0x0.6fa#10", Less);
    test("22/7", 10, Ceiling, "0.43652", "0x0.6fc#10", Greater);
    test("22/7", 10, Down, "0.43604", "0x0.6fa#10", Less);
    test("22/7", 10, Up, "0.43652", "0x0.6fc#10", Greater);
    test("22/7", 10, Nearest, "0.43652", "0x0.6fc#10", Greater);
    test(
        "1000001/1000",
        10,
        Floor,
        "2.4658e-2565",
        "0x1.788E-2130#10",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Ceiling,
        "2.4691e-2565",
        "0x1.790E-2130#10",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Down,
        "2.4658e-2565",
        "0x1.788E-2130#10",
        Less,
    );
    test(
        "1000001/1000",
        10,
        Up,
        "2.4691e-2565",
        "0x1.790E-2130#10",
        Greater,
    );
    test(
        "1000001/1000",
        10,
        Nearest,
        "2.4691e-2565",
        "0x1.790E-2130#10",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Floor,
        "-4.0539e2564",
        "-0xa.e0E+2129#10",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Ceiling,
        "-4.0481e2564",
        "-0xa.dcE+2129#10",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Down,
        "-4.0481e2564",
        "-0xa.dcE+2129#10",
        Greater,
    );
    test(
        "-1000001/1000",
        10,
        Up,
        "-4.0539e2564",
        "-0xa.e0E+2129#10",
        Less,
    );
    test(
        "-1000001/1000",
        10,
        Nearest,
        "-4.0539e2564",
        "-0xa.e0E+2129#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "7.9040e-31",
        "0x1.008E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "7.9040e-31",
        "0x1.008E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Floor,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Ceiling,
        "-7.8809e-31",
        "-0xf.fcE-26#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Down,
        "-7.8809e-31",
        "-0xf.fcE-26#10",
        Greater,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Up,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "-1/1267650600228229401496703205376",
        10,
        Nearest,
        "-7.8886e-31",
        "-0x1.000E-25#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test("100000000", 10, Floor, "0.0", "0x0.0", Less);
    test(
        "100000000",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("100000000", 10, Down, "0.0", "0x0.0", Less);
    test(
        "100000000",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("100000000", 10, Nearest, "0.0", "0x0.0", Less);
    test("200000001/2", 10, Floor, "0.0", "0x0.0", Less);
    test(
        "200000001/2",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("200000001/2", 10, Down, "0.0", "0x0.0", Less);
    test(
        "200000001/2",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("200000001/2", 10, Nearest, "0.0", "0x0.0", Less);
    test("-200000001/2", 10, Floor, "-Infinity", "-Infinity", Less);
    test(
        "-200000001/2",
        10,
        Ceiling,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test(
        "-200000001/2",
        10,
        Down,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test("-200000001/2", 10, Up, "-Infinity", "-Infinity", Less);
    test("-200000001/2", 10, Nearest, "-Infinity", "-Infinity", Less);
    test(
        "-200000003/2",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test("-200000003/2", 10, Ceiling, "Infinity", "Infinity", Greater);
    test(
        "-200000003/2",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test("-200000003/2", 10, Up, "Infinity", "Infinity", Greater);
    // - gamma_rational_prec_round_helper huge negative
    test("-200000003/2", 10, Nearest, "Infinity", "Infinity", Greater);
    test("0", 100, Floor, "0.0", "0x0.0", Equal);
    test("0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 100, Down, "0.0", "0x0.0", Equal);
    test("0", 100, Up, "0.0", "0x0.0", Equal);
    test("0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("0", 100, Exact, "0.0", "0x0.0", Equal);
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
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Ceiling,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Down,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Up,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Nearest,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "3",
        100,
        Exact,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "5",
        100,
        Floor,
        "0.041666666666666666666666666666634",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test(
        "5",
        100,
        Ceiling,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test(
        "5",
        100,
        Down,
        "0.041666666666666666666666666666634",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test(
        "5",
        100,
        Up,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test(
        "5",
        100,
        Nearest,
        "0.041666666666666666666666666666683",
        "0x0.0aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test(
        "25",
        100,
        Floor,
        "1.6117375710961183490487133048000e-24",
        "0x1.f2cf01972f577cca4b4067ca8E-20#100",
        Less,
    );
    test(
        "25",
        100,
        Ceiling,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20#100",
        Greater,
    );
    test(
        "25",
        100,
        Down,
        "1.6117375710961183490487133048000e-24",
        "0x1.f2cf01972f577cca4b4067ca8E-20#100",
        Less,
    );
    test(
        "25",
        100,
        Up,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20#100",
        Greater,
    );
    test(
        "25",
        100,
        Nearest,
        "1.6117375710961183490487133048013e-24",
        "0x1.f2cf01972f577cca4b4067caaE-20#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Floor,
        "0.56418958354775628694807945156018",
        "0x0.906eba8214db688d71d48a7f6#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "0.56418958354775628694807945156018",
        "0x0.906eba8214db688d71d48a7f6#100",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.56418958354775628694807945156097",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Floor,
        "0.37328217390739522832635031242307",
        "0x0.5f8f6ba91caf1d5cb7b2a514a0#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.37328217390739522832635031242307",
        "0x0.5f8f6ba91caf1d5cb7b2a514a0#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.37328217390739522832635031242347",
        "0x0.5f8f6ba91caf1d5cb7b2a514a8#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Floor,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test(
        "1/4",
        100,
        Ceiling,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Down,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test(
        "1/4",
        100,
        Up,
        "0.27581566283020931435994553998857",
        "0x0.469bdaf3948f50844c4a13ceb8#100",
        Greater,
    );
    test(
        "1/4",
        100,
        Nearest,
        "0.27581566283020931435994553998818",
        "0x0.469bdaf3948f50844c4a13ceb0#100",
        Less,
    );
    test(
        "1/6",
        100,
        Floor,
        "0.17965203550789734572626719518281",
        "0x0.2dfdad012a8f59a314ad4a6878#100",
        Less,
    );
    test(
        "1/6",
        100,
        Ceiling,
        "0.17965203550789734572626719518301",
        "0x0.2dfdad012a8f59a314ad4a687c#100",
        Greater,
    );
    test(
        "1/6",
        100,
        Down,
        "0.17965203550789734572626719518281",
        "0x0.2dfdad012a8f59a314ad4a6878#100",
        Less,
    );
    test(
        "1/6",
        100,
        Up,
        "0.17965203550789734572626719518301",
        "0x0.2dfdad012a8f59a314ad4a687c#100",
        Greater,
    );
    test(
        "1/6",
        100,
        Nearest,
        "0.17965203550789734572626719518281",
        "0x0.2dfdad012a8f59a314ad4a6878#100",
        Less,
    );
    test(
        "2/3",
        100,
        Floor,
        "0.73848811162164831293575437516416",
        "0x0.bd0d8e8fe657c99422d13c76e#100",
        Less,
    );
    test(
        "2/3",
        100,
        Ceiling,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f#100",
        Greater,
    );
    test(
        "2/3",
        100,
        Down,
        "0.73848811162164831293575437516416",
        "0x0.bd0d8e8fe657c99422d13c76e#100",
        Less,
    );
    test(
        "2/3",
        100,
        Up,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f#100",
        Greater,
    );
    test(
        "2/3",
        100,
        Nearest,
        "0.73848811162164831293575437516495",
        "0x0.bd0d8e8fe657c99422d13c76f#100",
        Greater,
    );
    test(
        "-1/2",
        100,
        Floor,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8#100",
        Less,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-0.28209479177387814347403972578009",
        "-0x0.48375d410a6db446b8ea453fb0#100",
        Greater,
    );
    test(
        "-1/2",
        100,
        Down,
        "-0.28209479177387814347403972578009",
        "-0x0.48375d410a6db446b8ea453fb0#100",
        Greater,
    );
    test(
        "-1/2",
        100,
        Up,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8#100",
        Less,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-0.28209479177387814347403972578049",
        "-0x0.48375d410a6db446b8ea453fb8#100",
        Less,
    );
    // - gamma_rational_prec_round_helper huge negative integer or huge denominator
    test("-67108864", 3, Up, "0.0", "0x0.0", Equal);
    // - gamma_rational_prec_round_helper huge positive
    test(
        "100663295",
        2,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#2",
        Greater,
    );
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_round_fail_1() {
    Float::reciprocal_gamma_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_round_fail_2() {
    Float::reciprocal_gamma_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn reciprocal_gamma_rational_prec_round_ref_fail() {
    Float::reciprocal_gamma_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_gamma_rational_prec_round_properties_helper(
    x: Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let (reciprocal_gamma, o) = Float::reciprocal_gamma_rational_prec_round(x.clone(), prec, rm);
    assert!(reciprocal_gamma.is_valid());
    assert_rounding_ordering_consistent(&reciprocal_gamma, rm, o);

    let (reciprocal_gamma_alt, o_alt) =
        Float::reciprocal_gamma_rational_prec_round_ref(&x, prec, rm);
    assert!(reciprocal_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    // The reference computes the reciprocal of the gamma function, in time that grows with x, and
    // with twice as many bits as a tiny x has leading zeros.
    if rm != Exact
        && o != Equal
        && reciprocal_gamma.is_normal()
        && result_in_range(&reciprocal_gamma, o)
        && x.lt_abs(&0x100000u32)
        && x.ge_abs(&Rational::power_of_2(-1000i64))
    {
        let (g, go) = reciprocal_gamma_rational_prec_round_naive(&x, prec, rm);
        assert_eq!(float_to_big_exponent_float(&reciprocal_gamma), g);
        assert_eq!(o, go);
    }

    if reciprocal_gamma.is_normal() {
        assert_eq!(reciprocal_gamma.get_prec(), Some(prec));
    }
    if x > 0u32 {
        // The gamma function is positive on the positive reals; its reciprocal may underflow.
        assert!(reciprocal_gamma >= 0u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::reciprocal_gamma_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(s),
                ComparableFloat(reciprocal_gamma.clone())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::reciprocal_gamma_rational_prec_round_ref(
            &x, prec, Exact
        ));
    }
}

#[test]
fn reciprocal_gamma_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        reciprocal_gamma_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_gamma_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (reciprocal_gamma, o) = Float::reciprocal_gamma_rational_prec(x.clone(), prec);
    assert!(reciprocal_gamma.is_valid());

    let (reciprocal_gamma_alt, o_alt) = Float::reciprocal_gamma_rational_prec_ref(&x, prec);
    assert!(reciprocal_gamma_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);

    let (reciprocal_gamma_alt, o_alt) =
        Float::reciprocal_gamma_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&reciprocal_gamma_alt),
        ComparableFloatRef(&reciprocal_gamma)
    );
    assert_eq!(o_alt, o);
}

#[test]
fn reciprocal_gamma_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        reciprocal_gamma_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_reciprocal_gamma_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_reciprocal_gamma_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", 1.0);
    test::<f32>("2", 1.0);
    test::<f32>("3", 0.5);
    test::<f32>("5", 0.041666668);
    test::<f32>("25", 1.6117376e-24);
    test::<f32>("1/2", 0.5641896);
    test::<f32>("1/3", 0.37328216);
    test::<f32>("1/4", 0.27581567);
    test::<f32>("1/6", 0.17965204);
    test::<f32>("2/3", 0.73848814);
    test::<f32>("-1/2", -0.2820948);
    test::<f32>("-1", 0.0);
    test::<f32>("-2", 0.0);
    test::<f32>("-5/2", -1.0578555);
    test::<f32>("-7/3", -0.7658395);
    test::<f32>("22/7", 0.43651587);
    test::<f32>("1000001/1000", 0.0);
    test::<f32>("-1000001/1000", f32::NEGATIVE_INFINITY);
    test::<f32>("1/1267650600228229401496703205376", 7.888609e-31);
    test::<f32>("-1/1267650600228229401496703205376", -7.888609e-31);
    test::<f32>("35", 3.387157e-39);
    test::<f32>("36", 9.6776e-41);
    test::<f32>("171", 0.0);
    test::<f32>("172", 0.0);
    test::<f64>("0", 0.0);
    test::<f64>("1", 1.0);
    test::<f64>("2", 1.0);
    test::<f64>("3", 0.5);
    test::<f64>("5", 0.041666666666666664);
    test::<f64>("25", 1.6117375710961184e-24);
    test::<f64>("1/2", 0.5641895835477563);
    test::<f64>("1/3", 0.3732821739073952);
    test::<f64>("1/4", 0.2758156628302093);
    test::<f64>("1/6", 0.17965203550789735);
    test::<f64>("2/3", 0.7384881116216483);
    test::<f64>("-1/2", -0.28209479177387814);
    test::<f64>("-1", 0.0);
    test::<f64>("-2", 0.0);
    test::<f64>("-5/2", -1.057855469152043);
    test::<f64>("-7/3", -0.7658395231631908);
    test::<f64>("22/7", 0.43651586116628777);
    test::<f64>("1000001/1000", 0.0);
    test::<f64>("-1000001/1000", f64::NEGATIVE_INFINITY);
    test::<f64>("1/1267650600228229401496703205376", 7.888609052210118e-31);
    test::<f64>("-1/1267650600228229401496703205376", -7.888609052210118e-31);
    test::<f64>("35", 3.387157535521162e-39);
    test::<f64>("36", 9.67759295863189e-41);
    test::<f64>("171", 1.3779009677917706e-307);
    test::<f64>("172", 8.05790039644312e-310);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_reciprocal_gamma_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        primitive_float_reciprocal_gamma_rational::<T>(&x);
    });
}

#[test]
fn primitive_float_reciprocal_gamma_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_reciprocal_gamma_rational_properties_helper);
}
