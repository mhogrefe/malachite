// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, rounding_mode_gen, unsigned_gen_var_11,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::test_util::arithmetic::square::{
    rug_square, rug_square_prec, rug_square_prec_round, rug_square_round, square_prec_round_naive,
};
use malachite_float::test_util::common::{
    emulate_primitive_float_fn, parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_10, float_gen_var_12, float_gen_var_6, float_gen_var_7,
    float_gen_var_8, float_gen_var_9, float_rounding_mode_pair_gen_var_10,
    float_rounding_mode_pair_gen_var_11, float_rounding_mode_pair_gen_var_12,
    float_rounding_mode_pair_gen_var_22, float_rounding_mode_pair_gen_var_7,
    float_rounding_mode_pair_gen_var_8, float_rounding_mode_pair_gen_var_9,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_11, float_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use std::panic::catch_unwind;

#[test]
fn test_square() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let square = x.clone().square();
        assert!(square.is_valid());

        assert_eq!(square.to_string(), out);
        assert_eq!(to_hex_string(&square), out_hex);

        let square_alt = (&x).square();
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));

        let mut square_alt = x.clone();
        square_alt.square_assign();
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_square(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&square)
        );

        let square_alt = square_prec_round_naive(x.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "Infinity", "Infinity");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "1.0", "0x1.0#1");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );

    test("123.0", "0x7b.0#7", "1.51e4", "0x3.b0E+3#7");
    test("-123.0", "-0x7b.0#7", "1.51e4", "0x3.b0E+3#7");
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "2.0000000000000004",
        "0x2.0000000000002#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "2.0000000000000004",
        "0x2.0000000000002#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
    );

    // - in square_float_significand_same_prec_lt_w
    // - decrement_exp in square_float_significand_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    // - !decrement_exp in square_float_significand_same_prec_lt_w
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_lt_w
    // - rm == Nearest in square_float_significand_same_prec_lt_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && square & shift_bit == 0)) in
    //   square_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", "2.0", "0x2.0#2");
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || square & shift_bit != 0) &&
    //   !overflow in square_float_significand_same_prec_lt_w
    test("1.6", "0x1.a#4", "2.8", "0x2.c#4");
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || square & shift_bit != 0) && overflow
    //   in square_float_significand_same_prec_lt_w
    test("1.414", "0x1.6a#8", "2.0", "0x2.00#8");
    // - in square_float_significand_same_prec_w
    // - decrement_exp in square_float_significand_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
    );
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_w
    // - rm == Nearest in square_float_significand_same_prec_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && product.even())) in
    //   square_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
    );
    // - !decrement_exp in square_float_significand_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || product.odd()) && !overflow in
    //   square_float_significand_same_prec_w
    test(
        "3.2729513077064011786e-37",
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        "1.0712210262617041571e-73",
        "0x3.073e45e79ac604c4E-61#64",
    );
    test(
        "1.9999999999999999999",
        "0x1.fffffffffffffffe#64",
        "3.9999999999999999996",
        "0x3.fffffffffffffff8#64",
    );
    test(
        "1.9999999999999999998",
        "0x1.fffffffffffffffc#64",
        "3.9999999999999999991",
        "0x3.fffffffffffffff0#64",
    );
    test(
        "1.4142135623730950488",
        "0x1.6a09e667f3bcc908#64",
        "1.9999999999999999999",
        "0x1.fffffffffffffffe#64",
    );
    test(
        "1.4142135623730950489",
        "0x1.6a09e667f3bcc90a#64",
        "2.0000000000000000002",
        "0x2.0000000000000004#64",
    );

    // - in square_float_significand_same_prec_gt_w_lt_2w
    // - lo.wrapping_add(2) & (mask >> 2) <= 2 in square_float_significand_same_prec_gt_w_lt_2w
    // - decrement_exp in square_float_significand_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
    );
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest in square_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && (z_0 & shift_bit) == 0) in
    //   square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
    );
    // - lo.wrapping_add(2) & (mask >> 2) > 2 in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "1.00000000000000000022",
        "0x1.0000000000000004#65",
    );
    // - !decrement_exp in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "2.0742001767277848782373298e56",
        "0x8.7590e74562e8c0aeed1d0E+46#85",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && !overflow
    //   in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "119368.6474438890389479272222539538",
        "0x1d248.a5bee1f96ad66a5061314f7#109",
        "14248873992.58347719172623084795998",
        "0x3514c9008.955ec2e06d13cb2d862#109",
    );
    // - in square_float_significand_same_prec_gt_2w_lt_3w
    // - a0.wrapping_add(4) & (mask >> 2) <= 4 in square_float_significand_same_prec_gt_2w_lt_3w
    // - decrement_exp in square_float_significand_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
    );
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_gt_2w_lt_3w
    // - rm == Nearest in square_float_significand_same_prec_gt_2w_lt_3w
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && z_0 & shift_bit == 0) in
    //   square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
    );
    // - a0.wrapping_add(4) & (mask >> 2) > 4 in square_float_significand_same_prec_gt_2w_lt_3w
    // - !decrement_exp in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "4.09688648907491713333499964381160220254051958e-58",
        "0x2.9258227caed7c4000630a1192a20211680c74E-48#149",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) in
    //   square_float_significand_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && !overflow
    //   in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "4.9709672065181108960570410290811793724062284431352e-48",
        "0x7.43dc113e95ca123693650af31435eac45c0e7a680E-40#165",
        "2.47105149682784709830100902225310999174453232836862e-95",
        "0x3.4c805dfa0982f9705aa6bbd6840b755493beef234E-79#165",
    );
}

