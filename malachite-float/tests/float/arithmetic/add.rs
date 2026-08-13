// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{NegAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::emulate_float_float_to_float_fn;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::add::{
    add_prec_round_naive, add_rational_prec_round_naive, rug_add, rug_add_prec, rug_add_prec_round,
    rug_add_rational, rug_add_rational_prec, rug_add_rational_prec_round, rug_add_rational_round,
    rug_add_round,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_1, float_float_rounding_mode_triple_gen_var_4,
    float_float_rounding_mode_triple_gen_var_5, float_float_rounding_mode_triple_gen_var_6,
    float_float_rounding_mode_triple_gen_var_7, float_float_rounding_mode_triple_gen_var_8,
    float_float_rounding_mode_triple_gen_var_9, float_float_rounding_mode_triple_gen_var_29,
    float_float_unsigned_rounding_mode_quadruple_gen_var_1,
    float_float_unsigned_rounding_mode_quadruple_gen_var_5, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_2, float_gen, float_pair_gen, float_pair_gen_var_2,
    float_pair_gen_var_3, float_pair_gen_var_4, float_pair_gen_var_5, float_pair_gen_var_6,
    float_pair_gen_var_7, float_pair_gen_var_10, float_rational_pair_gen,
    float_rational_pair_gen_var_2, float_rational_rounding_mode_triple_gen_var_1,
    float_rational_rounding_mode_triple_gen_var_8,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_7,
    float_rational_unsigned_triple_gen_var_1, float_rational_unsigned_triple_gen_var_2,
    float_rounding_mode_pair_gen, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_1, rational_rounding_mode_pair_gen_var_6,
    rational_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::cmp::{
    Ordering::{self, *},
    max,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let sum = x.clone() + y.clone();
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let sum_alt = x.clone() + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        let mut sum_alt = x.clone();
        sum_alt += y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let mut sum_alt = x.clone();
        sum_alt += &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_add(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&sum)
        );

        let sum_alt = add_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    };
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "NaN", "NaN");
    test("NaN", "NaN", "-Infinity", "-Infinity", "NaN", "NaN");
    test("NaN", "NaN", "0.0", "0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "-0.0", "-0x0.0", "NaN", "NaN");

    test("Infinity", "Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", "Infinity", "Infinity",
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", "Infinity", "Infinity",
    );

    test("-Infinity", "-Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
    );

    test("0.0", "0x0.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");

    test("-0.0", "-0x0.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", "NaN", "NaN", "NaN", "NaN");
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("123.0", "0x7b.0#7", "0.0", "0x0.0", "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", "-0.0", "-0x0.0", "123.0", "0x7b.0#7");

    test("NaN", "NaN", "123.0", "0x7b.0#7", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", "Infinity", "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "123.0", "0x7b.0#7", "123.0", "0x7b.0#7");
    test("-0.0", "-0x0.0", "123.0", "0x7b.0#7", "123.0", "0x7b.0#7");

    // - in add_float_significands_same_prec_lt_w
    // - x_exp < y_exp in add_float_significands_same_prec_lt_w
    // - exp_diff < shift in add_float_significands_same_prec_lt_w
    // - exp_diff < shift && !overflow in add_float_significands_same_prec_lt_w
    // - (round_bit != 0 || sticky_bit == 0) && rm == Nearest in
    //   add_float_significands_same_prec_lt_w
    // - round_bit != 0 && (sticky_bit != 0 || (sum & shift_bit) != 0) in
    //   add_float_significands_same_prec_lt_w
    // - rm == Nearest && sum == 0 in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "4.0", "0x4.0#1");
    // - in add_float_significands_same_prec_general
    // - out_bits > exp_diff in add_float_significands_same_prec_general
    // - overlap <= ys_len in add_float_significands_same_prec_general
    // - shift2 != 0 in add_float_significands_same_prec_general
    // - out_len - k <= overlap in add_float_significands_same_prec_general
    // - out_len <= xs_len second time in add_float_significands_same_prec_general
    // - !y in add_float_significands_same_prec_general
    // - round_bit == Uninitialized && shift != 0 in add_float_significands_same_prec_general
    // - shift > 1 in add_float_significands_same_prec_general
    // - x == 0 second time in add_float_significands_same_prec_general
    // - xs_len <= out_len && following_bits != True in add_float_significands_same_prec_general
    // - difw > 0 && difw > ys_len && exp_diff <= out_bits in
    //   add_float_significands_same_prec_general
    // - exp_diff_rem != 0 || yi != 0 second time in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 second time in add_float_significands_same_prec_general
    // - round_bit != Uninitialized seventh time in add_float_significands_same_prec_general
    // - yi == 0 second time in add_float_significands_same_prec_general
    // - in add_float_significands_same_prec_general_round
    // - following_bits == False && round_bit == False in
    //   add_float_significands_same_prec_general_round
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "3.0", "0x3.0#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#1", "3.0", "0x3.0#2");
    // round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#2", "2.0", "0x2.0#2", "3.0", "0x3.0#2");
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        "3.0000",
        "0x3.00#10",
    );

    // - exp_diff < shift && overflow in add_float_significands_same_prec_lt_w
    // - round_bit == 0 || (sticky_bit == 0 && (sum & shift_bit) == 0)
    // - in add_float_significands_same_prec_lt_w
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "4.5558062159628880",
        "0x4.8e4950f0795fc#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-4.5558062159628880",
        "-0x4.8e4950f0795fc#53",
    );

    // - x_exp > y_exp in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "0.00024", "0x0.001#1", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "-1.0", "-0x1.0#1", "0.0", "0x0.0");
    // - x_exp == y_exp in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", "2.0", "0x2.0#1");
    // - rm == Nearest && sum != 0 in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#3", "1.8", "0x1.c#3", "3.0", "0x3.0#3");
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_lt_w
    test(
        "7.697e14",
        "0x2.bcE+12#8",
        "1.237e-9",
        "0x5.50E-8#8",
        "7.697e14",
        "0x2.bcE+12#8",
    );
    // - shift <= exp_diff < Limb::WIDTH in add_float_significands_same_prec_lt_w
    // - shift <= exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_lt_w
    test(
        "1.852193494e22",
        "0x3.ec137baE+18#29",
        "241425944.0",
        "0xe63de18.0#29",
        "1.852193494e22",
        "0x3.ec137baE+18#29",
    );
    // - shift <= exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_lt_w
    test(
        "1.999999999999993",
        "0x1.fffffffffffe#48",
        "5.960464477539062e-8",
        "0x1.000000000000E-6#48",
        "2.000000059604645",
        "0x2.000001000000#48",
    );

    // - in add_float_significands_same_prec_w
    // - x_exp == y_exp in add_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_w
    // - !overflow in add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        "3.00000000000000000000",
        "0x3.0000000000000000#64",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_w
    test(
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "3.00000000000000000000",
        "0x3.0000000000000000#64",
    );
    // - (round_bit != 0) || (sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_w
    // - round_bit == 0 || (sticky_bit == 0 && (sum & 1) == 0) in add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) in add_float_significands_same_prec_w
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and !overflow in
    //   add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000033",
        "0x1.0000000000000006#64",
        "2.00000000000000000043",
        "0x2.0000000000000008#64",
    );
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_w
    test(
        "5.93763496769044317938e-6",
        "0x0.0000639df2b03f3e49a70#64",
        "2.93472512905146303523e-45",
        "0x1.0c11b075f03d6daeE-37#64",
        "5.93763496769044317938e-6",
        "0x0.0000639df2b03f3e49a70#64",
    );
    // - overflow in add_float_significands_same_prec_w
    test(
        "0.000221852535829092939590",
        "0x0.000e8a1162cbb1a4265#64",
        "0.0000297456615217170340009",
        "0x0.0001f30ca4b8117ff0a0#64",
        "0.000251598197350809973604",
        "0x0.00107d1e0783c324170#64",
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and overflow in
    //   add_float_significands_same_prec_w
    test(
        "63.9999999999999999965",
        "0x3f.ffffffffffffffc#64",
        "64.0000000000000000000",
        "0x40.000000000000000#64",
        "128.000000000000000000",
        "0x80.00000000000000#64",
    );

    // - in add_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        "3.00000000000000000000",
        "0x3.0000000000000000#65",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "3.00000000000000000000",
        "0x3.0000000000000000#65",
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (sum_0 != 0 || sum_1 != 0) in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
    );
    // - rm == Nearest && sum_1 != 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        "2.00000000000000000022",
        "0x2.0000000000000004#65",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "18.6312303425262679475864699",
        "0x12.a1984fcd64a8ae228eef#85",
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
    );
    // - exp_diff >= Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.85458229226496712261e27",
        "0xf.af9dc963a0709f78E+22#65",
        "1.14823551075108882469e-96",
        "0x2.73dea72af3fe6314E-80#65",
        "4.85458229226496712261e27",
        "0xf.af9dc963a0709f78E+22#65",
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "19585.2851423168986928116147584507795",
        "0x4c81.48ff163dc91a0d4bd90309b0f8#116",
        "372369974082165972902790.766638151683",
        "0x4eda377c7f0d747fa386.c44265dd58#116",
        "372369974082165972922376.051780468581",
        "0x4eda377c7f0d747ff008.0d417c1b20#116",
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "18.4926492161804358300005290",
        "0x12.7e1e424fe51f1bb914c0#85",
        "56.6375897899064714038448469",
        "0x38.a339159fe96c1722fdfe#85",
        "75.1302390060869072338453776",
        "0x4b.215757efce8b32dc12c0#85",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "5.293955922766053551082318577017522e-23",
        "0x4.00000007e000fffffff0000000E-19#107",
        "255.9999999999999999999999470604408",
        "0xff.ffffffffffffffffffc000000#107",
        "256.0000000000000000000000000000000",
        "0x100.0000000000000000000000000#107",
    );
    // - rm == Nearest && sum_1 == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.00006103515624999999999959",
        "0x0.0003ffffffffffffffff8#67",
        "17179869183.99993896484",
        "0x3ffffffff.fffc00000#67",
        "17179869184.00000000000",
        "0x400000000.00000000#67",
    );

    // - in add_float_significands_same_prec_2w
    // - x_exp == y_exp in add_float_significands_same_prec_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH && !overflow in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#128",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_2w
    test(
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#128",
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_2w
    // - rm == Nearest in add_float_significands_same_prec_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (sum_0 & 1) == 0)) in
    //   add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (sum_0 & 1) != 0) in
    //   add_float_significands_same_prec_2w
    // - rm == Nearest && !overflow in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000018",
        "0x1.00000000000000000000000000000006#128",
        "2.000000000000000000000000000000000000024",
        "0x2.00000000000000000000000000000008#128",
    );
    // - exp_diff >= TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff > TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "5.809911490453824289488892996394197332617e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        "5.078012491369571452708077262055118554215e-45",
        "0x1.cfd8608b7c32de2bbcfecf8bcf8a2d00E-37#128",
        "5.809911490453824289488892996394197332617e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
    );
    // - Limb::WIDTH <= exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "4354249796990942.354353575265977831431638",
        "0xf782ac869b7de.5ab6ea78fcf0cc5079f#128",
        "8.032394538257265122403070534052560160215e-10",
        "0x3.732bce7aa121827a284545a25f32dc68E-8#128",
        "4354249796990942.354353576069217285257364",
        "0xf782ac869b7de.5ab6ea7c701c9acb1b1#128",
    );
    // - exp_diff == TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "15732412727332569995335732833027757624.44",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.7#128",
        "0.03737083028200858887607458416398961289207",
        "0x0.0991227de2b63edc67164401ce8ebdb04#128",
        "15732412727332569995335732833027757624.50",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.8#128",
    );
    // - Limb::WIDTH == exp_diff in add_float_significands_same_prec_2w
    test(
        "1.057437459917463716438672572710788562701e-17",
        "0xc.310127aae1df1a1cb12f60c4d339d76E-15#128",
        "148.0549133677002965445211858794413066474",
        "0x94.0e0ecd6e62d0a8c7c7c2a633277e3e#128",
        "148.0549133677002965550955604786159438120",
        "0x94.0e0ecd6e62d0a98ad7d520e1456fe0#128",
    );
    // - exp_diff < TWICE_WIDTH && overflow in add_float_significands_same_prec_2w
    test(
        "990.8902848544842589812043163049608986645",
        "0x3de.e3e9b54e224e900a8701c94cea27bc#128",
        "111.9722425438858761689147540845231217720",
        "0x6f.f8e4e329c509f04b7f9497ec8ce6438#128",
        "1102.862527398370135150119070389484020437",
        "0x44e.dcce9877e758805606966139770e00#128",
    );
    // - rm == Nearest && overflow in add_float_significands_same_prec_2w
    test(
        "1152920954851033088.000000000000000000000",
        "0xfffff8000000000.00000000000000000#128",
        "549755813887.9999999999999999999999999984",
        "0x7fffffffff.ffffffffffffffffffffff8#128",
        "1152921504606846976.000000000000000000000",
        "0x1000000000000000.00000000000000000#128",
    );

    // - in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp > y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#129",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#129",
    );
    // - x_exp == y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
    );
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#129",
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 < exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
    );
    // - exp_diff >= Limb::WIDTH * 3 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "439761088891971104563481495859833667777753437696.0",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        "6.88923601593624215957288189353784878326857540585e-50",
        "0x1.9c693c182df3035eef00d41638bbdd942f4d498E-41#155",
        "439761088891971104563481495859833667777753437696.0",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.28505372386063746523518779888117963738987726e-22",
        "0x2.0607fd4819748c532aad3528693c1e3c1966E-18#146",
        "978.493288099344953918398808019894399812365694",
        "0x3d2.7e4820fe314caadcb9a156bef2f1c8e53c#146",
        "978.493288099344953918399236525266785876112215",
        "0x3d2.7e4820fe314caadcbba75ebc3b0b3d718f#146",
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4499918.462192628409483090774709614007863365",
        "0x44a9ce.7652418f789422bc22220831e2030#137",
        "64560208.02626195160236877593517814393478863",
        "0x3d91c50.06b91a6f42e5205070f82f89eefa#137",
        "69060126.48845458001185186670988775794265180",
        "0x41dc61e.7d0b5bfebb79430c931a37bbd0fc#137",
    );
    // - exp_diff == Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "563971925627753843356041629019151473018178607215.422",
        "0x62c960337e963a378ba6626ea422d8a5e623986f.6c#165",
        "1301375421.83361702516620516356439489325145225661938",
        "0x4d9169bd.d567ece47a47ef60371d48c969ba8765d4#165",
        "563971925627753843356041629019151473019479982637.250",
        "0x62c960337e963a378ba6626ea422d8a633b5022d.40#165",
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "226.93050907532437979947076285686054061939",
        "0xe2.ee35d7bf263fda8c632644ad7c49d98#130",
        "4262448175090788889452.9841882569848613913",
        "0xe71159efd3a67e736c.fbf3c2f8db72fb8#130",
        "4262448175090788889679.9146973323092411901",
        "0xe71159efd3a67e744f.ea299ab801b2d60#130",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1180591620717411303423.999999999999999999999999997",
        "0x3fffffffffffffffff.ffffffffffffffffffffff#158",
        "5.684341886163519548224296320364965458063242301206e-14",
        "0x1.000000000fffffffffffe0000000000000000038E-11#158",
        "1180591620717411303424.000000000000056843418861629",
        "0x400000000000000000.00000000001000000000fe#158",
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum == 0
    test(
        "4503599627370495.99999999999999999999999999960",
        "0xfffffffffffff.ffffffffffffffffffffffe#143",
        "3.30872245098247973850465205378347282876506684e-24",
        "0x4.00000003ffff0000000000000000fffffffE-20#143",
        "4503599627370496.00000000000000000000000330872",
        "0x10000000000000.00000000000000000004000#143",
    );

    // - in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && round_bit == 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec in add_float_significands_same_prec_ge_3w
    // - in add_significands_rsh_to_out
    // - exp_diff < Limb::WIDTH in add_significands_rsh_to_out
    // - 0 < exp_diff < prec && shift == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && limb == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_ge_3w
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - 0 < exp_diff < prec && shift != 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.00000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#193",
        "2.00000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#193",
        "3.00000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#193",
    );
    // - exp_diff == 0 && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && sticky_bit == 0 && out[0] &
    //   shift_bit == 0
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000010",
        "0x1.000000000000000000000000000000000000000000000006#192",
        "2.0000000000000000000000000000000000000000000000000000000013",
        "0x2.000000000000000000000000000000000000000000000008#192",
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && !carry in add_float_significands_same_prec_ge_3w
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000010",
        "0x1.000000000000000000000000000000000000000000000006#192",
        "3.0000000000000000000000000000000000000000000000000000000013",
        "0x3.000000000000000000000000000000000000000000000008#192",
    );
    // - exp_diff >= Limb::WIDTH in add_significands_rsh_to_out
    // - exp_diff >= Limb::WIDTH && r != 0 in add_significands_rsh_to_out
    test(
        "7.280571169383842274329034483677671964286795147653983789731014e-48",
        "0xa.a3fc2da1f20fb2d9771f86d3c16a444cd62d5d139e3935f24E-40#198",
        "3.512347377882557895896869518765758776035713939594826958897103e-27",
        "0x1.1646de419a6dbd3466f3081403a87d719b7a765a1ec69e4658E-22#198",
        "3.512347377882557895904150089935142618310042974078504630861396e-27",
        "0x1.1646de419a6dbd3471970441a59a8d2474f195e0f288088aa8E-22#198",
    );
    // - exp_diff >= prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Nearest || rm == Floor
    //   || rm == Down)
    test(
        "4.13222828802191621569015595751616491736159555180726072912070e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        "0.0239913867670311930420667487107083515019528907529246130057244",
        "0x0.06244cad8cd272134e34b325815ad281733f2c06231a0ee744#193",
        "4.13222828802191621569015595751616491736159555180726072912070e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
    );
    // - 0 < exp_diff < prec && limb != 0 in add_float_significands_same_prec_ge_3w
    test(
        "8.69977204237437814069372807483827970856267379941609710779639",
        "0x8.b32442b4a730454d66b1b2bdf7a2863d417e6ff22d7f6c58#193",
        "7.58974636819623954377405988444623535636829063921159080831479",
        "0x7.96f99e34566e7be1960d023e431dc5e0a7ad24ad691a1ac4#193",
        "16.2895184105706176844677879592845150649309644386276879161118",
        "0x10.4a1de0e8fd9ec12efcbeb4fc3ac04c1de92b949f9699872#193",
    );
    // - exp_diff >= Limb::WIDTH && r == 0 in add_significands_rsh_to_out
    test(
        "6.4425523507465541098853496915929918929896246856311922355487e-6",
        "0x0.00006c168d38e231899f0fc85d1888549d5177bdceaee72e15060#192",
        "1476808010161862576835936576709144.7975622615653024045505082",
        "0x48cff00a780a50d34bb694ada218.cc2d0a55f25f9f9126258#192",
        "1476808010161862576835936576709144.7975687041176531511046180",
        "0x48cff00a780a50d34bb694ada218.cc2d766c7f9881c2afc48#192",
    );
    // - exp_diff == prec in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && power && !carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power in add_float_significands_same_prec_ge_3w
    test(
        "4.06358384024552072294006982076688939253797681513643139422220e-23",
        "0x3.1202ecf10ff40b477337957dede18bd7b746884ec977474eE-19#194",
        "1174582238252884689829665592721065057.76655867827770290150723",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d4#194",
        "1174582238252884689829665592721065057.76655867827770290150729",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d8#194",
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && carry in add_float_significands_same_prec_ge_3w
    test(
        "4.3368086899420177360298112034797668456999388161777350954459e-19",
        "0x7.fffffffffffffffffffffffffffffffe0000000000000000E-16#192",
        "5192296858534827628530496329220095.9999999999999999995663191",
        "0xffffffffffffffffffffffffffff.fffffffffffffff80000#192",
        "5192296858534827628530496329220096.0000000000000000000000000",
        "0x10000000000000000000000000000.00000000000000000000#192",
    );
    // - exp_diff == prec && rm == Nearest && power && carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power && carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "158456325028528675187087900671.99999999999999999999999999997",
        "0x1ffffffffffffffffffffffff.fffffffffffffffffffffffe#192",
        "2.5243548967072377773175314089049159349542605923488736152645e-29",
        "0x1.fffffffffffffffffffffffffffffffffffffffffffffffeE-24#192",
        "158456325028528675187087900672.00000000000000000000000000000",
        "0x2000000000000000000000000.000000000000000000000000#192",
    );
    // - exp_diff == prec && rm == Nearest && power in add_float_significands_same_prec_ge_3w
    test(
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
        "2.6469779601696885595885078146238811314105987548828125000000e-23",
        "0x2.000000000000000000000000000000000000000000000000E-19#192",
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
    );

    // - shift2 == 0 in add_float_significands_same_prec_general
    // - y in add_float_significands_same_prec_general
    // - shift != 0 in add_float_significands_same_prec_general
    // - x == 0 first time in add_float_significands_same_prec_general
    // - shift == 0 || following_bits != Uninitialized in add_float_significands_same_prec_general
    // - round_bit != Uninitialized || shift == 0 in add_float_significands_same_prec_general
    // - exp_diff_rem == 0 && yi == 0 second time in add_float_significands_same_prec_general
    // - round_bit != Uninitialized sixth time in add_float_significands_same_prec_general
    test("1.0", "0x1.0#1", "1.0", "0x1.0#2", "2.0", "0x2.0#2");
    // - following_bits != False || round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_general_round
    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", "2.0", "0x2.0#2");
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && carry in
    //   add_float_significands_same_prec_general_round
    test("2.0", "0x2.0#1", "1.5", "0x1.8#2", "4.0", "0x4.0#2");
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_general_round
    test("1.0", "0x1.0#1", "1.8", "0x1.c#3", "3.0", "0x3.0#3");
    // - x != 0 && x != mask second time in add_float_significands_same_prec_general
    // - rm == Nearest && following_bits != False && round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits != False && round_bit != False && !carry in
    //   add_float_significands_same_prec_general_round
    test("1.5", "0x1.8#2", "4.0", "0x4.0#1", "6.0", "0x6.0#2");
    // - rm == Nearest && following_bits != False && round_bit == False in
    //   add_float_significands_same_prec_general_round
    test("4.0", "0x4.0#1", "1.2", "0x1.4#3", "5.0", "0x5.0#3");
    // - x != 0 && x != mask first time in add_float_significands_same_prec_general
    // - shift != 0 && following_bits == Uninitialized in add_float_significands_same_prec_general
    test("1.2", "0x1.4#3", "3.0", "0x3.0#2", "4.0", "0x4.0#3");
    // - rm == Nearest && following_bits != False && round_bit != False && carry in
    //   add_float_significands_same_prec_general_round
    test("1.8", "0x1.c#3", "6.0", "0x6.0#2", "8.0", "0x8.0#3");
    // - out_bits <= exp_diff in add_float_significands_same_prec_general
    // - out_len <= xs_len first time in add_float_significands_same_prec_general
    // - difw > 0 && difw > ys_len && exp_diff > out_bits in
    //   add_float_significands_same_prec_general
    // - round_bit != Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "8.821880e11",
        "0xc.d668E+9#18",
        "9.7545998337406e122",
        "0x1.79c17f063aE+102#40",
        "9.7545998337406e122",
        "0x1.79c17f063aE+102#40",
    );
    // - out_len > xs_len first time in add_float_significands_same_prec_general
    test(
        "2.8577648979177105962332201291018926848163080599637e-19",
        "0x5.458a93bffa7b1c05bdd1c0552b60196746d9083cE-16#162",
        "3.5697206995078675e50",
        "0xf.4400d3acf388E+41#51",
        "3.5697206995078675404584127554321345196383736430592e50",
        "0xf.4400d3acf3880000000000000000000000000000E+41#162",
    );
    // - overlap > ys_len in add_float_significands_same_prec_general
    // - out_len - k > overlap in add_float_significands_same_prec_general
    // - difw <= 0 || difw <= ys_len in add_float_significands_same_prec_general
    // - round_bit != Uninitialized fourth time in add_float_significands_same_prec_general
    test(
        "29780282551.7626844589368663631653",
        "0x6ef0b0cb7.c33f49e84d21bb6040#104",
        "0.0000394559894753800",
        "0x0.000295f62f36adb#46",
        "29780282551.7627239149263417431406",
        "0x6ef0b0cb7.c341dfde7c58691040#104",
    );
    // - out_len > xs_len second time in add_float_significands_same_prec_general
    test(
        "1.071839725139585312577139389278151e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        "0.003742228294246889823114823919652848",
        "0x0.00f5402c235119eafa12eb9821602#107",
    );
    // - exp_diff_rem == 0 second time in add_float_significands_same_prec_general
    test(
        "5.191920952034e-15",
        "0x1.761e097c5E-12#37",
        "7.37e4",
        "0x1.2E+4#5",
        "73728.00000000",
        "0x12000.00000#37",
    );
    // - shift <= 1 in add_float_significands_same_prec_general
    test(
        "15135.895602865542606017656527713819177465060416097749360065",
        "0x3b1f.e5463ab9b599ce49b83c7988b324dc93ce50b2ed51a18#191",
        "3.5815296244999700478867322252421807366487e-8",
        "0x9.9d355ad2b99a587727da095fa3226bf0E-7#130",
        "15135.895602901357902262656228192686499717482223464235884113",
        "0x3b1f.e5463b5388ef7b7551e200fb30c5728e007771ed51a18#191",
    );
    // - round_bit == Uninitialized fourth time in add_float_significands_same_prec_general
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "8.636437350163444678191748627983325934618e-6",
        "0x0.000090e5374a001358c6606f968bf3813ad9#128",
        "1.849037e-8",
        "0x4.f6a6E-7#18",
        "8.654927718511001474116650590263599372118e-6",
        "0x0.00009134a1aa001358c6606f968bf3813ad9#128",
    );
    // - round_bit == Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "2.389545997e25",
        "0x1.3c40f7bE+21#29",
        "0.0782632598242840004025",
        "0x0.14090f9d6c745bc06#64",
        "2.38954599675655770897e25",
        "0x1.3c40f7b000000000E+21#64",
    );
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "5.75055158778420135770e-7",
        "0x9.a5d7d56cabed47dE-6#64",
        "1.1758894e-14",
        "0x3.4f515E-12#22",
        "5.75055170537314368446e-7",
        "0x9.a5d7d8bbfd3d47dE-6#64",
    );
    // - x != 0 && x == mask second time in add_float_significands_same_prec_general
    // - xs_len <= out_len && following_bits == True in add_float_significands_same_prec_general
    test(
        "1.081090215247020702e-18",
        "0x1.3f14ddfe22c0634E-15#59",
        "6.379965759614693e-8",
        "0x1.12047722d26cE-6#47",
        "6.379965759722801557e-8",
        "0x1.12047722e65d4dcE-6#59",
    );
    // - shift == 0 in add_float_significands_same_prec_general
    test(
        "4.3055451539258443718732375731462554408177909736937057433067e-16",
        "0x1.f06543668e6018c20c17efed72ff6d3d65a4c5dc9db475b0E-13#192",
        "1.6388436e-15",
        "0x7.61754E-13#21",
        "2.0693980969049410047184094732421002104686218250305033868307e-15",
        "0x9.51da83668e6018c20c17efed72ff6d3d65a4c5dc9db475bE-13#192",
    );
    // - yi != 0 second time in add_float_significands_same_prec_general
    test(
        "2.241814356765462e-16",
        "0x1.0276ae5de1e8E-13#47",
        "7.643070003987853963842537238646240439305e-36",
        "0xa.28cd4cb186f5925ddb0d1ecb9681103E-30#128",
        "2.241814356765462069110833332462974460290e-16",
        "0x1.0276ae5de1e80000a28cd4cb186f5926E-13#128",
    );
    // - x != 0 && x == mask first time in add_float_significands_same_prec_general
    test(
        "2.14747955e9",
        "0x7.ffff00E+7#24",
        "8191.99987888336091",
        "0x1fff.fff80fffff0#54",
        "2147487743.99987888",
        "0x80000fff.fff810#54",
    );

    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "Infinity",
        "Infinity",
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0",
        "0x1.0#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "4.8e-323228497",
        "0x2.0E-268435456#1",
    );
    test(
        "3.6e-323228497",
        "0x1.8E-268435456#2",
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        "0.0",
        "0x0.0",
    );
    test(
        "3.0e-323228497",
        "0x1.4E-268435456#3",
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        "0.0",
        "0x0.0",
    );
    test(
        "1.262113616894785778907e-31",
        "0x2.8f53ace354a6b7284E-26#68",
        "-1.263e-31",
        "-0x2.90E-26#7",
        "-1.296426623240940697535e-34",
        "-0xa.c531cab5948d7c00E-29#68",
    );
    test(
        "1.56132e-323228494",
        "0x2.8f5E-268435454#14",
        "-1.56296e-323228494",
        "-0x2.900E-268435454#14",
        "-2.38256e-323228497",
        "-0x1.0000E-268435456#14",
    );
    test(
        "-1.56132e-323228494",
        "-0x2.8f5E-268435454#14",
        "1.56296e-323228494",
        "0x2.900E-268435454#14",
        "2.38256e-323228497",
        "0x1.0000E-268435456#14",
    );

    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "-1.0",
        "-0x1.0#1",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
    );
}

