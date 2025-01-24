// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{NegAssign, Reciprocal, ReciprocalAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, rounding_mode_gen, unsigned_gen_var_11,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::test_util::arithmetic::reciprocal::{
    reciprocal_prec_round_naive_1, reciprocal_prec_round_naive_2, rug_reciprocal,
    rug_reciprocal_prec, rug_reciprocal_prec_round, rug_reciprocal_round,
};
use malachite_float::test_util::common::{
    emulate_primitive_float_fn, parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_11, float_gen_var_6, float_gen_var_7, float_gen_var_8,
    float_rounding_mode_pair_gen_var_13, float_rounding_mode_pair_gen_var_14,
    float_rounding_mode_pair_gen_var_15, float_rounding_mode_pair_gen_var_16,
    float_rounding_mode_pair_gen_var_17, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_3,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use std::panic::catch_unwind;

#[test]
fn test_reciprocal() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let reciprocal = x.clone().reciprocal();
        assert!(reciprocal.is_valid());

        assert_eq!(reciprocal.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal), out_hex);

        let reciprocal_alt = (&x).reciprocal();
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );

        let mut reciprocal_alt = x.clone();
        reciprocal_alt.reciprocal_assign();
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );

        let reciprocal_alt =
            reciprocal_prec_round_naive_1(x.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        let reciprocal_alt =
            reciprocal_prec_round_naive_2(x.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );

        let rug_reciprocal = rug_reciprocal(&rug::Float::exact_from(&x));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal)),
            ComparableFloatRef(&reciprocal),
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "0.0", "0x0.0");
    test("-Infinity", "-Infinity", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "Infinity", "Infinity");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "-1.0", "-0x1.0#1");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        "-1.0",
        "-0x1.0000000000000000000000000#100",
    );

    test("123.0", "0x7b.0#7", "0.0082", "0x0.0218#7");
    test("-123.0", "-0x7b.0#7", "-0.0082", "-0x0.0218#7");
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "0.7071067811865475",
        "0x0.b504f333f9de60#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-0.7071067811865475",
        "-0x0.b504f333f9de60#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0.31830988618379069",
        "0x0.517cc1b727220c#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-0.31830988618379069",
        "-0x0.517cc1b727220c#53",
    );

    // - x.is_power_of_2() in reciprocal_prec_round
    // - !x.is_power_of_2() in reciprocal_prec_round
    // - in reciprocal_float_significand_same_prec_lt_w
    // - x != HIGH_BIT in reciprocal_float_significand_same_prec_lt_w
    // - (q + 2) & (mask >> 1) > 2 in reciprocal_float_significand_same_prec_lt_w;
    // - round_bit != 0 || sticky_bit != 0 in reciprocal_float_significand_same_prec_lt_w
    // - rm == Nearest in reciprocal_float_significand_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 && reciprocal & shift_bit != 0) in
    //   reciprocal_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", "0.8", "0x0.c#2");
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && reciprocal & shift_bit == 0) in
    //   reciprocal_float_significand_same_prec_lt_w
    test("1.2", "0x1.4#3", "0.8", "0x0.c#3");
    // - (q + 2) & (mask >> 1) <= 2 in reciprocal_float_significand_same_prec_lt_w
    // - hi == 0 && lo < x first time in reciprocal_float_significand_same_prec_lt_w
    // - hi == 0 && lo < x second time in reciprocal_float_significand_same_prec_lt_w
    test(
        "3615091.606162289805",
        "0x372973.9b2d73aac8#61",
        "2.7661816322867136e-7",
        "0x4.a410e30d72ea318E-6#61",
    );
    // - in reciprocal_float_significand_same_prec_w
    // - x != HIGH_BIT in reciprocal_float_significand_same_prec_w
    // - hi == 0 && lo < x first time in reciprocal_float_significand_same_prec_w
    // - hi == 0 && lo < x second time in reciprocal_float_significand_same_prec_w
    // - !round_bit in reciprocal_float_significand_same_prec_w
    // - round_bit || sticky_bit != 0 in reciprocal_float_significand_same_prec_w
    // - rm == Exact in reciprocal_float_significand_same_prec_w
    // - rm == Exact && (!round_bit || sticky_bit == 0 && reciprocal.even()) in
    //   reciprocal_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "0.99999999999999999989",
        "0x0.fffffffffffffffe#64",
    );
    // - round_bit in reciprocal_float_significand_same_prec_w
    // - rm == Exact && round_bit && (sticky_bit != 0 || reciprocal.even() in
    //   reciprocal_float_significand_same_prec_w
    test(
        "0.113243462684988497952",
        "0x0.1cfd8608b7c32de2a#64",
        "8.830531814288645436",
        "0x8.d49dbba4a843592#64",
    );
    // - in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - x_0 != 0 || x_1 != HIGH_BIT in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - in reciprocal_float_2_approx
    // - x_1 != Limb::MAX in reciprocal_float_2_approx
    // - yy == 0 in reciprocal_float_2_approx
    // - r_0 != 0 || yy == 0 in reciprocal_float_2_approx
    // - carry in reciprocal_float_2_approx
    // - r_1 == 0 in reciprocal_float_2_approx
    // - (q_0.wrapping_add(21)) & (mask >> 1) <= 21 in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - s_1 != 0 || s_0 != 0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - s_2 > 0 || s_1 > x_1 || s_1 == x_1 && s_0 >= x_0 in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - s_1 < x_1 || s_1 == x_1 && s_0 < x_0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - round_bit != 0 || sticky_bit != 0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && z_0 & shift_bit == 0) in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
    );
    // - r_1 != 0 in reciprocal_float_2_approx
    // - s_1 >= x_1 && (s_1 != x_1 || s_0 >= x_0) in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "0.99999999999999999989",
        "0x0.fffffffffffffffe0#65",
    );
    // - yy != 0 in reciprocal_float_2_approx
    test(
        "1.00000000000000000003",
        "0x1.00000000000000008#66",
        "0.99999999999999999997",
        "0x0.ffffffffffffffff8#66",
    );
    // - (q_0.wrapping_add(21)) & (mask >> 1) > 21 in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        "0.99999999999999999984",
        "0x0.fffffffffffffffd0#65",
    );
    // - rm != Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && !carry in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "6.9434396726939059937558762e-29",
        "0x5.804bfffff864a7e6a3c7cE-24#85",
    );
    // - s_2 <= 0 && s_1 <= x_1 && (s_1 != x_1 || s_0 < x_0) in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.1066650957130428898050817125418740852e-35",
        "0xe.b5c943322fb9cafab82fc881e3c1f4E-30#123",
        "90361574054676697026138186100092211.86",
        "0x11672b68e1cda153b96db5b555ad33.dc#123",
    );
    // - x_1 == Limb::MAX in reciprocal_float_2_approx
    test(
        "4.9517601571415210995e27",
        "0xf.fffffffffffffff8E+22#65",
        "2.019483917365790222e-28",
        "0x1.0000000000000001E-23#65",
    );
    // - !carry in reciprocal_float_2_approx
    test(
        "1.809457589959748038781206513903043742e-25",
        "0x3.800000000000000000000000000000E-21#121",
        "5526518032524019084371090.285714285714",
        "0x492492492492492492492.4924924924#121",
    );
    // - s_1 == 0 && s_0 == 0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "4095.75",
        "0xfff.c00000000000000000000000000#120",
        "0.0002441555270707440639687480925349447598",
        "0x0.001000400100040010004001000400100#120",
    );
    // - in reciprocal_float_significand_general
    // - extra_bit in reciprocal_float_significand_general
    // - qs_len < MPFR_DIV_THRESHOLD || ds_len < MPFR_DIV_THRESHOLD in
    //   reciprocal_float_significand_general
    // - rm != Nearest || shift != 0 in reciprocal_float_significand_general
    // - ds_len >= qs_2_len in reciprocal_float_significand_general
    // - qs_2_len == qs_len in reciprocal_float_significand_general
    // - sticky_bit != 0 || sticky_3 != 0 in reciprocal_float_significand_general
    // - ds_len <= qs_2_len in reciprocal_float_significand_general
    // - ds_len <= qs_2_len && rm == Nearest in reciprocal_float_significand_general
    // - cleanup == None in reciprocal_float_significand_general
    // - cleanup == None && rm == Nearest && (round_bit != 0 || sticky_bit != 0) in
    //   reciprocal_float_significand_general
    // - cleanup == None && rm == Nearest && (round_bit != 0 || sticky_bit != 0) && round_bit == 0
    //   in reciprocal_float_significand_general
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "0.999999999999999999999999999999999999997",
        "0x0.ffffffffffffffffffffffffffffffff0#129",
    );
    // - !extra_bit in reciprocal_float_significand_general
    test(
        "3.8524937267946719191140399538619613749184e-10",
        "0x1.a7960ee660129a7bc6beccda5d8bb012f0E-8#135",
        "2595721293.573692163399156109109436743137",
        "0x9ab7904d.92dd7d57c55752828aeb2a056a#135",
    );
    // - cleanup == None && rm == Nearest && (round_bit != 0 || sticky_bit != 0) && round_bit != 0
    //   && sticky_bit == 0 in reciprocal_float_significand_general
    test(
        "59494692712728004820788608585666.4829798",
        "0x2eeedb85c9cdc503a8e25ed4fc2.7ba490#129",
        "1.680822195062896543011322450000404260061e-32",
        "0x5.745f58c91536fd9586859912d6b99220E-27#129",
    );
    // - rm == Nearest && shift == 0 in reciprocal_float_significand_general
    // - ds_len < qs_2_len in reciprocal_float_significand_general
    // - qs_2_len != qs_len in reciprocal_float_significand_general
    test(
        "2.652265028059746721807174554033221706564e-11",
        "0x1.d29765de1f777af51db92558a3d9f542E-9#128",
        "37703622730.7776167463689706185944181549",
        "0x8c74fa04a.c711e41e7a061938aeb7ca7#128",
    );
    // - qs_len >= MPFR_DIV_THRESHOLD && ds_len >= MPFR_DIV_THRESHOLD in
    //   reciprocal_float_significand_general
    // - ds_len < n in reciprocal_float_significand_general
    // - !q_high first time in reciprocal_float_significand_general
    test(
        "99775868891207693182758620905617766484977359141657322302733467080906379945858675686059451\
        2527853476231275058799551652072546.7114971760702609364731573674336745185834760605451122614\
        680178551142556046183482705875960001033145321970465204907865385015751310573750415565593472\
        515573584122133946534420508845514863685042630834456885627933663697385547769664847990486584\
        336882273751721721644989648370590667737234950547668737865047573751482757356022197920174371\
        088074314780588737501583713833755004374512024407585163195094394292034507503368814534990168\
        9912721166210129145585",
        "0x1826427338bc8ee8c907c3ce5e6a2a793f6ba67df6e738f22dc8aee7eb1838ddc4290e49186e61bdbedb847\
        d19c5d8c4bf88c62.b624adce6b0a3564827e04608c1aec0c8b10390491e15df75402c1788241935e791ebd5f4\
        25d73042c03e3bad5f0d11257d8bcdab6c8bae677785865be19fa4f42690ddb02174b09bb2c1c9ce6cf3dc2d80\
        9f0b0b79c42ae70f14ec682ac3850e91ee3b6ef02555e18758417024bf2e8801a759e710b3ac91f28b15277ff4\
        f6380b7ba380aa56c032ce8db2107bfd99a9c789098467f2b27a7b3e1bb6a9e7804ef8a26a3baea51e9a8da4d5\
        02af09995fd6ced97b00#1859",
        "1.002246345847779005201453959044909523251487705121768272318254782834562788571992915761065\
        827618366102604676901058851697875789517863827988808181377939460972322574747531876040609995\
        972589309563651666694269954573404860608746408716939006674422401722850836821062009150256011\
        720454011696660779666788543360159960577274185817743487975811370456064842946971427355525804\
        257690941749159301402957505054859414089331855848531064209977516186532202351442372831975270\
        870077932986581025601789533966442159881772178301189187013534424007199259091978932352502557\
        2202453587766889671779e-123",
        "0xa.99c4846d5eeedd01292b3ecbfdcde3e23e86f2c0de91a7853e3f16d01225356463802a0309555fe6a982a\
        b689ccb12d932eab55b6ffd61c4fdd7cd737afd36bd5acda69948c10851f5fd1a254537be41d4c013aa43aaa09\
        93fccd821acb36881a3a14540999fa35a76a34b052ec4c6e62c85b8890330ad74145c3af385378890639293f97\
        87eeb51c942fb1b0480f7e5dcadd2da6f8bbf05ac6e562e773bff36faf231658530929fef9e7c5b84843c5674a\
        883eede0deef889addd0d20f57f1eaeb61dfb8dd23ed0ba6dfc00929192924d8b397f3d5d4913b580d5176e47b\
        5900b7857bdc095ca14E-103#1859",
    );
    // - r_0 == 0 && yy != 0 in reciprocal_float_2_approx
    test(
        "206158430208.00000000558794",
        "0x3000000000.000000180000#83",
        "4.850638409455617268748732e-12",
        "0x5.5555555555555552aaabE-10#83",
    );
}