#[test]
fn test_square_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (square, o) = x.clone().square_prec(prec);
        assert!(square.is_valid());

        assert_eq!(square.to_string(), out);
        assert_eq!(to_hex_string(&square), out_hex);
        assert_eq!(o, o_out);

        let (square_alt, o_alt) = x.square_prec_ref(prec);
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));
        assert_eq!(o_alt, o_out);

        let mut square_alt = x.clone();
        let o_alt = square_alt.square_prec_assign(prec);
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));
        assert_eq!(o_alt, o_out);

        let (square_alt, o_alt) = square_prec_round_naive(x.clone(), prec, Nearest);
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
        assert_eq!(o_alt, o);

        let (rug_square, rug_o) = rug_square_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_square)),
            ComparableFloatRef(&square),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 10, "1.0", "0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        "1.0",
        "0x1.000#10",
        Equal,
    );

    test("123.0", "0x7b.0#7", 1, "2.0e4", "0x4.0E+3#1", Greater);
    test("123.0", "0x7b.0#7", 10, "1.514e4", "0x3.b2E+3#10", Greater);
    test("-123.0", "-0x7b.0#7", 1, "2.0e4", "0x4.0E+3#1", Greater);
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        "2.0",
        "0x2.00#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        "2.0",
        "0x2.00#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "9.88",
        "0x9.e0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "9.88",
        "0x9.e0#10",
        Greater,
    );

    // - in square_float_significands_general
    // - xs_len <= 2 in square_float_significands_general
    // - xs_len == 1 in square_float_significands_general
    // - !b1 first time in square_float_significands_general
    // - !goto_full_square second time in square_float_significands_general
    // - !increment_exp in square_float_significands_general
    test("1.0", "0x1.0#1", 2, "1.0", "0x1.0#2", Equal);
    // - b1 first time in square_float_significands_general
    test("1.5", "0x1.8#2", 1, "2.0", "0x2.0#1", Less);
    // - increment_exp in square_float_significands_general
    test("1.2", "0x1.4#3", 1, "2.0", "0x2.0#1", Greater);
    // - xs_len < 2 <= MPFR_SQR_THRESHOLD in square_float_significands_general
    // - goto_full_square second time in square_float_significands_general
    // - b1 third time in square_float_significands_general
    test(
        "0.000199046277632504184666664672269768242929310652018203552191617720205649",
        "0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225",
        26,
        "3.9619421e-8",
        "0xa.a2a038E-7#26",
        Less,
    );
    // - xs_len == 2 in square_float_significands_general
    test(
        "330.3297903358337046735382484655688",
        "0x14a.546d23b2f14f71a1f4c12ddaa88#115",
        45,
        "109117.770383317",
        "0x1aa3d.c537d75#45",
        Greater,
    );
    // - !b1 third time in square_float_significands_general
    test(
        "88.32972592752556369746097031876493672699524930031683012093294198918",
        "0x58.5468eb1b5d957d68d5c161060f2abd3d11568e57fb44ace5b9530c#222",
        41,
        "7802.140482433",
        "0x1e7a.23f6a82#41",
        Greater,
    );
    // - xs_len > MPFR_SQR_THRESHOLD in square_float_significands_general
    // - xs[0] != 0 || xs[1] != 0 in square_float_significands_general
    // - out_prec <= p - 5 in square_float_significands_general
    // - !goto_full_square first time in square_float_significands_general
    // - in limbs_float_square_high
    // - k.is_none() in limbs_float_square_high
    // - b1 second time in square_float_significands_general
    // - can_round in square_float_significands_general
    test(
        "396.3412272551190567593891335756570564001425784787446073953740531440207891605323131867948\
        869355406520857599475406495997289490143735096829625551391579153111055037678989978858579586\
        021105696471344997604889396207883425571313379937012790014662248134632025438319010426957144\
        355546486040271820495868974546882981476343977667254209899704548500299738343700283443799494\
        532151589935402512791651191411866655264587358069356924819557816073395721436815590916015139\
        840293409397442",
        "0x18c.575aab5d3d7d931fe7b1df9a17c5180eb11799ddcdb0b514b818c68be091c380b4f49e26c6357fc5521\
        89c83f7fb1af51d3ad56a7621351488217dec257419292d332960be3dcbdbf0989c56e4cc82fcee123ce6c00b8\
        63f9e45062bf3fbae296936b2ab5e3779905657246488ffaf88040c9e3456d8e0dcb25c0b55feba5452509320e\
        91f9be5ce584d555f8c77c79aeec4f5bb5aff76c235d04d87f5adafe9a729dccc5692f11d14f4aeeec72b2dc3e\
        c35bf6b0a8d1d636bb2afad1ab82c0#1539",
        41,
        "157086.3684222",
        "0x2659e.5e50ea#41",
        Greater,
    );
    // - !b1 second time in square_float_significands_general
    test(
        "0.289565969492664694638578202456729810638628879666435586140728331245058747654800231361781\
        733367215018660480916949243715433401890725345322341436886642908102373347900363371546861032\
        804628537167569586473086484745604528149555432827776527274466086199528238950727774219761110\
        278394595668030807862161516149163941956835588098749888947102946654461061159884953097421811\
        149735301912628233431116932895412543378996752139902311082164451547359831823770756258588441\
        392364079976282812844068282256919410425900496233045740740436555471195699615728406687884353\
        411870520651275282496307876556394549674324563102261561135259821040813150131202195032317605\
        601108576080171231347456850423368707704016287732147225807113778613580546356726188558446009\
        37667532705200099920676965417288103441",
        "0x0.4a20fed1016a521a64864d01138a193df1e23f2f35edd21912c8d7ea345216c5ec7c59aca3d2ce1378dd1\
        89d7419984353ecef1dc303caddc54cc6cf871a7d44df07922bfe75b3e27fa2ac58a45a2700c2b9d233b14afbe\
        c4aeeed4683073d061d99c2f92084cbae1d1bf26f8fb2323910d590864daa5f4b534e0509ff60a7238357ab7ff\
        36bcc0024c64f4507f522e91df687d9649e2e609840feea945bd53cec1fec45c7ccee6ad51aacd8c59427836f6\
        871c316bc3ba008d00a81638ec6c5630fafa1ebe3c492a94c04c96f772e57d74f808872161bf166402ea4e9dd0\
        42397a51a2e1ffd33b8865cca279beaaf61c288bdc67dceb9e84f124abd1c1016bfc1d67e916ef24dc5a541f8c\
        e0888c25325a3955899b701e5035429c76e90b7de02973eaa08674d6315156df3c4afdd962afc0b2fb855c12c7\
        cb#2507",
        41,
        "0.08384845068821",
        "0x0.1577179186b#41",
        Less,
    );
    // - k == Some(0) in limbs_float_square_high
    // - in limbs_float_square_high_basecase
    test(
        "2.534905093269173913399961419051078001207362536676304036217117050023406987769089211085345\
        845282321276763762108762278121117963061716139888836603176571082592497694546991532956806762\
        060078393566599448512584861538013806584451166069797239642908909634341260487656925769615819\
        575456879212968568058838065343005029399246396366977778814769813140337173953816940118229696\
        975910769633258534685767958638383299293048391708530965415961893726949253541358150164947850\
        354131943712963257075685505691799739914956071583726713409225265256065457404089250831362505\
        964054898560581202320837085103659819920563909237065863856674551755094031141043600118470396\
        110548066038755307598657006758419658655479386061317383935495778887020750969220495044500181\
        502836698351684979845272389537925642124685759621079734656489541651071700522564930994999675\
        548215393164945318735677787876975365012973524228071379171196099071790619084107585184183873\
        649229987470512431316532998401599147457982188040992886626796443210441148620644905909977484\
        092106548023293796713001951271606863906640116960398160889963208431853184379130926702446309\
        16665296291511e-18",
        "0x2.ec2c037afc439f03a036777b3b49c929652bc0198062ee846d167a89e9a1b649beaab8e3935d29165383b\
        8032ab14cd30954f6e10277bf2adbaa3dc7f4af85aa6a3dfe37f0afee4bafa862c5d97d6b0ad6fe3a473f086a0\
        d78eaa4db0710879d93d26ada2de74e4133838c88fda61bf559b836a376e2ece72417c51fff1870b23e21aa228\
        c6b132e5b2dea393b782ddc27593a878aec5ddd0254c2fe5515f3036b02fc8ab225f7433c0e931717894c1f581\
        86927d1a1d3ae8a8d5a52d6ffd5bb9a6f1d8c55f9c54d16411a5a7a6bf1616d3eaf4692379eb81ba2b11179269\
        987c75a403cd7845851b66bf9758892745e79ac67a6a8feb7dcdcb16690d8fae58f96bc20e8495bdee03ef7ad7\
        d1821419ec1bfe2bb5b49cbfcab0f7a3523939693c97830d3c6e027badb91529d6076972fcea0ace6d83b34f64\
        0da99faaa4862efc02a02d3a5066111bfc08e28a6b55ff9e7c245b0232f0a85e97a68fa69e8d218bce6abe990a\
        2e0b503cf831ad5df31caba8915c38bad1cc785b9e99b3bb855801ec29ad43668762f37f228f77bf7a8590649f\
        937d5dc5039a46479d0dac0be7aa72929e8868980b3f76831bf434c15ba83ba74394414959b1c8bc010f735a6c\
        14c8b93afc80E-15#3627",
        423,
        "6.425743831881999297033060889967219634391598848272085805147756300415778053639020164208035\
        3509860715112301888671841760135704785647e-36",
        "0x8.8a913be7e47dfd7ad8403512f1a0ee8d3fad7395038c39eb4039b4703fdcd0314b45e761b67375d18eb58\
        ced237cd5fd6c79c02dcE-30#423",
        Less,
    );
    // - xs[0] == 0 && xs[1] == 0 in square_float_significands_general
    test(
        "511.9999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999858969189385560189369588899908301051658131815921691396545986807150339147015462\
        016914098983479659782731543057846003175884332840513803623318182318832820724717407320371823\
        6294395745915856",
        "0x1ff.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000#1539",
        45,
        "262144.0",
        "0x40000.0000000#45",
        Greater,
    );
    // - !can_round in square_float_significands_general
    test(
        "5070602400912917605986812821504.000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000072282982868771681166004491412846722101397844122267684138688870467552820474559090102317\
        447234871538375909335604136421707865093288809202542467085297095776399695369977155046440218\
        72699461509694558652328992590065121835881232",
        "0x40000000000000000000000000.000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffe#2230",
        75,
        "2.5711008708143844408671e61",
        "0x1.0000000000000000000E+51#75",
        Less,
    );
    // - k != None && k != Some(0) && len <= SQR_FFT_THRESHOLD in limbs_float_square_high
    test(
        "396.3412272551190567593891335756570564001425784787446073953740531440207891605323131867948\
        869355406520857599475406495997289490143735096829625551391579153111055037678989978858579586\
        021105696471344997604889396207883425571313379937012790014662248134632025438319010426957144\
        355546486040271820495868974546882981476343977667254209899704548500299738343700283443799494\
        532151589935402512791651191411866655264587358069356924819557816073395721436815590916015139\
        840293409397442",
        "0x18c.575aab5d3d7d931fe7b1df9a17c5180eb11799ddcdb0b514b818c68be091c380b4f49e26c6357fc5521\
        89c83f7fb1af51d3ad56a7621351488217dec257419292d332960be3dcbdbf0989c56e4cc82fcee123ce6c00b8\
        63f9e45062bf3fbae296936b2ab5e3779905657246488ffaf88040c9e3456d8e0dcb25c0b55feba5452509320e\
        91f9be5ce584d555f8c77c79aeec4f5bb5aff76c235d04d87f5adafe9a729dccc5692f11d14f4aeeec72b2dc3e\
        c35bf6b0a8d1d636bb2afad1ab82c0#1539",
        1000,
        "157086.3684220939290392835990654860444953123468682922432646919273797192952403374080185049\
        441414358725029167080302705853255170809678818388441806686518494713508728113056316939955366\
        580321349638671885788642336442940811575310873763709312578497006621517503935000405848895060\
        9190911436529068384755172193385643",
        "0x2659e.5e50e90c8c8f731b6ec42e31ac581c6d5c6e7051268a54ce30dbe17faafc9ad4523be4827db001d48\
        8a322a36c3d610868daa74b44da3bd6945b467643769f500a263ca6634a2786c8e089cd305b177ffba7d9cf923\
        74c0483fa9a736b452c7e65bfe13f1fbababe9090e56a0abfa7e4982915fe0dff247e8378e8#1000",
        Greater,
    );
    // - out_prec > p - 5 in square_float_significands_general
    // - out_prec > p - 5 + Limb::WIDTH || xs_len <= MPFR_SQR_THRESHOLD + 1 in
    //   square_float_significands_general
    // - goto_full_square first time in square_float_significands_general
    test(
        "396.3412272551190567593891335756570564001425784787446073953740531440207891605323131867948\
        869355406520857599475406495997289490143735096829625551391579153111055037678989978858579586\
        021105696471344997604889396207883425571313379937012790014662248134632025438319010426957144\
        355546486040271820495868974546882981476343977667254209899704548500299738343700283443799494\
        532151589935402512791651191411866655264587358069356924819557816073395721436815590916015139\
        840293409397442",
        "0x18c.575aab5d3d7d931fe7b1df9a17c5180eb11799ddcdb0b514b818c68be091c380b4f49e26c6357fc5521\
        89c83f7fb1af51d3ad56a7621351488217dec257419292d332960be3dcbdbf0989c56e4cc82fcee123ce6c00b8\
        63f9e45062bf3fbae296936b2ab5e3779905657246488ffaf88040c9e3456d8e0dcb25c0b55feba5452509320e\
        91f9be5ce584d555f8c77c79aeec4f5bb5aff76c235d04d87f5adafe9a729dccc5692f11d14f4aeeec72b2dc3e\
        c35bf6b0a8d1d636bb2afad1ab82c0#1539",
        5000,
        "157086.3684220939290392835990654860444953123468682922432646919273797192952403374080185049\
        441414358725029167080302705853255170809678818388441806686518494713508728113056316939955366\
        580321349638671885788642336442940811575310873763709312578497006621517503935000405848895060\
        919091143652906838475517219338564216871512733323414114900025582889108770319888495423472591\
        622217081406162593383439817010743870950284355953948594596501930134914144626602460886577742\
        699397445023366416628676609358289067494944516044161793867980863078877928160882497479870173\
        995550846564918220947311792283316244784987871826325087158139795457567671300755416634197565\
        825272103067497350477832613308006740370014538444363794113999101665422314265225122590389544\
        388555686073293478850435707082986891289816937189501867266947100104879700862539689861260203\
        303222356049565246944094013202651746222554947384760413370539804854333289940364855469937239\
        014120251649267431651939212811005766277308077834857972931251756411522886575455852040387255\
        586633599190053514685721006798676467868675709844997689304699851005119605477859913661835009\
        980047325267722717984269099465289430789912767924820742466247451598658072993390431860927262\
        510441796888642732252820490424273307195428579227975601954580861122408746440602416210603394\
        010320119854126656872002234654813295901729990426282838623889803078731960696692381273900064\
        363258066357675869791572233715232583553618635258761844046010323346239050118033120274686587\
        47626978436794438933236478602171106153180118157402691906229607357337",
        "0x2659e.5e50e90c8c8f731b6ec42e31ac581c6d5c6e7051268a54ce30dbe17faafc9ad4523be4827db001d48\
        8a322a36c3d610868daa74b44da3bd6945b467643769f500a263ca6634a2786c8e089cd305b177ffba7d9cf923\
        74c0483fa9a736b452c7e65bfe13f1fbababe9090e56a0abfa7e4982915fe0dff247e8378e637c71563f40ac88\
        bff59ccb63d55f07b40f44324086d33c752ca2a398f8021f9ab7e61160ce80b0eb4d350cfcb2745c0ffb94a7ea\
        c2a42031e1d67c39dabea5ed0ed1839b3cb19c95afad38ff4d85fe726701b1400759bff428fc5d441579646a9a\
        edbda6e96c39642ea5102dcc1744e2f22bc2669606b02248465eda50f7ab00888fe70cd5b94c6886478a9d89d4\
        721fd315aae640835a05a2ac14cf927011012c9a0b12526c935c3b86e035f9369aa246b7f864ac313401679f2d\
        585686dd60ca3610690fc6e131d703a07afa9f50e1c62593876e5dfcd53a06dfc5c84a2d7c8129428f50ff8669\
        462d4cbbacfe9e72bf3cd17875fb28dd45607447a1bdd656f47900000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        #5000",
        Equal,
    );
    // - out_prec <= p - 5 + Limb::WIDTH && xs_len > MPFR_SQR_THRESHOLD + 1 in
    //   square_float_significands_general
    // - xs_len <= len in square_float_significands_general
    // - k < twice_len in square_float_significands_general
    test(
        "70459532516206275717738.24583571192785940804485220379113093012216854237552273164629839052\
        295542459627091867686732592292054910255896821543632944963319945361030770270423888774548519\
        352455933682416235026529894320254192088358055829902767990508592251676851896653974385735963\
        818455173075208105139936148350808080166279329650970874228499292765775926866402018763767601\
        932631118142152227916465948527446827475834919059052275938611139784416871828297279794775881\
        754156488112402263366312959738244152841157181731104967167515387928211427131697792191239671\
        200062974233522321766736844717284659803787329075216256925936831334895804054605134987158729\
        635284806758566129642530508638215601547746106827192539740893000302521313674983124372016914\
        389464631779788838617640381788549537988633853246170604449289457144019599904363663940307935\
        010893280729695200089400652615175194820160750890242280186267733830422108880046398617008039\
        346196610860314292217862221940568925947020858793049858538542201667012791629444317697230994\
        497432161328637397726885087365856366871387393212442806903408576734184446519785519751530501\
        203958503763892329891980999384336300765212927244594343049091702158556853491733512332595607\
        980340866258213963813028256778277899307861580399445091884912999955742998432553541817046928\
        089178846440489676733765036119262237349966327782496860922729317780078565098542288671006633\
        263750319390117261972206095532634412489779996284041680459224965604552001326514848875913522\
        75401658328725688546699002488305760364795486416541",
        "0xeeb9e70f8e3103fea6a.3eef16d6eb45c3a998fb982ebe639fc8759b70c44fe5810571a8155bfb031e4bc24\
        7dda5a8581f5ab0434d47c9eb35f6a9fb469e23b989044db9aefcec6a8d1de5bf14332cc4bb1bc75c32a4e36cc\
        f907a5658e025a6f217cb147f7a8ce671eee07c655ecbe2598eb280ca4d26207f6c1e55ed67b66c69d289c56d3\
        4f2d2b25f411c0b9e4696d67a2ebe0b5716309bb26e24a3cc52943382e8956c46f600932e74aed277d50d0b801\
        d8cdf51e8d20373cc3c14623e93446643f969b662cd3d7da7f3f840246325c873da49be886e93a9f2b9debb19d\
        6b28dc9e7e168f16ac806d56ffbeb1836aff27be850217d6311ab97d9742412187d3ecde0f7d4a0c89c6b46c37\
        610de9e5d478f868be38875f5c45f683618305161b4a1e6811cc1c53c2798dddbe1daba42c542637900cae7fe8\
        7cb8178a8825ae05ae6c791d728ab540947bc63fcb32f5af0d89b211e00aa940c8ccb7aa1ff76a1590b970f2c4\
        010fddbeb773600a0bd98bc005244e5ce6bf94d6ff4be6c4ddaffe2d3f89ae99eb2b79591c994977331c5caa40\
        d57d28f6fa0bd1458390ffd906d84e849964bf2c3afb5fa3e251d60fb42251182bc006f0da82dd32c5877d3183\
        b771fa955b64fe611c8a57edf9070898139a48e93a9f7cf1bc280142a0c900175e97125e4dba6cd579985765eb\
        ea2bf0ecfb3754e8373897fd5f3fbdd8a04c429f1eb274d27b90c5ba4cdc5221ee37d3885cc6b8a0745a21d803\
        9a0571a0a77248c1a21338919b63ada2c014b8d1864ad65dc80551ded9cd5e048f7a262866d51e252ee527629b\
        0628f7561340ddc20398c57d6f3159cec6f4979a02c5610d3743e31aa8993d652c6ea0#4942",
        5000,
        "4964545722402329471538520890548020778505727316.714928212970963455240096184214428743957383\
        329922752233199764049780878042702316184608300084915002665385500366536453418018453492928341\
        017054541630873437214644149654854522755654366831843415669733075028381314388954345055868246\
        000218989221937469258574896624097067887464348708304868331849053151758619452575265628276389\
        682599022898309226512794404632814475266579950972366209872195059885305874209009896969746959\
        550374594439111632193759127250892146888326838744623554050318490109344201586718804154701606\
        966010758459087950191998865560033257935278780535428646473292047088396527777426223171234483\
        188009004262125992356660280897524089496628310376755558332111429032108512629018171093951768\
        666071288536225840334502196980458876275123956423686107784807652541652782835788006847858977\
        993995691507283189780038044528425354524130033815262491772309833202212299992561581175232232\
        053665702559586771762340461465421956694347639348699503880840804336286789065206696824702252\
        001401065405887247144705146834255039213991488108844897575368853688286293567906566377054487\
        855713514655373784168449710429759002204390811345191073301339247358231849563161226052987665\
        594985477181835691490984499872892234197121121888658800164578709448237427211277623000068159\
        736988165669283234595563391379976241167126662466254710156731663896405093320111508971147717\
        596133787650005180714732054756611220213766696881623761124569357842183971615077248468826825\
        78028047188611500722085706480035062367867718429318751607371955873121",
        "0xde9e30a0b16bb9d3b1d41b9ca941523105ed54.b705890db2b0d9f3e32b9f103e1b83c7f9edba5c1f4e2af3\
        adeb66de82892d7410e07fd6532bc0baf1535d9dfdf455d02d7b3cdcd6b683dd9909ef54c1c3138d14f10d53ef\
        a61999322d530e26efcb69e290c671098fb49bb695602d0e25cb7d993dc249014fc873b17a61bb8c9a878cb42e\
        f205fa6138dbfc05bd5c65440690ca6b0054108cab3099860992a8d16ad6797a10fba9af26f477b820e5a0499c\
        bf71480aff1edfd22ad78810350210eae0cec2b5d0248df96b630be8372423d110cdd49422b85db636542c44ca\
        d9c82a307aaeb2d1795d93184dd19dc0e3bd6c9cdc631fe51a88f0a178def01f6ae41386a6563a14613e79ab7d\
        b81b5373a481694455cc8778c037bd3a3c6e2b7a0f448025095104d91359860535455349496f70c9ca535c3e69\
        8563c5e868485868fb51e33f09ed11186e72d4c6b2ce9f4b7629a78b907f198ac6cdde2f6642df2b1c9737ce1a\
        48ac426a8d401ef2aa8acbd3185bfd87dc282064adc7145f4dc07a12d6b35f38fb04eb2c4da98e45b7287d9982\
        2aea6ee0f4d2b8771272e2f4e9d0afb433896b20f842155ae13c2df308b254065539842f8cb05363f97ef29bbb\
        6b1fd5ac471efcf6cd026fe29de50b9a08d5e4610e95cba281a1006f876c351b4b8c381caf1f9f849a3ad17f6b\
        fed7e727b78c97f11622da2ae8fd3c5759f030ea2800bd1d267f019d2292f18d7360d036e88707bbd93901fda7\
        c8fa8cfeeb7aca62069cec0809ae3add2dfdbffa773246c8ae9749ac6c8b0cced5911da8050ea0171f866099b7\
        44d3739341181361c2df7c3d9320134683888ed78e4d80fc191958b201777943c0cac9c3c082a01b24d3#5000",
        Less,
    );
}