#[test]
fn test_add_prec() {
    let test = |s, s_hex, t, t_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (sum, o) = x.clone().add_prec(y.clone(), prec);
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.clone().add_prec_val_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_ref_val(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_ref_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_assign(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_assign_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = add_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (rug_sum, rug_o) = rug_add_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", "Infinity", "Infinity", 1, "NaN", "NaN", Equal);
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test("NaN", "NaN", "0.0", "0x0.0", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal);

    test("Infinity", "Infinity", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "-0.0", "-0x0.0", 1, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal,
    );

    test("123.0", "0x7b.0#7", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test("NaN", "NaN", "123.0", "0x7b.0#7", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 2, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 2, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 2, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 2, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        1,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        2,
        "3.0",
        "0x3.0#2",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "4.5547",
        "0x4.8e#10",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-1.7266",
        "-0x1.ba0#10",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "1.7266",
        "0x1.ba0#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-4.5547",
        "-0x4.8e#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, "0.0", "0x0.0", Equal,
    );

    // - xs_len > out_len in add_float_significands_same_prec_general
    // - following_bits != False || difw > 0 in add_float_significands_same_prec_general
    // - difw <= ys_len in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 || yi != 0 first time in add_float_significands_same_prec_general
    // - yi >= 0 && yi == ys_len in add_float_significands_same_prec_general
    // - following_bits != Uninitialized first time in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 first time in add_float_significands_same_prec_general
    // - yi >= 0 second time in add_float_significands_same_prec_general
    // - yi >= 0 third time in add_float_significands_same_prec_general
    // - x >= y in add_float_significands_same_prec_general
    // - following_bits != False || x == 0 in add_float_significands_same_prec_general
    // - following_bits != False && x != Limb::MAX second time in
    //   add_float_significands_same_prec_general
    test(
        "1.734147472944056e-17",
        "0x1.3fe4cc8cf520E-14#48",
        "5095194424.16793745804039398847854893",
        "0x12fb27f38.2afdf3020e8eaac84a7ec#116",
        62,
        "5095194424.1679374576",
        "0x12fb27f38.2afdf300#62",
        Less,
    );
    // - yi < 0 || yi != ys_len in add_float_significands_same_prec_general
    // - yi < 0 third time in add_float_significands_same_prec_general
    test(
        "102490490.309858195",
        "0x61be17a.4f52dde#54",
        "140024642456267714260687682888.7395345704093208802906032076327334",
        "0x1c471ab65a12bae8410f87d48.bd52233a2b6095a9fa45bd79bbfc#209",
        126,
        "140024642456267714260790173379.049392765",
        "0x1c471ab65a12bae8417145ec3.0ca50118#126",
        Less,
    );
    // - following_bits != 0 && following_bits != Limb::MAX in
    //   add_float_significands_same_prec_general
    // - x != Limb::MAX first time in add_float_significands_same_prec_general
    test(
        "3.65870e-6",
        "0x0.00003d62#14",
        "2.26998937985590408943624715555e46",
        "0x3.f9e6004fa97f534f98cad274E+38#96",
        61,
        "2.2699893798559040890e46",
        "0x3.f9e6004fa97f534E+38#61",
        Less,
    );
    // - following_bits == Uninitialized first time in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 in add_float_significands_same_prec_general
    // - yi < 0 first time in add_float_significands_same_prec_general
    // - x >= y or not other conditions in add_float_significands_same_prec_general
    // - round_bit == Limb::MAX second time in add_float_significands_same_prec_general
    // - following_bits != False && x != Limb::MAX first time in
    //   add_float_significands_same_prec_general
    test(
        "6058.052082725914153064469688823596059469551684564539",
        "0x17aa.0d554b247ce1b6ab28ba39c8d5992a74c7ac91a#169",
        "0.0001445668922083233",
        "0x0.0009796e12f9784#47",
        64,
        "6058.05222729280636118",
        "0x17aa.0d5ec4928fdb2#64",
        Less,
    );
    // - following_bits == True in add_float_significands_same_prec_general
    // - round_bit == Uninitialized first time in add_float_significands_same_prec_general
    // - x != Limb::MAX second time in add_float_significands_same_prec_general
    test(
        "3.65965173691106590893554423956548915846e48",
        "0x2.810875a0ca3206afd8c6cf841941830E+40#123",
        "1545.699550397407201099813420288295",
        "0x609.b315bc1ec48a143a74bd53048#109",
        64,
        "3.65965173691106590895e48",
        "0x2.810875a0ca3206b0E+40#64",
        Greater,
    );
    // - yi >= 0 first time in add_float_significands_same_prec_general
    test(
        "2.809154296046691025933830524368088854018547244107388290e-11",
        "0x1.ee310ffa09a06a6361f52c2cd8a9569a780b775dc213E-9#177",
        "519241072363118296470.333928838103121952666621563036",
        "0x1c25eadc41d4907d96.557c5c3ed81cab65dab0cf920#166",
        64,
        "519241072363118296480.0",
        "0x1.c25eadc41d4907daE+17#64",
        Greater,
    );
    // - round_bit != Uninitialized second time in add_float_significands_same_prec_general
    test(
        "559935046210054011882951826578284118061013900.58534479",
        "0x191bbd3588c78488c2f4d122814d5fb34edb8c.95d928#170",
        "3.0279323e11",
        "0x4.67fe2E+9#22",
        63,
        "5.5993504621005401188e44",
        "0x1.91bbd3588c78488cE+37#63",
        Less,
    );
    // - following_bits == False && difw <= 0 in add_float_significands_same_prec_general
    test(
        "7184368264698708563285024670194469326968686224.863863495065914",
        "0x1422880c600dc4fd90a02f1814859aafd658690.dd2628738430#198",
        "1.02960603282018e-24",
        "0x1.3ea5cb49bdaE-20#44",
        61,
        "7.1843682646987085654e45",
        "0x1.422880c600dc4feE+38#61",
        Greater,
    );
    // - following_bits == False && x != 0 in add_float_significands_same_prec_general
    test(
        "19710666.8219848980599857068492",
        "0x12cc2ca.d26d9a2ef9396c5108#94",
        "6.6e4",
        "0x1.0E+4#2",
        61,
        "19776202.821984898066",
        "0x12dc2ca.d26d9a2f0#61",
        Greater,
    );
    // - round_bit != Uninitialized first time in add_float_significands_same_prec_general
    test(
        "2.3370796820961060045359802932823709e39",
        "0x6.de392c9978b4267553b428865de8E+32#112",
        "1.18771971548231249401e-58",
        "0xb.edbf4827e1e28aaE-49#64",
        63,
        "2.3370796820961060044e39",
        "0x6.de392c9978b4267E+32#63",
        Less,
    );
    // - difw > ys_len in add_float_significands_same_prec_general
    // - difw > ys_len || goto_c_read in add_float_significands_same_prec_general
    // - following_bits != Uninitialized second time in add_float_significands_same_prec_general
    // - following_bits == False second time in add_float_significands_same_prec_general
    // - xs[xi] != 0 in add_float_significands_same_prec_general
    test(
        "1248577957.9617995883835430866672787859939813175787209064549678049868",
        "0x4a6bc9a5.f6387f7169e05140ece8db047baba25ac8c576b8fed10fa4#221",
        "1.65314121012e-6",
        "0x0.00001bbc2ffb9e#36",
        126,
        "1248577957.96180124152475320394882420796",
        "0x4a6bc9a5.f6389b2d99dbef40ece8db04#126",
        Less,
    );
    // - x < y in add_float_significands_same_prec_general
    // - following_bits == False first time in add_float_significands_same_prec_general
    // - round_bit != False or not other condition in add_float_significands_same_prec_general
    test(
        "1.85445e-25",
        "0x3.9648E-21#15",
        "1.29757390426144922727693550499097125604637196578826715875579996363879712057e-6",
        "0x0.000015c509987b7b0dbb1bf2aae59a4afde515d3ec2c3af539738e362659e1f1b0#243",
        62,
        "1.2975739042614492276e-6",
        "0x0.000015c509987b7b0dbb8#62",
        Greater,
    );
    // - following_bits == Uninitialized second time in add_float_significands_same_prec_general
    // - round_bit == Uninitialized third time in add_float_significands_same_prec_general
    // - following_bits != False second time in add_float_significands_same_prec_general
    test(
        "1.3111820218254081035114504135472568116036464005e-6",
        "0x0.000015ff7be10e865ada82cd25acef5baa9c89c25f4#152",
        "2.514658916011e-20",
        "0x7.6c05c64a8E-17#38",
        128,
        "1.311182021825433250100610518712675662596e-6",
        "0x0.000015ff7be10e86d19adf31cdacef5baa9c8#128",
        Less,
    );
    // - following_bits == False || x == Limb::MAX second time in
    //   add_float_significands_same_prec_general
    // - !goto_c_read in add_float_significands_same_prec_general
    // - following_bits != False || yi < 0 in add_float_significands_same_prec_general
    test(
        "7.99999999999954525264911353588104248",
        "0x7.ffffffffff800000000000000000#114",
        "0.00390627977640",
        "0x0.0100007fe38#34",
        49,
        "8.003906279775947",
        "0x8.0100007fe300#49",
        Equal,
    );
    // - following_bits != False first time in add_float_significands_same_prec_general
    test(
        "2.899015055705895854352e-11",
        "0x1.fe0007ffc00000000E-9#67",
        "134217728.000003814697251",
        "0x8000000.00003ffffffc#74",
        51,
        "134217728.00000381",
        "0x8000000.000040#51",
        Less,
    );
    // - yi < 0 second time in add_float_significands_same_prec_general;
    // - goto_c_read in add_float_significands_same_prec_general
    test(
        "4.44747943e-47",
        "0x4.0ffff0E-39#25",
        "7.523135146670945963453530822847271532226579417449854573285752516755e-37",
        "0xf.fffc000001fffffffffffffffffffffffffffc03fffffffc001ffeE-31#219",
        57,
        "7.523135147115693907e-37",
        "0xf.fffc000411fff0E-31#57",
        Greater,
    );
    // - round_bit == False and other condition in add_float_significands_same_prec_general
    test(
        "1048575.9999999999999999965",
        "0xfffff.ffffffffffffffc0#81",
        "1.11022302048925347813991700511654794719796250e-16",
        "0x7.ffffff800000003ffe000001fffffffc0000E-14#145",
        48,
        "1048576.000000000",
        "0x100000.0000000#48",
        Less,
    );
    // - x == Limb::MAX first time in add_float_significands_same_prec_general
    // - xi != 0 in add_float_significands_same_prec_general
    test(
        "3.8109719753265389e-6",
        "0x0.00003ff00000000004#52",
        "4.903985730770844346746710486880989387579965177e55",
        "0x1.ffffffffffffffffffffffffffffffffffffeE+46#148",
        21,
        "4.9039857e55",
        "0x2.00000E+46#21",
        Greater,
    );
    // - xs[xi] == 0 in add_float_significands_same_prec_general
    test(
        "135.9980",
        "0x87.ff8#19",
        "8796093087743.998046875000000000000000000000000000000",
        "0x8000000ffff.ff80000000000000000000000000000#168",
        114,
        "8796093087879.99609375000000000000000",
        "0x80000010087.ff0000000000000000#114",
        Equal,
    );
    // - following_bits == False && yi >= 0 in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 && y_prec << (Limb::WIDTH - exp_diff_rem) != 0 in
    //   add_float_significands_same_prec_general
    test(
        "4610525002867933183.99999999999977",
        "0x3ffbe00fffffffff.ffffffffffc#104",
        "2.273736889924504931e-13",
        "0x4.000003ffc00000E-11#58",
        47,
        "4.610525002867933e18",
        "0x3.ffbe01000000E+15#47",
        Less,
    );
    // - exp_diff_rem == 0 first time in add_float_significands_same_prec_general
    test(
        "199484.9246647061832808582",
        "0x30b3c.ecb6d380d2988d78#79",
        "9.29457676064835e-15",
        "0x2.9dbeadc4568E-12#45",
        63,
        "199484.92466470618328",
        "0x30b3c.ecb6d380d298#63",
        Less,
    );
    // - exp_diff_rem == 0 || y_prec << (Limb::WIDTH - exp_diff_rem) == 0 in
    //   add_float_significands_same_prec_general
    // - difw <= ys_len && !goto_c_read in add_float_significands_same_prec_general
    test(
        "128.000244140566792339087",
        "0x80.000fffffc00000000#75",
        "7.500000000000000000000000000",
        "0x7.800000000000000000000#87",
        60,
        "135.50024414056679234",
        "0x87.800fffffc0000#60",
        Equal,
    );
    // - exp_diff_rem == 0 second time in add_float_significands_same_prec_general
    // - yi != 0 first time in add_float_significands_same_prec_general
    test(
        "127.99999999999999977448594853660538400296071611887941935108",
        "0x7f.ffffffffffffefc000007fffffffffff0000000001e000#190",
        "64.000000000000000000000000000000000000000000000041053665947096",
        "0x40.000000000000000000000000000000000000003c000000008#200",
        90,
        "191.99999999999999977448594854",
        "0xbf.ffffffffffffefc000008#90",
        Greater,
    );
    // - following_bits == False || x == Limb::MAX first time in
    //   add_float_significands_same_prec_general
    test(
        "1927941831.98168915743",
        "0x72ea0ec7.fb4ffb0a0#65",
        "10702647.0",
        "0xa34f37.0#24",
        63,
        "1938644478.9816891574",
        "0x738d5dfe.fb4ffb0a#63",
        Equal,
    );
    // - round_bit == Uninitialized sixth time in add_float_significands_same_prec_general
    test(
        "8.691343304080493e-16",
        "0x3.ea0b2d4be674E-13#48",
        "15492.756587362761273446",
        "0x3c84.c1afb59ba066160#71",
        128,
        "15492.75658736276127431557748582959734014",
        "0x3c84.c1afb59ba06654a0b2d4be6740000#128",
        Equal,
    );
    // - ys[yi] != 0 in add_float_significands_same_prec_general
    test(
        "2.3841812435421161353588104248046875000000000000000000248859839481e-7",
        "0x3.ffff8000000000000000000000000000000000000003ffc00000E-6#210",
        "1.50000000000000000000000",
        "0x1.8000000000000000000#75",
        55,
        "1.50000023841812435",
        "0x1.800003ffff8000#55",
        Less,
    );
    // - x == Limb::MAX second time in add_float_significands_same_prec_general
    test(
        "98079714615416886934934209737619787751599303785390800896.00024414051",
        "0x3fffffffffffffffffffffffffffffffffffff800000000.000fffff8#219",
        "5.82076609067e-11",
        "0x3.fffffffe0E-9#35",
        63,
        "9.8079714615416886935e55",
        "0x4.000000000000000E+46#63",
        Greater,
    );
    // - xi == 0 in add_float_significands_same_prec_general
    test(
        "2.9962728642125103686e-95",
        "0x3.fffffff00000000E-79#60",
        "1.490121803726651705801486968994140624991e-8",
        "0x4.0000fffffffffffffffffffffffffff8E-7#128",
        27,
        "1.490121804e-8",
        "0x4.000100E-7#27",
        Greater,
    );
    // - x < y and other conditions in add_float_significands_same_prec_general
    test(
        "0.000244140624999999999999999798051608263421",
        "0x0.000ffffffffffffffffffff00000000000#124",
        "1.323488977619653960e-23",
        "0xf.ffffff80000008E-20#57",
        63,
        "0.00024414062500000000000",
        "0x0.0010000000000000000#63",
        Less,
    );
    // - ys[yi] == 0 in add_float_significands_same_prec_general
    test(
        "0.99951171875000000000000000000",
        "0x0.ffe00000000000000000000#92",
        "0.01757812499999644728632119949907064437866210937500000000000000000000",
        "0x0.047fffffffff00000000000000000000000000000000000000000000#217",
        48,
        "1.017089843750000",
        "0x1.046000000000#48",
        Greater,
    );
    // - yi == 0 first time in add_float_significands_same_prec_general
    test(
        "67108864.00000000000000000002710505431213761085018632002174854278437005772",
        "0x4000000.00000000000000007ffffffffffffffffffffffffffffff800000#236",
        "3.63797880709171295166015625e-12",
        "0x4.000000000000000000000E-10#86",
        103,
        "67108864.0000000000036379788341968",
        "0x4000000.0000000004000000800#103",
        Greater,
    );
    test(
        "3.919234147519957759670651047978964644479784578601128330613474e-17",
        "0x2.d2f8998a5fa796b9de129923cf4d51e04f27e6b6a368fadb0E-14#197",
        "2.751649483360146129687597980850257480082e-17",
        "0x1.fb96f911d37dd9fe4933cbeee25d7ddeE-14#128",
        8,
        "6.679e-17",
        "0x4.d0E-14#8",
        Greater,
    );

    test(
        "-5.514644e-323228497",
        "-0x2.50888E-268435456#19",
        "7.2966e-323228497",
        "0x3.10E-268435456#10",
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-7.2966e-323228497",
        "-0x3.10E-268435456#10",
        "5.514644e-323228497",
        "0x2.50888E-268435456#19",
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn add_prec_fail() {
    assert_panic!(Float::NAN.add_prec(Float::NAN, 0));
    assert_panic!(Float::NAN.add_prec_val_ref(&Float::NAN, 0));
    assert_panic!(Float::NAN.add_prec_ref_val(Float::NAN, 0));
    assert_panic!(Float::NAN.add_prec_ref_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.add_prec_assign(Float::NAN, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.add_prec_assign_ref(&Float::NAN, 0)
    });
}

#[test]
fn test_add_round() {
    let test = |s, s_hex, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (sum, o) = x.clone().add_round(y.clone(), rm);
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.clone().add_round_val_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_round_ref_val(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_round_ref_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_round_assign(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_round_assign_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) =
                rug_add_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum),
            );
            assert_eq!(rug_o, o);
        }

        let (sum_alt, o_alt) = add_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            rm,
        );
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test("NaN", "NaN", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "NaN", "NaN", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("NaN", "NaN", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal);

    test("NaN", "NaN", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal);

    test(
        "Infinity", "Infinity", "NaN", "NaN", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal);

    // Note different behavior for Floor
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test("-0.0", "-0x0.0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    // Note different behavior for Floor
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Floor, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal,
    );
    test("123.0", "0x7b.0#7", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("123.0", "0x7b.0#7", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Exact, "NaN", "NaN", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Floor, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Down, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Up, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Exact, "123.0", "0x7b.0#7", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Floor, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Down, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Up, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Exact, "123.0", "0x7b.0#7", Equal,
    );

    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "123.0", "0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123.0", "0x7b.0#7", Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Floor, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Down, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Up, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Exact, "123.0", "0x7b.0#7", Equal,
    );

    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Floor, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Down, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Up, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Exact, "123.0", "0x7b.0#7", Equal,
    );

    // - (round_bit != 0 || sticky_bit == 0) && (rm == Floor || rm == Down) in
    //   add_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Floor, "2.0", "0x2.0#1", Less,
    );
    // - (round_bit != 0 || sticky_bit == 0) && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_lt_w
    // - (rm == Ceiling || rm == Up) && sum == 0 in add_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Down, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Up, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Nearest, "4.0", "0x4.0#1", Greater,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "3.0", "0x3.0#2", Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );

    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "4.5558062159628880",
        "0x4.8e4950f0795fc#53",
        Less,
    );
    // - (rm == Ceiling || rm == Up) && sum != 0 in add_float_significands_same_prec_lt_w
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "4.5558062159628889",
        "0x4.8e4950f079600#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "4.5558062159628880",
        "0x4.8e4950f0795fc#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "4.5558062159628889",
        "0x4.8e4950f079600#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "4.5558062159628880",
        "0x4.8e4950f0795fc#53",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Exact,
        "-1.7273790912166980",
        "-0x1.ba35842091e63#53",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Exact,
        "1.7273790912166980",
        "0x1.ba35842091e63#53",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-4.5558062159628889",
        "-0x4.8e4950f079600#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-4.5558062159628880",
        "-0x4.8e4950f0795fc#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-4.5558062159628880",
        "-0x4.8e4950f0795fc#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-4.5558062159628889",
        "-0x4.8e4950f079600#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-4.5558062159628889",
        "-0x4.8e4950f079600#53",
        Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );

    // Note different behavior for Floor
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Exact, "0.0", "0x0.0", Equal,
    );

    // - x_exp > y_exp in add_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Nearest, "0.0", "0x0.0", Equal,
    );
    // - x_exp == y_exp in add_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", Nearest, "2.0", "0x2.0#1", Equal,
    );
    // - rm == Nearest && sum != 0 in add_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#3", "1.8", "0x1.c#3", Nearest, "3.0", "0x3.0#3", Greater,
    );
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_lt_w
    test(
        "7.697e14",
        "0x2.bcE+12#8",
        "1.237e-9",
        "0x5.50E-8#8",
        Nearest,
        "7.697e14",
        "0x2.bcE+12#8",
        Less,
    );
    // - shift <= exp_diff < Limb::WIDTH in add_float_significands_same_prec_lt_w
    // - shift <= exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_lt_w
    test(
        "1.852193494e22",
        "0x3.ec137baE+18#29",
        "241425944.0",
        "0xe63de18.0#29",
        Nearest,
        "1.852193494e22",
        "0x3.ec137baE+18#29",
        Less,
    );
    test(
        "1.999999999999993",
        "0x1.fffffffffffe#48",
        "5.960464477539062e-8",
        "0x1.000000000000E-6#48",
        Nearest,
        "2.000000059604645",
        "0x2.000001000000#48",
        Greater,
    );

    // - in add_float_significands_same_prec_w
    // - x_exp == y_exp in add_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        Nearest,
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_w
    // - !overflow in add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        Nearest,
        "3.00000000000000000000",
        "0x3.0000000000000000#64",
        Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_w
    test(
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        Nearest,
        "3.00000000000000000000",
        "0x3.0000000000000000#64",
        Equal,
    );
    // - (round_bit != 0) || (sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_w
    // - round_bit == 0 || (sticky_bit == 0 && (sum & 1) == 0) in add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        Nearest,
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        Less,
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) in add_float_significands_same_prec_w
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and !overflow in
    //   add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000033",
        "0x1.0000000000000006#64",
        Nearest,
        "2.00000000000000000043",
        "0x2.0000000000000008#64",
        Greater,
    );
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_w
    test(
        "5.93763496769044317938e-6",
        "0x0.0000639df2b03f3e49a70#64",
        "2.93472512905146303523e-45",
        "0x1.0c11b075f03d6daeE-37#64",
        Nearest,
        "5.93763496769044317938e-6",
        "0x0.0000639df2b03f3e49a70#64",
        Less,
    );
    // - overflow in add_float_significands_same_prec_w
    test(
        "0.000221852535829092939590",
        "0x0.000e8a1162cbb1a4265#64",
        "0.0000297456615217170340009",
        "0x0.0001f30ca4b8117ff0a0#64",
        Nearest,
        "0.000251598197350809973604",
        "0x0.00107d1e0783c324170#64",
        Greater,
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and overflow in
    //   add_float_significands_same_prec_w
    test(
        "63.9999999999999999965",
        "0x3f.ffffffffffffffc#64",
        "64.0000000000000000000",
        "0x40.000000000000000#64",
        Nearest,
        "128.000000000000000000",
        "0x80.00000000000000#64",
        Greater,
    );

    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Floor || rm == Down) in
    //   add_float_significands_same_prec_w
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Floor || rm == Down) && sum != 0 in
    //   add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        Down,
        "2.00000000000000000000",
        "0x2.0000000000000000#64",
        Less,
    );
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_w
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum != 0 in
    //   add_float_significands_same_prec_w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        Up,
        "2.00000000000000000022",
        "0x2.0000000000000004#64",
        Greater,
    );
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum == 0 in
    //   add_float_significands_same_prec_w
    test(
        "536870911.999999999971",
        "0x1fffffff.ffffffffe#64",
        "1.00974195868289511071e-28",
        "0x7.ffffffffffffffe0E-24#64",
        Up,
        "536870912.000000000000",
        "0x20000000.000000000#64",
        Greater,
    );

    // - in add_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        Nearest,
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        Nearest,
        "3.00000000000000000000",
        "0x3.0000000000000000#65",
        Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        Nearest,
        "3.00000000000000000000",
        "0x3.0000000000000000#65",
        Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (sum_0 != 0 || sum_1 != 0) in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        Less,
    );
    // - rm == Nearest && sum_1 != 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        Nearest,
        "2.00000000000000000022",
        "0x2.0000000000000004#65",
        Greater,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "18.6312303425262679475864699",
        "0x12.a1984fcd64a8ae228eef#85",
        Nearest,
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        Less,
    );
    // - exp_diff >= Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.85458229226496712261e27",
        "0xf.af9dc963a0709f78E+22#65",
        "1.14823551075108882469e-96",
        "0x2.73dea72af3fe6314E-80#65",
        Nearest,
        "4.85458229226496712261e27",
        "0xf.af9dc963a0709f78E+22#65",
        Less,
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "19585.2851423168986928116147584507795",
        "0x4c81.48ff163dc91a0d4bd90309b0f8#116",
        "372369974082165972902790.766638151683",
        "0x4eda377c7f0d747fa386.c44265dd58#116",
        Nearest,
        "372369974082165972922376.051780468581",
        "0x4eda377c7f0d747ff008.0d417c1b20#116",
        Less,
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "18.4926492161804358300005290",
        "0x12.7e1e424fe51f1bb914c0#85",
        "56.6375897899064714038448469",
        "0x38.a339159fe96c1722fdfe#85",
        Nearest,
        "75.1302390060869072338453776",
        "0x4b.215757efce8b32dc12c0#85",
        Greater,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "5.293955922766053551082318577017522e-23",
        "0x4.00000007e000fffffff0000000E-19#107",
        "255.9999999999999999999999470604408",
        "0xff.ffffffffffffffffffc000000#107",
        Nearest,
        "256.0000000000000000000000000000000",
        "0x100.0000000000000000000000000#107",
        Less,
    );
    // - rm == Nearest && sum_1 == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.00006103515624999999999959",
        "0x0.0003ffffffffffffffff8#67",
        "17179869183.99993896484",
        "0x3ffffffff.fffc00000#67",
        Nearest,
        "17179869184.00000000000",
        "0x400000000.00000000#67",
        Greater,
    );

    // - rm == Floor || rm == Down in add_float_significands_same_prec_gt_w_lt_2w
    // - (rm == Floor || rm == Down) && (sum_0 != 0 || sum_1 != 0) in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Floor,
        "2.00000000000000000000",
        "0x2.0000000000000000#65",
        Less,
    );
    // - rm == Ceiling || rm == Up in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Ceiling || rm == Up && sum_1 != 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000000",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Ceiling,
        "2.00000000000000000011",
        "0x2.0000000000000002#65",
        Greater,
    );
    // - rm == Ceiling || rm == Up && sum_1 == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.9999999999999999999999998",
        "0x1.ffffffffffffffffffffc#83",
        "2.4074118565121938372272894e-35",
        "0x1.fffff8000000007fffff8E-29#83",
        Ceiling,
        "2.0000000000000000000000000",
        "0x2.000000000000000000000#83",
        Greater,
    );

    // - in add_float_significands_same_prec_2w
    // - x_exp == y_exp in add_float_significands_same_prec_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        Nearest,
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH && !overflow in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        Nearest,
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#128",
        Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_2w
    test(
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        Nearest,
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#128",
        Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_2w
    // - rm == Nearest in add_float_significands_same_prec_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (sum_0 & 1) == 0)) in
    //   add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        Nearest,
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (sum_0 & 1) != 0) in
    //   add_float_significands_same_prec_2w
    // - rm == Nearest && !overflow in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000018",
        "0x1.00000000000000000000000000000006#128",
        Nearest,
        "2.000000000000000000000000000000000000024",
        "0x2.00000000000000000000000000000008#128",
        Greater,
    );
    // - exp_diff >= TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff > TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "5.809911490453824289488892996394197332617e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        "5.078012491369571452708077262055118554215e-45",
        "0x1.cfd8608b7c32de2bbcfecf8bcf8a2d00E-37#128",
        Nearest,
        "5.809911490453824289488892996394197332617e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        Less,
    );
    // - Limb::WIDTH <= exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "4354249796990942.354353575265977831431638",
        "0xf782ac869b7de.5ab6ea78fcf0cc5079f#128",
        "8.032394538257265122403070534052560160215e-10",
        "0x3.732bce7aa121827a284545a25f32dc68E-8#128",
        Nearest,
        "4354249796990942.354353576069217285257364",
        "0xf782ac869b7de.5ab6ea7c701c9acb1b1#128",
        Less,
    );
    // - exp_diff == TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "15732412727332569995335732833027757624.44",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.7#128",
        "0.03737083028200858887607458416398961289207",
        "0x0.0991227de2b63edc67164401ce8ebdb04#128",
        Nearest,
        "15732412727332569995335732833027757624.50",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.8#128",
        Greater,
    );
    // - Limb::WIDTH == exp_diff in add_float_significands_same_prec_2w
    test(
        "1.057437459917463716438672572710788562701e-17",
        "0xc.310127aae1df1a1cb12f60c4d339d76E-15#128",
        "148.0549133677002965445211858794413066474",
        "0x94.0e0ecd6e62d0a8c7c7c2a633277e3e#128",
        Nearest,
        "148.0549133677002965550955604786159438120",
        "0x94.0e0ecd6e62d0a98ad7d520e1456fe0#128",
        Greater,
    );
    // - exp_diff < TWICE_WIDTH && overflow in add_float_significands_same_prec_2w
    test(
        "990.8902848544842589812043163049608986645",
        "0x3de.e3e9b54e224e900a8701c94cea27bc#128",
        "111.9722425438858761689147540845231217720",
        "0x6f.f8e4e329c509f04b7f9497ec8ce6438#128",
        Nearest,
        "1102.862527398370135150119070389484020437",
        "0x44e.dcce9877e758805606966139770e00#128",
        Greater,
    );
    // - rm == Nearest && overflow in add_float_significands_same_prec_2w
    test(
        "1152920954851033088.000000000000000000000",
        "0xfffff8000000000.00000000000000000#128",
        "549755813887.9999999999999999999999999984",
        "0x7fffffffff.ffffffffffffffffffffff8#128",
        Nearest,
        "1152921504606846976.000000000000000000000",
        "0x1000000000000000.00000000000000000#128",
        Greater,
    );
    // - rm == Floor || m == Down in add_float_significands_same_prec_2w
    // - (rm == Floor || m == Down) && (sum_0 != 0 || sum_1 != 0) in
    //   add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        Floor,
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#128",
        Less,
    );
    // - rm == Ceiling || m == Up in add_float_significands_same_prec_2w
    // - (rm == Ceiling || m == Up) && !overflow in add_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        Ceiling,
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#128",
        Greater,
    );
    // - (rm == Ceiling || m == Up) && overflow in add_float_significands_same_prec_2w
    test(
        "69631.99999998509883902819694864918230760",
        "0x10fff.ffffffc000000ffffffff80007fc#128",
        "1.255420347077336152767157884641533283217e58",
        "0x1.fffffffffffffffffffffffffffffffeE+48#128",
        Ceiling,
        "1.255420347077336152767157884641533283220e58",
        "0x2.00000000000000000000000000000000E+48#128",
        Greater,
    );

    // - (round_bit != 0 || sticky_bit != 0) && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum_2 != 0 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        Up,
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#129",
        Greater,
    );
    test(
        "9903517953099800764370386944.00000000002910383045663",
        "0x1fffff800000000000000000.000000001fffffffff8#166",
        "1.68499666669691498716668845382709804457445989930032e66",
        "0x1.0000000000000000000001fff80000000000000000E+55#166",
        Up,
        "1.68499666669691498716668845382709804458436341725346e66",
        "0x1.0000000000000000000001fff8000001fffff80008E+55#166",
        Greater,
    );
    // - in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp > y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Nearest,
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#129",
        Less,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
        Nearest,
        "3.000000000000000000000000000000000000000",
        "0x3.00000000000000000000000000000000#129",
        Less,
    );
    // - x_exp == y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        Nearest,
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
        Equal,
    );
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        Nearest,
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#129",
        Greater,
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 < exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        Nearest,
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        Less,
    );
    // - exp_diff >= Limb::WIDTH * 3 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "439761088891971104563481495859833667777753437696.0",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        "6.88923601593624215957288189353784878326857540585e-50",
        "0x1.9c693c182df3035eef00d41638bbdd942f4d498E-41#155",
        Nearest,
        "439761088891971104563481495859833667777753437696.0",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        Less,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.28505372386063746523518779888117963738987726e-22",
        "0x2.0607fd4819748c532aad3528693c1e3c1966E-18#146",
        "978.493288099344953918398808019894399812365694",
        "0x3d2.7e4820fe314caadcb9a156bef2f1c8e53c#146",
        Nearest,
        "978.493288099344953918399236525266785876112215",
        "0x3d2.7e4820fe314caadcbba75ebc3b0b3d718f#146",
        Less,
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4499918.462192628409483090774709614007863365",
        "0x44a9ce.7652418f789422bc22220831e2030#137",
        "64560208.02626195160236877593517814393478863",
        "0x3d91c50.06b91a6f42e5205070f82f89eefa#137",
        Nearest,
        "69060126.48845458001185186670988775794265180",
        "0x41dc61e.7d0b5bfebb79430c931a37bbd0fc#137",
        Less,
    );
    // - exp_diff == Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "563971925627753843356041629019151473018178607215.422",
        "0x62c960337e963a378ba6626ea422d8a5e623986f.6c#165",
        "1301375421.83361702516620516356439489325145225661938",
        "0x4d9169bd.d567ece47a47ef60371d48c969ba8765d4#165",
        Nearest,
        "563971925627753843356041629019151473019479982637.250",
        "0x62c960337e963a378ba6626ea422d8a633b5022d.40#165",
        Less,
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "226.93050907532437979947076285686054061939",
        "0xe2.ee35d7bf263fda8c632644ad7c49d98#130",
        "4262448175090788889452.9841882569848613913",
        "0xe71159efd3a67e736c.fbf3c2f8db72fb8#130",
        Nearest,
        "4262448175090788889679.9146973323092411901",
        "0xe71159efd3a67e744f.ea299ab801b2d60#130",
        Less,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1180591620717411303423.999999999999999999999999997",
        "0x3fffffffffffffffff.ffffffffffffffffffffff#158",
        "5.684341886163519548224296320364965458063242301206e-14",
        "0x1.000000000fffffffffffe0000000000000000038E-11#158",
        Nearest,
        "1180591620717411303424.000000000000056843418861629",
        "0x400000000000000000.00000000001000000000fe#158",
        Less,
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum == 0
    test(
        "4503599627370495.99999999999999999999999999960",
        "0xfffffffffffff.ffffffffffffffffffffffe#143",
        "3.30872245098247973850465205378347282876506684e-24",
        "0x4.00000003ffff0000000000000000fffffffE-20#143",
        Nearest,
        "4503599627370496.00000000000000000000000330872",
        "0x10000000000000.00000000000000000004000#143",
        Greater,
    );
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Floor || rm == Down) in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Floor || rm == Down) && sum != 0 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000000",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Down,
        "2.000000000000000000000000000000000000000",
        "0x2.00000000000000000000000000000000#129",
        Less,
    );
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum_2 == 0 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "18014398509481983.9999999999999999999999999999999998",
        "0x3fffffffffffff.ffffffffffffffffffffffffffff#166",
        "9.62964972193617926527988971292463659269050824107684e-35",
        "0x7.ffffffffffffffffffffffffffffffffffffffffeE-29#166",
        Up,
        "18014398509481984.0000000000000000000000000000000000",
        "0x40000000000000.0000000000000000000000000000#166",
        Greater,
    );

    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && round_bit == 0 && sticky_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Up,
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Equal,
    );
    // - in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && round_bit == 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        Nearest,
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec in add_float_significands_same_prec_ge_3w
    // - in add_significands_rsh_to_out
    // - exp_diff < Limb::WIDTH in add_significands_rsh_to_out
    // - 0 < exp_diff < prec && shift == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && limb == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Nearest,
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_ge_3w
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        Nearest,
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Equal,
    );
    // - 0 < exp_diff < prec && shift != 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.00000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#193",
        "2.00000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#193",
        Nearest,
        "3.00000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#193",
        Equal,
    );
    // - exp_diff == 0 && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        Nearest,
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Less,
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && sticky_bit == 0 && out[0] &
    //   shift_bit == 0 in add_float_significands_same_prec_ge_3w
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        Nearest,
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Less,
    );
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000010",
        "0x1.000000000000000000000000000000000000000000000006#192",
        Nearest,
        "2.0000000000000000000000000000000000000000000000000000000013",
        "0x2.000000000000000000000000000000000000000000000008#192",
        Greater,
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && !carry in add_float_significands_same_prec_ge_3w
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000010",
        "0x1.000000000000000000000000000000000000000000000006#192",
        Nearest,
        "3.0000000000000000000000000000000000000000000000000000000013",
        "0x3.000000000000000000000000000000000000000000000008#192",
        Greater,
    );
    // - exp_diff >= Limb::WIDTH in add_significands_rsh_to_out
    // - exp_diff >= Limb::WIDTH && r != 0 in add_significands_rsh_to_out
    test(
        "7.280571169383842274329034483677671964286795147653983789731014e-48",
        "0xa.a3fc2da1f20fb2d9771f86d3c16a444cd62d5d139e3935f24E-40#198",
        "3.512347377882557895896869518765758776035713939594826958897103e-27",
        "0x1.1646de419a6dbd3466f3081403a87d719b7a765a1ec69e4658E-22#198",
        Nearest,
        "3.512347377882557895904150089935142618310042974078504630861396e-27",
        "0x1.1646de419a6dbd3471970441a59a8d2474f195e0f288088aa8E-22#198",
        Greater,
    );
    // - exp_diff >= prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Nearest || rm == Floor
    //   || rm == Down)
    test(
        "4.13222828802191621569015595751616491736159555180726072912070e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        "0.0239913867670311930420667487107083515019528907529246130057244",
        "0x0.06244cad8cd272134e34b325815ad281733f2c06231a0ee744#193",
        Nearest,
        "4.13222828802191621569015595751616491736159555180726072912070e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        Less,
    );
    // - 0 < exp_diff < prec && limb != 0 in add_float_significands_same_prec_ge_3w
    test(
        "8.69977204237437814069372807483827970856267379941609710779639",
        "0x8.b32442b4a730454d66b1b2bdf7a2863d417e6ff22d7f6c58#193",
        "7.58974636819623954377405988444623535636829063921159080831479",
        "0x7.96f99e34566e7be1960d023e431dc5e0a7ad24ad691a1ac4#193",
        Nearest,
        "16.2895184105706176844677879592845150649309644386276879161118",
        "0x10.4a1de0e8fd9ec12efcbeb4fc3ac04c1de92b949f9699872#193",
        Greater,
    );
    // - exp_diff >= Limb::WIDTH && r == 0 in add_significands_rsh_to_out
    test(
        "6.4425523507465541098853496915929918929896246856311922355487e-6",
        "0x0.00006c168d38e231899f0fc85d1888549d5177bdceaee72e15060#192",
        "1476808010161862576835936576709144.7975622615653024045505082",
        "0x48cff00a780a50d34bb694ada218.cc2d0a55f25f9f9126258#192",
        Nearest,
        "1476808010161862576835936576709144.7975687041176531511046180",
        "0x48cff00a780a50d34bb694ada218.cc2d766c7f9881c2afc48#192",
        Less,
    );
    // - exp_diff == prec in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && power && !carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power in add_float_significands_same_prec_ge_3w
    test(
        "4.06358384024552072294006982076688939253797681513643139422220e-23",
        "0x3.1202ecf10ff40b477337957dede18bd7b746884ec977474eE-19#194",
        "1174582238252884689829665592721065057.76655867827770290150723",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d4#194",
        Nearest,
        "1174582238252884689829665592721065057.76655867827770290150729",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d8#194",
        Greater,
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && carry in add_float_significands_same_prec_ge_3w
    test(
        "4.3368086899420177360298112034797668456999388161777350954459e-19",
        "0x7.fffffffffffffffffffffffffffffffe0000000000000000E-16#192",
        "5192296858534827628530496329220095.9999999999999999995663191",
        "0xffffffffffffffffffffffffffff.fffffffffffffff80000#192",
        Nearest,
        "5192296858534827628530496329220096.0000000000000000000000000",
        "0x10000000000000000000000000000.00000000000000000000#192",
        Greater,
    );
    // - exp_diff == prec && rm == Nearest && power && carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power && carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "158456325028528675187087900671.99999999999999999999999999997",
        "0x1ffffffffffffffffffffffff.fffffffffffffffffffffffe#192",
        "2.5243548967072377773175314089049159349542605923488736152645e-29",
        "0x1.fffffffffffffffffffffffffffffffffffffffffffffffeE-24#192",
        Nearest,
        "158456325028528675187087900672.00000000000000000000000000000",
        "0x2000000000000000000000000.000000000000000000000000#192",
        Greater,
    );
    // - exp_diff == prec && rm == Nearest && power in add_float_significands_same_prec_ge_3w
    test(
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
        "2.6469779601696885595885078146238811314105987548828125000000e-23",
        "0x2.000000000000000000000000000000000000000000000000E-19#192",
        Nearest,
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
        Less,
    );
    // - 0 < exp_diff < prec && (rm == Floor || rm == Down || rm == Exact) in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Down,
        "3.0000000000000000000000000000000000000000000000000000000000",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Equal,
    );
    // - exp_diff == 0 && (rm == Floor || rm == Down) in add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        Down,
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Less,
    );
    // - exp_diff == 0 && (rm == Ceiling || rm == Up) in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && (rm == Ceiling || rm == Up) && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000000",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        Up,
        "2.0000000000000000000000000000000000000000000000000000000006",
        "0x2.000000000000000000000000000000000000000000000004#192",
        Greater,
    );
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0)
    //   in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0)
    //   && !carry in add_float_significands_same_prec_ge_3w
    test(
        "2.0000000000000000000000000000000000000000000000000000000000",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        Up,
        "3.0000000000000000000000000000000000000000000000000000000006",
        "0x3.000000000000000000000000000000000000000000000004#192",
        Greater,
    );
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Ceiling || rm == Up)
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Ceiling || rm == Up) &&
    //   !carry
    test(
        "6823.4729678517668736293480068930034606783765135140255689272",
        "0x1aa7.79146bcf65e9c10b73dc31b712bdbba94db27f42827ee#192",
        "3.4171243258195824440860481554099490634319461554553884152664e-68",
        "0xe.bd75c60b3fb1b2daadd125b174611af23cd95ed37b18fd3E-57#192",
        Up,
        "6823.4729678517668736293480068930034606783765135140255689285",
        "0x1aa7.79146bcf65e9c10b73dc31b712bdbba94db27f42827f0#192",
        Greater,
    );
    // - exp_diff == prec && (rm == Ceiling || rm == Up) in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && (rm == Ceiling || rm == Up) && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.1549982013361157285883413763473468330143594437077681839568e-64",
        "0xc.29de762f7d1efb4e2c76c8e4086645c726cd7efd160d9b2E-54#192",
        "7.1837646826832187615349282787452595692709113368513158802894e-7",
        "0xc.0d6747ace11077b45ef60fe0663937937659be6facba820E-6#192",
        Up,
        "7.1837646826832187615349282787452595692709113368513158802909e-7",
        "0xc.0d6747ace11077b45ef60fe0663937937659be6facba821E-6#192",
        Greater,
    );
    // - exp_diff == prec && (rm == Floor || rm == Down) in add_float_significands_same_prec_ge_3w
    test(
        "1.6331850176524973178028299112770291204053359324253462130430e-62",
        "0x6.b7f1cf4acb21f3fca0c966202fee44bb9bb293511aa1d780E-52#192",
        "0.00010303969992625256008619861293450462215178250710705317873879",
        "0x0.0006c0b8243103d1ef7ab2f1e9a66ec9ee623a5e72e237199db8#192",
        Floor,
        "0.00010303969992625256008619861293450462215178250710705317873879",
        "0x0.0006c0b8243103d1ef7ab2f1e9a66ec9ee623a5e72e237199db8#192",
        Less,
    );
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Ceiling || rm == Up) &&
    //   carry
    test(
        "2.35097076557161387088991599992419850310539439291329834333210e-38",
        "0x7.fffc0007ffffffff0000000007ffffffffffffffffffffffE-32#195",
        "2.15679573337205118357336120696157045389097155380324579848825e68",
        "0x7.ffffffffffffffffffffffffffffffffffffffffffffffffE+56#195",
        Up,
        "2.15679573337205118357336120696157045389097155380324579848829e68",
        "0x8.000000000000000000000000000000000000000000000000E+56#195",
        Greater,
    );
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0)
    //   && carry in add_float_significands_same_prec_ge_3w
    test(
        "8388607.9999999995343387126922607421875000000000000000000000",
        "0x7fffff.fffffffe00000000000000000000000000000000000#192",
        "4.6566128730773925781249999999999999999999999999999999999993e-10",
        "0x1.fffffffffffffffffffffffffffffffffffffffffffffffeE-8#192",
        Up,
        "8388608.0000000000000000000000000000000000000000000000000000",
        "0x800000.000000000000000000000000000000000000000000#192",
        Greater,
    );
    // - exp_diff == prec && (rm == Ceiling || rm == Up) && carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "511.99999999999999999999999999999999999999999999999999999992",
        "0x1ff.fffffffffffffffffffffffffffffffffffffffffffffe#192",
        "8.1566305849981556017899813460106708282519025526402724189262e-56",
        "0x1.ffffffffffffff00001ffffffffffffff80003f003000000E-46#192",
        Up,
        "512.00000000000000000000000000000000000000000000000000000000",
        "0x200.0000000000000000000000000000000000000000000000#192",
        Greater,
    );

    // - rm == Floor || rm == Down in add_float_significands_same_prec_general_round
    test(
        "1.0", "0x1.0#1", "1.5", "0x1.8#2", Down, "2.0", "0x2.0#2", Less,
    );
    // - rm == Ceiling || rm == Up in add_float_significands_same_prec_general_round
    // - (rm == Ceiling || rm == Up) && !carry in add_float_significands_same_prec_general_round
    test(
        "1.0", "0x1.0#1", "1.5", "0x1.8#2", Up, "3.0", "0x3.0#2", Greater,
    );
    // - (rm == Ceiling || rm == Up) && carry in add_float_significands_same_prec_general_round
    test(
        "2.0", "0x2.0#1", "1.5", "0x1.8#2", Up, "4.0", "0x4.0#2", Greater,
    );

    // - shift2 == 0 in add_float_significands_same_prec_general
    // - y in add_float_significands_same_prec_general
    // - shift != 0 in add_float_significands_same_prec_general
    // - x == 0 first time in add_float_significands_same_prec_general
    // - shift == 0 || following_bits != Uninitialized in add_float_significands_same_prec_general
    // - round_bit != Uninitialized || shift == 0 in add_float_significands_same_prec_general
    // - exp_diff_rem == 0 && yi == 0 second time in add_float_significands_same_prec_general
    // - round_bit != Uninitialized sixth time in add_float_significands_same_prec_general
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#2", Nearest, "2.0", "0x2.0#2", Equal,
    );
    // - following_bits != False || round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_general_round
    test(
        "1.0", "0x1.0#1", "1.5", "0x1.8#2", Nearest, "2.0", "0x2.0#2", Less,
    );
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && carry in
    //   add_float_significands_same_prec_general_round
    test(
        "2.0", "0x2.0#1", "1.5", "0x1.8#2", Nearest, "4.0", "0x4.0#2", Greater,
    );
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_general_round
    test(
        "1.0", "0x1.0#1", "1.8", "0x1.c#3", Nearest, "3.0", "0x3.0#3", Greater,
    );
    // - x != 0 && x != mask second time in add_float_significands_same_prec_general
    // - rm == Nearest && following_bits != False && round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits != False && round_bit != False && !carry in
    //   add_float_significands_same_prec_general_round
    test(
        "1.5", "0x1.8#2", "4.0", "0x4.0#1", Nearest, "6.0", "0x6.0#2", Greater,
    );
    // - rm == Nearest && following_bits != False && round_bit == False in
    //   add_float_significands_same_prec_general_round
    test(
        "4.0", "0x4.0#1", "1.2", "0x1.4#3", Nearest, "5.0", "0x5.0#3", Less,
    );
    // - x != 0 && x != mask first time in add_float_significands_same_prec_general
    // - shift != 0 && following_bits == Uninitialized in add_float_significands_same_prec_general
    test(
        "1.2", "0x1.4#3", "3.0", "0x3.0#2", Nearest, "4.0", "0x4.0#3", Less,
    );
    // - rm == Nearest && following_bits != False && round_bit != False && carry in
    //   add_float_significands_same_prec_general_round
    test(
        "1.8", "0x1.c#3", "6.0", "0x6.0#2", Nearest, "8.0", "0x8.0#3", Greater,
    );
    // - out_bits <= exp_diff in add_float_significands_same_prec_general
    // - out_len <= xs_len first time in add_float_significands_same_prec_general
    // - difw > 0 && difw > ys_len && exp_diff > out_bits in
    //   add_float_significands_same_prec_general
    // - round_bit != Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "8.821880e11",
        "0xc.d668E+9#18",
        "9.7545998337406e122",
        "0x1.79c17f063aE+102#40",
        Nearest,
        "9.7545998337406e122",
        "0x1.79c17f063aE+102#40",
        Less,
    );
    // - out_len > xs_len first time in add_float_significands_same_prec_general
    test(
        "2.8577648979177105962332201291018926848163080599637e-19",
        "0x5.458a93bffa7b1c05bdd1c0552b60196746d9083cE-16#162",
        "3.5697206995078675e50",
        "0xf.4400d3acf388E+41#51",
        Nearest,
        "3.5697206995078675404584127554321345196383736430592e50",
        "0xf.4400d3acf3880000000000000000000000000000E+41#162",
        Less,
    );
    // - overlap > ys_len in add_float_significands_same_prec_general
    // - out_len - k > overlap in add_float_significands_same_prec_general
    // - difw <= 0 || difw <= ys_len in add_float_significands_same_prec_general
    // - round_bit != Uninitialized fourth time in add_float_significands_same_prec_general
    test(
        "29780282551.7626844589368663631653",
        "0x6ef0b0cb7.c33f49e84d21bb6040#104",
        "0.0000394559894753800",
        "0x0.000295f62f36adb#46",
        Nearest,
        "29780282551.7627239149263417431406",
        "0x6ef0b0cb7.c341dfde7c58691040#104",
        Equal,
    );
    // - out_len > xs_len second time in add_float_significands_same_prec_general
    test(
        "1.071839725139585312577139389278151e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        Nearest,
        "0.003742228294246889823114823919652848",
        "0x0.00f5402c235119eafa12eb9821602#107",
        Greater,
    );
    // - exp_diff_rem == 0 second time in add_float_significands_same_prec_general
    test(
        "5.191920952034e-15",
        "0x1.761e097c5E-12#37",
        "7.37e4",
        "0x1.2E+4#5",
        Nearest,
        "73728.00000000",
        "0x12000.00000#37",
        Less,
    );
    // - shift <= 1 in add_float_significands_same_prec_general
    test(
        "15135.895602865542606017656527713819177465060416097749360065",
        "0x3b1f.e5463ab9b599ce49b83c7988b324dc93ce50b2ed51a18#191",
        "3.5815296244999700478867322252421807366487e-8",
        "0x9.9d355ad2b99a587727da095fa3226bf0E-7#130",
        Nearest,
        "15135.895602901357902262656228192686499717482223464235884113",
        "0x3b1f.e5463b5388ef7b7551e200fb30c5728e007771ed51a18#191",
        Equal,
    );
    // - round_bit == Uninitialized fourth time in add_float_significands_same_prec_general
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "8.636437350163444678191748627983325934618e-6",
        "0x0.000090e5374a001358c6606f968bf3813ad9#128",
        "1.849037e-8",
        "0x4.f6a6E-7#18",
        Nearest,
        "8.654927718511001474116650590263599372118e-6",
        "0x0.00009134a1aa001358c6606f968bf3813ad9#128",
        Equal,
    );
    // - round_bit == Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "2.389545997e25",
        "0x1.3c40f7bE+21#29",
        "0.0782632598242840004025",
        "0x0.14090f9d6c745bc06#64",
        Nearest,
        "2.38954599675655770897e25",
        "0x1.3c40f7b000000000E+21#64",
        Less,
    );
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "5.75055158778420135770e-7",
        "0x9.a5d7d56cabed47dE-6#64",
        "1.1758894e-14",
        "0x3.4f515E-12#22",
        Nearest,
        "5.75055170537314368446e-7",
        "0x9.a5d7d8bbfd3d47dE-6#64",
        Equal,
    );
    // - x != 0 && x == mask second time in add_float_significands_same_prec_general
    // - xs_len <= out_len && following_bits == True in add_float_significands_same_prec_general
    test(
        "1.081090215247020702e-18",
        "0x1.3f14ddfe22c0634E-15#59",
        "6.379965759614693e-8",
        "0x1.12047722d26cE-6#47",
        Nearest,
        "6.379965759722801557e-8",
        "0x1.12047722e65d4dcE-6#59",
        Less,
    );
    // - shift == 0 in add_float_significands_same_prec_general
    test(
        "4.3055451539258443718732375731462554408177909736937057433067e-16",
        "0x1.f06543668e6018c20c17efed72ff6d3d65a4c5dc9db475b0E-13#192",
        "1.6388436e-15",
        "0x7.61754E-13#21",
        Nearest,
        "2.0693980969049410047184094732421002104686218250305033868307e-15",
        "0x9.51da83668e6018c20c17efed72ff6d3d65a4c5dc9db475bE-13#192",
        Equal,
    );
    // - yi != 0 second time in add_float_significands_same_prec_general
    test(
        "2.241814356765462e-16",
        "0x1.0276ae5de1e8E-13#47",
        "7.643070003987853963842537238646240439305e-36",
        "0xa.28cd4cb186f5925ddb0d1ecb9681103E-30#128",
        Nearest,
        "2.241814356765462069110833332462974460290e-16",
        "0x1.0276ae5de1e80000a28cd4cb186f5926E-13#128",
        Greater,
    );
    // - x != 0 && x == mask first time in add_float_significands_same_prec_general
    test(
        "2.14747955e9",
        "0x7.ffff00E+7#24",
        "8191.99987888336091",
        "0x1fff.fff80fffff0#54",
        Nearest,
        "2147487743.99987888",
        "0x80000fff.fff810#54",
        Greater,
    );

    test(
        "-5.514644e-323228497",
        "-0x2.50888E-268435456#19",
        "7.2966e-323228497",
        "0x3.10E-268435456#10",
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
}