#[test]
fn test_reciprocal_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal, o) = x.clone().reciprocal_prec(prec);
        assert!(reciprocal.is_valid());

        assert_eq!(reciprocal.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_alt, o_alt) = x.reciprocal_prec_ref(prec);
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_alt = x.clone();
        let o_alt = reciprocal_alt.reciprocal_prec_assign(prec);
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );
        assert_eq!(o_alt, o_out);

        let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_1(x.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        assert_eq!(o_alt, o);
        let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_2(x.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        assert_eq!(o_alt, o);

        let (rug_reciprocal, rug_o) = rug_reciprocal_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal)),
            ComparableFloatRef(&reciprocal),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 1, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 1, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 1, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 10, "-1.0", "-0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        "-1.0",
        "-0x1.0#1",
        Equal,
    );

    test("123.0", "0x7b.0#7", 1, "0.008", "0x0.02#1", Less);
    test("123.0", "0x7b.0#7", 10, "0.00813", "0x0.0215#10", Greater);
    test("-123.0", "-0x7b.0#7", 1, "-0.008", "-0x0.02#1", Greater);
    test("-123.0", "-0x7b.0#7", 10, "-0.00813", "-0x0.0215#10", Less);
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        "0.707",
        "0x0.b50#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        "-0.707",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.3184",
        "0x0.518#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.3184",
        "-0x0.518#10",
        Less,
    );

    // - in reciprocal_float_significand_short
    // - in limbs_reciprocal_limb_to_out_mod_with_fraction
    // - fraction_len != 0 in limbs_reciprocal_limb_to_out_mod_with_fraction
    // - out_last == 0 in reciprocal_float_significand_short
    // - out_last == 0 && shift != 0 in reciprocal_float_significand_short
    // - round_bit == 0 && sticky_bit != 0 in reciprocal_float_significand_short
    // - rm == Nearest in reciprocal_float_significand_short
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && out[0] & shift_bit == 0) in
    //   reciprocal_float_significand_short
    test("1.5", "0x1.8#2", 1, "0.5", "0x0.8#1", Less);
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] & shift_bit != 0) in
    //   reciprocal_float_significand_short
    test("1.5", "0x1.8#2", 4, "0.7", "0x0.b#4", Greater);
    // - ds_len > qs_2_len in reciprocal_float_significand_general
    // - inex != Equal in reciprocal_float_significand_general
    // - ds_len > qs_2_len && rm == Nearest in reciprocal_float_significand_general
    // - sticky_3 > 1 in reciprocal_float_significand_general
    test(
        "0.000199046277632504184666664672269768242929310652018203552191617720205649",
        "0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225",
        26,
        "5023.9573",
        "0x139f.f510#26",
        Less,
    );
    // - out_last == 0 && shift == 0 in reciprocal_float_significand_short
    // - out_last == 0 && shift == 0 && c >= y - c
    test(
        "1.4904942e-19",
        "0x2.bfddbE-16#22",
        64,
        "6709184284559176977.0",
        "0x5d1bcf8f5dc87511.0#64",
        Greater,
    );
    // - sticky_3 <= 1 in reciprocal_float_significand_general
    // - !q_high second time in reciprocal_float_significand_general
    // - !q_high_2 in reciprocal_float_significand_general
    // - cmp_s_r != Equal in reciprocal_float_significand_general
    // - cmp_s_r <= Equal first time in reciprocal_float_significand_general
    test(
        "13104.5238818416080254535",
        "0x3330.861d1ed0acba8a3a#77",
        61,
        "0.00007630952555137529844",
        "0x0.00050042eaa75fe3e40#61",
        Less,
    );
    // - cmp_s_r > Equal first time in reciprocal_float_significand_general
    // - cmp_s_r > Equal && !q_high_2 in reciprocal_float_significand_general
    // - cmp_s_r <= Equal second time in reciprocal_float_significand_general
    // - sticky_3 != 1 && round_bit != 0 in reciprocal_float_significand_general
    test(
        "4047252243163522937320069504914937843.384444039",
        "0x30b78f117589e437888c5573227d7f3.626aecb#150",
        126,
        "2.47081214591743090926825573642415225879e-37",
        "0x5.413cf35bf8a6be7eed9713c3529cac4E-31#126",
        Less,
    );
    // - out_last == 0 && shift == 0 && c < y - c in reciprocal_float_significand_short
    test(
        "0.252",
        "0x0.408#8",
        64,
        "3.9689922480620155039",
        "0x3.f80fe03f80fe03f8#64",
        Less,
    );
    // - sticky_3 != 1 && round_bit == 0 in reciprocal_float_significand_general
    // - cmp_s_r != Equal || shift != 0 in reciprocal_float_significand_general
    // - rm == Nearest || ((rm == Ceiling || rm == Up) && inex != Equal) in
    //   reciprocal_float_significand_general
    // - cleanup == TruncateCheckQHigh in reciprocal_float_significand_general
    // - !q_high third time in reciprocal_float_significand_general
    test(
        "13486082141.77132281557478202754",
        "0x323d5485d.c575697b8d65625d0#99",
        60,
        "7.41505197349076425e-11",
        "0x5.187840c7b7df518E-9#60",
        Greater,
    );
    // - sticky_3 == 1 in reciprocal_float_significand_general
    test(
        "1.22280082196367917099634553e-19",
        "0x2.41738c7082eda42f40f3f0E-16#87",
        62,
        "8177946743559704448.0",
        "0x717de73c11f04f80.0#62",
        Less,
    );
    // - cmp_s_r > Equal second time in reciprocal_float_significand_general
    // - cmp_s_r > Equal && rm == Nearest in reciprocal_float_significand_general
    // - cmp_s_r > Equal && rm == Nearest && shift == 1 in reciprocal_float_significand_general
    // - cmp_s_r > Equal && rm == Nearest && shift == 1 && round_bit == 0 in
    //   reciprocal_float_significand_general
    // - cleanup == Sub2Ulp in reciprocal_float_significand_general
    test(
        "18686733767405.50192797818236099916512095073138146049740456616",
        "0x10fed820d2ed.807e5a1b3d9ab71287cc373ef7f3521609fa72f#201",
        63,
        "5.351389988464750895e-14",
        "0xf.1015372ed29c6daE-12#63",
        Less,
    );
    // - cmp_s_r > Equal && rm == Nearest && shift == 1 && round_bit != 0 in
    //   reciprocal_float_significand_general
    test(
        "5.1485428388978050057923204436e-7",
        "0x8.a348459137c894fe7ce2ce4E-6#94",
        63,
        "1942297.1339480570737",
        "0x1da319.224a6b7c7e8#63",
        Greater,
    );
    // - q_high second time in reciprocal_float_significand_general
    // - q_high third time in reciprocal_float_significand_general
    test(
        "77371252455336267181195267.98438",
        "0x4000000000000000000003.fc00#101",
        21,
        "1.29247e-26",
        "0x4.00000E-22#21",
        Greater,
    );
    // - cmp_s_r > Equal && rm == Nearest && shift != 1 in reciprocal_float_significand_general
    test(
        "2658455991647202407967140029027844095.999984793",
        "0x2000000003fffdfffffffffff7fffff.ffff00e#150",
        126,
        "3.76158192252184441821673862447332945247e-37",
        "0x7.ffffffff000080001fffe0020400060E-31#126",
        Greater,
    );
    // - cmp_s_r == Equal in reciprocal_float_significand_general
    // - !slice_test_zero(sp_lo) in reciprocal_float_significand_general
    test(
        "1.00000000000000000000000000000000000000000000035028",
        "0x1.00000000000000000000000000000000000001fff#165",
        9,
        "1.0",
        "0x1.00#9",
        Greater,
    );
    // - ds_len >= n in reciprocal_float_significand_general
    test(
        "0.055494458197186880675630915676192867532090892470422627386324587939238035948781984969197\
        293549419550977162311955781124943110956302191334821801709357381779895051695270613703928454\
        324596584327643817343816609304133584205767499993731432854297161698269943482406592768874444\
        537959922587924718339566077249075342435855861361805700477320039260219431275491192763683820\
        335274971653357747385666948482136785955293678142161982025522843889154198503897563042426395\
        128128925013857602497675148024154470366154782845358820106881972937537492453282371069714224\
        342987087633028783648188309716648625167308459669051113846104707695985727291553732092985376\
        828259777127224883258056131830095048084139109366273880047266946456087245977725112865710579\
        525111935320391211042996099454878241904334424644681645423208503968089041208566140555448097\
        852910157786980160921009881876094041863058139126904649967001982231878732430835123816538920\
        677999964668461960985136485079991108378659476109039768366016053541284627738754596865400997\
        112674267336505852768050602674774522669073467755292625683686148376948941511306559626232154\
        752250188007464931234370449669484827725942224651581822979291904312419637250173763557353262\
        213949126273235198961215664565140295983002285722717251104994492638175694612259962414559660\
        670986455111669460799863783422503680534332809871688740315758114042751185650495859242221856\
        676315059490112305738127164937772981750053994564437960009449100953381044642977719710987666\
        446442147625231494914221140350124646594870978197268779196749153799654193158076220975432784\
        285250458268961487489605062990550609833462756239703423977438296502233632161406744847227123\
        129345837972247611423244017961911382138681216321381679915794615975763676728759826413970576\
        860022980739678182223426387634227578298610034617798500806276338343012622846552838812662393\
        624312342667652111927682534015047744310614980669350777266223429538479188836406048087315918\
        309894499222120214872928367335411152843249721184424372084873571670196566303743654124450236\
        183156500319414376380635601282651645722305689309629762280330704861001884908133936528480642\
        756123234",
        "0x0.0e34e28310efa672de91bbf3447b991e67d8318cdf0c4058e4c9c730c71dc5b4bf675e849e3ac19fa3e70\
        d9cbfb926620de7c9c66fc396364a70516bd66e253f5b318c8f82fabc4da09cab178fa55b2cd32603f085d4149\
        a70fb07eb0959a5a78a00603aae495e7a094d9ee04b63747d7a023fb4369e5bf83efc20d6dfe31bafee72256df\
        1e39a8949cb554b519bb7b532f0ffb97b7fe5238cb68f0cfca74556fd7588422c7b383f4183a4193de48c69bae\
        7faf54820ec3c71871cbb288daf37266ee4f1fd3e40e483ed25f601b87f9c4a92b8cec5ec0e8d7edbd4234fba3\
        1a0a463a29b8f32d02f0d1f55195b2ec33db71548c7ccaf7e3a658ee71d308649653be586268e028c9f15d7788\
        cbbd9d825fadf58d6f13183242c54015f254a271b4c89cfc1d82fc8c6182b0d0c620d53cf81728d495822994c6\
        63bb058d661f74be0ae8c8c10381afa72bc89a2a01da15a5f6d117b31e04e34a6e11e0aa8ad521b7a6117ce813\
        b8e9326335c30e14052e4aa58c9062245ce2b058e7ca1221aa19af3c21653dac76aa59bb843a7a0ad8c6e22992\
        639766e47bfcc7de81f7a5a43e5faba0bd029002632b5d26d5a4303d9bbf4cf2664d57e50df98491d49ceb0f2f\
        a1db2579ec23c459066023beebebd800fc879d76d59a3d6d2949650b07ef7b31e752ca051f4f107c7ff4af3f56\
        43eb287784ebcbcbefaa6a9496458e1b5afa0a74a27a47545d2f9091189f54f315a8bc904dedd3248f8cfbf32b\
        19df9037d47bd021b075b70aaa2087c30b755c65f93508e4564f2d998430e8f68565d451de80e317a96c18493e\
        662aa4f6ed98647d9051d9c993accc85820fe55a06d956025618ac1fbd020af906edd248ee088ee34f3ec8fc99\
        1e9392efafd1acd495d65fe834644d939ef5fa91d984969c79257afb98f8d94a3551a1a4b533c182b0d198d8a9\
        53e030b1532cd0e54117934099dd0b0027cd6058fb4190686ba78ded239aca8ac146e13f4c88052537903cb587\
        95eae03289ef10a0dee81bb143567c686a600364a57829437fc75160f8660a0ac48daacd98471fb2c0c490cd90\
        2cea0ea9aa3a814462dd78ab7d573f356b8c0ee7ac1f0c13819e3aeb5b2c8ea9f3a386bf90e5898fb38867db59\
        a9e8ae436df1ececb24a9e478c52dd17ad020f761aca8761acb63eb70ac73687a85f5fc45a10ab7873104f3489\
        9df7961eaa91080cf7a#6892",
        2006,
        "18.01981733827778674260325184800154550178488687484021553354418910902303106544992907597073\
        029189149288233180017138039419588169269797921955767649134253967949783482048904124978427680\
        356665076989426379081885511868423028921432842040598017121872298172949413285212017353511719\
        635837927429422415155120291649700552853758008370658383590920718577880818445566446372050296\
        508733341553853089827665379953621398768316388977766832203115700511569639084749508682625973\
        200934933860726967306493882349632958444320046459537281159643562613023877018494385007867594\
        9194217635032456311998703209522721567033904823795308705191538464412",
        "0x12.0512bfc3cbff370e13079bf70f24c8fc14fb1154aa0638c41252b425cfb276f04379cf0908b1732ae3ca\
        5d5ce2ef398eca257c87dfb650e9648bf8397dd443e42a5fd3663c3d58b9b6cd2a1ebf3c9246c45a1f6086158e\
        a1e93d0f78945b71e8bdf265700c826de9776e648ce900159a0f4f2716ec326e2b8289920112637767e8fee268\
        3363b8e4c4bf07c685c226e6b97d260fdc2910d7e8a10fada9bda175aeb73def046bae399c664bfe3cfb3f7e73\
        136427cf5ca96cad226976adc2a9a0117dd38cbd0aaf5edbdb6a9e925126c38670a8946bc5ee1840200876c486\
        de477d9c4b50e410143b115c27f9ba57c1176894c9d33effdbbd50928#2006",
        Less,
    );
    // - q_high first time in reciprocal_float_significand_general
    // - !round_helper_2 in reciprocal_float_significand_general
    test(
        "3.725290298461914062500000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000011016237598729697344643067035553076048795855845749823769296583726265856513\
        808851994012395922156678620494270052333942368952051919368238891272218333e-9",
        "0x1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000003fffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeE-7#3520",
        1969,
        "268435456.0",
        "0x10000000.000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000#1969",
        Greater,
    );
}