#[test]
fn square_prec_fail() {
    assert_panic!(Float::NAN.square_prec(0));
    assert_panic!(Float::NAN.square_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.square_prec_assign(0)
    });
}

#[test]
fn test_square_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (square, o) = x.clone().square_round(rm);
        assert!(square.is_valid());

        assert_eq!(square.to_string(), out);
        assert_eq!(to_hex_string(&square), out_hex);
        assert_eq!(o, o_out);

        let (square_alt, o_alt) = x.square_round_ref(rm);
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));
        assert_eq!(o_alt, o_out);

        let mut square_alt = x.clone();
        let o_alt = square_alt.square_round_assign(rm);
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_square, rug_o) = rug_square_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_square)),
                ComparableFloatRef(&square),
            );
            assert_eq!(rug_o, o);
        }

        let (square_alt, o_alt) = square_prec_round_naive(x.clone(), x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
        assert_eq!(o_alt, o);
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
    test("Infinity", "Infinity", Down, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal);

    test(
        "-Infinity",
        "-Infinity",
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", Up, "Infinity", "Infinity", Equal);
    test(
        "-Infinity",
        "-Infinity",
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Ceiling, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Down, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Up, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Exact, "0.0", "0x0.0", Equal);

    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Exact, "1.0", "0x1.0#1", Equal);

    test("-1.0", "-0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Exact, "1.0", "0x1.0#1", Equal);

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
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test("123.0", "0x7b.0#7", Floor, "1.51e4", "0x3.b0E+3#7", Less);
    test(
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "1.52e4",
        "0x3.b8E+3#7",
        Greater,
    );
    test("123.0", "0x7b.0#7", Down, "1.51e4", "0x3.b0E+3#7", Less);
    test("123.0", "0x7b.0#7", Up, "1.52e4", "0x3.b8E+3#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "1.51e4", "0x3.b0E+3#7", Less);

    test("-123.0", "-0x7b.0#7", Floor, "1.51e4", "0x3.b0E+3#7", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        Ceiling,
        "1.52e4",
        "0x3.b8E+3#7",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", Down, "1.51e4", "0x3.b0E+3#7", Less);
    test("-123.0", "-0x7b.0#7", Up, "1.52e4", "0x3.b8E+3#7", Greater);
    test(
        "-123.0",
        "-0x7b.0#7",
        Nearest,
        "1.51e4",
        "0x3.b0E+3#7",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Floor,
        "2.0",
        "0x2.0000000000000#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Ceiling,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Down,
        "2.0",
        "0x2.0000000000000#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Up,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Nearest,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Floor,
        "2.0",
        "0x2.0000000000000#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Ceiling,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Down,
        "2.0",
        "0x2.0000000000000#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Up,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Nearest,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "9.869604401089356",
        "0x9.de9e64df22ee8#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "9.869604401089356",
        "0x9.de9e64df22ee8#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "9.869604401089356",
        "0x9.de9e64df22ee8#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "9.869604401089356",
        "0x9.de9e64df22ee8#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "9.869604401089358",
        "0x9.de9e64df22ef0#53",
        Greater,
    );

    // - rm == Floor || rm == Down in square_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", Down, "2.0", "0x2.0#2", Less);
    // - rm == Ceiling || rm == Up in square_float_significand_same_prec_lt_w
    // - (rm == Ceiling || rm == Up) && !overflow in square_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", Up, "3.0", "0x3.0#2", Greater);
    // - (rm == Ceiling || rm == Up) && overflow in square_float_significand_same_prec_lt_w
    test("1.4", "0x1.6#4", Up, "2.0", "0x2.0#4", Greater);
    // - rm == Floor || rm == Down in square_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Down,
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        Less,
    );
    // - rm == Ceiling || rm == Up in square_float_significand_same_prec_w
    // - (rm == Ceiling || rm == Up) && !overflow in square_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Up,
        "1.0000000000000000003",
        "0x1.0000000000000006#64",
        Greater,
    );
    // - rm == Floor || rm == Down in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Down,
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Less,
    );
    // - rm == Ceiling || rm == Up in square_float_significand_same_prec_gt_w_lt_2w
    // - (rm == Ceiling || rm == Up) && !overflow in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Up,
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        Greater,
    );
    // - rm == Floor || rm == Down in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Down,
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
        Less,
    );
    // - rm == Ceiling || rm == Up in square_float_significand_same_prec_gt_2w_lt_3w
    // - (rm == Floor || rm == Down) && !overflow in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Up,
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        Greater,
    );

    // - in square_float_significand_same_prec_lt_w
    // - decrement_exp in square_float_significand_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_lt_w
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    // - !decrement_exp in square_float_significand_same_prec_lt_w
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_lt_w
    // - rm == Nearest in square_float_significand_same_prec_lt_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && square & shift_bit == 0)) in
    //   square_float_significand_same_prec_lt_w
    test("1.5", "0x1.8#2", Nearest, "2.0", "0x2.0#2", Less);
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || square & shift_bit != 0) &&
    //   !overflow in square_float_significand_same_prec_lt_w
    test("1.6", "0x1.a#4", Nearest, "2.8", "0x2.c#4", Greater);
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || square & shift_bit != 0) && overflow
    //   in square_float_significand_same_prec_lt_w
    test("1.414", "0x1.6a#8", Nearest, "2.0", "0x2.00#8", Greater);
    // - in square_float_significand_same_prec_w
    // - decrement_exp in square_float_significand_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        Nearest,
        "1.0",
        "0x1.0000000000000000#64",
        Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_w
    // - rm == Nearest in square_float_significand_same_prec_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && product.even())) in
    //   square_float_significand_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Nearest,
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        Less,
    );
    // - !decrement_exp in square_float_significand_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || product.odd()) && !overflow in
    //   square_float_significand_same_prec_w
    test(
        "3.2729513077064011786e-37",
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        Nearest,
        "1.0712210262617041571e-73",
        "0x3.073e45e79ac604c4E-61#64",
        Greater,
    );
    test(
        "1.9999999999999999999",
        "0x1.fffffffffffffffe#64",
        Nearest,
        "3.9999999999999999996",
        "0x3.fffffffffffffff8#64",
        Less,
    );
    test(
        "1.9999999999999999998",
        "0x1.fffffffffffffffc#64",
        Nearest,
        "3.9999999999999999991",
        "0x3.fffffffffffffff0#64",
        Less,
    );
    test(
        "1.4142135623730950488",
        "0x1.6a09e667f3bcc908#64",
        Nearest,
        "1.9999999999999999999",
        "0x1.fffffffffffffffe#64",
        Less,
    );
    test(
        "1.4142135623730950489",
        "0x1.6a09e667f3bcc90a#64",
        Nearest,
        "2.0000000000000000002",
        "0x2.0000000000000004#64",
        Greater,
    );

    // - in square_float_significand_same_prec_gt_w_lt_2w
    // - lo.wrapping_add(2) & (mask >> 2) <= 2 in square_float_significand_same_prec_gt_w_lt_2w
    // - decrement_exp in square_float_significand_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        Nearest,
        "1.0",
        "0x1.0000000000000000#65",
        Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest in square_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && (z_0 & shift_bit) == 0) in
    //   square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Less,
    );
    // - lo.wrapping_add(2) & (mask >> 2) > 2 in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Nearest,
        "1.00000000000000000022",
        "0x1.0000000000000004#65",
        Less,
    );
    // - !decrement_exp in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        Nearest,
        "2.0742001767277848782373298e56",
        "0x8.7590e74562e8c0aeed1d0E+46#85",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && !overflow
    //   in square_float_significand_same_prec_gt_w_lt_2w
    test(
        "119368.6474438890389479272222539538",
        "0x1d248.a5bee1f96ad66a5061314f7#109",
        Nearest,
        "14248873992.58347719172623084795998",
        "0x3514c9008.955ec2e06d13cb2d862#109",
        Greater,
    );
    // - in square_float_significand_same_prec_gt_2w_lt_3w
    // - a0.wrapping_add(4) & (mask >> 2) <= 4 in square_float_significand_same_prec_gt_2w_lt_3w
    // - decrement_exp in square_float_significand_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        Nearest,
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in square_float_significand_same_prec_gt_2w_lt_3w
    // - rm == Nearest in square_float_significand_same_prec_gt_2w_lt_3w
    // - rm == Nearest && (round_bit == 0 || sticky_bit == 0 && z_0 & shift_bit == 0) in
    //   square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Nearest,
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
        Less,
    );
    // - a0.wrapping_add(4) & (mask >> 2) > 4 in square_float_significand_same_prec_gt_2w_lt_3w
    // - !decrement_exp in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        Nearest,
        "4.09688648907491713333499964381160220254051958e-58",
        "0x2.9258227caed7c4000630a1192a20211680c74E-48#149",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) in
    //   square_float_significand_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && !overflow
    //   in square_float_significand_same_prec_gt_2w_lt_3w
    test(
        "4.9709672065181108960570410290811793724062284431352e-48",
        "0x7.43dc113e95ca123693650af31435eac45c0e7a680E-40#165",
        Nearest,
        "2.47105149682784709830100902225310999174453232836862e-95",
        "0x3.4c805dfa0982f9705aa6bbd6840b755493beef234E-79#165",
        Greater,
    );
    // - twice_exp - 1 > Float::MAX_EXPONENT
    // - twice_exp - 1 > Float::MAX_EXPONENT && rm == Floor | Down
    test(
        "too_big",
        "0x4.0E+268435455#1",
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    // - twice_exp - 1 > Float::MAX_EXPONENT && rm == Ceiling | Up | Nearest
    test(
        "too_big",
        "0x4.0E+268435455#1",
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - exp > Float::MAX_EXPONENT
    // - exp > Float::MAX_EXPONENT && rm == Floor | Down
    test(
        "too_big",
        "0xc.0E+134217727#2",
        Down,
        "too_big",
        "0x6.0E+268435455#2",
        Less,
    );
    // - exp > Float::MAX_EXPONENT && rm == Ceiling | Up | Nearest
    test(
        "too_big",
        "0xc.0E+134217727#2",
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - twice_exp < Float::MIN_EXPONENT - 1
    // - twice_exp < Float::MIN_EXPONENT - 1 && rm == Floor | Down | Nearest
    test(
        "too_small",
        "0x1.0E-268435456#1",
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    // - twice_exp < Float::MIN_EXPONENT - 1 && rm == Ceiling | Up
    test(
        "too_small",
        "0x1.0E-268435456#1",
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    // - exp < Float::MIN_EXPONENT
    // - exp < Float::MIN_EXPONENT no special
    // - exp < Float::MIN_EXPONENT no special && rm == Floor | Down | Nearest
    test(
        "too_small",
        "0x8.0E-134217729#1",
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    // - exp < Float::MIN_EXPONENT no special && rm == Ceiling | Up
    test(
        "too_small",
        "0x8.0E-134217729#1",
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    // - exp < Float::MIN_EXPONENT special
    test(
        "too_small",
        "0xc.0E-134217729#2",
        Nearest,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
}

#[test]
fn square_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.square_round(Exact));
    assert_panic!(THREE.square_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.square_round_assign(Exact);
    });
}

#[test]
fn test_square_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (square, o) = x.clone().square_prec_round(prec, rm);
        assert!(square.is_valid());

        assert_eq!(square.to_string(), out);
        assert_eq!(to_hex_string(&square), out_hex);
        assert_eq!(o, o_out);

        let (square_alt, o_alt) = x.square_prec_round_ref(prec, rm);
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));
        assert_eq!(o_alt, o_out);

        let mut square_alt = x.clone();
        let o_alt = square_alt.square_prec_round_assign(prec, rm);
        assert!(square_alt.is_valid());
        assert_eq!(ComparableFloatRef(&square), ComparableFloatRef(&square_alt));
        assert_eq!(o_alt, o_out);

        let (square_alt, o_alt) = square_prec_round_naive(x.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_square, rug_o) = rug_square_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_square)),
                ComparableFloatRef(&square),
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

    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Down, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Up, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Exact, "0.0", "0x0.0", Equal);

    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1.0", "0x1.0#1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test("-1.0", "-0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("-1.0", "-0x1.0#1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Exact,
        "1.0",
        "0x1.0#1",
        Equal,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Floor,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Down,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Up,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Exact,
        "1.0",
        "0x1.000#10",
        Equal,
    );

    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Exact,
        "1.0",
        "0x1.0#1",
        Equal,
    );

    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Floor,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Down,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Up,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Exact,
        "1.0",
        "0x1.000#10",
        Equal,
    );

    test("123.0", "0x7b.0#7", 1, Floor, "8.0e3", "0x2.0E+3#1", Less);
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "2.0e4",
        "0x4.0E+3#1",
        Greater,
    );
    test("123.0", "0x7b.0#7", 1, Down, "8.0e3", "0x2.0E+3#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "2.0e4", "0x4.0E+3#1", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "2.0e4",
        "0x4.0E+3#1",
        Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "1.512e4",
        "0x3.b1E+3#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "1.512e4",
        "0x3.b1E+3#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );

    test("-123.0", "-0x7b.0#7", 1, Floor, "8.0e3", "0x2.0E+3#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        1,
        Ceiling,
        "2.0e4",
        "0x4.0E+3#1",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", 1, Down, "8.0e3", "0x2.0E+3#1", Less);
    test("-123.0", "-0x7b.0#7", 1, Up, "2.0e4", "0x4.0E+3#1", Greater);
    test(
        "-123.0",
        "-0x7b.0#7",
        1,
        Nearest,
        "2.0e4",
        "0x4.0E+3#1",
        Greater,
    );

    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Floor,
        "1.512e4",
        "0x3.b1E+3#10",
        Less,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Ceiling,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Down,
        "1.512e4",
        "0x3.b1E+3#10",
        Less,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Up,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Nearest,
        "1.514e4",
        "0x3.b2E+3#10",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Up,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Ceiling,
        "2.004",
        "0x2.01#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Up,
        "2.004",
        "0x2.01#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Up,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Ceiling,
        "2.004",
        "0x2.01#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Up,
        "2.004",
        "0x2.01#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "9.86",
        "0x9.dc#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "9.88",
        "0x9.e0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "9.86",
        "0x9.dc#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "9.88",
        "0x9.e0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "9.88",
        "0x9.e0#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "9.86",
        "0x9.dc#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "9.88",
        "0x9.e0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "9.86",
        "0x9.dc#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "9.88",
        "0x9.e0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "9.88",
        "0x9.e0#10",
        Greater,
    );
}