#[test]
fn add_round_fail() {
    assert_panic!(Float::one_prec(1).add_round(Float::two_prec(1), Exact));
    assert_panic!(Float::one_prec(1).add_round_val_ref(&Float::two_prec(1), Exact));
    assert_panic!(Float::one_prec(1).add_round_ref_val(Float::two_prec(1), Exact));
    assert_panic!(Float::one_prec(1).add_round_ref_ref(&Float::two_prec(1), Exact));

    assert_panic!(parse_hex_string("0x1.0#1").add_round(parse_hex_string("0x0.001#1"), Exact));
    assert_panic!(
        parse_hex_string("0x1.0#1").add_round_val_ref(&parse_hex_string("0x0.001#1"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0#1").add_round_ref_val(parse_hex_string("0x0.001#1"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0#1").add_round_ref_ref(&parse_hex_string("0x0.001#1"), Exact)
    );

    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64")
            .add_round(parse_hex_string("0x1.0000000000000002#64"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64")
            .add_round_val_ref(&parse_hex_string("0x1.0000000000000002#64"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64")
            .add_round_ref_val(parse_hex_string("0x1.0000000000000002#64"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64")
            .add_round_ref_ref(&parse_hex_string("0x1.0000000000000002#64"), Exact)
    );

    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65")
            .add_round(parse_hex_string("0x1.0000000000000001#65"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65")
            .add_round_val_ref(&parse_hex_string("0x1.0000000000000001#65"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65")
            .add_round_ref_val(parse_hex_string("0x1.0000000000000001#65"), Exact)
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65")
            .add_round_ref_ref(&parse_hex_string("0x1.0000000000000001#65"), Exact)
    );

    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round(
            parse_hex_string("0x1.00000000000000000000000000000002#128"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round_val_ref(
            &parse_hex_string("0x1.00000000000000000000000000000002#128"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round_ref_val(
            parse_hex_string("0x1.00000000000000000000000000000002#128"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round_ref_ref(
            &parse_hex_string("0x1.00000000000000000000000000000002#128"),
            Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round(
            parse_hex_string("0x1.00000000000000000000000000000003#129"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round_val_ref(
            &parse_hex_string("0x1.00000000000000000000000000000003#129"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round_ref_val(
            parse_hex_string("0x1.00000000000000000000000000000003#129"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round_ref_ref(
            &parse_hex_string("0x1.00000000000000000000000000000003#129"),
            Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192").add_round(
            parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
            Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .add_round_val_ref(
                &parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
                Exact
            )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .add_round_ref_val(
                parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
                Exact
            )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .add_round_ref_ref(
                &parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
                Exact
            )
    );

    assert_panic!(parse_hex_string("0x1.0#1").add_round(parse_hex_string("0x1.8#2"), Exact));
    assert_panic!(
        parse_hex_string("0x1.0#1").add_round_val_ref(&parse_hex_string("0x1.8#2"), Exact)
    );
    assert_panic!({
        parse_hex_string("0x1.0#1").add_round_ref_val(parse_hex_string("0x1.8#2"), Exact)
    });
    assert_panic!(
        parse_hex_string("0x1.0#1").add_round_ref_ref(&parse_hex_string("0x1.8#2"), Exact)
    );

    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_round_assign(Float::two_prec(1), Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_round_assign_ref(&Float::two_prec(1), Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign(parse_hex_string("0x0.001#1"), Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign_ref(&parse_hex_string("0x0.001#1"), Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#64");
        x.add_round_assign(parse_hex_string("0x1.0000000000000002#64"), Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#64");
        x.add_round_assign_ref(&parse_hex_string("0x1.0000000000000002#64"), Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#65");
        x.add_round_assign(parse_hex_string("0x1.0000000000000001#65"), Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#65");
        x.add_round_assign_ref(&parse_hex_string("0x1.0000000000000001#65"), Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#128");
        x.add_round_assign(
            parse_hex_string("0x1.00000000000000000000000000000002#128"),
            Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#128");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.00000000000000000000000000000002#128"),
            Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#129");
        x.add_round_assign(
            parse_hex_string("0x1.00000000000000000000000000000003#129"),
            Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#129");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.00000000000000000000000000000003#129"),
            Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192");
        x.add_round_assign(
            parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
            Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
            Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign(parse_hex_string("0x1.8#2"), Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign_ref(&parse_hex_string("0x1.8#2"), Exact)
    });
}

#[test]
fn test_add_prec_round() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (sum, o) = x.clone().add_prec_round(y.clone(), prec, rm);
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.clone().add_prec_round_val_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_round_ref_val(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_round_ref_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_round_assign(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_round_assign_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = add_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) = rug_add_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("NaN", "NaN", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal);

    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    // Note different behavior for Floor
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("-0.0", "-0x0.0", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    // Note different behavior for Floor
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        Up,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Down, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Up, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Down, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Up, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Down, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Up, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Down, "2.0", "0x2.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Up, "4.0", "0x4.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        1,
        Up,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Greater,
    );

    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2.0000",
        "0x2.00#10",
        10,
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "4.5547",
        "0x4.8e#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "4.5625",
        "0x4.90#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "4.5547",
        "0x4.8e#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "4.5625",
        "0x4.90#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "4.5547",
        "0x4.8e#10",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-1.7285",
        "-0x1.ba8#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-1.7266",
        "-0x1.ba0#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-1.7266",
        "-0x1.ba0#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-1.7285",
        "-0x1.ba8#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-1.7266",
        "-0x1.ba0#10",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "1.7266",
        "0x1.ba0#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "1.7285",
        "0x1.ba8#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "1.7266",
        "0x1.ba0#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "1.7285",
        "0x1.ba8#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "1.7266",
        "0x1.ba0#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-4.5625",
        "-0x4.90#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-4.5547",
        "-0x4.8e#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-4.5547",
        "-0x4.8e#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-4.5625",
        "-0x4.90#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-4.5547",
        "-0x4.8e#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        Floor,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        Ceiling,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        Down,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        Up,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        Nearest,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.00024",
        "0x0.001#1",
        20,
        Exact,
        "1.0002441",
        "0x1.00100#20",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal,
    );

    // - exp_diff_rem == 0 && yi == 0 first time in add_float_significands_same_prec_general
    test(
        "0.000487804412841796875000000000000000000000000000000000000000291703841149741293879771765\
        5422",
        "0x0.001ff8000000000000000000000000000000000000000000007800000000001fffffffffffe#288",
        "0.0004882812499999965",
        "0x0.001fffffffffffc#47",
        53,
        Ceiling,
        "0.00097608566284179351",
        "0x0.003ff7ffffffffc2#53",
        Greater,
    );
    test(
        "0.0004882812499999965",
        "0x0.001fffffffffffc#47",
        "0.000487804412841796875000000000000000000000000000000000000000291703841149741293879771765\
        5422",
        "0x0.001ff8000000000000000000000000000000000000000000007800000000001fffffffffffe#288",
        53,
        Ceiling,
        "0.00097608566284179351",
        "0x0.003ff7ffffffffc2#53",
        Greater,
    );
    // - round_bit != Uninitialized third time in add_float_significands_same_prec_general
    test(
        "54004143877011445683.033643329400063284751246570213",
        "0x2ed7542badc5a97b3.089cd9678077bdca8dabe52c#160",
        "8.0",
        "0x8.0#1",
        127,
        Up,
        "54004143877011445691.03364332940006328505",
        "0x2ed7542badc5a97bb.089cd9678077bdd0#127",
        Greater,
    );

    test(
        "-1.9480776684637e-10",
        "-0xd.63182fa960E-9#41",
        "2.0360756923641010557931561155205e323228496",
        "0x7.c300ddfc112a6cf2a0bb36cf8E+268435455#100",
        3,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.9480776684637e-10",
        "0xd.63182fa960E-9#41",
        "-2.0360756923641010557931561155205e323228496",
        "-0x7.c300ddfc112a6cf2a0bb36cf8E+268435455#100",
        3,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );

    test(
        "2.46e-323228497",
        "0x1.08E-268435456#6",
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.46e-323228497",
        "-0x1.08E-268435456#6",
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn add_prec_round_fail() {
    assert_panic!(Float::one_prec(1).add_prec_round(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).add_prec_round_val_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_val(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign(Float::two_prec(1), 0, Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign_ref(&Float::two_prec(1), 0, Floor)
    });

    assert_panic!(Float::one_prec(1).add_prec_round(Float::two_prec(1), 1, Exact));
    assert_panic!(Float::one_prec(1).add_prec_round_val_ref(&Float::two_prec(1), 1, Exact));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_val(Float::two_prec(1), 1, Exact));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_ref(&Float::two_prec(1), 1, Exact));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign(Float::two_prec(1), 1, Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign_ref(&Float::two_prec(1), 1, Exact)
    });
}

#[test]
fn test_add_rational() {
    let test = |s, s_hex, t, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let sum = x.clone() + y.clone();
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let sum_alt = x.clone() + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        let sum_alt = y.clone() + x.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = y.clone() + &x;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &y + x.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &y + &x;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        let mut sum_alt = x.clone();
        sum_alt += y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let mut sum_alt = x.clone();
        sum_alt += &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_add_rational(
                &rug::Float::exact_from(&x),
                &rug::Rational::from(&y)
            ))),
            ComparableFloatRef(&sum)
        );

        let sum_alt =
            add_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    };
    test("NaN", "NaN", "123", "NaN", "NaN");
    test("Infinity", "Infinity", "123", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "123", "-Infinity", "-Infinity");
    test("0.0", "0x0.0", "0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "123", "1.3e2", "0x8.0E+1#1");
    test("-0.0", "-0x0.0", "123", "1.3e2", "0x8.0E+1#1");
    test("0.0", "0x0.0", "1/3", "0.25", "0x0.4#1");
    test("-0.0", "-0x0.0", "1/3", "0.25", "0x0.4#1");
    test("123.0", "0x7b.0#7", "0", "123.0", "0x7b.0#7");

    // - same sign in float_rational_sum_exponent_range
    test("1.0", "0x1.0#1", "2", "4.0", "0x4.0#1");
    test("1.0", "0x1.0#2", "2", "3.0", "0x3.0#2");
    test("1.0000", "0x1.000#10", "2", "3.0000", "0x3.00#10");
    test("1.0000", "0x1.000#10", "1/3", "1.3340", "0x1.558#10");
    test(
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "1/3",
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
    );
    // - different signs and log_x_abs.abs_diff(log_y_abs) > 1 in float_rational_sum_exponent_range
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
    );

    test("1.0", "0x1.0#1", "1/50000", "1.0", "0x1.0#1");
    // - signs different and x.eq_abs(&y) in add_rational_prec_round
    test("1.0", "0x1.0#1", "-1", "0.0", "0x0.0");

    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        "1.8316e180",
        "0x7.10E+149#10",
    );
    test("1.0", "0x1.0#1", "0", "1.0", "0x1.0#1");
    // - o == Equal in add_rational_prec_round_assign_helper
    test("1.0", "0x1.0#1", "1", "2.0", "0x2.0#1");
    // - o != Equal in add_rational_prec_round_assign_helper
    // - t != 0 in add_rational_prec_round_assign_helper
    // - small Natural in float_can_round
    // - err0 > prec && err > prec in float_can_round
    // - s != Limb::WIDTH first time in float_can_round
    // - n == 0 in float_can_round
    // - float_can_round in add_rational_prec_round_assign_helper
    test("1.0", "0x1.0#1", "1/3", "1.0", "0x1.0#1");
    // - large Natural in float_can_round
    // - n != 0 && tmp != 0 && tmp != mask in float_can_round
    test(
        "269104312292334.3027",
        "0xf4bfbaf113ee.4d8#57",
        "517543599148951977/6042448266342026218192",
        "269104312292334.3027",
        "0xf4bfbaf113ee.4d8#57",
    );
    // - s == Limb::WIDTH first time in float_can_round
    test(
        "1.1595752615776271305e-33",
        "0x6.055703bef650178E-28#63",
        "-2457795567751474853961645492284796573970712506001349310379799846240055987994196472114730\
        61298054082441720799/23",
        "-1.0686067685875977626e106",
        "-0x1.2a31c100f5e98110E+88#63",
    );
    // - !float_can_round in add_rational_prec_round_assign_helper
    // - n != 0 && tmp == 0 in float_can_round
    // - n <= 0 first time in float_can_round
    // - s != Limb::WIDTH second time in float_can_round
    // - n > 0 first time in float_can_round
    // - x == 0 in float_can_round
    // - x != 0 in float_can_round
    test(
        "5.8215566e33",
        "0x1.1f066E+28#20",
        "8238723/1413731881599214931",
        "5.8215566e33",
        "0x1.1f066E+28#20",
    );
    // - n != 0 && tmp != 0 && tmp == mask in float_can_round
    // - n <= 0 second time in float_can_round
    // - s != Limb::WIDTH third time in float_can_round
    test(
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        "-3183521742267703581572109801877979/35801077870645726",
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
    );
    // - s == Limb::WIDTH second time in float_can_round
    // - n > 0 second time in float_can_round
    // - x == Limb::MAX in float_can_round
    test(
        "5.35864233739103574e63",
        "0xd.06b0f9ea88b7aE+52#55",
        "-808694/11557016618486698036897716112870533564271922504817764249868611771",
        "5.35864233739103574e63",
        "0xd.06b0f9ea88b7aE+52#55",
    );
    // - x != Limb::MAX in float_can_round
    test(
        "4.904596e10",
        "0xb.6b5eE+8#19",
        "-182971083215/47776579472461276067598384651239241670113870043",
        "4.904596e10",
        "0xb.6b5eE+8#19",
    );
    // - s == Limb::WIDTH third time in float_can_round
    test(
        "2.51594092644911199e24",
        "0x2.14c56e507323d0E+20#55",
        "-3161/286335616056113777",
        "2.51594092644911199e24",
        "0x2.14c56e507323d0E+20#55",
    );
    // - err0 <= prec || err <= prec in float_can_round
    // - different signs and log_x_abs.abs_diff(log_y_abs) <= 1 in float_rational_sum_exponent_range
    // - different signs and log_x_abs.abs_diff(log_y_abs) == 1 in float_rational_sum_exponent_range
    test(
        "5.960464455e-8",
        "0xf.ffffffE-7#28",
        "-255/4261412864",
        "-2.346641787e-10",
        "-0x1.0204182E-8#28",
    );
    // - t == 0 in add_rational_prec_round_assign_helper
    // - different signs and log_x_abs == log_y_abs in float_rational_sum_exponent_range
    test(
        "64.0000000",
        "0x40.00000#24",
        "-215679573337202053366254388918461596342975394375083745820919489101824/336999333319768176\
        9435117813341997219028918889483691361889027096575",
        "-3.72416764e-9",
        "-0xf.fec40E-8#24",
    );
}

#[test]
fn test_add_rational_prec() {
    let test = |s, s_hex, t, prec, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (sum, o) = x.clone().add_rational_prec(y.clone(), prec);
        assert!(sum.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let (sum_alt, o_alt) = x.clone().add_rational_prec_val_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_ref_val(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_ref_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_assign(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_assign_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = add_rational_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (rug_sum, rug_o) = rug_add_rational_prec(
            &rug::Float::exact_from(&x),
            &rug::Rational::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", "123", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "123", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", "0", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, "1.3e2", "0x8.0E+1#1", Greater);
    test("-0.0", "-0x0.0", "123", 1, "1.3e2", "0x8.0E+1#1", Greater);
    test("0.0", "0x0.0", "1/3", 1, "0.25", "0x0.4#1", Less);
    test("-0.0", "-0x0.0", "1/3", 1, "0.25", "0x0.4#1", Less);
    test("123.0", "0x7b.0#7", "0", 1, "1.3e2", "0x8.0E+1#1", Greater);

    test("1.0", "0x1.0#1", "2", 1, "4.0", "0x4.0#1", Greater);
    test("1.0", "0x1.0#1", "2", 2, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#2", "2", 1, "4.0", "0x4.0#1", Greater);
    test("1.0", "0x1.0#2", "2", 2, "3.0", "0x3.0#2", Equal);
    test("1.0000", "0x1.000#10", "2", 1, "4.0", "0x4.0#1", Greater);
    test("1.0000", "0x1.000#10", "2", 2, "3.0", "0x3.0#2", Equal);
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        100,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        "3.4766",
        "0x3.7a#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "2.8086",
        "0x2.cf#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        "-2.8086",
        "-0x2.cf#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "-3.4766",
        "-0x3.7a#10",
        Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test("1.0", "0x1.0#1", "-1", 10, "0.0", "0x0.0", Equal);

    test(
        "4.54747350886464118958289900289019017286e-13",
        "0x8.00000000000000003ffff000000000E-11#123",
        "-4194304/9223372019742015487",
        3,
        "-8.5e-22",
        "-0x4.0E-18#3",
        Less,
    );
}

#[test]
fn add_rational_prec_fail() {
    assert_panic!(Float::NAN.add_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::NAN.add_rational_prec_val_ref(&Rational::ZERO, 0));
    assert_panic!(Float::NAN.add_rational_prec_ref_val(Rational::ZERO, 0));
    assert_panic!(Float::NAN.add_rational_prec_ref_ref(&Rational::ZERO, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.add_rational_prec_assign(Rational::ZERO, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.add_rational_prec_assign_ref(&Rational::ZERO, 0)
    });
}

#[test]
fn test_add_rational_round() {
    let test_helper =
        |s, s_hex, y: Rational, rm, out: &str, out_hex: &str, o_out, extreme: bool| {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);

            let (sum, o) = x.clone().add_rational_round(y.clone(), rm);
            assert!(sum.is_valid());
            assert_eq!(o, o_out);

            assert_eq!(sum.to_string(), out);
            assert_eq!(to_hex_string(&sum), out_hex);

            if !extreme {
                let (sum_alt, o_alt) = x.clone().add_rational_round_val_ref(&y, rm);
                assert!(sum_alt.is_valid());
                assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
                assert_eq!(o_alt, o);

                let (sum_alt, o_alt) = x.add_rational_round_ref_val(y.clone(), rm);
                assert!(sum_alt.is_valid());
                assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
                assert_eq!(o_alt, o);

                let (sum_alt, o_alt) = x.add_rational_round_ref_ref(&y, rm);
                assert!(sum_alt.is_valid());
                assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
                assert_eq!(o_alt, o);

                let mut sum_alt = x.clone();
                let o_alt = sum_alt.add_rational_round_assign(y.clone(), rm);
                assert!(sum_alt.is_valid());
                assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
                assert_eq!(o_alt, o);

                let mut sum_alt = x.clone();
                let o_alt = sum_alt.add_rational_round_assign_ref(&y, rm);
                assert!(sum_alt.is_valid());
                assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
                assert_eq!(o_alt, o);
            }

            if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_sum, rug_o) = rug_add_rational_round(
                    &rug::Float::exact_from(&x),
                    &rug::Rational::exact_from(&y),
                    rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_sum)),
                    ComparableFloatRef(&sum)
                );
                assert_eq!(rug_o, o);
            }

            let (sum_alt, o_alt) =
                add_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
        };
    let test = |s, s_hex, t, rm, out: &str, out_hex: &str, o_out| {
        test_helper(
            s,
            s_hex,
            Rational::from_str(t).unwrap(),
            rm,
            out,
            out_hex,
            o_out,
            false,
        );
    };
    test("NaN", "NaN", "123", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", Exact, "NaN", "NaN", Equal);

    test(
        "Infinity", "Infinity", "123", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "0", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", Exact, "-0.0", "-0x0.0", Equal);

    test("0.0", "0x0.0", "123", Floor, "64.0", "0x4.0E+1#1", Less);
    test(
        "0.0",
        "0x0.0",
        "123",
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test("0.0", "0x0.0", "123", Down, "64.0", "0x4.0E+1#1", Less);
    test("0.0", "0x0.0", "123", Up, "1.3e2", "0x8.0E+1#1", Greater);
    test(
        "0.0",
        "0x0.0",
        "123",
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test("-0.0", "-0x0.0", "123", Floor, "64.0", "0x4.0E+1#1", Less);
    test(
        "-0.0",
        "-0x0.0",
        "123",
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test("-0.0", "-0x0.0", "123", Down, "64.0", "0x4.0E+1#1", Less);
    test("-0.0", "-0x0.0", "123", Up, "1.3e2", "0x8.0E+1#1", Greater);
    test(
        "-0.0",
        "-0x0.0",
        "123",
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test("123.0", "0x7b.0#7", "0", Floor, "123.0", "0x7b.0#7", Equal);
    test(
        "123.0", "0x7b.0#7", "0", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test("123.0", "0x7b.0#7", "0", Down, "123.0", "0x7b.0#7", Equal);
    test("123.0", "0x7b.0#7", "0", Up, "123.0", "0x7b.0#7", Equal);
    test(
        "123.0", "0x7b.0#7", "0", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test("123.0", "0x7b.0#7", "0", Exact, "123.0", "0x7b.0#7", Equal);

    test("0.0", "0x0.0", "1/3", Floor, "0.25", "0x0.4#1", Less);
    test("0.0", "0x0.0", "1/3", Ceiling, "0.50", "0x0.8#1", Greater);
    test("0.0", "0x0.0", "1/3", Down, "0.25", "0x0.4#1", Less);
    test("0.0", "0x0.0", "1/3", Up, "0.50", "0x0.8#1", Greater);
    test("0.0", "0x0.0", "1/3", Nearest, "0.25", "0x0.4#1", Less);

    test("-0.0", "-0x0.0", "1/3", Floor, "0.25", "0x0.4#1", Less);
    test("-0.0", "-0x0.0", "1/3", Ceiling, "0.50", "0x0.8#1", Greater);
    test("-0.0", "-0x0.0", "1/3", Down, "0.25", "0x0.4#1", Less);
    test("-0.0", "-0x0.0", "1/3", Up, "0.50", "0x0.8#1", Greater);
    test("-0.0", "-0x0.0", "1/3", Nearest, "0.25", "0x0.4#1", Less);

    test("1.0", "0x1.0#1", "2", Floor, "2.0", "0x2.0#1", Less);
    test("1.0", "0x1.0#1", "2", Ceiling, "4.0", "0x4.0#1", Greater);
    test("1.0", "0x1.0#1", "2", Down, "2.0", "0x2.0#1", Less);
    test("1.0", "0x1.0#1", "2", Up, "4.0", "0x4.0#1", Greater);
    test("1.0", "0x1.0#1", "2", Nearest, "4.0", "0x4.0#1", Greater);

    test("1.0", "0x1.0#2", "2", Floor, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Ceiling, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Down, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Up, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Nearest, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Exact, "3.0", "0x3.0#2", Equal);

    test(
        "1.0000",
        "0x1.000#10",
        "2",
        Floor,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2",
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2",
        Down,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2",
        Up,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2",
        Nearest,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "2",
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );

    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        Floor,
        "1.3320",
        "0x1.550#10",
        Less,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        Ceiling,
        "1.3340",
        "0x1.558#10",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        Down,
        "1.3320",
        "0x1.550#10",
        Less,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        Up,
        "1.3340",
        "0x1.558#10",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        Nearest,
        "1.3340",
        "0x1.558#10",
        Greater,
    );

    test(
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Floor,
        "1.3333333333333333333333333333323",
        "0x1.5555555555555555555555554#100",
        Less,
    );
    test(
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Ceiling,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Down,
        "1.3333333333333333333333333333323",
        "0x1.5555555555555555555555554#100",
        Less,
    );
    test(
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Up,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Nearest,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Floor,
        "3.4749259869231262",
        "0x3.7994bfdddaf84#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Ceiling,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Down,
        "3.4749259869231262",
        "0x3.7994bfdddaf84#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Up,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Nearest,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Floor,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Ceiling,
        "2.8082593202564601",
        "0x2.ceea1533304dc#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Down,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Up,
        "2.8082593202564601",
        "0x2.ceea1533304dc#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Nearest,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Floor,
        "-2.8082593202564601",
        "-0x2.ceea1533304dc#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Ceiling,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Down,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Up,
        "-2.8082593202564601",
        "-0x2.ceea1533304dc#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Nearest,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Floor,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Ceiling,
        "-3.4749259869231262",
        "-0x3.7994bfdddaf84#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Down,
        "-3.4749259869231262",
        "-0x3.7994bfdddaf84#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Up,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Nearest,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Less,
    );

    test("1.0", "0x1.0#1", "1/50000", Floor, "1.0", "0x1.0#1", Less);
    test(
        "1.0", "0x1.0#1", "1/50000", Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test("1.0", "0x1.0#1", "1/50000", Down, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", "1/50000", Up, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", "1/50000", Nearest, "1.0", "0x1.0#1", Less);

    test("1.0", "0x1.0#1", "-1", Floor, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", Exact, "0.0", "0x0.0", Equal);

    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        Floor,
        "1.8296e180",
        "0x7.0eE+149#10",
        Less,
    );
    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        Ceiling,
        "1.8316e180",
        "0x7.10E+149#10",
        Greater,
    );
    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        Down,
        "1.8296e180",
        "0x7.0eE+149#10",
        Less,
    );
    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        Up,
        "1.8316e180",
        "0x7.10E+149#10",
        Greater,
    );
    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        Nearest,
        "1.8316e180",
        "0x7.10E+149#10",
        Greater,
    );
    test(
        "1.8316e180",
        "0x7.10E+149#10",
        "-1/8388607",
        Nearest,
        "1.8316e180",
        "0x7.10E+149#10",
        Greater,
    );
    test("1.0", "0x1.0#1", "0", Nearest, "1.0", "0x1.0#1", Equal);
    // - o == Equal in add_rational_prec_round_assign_helper
    test("1.0", "0x1.0#1", "1", Nearest, "2.0", "0x2.0#1", Equal);
    // - o != Equal in add_rational_prec_round_assign_helper
    // - t != 0 in add_rational_prec_round_assign_helper
    // - small Natural in float_can_round
    // - err0 > prec && err > prec in float_can_round
    // - s != Limb::WIDTH first time in float_can_round
    // - n == 0 in float_can_round
    // - float_can_round in add_rational_prec_round_assign_helper
    test("1.0", "0x1.0#1", "1/3", Nearest, "1.0", "0x1.0#1", Less);
    // - large Natural in float_can_round
    // - n != 0 && tmp != 0 && tmp != mask in float_can_round
    test(
        "269104312292334.3027",
        "0xf4bfbaf113ee.4d8#57",
        "517543599148951977/6042448266342026218192",
        Nearest,
        "269104312292334.3027",
        "0xf4bfbaf113ee.4d8#57",
        Less,
    );
    // - s == Limb::WIDTH first time in float_can_round
    test(
        "1.1595752615776271305e-33",
        "0x6.055703bef650178E-28#63",
        "-2457795567751474853961645492284796573970712506001349310379799846240055987994196472114730\
        61298054082441720799/23",
        Nearest,
        "-1.0686067685875977626e106",
        "-0x1.2a31c100f5e98110E+88#63",
        Less,
    );
    // - !float_can_round in add_rational_prec_round_assign_helper
    // - n != 0 && tmp == 0 in float_can_round
    // - n <= 0 first time in float_can_round
    // - s != Limb::WIDTH second time in float_can_round
    // - n > 0 first time in float_can_round
    // - x == 0 in float_can_round
    // - x != 0 in float_can_round
    test(
        "5.8215566e33",
        "0x1.1f066E+28#20",
        "8238723/1413731881599214931",
        Nearest,
        "5.8215566e33",
        "0x1.1f066E+28#20",
        Less,
    );
    // - n != 0 && tmp != 0 && tmp == mask in float_can_round
    // - n <= 0 second time in float_can_round
    // - s != Limb::WIDTH third time in float_can_round
    test(
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        "-3183521742267703581572109801877979/35801077870645726",
        Nearest,
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        Greater,
    );
    // - s == Limb::WIDTH second time in float_can_round
    // - n > 0 second time in float_can_round
    // - x == Limb::MAX in float_can_round
    test(
        "5.35864233739103574e63",
        "0xd.06b0f9ea88b7aE+52#55",
        "-808694/11557016618486698036897716112870533564271922504817764249868611771",
        Nearest,
        "5.35864233739103574e63",
        "0xd.06b0f9ea88b7aE+52#55",
        Greater,
    );
    // - x != Limb::MAX in float_can_round
    test(
        "4.904596e10",
        "0xb.6b5eE+8#19",
        "-182971083215/47776579472461276067598384651239241670113870043",
        Nearest,
        "4.904596e10",
        "0xb.6b5eE+8#19",
        Greater,
    );
    // - s == Limb::WIDTH third time in float_can_round
    test(
        "2.51594092644911199e24",
        "0x2.14c56e507323d0E+20#55",
        "-3161/286335616056113777",
        Nearest,
        "2.51594092644911199e24",
        "0x2.14c56e507323d0E+20#55",
        Greater,
    );
    // - err0 <= prec || err <= prec in float_can_round
    test(
        "5.960464455e-8",
        "0xf.ffffffE-7#28",
        "-255/4261412864",
        Nearest,
        "-2.346641787e-10",
        "-0x1.0204182E-8#28",
        Less,
    );
    // - t == 0 in add_rational_prec_round_assign_helper
    test(
        "64.0000000",
        "0x40.00000#24",
        "-215679573337202053366254388918461596342975394375083745820919489101824/336999333319768176\
        9435117813341997219028918889483691361889027096575",
        Nearest,
        "-3.72416764e-9",
        "-0xf.fec40E-8#24",
        Greater,
    );
    // - max_exponent > i64::from(Float::MAX_EXPONENT) - 2 in add_rational_prec_round
    test_helper(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        true,
    );
    test_helper(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
        true,
    );
    // - min_exponent >= Float::MAX_EXPONENT in add_rational_prec_round
    // - both signs positive in float_rational_sum_sign
    // - float_rational_sum_sign(x, &y) && (rm == Ceiling || rm == Up || rm == Nearest) in
    //   add_rational_prec_round
    test_helper(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        true,
    );
    // - float_rational_sum_sign(x, &y) && (rm == Floor || rm == Down) in add_rational_prec_round
    test_helper(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
        true,
    );
    // - !float_rational_sum_sign(x, &y) && (rm == Ceiling || rm == Down) in add_rational_prec_round
    test_helper(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
        true,
    );
    // - signs different and x.lt_abs(y) in float_rational_sum_sign
    test_helper(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) + 1),
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
        true,
    );
    // - both signs negative in float_rational_sum_sign
    // - !float_rational_sum_sign(x, &y) && (rm == Floor || rm == Up || rm == Nearest) in
    //   add_rational_prec_round
    test_helper(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
        true,
    );
    test_helper(
        "-1.0",
        "-0x1.0#1",
        Rational::ONE + Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        Nearest,
        "0.0",
        "0x0.0",
        Less,
        true,
    );
    test_helper(
        "1.0",
        "0x1.0#1",
        Rational::NEGATIVE_ONE - Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
        true,
    );
}

#[test]
fn add_rational_round_fail() {
    assert_panic!(Float::one_prec(1).add_rational_round(Rational::from_unsigneds(1u32, 3), Exact));
    assert_panic!(
        Float::one_prec(1).add_rational_round_val_ref(&Rational::from_unsigneds(1u32, 3), Exact)
    );
    assert_panic!(
        Float::one_prec(1).add_rational_round_ref_val(Rational::from_unsigneds(1u32, 3), Exact)
    );
    assert_panic!(
        Float::one_prec(1).add_rational_round_ref_ref(&Rational::from_unsigneds(1u32, 3), Exact)
    );
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_round_assign(Rational::from_unsigneds(1u32, 3), Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_round_assign_ref(&Rational::from_unsigneds(1u32, 3), Exact)
    });
}

#[test]
fn test_add_rational_prec_round() {
    let test = |s, s_hex, t, prec, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (sum, o) = x.clone().add_rational_prec_round(y.clone(), prec, rm);
        assert!(sum.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let (sum_alt, o_alt) = x.clone().add_rational_prec_round_val_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_round_assign(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_round_assign_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = add_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) = rug_add_rational_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Rational::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", "123", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", 1, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123", 1, Exact, "NaN", "NaN", Equal);

    test(
        "Infinity", "Infinity", "123", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", 1, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", 1, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0", 1, Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "0", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, Exact, "-0.0", "-0x0.0", Equal);

    test("0.0", "0x0.0", "123", 1, Floor, "64.0", "0x4.0E+1#1", Less);
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test("0.0", "0x0.0", "123", 1, Down, "64.0", "0x4.0E+1#1", Less);
    test("0.0", "0x0.0", "123", 1, Up, "1.3e2", "0x8.0E+1#1", Greater);
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test("-0.0", "-0x0.0", "123", 1, Down, "64.0", "0x4.0E+1#1", Less);
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        Up,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test("0.0", "0x0.0", "1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test(
        "0.0", "0x0.0", "1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater,
    );
    test("0.0", "0x0.0", "1/3", 1, Down, "0.25", "0x0.4#1", Less);
    test("0.0", "0x0.0", "1/3", 1, Up, "0.50", "0x0.8#1", Greater);
    test("0.0", "0x0.0", "1/3", 1, Floor, "0.25", "0x0.4#1", Less);

    test("-0.0", "-0x0.0", "1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test(
        "-0.0", "-0x0.0", "1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater,
    );
    test("-0.0", "-0x0.0", "1/3", 1, Down, "0.25", "0x0.4#1", Less);
    test("-0.0", "-0x0.0", "1/3", 1, Up, "0.50", "0x0.8#1", Greater);
    test("-0.0", "-0x0.0", "1/3", 1, Floor, "0.25", "0x0.4#1", Less);

    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        Up,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );

    test("1.0", "0x1.0#1", "2", 1, Floor, "2.0", "0x2.0#1", Less);
    test("1.0", "0x1.0#1", "2", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("1.0", "0x1.0#1", "2", 1, Down, "2.0", "0x2.0#1", Less);
    test("1.0", "0x1.0#1", "2", 1, Up, "4.0", "0x4.0#1", Greater);
    test("1.0", "0x1.0#1", "2", 1, Nearest, "4.0", "0x4.0#1", Greater);

    test("1.0", "0x1.0#1", "2", 2, Floor, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#1", "2", 2, Ceiling, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#1", "2", 2, Down, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#1", "2", 2, Up, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#1", "2", 2, Nearest, "3.0", "0x3.0#2", Equal);
    test("1.0", "0x1.0#1", "2", 2, Exact, "3.0", "0x3.0#2", Equal);

    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        100,
        Floor,
        "1.3333333333333333333333333333323",
        "0x1.5555555555555555555555554#100",
        Less,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        100,
        Ceiling,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        100,
        Down,
        "1.3333333333333333333333333333323",
        "0x1.5555555555555555555555554#100",
        Less,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        100,
        Up,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );
    test(
        "1.0000",
        "0x1.000#10",
        "1/3",
        100,
        Nearest,
        "1.3333333333333333333333333333339",
        "0x1.5555555555555555555555556#100",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Floor,
        "3.4727",
        "0x3.79#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Ceiling,
        "3.4766",
        "0x3.7a#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Down,
        "3.4727",
        "0x3.79#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Up,
        "3.4766",
        "0x3.7a#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Nearest,
        "3.4766",
        "0x3.7a#10",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Floor,
        "2.8047",
        "0x2.ce#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Ceiling,
        "2.8086",
        "0x2.cf#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Down,
        "2.8047",
        "0x2.ce#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Up,
        "2.8086",
        "0x2.cf#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Nearest,
        "2.8086",
        "0x2.cf#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Floor,
        "-2.8086",
        "-0x2.cf#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Ceiling,
        "-2.8047",
        "-0x2.ce#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Down,
        "-2.8047",
        "-0x2.ce#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Up,
        "-2.8086",
        "-0x2.cf#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Nearest,
        "-2.8086",
        "-0x2.cf#10",
        Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Floor,
        "-3.4766",
        "-0x3.7a#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Ceiling,
        "-3.4727",
        "-0x3.79#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Down,
        "-3.4727",
        "-0x3.79#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Up,
        "-3.4766",
        "-0x3.7a#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Nearest,
        "-3.4766",
        "-0x3.7a#10",
        Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );

    test("1.0", "0x1.0#1", "-1", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", 10, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", 10, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", "-1", 10, Exact, "0.0", "0x0.0", Equal);
}

#[test]
fn add_rational_prec_round_fail() {
    assert_panic!(Float::one_prec(1).add_rational_prec_round(
        Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).add_rational_prec_round_val_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).add_rational_prec_round_ref_val(
        Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).add_rational_prec_round_ref_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_prec_round_assign(Rational::from_unsigneds(5u32, 8), 1, Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(5u32, 8), 1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_prec_round_properties_helper(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (sum, o) = x.clone().add_prec_round(y.clone(), prec, rm);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_prec_round_val_ref(&y, prec, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_prec_round_ref_val(y.clone(), prec, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_prec_round_ref_ref(&y, prec, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if !extreme {
        let (sum_alt, o_alt) = add_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sum, rug_o) = rug_add_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal && sum.is_finite() {
        assert_eq!(
            ComparableFloat(
                sum.sub_prec_round_ref_ref(&x, y.significant_bits(), Exact)
                    .0
                    .abs_negative_zero()
            ),
            ComparableFloat(y.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloat(
                sum.sub_prec_round_ref_ref(&y, x.significant_bits(), Exact)
                    .0
                    .abs_negative_zero()
            ),
            ComparableFloat(x.abs_negative_zero_ref())
        );
    }

    if !extreme {
        let r_sum = if sum.is_finite() {
            if sum.is_normal() {
                assert_eq!(sum.get_prec(), Some(prec));
            }
            let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
            Some(r_sum)
        } else {
            assert_eq!(o, Equal);
            None
        };

        match (r_sum.is_some() && *r_sum.as_ref().unwrap() >= 0u32, rm) {
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

    let (sum_alt, o_alt) = y.add_prec_round_ref_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let (sum_alt, o_alt) = x.sub_prec_round_ref_val(-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let (mut sum_alt, mut o_alt) = (-&x).sub_prec_round_val_ref(&y, prec, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    let (mut sum_alt, mut o_alt) = (-&x).add_prec_round(-&y, prec, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.add_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.add_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn add_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(|(x, y, prec, rm)| {
        add_prec_round_properties_helper(x, y, prec, rm, false);
    });

    float_float_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(|(x, y, prec, rm)| {
        add_prec_round_properties_helper(x, y, prec, rm, true);
    });

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (sum, o) = x.add_prec_round_ref_val(Float::NAN, prec, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        let (sum, o) = Float::NAN.add_prec_round_val_ref(&x, prec, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.add_prec_round_ref_val(Float::INFINITY, prec, rm),
                    (Float::INFINITY, Equal)
                );
                assert_eq!(
                    Float::INFINITY.add_prec_round_val_ref(&x, prec, rm),
                    (Float::INFINITY, Equal)
                );
            }
            if x != Float::INFINITY {
                assert_eq!(
                    x.add_prec_round_ref_val(Float::NEGATIVE_INFINITY, prec, rm),
                    (Float::NEGATIVE_INFINITY, Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.add_prec_round_val_ref(&x, prec, rm),
                    (Float::NEGATIVE_INFINITY, Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (sum, o) = x.add_prec_round_ref_val(Float::ZERO, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);

            let (sum, o) = Float::ZERO.add_prec_round_val_ref(&x, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
        if rm != Floor || !x.is_positive_zero() {
            let (sum, o) = x.add_prec_round_ref_val(Float::NEGATIVE_ZERO, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);

            let (sum, o) = Float::NEGATIVE_ZERO.add_prec_round_val_ref(&x, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
    });
}

fn add_prec_properties_helper(x: Float, y: Float, prec: u64, extreme: bool) {
    let (sum, o) = x.clone().add_prec(y.clone(), prec);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_prec_val_ref(&y, prec);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_prec_ref_val(y.clone(), prec);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_prec_ref_ref(&y, prec);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_prec_assign(y.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_prec_assign_ref(&y, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if !extreme {
        let (sum_alt, o_alt) = add_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    let (rug_sum, rug_o) = rug_add_prec(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum)
    );
    assert_eq!(rug_o, o);

    let (sum_alt, o_alt) = x.add_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if o == Equal && sum.is_finite() {
        assert_eq!(
            ComparableFloat(
                sum.sub_prec_ref_ref(&x, y.significant_bits())
                    .0
                    .abs_negative_zero()
            ),
            ComparableFloat(y.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloat(
                sum.sub_prec_ref_ref(&y, x.significant_bits())
                    .0
                    .abs_negative_zero()
            ),
            ComparableFloat(x.abs_negative_zero_ref())
        );
    }

    if sum.is_finite() {
        if sum.is_normal() {
            assert_eq!(sum.get_prec(), Some(prec));
        }
        if !extreme {
            let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        }
    }

    let (sum_alt, o_alt) = y.add_prec_ref_ref(&x, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let (sum_alt, o_alt) = x.sub_prec_ref_val(-&y, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        let (mut sum_alt, mut o_alt) = (-&x).sub_prec_val_ref(&y, prec);
        sum_alt.neg_assign();
        sum_alt.abs_negative_zero_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloatRef(&sum_alt),
            ComparableFloatRef(&sum.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (mut sum_alt, mut o_alt) = (-x).add_prec(-y, prec);
        sum_alt.neg_assign();
        sum_alt.abs_negative_zero_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloatRef(&sum_alt),
            ComparableFloatRef(&sum.abs_negative_zero())
        );
        assert_eq!(o_alt, o);
    }
}

#[test]
fn add_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        add_prec_properties_helper(x, y, prec, false);
    });

    float_float_unsigned_triple_gen_var_2().test_properties(|(x, y, prec)| {
        add_prec_properties_helper(x, y, prec, true);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (sum, o) = x.add_prec_ref_val(Float::NAN, prec);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        let (sum, o) = Float::NAN.add_prec_val_ref(&x, prec);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.add_prec_ref_val(Float::INFINITY, prec),
                    (Float::INFINITY, Equal)
                );
                assert_eq!(
                    Float::INFINITY.add_prec_val_ref(&x, prec),
                    (Float::INFINITY, Equal)
                );
            }
            if x != Float::INFINITY {
                assert_eq!(
                    x.add_prec_ref_val(Float::NEGATIVE_INFINITY, prec),
                    (Float::NEGATIVE_INFINITY, Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.add_prec_val_ref(&x, prec),
                    (Float::NEGATIVE_INFINITY, Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (sum, o) = x.add_prec_ref_val(Float::ZERO, prec);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec(prec);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);

            let (sum, o) = Float::ZERO.add_prec_val_ref(&x, prec);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec(prec);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
        let (sum, o) = x.add_prec_ref_val(Float::NEGATIVE_ZERO, prec);
        let mut sum_alt = x.clone();
        let o_alt = sum_alt.set_prec(prec);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_prec_val_ref(&x, prec);
        let mut sum_alt = x.clone();
        let o_alt = sum_alt.set_prec(prec);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_round_properties_helper(x: Float, y: Float, rm: RoundingMode, extreme: bool) {
    let (sum, o) = x.clone().add_round(y.clone(), rm);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_round_val_ref(&y, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_round_ref_val(y.clone(), rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o_alt) = x.add_round_ref_ref(&y, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if !extreme {
        let (sum_alt, o_alt) = add_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            rm,
        );
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }
    let (sum_alt, o_alt) =
        x.add_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if o == Equal && sum.is_finite() {
        assert_eq!(sum.sub_round_ref_ref(&x, Exact).0, y);
        assert_eq!(sum.sub_round_ref_ref(&y, Exact).0, x);
    }

    if !extreme {
        let r_sum = if sum.is_finite() {
            if x.is_normal() && y.is_normal() && sum.is_normal() {
                assert_eq!(
                    sum.get_prec(),
                    Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
                );
            }
            let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
            Some(r_sum)
        } else {
            assert_eq!(o, Equal);
            None
        };

        match (r_sum.is_some() && *r_sum.as_ref().unwrap() >= 0u32, rm) {
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

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sum, rug_o) =
            rug_add_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    }

    let (sum_alt, o_alt) = y.add_round_ref_ref(&x, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let (sum_alt, o_alt) = x.sub_round_ref_val(-&y, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let (mut sum_alt, mut o_alt) = (-&x).sub_round_val_ref(&y, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );

    let (mut sum_alt, mut o_alt) = (-&x).add_round(-&y, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.add_round_ref_ref(&y, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.add_round_ref_ref(&y, Exact));
    }
}

#[test]
fn add_round_properties() {
    float_float_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_29().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, true);
    });

    float_float_rounding_mode_triple_gen_var_4().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_5().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_6().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_7().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_8().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_9().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm, false);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (sum, o) = x.add_round_ref_val(Float::NAN, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        let (sum, o) = Float::NAN.add_round_val_ref(&x, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.add_round_ref_val(Float::INFINITY, rm),
                    (Float::INFINITY, Equal)
                );
                assert_eq!(
                    Float::INFINITY.add_round_val_ref(&x, rm),
                    (Float::INFINITY, Equal)
                );
            }
            if x != Float::INFINITY {
                assert_eq!(
                    x.add_round_ref_val(Float::NEGATIVE_INFINITY, rm),
                    (Float::NEGATIVE_INFINITY, Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.add_round_val_ref(&x, rm),
                    (Float::NEGATIVE_INFINITY, Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (sum, o) = x.add_round_ref_val(Float::ZERO, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
            let (sum, o) = Float::ZERO.add_round_val_ref(&x, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }
        if rm != Floor || !x.is_positive_zero() {
            let (sum, o) = x.add_round_ref_val(Float::NEGATIVE_ZERO, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
            let (sum, o) = Float::NEGATIVE_ZERO.add_round_val_ref(&x, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn add_properties_helper_1<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let sum_1 = x + y;
        let sum_2 = emulate_float_float_to_float_fn(Float::add_prec, x, y);
        assert_eq!(NiceFloat(sum_1), NiceFloat(sum_2));
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_properties_helper_2(x: Float, y: Float, extreme: bool) {
    let sum = x.clone() + y.clone();
    assert!(sum.is_valid());
    let sum_alt = x.clone() + &y;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &x + y.clone();
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &x + &y;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    x_alt += y.clone();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    x_alt += &y;
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    if !extreme {
        let sum_alt = add_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    }
    let sum_alt = x
        .add_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), Nearest)
        .0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x
        .add_prec_ref_ref(&y, max(x.significant_bits(), y.significant_bits()))
        .0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o) = x.add_round_ref_ref(&y, Nearest);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if o == Equal && sum.is_finite() {
        assert_eq!(&sum - &x, y);
        assert_eq!(&sum - &y, x);
    }

    if sum.is_finite() && x.is_normal() && y.is_normal() && sum.is_normal() {
        assert_eq!(
            sum.get_prec(),
            Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
        );
        if !extreme {
            let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
            if sum < r_sum {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if sum > r_sum {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        }
    }

    let rug_sum = rug_add(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum),
    );

    let sum_alt = &y + &x;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let sum_alt = &x - -&y;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        let sum_alt = (-(-&x - &y)).abs_negative_zero();
        assert_eq!(
            ComparableFloatRef(&sum_alt),
            ComparableFloatRef(&sum.abs_negative_zero_ref())
        );

        let sum_alt = (-(-&x + -&y)).abs_negative_zero();
        assert_eq!(
            ComparableFloatRef(&sum_alt),
            ComparableFloatRef(&sum.abs_negative_zero())
        );
    }

    // example of associativity failure: 0x1.0#1 0x2.0#1 -0x1.0#1
}

#[test]
fn add_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, true);
    });

    float_pair_gen_var_2().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    float_pair_gen_var_3().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    float_pair_gen_var_4().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    float_pair_gen_var_5().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    float_pair_gen_var_6().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    float_pair_gen_var_7().test_properties(|(x, y)| {
        add_properties_helper_2(x, y, false);
    });

    apply_fn_to_primitive_floats!(add_properties_helper_1);

    float_gen().test_properties(|x| {
        assert!((&x + Float::NAN).is_nan());
        assert!((Float::NAN + &x).is_nan());
        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(&x + Float::INFINITY, Float::INFINITY);
                assert_eq!(Float::INFINITY + &x, Float::INFINITY);
            }
            if x != Float::INFINITY {
                assert_eq!(&x + Float::NEGATIVE_INFINITY, Float::NEGATIVE_INFINITY);
                assert_eq!(Float::NEGATIVE_INFINITY + &x, Float::NEGATIVE_INFINITY);
            }
        }
        if !x.is_negative_zero() {
            assert_eq!(
                ComparableFloatRef(&(&x + Float::ZERO)),
                ComparableFloatRef(&x)
            );
            assert_eq!(
                ComparableFloatRef(&(Float::ZERO + &x)),
                ComparableFloatRef(&x)
            );
        }
        assert_eq!(
            ComparableFloatRef(&(&x + Float::NEGATIVE_ZERO)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&(Float::NEGATIVE_ZERO + &x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(ComparableFloat(&x + &x), ComparableFloat(x << 1u64));
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_rational_prec_round_properties_helper(
    x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (sum, o) = x.clone().add_rational_prec_round(y.clone(), prec, rm);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_rational_prec_round_val_ref(&y, prec, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_rational_prec_round_ref_val(y.clone(), prec, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, prec, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_rational_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if !extreme {
        let (sum_alt, o_alt) = add_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sum, rug_o) = rug_add_rational_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Rational::exact_from(&y),
            prec,
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal && sum.is_finite() {
        assert_eq!(
            ComparableFloat(
                sum.sub_rational_prec_round_ref_ref(&y, x.significant_bits(), Exact)
                    .0
                    .abs_negative_zero()
            ),
            ComparableFloat(x.abs_negative_zero_ref())
        );
        // TODO additional test
    }

    if sum.is_finite() {
        if sum.is_normal() {
            assert_eq!(sum.get_prec(), Some(prec));
        }
        if !extreme {
            let r_sum = Rational::exact_from(&x) + &y;
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
            match (r_sum >= 0u32, rm) {
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

    let (sum_alt, o_alt) = x.sub_rational_prec_round_ref_val(-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let (mut sum_alt, mut o_alt) = (-&x).sub_rational_prec_round_val_ref(&y, prec, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    let (mut sum_alt, mut o_alt) = (-&x).add_rational_prec_round(-&y, prec, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.add_rational_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.add_rational_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn add_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, prec, rm)| {
            add_rational_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );

    float_rational_unsigned_rounding_mode_quadruple_gen_var_7().test_properties_with_limit(
        20,
        |(x, y, prec, rm)| {
            add_rational_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_negative_zero() {
            let (sum, o) = x.add_rational_prec_round_ref_val(Rational::ZERO, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (sum, o) = Float::NAN.add_rational_prec_round_val_ref(&x, prec, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.add_rational_prec_round_val_ref(&x, prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.add_rational_prec_round_val_ref(&x, prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (sum, o) = Float::ZERO.add_rational_prec_round_val_ref(&x, prec, rm);
        let (sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_rational_prec_round_val_ref(&x, prec, rm);
        let (mut sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
        if x == 0u32 {
            sum_alt.neg_assign();
        }
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

fn add_rational_prec_properties_helper(x: Float, y: Rational, prec: u64, extreme: bool) {
    let (sum, o) = x.clone().add_rational_prec(y.clone(), prec);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_rational_prec_val_ref(&y, prec);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_rational_prec_ref_val(y.clone(), prec);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) = x.add_rational_prec_ref_ref(&y, prec);
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_rational_prec_assign(y.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_rational_prec_assign_ref(&y, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if !extreme {
        let (sum_alt, o_alt) = add_rational_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    let (rug_sum, rug_o) = rug_add_rational_prec(
        &rug::Float::exact_from(&x),
        &rug::Rational::exact_from(&y),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum)
    );
    assert_eq!(rug_o, o);

    if o == Equal && sum.is_finite() {
        assert_eq!(
            ComparableFloat(
                sum.sub_rational_prec_ref_ref(&y, x.significant_bits())
                    .0
                    .abs_negative_zero()
            ),
            ComparableFloat(x.abs_negative_zero_ref())
        );
        // TODO additional test
    }

    let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if sum.is_finite() {
        if sum.is_normal() {
            assert_eq!(sum.get_prec(), Some(prec));
        }
        if !extreme {
            let r_sum = Rational::exact_from(&x) + &y;
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        }
    }

    let (sum_alt, o_alt) = x.sub_rational_prec_ref_val(-&y, prec);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if (x != 0u32 && y != 0u32) || x.is_sign_positive() {
        let (mut sum_alt, mut o_alt) = (-&x).sub_rational_prec_val_ref(&y, prec);
        sum_alt.neg_assign();
        sum_alt.abs_negative_zero_assign();
        o_alt = o_alt.reverse();
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (mut sum_alt, mut o_alt) = (-x).add_rational_prec(-y, prec);
        sum_alt.neg_assign();
        sum_alt.abs_negative_zero_assign();
        o_alt = o_alt.reverse();
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn add_rational_prec_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        add_rational_prec_properties_helper(x, y, prec, false);
    });

    float_rational_unsigned_triple_gen_var_2().test_properties_with_limit(20, |(x, y, prec)| {
        add_rational_prec_properties_helper(x, y, prec, true);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if !x.is_negative_zero() {
            let (sum, o) = x.add_rational_prec_ref_val(Rational::ZERO, prec);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec(prec);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (sum, o) = Float::NAN.add_rational_prec_val_ref(&x, prec);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.add_rational_prec_val_ref(&x, prec),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.add_rational_prec_val_ref(&x, prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        let (sum, o) = Float::ZERO.add_rational_prec_val_ref(&x, prec);
        let (sum_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_rational_prec_val_ref(&x, prec);
        let (mut sum_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        if x == 0u32 {
            sum_alt.neg_assign();
        }
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_rational_round_properties_helper(x: Float, y: Rational, rm: RoundingMode, extreme: bool) {
    let (sum, o) = x.clone().add_rational_round(y.clone(), rm);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_rational_round_val_ref(&y, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o_alt) = x.add_rational_round_ref_val(y.clone(), rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o_alt) = x.add_rational_round_ref_ref(&y, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_rational_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_rational_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if !extreme {
        let (sum_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    if o == Equal && sum.is_finite() && sum != 0 {
        assert_eq!(sum.sub_rational_round_ref_ref(&y, Exact).0, x);
        // TODO additional test
    }

    if sum.is_finite() {
        if x.is_normal() && sum.is_normal() {
            assert_eq!(sum.get_prec(), Some(x.get_prec().unwrap()));
        }
        if !extreme {
            let r_sum = Rational::exact_from(&x) + &y;
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }

            match (r_sum >= 0u32, rm) {
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
        let (rug_sum, rug_o) = rug_add_rational_round(
            &rug::Float::exact_from(&x),
            &rug::Rational::exact_from(&y),
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    }

    let (sum_alt, o_alt) = x.sub_rational_round_ref_val(-&y, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let (mut sum_alt, mut o_alt) = (-&x).sub_rational_round_val_ref(&y, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );

    let (mut sum_alt, mut o_alt) = (-&x).add_rational_round(-&y, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero_ref()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.add_rational_round_ref_ref(&y, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.add_rational_round_ref_ref(&y, Exact));
    }
}

#[test]
fn add_rational_round_properties() {
    float_rational_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        add_rational_round_properties_helper(x, y, rm, false);
    });

    float_rational_rounding_mode_triple_gen_var_8().test_properties_with_limit(20, |(x, y, rm)| {
        add_rational_round_properties_helper(x, y, rm, true);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        if !x.is_negative_zero() {
            let (sum, o) = x.add_rational_round_ref_val(Rational::ZERO, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(x));
            assert_eq!(o, Equal);
        }
    });

    rational_rounding_mode_pair_gen_var_6().test_properties(|(x, rm)| {
        let (sum, o) = Float::NAN.add_rational_round_val_ref(&x, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.add_rational_round_val_ref(&x, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.add_rational_round_val_ref(&x, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        let (sum, o) = Float::ZERO.add_rational_round_val_ref(&x, rm);
        let (sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, 1, rm);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_rational_round_val_ref(&x, rm);
        let (mut sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, 1, rm);
        if x == 0u32 {
            sum_alt.neg_assign();
        }
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_rational_properties_helper(x: Float, y: Rational, extreme: bool) {
    let sum = x.clone() + y.clone();
    assert!(sum.is_valid());
    let sum_alt = x.clone() + &y;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &x + y.clone();
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &x + &y;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let sum_alt = y.clone() + x.clone();
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = y.clone() + &x;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &y + x.clone();
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &y + &x;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    x_alt += y.clone();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    x_alt += &y;
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    if !extreme {
        let sum_alt =
            add_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    }

    let sum_alt = x
        .add_rational_prec_round_ref_ref(&y, x.significant_bits(), Nearest)
        .0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x.add_rational_prec_ref_ref(&y, x.significant_bits()).0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o) = x.add_rational_round_ref_ref(&y, Nearest);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if o == Equal && sum.is_finite() && sum != 0 {
        assert_eq!(&sum - &y, x);
        // TODO additional test
    }

    if sum.is_finite() && x.is_normal() && sum.is_normal() {
        assert_eq!(sum.get_prec(), Some(x.get_prec().unwrap()));
        if !extreme {
            let r_sum = Rational::exact_from(&x) + &y;
            if sum < r_sum {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if sum > r_sum {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        }
    }

    let rug_sum = rug_add_rational(&rug::Float::exact_from(&x), &rug::Rational::from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum),
    );

    let sum_alt = &x - -&y;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if (x != 0u32 && y != 0u32) || x.is_sign_positive() {
        let sum_alt = (-(-&x - &y)).abs_negative_zero();
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let sum_alt = (-(-&x + -&y)).abs_negative_zero();
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    }
}

#[test]
fn add_rational_properties() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        add_rational_properties_helper(x, y, false);
    });

    float_rational_pair_gen_var_2().test_properties_with_limit(20, |(x, y)| {
        add_rational_properties_helper(x, y, true);
    });

    float_gen().test_properties(|x| {
        assert_eq!(
            ComparableFloatRef(&(&x + Rational::ZERO)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&(Rational::ZERO + &x)),
            ComparableFloatRef(&x)
        );
    });

    rational_gen().test_properties(|x| {
        assert!((&x + Float::NAN).is_nan());
        assert!((Float::NAN + &x).is_nan());
        assert_eq!(&x + Float::INFINITY, Float::INFINITY);
        assert_eq!(Float::INFINITY + &x, Float::INFINITY);
        assert_eq!(&x + Float::NEGATIVE_INFINITY, Float::NEGATIVE_INFINITY);
        assert_eq!(Float::NEGATIVE_INFINITY + &x, Float::NEGATIVE_INFINITY);
        let sum_alt = Float::from_rational_prec_ref(&x, 1).0;
        assert_eq!(
            ComparableFloat(&x + Float::ZERO),
            ComparableFloat(sum_alt.clone())
        );
        assert_eq!(
            ComparableFloat(Float::ZERO + &x),
            ComparableFloat(sum_alt.clone())
        );
        assert_eq!(
            ComparableFloat((&x + Float::NEGATIVE_ZERO).abs_negative_zero()),
            ComparableFloat(sum_alt.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloat((Float::NEGATIVE_ZERO + &x).abs_negative_zero()),
            ComparableFloat(sum_alt.abs_negative_zero())
        );
    });
}