#[test]
fn reciprocal_prec_fail() {
    assert_panic!(Float::NAN.reciprocal_prec(0));
    assert_panic!(Float::NAN.reciprocal_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.reciprocal_prec_assign(0)
    });
}

#[test]
fn test_reciprocal_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal, o) = x.clone().reciprocal_round(rm);
        assert!(reciprocal.is_valid());

        assert_eq!(reciprocal.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_alt, o_alt) = x.reciprocal_round_ref(rm);
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_alt = x.clone();
        let o_alt = reciprocal_alt.reciprocal_round_assign(rm);
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );
        assert_eq!(o_alt, o_out);

        let (reciprocal_alt, o_alt) =
            reciprocal_prec_round_naive_1(x.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        assert_eq!(o_alt, o);
        let (reciprocal_alt, o_alt) =
            reciprocal_prec_round_naive_2(x.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_reciprocal, rug_o) = rug_reciprocal_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_reciprocal)),
                ComparableFloatRef(&reciprocal),
            );
            assert_eq!(rug_o, o);
        }
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

    test("-Infinity", "-Infinity", Floor, "-0.0", "-0x0.0", Equal);
    test("-Infinity", "-Infinity", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-Infinity", "-Infinity", Down, "-0.0", "-0x0.0", Equal);
    test("-Infinity", "-Infinity", Up, "-0.0", "-0x0.0", Equal);
    test("-Infinity", "-Infinity", Nearest, "-0.0", "-0x0.0", Equal);
    test("-Infinity", "-Infinity", Exact, "-0.0", "-0x0.0", Equal);

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

    test("-1.0", "-0x1.0#1", Floor, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Down, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Up, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Exact, "-1.0", "-0x1.0#1", Equal);

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Floor,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Ceiling,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Down,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Up,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Nearest,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Exact,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );

    test("123.0", "0x7b.0#7", Floor, "0.0081", "0x0.0210#7", Less);
    test(
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );
    test("123.0", "0x7b.0#7", Down, "0.0081", "0x0.0210#7", Less);
    test("123.0", "0x7b.0#7", Up, "0.0082", "0x0.0218#7", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        Nearest,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );

    test("-123.0", "-0x7b.0#7", Floor, "-0.0082", "-0x0.0218#7", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        Ceiling,
        "-0.0081",
        "-0x0.0210#7",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        Down,
        "-0.0081",
        "-0x0.0210#7",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", Up, "-0.0082", "-0x0.0218#7", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        Nearest,
        "-0.0082",
        "-0x0.0218#7",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Floor,
        "0.7071067811865475",
        "0x0.b504f333f9de60#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Ceiling,
        "0.7071067811865476",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Down,
        "0.7071067811865475",
        "0x0.b504f333f9de60#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Up,
        "0.7071067811865476",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Nearest,
        "0.7071067811865475",
        "0x0.b504f333f9de60#53",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Floor,
        "-0.7071067811865476",
        "-0x0.b504f333f9de68#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Ceiling,
        "-0.7071067811865475",
        "-0x0.b504f333f9de60#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Down,
        "-0.7071067811865475",
        "-0x0.b504f333f9de60#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Up,
        "-0.7071067811865476",
        "-0x0.b504f333f9de68#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Nearest,
        "-0.7071067811865475",
        "-0x0.b504f333f9de60#53",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "0.31830988618379064",
        "0x0.517cc1b7272208#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "0.31830988618379069",
        "0x0.517cc1b727220c#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "0.31830988618379064",
        "0x0.517cc1b7272208#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "0.31830988618379069",
        "0x0.517cc1b727220c#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "0.31830988618379069",
        "0x0.517cc1b727220c#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-0.31830988618379069",
        "-0x0.517cc1b727220c#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-0.31830988618379064",
        "-0x0.517cc1b7272208#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-0.31830988618379064",
        "-0x0.517cc1b7272208#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-0.31830988618379069",
        "-0x0.517cc1b727220c#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-0.31830988618379069",
        "-0x0.517cc1b727220c#53",
        Less,
    );

    // - rm == Floor || rm == Down in reciprocal_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", Down, "0.5", "0x0.8#2", Less);
    // - rm == Ceiling || rm == Up in reciprocal_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", Up, "0.8", "0x0.c#2", Greater);
    // - rm == Floor || rm == Down in reciprocal_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Down,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe#64",
        Less,
    );
    // - rm == Ceiling || rm == Up in reciprocal_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Up,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff#64",
        Greater,
    );
    // - rm == Floor || rm == Down in reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Down,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
        Less,
    );
    // - rm == Ceiling || rm == Up in reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Up,
        "0.99999999999999999997",
        "0x0.ffffffffffffffff8#65",
        Greater,
    );
    // - ds_len <= qs_2_len && rm == Ceiling || rm == Up && inex != Equal in
    //   reciprocal_float_significand_general
    // - cleanup == None && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0) in
    //   reciprocal_float_significand_general
    test(
        "2.4914040842493675536005152793625253098043524808533216867315977e-8",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E-7#202",
        Up,
        "40138009.17811728321982547739205337771132288200540084196230409",
        "0x2647519.2d9918224811b1eb5289a86e1aa22ec284493440dbd2#202",
        Greater,
    );
    // - ds_len <= qs_2_len && rm == Floor || rm == Down || (rm != Nearest && inex == Equal) in
    //   reciprocal_float_significand_general
    // - cleanup == None && (rm == Floor || rm == Down || round_bit == 0 && sticky_bit == 0) in
    //   reciprocal_float_significand_general
    test(
        "2.4914040842493675536005152793625253098043524808533216867315977e-8",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E-7#202",
        Down,
        "40138009.17811728321982547739205337771132288200540084196230408",
        "0x2647519.2d9918224811b1eb5289a86e1aa22ec284493440dbd1#202",
        Less,
    );

    // - x.is_power_of_2() in reciprocal_prec_round
    // - !x.is_power_of_2() in reciprocal_prec_round
    // - in reciprocal_float_significand_same_prec_lt_w
    // - x != HIGH_BIT in reciprocal_float_significand_same_prec_lt_w
    // - (q + 2) & (mask >> 1) > 2 in reciprocal_float_significand_same_prec_lt_w;
    // - round_bit != 0 || sticky_bit != 0 in reciprocal_float_significand_same_prec_lt_w
    // - rm == Nearest in reciprocal_float_significand_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 && reciprocal & shift_bit != 0) in
    //   reciprocal_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", Nearest, "0.8", "0x0.c#2", Greater);
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && reciprocal & shift_bit == 0) in
    //   reciprocal_float_significand_same_prec_lt_w
    test("1.2", "0x1.4#3", Nearest, "0.8", "0x0.c#3", Less);
    // - (q + 2) & (mask >> 1) <= 2 in reciprocal_float_significand_same_prec_lt_w
    // - hi == 0 && lo < x first time in reciprocal_float_significand_same_prec_lt_w
    // - hi == 0 && lo < x second time in reciprocal_float_significand_same_prec_lt_w
    test(
        "3615091.606162289805",
        "0x372973.9b2d73aac8#61",
        Nearest,
        "2.7661816322867136e-7",
        "0x4.a410e30d72ea318E-6#61",
        Less,
    );
    // - in reciprocal_float_significand_same_prec_w
    // - x != HIGH_BIT in reciprocal_float_significand_same_prec_w
    // - hi == 0 && lo < x first time in reciprocal_float_significand_same_prec_w
    // - hi == 0 && lo < x second time in reciprocal_float_significand_same_prec_w
    // - !round_bit in reciprocal_float_significand_same_prec_w
    // - round_bit || sticky_bit != 0 in reciprocal_float_significand_same_prec_w
    // - rm == Exact in reciprocal_float_significand_same_prec_w
    // - rm == Exact && (!round_bit || sticky_bit == 0 && reciprocal.even()) in
    //   reciprocal_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Nearest,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe#64",
        Less,
    );
    // - round_bit in reciprocal_float_significand_same_prec_w
    // - rm == Exact && round_bit && (sticky_bit != 0 || reciprocal.even() in
    //   reciprocal_float_significand_same_prec_w
    test(
        "0.113243462684988497952",
        "0x0.1cfd8608b7c32de2a#64",
        Nearest,
        "8.830531814288645436",
        "0x8.d49dbba4a843592#64",
        Greater,
    );
    // - in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - x_0 != 0 || x_1 != HIGH_BIT in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - in reciprocal_float_2_approx
    // - x_1 != Limb::MAX in reciprocal_float_2_approx
    // - yy == 0 in reciprocal_float_2_approx
    // - r_0 != 0 || yy == 0 in reciprocal_float_2_approx
    // - carry in reciprocal_float_2_approx
    // - r_1 == 0 in reciprocal_float_2_approx
    // - (q_0.wrapping_add(21)) & (mask >> 1) <= 21 in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - s_1 != 0 || s_0 != 0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - s_2 > 0 || s_1 > x_1 || s_1 == x_1 && s_0 >= x_0 in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - s_1 < x_1 || s_1 == x_1 && s_0 < x_0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - round_bit != 0 || sticky_bit != 0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest in reciprocal_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && z_0 & shift_bit == 0) in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
        Less,
    );
    // - r_1 != 0 in reciprocal_float_2_approx
    // - s_1 >= x_1 && (s_1 != x_1 || s_0 >= x_0) in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Nearest,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe0#65",
        Less,
    );
    // - yy != 0 in reciprocal_float_2_approx
    test(
        "1.00000000000000000003",
        "0x1.00000000000000008#66",
        Nearest,
        "0.99999999999999999997",
        "0x0.ffffffffffffffff8#66",
        Less,
    );
    // - (q_0.wrapping_add(21)) & (mask >> 1) > 21 in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        Nearest,
        "0.99999999999999999984",
        "0x0.fffffffffffffffd0#65",
        Less,
    );
    // - rm != Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && !carry in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        Nearest,
        "6.9434396726939059937558762e-29",
        "0x5.804bfffff864a7e6a3c7cE-24#85",
        Greater,
    );
    // - s_2 <= 0 && s_1 <= x_1 && (s_1 != x_1 || s_0 < x_0) in
    //   reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.1066650957130428898050817125418740852e-35",
        "0xe.b5c943322fb9cafab82fc881e3c1f4E-30#123",
        Nearest,
        "90361574054676697026138186100092211.86",
        "0x11672b68e1cda153b96db5b555ad33.dc#123",
        Greater,
    );
    // - x_1 == Limb::MAX in reciprocal_float_2_approx
    test(
        "4.9517601571415210995e27",
        "0xf.fffffffffffffff8E+22#65",
        Nearest,
        "2.019483917365790222e-28",
        "0x1.0000000000000001E-23#65",
        Greater,
    );
    // - !carry in reciprocal_float_2_approx
    test(
        "1.809457589959748038781206513903043742e-25",
        "0x3.800000000000000000000000000000E-21#121",
        Nearest,
        "5526518032524019084371090.285714285714",
        "0x492492492492492492492.4924924924#121",
        Less,
    );
    // - s_1 == 0 && s_0 == 0 in reciprocal_float_significand_same_prec_gt_w_lt_2w
    test(
        "4095.75",
        "0xfff.c00000000000000000000000000#120",
        Nearest,
        "0.0002441555270707440639687480925349447598",
        "0x0.001000400100040010004001000400100#120",
        Less,
    );
    // - in reciprocal_float_significand_general
    // - extra_bit in reciprocal_float_significand_general
    // - qs_len < MPFR_DIV_THRESHOLD || ds_len < MPFR_DIV_THRESHOLD in
    //   reciprocal_float_significand_general
    // - rm != Nearest || shift != 0 in reciprocal_float_significand_general
    // - ds_len >= qs_2_len in reciprocal_float_significand_general
    // - qs_2_len == qs_len in reciprocal_float_significand_general
    // - sticky_bit != 0 || sticky_3 != 0 in reciprocal_float_significand_general
    // - ds_len <= qs_2_len in reciprocal_float_significand_general
    // - ds_len <= qs_2_len && rm == Nearest in reciprocal_float_significand_general
    // - cleanup == None in reciprocal_float_significand_general
    // - cleanup == None && rm == Nearest && (round_bit != 0 || sticky_bit != 0) in
    //   reciprocal_float_significand_general
    // - cleanup == None && rm == Nearest && (round_bit != 0 || sticky_bit != 0) && round_bit == 0
    //   in reciprocal_float_significand_general
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Nearest,
        "0.999999999999999999999999999999999999997",
        "0x0.ffffffffffffffffffffffffffffffff0#129",
        Less,
    );
    // - !extra_bit in reciprocal_float_significand_general
    test(
        "3.8524937267946719191140399538619613749184e-10",
        "0x1.a7960ee660129a7bc6beccda5d8bb012f0E-8#135",
        Nearest,
        "2595721293.573692163399156109109436743137",
        "0x9ab7904d.92dd7d57c55752828aeb2a056a#135",
        Less,
    );
    // - cleanup == None && rm == Nearest && (round_bit != 0 || sticky_bit != 0) && round_bit != 0
    //   && sticky_bit == 0 in reciprocal_float_significand_general
    test(
        "59494692712728004820788608585666.4829798",
        "0x2eeedb85c9cdc503a8e25ed4fc2.7ba490#129",
        Nearest,
        "1.680822195062896543011322450000404260061e-32",
        "0x5.745f58c91536fd9586859912d6b99220E-27#129",
        Greater,
    );
    // - rm == Nearest && shift == 0 in reciprocal_float_significand_general
    // - ds_len < qs_2_len in reciprocal_float_significand_general
    // - qs_2_len != qs_len in reciprocal_float_significand_general
    test(
        "2.652265028059746721807174554033221706564e-11",
        "0x1.d29765de1f777af51db92558a3d9f542E-9#128",
        Nearest,
        "37703622730.7776167463689706185944181549",
        "0x8c74fa04a.c711e41e7a061938aeb7ca7#128",
        Greater,
    );
    // - qs_len >= MPFR_DIV_THRESHOLD && ds_len >= MPFR_DIV_THRESHOLD in
    //   reciprocal_float_significand_general
    // - ds_len < n in reciprocal_float_significand_general
    // - !q_high first time in reciprocal_float_significand_general
    test(
        "99775868891207693182758620905617766484977359141657322302733467080906379945858675686059451\
        2527853476231275058799551652072546.7114971760702609364731573674336745185834760605451122614\
        680178551142556046183482705875960001033145321970465204907865385015751310573750415565593472\
        515573584122133946534420508845514863685042630834456885627933663697385547769664847990486584\
        336882273751721721644989648370590667737234950547668737865047573751482757356022197920174371\
        088074314780588737501583713833755004374512024407585163195094394292034507503368814534990168\
        9912721166210129145585",
        "0x1826427338bc8ee8c907c3ce5e6a2a793f6ba67df6e738f22dc8aee7eb1838ddc4290e49186e61bdbedb847\
        d19c5d8c4bf88c62.b624adce6b0a3564827e04608c1aec0c8b10390491e15df75402c1788241935e791ebd5f4\
        25d73042c03e3bad5f0d11257d8bcdab6c8bae677785865be19fa4f42690ddb02174b09bb2c1c9ce6cf3dc2d80\
        9f0b0b79c42ae70f14ec682ac3850e91ee3b6ef02555e18758417024bf2e8801a759e710b3ac91f28b15277ff4\
        f6380b7ba380aa56c032ce8db2107bfd99a9c789098467f2b27a7b3e1bb6a9e7804ef8a26a3baea51e9a8da4d5\
        02af09995fd6ced97b00#1859",
        Nearest,
        "1.002246345847779005201453959044909523251487705121768272318254782834562788571992915761065\
        827618366102604676901058851697875789517863827988808181377939460972322574747531876040609995\
        972589309563651666694269954573404860608746408716939006674422401722850836821062009150256011\
        720454011696660779666788543360159960577274185817743487975811370456064842946971427355525804\
        257690941749159301402957505054859414089331855848531064209977516186532202351442372831975270\
        870077932986581025601789533966442159881772178301189187013534424007199259091978932352502557\
        2202453587766889671779e-123",
        "0xa.99c4846d5eeedd01292b3ecbfdcde3e23e86f2c0de91a7853e3f16d01225356463802a0309555fe6a982a\
        b689ccb12d932eab55b6ffd61c4fdd7cd737afd36bd5acda69948c10851f5fd1a254537be41d4c013aa43aaa09\
        93fccd821acb36881a3a14540999fa35a76a34b052ec4c6e62c85b8890330ad74145c3af385378890639293f97\
        87eeb51c942fb1b0480f7e5dcadd2da6f8bbf05ac6e562e773bff36faf231658530929fef9e7c5b84843c5674a\
        883eede0deef889addd0d20f57f1eaeb61dfb8dd23ed0ba6dfc00929192924d8b397f3d5d4913b580d5176e47b\
        5900b7857bdc095ca14E-103#1859",
        Greater,
    );
    // - r_0 == 0 && yy != 0 in reciprocal_float_2_approx
    test(
        "206158430208.00000000558794",
        "0x3000000000.000000180000#83",
        Nearest,
        "4.850638409455617268748732e-12",
        "0x5.5555555555555552aaabE-10#83",
        Greater,
    );
}