#[test]
fn square_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).square_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).square_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.square_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.square_prec_round(1, Exact));
    assert_panic!(THREE.square_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.square_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn square_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode, extreme: bool) {
    let (square, o) = x.clone().square_prec_round(prec, rm);
    assert!(square.is_valid());

    let (square_alt, o_alt) = x.clone().square_prec_round_ref(prec, rm);
    assert!(square_alt.is_valid());
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.square_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    if !extreme {
        let (square_alt, o_alt) = square_prec_round_naive(x.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
        assert_eq!(o_alt, o);
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_square, rug_o) = rug_square_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_square)),
            ComparableFloatRef(&square),
        );
        assert_eq!(rug_o, o);
    }

    if !x.is_nan() {
        assert!(square.is_sign_positive());
    }

    if square.is_finite() {
        if square.is_normal() {
            assert_eq!(square.get_prec(), Some(prec));
        }
        if !extreme {
            let r_square = Rational::exact_from(&x).square();
            assert_eq!(square.partial_cmp(&r_square), Some(o));
            if o == Less {
                let mut next = square.clone();
                next.increment();
                assert!(next > r_square);
            } else if o == Greater {
                let mut next = square.clone();
                next.decrement();
                assert!(next < r_square);
            }
            match (r_square >= 0u32, rm) {
                (_, Floor) | (true, Down) | (false, Up) => {
                    assert_ne!(o, Greater);
                }
                (_, Ceiling) | (true, Up) | (false, Down) => {
                    assert_ne!(o, Less);
                }
                (_, Exact) => assert_eq!(o, Equal),
                _ => {}
            }
        }
    }

    let (square_alt, o_alt) = (-&x).square_prec_round(prec, rm);
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.square_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(square.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.square_prec_round_ref(prec, Exact));
    }
}