#[test]
fn reciprocal_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.reciprocal_round(Exact));
    assert_panic!(THREE.reciprocal_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.reciprocal_round_assign(Exact);
    });
}

#[test]
fn test_reciprocal_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal, o) = x.clone().reciprocal_prec_round(prec, rm);
        assert!(reciprocal.is_valid());

        assert_eq!(reciprocal.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_alt, o_alt) = x.reciprocal_prec_round_ref(prec, rm);
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_alt = x.clone();
        let o_alt = reciprocal_alt.reciprocal_prec_round_assign(prec, rm);
        assert!(reciprocal_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal),
            ComparableFloatRef(&reciprocal_alt)
        );
        assert_eq!(o_alt, o_out);

        let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_1(x.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        assert_eq!(o_alt, o);
        let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_2(x.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&reciprocal_alt),
            ComparableFloatRef(&reciprocal)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_reciprocal, rug_o) =
                rug_reciprocal_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_reciprocal)),
                ComparableFloatRef(&reciprocal),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test("Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Down, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Up, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Exact, "0.0", "0x0.0", Equal);

    test("-Infinity", "-Infinity", 1, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, Down, "-0.0", "-0x0.0", Equal);
    test("-Infinity", "-Infinity", 1, Up, "-0.0", "-0x0.0", Equal);
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, Exact, "-0.0", "-0x0.0", Equal);

    test("0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Down, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Up, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal);

    test("-0.0", "-0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 1, Down, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Up, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 1, Exact, "-Infinity", "-Infinity", Equal);

    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("-1.0", "-0x1.0#1", 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Down, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Up, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test("1.0", "0x1.0#1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test("-1.0", "-0x1.0#1", 10, Floor, "-1.0", "-0x1.000#10", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-1.0",
        "-0x1.000#10",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 10, Down, "-1.0", "-0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Up, "-1.0", "-0x1.000#10", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 10, Exact, "-1.0", "-0x1.000#10", Equal);

    test("123.0", "0x7b.0#7", 1, Floor, "0.008", "0x0.02#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "0.02", "0x0.04#1", Greater);
    test("123.0", "0x7b.0#7", 1, Down, "0.008", "0x0.02#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "0.02", "0x0.04#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "0.008", "0x0.02#1", Less);

    test("-123.0", "-0x7b.0#7", 1, Floor, "-0.02", "-0x0.04#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        1,
        Ceiling,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        1,
        Down,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", 1, Up, "-0.02", "-0x0.04#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        1,
        Nearest,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "0.00812",
        "0x0.0214#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "0.00812",
        "0x0.0214#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );

    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Floor,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Ceiling,
        "-0.00812",
        "-0x0.0214#10",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Down,
        "-0.00812",
        "-0x0.0214#10",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Up,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Nearest,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Floor,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Down,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Down,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Nearest,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Floor,
        "0.707",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Ceiling,
        "0.708",
        "0x0.b54#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Down,
        "0.707",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Up,
        "0.708",
        "0x0.b54#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Nearest,
        "0.707",
        "0x0.b50#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Floor,
        "-0.708",
        "-0x0.b54#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Ceiling,
        "-0.707",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Down,
        "-0.707",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Up,
        "-0.708",
        "-0x0.b54#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Nearest,
        "-0.707",
        "-0x0.b50#10",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.2",
        "0x0.4#1",
        Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.3179",
        "0x0.516#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.3184",
        "0x0.518#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "0.3179",
        "0x0.516#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "0.3184",
        "0x0.518#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.3184",
        "0x0.518#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-0.3184",
        "-0x0.518#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.3179",
        "-0x0.516#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-0.3179",
        "-0x0.516#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-0.3184",
        "-0x0.518#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.3184",
        "-0x0.518#10",
        Less,
    );

    // - rm == Floor || rm == Down in reciprocal_float_significand_short
    test("1.5", "0x1.8#2", 1, Down, "0.5", "0x0.8#1", Less);
    // - rm == Ceiling || rm == Up in reciprocal_float_significand_short
    test("1.5", "0x1.8#2", 1, Up, "1.0", "0x1.0#1", Greater);
    // - ds_len > qs_2_len && rm != Nearest in reciprocal_float_significand_general
    test(
        "2.4914040842493675536005152793625253098043524808533216867315977e-8",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E-7#202",
        15,
        Up,
        "4.0139e7",
        "0x2.6478E+6#15",
        Greater,
    );
    // - rm != Nearest && (rm != Ceiling && rm != Up || inex == Equal) && (rm != Exact || inex !=
    //   Equal) in reciprocal_float_significand_general
    // - cleanup == Sub1Ulp in reciprocal_float_significand_general
    test(
        "1164607425.036820041559",
        "0x456a7fc1.096d09ca55#71",
        64,
        Down,
        "8.586584444697183348e-10",
        "0x3.b01add9bdcc8ca28E-8#64",
        Less,
    );
    // - cmp_s_r > Equal && (rm == Ceiling || rm == Up) in reciprocal_float_significand_general
    // - cmp_s_r > Equal && (rm == Ceiling || rm == Up) && shift != 0 in
    //   reciprocal_float_significand_general
    test(
        "18686733767405.50192797818236099916512095073138146049740456616",
        "0x10fed820d2ed.807e5a1b3d9ab71287cc373ef7f3521609fa72f#201",
        63,
        Up,
        "5.3513899884647508956e-14",
        "0xf.1015372ed29c6dcE-12#63",
        Greater,
    );
    // - cmp_s_r > Equal && (rm == Floor || rm == Down) in reciprocal_float_significand_general
    // - cmp_s_r > Equal && (rm == Floor || rm == Down) && shift != 0 in
    //   reciprocal_float_significand_general
    test(
        "18686733767405.50192797818236099916512095073138146049740456616",
        "0x10fed820d2ed.807e5a1b3d9ab71287cc373ef7f3521609fa72f#201",
        63,
        Down,
        "5.351389988464750895e-14",
        "0xf.1015372ed29c6daE-12#63",
        Less,
    );
    // - cmp_s_r > Equal && (rm == Ceiling || rm == Up) && shift == 0 in
    //   reciprocal_float_significand_general
    test(
        "1.063382396643406948814112e37",
        "0x8.000000007ffffffffff8E+30#81",
        64,
        Up,
        "9.4039548064414545111e-38",
        "0x1.ffffffffe0000002E-31#64",
        Greater,
    );
    // - cmp_s_r > Equal && (rm == Floor || rm == Down) && shift == 0 in
    //   reciprocal_float_significand_general
    test(
        "1.063382396643406948814112e37",
        "0x8.000000007ffffffffff8E+30#81",
        64,
        Down,
        "9.4039548064414545106e-38",
        "0x1.ffffffffe0000000E-31#64",
        Less,
    );
}

#[test]
fn reciprocal_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).reciprocal_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).reciprocal_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.reciprocal_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.reciprocal_prec_round(1, Exact));
    assert_panic!(THREE.reciprocal_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.reciprocal_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (reciprocal, o) = x.clone().reciprocal_prec_round(prec, rm);
    assert!(reciprocal.is_valid());
    let (reciprocal_alt, o_alt) = x.reciprocal_prec_round_ref(prec, rm);
    assert!(reciprocal_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&reciprocal));
    assert_eq!(o_alt, o);

    let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_1(x.clone(), prec, rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);
    let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_2(x.clone(), prec, rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_reciprocal, rug_o) =
            rug_reciprocal_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal)),
            ComparableFloatRef(&reciprocal),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal && reciprocal.is_finite() && reciprocal != 0 {
        assert_eq!(
            ComparableFloatRef(
                &reciprocal
                    .reciprocal_prec_round_ref(x.significant_bits(), Exact)
                    .0
            ),
            ComparableFloatRef(&x)
        );
    }

    let (reciprocal_alt, o_alt) = Float::ONE.div_prec_round(x.clone(), prec, rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    let r_reciprocal = if reciprocal.is_finite() && x.is_finite() {
        if reciprocal.is_normal() {
            assert_eq!(reciprocal.get_prec(), Some(prec));
        }
        let r_reciprocal = Rational::exact_from(&x).reciprocal();
        assert_eq!(reciprocal.partial_cmp(&r_reciprocal), Some(o));
        if o == Less {
            let mut next = reciprocal.clone();
            next.increment();
            assert!(next > r_reciprocal);
        } else if o == Greater {
            let mut next = reciprocal.clone();
            next.decrement();
            assert!(next < r_reciprocal);
        }
        Some(r_reciprocal)
    } else {
        assert_eq!(o, Equal);
        None
    };

    match (
        r_reciprocal.is_some() && *r_reciprocal.as_ref().unwrap() >= 0u32,
        rm,
    ) {
        (_, Floor) | (true, Down) | (false, Up) => {
            assert_ne!(o, Greater);
        }
        (_, Ceiling) | (true, Up) | (false, Down) => {
            assert_ne!(o, Less);
        }
        (_, Exact) => assert_eq!(o, Equal),
        _ => {}
    }

    let (mut reciprocal_alt, mut o_alt) = (-&x).reciprocal_prec_round(prec, -rm);
    reciprocal_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(reciprocal_alt.abs_negative_zero()),
        ComparableFloat(reciprocal.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.reciprocal_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(reciprocal.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.reciprocal_prec_round_ref(prec, Exact));
    }
}