#[test]
fn square_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_2().test_properties(|(x, prec, rm)| {
        square_prec_round_properties_helper(x, prec, rm, false);
    });

    float_unsigned_rounding_mode_triple_gen_var_11().test_properties(|(x, prec, rm)| {
        square_prec_round_properties_helper(x, prec, rm, true);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (product, o) = Float::NAN.square_prec_round(prec, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.square_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.square_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::ZERO.square_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.square_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ONE.square_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );
        assert_eq!(
            Float::NEGATIVE_ONE.square_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn square_prec_properties_helper(x: Float, prec: u64, extreme: bool) {
    let (square, o) = x.clone().square_prec(prec);
    assert!(square.is_valid());

    let (square_alt, o_alt) = x.square_prec_ref(prec);
    assert!(square_alt.is_valid());
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.square_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    if !extreme {
        let (square_alt, o_alt) = square_prec_round_naive(x.clone(), prec, Nearest);
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
        assert_eq!(o_alt, o);
    }

    let (rug_square, rug_o) = rug_square_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_square)),
        ComparableFloatRef(&square),
    );
    assert_eq!(rug_o, o);

    let (square_alt, o_alt) = x.square_prec_round_ref(prec, Nearest);
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    if !x.is_nan() {
        assert!(square.is_sign_positive());
    }

    if square.is_finite() {
        if square.is_normal() {
            assert_eq!(square.get_prec(), Some(prec));
        }
        if !extreme {
            let r_square = Rational::exact_from(&x).square();
            assert_eq!(square.partial_cmp(&r_square), Some(o));
            if o == Less {
                let mut next = square.clone();
                next.increment();
                assert!(next > r_square);
            } else if o == Greater {
                let mut next = square.clone();
                next.decrement();
                assert!(next < r_square);
            }
        }
    }

    let (square_alt, o_alt) = (-&x).square_prec(prec);
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);
}