#[test]
fn reciprocal_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(x, prec, rm)| {
        reciprocal_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_rounding_mode_triple_gen_var_3().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            reciprocal_prec_round_properties_helper(x, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 2048);
    float_unsigned_rounding_mode_triple_gen_var_3().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            reciprocal_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (reciprocal, o) = Float::NAN.reciprocal_prec_round(prec, rm);
        assert!(reciprocal.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(
            Float::INFINITY.reciprocal_prec_round(prec, rm),
            (Float::ZERO, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.reciprocal_prec_round(prec, rm),
            (Float::NEGATIVE_ZERO, Equal)
        );
        assert_eq!(
            Float::ZERO.reciprocal_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_ZERO.reciprocal_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert_eq!(
            Float::ONE.reciprocal_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );
        assert_eq!(
            Float::NEGATIVE_ONE.reciprocal_prec_round(prec, rm),
            (Float::negative_one_prec(prec), Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_prec_properties_helper(x: Float, prec: u64) {
    let (reciprocal, o) = x.clone().reciprocal_prec(prec);
    assert!(reciprocal.is_valid());
    let (reciprocal_alt, o_alt) = x.reciprocal_prec_ref(prec);
    assert!(reciprocal_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&reciprocal));
    assert_eq!(o_alt, o);

    let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_1(x.clone(), prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);
    let (reciprocal_alt, o_alt) = reciprocal_prec_round_naive_2(x.clone(), prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    let (rug_reciprocal, rug_o) = rug_reciprocal_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_reciprocal)),
        ComparableFloatRef(&reciprocal),
    );
    assert_eq!(rug_o, o);

    let (reciprocal_alt, o_alt) = Float::ONE.div_prec(x.clone(), prec);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    if reciprocal.is_finite() && x.is_finite() {
        if reciprocal.is_normal() {
            assert_eq!(reciprocal.get_prec(), Some(prec));
        }
        let r_reciprocal = Rational::exact_from(&x).reciprocal();
        assert_eq!(reciprocal.partial_cmp(&r_reciprocal), Some(o));
        if o == Less {
            let mut next = reciprocal.clone();
            next.increment();
            assert!(next > r_reciprocal);
        } else if o == Greater {
            let mut next = reciprocal.clone();
            next.decrement();
            assert!(next < r_reciprocal);
        }
    } else {
        assert_eq!(o, Equal);
    };

    let (mut reciprocal_alt, mut o_alt) = (-&x).reciprocal_prec(prec);
    reciprocal_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(reciprocal_alt.abs_negative_zero()),
        ComparableFloat(reciprocal.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);
}

#[test]
fn reciprocal_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        reciprocal_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        reciprocal_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 2048);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        reciprocal_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (reciprocal, o) = Float::NAN.reciprocal_prec(prec);
        assert!(reciprocal.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.reciprocal_prec(prec), (Float::ZERO, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.reciprocal_prec(prec),
            (Float::NEGATIVE_ZERO, Equal)
        );
        assert_eq!(Float::ZERO.reciprocal_prec(prec), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_ZERO.reciprocal_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert_eq!(
            Float::ONE.reciprocal_prec(prec),
            (Float::one_prec(prec), Equal)
        );
        assert_eq!(
            Float::NEGATIVE_ONE.reciprocal_prec(prec),
            (Float::negative_one_prec(prec), Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_round_properties_helper(x: Float, rm: RoundingMode) {
    let (reciprocal, o) = x.clone().reciprocal_round(rm);
    assert!(reciprocal.is_valid());
    let (reciprocal_alt, o_alt) = x.reciprocal_round_ref(rm);
    assert!(reciprocal_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&reciprocal));
    assert_eq!(o_alt, o);

    let (reciprocal_alt, o_alt) =
        reciprocal_prec_round_naive_1(x.clone(), x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);
    let (reciprocal_alt, o_alt) =
        reciprocal_prec_round_naive_2(x.clone(), x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_reciprocal, rug_o) = rug_reciprocal_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal)),
            ComparableFloatRef(&reciprocal),
        );
        assert_eq!(rug_o, o);
    }

    let (reciprocal_alt, o_alt) = Float::ONE.div_round(x.clone(), rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(o_alt, o);

    let r_reciprocal = if reciprocal.is_finite() && x.is_finite() {
        if reciprocal.is_normal() {
            assert_eq!(reciprocal.get_prec(), Some(x.significant_bits()));
        }
        let r_reciprocal = Rational::exact_from(&x).reciprocal();
        assert_eq!(reciprocal.partial_cmp(&r_reciprocal), Some(o));
        if o == Less {
            let mut next = reciprocal.clone();
            next.increment();
            assert!(next > r_reciprocal);
        } else if o == Greater {
            let mut next = reciprocal.clone();
            next.decrement();
            assert!(next < r_reciprocal);
        }
        Some(r_reciprocal)
    } else {
        assert_eq!(o, Equal);
        None
    };

    match (
        r_reciprocal.is_some() && *r_reciprocal.as_ref().unwrap() >= 0u32,
        rm,
    ) {
        (_, Floor) | (true, Down) | (false, Up) => {
            assert_ne!(o, Greater);
        }
        (_, Ceiling) | (true, Up) | (false, Down) => {
            assert_ne!(o, Less);
        }
        (_, Exact) => assert_eq!(o, Equal),
        _ => {}
    }

    let (mut reciprocal_alt, mut o_alt) = (-&x).reciprocal_round(-rm);
    reciprocal_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(reciprocal_alt.abs_negative_zero()),
        ComparableFloat(reciprocal.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.reciprocal_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(reciprocal.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.reciprocal_round_ref(Exact));
    }
}

#[test]
fn reciprocal_round_properties() {
    float_rounding_mode_pair_gen_var_13().test_properties(|(x, rm)| {
        reciprocal_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_13().test_properties_with_config(&config, |(x, rm)| {
        reciprocal_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_14().test_properties(|(x, rm)| {
        reciprocal_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_15().test_properties(|(x, rm)| {
        reciprocal_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_16().test_properties(|(x, rm)| {
        reciprocal_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_17().test_properties(|(x, rm)| {
        reciprocal_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (reciprocal, o) = Float::NAN.reciprocal_round(rm);
        assert!(reciprocal.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.reciprocal_round(rm), (Float::ZERO, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.reciprocal_round(rm),
            (Float::NEGATIVE_ZERO, Equal)
        );
        assert_eq!(Float::ZERO.reciprocal_round(rm), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_ZERO.reciprocal_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert_eq!(Float::ONE.reciprocal_round(rm), (Float::ONE, Equal));
        assert_eq!(
            Float::NEGATIVE_ONE.reciprocal_round(rm),
            (Float::NEGATIVE_ONE, Equal)
        );
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn reciprocal_properties_helper_2<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let reciprocal_1 = x.reciprocal();
        let reciprocal_2 = emulate_primitive_float_fn(|x, prec| x.reciprocal_prec(prec).0, x);
        assert_eq!(NiceFloat(reciprocal_1), NiceFloat(reciprocal_2));
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_properties_helper(x: Float) {
    let reciprocal = x.clone().reciprocal();
    assert!(reciprocal.is_valid());

    let reciprocal_alt = (&x).reciprocal();
    assert!(reciprocal_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_alt),
        ComparableFloatRef(&reciprocal)
    );

    let mut x_alt = x.clone();
    x_alt.reciprocal_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&reciprocal));

    assert_eq!(
        ComparableFloatRef(
            &reciprocal_prec_round_naive_1(x.clone(), x.significant_bits(), Nearest).0
        ),
        ComparableFloatRef(&reciprocal)
    );
    assert_eq!(
        ComparableFloatRef(
            &reciprocal_prec_round_naive_2(x.clone(), x.significant_bits(), Nearest).0
        ),
        ComparableFloatRef(&reciprocal)
    );

    let rug_reciprocal = rug_reciprocal(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_reciprocal)),
        ComparableFloatRef(&reciprocal),
    );

    assert_eq!(
        ComparableFloatRef(&(Float::ONE / x.clone())),
        ComparableFloatRef(&reciprocal)
    );

    if reciprocal.is_normal() && x.is_finite() {
        assert_eq!(reciprocal.get_prec(), Some(x.significant_bits()));
    }

    let mut reciprocal_alt = (-&x).reciprocal();
    reciprocal_alt.neg_assign();
    assert_eq!(
        ComparableFloat(reciprocal_alt.abs_negative_zero()),
        ComparableFloat(reciprocal.abs_negative_zero_ref())
    );
}

#[test]
fn reciprocal_properties() {
    float_gen().test_properties(|x| {
        reciprocal_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        reciprocal_properties_helper(x);
    });

    float_gen_var_6().test_properties(|x| {
        reciprocal_properties_helper(x);
    });

    float_gen_var_7().test_properties(|x| {
        reciprocal_properties_helper(x);
    });

    float_gen_var_8().test_properties(|x| {
        reciprocal_properties_helper(x);
    });

    float_gen_var_11().test_properties(|x| {
        reciprocal_properties_helper(x);
    });

    apply_fn_to_primitive_floats!(reciprocal_properties_helper_2);
}