#[test]
fn square_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        square_prec_properties_helper(x, prec, false);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        square_prec_properties_helper(x, prec, false);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        square_prec_properties_helper(x, prec, true);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (square, o) = Float::NAN.square_prec(prec);
        assert!(square.is_nan());
        assert_eq!(o, Equal);

        let (square, o) = Float::ZERO.square_prec(prec);
        assert_eq!(ComparableFloat(square), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (square, o) = Float::NEGATIVE_ZERO.square_prec(prec);
        assert_eq!(ComparableFloat(square), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        assert_eq!(Float::INFINITY.square_prec(prec), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.square_prec(prec),
            (Float::INFINITY, Equal)
        );
        assert_eq!(Float::ONE.square_prec(prec), (Float::one_prec(prec), Equal));
        assert_eq!(
            Float::NEGATIVE_ONE.square_prec(prec),
            (Float::one_prec(prec), Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn square_round_properties_helper(x: Float, rm: RoundingMode, extreme: bool) {
    let (square, o) = x.clone().square_round(rm);
    assert!(square.is_valid());

    let (square_alt, o_alt) = x.square_round_ref(rm);
    assert!(square_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));

    let mut x_alt = x.clone();
    let o_alt = x_alt.square_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    if !extreme {
        let (square_alt, o_alt) = square_prec_round_naive(x.clone(), x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
        assert_eq!(o_alt, o);
    }

    let (square_alt, o_alt) = x.square_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    assert_eq!(o_alt, o);

    if !x.is_nan() {
        assert!(square.is_sign_positive());
    }

    if square.is_finite() {
        if x.is_normal() && square.is_normal() {
            assert_eq!(square.get_prec(), Some(x.get_prec().unwrap()));
        }
        if !extreme {
            let r_square = Rational::exact_from(&x).square();
            assert_eq!(square.partial_cmp(&r_square), Some(o));
            if o == Less {
                let mut next = square.clone();
                next.increment();
                assert!(next > r_square);
            } else if o == Greater {
                let mut next = square.clone();
                next.decrement();
                assert!(next < r_square);
            }
            match (r_square >= 0u32, rm) {
                (_, Floor) | (true, Down) | (false, Up) => {
                    assert_ne!(o, Greater);
                }
                (_, Ceiling) | (true, Up) | (false, Down) => {
                    assert_ne!(o, Less);
                }
                (_, Exact) => assert_eq!(o, Equal),
                _ => {}
            }
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_square, rug_o) = rug_square_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_square)),
            ComparableFloatRef(&square),
        );
        assert_eq!(rug_o, o);
    }

    let (square_alt, o_alt) = (-&x).square_round(rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.square_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(square.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.square_round_ref(Exact));
    }
}

#[test]
fn square_round_properties() {
    float_rounding_mode_pair_gen_var_7().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_8().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_9().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_10().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_11().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_12().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_22().test_properties(|(x, rm)| {
        square_round_properties_helper(x, rm, true);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (square, o) = Float::NAN.square_round(rm);
        assert!(square.is_nan());
        assert_eq!(o, Equal);

        let (square, o) = Float::ZERO.square_round(rm);
        assert_eq!(ComparableFloat(square), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (square, o) = Float::NEGATIVE_ZERO.square_round(rm);
        assert_eq!(ComparableFloat(square), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        assert_eq!(Float::INFINITY.square_round(rm), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.square_round(rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(Float::ONE.square_round(rm), (Float::ONE, Equal));
        assert_eq!(Float::NEGATIVE_ONE.square_round(rm), (Float::ONE, Equal));
    });
}

fn square_properties_helper_1(x: Float, extreme: bool) {
    let square = x.clone().square();
    assert!(square.is_valid());

    let square_alt = (&x).square();
    assert!(square_alt.is_valid());
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));

    let mut x_alt = x.clone();
    x_alt.square_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&square));

    if !extreme {
        let square_alt = square_prec_round_naive(x.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    }

    let square_alt = x.square_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));
    let square_alt = x.square_prec_ref(x.significant_bits()).0;
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));

    let square_alt = x.square_round_ref(Nearest).0;
    assert_eq!(ComparableFloatRef(&square_alt), ComparableFloatRef(&square));

    if !x.is_nan() {
        assert!(square.is_sign_positive());
    }

    if square.is_finite() && x.is_normal() && square.is_normal() {
        assert_eq!(square.get_prec(), Some(x.get_prec().unwrap()));
        if !extreme {
            let r_square = Rational::exact_from(&x).square();
            if square < r_square {
                let mut next = square.clone();
                next.increment();
                assert!(next > r_square);
            } else if square > r_square {
                let mut next = square.clone();
                next.decrement();
                assert!(next < r_square);
            }
        }
    }

    let rug_square = rug_square(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_square)),
        ComparableFloatRef(&square),
    );

    assert_eq!(ComparableFloat((-x).square()), ComparableFloat(square));
}

#[allow(clippy::type_repetition_in_bounds)]
fn square_properties_helper_2<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let square_1 = x.square();
        let square_2 = emulate_primitive_float_fn(|x, prec| x.square_prec(prec).0, x);
        assert_eq!(NiceFloat(square_1), NiceFloat(square_2));
    });
}

#[test]
fn square_properties() {
    float_gen().test_properties(|x| {
        square_properties_helper_1(x, false);
    });

    float_gen_var_6().test_properties(|x| {
        square_properties_helper_1(x, false);
    });

    float_gen_var_7().test_properties(|x| {
        square_properties_helper_1(x, false);
    });

    float_gen_var_8().test_properties(|x| {
        square_properties_helper_1(x, false);
    });

    float_gen_var_9().test_properties(|x| {
        square_properties_helper_1(x, false);
    });

    float_gen_var_10().test_properties(|x| {
        square_properties_helper_1(x, false);
    });

    float_gen_var_12().test_properties(|x| {
        square_properties_helper_1(x, true);
    });

    apply_fn_to_primitive_floats!(square_properties_helper_2);
}
