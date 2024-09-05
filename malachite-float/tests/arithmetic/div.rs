// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::arithmetic::div::{
    div_rational_prec_round_direct, div_rational_prec_round_direct_ref_ref,
    div_rational_prec_round_direct_ref_val, div_rational_prec_round_direct_val_ref,
    div_rational_prec_round_naive, div_rational_prec_round_naive_ref_ref,
    div_rational_prec_round_naive_ref_val, div_rational_prec_round_naive_val_ref,
    rational_div_float_prec_round_direct, rational_div_float_prec_round_direct_ref_ref,
    rational_div_float_prec_round_direct_ref_val, rational_div_float_prec_round_direct_val_ref,
    rational_div_float_prec_round_naive, rational_div_float_prec_round_naive_ref_ref,
    rational_div_float_prec_round_naive_ref_val, rational_div_float_prec_round_naive_val_ref,
};
use malachite_float::test_util::arithmetic::div::{
    div_prec_round_naive, rug_div, rug_div_prec, rug_div_prec_round, rug_div_rational,
    rug_div_rational_prec, rug_div_rational_prec_round, rug_div_rational_round, rug_div_round,
    rug_rational_div_float, rug_rational_div_float_prec, rug_rational_div_float_prec_round,
    rug_rational_div_float_round,
};
use malachite_float::test_util::common::{
    emulate_primitive_float_fn_2, parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_23, float_float_rounding_mode_triple_gen_var_24,
    float_float_rounding_mode_triple_gen_var_25, float_float_rounding_mode_triple_gen_var_26,
    float_float_rounding_mode_triple_gen_var_27, float_float_rounding_mode_triple_gen_var_28,
    float_float_unsigned_rounding_mode_quadruple_gen_var_4, float_float_unsigned_triple_gen_var_1,
    float_gen, float_pair_gen, float_pair_gen_var_2, float_pair_gen_var_3, float_pair_gen_var_4,
    float_pair_gen_var_8, float_pair_gen_var_9, float_rational_pair_gen,
    float_rational_rounding_mode_triple_gen_var_5, float_rational_rounding_mode_triple_gen_var_6,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_4,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_5,
    float_rational_unsigned_triple_gen_var_1, float_rounding_mode_pair_gen,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_1,
    rational_rounding_mode_pair_gen_var_6, rational_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use malachite_q::Rational;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_div() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let quotient = x.clone() / y.clone();
        assert!(quotient.is_valid());

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let quotient_alt = x.clone() / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let quotient_alt = &x / y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let quotient_alt = &x / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );

        let mut quotient_alt = x.clone();
        quotient_alt /= y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let mut quotient_alt = x.clone();
        quotient_alt /= &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_div(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&quotient),
        );

        let quotient_alt = div_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            Nearest,
        )
        .0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
    };
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "NaN", "NaN");
    test("NaN", "NaN", "-Infinity", "-Infinity", "NaN", "NaN");
    test("NaN", "NaN", "0.0", "0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "-0.0", "-0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "1.0", "0x1.0#1", "NaN", "NaN");
    test("NaN", "NaN", "-1.0", "-0x1.0#1", "NaN", "NaN");

    test("Infinity", "Infinity", "NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity", "NaN", "NaN");
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
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        "-Infinity",
        "-Infinity",
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
        "NaN",
        "NaN",
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
        "Infinity",
        "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        "Infinity",
        "Infinity",
    );

    test("0.0", "0x0.0", "NaN", "NaN", "NaN", "NaN");
    test("0.0", "0x0.0", "Infinity", "Infinity", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-Infinity", "-Infinity", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "0.0", "0x0.0", "NaN", "NaN");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "NaN", "NaN");
    test("0.0", "0x0.0", "1.0", "0x1.0#1", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-1.0", "-0x1.0#1", "-0.0", "-0x0.0");

    test("-0.0", "-0x0.0", "NaN", "NaN", "NaN", "NaN");
    test("-0.0", "-0x0.0", "Infinity", "Infinity", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "NaN", "NaN");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "NaN", "NaN");
    test("-0.0", "-0x0.0", "1.0", "0x1.0#1", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-1.0", "-0x1.0#1", "0.0", "0x0.0");

    test("123.0", "0x7b.0#7", "NaN", "NaN", "NaN", "NaN");
    test("123.0", "0x7b.0#7", "Infinity", "Infinity", "0.0", "0x0.0");
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
    );
    test("123.0", "0x7b.0#7", "0.0", "0x0.0", "Infinity", "Infinity");
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
    );
    test("123.0", "0x7b.0#7", "1.0", "0x1.0#1", "123.0", "0x7b.0#7");
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        "-123.0",
        "-0x7b.0#7",
    );

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
    test("0.0", "0x0.0", "123.0", "0x7b.0#7", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "123.0", "0x7b.0#7", "-0.0", "-0x0.0");
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        "0.0082",
        "0x0.0218#7",
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        "-0.0082",
        "-0x0.0218#7",
    );

    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "0.5", "0x0.8#1");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "0.5", "0x0.8#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#1", "0.5", "0x0.8#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#2", "0.5", "0x0.8#2");
    test("1.0", "0x1.000#10", "2.0", "0x2.00#10", "0.5", "0x0.800#10");

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
    );

    // - in div_float_significands_same_prec_lt_w
    // - increment_exp in div_float_significands_same_prec_lt_w
    // - (q0 + 2) & (mask >> 1) <= 2 in div_float_significands_same_prec_lt_w
    // - h == 0 && l < y in div_float_significands_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", "1.0", "0x1.0#1");
    // - !increment_exp in div_float_significands_same_prec_lt_w
    // - (q0 + 2) & (mask >> 1) > 2 in div_float_significands_same_prec_lt_w
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_same_prec_lt_w
    // - rm == Nearest in div_float_significands_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (quotient & shift_bit) != 0) in
    //   div_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#2", "1.5", "0x1.8#2", "0.8", "0x0.c#2");
    // - h != 0 || l >= y in div_float_significands_same_prec_lt_w
    test("1.5", "0x1.8#2", "1.0", "0x1.0#2", "1.5", "0x1.8#2");
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (quotient & shift_bit) == 0)) in
    //   div_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#3", "1.2", "0x1.4#3", "0.8", "0x0.c#3");

    // - in div_float_significands_same_prec_w
    // - increment_exp in div_float_significands_same_prec_w
    // - hi == 0 && lo < y in div_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
    );
    // - !increment_exp in div_float_significands_same_prec_w
    // - round_bit == 0 in div_float_significands_same_prec_w
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_same_prec_w
    // - rm == Nearest in div_float_significands_same_prec_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (quotient & 1) == 0)) in
    //   div_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "0.99999999999999999989",
        "0x0.fffffffffffffffe#64",
    );
    // - hi != 0 || lo >= y in div_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
    );
    // - rm == Nearest || round_bit != 0 && (sticky_bit != 0 || (quotient & 1) != 0) in
    //   div_float_significands_same_prec_w
    test(
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
    );
    // - round_bit != 0 in div_float_significands_same_prec_w
    test(
        "3.1790543009742223972e-11",
        "0x2.2f43e0add6ebd01cE-9#64",
        "7770090901.6225594673",
        "0x1cf222d95.9f600ea8#64",
        "4.0913991113158902183e-21",
        "0x1.35232b1b3b9aeabeE-17#64",
    );

    // - in div_float_significands_same_prec_gt_w_lt_2w
    // - increment_exp in div_float_significands_same_prec_gt_w_lt_2w
    // - in div_float_2_approx
    // - y_1 != Limb::MAX in div_float_2_approx
    // - r_1 == 0 in div_float_2_approx
    // - (q_0.wrapping_add(21)) & (mask >> 1) <= 21 in div_float_significands_same_prec_gt_w_lt_2w
    // - s_2 == 0 && s_1 <= y_1 && (s_1 != y_1 || s_0 < y_0) in
    //   div_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
    );
    // - !increment_exp in div_float_significands_same_prec_gt_w_lt_2w
    // - s_2 > 0 || s_1 > y_1 || (s_1 == y_1 && s_0 >= y_0) in
    //   div_float_significands_same_prec_gt_w_lt_2w
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in div_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (z_0 & shift_bit) == 0)) in
    //   div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
    );
    // - r_1 != 0 in div_float_2_approx
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "0.99999999999999999989",
        "0x0.fffffffffffffffe0#65",
    );
    // - (q_0.wrapping_add(21)) & (mask >> 1) > 21 in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        "0.99999999999999999984",
        "0x0.fffffffffffffffd0#65",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && !overflow
    //   in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
    );
    // - y_1 == Limb::MAX in div_float_2_approx
    test(
        "5.29395592276605355108231857701752e-23",
        "0x4.00000007e000fffffff0000000E-19#107",
        "255.999999999999999999999947060441",
        "0xff.ffffffffffffffffffc000000#107",
        "2.06795153233048966839153112178982e-25",
        "0x4.00000007e000fffffff1000000E-21#107",
    );

    // - in div_float_significands_long_by_short
    // - diff >= 0 in div_float_significands_long_by_short
    // - in limbs_div_limb_to_out_mod_with_fraction
    // - d.get_highest_bit() in limbs_div_limb_to_out_mod_with_fraction
    // - sticky_bit != 0 || diff >= 0 || i >= abs_diff in div_float_significands_long_by_short
    // - tmp[ys_len] == 0 in div_float_significands_long_by_short
    // - tmp[ys_len] == 0 && shift != 0 in div_float_significands_long_by_short
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_long_by_short
    // - rm == Nearest in div_float_significands_long_by_short
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (ys[0] & shift_bit) != 0) in
    //   div_float_significands_long_by_short
    // - rm == Nearest && !overflow in div_float_significands_long_by_short
    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", "0.8", "0x0.c#2");
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (ys[0] & shift_bit) == 0)) in
    //   div_float_significands_long_by_short
    test("1.0", "0x1.0#1", "1.2", "0x1.4#3", "0.8", "0x0.c#3");
    // - tmp[ys_len] != 0 in div_float_significands_long_by_short
    // - tmp[ys_len] != 0 && shift != 0 in div_float_significands_long_by_short
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", "1.2", "0x1.4#3");
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_long_by_short
    test("1.5", "0x1.8#2", "1.5", "0x1.8#3", "1.0", "0x1.0#3");
    // - tmp[ys_len] == 0 && shift == 0 in div_float_significands_long_by_short
    // - c >= u - c in div_float_significands_long_by_short
    test(
        "1539239.2465826685826",
        "0x177ca7.3f200ab152a#64",
        "0.00009",
        "0x0.0006#3",
        "16812597210.673628039",
        "0x3ea1bdfda.ac72e31c#64",
    );
    // - c < u - c in div_float_significands_long_by_short
    // - round_bit == 0 in div_float_significands_long_by_short
    test(
        "1.7088961703394199635e-73",
        "0x4.d4baa70e83509ad8E-61#64",
        "1.7359472818744e-34",
        "0xe.6bf39991dcE-29#42",
        "9.844170892645193631e-40",
        "0x5.5c13c13c6d059800E-33#64",
    );
    // - tmp[ys_len] != 0 && shift == 0 in div_float_significands_long_by_short
    test(
        "4.874956728709606165589080471392071684004548689044982493122e-71",
        "0x5.6220e3ededa8be921ace72bbb95a16164a2f0abd57c49f18E-59#192",
        "1.5092483e-10",
        "0xa.5f190E-9#22",
        "3.230056172437141772802006354545046772521759341614858124236e-61",
        "0x8.4e07636cdfc96e412c1de0a522f40a5f092091c1a3aa159E-51#192",
    );

    test(
        "6.88621557179233820703925296804982406452e-28",
        "0x3.68ee78c4dbb67961d201a40495749728E-23#127",
        "0.1418399214207466117788070203268",
        "0x0.244f9effc4f1edfd85dfab3008#99",
        "4.85492060543760755133907256608679730501e-27",
        "0x1.80a57d020f8b7083401eec627a6787ccE-22#127",
    );

    // - in div_float_significands_general
    // - up[u_size - 1] == vp[vsize - 1] in div_float_significands_general
    // - k == 0 || l == 0 in div_float_significands_general
    // - up[k] == vp[l] && l != 0 in div_float_significands_general
    // - q0size < MPFR_DIV_THRESHOLD || vsize < MPFR_DIV_THRESHOLD in div_float_significands_general
    // - rm != Nearest || shift != 0 second time in div_float_significands_general
    // - qqsize > u_size in div_float_significands_general
    // - qqsize > u_size && !extra_bit in div_float_significands_general
    // - vsize >= qsize in div_float_significands_general
    // - in limbs_div_helper
    // - ds_len == 2 in limbs_div_helper
    // - qsize == q0size in div_float_significands_general
    // - vsize <= qsize in div_float_significands_general
    // - rm == Nearest second time in div_float_significands_general
    // - !goto_truncate_check_qh && !goto_sub_1_ulp && !goto_sub_1_ulp && !goto_sub_2_ulp in
    //   div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) in div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit == 0 in
    //   div_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
    );
    // - up[u_size - 1] != vp[vsize - 1] in div_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "0.99999999999999999989",
        "0x0.fffffffffffffffe0#65",
    );
    // - qqsize > u_size && extra_bit in div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky != 0 in
    //   div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky != 0 && !carry
    //   first time in div_float_significands_general
    test(
        "1.5",
        "0x1.8#2",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.49999999999999999995",
        "0x1.7fffffffffffffff#65",
    );
    // - ds_len > 1 in limbs_div_helper
    test(
        "12077.327578390390934514",
        "0x2f2d.53dc2d699afa78b8#75",
        "4.90332775049862782951473377323022738896770775e-11",
        "0x3.5e9a4013acb1890afeca956568e5bffe30edE-9#146",
        "246308796656764.923124719743308898445103382544",
        "0xe0043c546c7c.ec51e6d16d3ab81c76ba65494#146",
    );
    // - vsize < qsize in div_float_significands_general
    test(
        "1917511442.613985761391508315964935868035476119276770671",
        "0x724ae712.9d2e2bbd62dd31f140b2b9b664635f251b18c0#180",
        "3.352896739388742667241376e25",
        "0x1.bbc08f6e851e14a094c4E+21#79",
        "5.718969570663133425280005234972069245666491961744252885e-17",
        "0x4.1ef6b3c1725013efb2a8179983b542a97b0131f39a938E-14#180",
    );
    // - rm == Nearest && shift == 0 second time in div_float_significands_general
    // - qsize != q0size in div_float_significands_general
    test(
        "1.490328e-27",
        "0x7.61370E-23#20",
        "2.89262335038499315783322011549431655756e-75",
        "0x1.4ef161f7b7fc2c6cb4464f827b58b972E-62#128",
        "5.15216656741446303242577558053691246166e47",
        "0x5.a3f1d299f6f20544fbba161403f075f8E+39#128",
    );
    // - rm == Floor || rm == Down || (round_bit == 0 && sticky == 0) in
    //   div_float_significands_general
    test(
        "2.38418579101562499999949513e-7",
        "0x3.fffffffffffffffffc0000E-6#87",
        "2113929216.0",
        "0x7e000000.000000000#66",
        "1.12784561231761934265233937e-16",
        "0x8.208208208208208200000E-14#87",
    );
    // - k != 0 && l != 0 in div_float_significands_general
    // - up[k] != vp[l] first time in div_float_significands_general
    // - up[k] != vp[l] second time in div_float_significands_general
    test(
        "65535.99999999999999994",
        "0xffff.fffffffffffffc#70",
        "22835963083295358096932575511189670382427701248.00000000000000022202",
        "0x3fffffffffffffffffffffffff8000007000000.0000000000000fff8#219",
        "2.869859254937225361249367321235116718339077564583058127288930659162e-42",
        "0x3.fffffffffffffffff000000007fffff8ffffffffffe000001c00008E-35#219",
    );
    // - up[k] == vp[l] first time in div_float_significands_general
    test(
        "1.91561942608236107295e53",
        "0x2.0000000000000000E+44#66",
        "43556142965880123323311949751266331066368.000061035156249999998",
        "0x8000000000000000000000000000000000.0003fffffffffffff8#205",
        "4398046511103.99999999999999999999999999999999383702417796084544",
        "0x3ffffffffff.ffffffffffffffffffffffffffe00000000000004#205",
    );
    // - up[k] == vp[l] && l == 0 second time in div_float_significands_general
    test(
        "255.99999999813735485076904",
        "0xff.fffffff800000000000#82",
        "1.35525271559701978119405335351978053e-20",
        "0x3.ffffffffe0000000000000000000E-17#114",
        "18889465931478580854784.0",
        "0x4000000000000000000.0000000000#114",
    );
    // - q0size >= MPFR_DIV_THRESHOLD && vsize >= MPFR_DIV_THRESHOLD in
    //   div_float_significands_general
    // - u_size < n << 1 in div_float_significands_general
    // - vsize < n in div_float_significands_general
    // - in limbs_float_div_high
    // - len >= MPFR_DIVHIGH_TAB.len() in limbs_float_div_high
    // - k != 0 in limbs_float_div_high
    // - q_high != 0 in limbs_float_div_high
    // - carry == 0 in limbs_float_div_high
    // - carry != 0 in limbs_float_div_high
    // - len < MPFR_DIVHIGH_TAB.len() in limbs_float_div_high
    // - k == 0 in limbs_float_div_high
    // - len > 2 in limbs_float_div_high
    // - qh == 1 in div_float_significands_general
    // - in round_helper_2
    // - err0 > 0 in round_helper_2
    // - err0 > prec && prec < err in round_helper_2
    // - s != Limb::WIDTH in round_helper_2
    // - n != 0 && tmp != 0 && tmp != mask in round_helper_2
    // - round_helper_2 in div_float_significands_general
    // - rm == Nearest first time in div_float_significands_general
    // - rm == Nearest && shift != 0 first time in div_float_significands_general
    // - rm == Nearest && round_bit == 0 in div_float_significands_general
    test(
        "914122363545.7300954288539961362078521335512160921125366724096502748846903936195389924148\
        154757063704910629280973433563521737016298541257057972452261335506117124831272191707877190\
        2119190824963185068512647039440777212199983388696",
        "0xd4d5f05299.bae788b5e312f78f55ac79e4ca82e12494296afdb40ba21e0c21a4b3915ba2e217c389f8c9fd\
        22042f5ed70da20cfb9f1ee797b1433e077a2d34b1ae5781f975eebbcb21a32ee0c5afa5e59f8f382fe0c754a4\
        a3fb57fa4d668#754",
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
        "9.161757985214429764992710266551647359057325985892606639113002591596898046569993924851132\
        469009506849046579846739625666450546615840413944516736833310444241924771226669449467280905\
        847180462085951493210441487438964221396785151298524525494386941673630175499486324164244756\
        513337189186243674611400515366863669226025015162200893355049656715514081022497216123358900\
        204159965608607573130556648229035236124949246847315199932950877590362471534070708393335471\
        028355306786566135185894674384218244976402468322690432705335567965983855642753641740432604\
        3437409476409135277542e-112",
        "0x8.d032a1c09c5a822250facc4d03dcbdde26d4f5fe102c1e08a4f87e413b615d798e202484a718a7e4ee277\
        3677de7769fc7d817e371393d771d3460b42f92e9ba23196df3ebdff7cdda4294aecfb6c43776a893a979bdc8c\
        cac166e11d435edd52a1481ecb355a6595fcd794f14478ca886b31b8422e8bc9fdcdbc2261e6c6dfdfea3875fd\
        d48e82b6f89b37437a8064efc36e3671100bf00cb530951d17bbaefe545249991b357ff0fbc5a593a69916e391\
        e844f20336e8635a395cbda774a8ed440b65ccac5a4a48827068b6780bdeecccb424ecbcea085547d055a670dd\
        a2ce7fd1bc8ccfff3fcE-93#1859",
    );
    // - q_high == 0 in limbs_float_div_high
    test(
        "3.667390117738159207950705349477719105571949429980976394812181671883945054829509256768693\
        428777428660656941387302685172371854480204626447190818847235223777196475037450938977825246\
        002439873468885558547110470430228085143851019175355894923610792842932340338852051239754427\
        095026930245556704615627992643819617817074019244527799280182728405175259498139200084973400\
        185025632529817381341736837108765891093445296142846875524815723636775475593320258020981988\
        641285509338653295726597545939874976233498010217572353942629414771256828458910867296816672\
        522543163090158525097032013393556880192772591230747507060825723928635133507431423165786715\
        278150157159772706487785219042905040143153742214007713387626849311191420055238664770362170\
        343983987545766271318010394410223132309343859005046365250499465556871672911799580499791909\
        295656508202340485571517780009270726548788594036564495463224892161734462687808109262481368\
        096969794376769660230615691331025217443348007977074560288273633401967130359649250852147512\
        141074330343117535301250887203664127461249846867757356639750473042390297739319749311431157\
        14372183907772936544329974943881256368038028229439176970805917055179180439477409e-9",
        "0xf.c0568c485e0b826908a56a1e9eed605a795d47bbb3b22b86ff364a5aa967860d79fa907ffa4b598c74ca2\
        768fd610cc65e72d1328231f74c2896a372707f3fffd4713cd781c36ddc8c429a53c9de0a260ab39221aa6723f\
        639d4f0a18f42a39ce148ec18caa8292a2404e421cb5af96a525988ace64d3b66492e8b29b9f1982af075eac7f\
        a4c4f560684706f9c92a1babe3a7cedd233045842df3c534b90481e818908a787ba694e61d3bd3d93a45651240\
        a1926f3b818e8c51165d9c7c186dd99b0afededda17332acec6e4419ca2c498ecac62e9670b8cc359ac4ce5abb\
        e6a858a9ad732af4717655c73ab36f06357d16912bd759fba2c774b33607e2ee49fbf3328842b34b1649846034\
        e601a686e91c2040c578ab8676f4c413bc62718b75fe591900b6f10a6ee20a73c59ab3be30fb9a154c1a50b4b5\
        d60d7a76de24b93f804302eb4d625df61cf824be4c93189bd500d72fe88443b2e506a11e3b57403b447b8602ef\
        45e256c2e9cbfbc69697901d340ae418d96a38e3f87b38c8ee8b168c15df448ce29060725fff6438c91fd406bf\
        6cf95e07431942e379a50250441c4ed69a634f4e155cb67d47b7b4b285388f957b3809dcfb73606173ca9a64c8\
        9b5ee06f42fc1ee8c752cf947957f346aac01a1e21759f8267f58d36b22e7bd14E-8#3843",
        "3187923845432148642442057154569059.126715487792372839803386914179538752693826970283207166\
        811288798839227319311871148502823407877176659352907588186871022894787193533982873662011253\
        290757573915501169313819926148549862448642272469031186766746849160076154409190019980289710\
        170792165652792217117270925812431819493193080694795589891807146039351866024622601910524654\
        975993653145125921373829181052606711648254654081125875153947355721451359688668050452670532\
        460418624462017975144128989503732730892234660879379487543472739334395798501406522301770530\
        084261662015482020833397653047706275744771695945820139179975325296925632712346348118093097\
        953095934511815810581175318735500116787412839224213098543182657584610954902591533740060963\
        289805212670558460977431314581393471573332429725647583364688986461335610003995668212280028\
        807977055980202986273442266172236653427698776974320204115552560417196660880213932819325142\
        548937684752935846670101028764484218237392844524558383599287530029421881169570993841163993\
        843829902198804691520255195056203676272889080365643704609455722537324606271987166289767672\
        190663805227886932691226996255254535007618551610966568052639325048438160780381909128343538\
        211967934803057176881479842550254050201767779261994751352264395465646274141983125281497566\
        020553366225193569060382295548356106219949376044134821789228041804290511458952966410365196\
        222090758059421770693182158103609003570428820956594490269060711518240230638460085565864341\
        256289190220580928350048868798606128912317218138793827337661513849296003850300428079774414\
        62431384255329048179650372924700507846477189871631671161154559755984562472291",
        "0x9d2d417f3ca9f32fea99c6482363.20706d1bf7058f4c6275f668a177cd076adccb2fda12b6ed78a3b56bb5\
        9dfb518b8b3c05c40c48fd5544dac5cf4c4b5097a348e21623af642ca54df95b1dc69591e2bdc1e3f296461a0e\
        73545f0b1a728f095b34af1c14dc3ff040878852b81a047198ec51c9f7dcfffac0ad33017fdb2f0c43edcff12d\
        ef18336029b6f47a305e278cb4eda766445530f250be179818a2d241b5afebc21b194dbd62400042f887100725\
        62fb877debcff302fcc5b1162c1450e14478eb4e96906a31d6843172390e3cd69b3c0f474a72a62036579c22fe\
        1d1ad35fc2be49e475a1be85f30bec6d387e595070d17b17f5b5a6f400fde641d92abee13055777fe7f6b647fc\
        7850f8002fadb99332ceffb5439a87b2ac7f223b73750c6b42112fffe8b992da6c3fbc5274503b1bba48602753\
        174ba7260f73f3fa02c00fc495aad0f85c84c966f0a98fa7d85cca68b07d58e6292617f3b67fd0aafc0dc0c457\
        806b811f2698bea27de70e9ea3de0e898978b9670aa90750e88ac855daaf830c9dedb5d22968f2b01302edc889\
        ce03e2af4ec2e339258ace8efa81eeb76b273039929d7289eadfb0bae898fd0257e0f1db349eba610dfb56e3d3\
        1520f08012e02d96edfbf9a1a05ad01f682c49e1cf1e0f2b1131943ffe95afd8c6454deffe4bfdbf15fe656e18\
        13690a6dbdca197ec4c2b29ac61a6ca074a2866ff9f55184ed344bb45b2e44eca9945a21cd78ccdd427dff1dab\
        1d449dccc0aa07e37c89bb61c7fc94ce0edd5fb60b7e2d8034decb7a0e2bba4c1159236fd7f800450c1516e64c\
        bb2206f385ee11aba1c6993b2d50b2437bc23cc47f6b85d72fdd7348a5e321b5c960e8e23830fc93c4393938b8\
        98c2f16e8452c9e81ce5aa01460fb108dca1e371c53a1e72ad6ad0cb80bd5bf0ace476ab08fe8#5329",
        "1.150400792350488243006374252439239370084430198894097883408930056885079539097149903334801\
        351771948494649335504249165228317854781872084884358035626531528587565616792440130659246789\
        547068913471358728267587391921067763401960740658485999234360708658257354709138940743401019\
        478517400580149085993404074744575447821383623813098520973947955049547026992303475579228215\
        436224676800479021561310494945776720018368502752527214637080415175177143606297950367602304\
        149567232708944245857383841379350871799797376084773487408407355299732079377175164962925047\
        553551234005632068337070468847266947004802579875459884664958168707856865409967591741263680\
        896819771668163339399940221050773763868745744855354003266260565352234279551485173590767169\
        460117377689246074482291141675360319952860733464727767370984256722737643645180588444886771\
        648355387388454942423178644791668457452750839522592156007162798991977390140623536578544490\
        057707937271642210929120388296125663413585185722459999909564986042307052548228205977082023\
        238981495642981332360042089388607651948288196055583153394379775735995327224157713864077477\
        321557707540034099204193983589868016082915953745995091314702115380175700364741814725184464\
        602065018950641261052531311066491931955988616475785792821351515116629573461861957542243077\
        532642867446492701937719979200426618485741197144774966492676324343483017759514367363624027\
        675809506514516688590348320872084123672477458766804242125009602995249222201904595636534318\
        096670348004414769559053243712710972449074750435098780379781902955436286126620063025099547\
        80435463720060716005725004637793753168276780047800093479230422676842555945701e-42",
        "0x1.9a7a0a4405b5655db3032989d155cf7a58151a06aacabc4789fac720edfb0e835fe88bc9af3cc179149fe\
        616753cd76b4c7d9c17f2f47389f4e0007572679dad2a5316ede08c14af0283577f171d41d795d4ff13631def2\
        630089c6f215d7b5b8948c52ff97a4a1d9f1eb6d67b60e55478c40ffd2a7cd9684f43637e46ce3ce3e33085654\
        9165c4a377c6ab1dbb9c9b40ece8c47d94ddd1318dd2e5e57388b2e8ef80705d97c3db61d805c43cf7ff7a9a1e\
        41ded3ff033e68dc751b34ffd9cf2eae50cb7e7875b9d8f24116927cd9f609a65c71e840166cf535bbf110404d\
        bc493350b17705c0e23a9091d61f544117f70c6c6387dfb9a1dcc2f513cfbebc4cdd4b7d94c9fc57ceebebe3a2\
        e7d85b9b488b5571ef7b7c8621b770d99c67f9a19252ec5f9be4b129c7755b4a8585b97ea68e60e390c0b5c2b2\
        7b5fc3a47825c136e3b2517a6a7490ae84cf61659a9b819bfe59d45f7254dd48e028c7b694a9b9b427e60358fd\
        52afbeed855580a61e351d523d4ffaabfc7ca00e9a5b40128e9fd8b2998c189e95abc1857ff9ddf1dac904a2de\
        dfce45cbc4f1ffac50c26ec7e1135aa9ca96f6d3ac8cb3a6620a3aecb003d246eade4cf0e6394df920dfba899f\
        44ed41072e121f0402f19fc4c43c348467a07566df372a7b1af45354f2b4c7f94d52f355813e84c1a95202029c\
        0056a974e856e7c42fd6463561d1b5e02ed6a7e0ea0ca50887bd1047f4abd068ea61e2095abdad6a0cbaf91846\
        a340717aa624d6c6ba02f5d3e835ff06c742f1343479ec9a9b184eaca8e7c8be7eaf4fa322afc13f046a4a2e5f\
        4e84c723c68079991a080ac6939780e172640d568c2bc3452c14317358ee8d27a18af7c9bf2de8bea3e5b8b113\
        d8e61b810d6103e805c2a8f85b9b88f8c9129b924ba95521aa83a066991bea980c8be16f1df53E-35#5329",
    );
    // - qh != 1 in div_float_significands_general
    test(
        "5.001744775175450910666028825162941035057223155811961434576858983758571141018459147239961\
        203150588371507863692097065128597336587126035820647361489875366021800666979434518756504925\
        080057234368614339602900137940067888065298554658519361482160014564328827389146902718969285\
        655774560873251528749737416878097111467317878479458938947793439179987668446365650646689222\
        368834166803110702160567896615568919058064520266761717855703906871630752776256537972706691\
        492064397304429453321564568279795029252524047182880248127317544801138445700458013706547493\
        607273723210196209139384085476634511414334117395068648152693457778920460359930382343697178\
        078573995749595631668772420417704996567074590195305455833132484492282116574194971013126880\
        3791636230633361526548302742414616516594455084620537903358416e-16",
        "0x2.40a97302ee75111e17146bc65c8925811ce517da511093e155a5f8d319eaddbeb4108f1636a175bfa8c49\
        995045d6820b2f007a269091d024c939d8b02f4910a81e4eb38a836a327a5c12207dbd4d7a81228e55fec96493\
        eb7d51704a03ee77c5caca6616fdc0b6cbe90c676923de6ef8bf3f132b9e5e0dcbae8db3a41502b6d35629f01c\
        0834af3506639efdaa9dba6adf35a24b53b04e032ba7f9821a7155eb04aa7d235436bb878e13e2f265b7a183bd\
        7830bf484c2c6b19e1df88120105ab6ceb5f940ee7e82d4a6da4e67b7532f20750db350a532138117c02fd3f63\
        1e917747a8217c0e647adfae38491beacae6be9197fecb6a639604eba9f3e2a0e1250124f9d994d6ae0f8077c0\
        ad1f961f00f0513cb1b3b92f03fd2e19ce799415d8c26352d23ab730bff342c3d10823b5d476e3a74e5e3a1265\
        3a2e81ad38c5d7f45687a8E-13#2587",
        "1.142392802815468388118014752111991104436260746248041498551240097570984474280784266879307\
        592064853042631818930172030116326290909317377878988867978348974337550025356060840134215623\
        183687852648862683292152461337367387727519703906836027722282460995072637442171724001503892\
        471336699233392710738717656085295397789876649817787754823752786376233371866685422498954888\
        388883747226256845650864591251580129661172288008506642506027201072159168710566406994425528\
        61698637621752755004821872e-17",
        "0xd.2bbf98dfde60cfd72ff373085dca4697e7a8a2b1b6d379d3c49be918a519d5508c59f210662104e5d0b4b\
        bb4e9f09afcccb3c1655f91f2a86657e3f1315aa4e7c857d68f4d7b989d2a2f5d56a205e85ef7d6d2e9325e0fe\
        eded2158374d99d513a6d203143a26cfd251731f49e63a0e342dec62e52287bd673124d763a94038f4529cffd3\
        3599c97c0e19c589ce5603d9c26a084d360b9e7decaa7dda44ce1c27bb7c21adcb23b90d069b0a9b53b9d66094\
        d817f0420227841d34052ed2bd52e148923f8E-15#1571",
        "43.78305573046740119713641861874642911154821650761595780287653003720809501996262685108891\
        851641972710141298162279987632328545443442081070773994511258975201978303856309243168868354\
        195798884971190979692045031917290903217918542303368118161338247041052975773351929884789101\
        115569142934332750399827875003719838494787907290778719530315375971295186399328856671813524\
        401338950750393532748258170809380674066918530153006391759208813425198252649570832466781508\
        205219467712658427254330831981130973959961174357861904110964877950640959685965415405374479\
        749995867814629083616329619361738872731212580413192190669870683344353586783786957328312637\
        080558281918488071596583884466679323108552730394571978915258990045025562636051246193761815\
        9037942507064891051161378037771712070204599081778501997567779",
        "0x2b.c87657214d897953f5e5edbb169c290285fbd11622c9cf401ba99ad9f03da7ffc778df1db0d888d67c18\
        379efc8b4b36ed8cbb67da04b5b4cfdabc5f751b0a6fc68b1e3a2a16a62c4160ce4d10e00ae47020ca5d3867a7\
        2213145fe6456480971ef0cb9716c6136384fe41721979e86d1ea1bdc104f2967865add528a1367b01cc449a48\
        5786a74209d8e4c5e216fa7ae2dc897fd4926b55eacde3321f7c41bf2875c24933c8eecc7a8a26f738fd6d666b\
        678ec93b48bab7b34c5392d3ca76949dab6958fa5caaf70927d3e8b40d050bb607bc1b4fe656506e1b3e468e87\
        8b257c21e926286697a97538d3230475cd54415b8154351e72363b4b7509061108fc6ac5db47219368f3ca4011\
        5309edd7318a116c2b62a34277bfdc8a1faf656b14b6a046087cfc5dd238cd94fe91967fb6dfc52f8afa5699df\
        e2970ca40fb03c71d7d668#2587",
    );
    // - n != 0 && tmp == 0 in round_helper_2
    // - s != Limb::WIDTH first time in round_helper_2
    test(
        "0.029226865494398284939675773661541675546326681033634876986774906885711036844605915426240\
        340310096249605517533494947862661721742262334681480455706389428160732541825851560446846541\
        984468705310228771603694136853587701004882713625816107989162498255925332620064155091399923\
        221735997586925448254801064429910518067322424203818026901404115120552071966804127784634236\
        948421057038304255272147630765439482924461011451862112889343084859504616664030668233890526\
        750016895266382189553251377266295422223247821668554701473442441938933867323625955488726630\
        366895993417469747854898337776376811834753617592182604418498468334055402941851068346511528\
        636460609896950078921029074658151972716364203699936241491820871671807186599907001653993018\
        871354678119954470481273332266267481346670395677171242596948926022697465301700258863974068\
        74909984356479995046141060221761701111762543380195142151878885588292130260647185",
        "0x0.077b696f76893b930df354ab0e34b0df1508ee4503f673e22fa3b41867c5e4ffbc43b589d4cb4a00c472e\
        4046ccc9dd4a2b88b59dde14b46da030dc5a0f825fc1d9ff0213e8b046b1cd79785dd554b78e98759eae454c23\
        4fddf6ee7ae174bfc7c1ed096e905b41ce6b18511a9bfc9cfbc43c536393410fe83a634f402b0f18a446a3af90\
        9a4079394959da6918bd9094c5b587839c67f902f1f107259257f4ae96549552e41dbe7dbaddda5b9d8fa2b2bd\
        d01ba920c27d6ff6e44bd8f0ef230d60508f693680e1d769f920949bd35768a7ff10fa62210e3caf84f93cdccb\
        a5238b5e4be804a1422da22abe509c758d0cf44f202896613342ffd0fa93939f0c9bcd4de899fb72b286773da8\
        fe9cbfbd51894ec97176996bf2b6a61ac27a5f524cd408e8bca09d7cefc329a98f17616d4b48652d0a3f14cc49\
        a9bbe75a69ae9167aaa9d1951d446e95bb89c1760a549ff81f7b1d8ee454047a7d3c3e244dc499d97b256eca33\
        3d43933df1e0a046136e10#2940",
        "13755175900776476.03444104347769161034491994978466673515315430911280875941467408228432201\
        072744723926732268661761372710838010056791678637659254162102142124198282086034764229922487\
        783612307113854432839997318024745178344073274492535681224805316900558167281372795857375154\
        556654904332875058382920111153919687941523340022403648029377155100560618075774147398400587\
        446623619383420263487024723715538921428293815054071349051549046244877203394687286110159347\
        188168590858399073350484327515115795260226962105536078430163098017753828590552884767518140\
        1985404905771271934",
        "0x30de427563881c.08d120d36075efcee88657ce81cdcaedf45cd89aeca352b6e32212d20771ea31a54387b4\
        8b1eb8738ae1d31c6213ddc44bdc809d5f5b278e3449ebd13c9ab8d89ec9f0a2d87e7233cbd5128caca14e0c42\
        61e5c9ed6444b50d0cce082673e3c80b1a7102c8fc7520036bc3c6900dbcff7cecdf27ac4022bd4095736dba93\
        f47ec8ed66154c32a8eb07e14079a264e1e3370aebbfeacf3a1bbfe7aa657d9911acc70d626a35a29d86c84029\
        f97428f7cd8a3965838abf5dba9a9943b07c0ad2541156ef8e2aca1afd50c7dc55f986c835b95647701f744563\
        d15716174f2ac444#1845",
        "2.124790384741531086324618396766014790807036315098759090708607153306148130187657728807205\
        824808003798574987452867827342282934278081982848900135498971286785256137258700150407383617\
        040883394430575593960834148265479663570529896251340120502267014474793716347382140126837773\
        997589830815187190064041312015350810806968062929485379465521677249527992330225473850965169\
        559943150658138282772411679190301632849325467452565481282203194749783450341076382685241552\
        308193411477432500590221491760317417660435511777399629140824293491122934845121250984106958\
        267596015361360781611025519421253206751640221273565905944385402975185311734694681019948131\
        664198303801199498616289444621169851399879692298796098215712787193626312888177618196672010\
        550069281366814417353104090880010484400040465312380510548737422916103805415326232623198929\
        338433641210905412883566788706510371353536321443505713555119784985687877546815e-18",
        "0x2.73209f5170c5b9aaeb5a7e9e79e1dba6ba9eb57b8701701f4d2be387a03993b7e53f907a48a9029ff962b\
        4eb20e6ade6771889642b19b1985ec76b2b24fb517b27eb86681dab6bc7d5a5a203545d850396986ce5c6f9542\
        50b478a0dd27222c6c45900f2d06dad9d7f78a79b9978e3ce203479c5dce6dde3affc40e370565c038007c8bc1\
        ef1fdf0f6398b88721063c52e5eb2c4b5ba1f10d93710d5abe8aab35f5bc5cdf7031f7765dd4f9d4065b1b5b86\
        4ccd6665b73715bdfe783fae157cdc8a78e9d053cae011d4dddf28499ac3809e290ca0a221e38d2a6dd8d01980\
        c64da2f6836e0815e2ae3feb8a0d765808afcbdf6df10cf661eaf6c064ec8023cad01912101fb7e8b732b555b4\
        a053a203ab5ec17c24af5694ed7db4f67c3c76a7f360512bc9a2018d2860111211238048d21af3d79aa0904474\
        22c0d9c9883b2f3769a5fe3faeaf8bab1409329c376b70c7b54fe1393115359c5a7ff43560bc0e2548a02ffb68\
        184585e5023a6fb507d0E-15#2940",
    );
    // - rm == Nearest && round_bit != 0 in div_float_significands_general
    // - rm == Nearest && round_bit != 0 && !carry in div_float_significands_general
    test(
        "767950667289861979450190915916317600206727962344150969868.8721759327117422801652737665276\
        756590281054535919141164571740727630029753345081284383462360338144272865435150624843971065\
        632159910277471560574746205336214689461444841220676837587640834824669960199738043592424986\
        826697488691269778053087892841924823231361961899657984958299046295741089107389167540777341\
        1137893901502419455",
        "0x1f51c7714fd0115fee394111538cd8cc2697edb4db72ae0c.df46ec035af05536a25a7e2694997099b2577e\
        a12f12bb82f781cda7cd6a148cc07ab56a0bac9e90b8590cb04b95fcb27209c3c8704cb6940c7fb857c1688d50\
        6042d2fb6c58e0600ed4d86a5af398f029ebf3521880629fcd23f2bfd5f9447e8dee8310647fde5e5f5e2a0a18\
        7cdc4e8c046be95417ea73f5d4a1962ebecd092b613af810#1250",
        "51.02908107282950125513822733633990251324880237789609831750259919822628384179764854139813\
        664310170076349095466129599769105832710389521373306698912599298264052853550294941822816681\
        854313580668986120583821047373123379751687690065811623363578923931243167028825931454472450\
        957582633858214796963862561877157489833071159740156527107422181977546544800568916528954101\
        973657910504690951238460938847030239913388867386095316629182004723248011669496406544341717\
        280330814897033602543594303927085904297027340275835376721330553519116838042737905906942433\
        571685773257005175176741271374980135164819404821829229591371965448368260666618720681201228\
        891015077648242337901859343084853665103006650868094946302847691177013349096688713595881758\
        621480481903927608236275982505035171940737146501502983532266096788435188627685343250723400\
        305856753098249087347464717300786991295322157886984156263306019163631926110778355478267431\
        730519001037411438399866507817311259465545754442998333101310788661554118860610086857525556\
        649462275011688416941329847092728287810185063940836755796270353054485056127471728114260616\
        408519706805571508639638485588089026599284155688404740772260862102363921554760355678529807\
        545078380365647104062842403550988330511500383389703046031841433893387481098301402534069795\
        658730174573583314000979689827444189825731612831252242895060479770394162281620141901978731\
        360879005964038152579829187416621515149774573766217606037565254139422563209723838864340747\
        448492223949482531030183865551640816462517599780266133382572731428229045787209942986614817\
        192016158247860998226332096129248802519217902568127470669978186812858044800325333178140023\
        387809105983555698845635663825858578549915679237164757880235025950739307212793316009945989\
        415835829851915334677834156538103555704104713171341701847682567317855121655570126973890951\
        3929068030854124664713500553256",
        "0x33.0771db70bc3cc1bbfd03fee9ecfaaa1f99d76266a08107a7c922f5787496298c9bd6b5bfa13889bc0bb1\
        0f2e280f2673b20cb2191b3f747978b1483ed5890a8f1e9b4ef8665dff89aeff7e04820fcb58e76837b70b36b4\
        946ecf9ebe8fba5e510503f922f8e39500946e3ba0fd0a28c3a881101047c77426f1160e2835ecd5cdfc3c85d7\
        78adf772e0b5f5d5913cda27866ff4a68981bb0b247705d4a7a13e0cf5df9064561c207ad89d6bd10ed4faf445\
        ceca3d7f86bbdcd652aaf5c547a0071a203dca41ee8ec829aff439308e3dd8d470556949fb583c7ed1bd6c7854\
        bb629c27db1c0caa83e77e13d983d022e1865331aa5f67de9bca45976769e471933efa23a7d5fe8e03b8eed13a\
        3920db5d0f4052f811bcd1955c217ad35a8b75478eb3f2e077ecc810af955e23d57d0b957bf2104261c9f16ba6\
        a16f119f6d83e2b35b1a28b6fc7a029bcec426c495328cba2082e252a65c7267a9a83365475cc6b4672f77d481\
        40ec81e987a366445896d2ae795891105da2f608b56dca4a3e4166c6a0338423e51de87dcbfe3717817893141c\
        8b61f1377d82379374f5ad121cb9e04cf51776a20bc8b0ccaa51862efa4f51d52333818ee4877c039261bcd8dc\
        152db0a6119f3724603b4aaf9994eaf197d5adbcb723d1dc6ebdd8d2cfd37952c4128f3b79556ea134b7193dcf\
        afdc170fa41bf528ba4deac3f3d79d4407db9fd076aaca428efe74dbbc1bc7fad8b57ab1a693330f49aab1ddcc\
        f26bdc853360568f201c8fea22c816ae67afff2668debe399f951e72144cfa93dea4f18d1ee734ed2bf350fed9\
        d126c9b660f6b27ba5e13f15a8be20837e071c52d7588c0a856a969903419e91d47e7011235886759942c1c0e1\
        896e1621b2d23df869694531248722482999c8600632a5ab2279907e29cb3c38994bfbe299cb368a72ef45ecaa\
        b9646b4f1e2f37f24aa954535b1ba220c8e91dfb8f81e56dc45ec4cb3181511fa5b1854096fb3f03f2aa052eb1\
        5111548f398b2a0ffeecd95498fef2bd7f25126507f63bd3803c3a9d1aff24563f7f0baf024307e9c75#6074",
        "15049274867282653916783425765897232338286237097021544510.63055696334657926692700975852105\
        429908991325181901560777184807721684871576977542689434849596891044659963521044055457283641\
        367304991334226236960366994118987961185804836414630696056584495778598754606288486597978327\
        332370268104018122071463105903604569912790301532188931593783003198530412792016208481406460\
        177569545284296438034561831086095957417103960718034276692579871391678752019863068680376188\
        441084892942908419595289335759422879108451807263984856494740183999962091220506257869533189\
        151879060260534882539324732360323776701789524149049182180697710000483323367419829770508959\
        886298233038628402259126153349007515813222501560286947664943532169644098519601985837510236\
        220345218089637472842801890223279059211522040546360301667071765816467200381174390649332596\
        068474921226001156721359127966627401319343668619013146928630922944213877247277873392955093\
        504639915667110210542526572018725370251615388653789028473354868425502960351888594917293217\
        742712292964655900915998259809033409553076012583762301750508167459913039705278696038307105\
        198133823120123667996810391703122691279674638629684591634520584549627376733588876153831722\
        199369165407756045645488517825617351416613370266999295351664343537186993481753740631996768\
        537474147168714208012012313452364629821090786481898210882140832386720226088895885830143936\
        713915428453413001094302216345205004980013281182978168058085052927135299009881922560519790\
        174395754146412113876346433499785448301225654969985504964945207336686282652994022882395571\
        250336046288750244045997533647713495015654011612857222465008362940468446125031495141758501\
        398188163302138089732641998828146315905675208381266922364102647252983963785718802006725178\
        667871858412814374481149992996719213942377608145948042933317201168269095689729130915320304\
        63030844892155532105060912300876",
        "0x9d1f2bb817353ba61ad13135f94f65b1b52180f58a183e.a16c2e5fd6b05e4155475ec873e35d0f193a9765\
        ef45957a4681138fd789135172e7be4efd1b67c60d22430a10832c82a4dc4a53156de6d8638ce6ffe089ebf880\
        f2e1c68c90b576b5dc0b99085865ed663bd642b7743ff5500d4c6d3e2cf4977af36122c98fc49e81ee87b80d89\
        3fe81fa07bdc5986b40bdb0bf7e6bfde432dcedd2063308cf685bfee2b964ff62d434434a9518683156e532f30\
        11f2ac8f98a75178cd412e00f2261a83f952b6a94bb97c280cb51f16f85891ddd7fe6ad8030e20422da11497e5\
        efe8d88db4f96479fd0b16f3703dca8946d944979a3454bb8155d8dbdd3a765584148771967d02f798d157b6a1\
        59e10461bc83d8ec9e55b557614c35d75b391c0c9d04aefe96cab5078bd3a13d5618ca219640c68919f1fefea9\
        a3d1e47a3fcbc8c19de2210708fd96fed608648d183fd4c1177d803a49f7d276f940aeef6feaffded75f8e03ce\
        33df996eeb67ac6c0bec62d821bfce22d9a30baa6f7f4963eb4eaa91707ba1b12fd6f3e04f75cfea4dc39c6488\
        d72e86c36ba981115f42300b97a7caa427023f16c4f66213cf0c18f04cb6aa66e4830cc7040b3103e27c2e800a\
        0bce21b653566628a5bb8b0becb80b441801f31aa100fb4539cf7e4d6d68815a666c11c6cf4ac97878c551c043\
        3750e9ab6fdeb65765ae3ece107302baf12b3086988bf4d0b37206bde4041cc7c4fa79d38170719e92c052187e\
        e810ed1b2b425c081512c7ee6ea722c413215229ebaecc207fb1126644e66dea7e0139682e90f91c71b579cd86\
        b91211305fe40770c3176e35b783732c2d74c8aa1a09da66c4f34dfa1f9fd35662c5c3d1f82eeb37498b121357\
        e73ed7eea79adeab91001b3c63b1f75aa82793cd1a2b39e1bb09ecf5c6522ccc46652d831abe3ad1f9bc301df5\
        2c646068fd97c0402a29caa4ea3f4de8e5fb8a4d537d45d685f87d05d95f7ba40fbb6a919e93b44fb78b9c80ea\
        6c0a75b4dff2f73844bf4f7172907d8165f606a47821da925eda50af0ce44be22fa2b36d56e1d1698a8#6074",
    );
    // - s == Limb::WIDTH in round_helper_2
    // - n == 0 first time in round_helper_2
    test(
        "2.169542166817986625468879014599175406350620737442480370882139687492174119453066131804433\
        632496405940270655469169364935548092764516132338564210201385531365409396738163453793191332\
        174443470862749001312126324808896288354477512722829484994475343567852816045883088813362218\
        109536965926916816038009290804934132950340684582117875938116310656669280480683072639988319\
        858148103369026349100099758130382359401952836454392007054797540845974323496094771400665868\
        125503436816661354237720713751777833086011158065863046075212704099660843597053397398549954\
        8348685976904925033580113969653312280822082757679509546197165e-14",
        "0x6.1b51e41e16de577dd7b5a3a6222357b305d4e7313b1d47721ebe3d4275ef95b0d98ad997627ec7acc76f5\
        d426b7c5a9333cbc0dec080499093952901e541880379e2fdf874d1d931d1243e2b2ab072c0772ce94734ae65d\
        ff7afda628d44635b3fba75efa9bd2c10d8bdcb3a61a8b6a7697f598758d80bd808f17f8351b1761850fd75cc1\
        9f86409ac25dd27dd0ce7c26478dae9d50aff0210dc4fa18674fd87aa017255dabd141e1289a7e734e21577610\
        bf92b6ce4fe21881cc5209081e0536f0aeb1dcf6e288feeed183095b510c09e50c432ef280e742528c0c4dd6d2\
        5e65c8b6d19c28914472a930aae1ad7fac96f6442134ee95f3bd8E-12#1993",
        "301180.0045246172416425721655892719573457356766058630317841133942315022590404351104810586\
        213517139802439747677458029756555586634849124296886483237621871644459126081050822492233083\
        707945595254133438910359944302659214080172068073620764660184379004827633051943726032292014\
        225559234820452819113849827971319776547835482966443724022128469656550054145203573809984759\
        336239968661049955088445384576034864711412948863817402256055440443111246220943775270251981\
        245519547063921282094595332683031347213016637666849460902039610984880445147686118036204784\
        051476863264255027838976527798504452504598268980029728768388473210371246534136227460265249\
        86471927",
        "0x4987c.0128867b146bf9c05b0bb90d2c480c2b610c9c19a0a03f58f0d0aefa84d41a94dbc0c1206d80eab12\
        18d0f5e72e0b72a6f063fe0f604b1eedcc3760c7f60b2aa6e35735292ea939fa59fc7da94b3e86d7bbba5f8ef6\
        8136a9a4c5d98df58e4ad215fee20274cd18a324d8b66b0119d3cf93efacf51659a9814222c8f9b53fe6356392\
        e2b27f1ee07621f888214936f129248d805ae614b37cae5b83f51b2be167dc62ef96c1322204921369dc6c7475\
        c195aa735676f467be6a45d895b6b08fba56a7919ac216a6dc76cf9f5c3184a2ffa7b1bc3d8760c250d651afca\
        18aa90ff70ee4532482978816617fb02f0de87b2abd54886d1c7c16d62550d5fd8a4abb55b0c4ebb8c#2111",
        "7.203473451839519375659920800256968930150281137907207966420457324091308743561158996788387\
        290694159461823269997598092194316979651412090098764718003237064733792291895103263169730962\
        123418174759909091404064508319172962888992299461557410206033700688023479037478761668781891\
        761303156472454780198812373188616203221872171676002219543916314587725944225532571475160352\
        707938049990541748698746656039607206067984173221685967465441429896808706646646536972493098\
        282122681082474772608417991249668805473092287771115239878542545454824251859441724816281902\
        754574769925885897122660854445163455617440019293712118274988718965330199709067927675503187\
        81705947e-20",
        "0x1.542ca6851865ac89e311ac1608cac34c9fe637305345b739b624981a50028d6f60e7fd803167413e1285b\
        796e7a5ed37e1cb19125606ca9d15a697c9c497b14455aae6477ad96ffa4f216a14878a9802e8350d104f0b9d8\
        cd86ff511d7efbd74d40104b107a9d7f33d0e8894d3b157e46b7fd4e6386f823e75ae0efa9be33aac3e252d7d2\
        411f8e2afd3773f3914778d26a6b76f5569fd822db5a66db515e3cdd6699301b71cbdb73f07c24fb20d0c66059\
        fe1f236a4656c3f508e25958bdef5ca863d7950c5740d7849b46bde7e1a38b797265dedd7d4dfdaee7bcb69dce\
        887bddd7a7bbd45a6561cfad8cd840e7d95599a81bb274cc02a161439f7280459a15c9865ad5b658ed8E-16\
        #2111",
    );
    // - n != 0 && tmp != 0 && tmp == mask in round_helper_2
    // - s != Limb::WIDTH second time in round_helper_2
    test(
        "7.967842945782984973942186035477141750978306951371418866981413625233901049016799636923049\
        043510064598367854094347269064974737243308027061881036076018766384961823690368428913274793\
        537e-19",
        "0xe.b2b51b3ba9b3fa4c3c91f60bbe2f30efe9403d1c1ed1fa2688711592167dc11f579d747f20609a0e8704a\
        660072ec620d514ab31d0845381f10e96f76ac41c97c2a7b53849757dc846fdeE-16#599",
        "4.290314881292277334232122993869736164625712811477787127079554140127474408529492187148865\
        860241941174878594521708166855716060386064937593774872957730026516247807479269506509835640\
        014575725456051856772978910171974972671741113731811243859146099959299700690748160884209812\
        639693266282036645473254451466381125403186024743001502094065938286200460441378264282871237\
        445194601993083290951436490295842815606015494575193282638997440331311694865085507331318327\
        437042408938144577985735854973264189023250983346696190654635795462654945584732891184659237\
        746883228154089313101793763347136552004708447204474905674167880735273390965084667673056531\
        378593414825374458582134295243495148718875340371312206621884585775967428131940924035968587\
        526069447315202760539547246403295371510536211424729007907823710381420209210303664575050843\
        559776729235120592670693730372232402604761499044741741752247404544150954858075273585139895\
        643242778698674255451287458469387850297372316908957826221759549123594364155475346755476293\
        552962991845182999537688763267208310118481794778770137863789646906639637779481228371710954\
        980900001879766864929946144490423459237336070803276511200399191948305713293997532907864745\
        24489311444825214442690999e-14",
        "0xc.137f67f6b60895b6164f36c36d5b134858a21d493d7d49584a1811d76bd92f10b6d0aa0bea20843896e0f\
        d0d2e93957b024a1b5e7101d0f679c3dcc134107c20f0664acbfdf6bafac9013ae41ce018c62b6cf36043f13a8\
        1c35291946c79569662de17adff4ec759b1ccbe440675ef95167b0d5a5481ea6e7a6b998233e094436c8eeaefb\
        e21fa0f9c24aad8d11f378034d73a5daec0111cef1b0b8426dd5df78555318d44c992e40ad5fa98171908c4019\
        636becfe749a93747c965c11e84b68df48e887e933449d42c1ec5c2d6a7658e91f6d68333ddfde5719ca117d72\
        dadec43975eb0b6b6a076c4ada32d70b0e93250cf5e8836b11ad6a8b13a4a957de6221168782640f2313ca3716\
        3e4da0decaee000e5824d53c71d0a36a55295f8ad1c7a86eb35eab709891d1a6ac96a10448e0e307c7d6742d8d\
        0617a3e21978394d0393bc9be8e32ff2d87e85ae44c3a76ac79752bcca4927ca5dc6dcfc4db10793dc0cfc2161\
        24fdf30070db19fd8a89982adf45a408e08499b77cf25011c54cf9270bf491a2186e1a5fad26087812cc3c2446\
        ca7e5457d75f66fe9e736ad07c6b1fe4b20eaf1f073d454f371f659f7402d24e6666c8e212ddccf50c22209ca5\
        7651a266ecba0559cacf587691f7f7df3389d9968023d71b412cc20516c9b1d00f1392474c6683bd0fd6c6dc7a\
        705d88E-12#3966",
        "0.000018571697337476093645597974800042221391343383471511758884717235995391699590980821846\
        925772010524277797520625056343676345716562878283777572676679678398279369283853673423775272\
        117775978501001731220831012727542639628984101788862199591237806493805996059161201835253203\
        204357442043212398495525692960904672132359565047969002327766123188917268873022220728326927\
        266813579456518458545345939557868848624450974456390610446369728959726525392113471132021539\
        038960803550352390128253151270734177749632737865390247648212171456318006032507634424453843\
        795159031314496937836591202252458892414236797451738560115150573320872948069964155298984838\
        456270978739037104719669912968807593222214861520588395175617611807975267322946381339673265\
        479787948188151606275111200784895864164305724683464061829109671306661764726602634895903888\
        287506327660181397735839535481997625450956961420572462126703944263526095290954835871182404\
        868513321447417245068813964246287015709186652991049553947407472630595976266674750290084010\
        397575525528176465754260767775733418629588880876176812207672741703984898153224615968196909\
        775917982890995046346113085550279268258183711136206964350043642244181512435164664671221839\
        6370838653686792137638621224928",
        "0x0.00013794d52b8b1e96ced9de16a585696e655c080cbd5da8030eef302763f4138b28d7261786b8ff50bc6\
        9d0a5f06f20dad7ee2a65fae9caeeaee187ea820eae6fd4c8a673a92def1c9a165c1aeec8807ddb464eac6f550\
        6dbe6d6e3a21a035c4472d414f4887b05775ede2ad98b9b380b663c0929394c811648792ef20f0756b6bad50de\
        099fda3dd792ae5616df8837945c3cb4cd833fb9bf0db07243887c0a8fedba7030c428024be8572bca9398f563\
        b2a661574fd7faf130ac3d404dbe94b7e0ca06f440962616e1879d4f15895a10229f04969c26dbb9a1b733f734\
        fd2be1c88c7b20af178cd1d3fa116fba33a435b040155b5f5f28f0668b798810c2acb1faf0581e46cc71e9b07f\
        c9e4ebcd8a96a7d7d318d649e4468baa2ce2cdf9b1adf74f6a6b8b95a3eed5991934327ddfeb243e80db0c230e\
        d593df31dce1201e64430a27d39e6760dcf2086c1cb86bfb4e9211f18940b72d1a492a5b9109c0fdf4f5fa9fce\
        9e0ec199756ee5f8e69ba7ded6b7507facbc46df62adaa4546b3113a80e7ea40bab782194bfd006099f6a79bb8\
        19aad950497cae351fdc370756b86b3188e5c2cf71ed56fdb3683c9cc38facff80b0f2076d0f3b3a8605ca24d2\
        c8b6301601e23b50ea0940f7ba05f92ddd4a644cca6e420d6bfcd06caab9c695ba67b857bc57e1000b5935d0a8\
        79821217280#3966",
    );
    // - rm == Nearest && shift == 0 first time in div_float_significands_general
    test(
        "3.494003589335061592114850452482839587826456952956376068177291318828682713177358264760784\
        874304744009907216335963217912223351596098015455390432300579562836209635380394751634810606\
        195414389573610568068623776330707730813932586805062287543490729241818555718410383777926489\
        343323917900958772688451527795643412381235039158654803215405274640835172391547486080681691\
        827169629800140882523056966137099117587677259077996268577162491409610597231997851472454718\
        312803304389117924206548413988545367622912505139619073824279361037770537659001099972259551\
        367613373821227769095845567435994753397767332318253172120529133505790353120177308303588487\
        770407205738279213609665027613843515660027068482463985188375646552261141849026408186728824\
        80307838589616979262426550353243006148083216398932181e-12",
        "0x3.d7797e014d587584d7875beed50257a2555726437bf03fdebac1110cae2c4b64f09f9a338bf2ca8b1fcf5\
        e0128d7f387a40893706e25c04e65fdd316e3fc348d2478632d632bae209325b6c681dde52405cd7f8d9707d7f\
        5d6de0abb73e130c41c21c4537ce41381fc43788187dab4fa280fa46503f1890d663ca441f49a6a7e2b712e710\
        4c826535fdf1c8ae0282d162e3d128a982e44f67c6104624863e7f3c2c60833df521e5bab88feddd4843e4b50b\
        81ba442bc612787ad38f640412f6cff81df9793590dfa6a0debdd7f2f6de7a089fc50d597d700dbeeecfc9d795\
        ceb9a69d05db5c520717ddd7e73fabaea4e2cb06b1e1874b8b541dfca2083cb277e4d1bbefa48c0a427afea0a5\
        87cd5085c2ba28c1cad42a97be72844e851abf698ac844915e9f5ac4af406a2c354aa055f3c0994b7932d1bdb7\
        b4999768f776148E-10#2560",
        "8.388557903692744678886673027706512178020882171440574341138664910770681048759242342335520\
        689291884051853125391850946672822384185374426629671262437149651332606541726962233658521936\
        440197380293004778812876511148916284206096731251130678261803308370362817534297504354768207\
        175302739282372189357904919163400327254111204148827422042723518774290057028465296755126014\
        371014512774422430414368313070504306047237723842986944935010614200247398223867859750512432\
        528263508555538801970472032683504663202558316239308702078808690323956381650112536926354687\
        819619269837340011822483859668579241157516055938780563664184620140501025776680716962470155\
        2e-35",
        "0x6.f80c88bef08546fc8a21f0f2152ee0612eebad2635acbe0d49ce7179b387d0719cd657923976ec2796026\
        5e330a5e71c0cd8417c2cf919556130f9b353cdf2435349f846c895ede372648bccd9c217f1bb7c3e4197c1806\
        c4744c8a05ddf4f67946a4972f056d84028e7023d956a95b2ff067c093002267f7015fecb9ca5ed8f58dde48d7\
        4510e965bfa6478370f4e71e5a240dabdd9a4d6e000d4af93eea8510c6c4e095290bce062925fd9a7245caff37\
        8b7be01d3b94b56154cbeb98c26f78338a98e416fa9acc3bd12c4953b058efdcdbe56335f952208a15166babaa\
        698da808f96df97655d3f5cdb4768e6370755a01515d4ad54f625432fc742e9121b7cce4fdb08E-29#2088",
        "41652017300815899948547.94831484736938823572318695739742204735159366375620848030675947597\
        937407214848121136435440017047965685656179645007771422864559798518659544278806178160023554\
        020245599674536568425190916908552958650848910192845046055088066614557330847090320384899468\
        575304634050066348271499359839369427201019441174516439045175255034723132436872248790761539\
        506937527488424041602486298663492699846695028977559705152038731125314966620233281505474337\
        677248672173837029868825260085661764008715204976773612281854954844288452423998027475082981\
        779489389492756600188312999400831430516184077432877044587246391185262444911853744883006186\
        684049728461939709785134255700251964580618924591175050653894151676243677123400686218961075\
        24208104943935667403367286485079704207117845193523456",
        "0x8d1f5db9d3f145a7603.f2c4c307c343da5b63ef331aa97f5e951921921a937336258bc4ab65fdf9d715d36\
        ef6755e61dd29859283e35c618271ec076a196c3ddb06ce536bafe52ad10a521ebfdcda2a3839fce6eadd33d87\
        eba1d25c5eacfa66f0af4f1ce568be4792717319611eb807fe7fc0d855f2cf1b099f908a269208b3ee36d33e71\
        3912e0557515bf16566f8cc4c8c45fd6bb2ced1b3d3f27c9b272c6e5dfaacdd66335f658951d70cd7b3190aac8\
        b90d7e564b5c0ac68a04f4681a552c50de11c466e3ac1230d426fdc851e7d5705e73d7ad30a82c2febb82c46b4\
        93762b8d7c80e514c1fe29a64d4189fc176b72bb816f1223676b93d38dc33a2fd578eaf5fa512468b21e723d6c\
        d5595dac5bfd84c94e4826fc5b9aff74dec22c3cb43d7970a1359eb2642295a920a70da20a166db400602f0f4f\
        2aee9255f2251c#2560",
    );
    // - !round_helper_2 in div_float_significands_general
    test(
        "3.999999999999999999999999999999999999999999999999999999999999999999999999999447285212473\
        955543975273480780774427448575976676077991358482977909210124597604668289823519777773553500\
        1249731874464215297923136674027554116062077582682832144200801849365234375",
        "0x3.ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000#2090",
        "3.944304526105059027058642826413931148366032175545115023851394653320312500000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000038180485e-31",
        "0x8.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000001ffffffeE-26#2567",
        "10141204801825835211973625643007.99999999999999999999999999999999999999999999859870153567\
        518292907627041671008386871973805812348422824293171611020891731413939851336181163787841796\
        874999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        99999999999999999999999999999999999999999999999018341217",
        "0x7fffffffffffffffffffffffff.fffffffffffffffffffffffffffffffffffff7ffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffe0000002#2567",
    );
}

#[test]
fn test_div_prec() {
    let test = |s, s_hex, t, t_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (quotient, o) = x.clone().div_prec(y.clone(), prec);
        assert!(quotient.is_valid());

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);
        assert_eq!(o, o_out);

        let (quotient_alt, o_alt) = x.clone().div_prec_val_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = x.div_prec_ref_val(y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = x.div_prec_ref_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_prec_assign(y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_prec_assign_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = div_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient),
        );
        assert_eq!(o_alt, o);

        let (rug_quotient, rug_o) = rug_div_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
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
    test("NaN", "NaN", "1.0", "0x1.0#1", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);

    test("Infinity", "Infinity", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, "NaN", "NaN", Equal,
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
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
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
        "NaN",
        "NaN",
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
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        "Infinity",
        "Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "1.0", "0x1.0#1", 1, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, "-0.0", "-0x0.0", Equal,
    );

    test("-0.0", "-0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", 1, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, "0.0", "0x0.0", Equal,
    );

    test("123.0", "0x7b.0#7", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        1,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
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
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, "-0.0", "-0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", 1, "0.008", "0x0.02#1", Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        "0.5",
        "0x0.8#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        "0.5",
        "0x0.800#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.4502",
        "0x0.734#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "0.4502",
        "0x0.734#10",
        Greater,
    );

    // - rm == Nearest && overflow in div_float_significands_long_by_short
    test(
        "5.8208294e-27",
        "0x1.cd2c72E-22#24",
        "5.322295e17",
        "0x7.62dbe8E+14#24",
        3,
        "1.1e-44",
        "0x4.0E-37#3",
        Greater,
    );
    // - diff < 0 in div_float_significands_long_by_short
    // - sticky_bit == 0 && diff < 0 && i < abs_diff in div_float_significands_long_by_short
    // - xs[i] != 0 first time in div_float_significands_long_by_short
    test(
        "2.4914040842493675536005152793625253098043524808533216867315977e-8",
        "0x6.b014710df6d8d0fb1901206ed24e1e002b4411ac77d2348fd2E-7#202",
        "0.38945825655685",
        "0x0.63b3894b11a0#45",
        74,
        "6.3971017234954763411416e-8",
        "0x1.12c0e0961c12c850368E-6#74",
        Less,
    );
    // - xs[i] == 0 first time in div_float_significands_long_by_short
    test(
        "5.94670436124321717863912904768573447358",
        "0x5.f25b378e852b0522e85279dd6e9b5e20#129",
        "5.93e6",
        "0x5.a8E+5#9",
        5,
        "1.0e-6",
        "0x0.000011#5",
        Greater,
    );

    // - qqsize <= u_size in div_float_significands_general
    // - qqsize <= u_size && !extra_bit in div_float_significands_general
    // - ds_len == 1 in limbs_div_helper
    // - vsize > qsize in div_float_significands_general
    // - inex != 0 in div_float_significands_general
    // - rm == Nearest third time in div_float_significands_general
    // - sticky_3 <= 1 in div_float_significands_general
    // - qh == 0 in div_float_significands_general first time
    // - !qh2 first time in div_float_significands_general
    // - cmp_s_r != Equal in div_float_significands_general
    // - cmp_s_r <= Equal first time in div_float_significands_general
    test(
        "4.171457951045116318706366624151444947334895358793933e-7",
        "0x6.ffa0a6f6450242ee750c35f12c7b17a438109694bfE-6#171",
        "6.7041531299542248604361e-9",
        "0x1.ccb4b589b3974d3c8a8E-7#75",
        63,
        "62.2219968754442595",
        "0x3e.38d4c987d8e7580#63",
        Greater,
    );
    // - cmp_s_r > Equal first time in div_float_significands_general
    // - !qh2 second time in div_float_significands_general
    // - low_u == 0 second time in div_float_significands_general
    // - cmp_s_r != Equal || low_u == 0 in div_float_significands_general
    // - cmp_s_r <= Equal second time in div_float_significands_general
    // - sticky_3 != 1 && round_bit == 0 in div_float_significands_general
    // - cmp_s_r != Equal || shift != 0 in div_float_significands_general
    // - rm == Nearest || ((rm == Ceiling || rm == Up) && inex != 0) in
    //   div_float_significands_general
    // - goto_truncate_check_qh || goto_sub_1_ulp || goto_sub_1_ulp || goto_sub_2_ulp in
    //   div_float_significands_general
    // - !goto_sub_2_ulp in div_float_significands_general
    // - !goto_sub_2_ulp && !goto_sub_1_ulp in div_float_significands_general
    // - qh == 0 in div_float_significands_general second time
    test(
        "3.7563361266e88",
        "0x4.b87f4dfa0E+73#36",
        "6.769173652614128677797571270436826716e-13",
        "0xb.e8909656207637d3379c02628519c4E-11#123",
        63,
        "5.549179736559369991e100",
        "0x6.57b76abe8193e56E+83#63",
        Greater,
    );
    // - sticky_3 > 1 in div_float_significands_general
    test(
        "6.4231308808e37",
        "0x3.05280069E+31#34",
        "737445296117739183341894639.41419934820825930816963962148939959402654898",
        "0x262001d9b6493f3dffcfbef.6a08f7ee9438af0b7e2168670bbe153c76748#236",
        61,
        "87099760682.70268959",
        "0x14478ce02a.b3e377#61",
        Greater,
    );
    // - cmp_s_r > Equal second time in div_float_significands_general
    // - rm == Nearest fourth time in div_float_significands_general
    // - rm == Nearest fourth time && shift == 1 in div_float_significands_general
    // - rm == Nearest fourth time && shift == 1 && round_bit == 0 in div_float_significands_general
    // - goto_sub_2_ulp in div_float_significands_general
    // - goto_sub_2_ulp || goto_sub_1_ulp in div_float_significands_general
    test(
        "6.4231308808e37",
        "0x3.05280069E+31#34",
        "737445296117739183341894639.41419934820825930816963962148939959402654898",
        "0x262001d9b6493f3dffcfbef.6a08f7ee9438af0b7e2168670bbe153c76748#236",
        63,
        "87099760682.70268956",
        "0x14478ce02a.b3e3768#63",
        Less,
    );
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky != 0 && carry
    //   first time in div_float_significands_general
    test(
        "0.00002882366272258",
        "0x0.0001e394b0518e#40",
        "4.407913996892399269446698943482826e-28",
        "0x2.2ec4fccc4a5e21c3fa6da2c36120E-23#113",
        1,
        "8.0e22",
        "0x1.0E+19#1",
        Greater,
    );
    // - qqsize <= u_size && extra_bit in div_float_significands_general
    test(
        "13863336.632654341786855779405528442674244",
        "0xd389a8.a1f5a28ba59ea1aca395f84bcc2#131",
        "88.32972592752556369746097031876493672699524930031683012093294198918",
        "0x58.5468eb1b5d957d68d5c161060f2abd3d11568e57fb44ace5b9530c#222",
        49,
        "156949.8431822285",
        "0x26515.d7daca60#49",
        Greater,
    );
    // - sticky_3 != 1 && round_bit != 0 in div_float_significands_general
    test(
        "2.60106",
        "0x2.99df#18",
        "1.12640651293023472114636230356467597616119208733434416176044102440877312604351521285e-35",
        "0xe.f8f67659254f23c6296e8bd68107bb0d6543b0d65b1f5b85639b43d42fafe22d0850E-30#273",
        62,
        "2.309165412400307267e35",
        "0x2.c7910820f8e1a98E+29#62",
        Less,
    );
    // - sticky_3 == 1 in div_float_significands_general
    test(
        "5.3925833329420346441e-59",
        "0x5.6a7cd168af6b5224E-49#65",
        "7.055235440683529923035882801220195059780252442336634700998228376328e-29",
        "0x5.96f8c4a5671f264deaaafa2151d5a576774640994697357cb67388aE-24#222",
        126,
        "7.6433782802570609426763368452677453109e-31",
        "0xf.80ab3d3f6f9720a3e3d6bc0dd473a20E-26#126",
        Greater,
    );
    // - low_u != 0 second time in div_float_significands_general
    // - l >= k in div_float_significands_general
    // - cy == 0 first time in div_float_significands_general
    // - in sub_helper
    // - len != 0 in sub_helper
    // - !extra in sub_helper
    test(
        "7.326919700506453074257659445909468362437490080995085670582700901565e-8",
        "0x1.3ab0558546a1bb0ffb3951411ea17cd193a72ecfb10c90503c09df8E-6#220",
        "6.204773818244710788233721e40",
        "0xb.65787b1d85852bbdaff0E+33#81",
        62,
        "1.1808520205784374405e-48",
        "0x1.b9cf274b9b9758d8E-40#62",
        Greater,
    );
    // - extra in sub_helper
    test(
        "8175446.9642736466252057884969624115178724187752436027351898906495229305213",
        "0x7cbf56.f6daa340a6612207973c17246d0fbeab7366f17c873b36e8a7c5e130#244",
        "2.2364028106870166695e31",
        "0x1.1a461277f5c6a045E+26#65",
        62,
        "3.655623631488002044e-25",
        "0x7.122d595c6ad6278E-21#62",
        Greater,
    );
    // - rm == Nearest fourth time && shift != 1 in div_float_significands_general
    test(
        "1.274876025e31",
        "0xa.0e9775E+25#28",
        "7.104011072486714881105976022274735719942619445087760266603169705559e-82",
        "0x5.6412fa517e8e5c9e2826903dbe9c6b4f020acbf4d07a5f83b6e4008E-68#222",
        126,
        "1.79458620199896394199805694868744557485e112",
        "0x1.dd946a676df629632baf4759d5af1090E+93#126",
        Greater,
    );
    // - l < k in div_float_significands_general
    // - cy == 0 second time in div_float_significands_general
    test(
        "1312304952.868834018993672867833994901273914152677",
        "0x4e382f38.de6be8013ae5b9256e2e3d80a6484417#159",
        "805222139786632223922.788562923013680863",
        "0x2ba6b3b5e2de25dcb2.c9df427d2e92a42#130",
        61,
        "1.629742760446910473e-12",
        "0x1.cabb579ba24a1c8E-10#61",
        Greater,
    );
    // - qh != 0 in div_float_significands_general first time
    // - qh != 0 in div_float_significands_general second time
    test(
        "4.77e-7",
        "0x8.00E-6#9",
        "7.27595761418342590332031250000000280259692864963414184745916657983226252757951518e-12",
        "0x8.000000000000000000000000000fffffffffffffffffffffffffffffffc00000000E-10#272",
        12,
        "6.554e4",
        "0x1.000E+4#12",
        Greater,
    );
    // - cmp_s_r == Equal in div_float_significands_general
    // - u_size < qqsize in div_float_significands_general
    // - !slice_test_zero in div_float_significands_general
    test(
        "7.37e19",
        "0x3.ffE+16#10",
        "5192296858534827628530496329220096.00001525877451",
        "0x10000000000000000000000000000.0000fffff000#160",
        19,
        "1.419698e-14",
        "0x3.ff000E-12#19",
        Greater,
    );
    // - cy != 0 first time in div_float_significands_general
    test(
        "68094456598293603327533905548505297620013066911.08376291606058693773517767794501897994582\
        258970805389024907002851553537297142201729190070813276436614829387252399053496817432401142\
        072590738640749961499805778301047693438889653237932045230908598957544293121509337746443019\
        818113697275492083034559948210251757918035033203320082922052180871824896122583688257790633\
        347767288424884057738898369318200284711885269916578380577251435556820233836251778387280672\
        557877138589743092466532437097578789439010809222705729003762084164876928991111619216295374\
        407477624059748769343096515734766384609704158183422827373811211581562563854555563208285030\
        789640954030131590715773225619459109078415421551825383051156907165239504581969296580217917\
        201401107075438836986951972027346920300296173028752019760961905082742144822764332949493643\
        236343403045805151576437841818592689216294090992083144539119316847195395330209116021520674\
        687172083129681053947608634984318972146599189025048336884553676307718881472973047561666876\
        170703037518590793048762200560314576630872589716790447601932356569152269838665699058277016\
        66840662018065630567707390037462589298017636098402605937122",
        "0xbed7606a24f1082b01518fdb3fb83b7bd6aaa9f.15717c89190a577b04eeec86844b1c5da7ec1d515c98354\
        34d7eaf573076ade2dd2a181b60b144c489be5eadf10a8fa23688bb3d5dc411a80de60bf15d00f4e2bb3d1dd9a\
        05d37673fc56cf152b3e89f3955fd67353655a855872202dd9bb5939503124c7c8c1c2e93073284ac47ffc59ea\
        a0e2d8533a243325fc33e1795ea157a7a42aac695237acf96951da79f695a7ba51387281685f9dafc3987d0cd5\
        f393a28b74f7077fac3ca1d127e47df74353b51a5191ef44d9d17d76e82b13992c6d3cb5330e49f88047554a32\
        6085af468cf06e8d9d64da3bef6a9dcd50d9c3d50c03f20851ad827d200fd778bb6ffade66b04462f28dd87893\
        32e49807a56a3d251a3123c64131caa8c495f18271f1511c3ebb467c8e07f5a65ce58538f1b5a0a5cffb750f2d\
        2aadcba0995b2bc3d8ec2c295ded7fa6a05959ac7560ab0a8d43761ff672a2823d42f9a3274e5d9f621920a078\
        13bf0f700cfd42309086f1a2a594e81d2cc56780a2d2146487d5c84e4a45ea287d5082ebbc00d963dfc85328fe\
        fe6406f3583a1d25b20043620066c12999c2438ae661a9ce3a7577bc39adbb7d9c6b2b95f70ab93d453deab5ba\
        879c49217ce044fb8c99aafeab2a3c2d7b81d1420f50f3124#3778",
        "5089003.986901636664002211687103226682676013278365930635943365557217778575140755071304051\
        262677914765136974076241170290188547560394627827342683941547868265780205495768503547504471\
        809839828485098007901365847412395780099805986364986150288103155890838726927248245864748917\
        545776909957932766621125809519317578017645670025405880924769796697930801127403727804288556\
        494641308619101409216398256844236435992132716137153671232504127486179270643290623882858426\
        225419756589684747143027545016939069261808018044193898155688966291933573563069322732013621\
        2807285459308838582521108293701",
        "0x4da6eb.fca595edd73c4fe606a4af64c155ce5a5a06c66381dac0cd6a8b987a7c57162437fab8f807ce8aab\
        ccbcb3f448d402e4caa52f0641d93df28bfaf5e58a6f16f39adc87b5a1dd5398bb2bbcc7d994a0201b397c4a74\
        f97fca0b917ba0bd07b9ce9d25bb5feb2db2ede12841338d892fca759aafb7971202ee42d8f016c781218a37c3\
        bc2cd12d5917a00cf157abd9f8e9ca4f5b16b4359dfe287ef904d8db4c7df271a3b90126ef56586929a4a65813\
        f3467787c4829222d7fb9cf2e133be397a765f905e56ae783ac80f559db68cd58a90a22f1ab1af1d4dcd872ca6\
        981a9ecca8670be84795798bfc4#1890",
        63,
        "1.3380704116868237389e40",
        "0x2.75285dfa208ffd00E+33#63",
        Greater,
    );
    // - cy != 0 second time in div_float_significands_general
    test(
        "5491.549122096322909336636529538856618128292880125595066488677776051102905046366315843918\
        913993420423080685689765224843845532100106275652902608353545283642619588636389226823484210\
        861629061239611056329749023362152227886065897973662117489519345928983306276603012226918628\
        850786900692391091591113247385622873420889775685843449415444179626888676565806730164439066\
        916153918074448979830160313236976037323911844599167437480714277238521850834771260598243342\
        817521177189023",
        "0x1573.8c93440537c5819a8b2239f718e330a799971b8d83971158cdd2e59617a3347f7b4fb69cbaf9e5ff87\
        522a9c5cda06c869912d81fca8e9ef0dbe259efb7b9b0a54a58449a185302363331ad127c4e9256f242984dbc8\
        bcbea4216ff87b05bc4790144521cef5ba2b27261d31a7b650ec83e23fe89b1b2f569c96581f78da0e0d4554f3\
        0f2ed4731ea3d9bfda8cb2254fa514c04681a9298a927fc2abb2b920ac830f6d5ceb4ccb7f627c3b8895ff1804\
        8c7517748878fce7a0839b14beb12#1536",
        "1.509673347429890062512836190652173917347254081193788057095565296339169922295435269039375\
        472698941288863827397739888112708775552600102260052264376059462711206952889795049703596839\
        232956098476826545419162632064629313103364676417718336633177039699753340705596727615974068\
        628610723831801827765947524025118267152138672831243078933782856373566584766564765627600767\
        096322262179300330204832084508935397859168676086969433875110241760537834330483926543135245\
        217039771663537327658657093592229424143926522163858843150047672118222822507365351980019570\
        622407757023485804508837072933740850465658604e-7",
        "0x2.88665707e5f5634041c4267ef6487dd6141defd41a214073fe6520a7bdcf0c026eeac8a1a76a6f7c7e2c0\
        73cf8d31cab2cfe226fc73bc3a686dc489877a18a6e4ecfc6970268300003802d27bf6e18d0623eb7b117d7342\
        0a73839c03f41c3d09bb282d8dedfe2fb5675ee89cf010503f7f0b26eede0bb1c37b08c5a55d42499a3d9a9175\
        c05b5b8d873a476906f4a54340f4892951e2c8ef3a29f769e86cfab6965b4f110f218061a287f47ff1b3c217cc\
        df00c65594ce0048e5bac7ab91da9e0ecbd44b6c0440d575c7f294899ff21e307008ade7d2fba1c820083431d2\
        3a0f94cf78f5937afde262a9f2aa86d36c391c0E-6#1935",
        189,
        "36375744007.4390200041449619204768129898564194090547870908",
        "0x87829ce07.70639d7017a741d4c383ed19081084c4464eff0#189",
        Less,
    );
    // - qh2 first time in div_float_significands_general
    // - qh2 second time in div_float_significands_general
    test(
        "17592186044415.99999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999",
        "0xfffffffffff.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe#619",
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
        26,
        "3.4359738e10",
        "0x8.000000E+8#26",
        Greater,
    );
    // - u_size >= qqsize in div_float_significands_general
    // - in cmp_helper
    // - xs_len >= ys_len in cmp_helper
    // - cmp == Equal && ys_len != 0 in cmp_helper
    // - extra first time in cmp_helper
    // - xs[k + ys_len] == bb in cmp_helper
    // - xs[k + ys_len] < bb in cmp_helper
    // - cmp != Equal || k == 0 in cmp_helper first time
    // - cmp != Equal || bb == 0 in cmp_helper
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
        "0.031250000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000519145120156837503\
        646385401176147001734918579024397648372858359187739423120395387040659114766108267370765089\
        588804048262656404437892026790384389944710053355139410493357036837062723311890087295336535\
        224886414974037589635945157614552911737025968561447428534517548726620712055254368601400677\
        671297010734467257376219342434577391120913670811726006378408285341591896262197466018812208\
        707818364202903001714252499366447760377357676976645014748597726032434627346717442135450023\
        055574853304203092750885653943947474937587714924946440549453028916954237373614371956358356\
        475429001114655812373726770920126725739237754419551706215855979284224297710219012046069209\
        939692719820661068941772746188111229015697420074924121315943073728508778962547216389153434\
        262911373438798885794691058258008791084816770299900324276184865853804713586768050489411191\
        664317769877562965352444256815704956578248231483831995594631665817366125592817899193355951\
        118530268969738737015352029171836561799730662727849622621896971327136981718066405555588128\
        499837506069085441884105298697942808825809875065414280125486286978200226472477679074148608\
        959361116404570999151605839083899731368423273690135641440237909114194036012915602847402345\
        640200131464172673937558930023395763819989040966673763510370208058366269042896065620543694\
        889060883071378600260819500918049255496885447711748266825241404608194240474286320016971013\
        505365433166482975272851975538460641506474893740018353758689194264153921667729046743564069\
        253579744",
        "0x0.0800000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000fffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffe000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        003fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01fffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffc000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000#6892",
        36,
        "1.62259276829e32",
        "0x8.00000000E+26#36",
        Less,
    );
    // - xs_len < ys_len in cmp_helper
    // - cmp == Equal && xs_len != 0 in cmp_helper
    // - extra second time in cmp_helper
    // - xs[xs_len] == bb in cmp_helper
    // - xs[xs_len] < bb in cmp_helper
    // - cmp != Equal || k == 0 in cmp_helper second time
    // - cmp != Equal || !extra || ys[0] & 1 == 0 in cmp_helper
    test(
        "0.000488281249999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999963663982709420587425167733195\
        304372898231735836009052811521034277852253692510619649191102380354831309742044158555819900\
        182303288606739679841769892362177959648480394191365684359543454337339931688383011417980993\
        719495486975941054744491093403718104080315380258293738327998799327152912644577698156976186\
        55095367486514165923991079335821758615173478",
        "0x0.001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000#2815",
        "2.168404344971008868014905601739883422851562499999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        99999999999997e-19",
        "0x3.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffcE-16#1232",
        36,
        "2.2517998137e15",
        "0x8.00000000E+12#36",
        Less,
    );
    // - !extra first time in cmp_helper
    // - xs[k + ys_len] > bb in cmp_helper
    test(
        "187072209578355573530071658587684226515959365500928.0000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000001101803207893244326789943155\
        018460305099242595345476423646015479873854823483513049621916060417380926502108264599637960\
        009028188448912132272078404411433183433802785019851743390002334059444086579631362687278982\
        092055516132867633211303823870165864587972287857634502648284831738260191440051776077031899\
        598142362977237015956245760229588121897813474421614887403259665611637741356303541231148645\
        802831472359942877820272387728004709359817918832294062785037876007514227493405888433177689\
        680714430224698456312336396309714864041562196381794611694655680402546092635353483367808913\
        752362293005799560093641139943406727218702402175348117018858577588516940143086501070121082\
        790860175377027",
        "0x800000000000000000000000000000000000000000.00000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000003ffffffff80000000000000000000000000000000000000000001fffffffffffffffffffffffffffff\
        0000000000000000000000003fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffff00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000001fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffff000000000000000000000000000000000000000000000000\
        000000000000000000000000000#3331",
        "4.814824860968089632639944856462318296345254120538470488099846988916397094726562500000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000043039187809582461740\
        237762479156173200033015160616639237674924575702164607097773158539128576763982320696088303\
        832038609517565761168327844792993520257347193172468143718482303439757405926333196250133060\
        288550223275967750822594171887174682861815132430042665799184768861200315909562005634908495\
        040482237984846081838841677107482858326355102804446586581213177924458440804958481615550624\
        777545362515887646344379566287174797564232997148384542907314841464505589746930096972150685\
        525970118605412491959870803444763295822500525548365368442918966063445202883350639365511116\
        639832781262353439062659724470167290957426899576854017375938862005470860735521697212635101\
        685662094078329428036605435693999242602513069760981920534839908438815346198461935722540210\
        326793005024577932413554325700374174982252819095920682430496242093539639719648496398673988\
        294181130946788869550483877630340388986962249045975498873325109426508397715202174256327738\
        115206226749346851411911082798138490772273613411588181424713572149102614119711767787285638\
        524703608044386935250150211880470005994669065127142264588906081548898113686963839840837625\
        878160925150811347647212869111896217042292003837889935947741223166647027572900535517651639\
        207495691251723099461646574220946216274869671971300965751857652439720077434006455131311793\
        590683604336422775710475248004038566299454724867611543407224462775327725726271022150051488\
        962306463251821198471593641698105320214416610382784775624895123451428368367748899615073448\
        540841180568439251922577807930504713787460146305777655304325240206357947725816143068424676\
        946204398859120790721921646336338115096532038905903264996890888572731012019093458139272007\
        836293712752099827742569952571629995231554586253215504385643489009213919397283063609836863\
        267686636840153036916740518806971074030344720455273318700117606897827915624482324081001558\
        580105852766963764590581776038099624170109326877344468594726954445140050830134037782433855\
        587285682701452954803937838579540908490074176523981670683042482665018271091095010951223690\
        5278182153271165591237853e-35",
        "0x4.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000003fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000003fffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffc0000000000000000007ffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffc000000000000000000000000000000000\
        0000000000003fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffc00000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00E-29#7548",
        94,
        "3.8853377844514581418389238136e84",
        "0x2.00000000000000000000000E+70#94",
        Greater,
    );
    // - cmp != Equal || ys_len == 0 in cmp_helper
    // - cmp == Equal && k != 0 in cmp_helper first time
    // - xs[k] > bb in cmp_helper
    test(
        "536870912.0",
        "0x20000000.000000000000000000#100",
        "618970019642690137449562112.000000000000000000000000413590306276513837413283570606",
        "0x20000000000000000000000.000000000000000000007fffffffffffffff800000000#267",
        7,
        "8.7e-19",
        "0x1.00E-15#7",
        Greater,
    );
    // - xs[k] == bb in cmp_helper
    test(
        "5.82076609134674072265625e-11",
        "0x4.0000000000000000000000000000000000000000000000000000000000E-9#234",
        "8192.000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000073621518290228626754368\
        661771449651176491350325096363488672030344706104289026372412636851722245811280577862962401\
        765246827402220719444460351741455902885984901380506690583579618039994586998414375011572379\
        779182346403549244684325279848137048396135313055840892271543868080199241336353523751067084\
        487605126715192128136637640780333317069117147264850291384068338053712809423009097565764707\
        945202720557662560964127631897773337487421543251587594809897712933202207298421785498016072\
        254449856519027840276532392122510707729637176938094908140620695527942817590009777550509094\
        454152645378859315289504747645957548190040101946887130819745338279179159631457317778420898\
        470611527072781043747280163229876864141766629003055536247648501825111792890761407396390374\
        874897179718966769929553634685717455680003107614184209129041106624130501684449541327017766\
        6757753582718",
        "0x2000.0000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffe00000000000000000000000000000000000000000fffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc\
        #3924",
        15,
        "7.1054e-15",
        "0x2.0000E-12#15",
        Greater,
    );
    // - !extra second time in cmp_helper
    // - xs[xs_len] > bb in cmp_helper
    test(
        "8388608.000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000684828866957428698438476408479242192022929376940887598\
        259032840130007525904001877047531991911904326913906546098688715629243382676607047027876451\
        719639473400356638537379034111884955325977768684942789942877048547849906527713086446752791\
        858723447410673711365769970369878574737183686864932741554880327679842509372652502021311890\
        709142794655721430723387345276067736929449744072002738257252158918229576617779883525921585\
        596123442220897772976221689427398725884900122239984724340651124172908938910231534143853336\
        754402",
        "0x800000.00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000000007ffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc000000000000000\
        0000007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffff80000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000#4496",
        "274877906944.0000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000060373564297453652118256309302784011108922184796264095\
        964856067554168050454843683714804467407509910692607381159281320687596446281451430072818608\
        057863867144256826337394425373143511933737397669081937627812007424941677129312470503091170\
        911733627368176008701133101413396404615425187244748480697366133742242352202719895177017100\
        690953740117147446309724130962650836546766701719441996132950568128342147049715911045228743\
        620410093793320012089795191308289370965558751489023393660780664527210525039278006479120686\
        667916583520393051061889574321018782852304715910223159623785353107527307680073164159574897\
        425049765870945886337964515155856670447604028696651128918360721209741264488097948768144159\
        7005421420614212549",
        "0x4000000000.0000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000fffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffc000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        #3940",
        17,
        "0.0000305176",
        "0x0.00020000#17",
        Greater,
    );
    // - rm == Nearest fourth time && shift == 1 && round_bit != 0 in div_float_significands_general
    test(
        "21778071482940061661655974875633165533184.00000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000038725919148493182\
        728180306332863518475702191920487908654877629413444163480976859648626822342770145969080575\
        425075544675393708363989923503155223180506533504920024360652705227860649920024820627696586\
        039669982633650149716545149281467950945053120039758890080803998602993819265261127978876161\
        185060505268781766584528622179744118464786493161582046377347178114187510333531866422621209\
        846804228328353733454965094209076361330420294172109758000406375110313580574114285388890693\
        305920075040658551907160730659163031529176823334039346417685325080367605674100555171634617\
        234928876787779313622179077907389499960227122096343446066535190397344011264079006874008414\
        382439268898793195608363864632580908988059168802931770976116933654316402639901396184475943\
        729788724295947748963417392515678664943418616270097079670860835720917411673092090040881885\
        21564006805419921875",
        "0x4000000000000000000000000000000000.0000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000ffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffe00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000#3999",
        "10384593794440907712397259839635455.99999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999993016492510700454214505644339937412396352916507799605382898196356288572513\
        255335843325740991172311658773153798524263030870893776870186113616631478977505034859608131\
        351745273180872140851736147478994985324507666557307045519001087646931454316452554197971038\
        325854489646439254746059314551440312981763083969256523175727926415817004690047870256478545\
        703194379897622730743058132509274625798800775561678711877861189187434830350273902228109246\
        439341677189967895547076747086150572427772646706738189826894090172821355170927596420947406\
        00086516265683600055",
        "0x20000003fffffffffffffffffffff.fffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc#2748",
        63,
        "2097151.9843750001164",
        "0x1fffff.fc000000800#63",
        Greater,
    );
    // - cmp == Equal && k != 0 in cmp_helper second time
    // - extra third time in cmp_helper
    // - bb == 0 in cmp_helper
    // - bb != 0 in cmp_helper
    test(
        "8191.999999999999999999999999999999605569547389494097294135717358606885163396782445488497\
        614860534667968750000000000000000000000000000000000000000000000000000000009104419837890877\
        372181354144852214156953700603760262322108787027839695294275046757",
        "0x1fff.fffffffffffffffffffffffff800000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000007ffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffffffffffff#809",
        "1.13686837721616029739379882812499995e-13",
        "0x1.ffffffffffffffffffffffffffff8E-11#114",
        12,
        "7.206e16",
        "0x1.000E+14#12",
        Less,
    );
    // - !extra third time in cmp_helper
    test(
        "1220323748666959369108273.2342529296875",
        "0x10269e20a74472df07b31.3bf8000000000000000000000000000#203",
        "7627023429168496056926.707714",
        "0x19d7636772071e31a5e.b52cc#93",
        3,
        "1.6e2",
        "0xa.0E+1#3",
        Equal,
    );
    // - inex == 0 in div_float_significands_general
    test(
        "37825066419214082102958195114240.0",
        "0x1dd6b619c4a053851a451b28100.0#108",
        "35597.514868846018999",
        "0x8b0d.83ce71d761800#66",
        41,
        "1.0625760410123e27",
        "0x3.6ef13a9ad6E+22#41",
        Equal,
    );
    // - slice_test_zero in div_float_significands_general
    test(
        "1.7093142438003562047006422345280738002737985244107760125e-18",
        "0x1.f88021f87c2effbffdfffffe1004000fbfdfffc00080000E-15#187",
        "5.216412786622324189132254660659649298902e-23",
        "0x3.f1003ffffffffffffc0000001fffffff8E-19#131",
        76,
        "32768.0019530653953552246",
        "0x8000.007fff000000000#76",
        Equal,
    );
    // - vsize >= n in div_float_significands_general
    test(
        "16383.875",
        "0x3fff.e000000000000000000#88",
        "73786413344884785152.0",
        "0x3fffe000000000000.000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000#2354",
        1573,
        "2.220446049250313080847263336181640625e-16",
        "0x1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000E-13#1573",
        Equal,
    );
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky == 0 in
    //   div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky == 0 && q0p[0]
    //   & Limb::power_of_2(shift) == 0 in div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky == 0 && inex <
    //   0 in div_float_significands_general
    test(
        "3.552659469104e-15",
        "0xf.fff00007f8E-13#41",
        "4.84676140167789653245e-27",
        "0x1.80000000000000000E-22#67",
        39,
        "732996567124.0",
        "0xaaaa000054.0#39",
        Less,
    );
    // - xs[k] < bb in cmp_helper
    test(
        "9444732965739290427392.000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000000007",
        "0x2000000000000000000.0000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000004#832",
        "2.646977960169688559588507814623881131410598754882812500000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000556268464626800345772558179333101016054803995115582957638331854221801108\
        703479548963570789753127755141009073606571985185530635329448182789571845492934678567064361\
        332708567592792632774560421640775880919059592137166530735473758036612778924544586718232305\
        427824501619198614662740023702411729616701208397136180761162254530729075026989491906995358\
        575857015870579521868248894534215532446934638248802017349968861414212348741850124926530061\
        137882705366115789406521732304520719972162648696165229524120554795461372143924212631388636\
        174362506256920084558049677121644286500487074477020716131861235836886832672281595198230113\
        567709366546419530594295018834136800596699897605463746431155006086897664830884143672193812\
        44610974066e-23",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000ffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000ffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00000000000000000\
        000000000000000000000000E-19#3316",
        166,
        "356811923176489970264571492362373784095686656.0",
        "0x10000000000000000000000000000000000000.00000#166",
        Less,
    );
    // - u_size >= n << 1 in div_float_significands_general
    test(
        "128577.8764868580029756801212656877698983158936282661425976800226962111142207941648763580\
        988345531602067112079580015968145327129866382559301038263207994838658330066205433266994831\
        467312285027812918907079178962008936026981323406687199299541735562087171599287887802035024\
        906243692028775515058236407279458918988174303955662670989299201833922393455224960273024665\
        835516171507144476172664636956597658975023887814926279578799575403427330090956378386637388\
        135713599495925645995132841117427501966959959182813336108896940311309853198603275957130349\
        180224933883131003754623341245570205903946496613695364321801198058339260724701640865654036\
        186860644657991116300768604961954024465425961935896721280763534922807739569795086883210509\
        579889273277174558334960325483739866345331038726073720427318649387253214131422147544043776\
        436460480773469181215055043763336182793567894315442629932401507848298532722652018799777250\
        015693272603467557015010371191540543654044197350381607085047315086344620959308640376127551\
        931466260812673044275557327198781497102634261681647151335962486321030934060949982367829468\
        020749656366004839444427964357828448988474386543697497400049219456871357814121030924016175\
        400479172517413298974822423000259842805397057515980600295281540360693708678122362364971796\
        779616611372222055466670808323170951832437930119313259384578003290031387708344288805368337\
        588807333974140623099430569336560667666750522945572113803323880761652974693642791189155133\
        946965374070768843306018991682723627881006841722563064115628027286093354777874808024620535\
        656673153645820545300742916103648617900434641706211663820400676448026510940215813847245690\
        787061247080030343563700831660998005203777247693478841377389920284883123241874488805072645\
        154615488002533716177437262647168075186850895612670638062910190731729666226316215350837312\
        425251155930035494177805868591779327099854491269372890119982152035832492671125032181675364\
        3983068068",
        "0x1f641.e06171567f1fa1ce238d5b2d095d742087c3ea45ea9119d15b04ad4fd22debcddfe61686f85e93d4b\
        77b8c6117bfe0d897686b10fb2562ff6e4d3b597d38561e37784dd68d9eefbce8d53e8394cca9a3e2dc4566d1c\
        9dc04bea4f3078e8da6ef5461f25f3bde96a287773d9c8276b6c9374c221c96a70582c7441d2e07612a8c3dd03\
        e35a43d31a9e2671a63722177907b421a95e476796ca49409bffcc43124eb7f620ccf522ed0e21b1b97a662a4a\
        5b35525007c94fe7415d10ce2fc897c6fb779527f47ad96624541900afc16557a726046c3df3e463a8d759e7c7\
        feef0e9670534db9d04b9d9ffe0466a19dfc275afe386b4af62dae0d9ed645db8afc44566a1737f4ce512e8a0d\
        04938ce2572f4040cf0d01959d07f5d5df2d20e13e12234e2e5c841a48fd2fd2bb90b8def7c213d1e639884238\
        3071b76c2f1dc091971382e76e3ca531afb15c870664c6e2640615bd8879fac86f33c0ebf5aab3d0d4b010c955\
        1bef7718a83f66989073d65be312959c7c9937aaabe885475695d9dd1902651a5cd5f2d9ce22a6048f450b946f\
        042d4f9a2d15ed6a44d25272dce6b428476ab4a30c7d0300bbdf8a8cbae64493c2f3d0ea96dbe0adc923d55fff\
        656cc9d6b580f8d0add486b744493d4f9b350e37ff03e161abf2c2fd4893f126eff0a9b7b4f139f5368f126fc3\
        52362625255424e59ab7a735c2af91f74edfe3d37697ce869643481c7dc622fd11f6214d54709f5388c25fb8e8\
        690a34e8f4636c8a0ca5813927775ec20ea47fd7f0224e27ed9ebff6f101134d49ff8c6dd63ba49afa639a248b\
        3f88841d75b76333f69d13436dc569ecefd0683730d63b9249361dc821d6ec48b599171f73d07d2d356ab1b0dc\
        9dcc7bcd835d04ecb8ed41b2352145ab7136bd22c3c41e6a588a6760c1a914f23552091fad57262a23ad399e35\
        51b0b228b6dfc03c6869855611a71ea2a1b5016273f7eed2871f29979b0d9ba2abb1b7e380f0cf87a751babe1e\
        3d654913b9404d39fa3600a7242059cfcb69ccc95cab565fb53487bda573040c9a25205b93635c890bf9a60959\
        13cffc5cdc250bd50af4696d0d8d39574875a1cdc764ff65ab0#6302",
        "0.007532733962226776718609740913688407060156761888422741155537371074310369441516668932842\
        179178987568947783738784004214845903753330652661093496965654876471036972958909346642328584\
        201643545013628367738833205156107907841411376234839693950678432472453643018318413087069746\
        828356026984992515666155286293579263416799907808093203442758385289281722930232963476563039\
        602580878024432965067810300259817455445278112730792320635207665972350444684469938601305178\
        752078424498914971701426156771409553942152089734535539717798744531200340412177539019002862\
        118652155125909377605665570522313722924827432454878041302391573666440354096007833415288205\
        656765947802568962289596330168089725205490246098452538786079372905549372857234270190725855\
        849112728556822692019699490095850190529727602819872838218231626101062980913313718469209544\
        852538752806279742668871810873531954763147922731900653003567552388163440686646259900626789\
        714304577755153280330851920000826258260164717299978188447909834297914804211039024170906197\
        475297083472857820673298493173502925728185522784910585353556804550521456290001513633298659\
        714500938359772146658459237683211430724499182734233300374504698265281053478589566729317858\
        790913972324776555314345736769665107752609864227197733165374999429056802940957299499067099\
        921050502361595526540479399993958945176147715155608250079631789775409187220084344594176902\
        12274124444296788210426683668353271294913726417315879718977231613999863580847401836166",
        "0x0.01edaa4e04695976df280a5d8036e4043050607d25086f92fec3527973e673e50d6fe26cc2a98d36ffdcf\
        64e7201043923f5dc51bce0937d27378352c24808ded1b4b5b3a72dc7bc6709f34a90a426ef3ea922afac190c2\
        8474a8ca4b8fe2f5cbcaee63764658133bf4ee8c398d367693260118ca7e11a3cf7c60021ea9a5c44e5fd2b55c\
        5399c4d2af416fb8495ab953e2fe7f8c37c09d378b6b7ef179c89219865c2e112567c3d0e9ef3604c1cd29671a\
        220859d38f610b87914bda4727d5eb49b9b7067ccd5b9c9ef57e7e59956f66be8e839ab87e7420a7b6d992b60d\
        f12bfa9df09e2ca91b91841d01a991c5a473b0d0e55e6f4fda6266333923fec6358fbe2a3c3c1e0005c483a9eb\
        a2f70261785a9d095677b794e195f2ebc2ed814ff87150cc3b9449451981dac2554d3c18cf95ed5a563a7acd99\
        5db1f27732d6f2451a3dab2f423f4b7723b8da4c3f303120fc532ea95736220939e15f80b7fd846c650cb099d8\
        a0ace134a39fe85ec810d523e13c25e0b719d1cd0977db7ff0a86625a3cb74ff315e7dc83f55296c934dc15f2d\
        4b725cbaa31d95ba6f92d9bc74295829174293ceeb74daf101051b9c789701110c3a4116a550a47c228b82da67\
        245a50e2f1d015ed256175a549250731cf4a90e7f114a6469e323d37552553f60f164d64a8a921b998c466ae6c\
        631027958fb2f7fa98987fb56bdf12919d0fd1695c202c6d66beaf0d9868a53b16e283f1c970a6c9c90790af72\
        75b90192f86671e698defb34b13fbcaaee1cdfc4beec69c2c46072b7a53ab26c823b403098b7657e3ca970d43d\
        9954f49e9d5b94039a66b09f4#4752",
        1662,
        "17069217.78090363703967642537830172830995997926929566889372077304847867142361027601405442\
        286459362568854752643069912976321422774071641269497000825786673743566716666986122201673566\
        954005548156596610807551133418147862857497307376377531996417888282239877181408646433952455\
        449832546174770944921797138738845637802606582911913934926484494169960897434718640487978822\
        397032739913032542773985132071836123040945096104343234766846881246804903855742298793870626\
        77027180448793731547987433698769543012728726607028778",
        "0x10474a1.c7e94cfe69b17ceb496b55c7673d812006e20b007a03948c3a4b4cfa0e6dbd76b73772e1f6a52b9\
        30ce94a60672db80391db2eb227cad27128df746aae7c1ddb5368fdf97185927d712711fe190297ac2c169017f\
        ac401dc6bb041c5fa94ba4452b5b1c6a3d3f8d6d83892439b9e9525639739ce0fdfe4663acc923263563286994\
        f78a08edfcad551d9695555741c5301f15857227aeb725ecb09ed40468e21e6e6fe4f0f94b9e338a39e9665362\
        acb0922dc06bca5074ed2c965471e97402ed8367fe149485e0bb52e243f78#1662",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && carry in div_float_significands_general
    test(
        "2.454546732648863276547885977493137821487607756249724782555774558593552627360857928230048\
        942018665976138591785782565939256583959305212359451403757412054481726126552732229930964269\
        470905492198653519153594970703124999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        995761114627674791277020932519574469505017610728137410250159407014720169738824650046673749\
        059339474479527195633303962573451642912661719349979993453218111358455268175773121496346851\
        580476856645369527796626919786850732045813122714395701160155300736782415936562761674226693\
        028929972451809327089060718204876263534635026488050357020524924317959135992439056643999723\
        656960561739673936265216544476386688997222734833105296938067057205929214479610015975275932\
        718619380628277147130611593861916399934357066581330096178050556305086195135562253719174061\
        496425118601354010153670564319137109364713055282273218494183562236985834389291157056469689\
        9828846337814e-91",
        "0x7.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000E-76#4517",
        "0.015625000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000771048757510508716764719334683019132168263108153512725551350734409293871565\
        903519457201213361797567020084883497703807999422859208123581192925346248118136084792628172\
        988363113468168413497993606331107182531734570308778128025953654408272649725745457159283262\
        0134958821588759455527143",
        "0x0.0400000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffffffffffffff\
        fffffffffff8000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000#3655",
        2337,
        "1.570909908895272496990647025595608205752068963999823860835695717499873681510949074067231\
        322891946224728698742900842201124213733955335910048898404743714868304720993748627155817132\
        46137951500713825225830078125e-89",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000E-74#2337",
        Greater,
    );
}

#[test]
fn div_prec_fail() {
    assert_panic!(Float::NAN.div_prec(Float::NAN, 0));
    assert_panic!(Float::NAN.div_prec_val_ref(&Float::NAN, 0));
    assert_panic!(Float::NAN.div_prec_ref_val(Float::NAN, 0));
    assert_panic!(Float::NAN.div_prec_ref_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.div_prec_assign(Float::NAN, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.div_prec_assign_ref(&Float::NAN, 0)
    });
}

#[test]
fn test_div_round() {
    let test = |s, s_hex, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (quotient, o) = x.clone().div_round(y.clone(), rm);
        assert!(quotient.is_valid());

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);
        assert_eq!(o, o_out);

        let (quotient_alt, o_alt) = x.clone().div_round_val_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = x.div_round_ref_val(y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = x.div_round_ref_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_round_assign(y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_round_assign_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) =
                rug_div_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient),
            );
            assert_eq!(rug_o, o);
        }

        let (quotient_alt, o_alt) = div_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
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

    test("NaN", "NaN", "1.0", "0x1.0#1", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "1.0", "0x1.0#1", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "1.0", "0x1.0#1", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "1.0", "0x1.0#1", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "1.0", "0x1.0#1", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "1.0", "0x1.0#1", Exact, "NaN", "NaN", Equal);

    test("NaN", "NaN", "-1.0", "-0x1.0#1", Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "-1.0", "-0x1.0#1", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-1.0", "-0x1.0#1", Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", Nearest, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "-1.0", "-0x1.0#1", Exact, "NaN", "NaN", Equal);

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
        "Infinity", "Infinity", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
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
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
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
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
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
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Up,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        Up,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("0.0", "0x0.0", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Up, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal);

    test("0.0", "0x0.0", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal);

    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", Down, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "1.0", "0x1.0#1", Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", Exact, "-0.0", "-0x0.0", Equal,
    );

    test("-0.0", "-0x0.0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );

    test("-0.0", "-0x0.0", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Up, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal);

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal,
    );
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Exact, "0.0", "0x0.0", Equal,
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
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Up,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "123.0", "0x7b.0#7", "1.0", "0x1.0#1", Floor, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "1.0", "0x1.0#1", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "1.0", "0x1.0#1", Down, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "1.0", "0x1.0#1", Up, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "1.0", "0x1.0#1", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "1.0", "0x1.0#1", Exact, "123.0", "0x7b.0#7", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        Floor,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        Ceiling,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        Down,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        Up,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        Nearest,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        Exact,
        "-123.0",
        "-0x7b.0#7",
        Equal,
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
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Floor,
        "0.0081",
        "0x0.0210#7",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Down,
        "0.0081",
        "0x0.0210#7",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Up,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Nearest,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Floor,
        "-0.0082",
        "-0x0.0218#7",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "-0.0081",
        "-0x0.0210#7",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Down,
        "-0.0081",
        "-0x0.0210#7",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Up,
        "-0.0082",
        "-0x0.0218#7",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Nearest,
        "-0.0082",
        "-0x0.0218#7",
        Less,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Floor, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Ceiling, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Down, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Up, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Nearest, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Exact, "0.5", "0x0.8#1", Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "0.45015815807855303",
        "0x0.733d90a6f99884#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "0.45015815807855303",
        "0x0.733d90a6f99884#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-0.45015815807855303",
        "-0x0.733d90a6f99884#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-0.45015815807855303",
        "-0x0.733d90a6f99884#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "-0.45015815807855303",
        "-0x0.733d90a6f99884#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "-0.45015815807855303",
        "-0x0.733d90a6f99884#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "-0.45015815807855308",
        "-0x0.733d90a6f99888#53",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "0.45015815807855303",
        "0x0.733d90a6f99884#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "0.45015815807855303",
        "0x0.733d90a6f99884#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "0.45015815807855308",
        "0x0.733d90a6f99888#53",
        Greater,
    );

    // - rm == Floor || rm == Down in div_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#2", "1.5", "0x1.8#2", Down, "0.5", "0x0.8#2", Less,
    );
    // - rm == Ceiling || rm == Up in div_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#2", "1.5", "0x1.8#2", Up, "0.8", "0x0.c#2", Greater,
    );

    // - rm == Floor || rm == Down in div_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Down,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe#64",
        Less,
    );
    // - rm == Ceiling || rm == Up in div_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Up,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff#64",
        Greater,
    );

    // - rm == Floor || rm == Down in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Down,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
        Less,
    );

    // - rm == Ceiling || rm == Up in div_float_significands_same_prec_gt_w_lt_2w
    // - (rm == Ceiling || rm == Up) && !overflow in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Up,
        "0.99999999999999999997",
        "0x0.ffffffffffffffff8#65",
        Greater,
    );
    // - rm == Floor || rm == Down in div_float_significands_long_by_short
    test(
        "1.0", "0x1.0#1", "1.5", "0x1.8#2", Down, "0.5", "0x0.8#2", Less,
    );
    // - rm == Ceiling || rm == Up in div_float_significands_long_by_short
    // - (rm == Ceiling || rm == Up) && !overflow in div_float_significands_long_by_short
    test(
        "1.0", "0x1.0#1", "1.5", "0x1.8#2", Up, "0.8", "0x0.c#2", Greater,
    );

    // - rm == Floor || rm == Down || inex == 0 second time in div_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Down,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
        Less,
    );
    // - (rm == Ceiling || rm == Up) && inex != 0 second time in div_float_significands_general
    // - (rm == Up || rm == Ceiling) && (round_bit != 0 || sticky != 0) in
    //   div_float_significands_general
    // - (rm == Up || rm == Ceiling) && (round_bit != 0 || sticky != 0) && !carry in
    //   div_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Up,
        "0.99999999999999999997",
        "0x0.ffffffffffffffff8#65",
        Greater,
    );
    // - rm == Down || rm == Floor first time in div_float_significands_general
    test(
        "9601551840755394477782155306615.400809036690567306698564563582349725588158197630704697078\
        013417362910194409963799900311565761064675199511838067316509390850104862662213505749078521\
        770377437536581181743514602025941917984526635269955727174740188131870994016855234632753317\
        016792561638598147688756431699678956508985348347746587700412259161976828470219450879730366\
        977549362693268375201952814531271011740413280015272635082852480719566333921902104982178638\
        916033220231617933208782646056491566119203568817027316959068412927057431189073591651262563\
        044927867605702941287180061603188567390575475013505450984801866920347023147022407388362970\
        59088991236828388853335477454316615451511519",
        "0x7930498878f5aafc3b8512a677.669b6bc886f9e61b8a5f2697dc5a3b5a2b2256b53dad0634eac355f6a0a5\
        1716868ade4d034f6702fb8253c1f989a028a5714d4d362b3553fc85427910a52d3d77e4f39bbc7914d902c799\
        e58ce8c619156cf04fd16baabf3b0abeb8808eb23612b9dac1cebfc00ede036daaa891cfe4e827cb450c1dc08a\
        8127d7ab7af3ad2ad37537634deffc4816e65732919e1337bdf6abce20a753777e12957b9282ddb2d803496313\
        dc5a1048926798d52bfa132da701a4550ee67ff9951f347ffc2fcd57993813839688be78495c10ef533661a217\
        c12a2f3327e0d96f3ee8d9b98f05db022bcbe3a69ce3b5009d3946e0ab02a8f9c18127f132344e97387cb60c19\
        2f6949d3c05183b19bc196#2230",
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
        Down,
        "173018210334417056237548035723683.1345550599384516271435235732043979258142510481520742047\
        130035554740186809762152344382670192648388101752940643314971638619112864154957540114182619\
        545209429070531797326950151630683621502942030447090067022672243119213825092070120260014287\
        650642444490245716249659346973699250580875272571662195525373996192885786789722040970217252\
        777370731438163261287964967884564981241085126934825161618449785175399006215884116607844104\
        145207013638867416463540845880013358109313225877568519816660562362871960260263575794026214\
        064890992255005169417359526730610750695374229149780488537259076167061214467019669082171678\
        943457474819859479858903499033921570632357132665267027905891468527976638032942305102942045\
        515459276591245938425195867555877301085883501327362346064575111830484879274114432829480494\
        563831995727699779881566675298660138793679903428981563834668966551589617302484043712626362\
        382798953692078892165133138064987411558895534104001531006894289080282050149571295909335181\
        515141217224946465992765152346329894825749923118373449366824218493894112781616587290370204\
        333887308955509161095001732854868950761797166686537700534752075294673973732109977084605381\
        070618564835872903186336652176818456094869891043565959999476502149984797401333397104760957\
        841309659194208245402430373249330458021584229695150780497429392832862209911691278370023807\
        694413776673373810444117733945944078894301474904905817592608430260686560983085185519158848\
        721775216278449134110149155717067705547304860141801798990958269542345867084594152084822858\
        297268178452913654780535118518042943385992804181256593442747503443497424231886204147202265\
        781726406780662404185957882728275703416562894011247737570346607390736902934407385727937750\
        626919779463789669366072755563597481186609564844136728714713287282513301914775072382866982\
        311110548050622515213045989841457194255484993918942830103968812317468779964982010673379443\
        545157108484730053634532297455783144465121662023533540802524486197805471362616441499282252\
        085672354541543506489067224840744574429825390924134914919433088957785876098287919516428588\
        72328203",
        "0x887cbfd350e72440510425951a3.2272334df26c980a06fd6f49c2120e08d01cbf40066a05dc52d2507fc8b\
        a1f7ad8adcfa81f54c2d43be83c3e9e1b2ae573ad5d2402f63f8286058a24ae0b84f54fb38a3e8763f3f65332c\
        7370e069a38ad39fac27c8792cedd37255b83172e20e4dad5cd71f1373de167311ac9c40b97f28dc1deb8f917a\
        4865473383f409e6cb0a10648e46d7a7b4e3635e29b5f290c2b02e89657217e160a8ead0781ee8d369a3f6e0e3\
        373a660b0b7d660faa1b7d91b4cf5cd42a8bdff468e78bef9c7ea60bb9ae18146e44c1a152917815a88c662a0e\
        8a69d92af1497fcb3af33505d54a549fda939b9762346d0a3acec60a952593d9b87d971a913e1a736e06692ed4\
        d0790ed0c9756fec1894ac0a4801a0f3ab65d90696cae94d0193ab36015ac78e049348dcf1a40ba53488aff432\
        71652cc4085e19eb0dee362c9c49aeb3ae6922f6bc840455cda7693538989a16dcbcd4771ea096b89b7b6ec780\
        e4860380764f3f7268711f226e1c48f3f0ae8068a5d25f97b077ce5d6fb11aa921f2c3ba7cf501ff5bc4456ff0\
        a7d490f2e7130fd865897809259d125e6c3a3766e4a746daaf401e8c0be059068b0514c0324881a6f2ed930ee7\
        2bad1215cb6436b3338bf75f190705e69ea7c9c4ebae6e62a0dce4ed74cbdd87135c82ac73fd0bb545e0f0950d\
        b386f309d2a674b58f9f446549a4ed3ae9a0de62e5e714f93368cb01396ec887f428e0d0df92bd1216c295f4e4\
        0268e7f8acb642c1e0df9c960e503d5774a53e0042cc13863d67afe7b2e6a631449876b9b307dfb8d3d8d23331\
        a7f27c4e21b77c1e3fa05115cd74fc8ddf363caf1d632bde07c5905d7b004ae049a1eb0b8862b102bab6364457\
        f072e4eda8795c7bb923327eb1948231771fd0cb4ee5dc7380738f0b1c42d79fbd02c6e1825eadf29ecc297410\
        bb9dcb7b2dbfdbf605008a87de17f15bcdae1b2fb9bc31db5d05410224a6ce36fad367262fff7a125c662ed7c8\
        cd2e30f4106d2b7a2ab98d4736946788f9fe057e0b0b1a6f1eb99ca9d05ba0091da8bcf3c05efa1e034913764a\
        6c0b52c6582c32ce658ca123e9d830d284a0b63dd2e432788616609ae6da4643f7d599c452e4c8b57689f717d1\
        07559c9efcba9e48001571c49a50a5b8bd100a652034b8f6bca1fe99f6e5537912ce6892e13dd1ccae7d8f4f91\
        cb7902ec4aa0749ea#6892",
        Less,
    );
    // - rm == Up || rm == Ceiling first time in div_float_significands_general
    // - rm == Up || rm == Ceiling && !carry in div_float_significands_general
    test(
        "9601551840755394477782155306615.400809036690567306698564563582349725588158197630704697078\
        013417362910194409963799900311565761064675199511838067316509390850104862662213505749078521\
        770377437536581181743514602025941917984526635269955727174740188131870994016855234632753317\
        016792561638598147688756431699678956508985348347746587700412259161976828470219450879730366\
        977549362693268375201952814531271011740413280015272635082852480719566333921902104982178638\
        916033220231617933208782646056491566119203568817027316959068412927057431189073591651262563\
        044927867605702941287180061603188567390575475013505450984801866920347023147022407388362970\
        59088991236828388853335477454316615451511519",
        "0x7930498878f5aafc3b8512a677.669b6bc886f9e61b8a5f2697dc5a3b5a2b2256b53dad0634eac355f6a0a5\
        1716868ade4d034f6702fb8253c1f989a028a5714d4d362b3553fc85427910a52d3d77e4f39bbc7914d902c799\
        e58ce8c619156cf04fd16baabf3b0abeb8808eb23612b9dac1cebfc00ede036daaa891cfe4e827cb450c1dc08a\
        8127d7ab7af3ad2ad37537634deffc4816e65732919e1337bdf6abce20a753777e12957b9282ddb2d803496313\
        dc5a1048926798d52bfa132da701a4550ee67ff9951f347ffc2fcd57993813839688be78495c10ef533661a217\
        c12a2f3327e0d96f3ee8d9b98f05db022bcbe3a69ce3b5009d3946e0ab02a8f9c18127f132344e97387cb60c19\
        2f6949d3c05183b19bc196#2230",
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
        Up,
        "173018210334417056237548035723683.1345550599384516271435235732043979258142510481520742047\
        130035554740186809762152344382670192648388101752940643314971638619112864154957540114182619\
        545209429070531797326950151630683621502942030447090067022672243119213825092070120260014287\
        650642444490245716249659346973699250580875272571662195525373996192885786789722040970217252\
        777370731438163261287964967884564981241085126934825161618449785175399006215884116607844104\
        145207013638867416463540845880013358109313225877568519816660562362871960260263575794026214\
        064890992255005169417359526730610750695374229149780488537259076167061214467019669082171678\
        943457474819859479858903499033921570632357132665267027905891468527976638032942305102942045\
        515459276591245938425195867555877301085883501327362346064575111830484879274114432829480494\
        563831995727699779881566675298660138793679903428981563834668966551589617302484043712626362\
        382798953692078892165133138064987411558895534104001531006894289080282050149571295909335181\
        515141217224946465992765152346329894825749923118373449366824218493894112781616587290370204\
        333887308955509161095001732854868950761797166686537700534752075294673973732109977084605381\
        070618564835872903186336652176818456094869891043565959999476502149984797401333397104760957\
        841309659194208245402430373249330458021584229695150780497429392832862209911691278370023807\
        694413776673373810444117733945944078894301474904905817592608430260686560983085185519158848\
        721775216278449134110149155717067705547304860141801798990958269542345867084594152084822858\
        297268178452913654780535118518042943385992804181256593442747503443497424231886204147202265\
        781726406780662404185957882728275703416562894011247737570346607390736902934407385727937750\
        626919779463789669366072755563597481186609564844136728714713287282513301914775072382866982\
        311110548050622515213045989841457194255484993918942830103968812317468779964982010673379443\
        545157108484730053634532297455783144465121662023533540802524486197805471362616441499282252\
        085672354541543506489067224840744574429825390924134914919433088957785876098287919516428588\
        7232821",
        "0x887cbfd350e72440510425951a3.2272334df26c980a06fd6f49c2120e08d01cbf40066a05dc52d2507fc8b\
        a1f7ad8adcfa81f54c2d43be83c3e9e1b2ae573ad5d2402f63f8286058a24ae0b84f54fb38a3e8763f3f65332c\
        7370e069a38ad39fac27c8792cedd37255b83172e20e4dad5cd71f1373de167311ac9c40b97f28dc1deb8f917a\
        4865473383f409e6cb0a10648e46d7a7b4e3635e29b5f290c2b02e89657217e160a8ead0781ee8d369a3f6e0e3\
        373a660b0b7d660faa1b7d91b4cf5cd42a8bdff468e78bef9c7ea60bb9ae18146e44c1a152917815a88c662a0e\
        8a69d92af1497fcb3af33505d54a549fda939b9762346d0a3acec60a952593d9b87d971a913e1a736e06692ed4\
        d0790ed0c9756fec1894ac0a4801a0f3ab65d90696cae94d0193ab36015ac78e049348dcf1a40ba53488aff432\
        71652cc4085e19eb0dee362c9c49aeb3ae6922f6bc840455cda7693538989a16dcbcd4771ea096b89b7b6ec780\
        e4860380764f3f7268711f226e1c48f3f0ae8068a5d25f97b077ce5d6fb11aa921f2c3ba7cf501ff5bc4456ff0\
        a7d490f2e7130fd865897809259d125e6c3a3766e4a746daaf401e8c0be059068b0514c0324881a6f2ed930ee7\
        2bad1215cb6436b3338bf75f190705e69ea7c9c4ebae6e62a0dce4ed74cbdd87135c82ac73fd0bb545e0f0950d\
        b386f309d2a674b58f9f446549a4ed3ae9a0de62e5e714f93368cb01396ec887f428e0d0df92bd1216c295f4e4\
        0268e7f8acb642c1e0df9c960e503d5774a53e0042cc13863d67afe7b2e6a631449876b9b307dfb8d3d8d23331\
        a7f27c4e21b77c1e3fa05115cd74fc8ddf363caf1d632bde07c5905d7b004ae049a1eb0b8862b102bab6364457\
        f072e4eda8795c7bb923327eb1948231771fd0cb4ee5dc7380738f0b1c42d79fbd02c6e1825eadf29ecc297410\
        bb9dcb7b2dbfdbf605008a87de17f15bcdae1b2fb9bc31db5d05410224a6ce36fad367262fff7a125c662ed7c8\
        cd2e30f4106d2b7a2ab98d4736946788f9fe057e0b0b1a6f1eb99ca9d05ba0091da8bcf3c05efa1e034913764a\
        6c0b52c6582c32ce658ca123e9d830d284a0b63dd2e432788616609ae6da4643f7d599c452e4c8b57689f717d1\
        07559c9efcba9e48001571c49a50a5b8bd100a652034b8f6bca1fe99f6e5537912ce6892e13dd1ccae7d8f4f91\
        cb7902ec4aa0749eb#6892",
        Greater,
    );

    // - in div_float_significands_same_prec_lt_w
    // - increment_exp in div_float_significands_same_prec_lt_w
    // - (q0 + 2) & (mask >> 1) <= 2 in div_float_significands_same_prec_lt_w
    // - h == 0 && l < y in div_float_significands_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal,
    );
    // - !increment_exp in div_float_significands_same_prec_lt_w
    // - (q0 + 2) & (mask >> 1) > 2 in div_float_significands_same_prec_lt_w
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_same_prec_lt_w
    // - rm == Nearest in div_float_significands_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (quotient & shift_bit) != 0) in
    //   div_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#2", "1.5", "0x1.8#2", Nearest, "0.8", "0x0.c#2", Greater,
    );
    // - h != 0 || l >= y in div_float_significands_same_prec_lt_w
    test(
        "1.5", "0x1.8#2", "1.0", "0x1.0#2", Nearest, "1.5", "0x1.8#2", Equal,
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (quotient & shift_bit) == 0)) in
    //   div_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#3", "1.2", "0x1.4#3", Nearest, "0.8", "0x0.c#3", Less,
    );

    // - in div_float_significands_same_prec_w
    // - increment_exp in div_float_significands_same_prec_w
    // - hi == 0 && lo < y in div_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        Nearest,
        "1.0",
        "0x1.0000000000000000#64",
        Equal,
    );
    // - !increment_exp in div_float_significands_same_prec_w
    // - round_bit == 0 in div_float_significands_same_prec_w
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_same_prec_w
    // - rm == Nearest in div_float_significands_same_prec_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (quotient & 1) == 0)) in
    //   div_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Nearest,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe#64",
        Less,
    );
    // - hi != 0 || lo >= y in div_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0",
        "0x1.0000000000000000#64",
        Nearest,
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Equal,
    );
    // - rm == Nearest || round_bit != 0 && (sticky_bit != 0 || (quotient & 1) != 0) in
    //   div_float_significands_same_prec_w
    test(
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Nearest,
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Greater,
    );
    // - round_bit != 0 in div_float_significands_same_prec_w
    test(
        "3.1790543009742223972e-11",
        "0x2.2f43e0add6ebd01cE-9#64",
        "7770090901.6225594673",
        "0x1cf222d95.9f600ea8#64",
        Nearest,
        "4.0913991113158902183e-21",
        "0x1.35232b1b3b9aeabeE-17#64",
        Greater,
    );

    // - in div_float_significands_same_prec_gt_w_lt_2w
    // - increment_exp in div_float_significands_same_prec_gt_w_lt_2w
    // - in div_float_2_approx
    // - y_1 != Limb::MAX in div_float_2_approx
    // - r_1 == 0 in div_float_2_approx
    // - (q_0.wrapping_add(21)) & (mask >> 1) <= 21 in div_float_significands_same_prec_gt_w_lt_2w
    // - s_2 == 0 && s_1 <= y_1 && (s_1 != y_1 || s_0 < y_0) in
    //   div_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        Nearest,
        "1.0",
        "0x1.0000000000000000#65",
        Equal,
    );
    // - !increment_exp in div_float_significands_same_prec_gt_w_lt_2w
    // - s_2 > 0 || s_1 > y_1 || (s_1 == y_1 && s_0 >= y_0) in
    //   div_float_significands_same_prec_gt_w_lt_2w
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in div_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (z_0 & shift_bit) == 0)) in
    //   div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
        Less,
    );
    // - r_1 != 0 in div_float_2_approx
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Nearest,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe0#65",
        Less,
    );
    // - (q_0.wrapping_add(21)) & (mask >> 1) > 21 in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        Nearest,
        "0.99999999999999999984",
        "0x0.fffffffffffffffd0#65",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && !overflow
    //   in div_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Greater,
    );
    // - y_1 == Limb::MAX in div_float_2_approx
    test(
        "5.29395592276605355108231857701752e-23",
        "0x4.00000007e000fffffff0000000E-19#107",
        "255.999999999999999999999947060441",
        "0xff.ffffffffffffffffffc000000#107",
        Nearest,
        "2.06795153233048966839153112178982e-25",
        "0x4.00000007e000fffffff1000000E-21#107",
        Less,
    );

    // - in div_float_significands_long_by_short
    // - diff >= 0 in div_float_significands_long_by_short
    // - in limbs_div_limb_to_out_mod_with_fraction
    // - d.get_highest_bit() in limbs_div_limb_to_out_mod_with_fraction
    // - sticky_bit != 0 || diff >= 0 || i >= abs_diff in div_float_significands_long_by_short
    // - tmp[ys_len] == 0 in div_float_significands_long_by_short
    // - tmp[ys_len] == 0 && shift != 0 in div_float_significands_long_by_short
    // - round_bit != 0 || sticky_bit != 0 in div_float_significands_long_by_short
    // - rm == Nearest in div_float_significands_long_by_short
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (ys[0] & shift_bit) != 0) in
    //   div_float_significands_long_by_short
    // - rm == Nearest && !overflow in div_float_significands_long_by_short
    test(
        "1.0", "0x1.0#1", "1.5", "0x1.8#2", Nearest, "0.8", "0x0.c#2", Greater,
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (ys[0] & shift_bit) == 0)) in
    //   div_float_significands_long_by_short
    test(
        "1.0", "0x1.0#1", "1.2", "0x1.4#3", Nearest, "0.8", "0x0.c#3", Less,
    );
    // - tmp[ys_len] != 0 in div_float_significands_long_by_short
    // - tmp[ys_len] != 0 && shift != 0 in div_float_significands_long_by_short
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Nearest, "1.2", "0x1.4#3", Greater,
    );
    // - round_bit == 0 && sticky_bit == 0 in div_float_significands_long_by_short
    test(
        "1.5", "0x1.8#2", "1.5", "0x1.8#3", Nearest, "1.0", "0x1.0#3", Equal,
    );
    // - tmp[ys_len] == 0 && shift == 0 in div_float_significands_long_by_short
    // - c >= u - c in div_float_significands_long_by_short
    test(
        "1539239.2465826685826",
        "0x177ca7.3f200ab152a#64",
        "0.00009",
        "0x0.0006#3",
        Nearest,
        "16812597210.673628039",
        "0x3ea1bdfda.ac72e31c#64",
        Greater,
    );
    // - c < u - c in div_float_significands_long_by_short
    // - round_bit == 0 in div_float_significands_long_by_short
    test(
        "1.7088961703394199635e-73",
        "0x4.d4baa70e83509ad8E-61#64",
        "1.7359472818744e-34",
        "0xe.6bf39991dcE-29#42",
        Nearest,
        "9.844170892645193631e-40",
        "0x5.5c13c13c6d059800E-33#64",
        Less,
    );
    // - tmp[ys_len] != 0 && shift == 0 in div_float_significands_long_by_short
    test(
        "4.874956728709606165589080471392071684004548689044982493122e-71",
        "0x5.6220e3ededa8be921ace72bbb95a16164a2f0abd57c49f18E-59#192",
        "1.5092483e-10",
        "0xa.5f190E-9#22",
        Nearest,
        "3.230056172437141772802006354545046772521759341614858124236e-61",
        "0x8.4e07636cdfc96e412c1de0a522f40a5f092091c1a3aa159E-51#192",
        Less,
    );

    test(
        "6.88621557179233820703925296804982406452e-28",
        "0x3.68ee78c4dbb67961d201a40495749728E-23#127",
        "0.1418399214207466117788070203268",
        "0x0.244f9effc4f1edfd85dfab3008#99",
        Nearest,
        "4.85492060543760755133907256608679730501e-27",
        "0x1.80a57d020f8b7083401eec627a6787ccE-22#127",
        Greater,
    );

    // - in div_float_significands_general
    // - up[u_size - 1] == vp[vsize - 1] in div_float_significands_general
    // - k == 0 || l == 0 in div_float_significands_general
    // - up[k] == vp[l] && l != 0 in div_float_significands_general
    // - q0size < MPFR_DIV_THRESHOLD || vsize < MPFR_DIV_THRESHOLD in div_float_significands_general
    // - rm != Nearest || shift != 0 second time in div_float_significands_general
    // - qqsize > u_size in div_float_significands_general
    // - qqsize > u_size && !extra_bit in div_float_significands_general
    // - vsize >= qsize in div_float_significands_general
    // - in limbs_div_helper
    // - ds_len == 2 in limbs_div_helper
    // - qsize == q0size in div_float_significands_general
    // - vsize <= qsize in div_float_significands_general
    // - rm == Nearest second time in div_float_significands_general
    // - !goto_truncate_check_qh && !goto_sub_1_ulp && !goto_sub_1_ulp && !goto_sub_2_ulp in
    //   div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) in div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit == 0 in
    //   div_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff0#65",
        Less,
    );
    // - up[u_size - 1] != vp[vsize - 1] in div_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Nearest,
        "0.99999999999999999989",
        "0x0.fffffffffffffffe0#65",
        Less,
    );
    // - qqsize > u_size && extra_bit in div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky != 0 in
    //   div_float_significands_general
    // - rm == Nearest && (round_bit != 0 || sticky != 0) && round_bit != 0 && sticky != 0 && !carry
    //   first time in div_float_significands_general
    test(
        "1.5",
        "0x1.8#2",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "1.49999999999999999995",
        "0x1.7fffffffffffffff#65",
        Greater,
    );
    // - ds_len > 1 in limbs_div_helper
    test(
        "12077.327578390390934514",
        "0x2f2d.53dc2d699afa78b8#75",
        "4.90332775049862782951473377323022738896770775e-11",
        "0x3.5e9a4013acb1890afeca956568e5bffe30edE-9#146",
        Nearest,
        "246308796656764.923124719743308898445103382544",
        "0xe0043c546c7c.ec51e6d16d3ab81c76ba65494#146",
        Less,
    );
    // - vsize < qsize in div_float_significands_general
    test(
        "1917511442.613985761391508315964935868035476119276770671",
        "0x724ae712.9d2e2bbd62dd31f140b2b9b664635f251b18c0#180",
        "3.352896739388742667241376e25",
        "0x1.bbc08f6e851e14a094c4E+21#79",
        Nearest,
        "5.718969570663133425280005234972069245666491961744252885e-17",
        "0x4.1ef6b3c1725013efb2a8179983b542a97b0131f39a938E-14#180",
        Greater,
    );
    // - rm == Nearest && shift == 0 second time in div_float_significands_general
    // - qsize != q0size in div_float_significands_general
    test(
        "1.490328e-27",
        "0x7.61370E-23#20",
        "2.89262335038499315783322011549431655756e-75",
        "0x1.4ef161f7b7fc2c6cb4464f827b58b972E-62#128",
        Nearest,
        "5.15216656741446303242577558053691246166e47",
        "0x5.a3f1d299f6f20544fbba161403f075f8E+39#128",
        Less,
    );
    // - rm == Floor || rm == Down || (round_bit == 0 && sticky == 0) in
    //   div_float_significands_general
    test(
        "2.38418579101562499999949513e-7",
        "0x3.fffffffffffffffffc0000E-6#87",
        "2113929216.0",
        "0x7e000000.000000000#66",
        Nearest,
        "1.12784561231761934265233937e-16",
        "0x8.208208208208208200000E-14#87",
        Equal,
    );
    // - k != 0 && l != 0 in div_float_significands_general
    // - up[k] != vp[l] first time in div_float_significands_general
    // - up[k] != vp[l] second time in div_float_significands_general
    test(
        "65535.99999999999999994",
        "0xffff.fffffffffffffc#70",
        "22835963083295358096932575511189670382427701248.00000000000000022202",
        "0x3fffffffffffffffffffffffff8000007000000.0000000000000fff8#219",
        Nearest,
        "2.869859254937225361249367321235116718339077564583058127288930659162e-42",
        "0x3.fffffffffffffffff000000007fffff8ffffffffffe000001c00008E-35#219",
        Greater,
    );
    // - up[k] == vp[l] first time in div_float_significands_general
    test(
        "1.91561942608236107295e53",
        "0x2.0000000000000000E+44#66",
        "43556142965880123323311949751266331066368.000061035156249999998",
        "0x8000000000000000000000000000000000.0003fffffffffffff8#205",
        Nearest,
        "4398046511103.99999999999999999999999999999999383702417796084544",
        "0x3ffffffffff.ffffffffffffffffffffffffffe00000000000004#205",
        Less,
    );
    // - up[k] == vp[l] && l == 0 second time in div_float_significands_general
    test(
        "255.99999999813735485076904",
        "0xff.fffffff800000000000#82",
        "1.35525271559701978119405335351978053e-20",
        "0x3.ffffffffe0000000000000000000E-17#114",
        Nearest,
        "18889465931478580854784.0",
        "0x4000000000000000000.0000000000#114",
        Equal,
    );
    // - q0size >= MPFR_DIV_THRESHOLD && vsize >= MPFR_DIV_THRESHOLD in
    //   div_float_significands_general
    // - u_size < n << 1 in div_float_significands_general
    // - vsize < n in div_float_significands_general
    // - in limbs_float_div_high
    // - len >= MPFR_DIVHIGH_TAB.len() in limbs_float_div_high
    // - k != 0 in limbs_float_div_high
    // - q_high != 0 in limbs_float_div_high
    // - carry == 0 in limbs_float_div_high
    // - carry != 0 in limbs_float_div_high
    // - len < MPFR_DIVHIGH_TAB.len() in limbs_float_div_high
    // - k == 0 in limbs_float_div_high
    // - len > 2 in limbs_float_div_high
    // - qh == 1 in div_float_significands_general
    // - in round_helper_2
    // - err0 > 0 in round_helper_2
    // - err0 > prec && prec < err in round_helper_2
    // - s != Limb::WIDTH in round_helper_2
    // - n != 0 && tmp != 0 && tmp != mask in round_helper_2
    // - round_helper_2 in div_float_significands_general
    // - rm == Nearest first time in div_float_significands_general
    // - rm == Nearest && shift != 0 first time in div_float_significands_general
    // - rm == Nearest && round_bit == 0 in div_float_significands_general
    test(
        "914122363545.7300954288539961362078521335512160921125366724096502748846903936195389924148\
        154757063704910629280973433563521737016298541257057972452261335506117124831272191707877190\
        2119190824963185068512647039440777212199983388696",
        "0xd4d5f05299.bae788b5e312f78f55ac79e4ca82e12494296afdb40ba21e0c21a4b3915ba2e217c389f8c9fd\
        22042f5ed70da20cfb9f1ee797b1433e077a2d34b1ae5781f975eebbcb21a32ee0c5afa5e59f8f382fe0c754a4\
        a3fb57fa4d668#754",
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
        "9.161757985214429764992710266551647359057325985892606639113002591596898046569993924851132\
        469009506849046579846739625666450546615840413944516736833310444241924771226669449467280905\
        847180462085951493210441487438964221396785151298524525494386941673630175499486324164244756\
        513337189186243674611400515366863669226025015162200893355049656715514081022497216123358900\
        204159965608607573130556648229035236124949246847315199932950877590362471534070708393335471\
        028355306786566135185894674384218244976402468322690432705335567965983855642753641740432604\
        3437409476409135277542e-112",
        "0x8.d032a1c09c5a822250facc4d03dcbdde26d4f5fe102c1e08a4f87e413b615d798e202484a718a7e4ee277\
        3677de7769fc7d817e371393d771d3460b42f92e9ba23196df3ebdff7cdda4294aecfb6c43776a893a979bdc8c\
        cac166e11d435edd52a1481ecb355a6595fcd794f14478ca886b31b8422e8bc9fdcdbc2261e6c6dfdfea3875fd\
        d48e82b6f89b37437a8064efc36e3671100bf00cb530951d17bbaefe545249991b357ff0fbc5a593a69916e391\
        e844f20336e8635a395cbda774a8ed440b65ccac5a4a48827068b6780bdeecccb424ecbcea085547d055a670dd\
        a2ce7fd1bc8ccfff3fcE-93#1859",
        Less,
    );
    // - q_high == 0 in limbs_float_div_high
    test(
        "3.667390117738159207950705349477719105571949429980976394812181671883945054829509256768693\
        428777428660656941387302685172371854480204626447190818847235223777196475037450938977825246\
        002439873468885558547110470430228085143851019175355894923610792842932340338852051239754427\
        095026930245556704615627992643819617817074019244527799280182728405175259498139200084973400\
        185025632529817381341736837108765891093445296142846875524815723636775475593320258020981988\
        641285509338653295726597545939874976233498010217572353942629414771256828458910867296816672\
        522543163090158525097032013393556880192772591230747507060825723928635133507431423165786715\
        278150157159772706487785219042905040143153742214007713387626849311191420055238664770362170\
        343983987545766271318010394410223132309343859005046365250499465556871672911799580499791909\
        295656508202340485571517780009270726548788594036564495463224892161734462687808109262481368\
        096969794376769660230615691331025217443348007977074560288273633401967130359649250852147512\
        141074330343117535301250887203664127461249846867757356639750473042390297739319749311431157\
        14372183907772936544329974943881256368038028229439176970805917055179180439477409e-9",
        "0xf.c0568c485e0b826908a56a1e9eed605a795d47bbb3b22b86ff364a5aa967860d79fa907ffa4b598c74ca2\
        768fd610cc65e72d1328231f74c2896a372707f3fffd4713cd781c36ddc8c429a53c9de0a260ab39221aa6723f\
        639d4f0a18f42a39ce148ec18caa8292a2404e421cb5af96a525988ace64d3b66492e8b29b9f1982af075eac7f\
        a4c4f560684706f9c92a1babe3a7cedd233045842df3c534b90481e818908a787ba694e61d3bd3d93a45651240\
        a1926f3b818e8c51165d9c7c186dd99b0afededda17332acec6e4419ca2c498ecac62e9670b8cc359ac4ce5abb\
        e6a858a9ad732af4717655c73ab36f06357d16912bd759fba2c774b33607e2ee49fbf3328842b34b1649846034\
        e601a686e91c2040c578ab8676f4c413bc62718b75fe591900b6f10a6ee20a73c59ab3be30fb9a154c1a50b4b5\
        d60d7a76de24b93f804302eb4d625df61cf824be4c93189bd500d72fe88443b2e506a11e3b57403b447b8602ef\
        45e256c2e9cbfbc69697901d340ae418d96a38e3f87b38c8ee8b168c15df448ce29060725fff6438c91fd406bf\
        6cf95e07431942e379a50250441c4ed69a634f4e155cb67d47b7b4b285388f957b3809dcfb73606173ca9a64c8\
        9b5ee06f42fc1ee8c752cf947957f346aac01a1e21759f8267f58d36b22e7bd14E-8#3843",
        "3187923845432148642442057154569059.126715487792372839803386914179538752693826970283207166\
        811288798839227319311871148502823407877176659352907588186871022894787193533982873662011253\
        290757573915501169313819926148549862448642272469031186766746849160076154409190019980289710\
        170792165652792217117270925812431819493193080694795589891807146039351866024622601910524654\
        975993653145125921373829181052606711648254654081125875153947355721451359688668050452670532\
        460418624462017975144128989503732730892234660879379487543472739334395798501406522301770530\
        084261662015482020833397653047706275744771695945820139179975325296925632712346348118093097\
        953095934511815810581175318735500116787412839224213098543182657584610954902591533740060963\
        289805212670558460977431314581393471573332429725647583364688986461335610003995668212280028\
        807977055980202986273442266172236653427698776974320204115552560417196660880213932819325142\
        548937684752935846670101028764484218237392844524558383599287530029421881169570993841163993\
        843829902198804691520255195056203676272889080365643704609455722537324606271987166289767672\
        190663805227886932691226996255254535007618551610966568052639325048438160780381909128343538\
        211967934803057176881479842550254050201767779261994751352264395465646274141983125281497566\
        020553366225193569060382295548356106219949376044134821789228041804290511458952966410365196\
        222090758059421770693182158103609003570428820956594490269060711518240230638460085565864341\
        256289190220580928350048868798606128912317218138793827337661513849296003850300428079774414\
        62431384255329048179650372924700507846477189871631671161154559755984562472291",
        "0x9d2d417f3ca9f32fea99c6482363.20706d1bf7058f4c6275f668a177cd076adccb2fda12b6ed78a3b56bb5\
        9dfb518b8b3c05c40c48fd5544dac5cf4c4b5097a348e21623af642ca54df95b1dc69591e2bdc1e3f296461a0e\
        73545f0b1a728f095b34af1c14dc3ff040878852b81a047198ec51c9f7dcfffac0ad33017fdb2f0c43edcff12d\
        ef18336029b6f47a305e278cb4eda766445530f250be179818a2d241b5afebc21b194dbd62400042f887100725\
        62fb877debcff302fcc5b1162c1450e14478eb4e96906a31d6843172390e3cd69b3c0f474a72a62036579c22fe\
        1d1ad35fc2be49e475a1be85f30bec6d387e595070d17b17f5b5a6f400fde641d92abee13055777fe7f6b647fc\
        7850f8002fadb99332ceffb5439a87b2ac7f223b73750c6b42112fffe8b992da6c3fbc5274503b1bba48602753\
        174ba7260f73f3fa02c00fc495aad0f85c84c966f0a98fa7d85cca68b07d58e6292617f3b67fd0aafc0dc0c457\
        806b811f2698bea27de70e9ea3de0e898978b9670aa90750e88ac855daaf830c9dedb5d22968f2b01302edc889\
        ce03e2af4ec2e339258ace8efa81eeb76b273039929d7289eadfb0bae898fd0257e0f1db349eba610dfb56e3d3\
        1520f08012e02d96edfbf9a1a05ad01f682c49e1cf1e0f2b1131943ffe95afd8c6454deffe4bfdbf15fe656e18\
        13690a6dbdca197ec4c2b29ac61a6ca074a2866ff9f55184ed344bb45b2e44eca9945a21cd78ccdd427dff1dab\
        1d449dccc0aa07e37c89bb61c7fc94ce0edd5fb60b7e2d8034decb7a0e2bba4c1159236fd7f800450c1516e64c\
        bb2206f385ee11aba1c6993b2d50b2437bc23cc47f6b85d72fdd7348a5e321b5c960e8e23830fc93c4393938b8\
        98c2f16e8452c9e81ce5aa01460fb108dca1e371c53a1e72ad6ad0cb80bd5bf0ace476ab08fe8#5329",
        Nearest,
        "1.150400792350488243006374252439239370084430198894097883408930056885079539097149903334801\
        351771948494649335504249165228317854781872084884358035626531528587565616792440130659246789\
        547068913471358728267587391921067763401960740658485999234360708658257354709138940743401019\
        478517400580149085993404074744575447821383623813098520973947955049547026992303475579228215\
        436224676800479021561310494945776720018368502752527214637080415175177143606297950367602304\
        149567232708944245857383841379350871799797376084773487408407355299732079377175164962925047\
        553551234005632068337070468847266947004802579875459884664958168707856865409967591741263680\
        896819771668163339399940221050773763868745744855354003266260565352234279551485173590767169\
        460117377689246074482291141675360319952860733464727767370984256722737643645180588444886771\
        648355387388454942423178644791668457452750839522592156007162798991977390140623536578544490\
        057707937271642210929120388296125663413585185722459999909564986042307052548228205977082023\
        238981495642981332360042089388607651948288196055583153394379775735995327224157713864077477\
        321557707540034099204193983589868016082915953745995091314702115380175700364741814725184464\
        602065018950641261052531311066491931955988616475785792821351515116629573461861957542243077\
        532642867446492701937719979200426618485741197144774966492676324343483017759514367363624027\
        675809506514516688590348320872084123672477458766804242125009602995249222201904595636534318\
        096670348004414769559053243712710972449074750435098780379781902955436286126620063025099547\
        80435463720060716005725004637793753168276780047800093479230422676842555945701e-42",
        "0x1.9a7a0a4405b5655db3032989d155cf7a58151a06aacabc4789fac720edfb0e835fe88bc9af3cc179149fe\
        616753cd76b4c7d9c17f2f47389f4e0007572679dad2a5316ede08c14af0283577f171d41d795d4ff13631def2\
        630089c6f215d7b5b8948c52ff97a4a1d9f1eb6d67b60e55478c40ffd2a7cd9684f43637e46ce3ce3e33085654\
        9165c4a377c6ab1dbb9c9b40ece8c47d94ddd1318dd2e5e57388b2e8ef80705d97c3db61d805c43cf7ff7a9a1e\
        41ded3ff033e68dc751b34ffd9cf2eae50cb7e7875b9d8f24116927cd9f609a65c71e840166cf535bbf110404d\
        bc493350b17705c0e23a9091d61f544117f70c6c6387dfb9a1dcc2f513cfbebc4cdd4b7d94c9fc57ceebebe3a2\
        e7d85b9b488b5571ef7b7c8621b770d99c67f9a19252ec5f9be4b129c7755b4a8585b97ea68e60e390c0b5c2b2\
        7b5fc3a47825c136e3b2517a6a7490ae84cf61659a9b819bfe59d45f7254dd48e028c7b694a9b9b427e60358fd\
        52afbeed855580a61e351d523d4ffaabfc7ca00e9a5b40128e9fd8b2998c189e95abc1857ff9ddf1dac904a2de\
        dfce45cbc4f1ffac50c26ec7e1135aa9ca96f6d3ac8cb3a6620a3aecb003d246eade4cf0e6394df920dfba899f\
        44ed41072e121f0402f19fc4c43c348467a07566df372a7b1af45354f2b4c7f94d52f355813e84c1a95202029c\
        0056a974e856e7c42fd6463561d1b5e02ed6a7e0ea0ca50887bd1047f4abd068ea61e2095abdad6a0cbaf91846\
        a340717aa624d6c6ba02f5d3e835ff06c742f1343479ec9a9b184eaca8e7c8be7eaf4fa322afc13f046a4a2e5f\
        4e84c723c68079991a080ac6939780e172640d568c2bc3452c14317358ee8d27a18af7c9bf2de8bea3e5b8b113\
        d8e61b810d6103e805c2a8f85b9b88f8c9129b924ba95521aa83a066991bea980c8be16f1df53E-35#5329",
        Less,
    );
    // - qh != 1 in div_float_significands_general
    test(
        "5.001744775175450910666028825162941035057223155811961434576858983758571141018459147239961\
        203150588371507863692097065128597336587126035820647361489875366021800666979434518756504925\
        080057234368614339602900137940067888065298554658519361482160014564328827389146902718969285\
        655774560873251528749737416878097111467317878479458938947793439179987668446365650646689222\
        368834166803110702160567896615568919058064520266761717855703906871630752776256537972706691\
        492064397304429453321564568279795029252524047182880248127317544801138445700458013706547493\
        607273723210196209139384085476634511414334117395068648152693457778920460359930382343697178\
        078573995749595631668772420417704996567074590195305455833132484492282116574194971013126880\
        3791636230633361526548302742414616516594455084620537903358416e-16",
        "0x2.40a97302ee75111e17146bc65c8925811ce517da511093e155a5f8d319eaddbeb4108f1636a175bfa8c49\
        995045d6820b2f007a269091d024c939d8b02f4910a81e4eb38a836a327a5c12207dbd4d7a81228e55fec96493\
        eb7d51704a03ee77c5caca6616fdc0b6cbe90c676923de6ef8bf3f132b9e5e0dcbae8db3a41502b6d35629f01c\
        0834af3506639efdaa9dba6adf35a24b53b04e032ba7f9821a7155eb04aa7d235436bb878e13e2f265b7a183bd\
        7830bf484c2c6b19e1df88120105ab6ceb5f940ee7e82d4a6da4e67b7532f20750db350a532138117c02fd3f63\
        1e917747a8217c0e647adfae38491beacae6be9197fecb6a639604eba9f3e2a0e1250124f9d994d6ae0f8077c0\
        ad1f961f00f0513cb1b3b92f03fd2e19ce799415d8c26352d23ab730bff342c3d10823b5d476e3a74e5e3a1265\
        3a2e81ad38c5d7f45687a8E-13#2587",
        "1.142392802815468388118014752111991104436260746248041498551240097570984474280784266879307\
        592064853042631818930172030116326290909317377878988867978348974337550025356060840134215623\
        183687852648862683292152461337367387727519703906836027722282460995072637442171724001503892\
        471336699233392710738717656085295397789876649817787754823752786376233371866685422498954888\
        388883747226256845650864591251580129661172288008506642506027201072159168710566406994425528\
        61698637621752755004821872e-17",
        "0xd.2bbf98dfde60cfd72ff373085dca4697e7a8a2b1b6d379d3c49be918a519d5508c59f210662104e5d0b4b\
        bb4e9f09afcccb3c1655f91f2a86657e3f1315aa4e7c857d68f4d7b989d2a2f5d56a205e85ef7d6d2e9325e0fe\
        eded2158374d99d513a6d203143a26cfd251731f49e63a0e342dec62e52287bd673124d763a94038f4529cffd3\
        3599c97c0e19c589ce5603d9c26a084d360b9e7decaa7dda44ce1c27bb7c21adcb23b90d069b0a9b53b9d66094\
        d817f0420227841d34052ed2bd52e148923f8E-15#1571",
        Nearest,
        "43.78305573046740119713641861874642911154821650761595780287653003720809501996262685108891\
        851641972710141298162279987632328545443442081070773994511258975201978303856309243168868354\
        195798884971190979692045031917290903217918542303368118161338247041052975773351929884789101\
        115569142934332750399827875003719838494787907290778719530315375971295186399328856671813524\
        401338950750393532748258170809380674066918530153006391759208813425198252649570832466781508\
        205219467712658427254330831981130973959961174357861904110964877950640959685965415405374479\
        749995867814629083616329619361738872731212580413192190669870683344353586783786957328312637\
        080558281918488071596583884466679323108552730394571978915258990045025562636051246193761815\
        9037942507064891051161378037771712070204599081778501997567779",
        "0x2b.c87657214d897953f5e5edbb169c290285fbd11622c9cf401ba99ad9f03da7ffc778df1db0d888d67c18\
        379efc8b4b36ed8cbb67da04b5b4cfdabc5f751b0a6fc68b1e3a2a16a62c4160ce4d10e00ae47020ca5d3867a7\
        2213145fe6456480971ef0cb9716c6136384fe41721979e86d1ea1bdc104f2967865add528a1367b01cc449a48\
        5786a74209d8e4c5e216fa7ae2dc897fd4926b55eacde3321f7c41bf2875c24933c8eecc7a8a26f738fd6d666b\
        678ec93b48bab7b34c5392d3ca76949dab6958fa5caaf70927d3e8b40d050bb607bc1b4fe656506e1b3e468e87\
        8b257c21e926286697a97538d3230475cd54415b8154351e72363b4b7509061108fc6ac5db47219368f3ca4011\
        5309edd7318a116c2b62a34277bfdc8a1faf656b14b6a046087cfc5dd238cd94fe91967fb6dfc52f8afa5699df\
        e2970ca40fb03c71d7d668#2587",
        Greater,
    );
    // - n != 0 && tmp == 0 in round_helper_2
    // - s != Limb::WIDTH first time in round_helper_2
    test(
        "0.029226865494398284939675773661541675546326681033634876986774906885711036844605915426240\
        340310096249605517533494947862661721742262334681480455706389428160732541825851560446846541\
        984468705310228771603694136853587701004882713625816107989162498255925332620064155091399923\
        221735997586925448254801064429910518067322424203818026901404115120552071966804127784634236\
        948421057038304255272147630765439482924461011451862112889343084859504616664030668233890526\
        750016895266382189553251377266295422223247821668554701473442441938933867323625955488726630\
        366895993417469747854898337776376811834753617592182604418498468334055402941851068346511528\
        636460609896950078921029074658151972716364203699936241491820871671807186599907001653993018\
        871354678119954470481273332266267481346670395677171242596948926022697465301700258863974068\
        74909984356479995046141060221761701111762543380195142151878885588292130260647185",
        "0x0.077b696f76893b930df354ab0e34b0df1508ee4503f673e22fa3b41867c5e4ffbc43b589d4cb4a00c472e\
        4046ccc9dd4a2b88b59dde14b46da030dc5a0f825fc1d9ff0213e8b046b1cd79785dd554b78e98759eae454c23\
        4fddf6ee7ae174bfc7c1ed096e905b41ce6b18511a9bfc9cfbc43c536393410fe83a634f402b0f18a446a3af90\
        9a4079394959da6918bd9094c5b587839c67f902f1f107259257f4ae96549552e41dbe7dbaddda5b9d8fa2b2bd\
        d01ba920c27d6ff6e44bd8f0ef230d60508f693680e1d769f920949bd35768a7ff10fa62210e3caf84f93cdccb\
        a5238b5e4be804a1422da22abe509c758d0cf44f202896613342ffd0fa93939f0c9bcd4de899fb72b286773da8\
        fe9cbfbd51894ec97176996bf2b6a61ac27a5f524cd408e8bca09d7cefc329a98f17616d4b48652d0a3f14cc49\
        a9bbe75a69ae9167aaa9d1951d446e95bb89c1760a549ff81f7b1d8ee454047a7d3c3e244dc499d97b256eca33\
        3d43933df1e0a046136e10#2940",
        "13755175900776476.03444104347769161034491994978466673515315430911280875941467408228432201\
        072744723926732268661761372710838010056791678637659254162102142124198282086034764229922487\
        783612307113854432839997318024745178344073274492535681224805316900558167281372795857375154\
        556654904332875058382920111153919687941523340022403648029377155100560618075774147398400587\
        446623619383420263487024723715538921428293815054071349051549046244877203394687286110159347\
        188168590858399073350484327515115795260226962105536078430163098017753828590552884767518140\
        1985404905771271934",
        "0x30de427563881c.08d120d36075efcee88657ce81cdcaedf45cd89aeca352b6e32212d20771ea31a54387b4\
        8b1eb8738ae1d31c6213ddc44bdc809d5f5b278e3449ebd13c9ab8d89ec9f0a2d87e7233cbd5128caca14e0c42\
        61e5c9ed6444b50d0cce082673e3c80b1a7102c8fc7520036bc3c6900dbcff7cecdf27ac4022bd4095736dba93\
        f47ec8ed66154c32a8eb07e14079a264e1e3370aebbfeacf3a1bbfe7aa657d9911acc70d626a35a29d86c84029\
        f97428f7cd8a3965838abf5dba9a9943b07c0ad2541156ef8e2aca1afd50c7dc55f986c835b95647701f744563\
        d15716174f2ac444#1845",
        Nearest,
        "2.124790384741531086324618396766014790807036315098759090708607153306148130187657728807205\
        824808003798574987452867827342282934278081982848900135498971286785256137258700150407383617\
        040883394430575593960834148265479663570529896251340120502267014474793716347382140126837773\
        997589830815187190064041312015350810806968062929485379465521677249527992330225473850965169\
        559943150658138282772411679190301632849325467452565481282203194749783450341076382685241552\
        308193411477432500590221491760317417660435511777399629140824293491122934845121250984106958\
        267596015361360781611025519421253206751640221273565905944385402975185311734694681019948131\
        664198303801199498616289444621169851399879692298796098215712787193626312888177618196672010\
        550069281366814417353104090880010484400040465312380510548737422916103805415326232623198929\
        338433641210905412883566788706510371353536321443505713555119784985687877546815e-18",
        "0x2.73209f5170c5b9aaeb5a7e9e79e1dba6ba9eb57b8701701f4d2be387a03993b7e53f907a48a9029ff962b\
        4eb20e6ade6771889642b19b1985ec76b2b24fb517b27eb86681dab6bc7d5a5a203545d850396986ce5c6f9542\
        50b478a0dd27222c6c45900f2d06dad9d7f78a79b9978e3ce203479c5dce6dde3affc40e370565c038007c8bc1\
        ef1fdf0f6398b88721063c52e5eb2c4b5ba1f10d93710d5abe8aab35f5bc5cdf7031f7765dd4f9d4065b1b5b86\
        4ccd6665b73715bdfe783fae157cdc8a78e9d053cae011d4dddf28499ac3809e290ca0a221e38d2a6dd8d01980\
        c64da2f6836e0815e2ae3feb8a0d765808afcbdf6df10cf661eaf6c064ec8023cad01912101fb7e8b732b555b4\
        a053a203ab5ec17c24af5694ed7db4f67c3c76a7f360512bc9a2018d2860111211238048d21af3d79aa0904474\
        22c0d9c9883b2f3769a5fe3faeaf8bab1409329c376b70c7b54fe1393115359c5a7ff43560bc0e2548a02ffb68\
        184585e5023a6fb507d0E-15#2940",
        Less,
    );
    // - rm == Nearest && round_bit != 0 in div_float_significands_general
    // - rm == Nearest && round_bit != 0 && !carry in div_float_significands_general
    test(
        "767950667289861979450190915916317600206727962344150969868.8721759327117422801652737665276\
        756590281054535919141164571740727630029753345081284383462360338144272865435150624843971065\
        632159910277471560574746205336214689461444841220676837587640834824669960199738043592424986\
        826697488691269778053087892841924823231361961899657984958299046295741089107389167540777341\
        1137893901502419455",
        "0x1f51c7714fd0115fee394111538cd8cc2697edb4db72ae0c.df46ec035af05536a25a7e2694997099b2577e\
        a12f12bb82f781cda7cd6a148cc07ab56a0bac9e90b8590cb04b95fcb27209c3c8704cb6940c7fb857c1688d50\
        6042d2fb6c58e0600ed4d86a5af398f029ebf3521880629fcd23f2bfd5f9447e8dee8310647fde5e5f5e2a0a18\
        7cdc4e8c046be95417ea73f5d4a1962ebecd092b613af810#1250",
        "51.02908107282950125513822733633990251324880237789609831750259919822628384179764854139813\
        664310170076349095466129599769105832710389521373306698912599298264052853550294941822816681\
        854313580668986120583821047373123379751687690065811623363578923931243167028825931454472450\
        957582633858214796963862561877157489833071159740156527107422181977546544800568916528954101\
        973657910504690951238460938847030239913388867386095316629182004723248011669496406544341717\
        280330814897033602543594303927085904297027340275835376721330553519116838042737905906942433\
        571685773257005175176741271374980135164819404821829229591371965448368260666618720681201228\
        891015077648242337901859343084853665103006650868094946302847691177013349096688713595881758\
        621480481903927608236275982505035171940737146501502983532266096788435188627685343250723400\
        305856753098249087347464717300786991295322157886984156263306019163631926110778355478267431\
        730519001037411438399866507817311259465545754442998333101310788661554118860610086857525556\
        649462275011688416941329847092728287810185063940836755796270353054485056127471728114260616\
        408519706805571508639638485588089026599284155688404740772260862102363921554760355678529807\
        545078380365647104062842403550988330511500383389703046031841433893387481098301402534069795\
        658730174573583314000979689827444189825731612831252242895060479770394162281620141901978731\
        360879005964038152579829187416621515149774573766217606037565254139422563209723838864340747\
        448492223949482531030183865551640816462517599780266133382572731428229045787209942986614817\
        192016158247860998226332096129248802519217902568127470669978186812858044800325333178140023\
        387809105983555698845635663825858578549915679237164757880235025950739307212793316009945989\
        415835829851915334677834156538103555704104713171341701847682567317855121655570126973890951\
        3929068030854124664713500553256",
        "0x33.0771db70bc3cc1bbfd03fee9ecfaaa1f99d76266a08107a7c922f5787496298c9bd6b5bfa13889bc0bb1\
        0f2e280f2673b20cb2191b3f747978b1483ed5890a8f1e9b4ef8665dff89aeff7e04820fcb58e76837b70b36b4\
        946ecf9ebe8fba5e510503f922f8e39500946e3ba0fd0a28c3a881101047c77426f1160e2835ecd5cdfc3c85d7\
        78adf772e0b5f5d5913cda27866ff4a68981bb0b247705d4a7a13e0cf5df9064561c207ad89d6bd10ed4faf445\
        ceca3d7f86bbdcd652aaf5c547a0071a203dca41ee8ec829aff439308e3dd8d470556949fb583c7ed1bd6c7854\
        bb629c27db1c0caa83e77e13d983d022e1865331aa5f67de9bca45976769e471933efa23a7d5fe8e03b8eed13a\
        3920db5d0f4052f811bcd1955c217ad35a8b75478eb3f2e077ecc810af955e23d57d0b957bf2104261c9f16ba6\
        a16f119f6d83e2b35b1a28b6fc7a029bcec426c495328cba2082e252a65c7267a9a83365475cc6b4672f77d481\
        40ec81e987a366445896d2ae795891105da2f608b56dca4a3e4166c6a0338423e51de87dcbfe3717817893141c\
        8b61f1377d82379374f5ad121cb9e04cf51776a20bc8b0ccaa51862efa4f51d52333818ee4877c039261bcd8dc\
        152db0a6119f3724603b4aaf9994eaf197d5adbcb723d1dc6ebdd8d2cfd37952c4128f3b79556ea134b7193dcf\
        afdc170fa41bf528ba4deac3f3d79d4407db9fd076aaca428efe74dbbc1bc7fad8b57ab1a693330f49aab1ddcc\
        f26bdc853360568f201c8fea22c816ae67afff2668debe399f951e72144cfa93dea4f18d1ee734ed2bf350fed9\
        d126c9b660f6b27ba5e13f15a8be20837e071c52d7588c0a856a969903419e91d47e7011235886759942c1c0e1\
        896e1621b2d23df869694531248722482999c8600632a5ab2279907e29cb3c38994bfbe299cb368a72ef45ecaa\
        b9646b4f1e2f37f24aa954535b1ba220c8e91dfb8f81e56dc45ec4cb3181511fa5b1854096fb3f03f2aa052eb1\
        5111548f398b2a0ffeecd95498fef2bd7f25126507f63bd3803c3a9d1aff24563f7f0baf024307e9c75#6074",
        Nearest,
        "15049274867282653916783425765897232338286237097021544510.63055696334657926692700975852105\
        429908991325181901560777184807721684871576977542689434849596891044659963521044055457283641\
        367304991334226236960366994118987961185804836414630696056584495778598754606288486597978327\
        332370268104018122071463105903604569912790301532188931593783003198530412792016208481406460\
        177569545284296438034561831086095957417103960718034276692579871391678752019863068680376188\
        441084892942908419595289335759422879108451807263984856494740183999962091220506257869533189\
        151879060260534882539324732360323776701789524149049182180697710000483323367419829770508959\
        886298233038628402259126153349007515813222501560286947664943532169644098519601985837510236\
        220345218089637472842801890223279059211522040546360301667071765816467200381174390649332596\
        068474921226001156721359127966627401319343668619013146928630922944213877247277873392955093\
        504639915667110210542526572018725370251615388653789028473354868425502960351888594917293217\
        742712292964655900915998259809033409553076012583762301750508167459913039705278696038307105\
        198133823120123667996810391703122691279674638629684591634520584549627376733588876153831722\
        199369165407756045645488517825617351416613370266999295351664343537186993481753740631996768\
        537474147168714208012012313452364629821090786481898210882140832386720226088895885830143936\
        713915428453413001094302216345205004980013281182978168058085052927135299009881922560519790\
        174395754146412113876346433499785448301225654969985504964945207336686282652994022882395571\
        250336046288750244045997533647713495015654011612857222465008362940468446125031495141758501\
        398188163302138089732641998828146315905675208381266922364102647252983963785718802006725178\
        667871858412814374481149992996719213942377608145948042933317201168269095689729130915320304\
        63030844892155532105060912300876",
        "0x9d1f2bb817353ba61ad13135f94f65b1b52180f58a183e.a16c2e5fd6b05e4155475ec873e35d0f193a9765\
        ef45957a4681138fd789135172e7be4efd1b67c60d22430a10832c82a4dc4a53156de6d8638ce6ffe089ebf880\
        f2e1c68c90b576b5dc0b99085865ed663bd642b7743ff5500d4c6d3e2cf4977af36122c98fc49e81ee87b80d89\
        3fe81fa07bdc5986b40bdb0bf7e6bfde432dcedd2063308cf685bfee2b964ff62d434434a9518683156e532f30\
        11f2ac8f98a75178cd412e00f2261a83f952b6a94bb97c280cb51f16f85891ddd7fe6ad8030e20422da11497e5\
        efe8d88db4f96479fd0b16f3703dca8946d944979a3454bb8155d8dbdd3a765584148771967d02f798d157b6a1\
        59e10461bc83d8ec9e55b557614c35d75b391c0c9d04aefe96cab5078bd3a13d5618ca219640c68919f1fefea9\
        a3d1e47a3fcbc8c19de2210708fd96fed608648d183fd4c1177d803a49f7d276f940aeef6feaffded75f8e03ce\
        33df996eeb67ac6c0bec62d821bfce22d9a30baa6f7f4963eb4eaa91707ba1b12fd6f3e04f75cfea4dc39c6488\
        d72e86c36ba981115f42300b97a7caa427023f16c4f66213cf0c18f04cb6aa66e4830cc7040b3103e27c2e800a\
        0bce21b653566628a5bb8b0becb80b441801f31aa100fb4539cf7e4d6d68815a666c11c6cf4ac97878c551c043\
        3750e9ab6fdeb65765ae3ece107302baf12b3086988bf4d0b37206bde4041cc7c4fa79d38170719e92c052187e\
        e810ed1b2b425c081512c7ee6ea722c413215229ebaecc207fb1126644e66dea7e0139682e90f91c71b579cd86\
        b91211305fe40770c3176e35b783732c2d74c8aa1a09da66c4f34dfa1f9fd35662c5c3d1f82eeb37498b121357\
        e73ed7eea79adeab91001b3c63b1f75aa82793cd1a2b39e1bb09ecf5c6522ccc46652d831abe3ad1f9bc301df5\
        2c646068fd97c0402a29caa4ea3f4de8e5fb8a4d537d45d685f87d05d95f7ba40fbb6a919e93b44fb78b9c80ea\
        6c0a75b4dff2f73844bf4f7172907d8165f606a47821da925eda50af0ce44be22fa2b36d56e1d1698a8#6074",
        Greater,
    );
    // - s == Limb::WIDTH in round_helper_2
    // - n == 0 first time in round_helper_2
    test(
        "2.169542166817986625468879014599175406350620737442480370882139687492174119453066131804433\
        632496405940270655469169364935548092764516132338564210201385531365409396738163453793191332\
        174443470862749001312126324808896288354477512722829484994475343567852816045883088813362218\
        109536965926916816038009290804934132950340684582117875938116310656669280480683072639988319\
        858148103369026349100099758130382359401952836454392007054797540845974323496094771400665868\
        125503436816661354237720713751777833086011158065863046075212704099660843597053397398549954\
        8348685976904925033580113969653312280822082757679509546197165e-14",
        "0x6.1b51e41e16de577dd7b5a3a6222357b305d4e7313b1d47721ebe3d4275ef95b0d98ad997627ec7acc76f5\
        d426b7c5a9333cbc0dec080499093952901e541880379e2fdf874d1d931d1243e2b2ab072c0772ce94734ae65d\
        ff7afda628d44635b3fba75efa9bd2c10d8bdcb3a61a8b6a7697f598758d80bd808f17f8351b1761850fd75cc1\
        9f86409ac25dd27dd0ce7c26478dae9d50aff0210dc4fa18674fd87aa017255dabd141e1289a7e734e21577610\
        bf92b6ce4fe21881cc5209081e0536f0aeb1dcf6e288feeed183095b510c09e50c432ef280e742528c0c4dd6d2\
        5e65c8b6d19c28914472a930aae1ad7fac96f6442134ee95f3bd8E-12#1993",
        "301180.0045246172416425721655892719573457356766058630317841133942315022590404351104810586\
        213517139802439747677458029756555586634849124296886483237621871644459126081050822492233083\
        707945595254133438910359944302659214080172068073620764660184379004827633051943726032292014\
        225559234820452819113849827971319776547835482966443724022128469656550054145203573809984759\
        336239968661049955088445384576034864711412948863817402256055440443111246220943775270251981\
        245519547063921282094595332683031347213016637666849460902039610984880445147686118036204784\
        051476863264255027838976527798504452504598268980029728768388473210371246534136227460265249\
        86471927",
        "0x4987c.0128867b146bf9c05b0bb90d2c480c2b610c9c19a0a03f58f0d0aefa84d41a94dbc0c1206d80eab12\
        18d0f5e72e0b72a6f063fe0f604b1eedcc3760c7f60b2aa6e35735292ea939fa59fc7da94b3e86d7bbba5f8ef6\
        8136a9a4c5d98df58e4ad215fee20274cd18a324d8b66b0119d3cf93efacf51659a9814222c8f9b53fe6356392\
        e2b27f1ee07621f888214936f129248d805ae614b37cae5b83f51b2be167dc62ef96c1322204921369dc6c7475\
        c195aa735676f467be6a45d895b6b08fba56a7919ac216a6dc76cf9f5c3184a2ffa7b1bc3d8760c250d651afca\
        18aa90ff70ee4532482978816617fb02f0de87b2abd54886d1c7c16d62550d5fd8a4abb55b0c4ebb8c#2111",
        Nearest,
        "7.203473451839519375659920800256968930150281137907207966420457324091308743561158996788387\
        290694159461823269997598092194316979651412090098764718003237064733792291895103263169730962\
        123418174759909091404064508319172962888992299461557410206033700688023479037478761668781891\
        761303156472454780198812373188616203221872171676002219543916314587725944225532571475160352\
        707938049990541748698746656039607206067984173221685967465441429896808706646646536972493098\
        282122681082474772608417991249668805473092287771115239878542545454824251859441724816281902\
        754574769925885897122660854445163455617440019293712118274988718965330199709067927675503187\
        81705947e-20",
        "0x1.542ca6851865ac89e311ac1608cac34c9fe637305345b739b624981a50028d6f60e7fd803167413e1285b\
        796e7a5ed37e1cb19125606ca9d15a697c9c497b14455aae6477ad96ffa4f216a14878a9802e8350d104f0b9d8\
        cd86ff511d7efbd74d40104b107a9d7f33d0e8894d3b157e46b7fd4e6386f823e75ae0efa9be33aac3e252d7d2\
        411f8e2afd3773f3914778d26a6b76f5569fd822db5a66db515e3cdd6699301b71cbdb73f07c24fb20d0c66059\
        fe1f236a4656c3f508e25958bdef5ca863d7950c5740d7849b46bde7e1a38b797265dedd7d4dfdaee7bcb69dce\
        887bddd7a7bbd45a6561cfad8cd840e7d95599a81bb274cc02a161439f7280459a15c9865ad5b658ed8E-16\
        #2111",
        Less,
    );
    // - n != 0 && tmp != 0 && tmp == mask in round_helper_2
    // - s != Limb::WIDTH second time in round_helper_2
    test(
        "7.967842945782984973942186035477141750978306951371418866981413625233901049016799636923049\
        043510064598367854094347269064974737243308027061881036076018766384961823690368428913274793\
        537e-19",
        "0xe.b2b51b3ba9b3fa4c3c91f60bbe2f30efe9403d1c1ed1fa2688711592167dc11f579d747f20609a0e8704a\
        660072ec620d514ab31d0845381f10e96f76ac41c97c2a7b53849757dc846fdeE-16#599",
        "4.290314881292277334232122993869736164625712811477787127079554140127474408529492187148865\
        860241941174878594521708166855716060386064937593774872957730026516247807479269506509835640\
        014575725456051856772978910171974972671741113731811243859146099959299700690748160884209812\
        639693266282036645473254451466381125403186024743001502094065938286200460441378264282871237\
        445194601993083290951436490295842815606015494575193282638997440331311694865085507331318327\
        437042408938144577985735854973264189023250983346696190654635795462654945584732891184659237\
        746883228154089313101793763347136552004708447204474905674167880735273390965084667673056531\
        378593414825374458582134295243495148718875340371312206621884585775967428131940924035968587\
        526069447315202760539547246403295371510536211424729007907823710381420209210303664575050843\
        559776729235120592670693730372232402604761499044741741752247404544150954858075273585139895\
        643242778698674255451287458469387850297372316908957826221759549123594364155475346755476293\
        552962991845182999537688763267208310118481794778770137863789646906639637779481228371710954\
        980900001879766864929946144490423459237336070803276511200399191948305713293997532907864745\
        24489311444825214442690999e-14",
        "0xc.137f67f6b60895b6164f36c36d5b134858a21d493d7d49584a1811d76bd92f10b6d0aa0bea20843896e0f\
        d0d2e93957b024a1b5e7101d0f679c3dcc134107c20f0664acbfdf6bafac9013ae41ce018c62b6cf36043f13a8\
        1c35291946c79569662de17adff4ec759b1ccbe440675ef95167b0d5a5481ea6e7a6b998233e094436c8eeaefb\
        e21fa0f9c24aad8d11f378034d73a5daec0111cef1b0b8426dd5df78555318d44c992e40ad5fa98171908c4019\
        636becfe749a93747c965c11e84b68df48e887e933449d42c1ec5c2d6a7658e91f6d68333ddfde5719ca117d72\
        dadec43975eb0b6b6a076c4ada32d70b0e93250cf5e8836b11ad6a8b13a4a957de6221168782640f2313ca3716\
        3e4da0decaee000e5824d53c71d0a36a55295f8ad1c7a86eb35eab709891d1a6ac96a10448e0e307c7d6742d8d\
        0617a3e21978394d0393bc9be8e32ff2d87e85ae44c3a76ac79752bcca4927ca5dc6dcfc4db10793dc0cfc2161\
        24fdf30070db19fd8a89982adf45a408e08499b77cf25011c54cf9270bf491a2186e1a5fad26087812cc3c2446\
        ca7e5457d75f66fe9e736ad07c6b1fe4b20eaf1f073d454f371f659f7402d24e6666c8e212ddccf50c22209ca5\
        7651a266ecba0559cacf587691f7f7df3389d9968023d71b412cc20516c9b1d00f1392474c6683bd0fd6c6dc7a\
        705d88E-12#3966",
        Nearest,
        "0.000018571697337476093645597974800042221391343383471511758884717235995391699590980821846\
        925772010524277797520625056343676345716562878283777572676679678398279369283853673423775272\
        117775978501001731220831012727542639628984101788862199591237806493805996059161201835253203\
        204357442043212398495525692960904672132359565047969002327766123188917268873022220728326927\
        266813579456518458545345939557868848624450974456390610446369728959726525392113471132021539\
        038960803550352390128253151270734177749632737865390247648212171456318006032507634424453843\
        795159031314496937836591202252458892414236797451738560115150573320872948069964155298984838\
        456270978739037104719669912968807593222214861520588395175617611807975267322946381339673265\
        479787948188151606275111200784895864164305724683464061829109671306661764726602634895903888\
        287506327660181397735839535481997625450956961420572462126703944263526095290954835871182404\
        868513321447417245068813964246287015709186652991049553947407472630595976266674750290084010\
        397575525528176465754260767775733418629588880876176812207672741703984898153224615968196909\
        775917982890995046346113085550279268258183711136206964350043642244181512435164664671221839\
        6370838653686792137638621224928",
        "0x0.00013794d52b8b1e96ced9de16a585696e655c080cbd5da8030eef302763f4138b28d7261786b8ff50bc6\
        9d0a5f06f20dad7ee2a65fae9caeeaee187ea820eae6fd4c8a673a92def1c9a165c1aeec8807ddb464eac6f550\
        6dbe6d6e3a21a035c4472d414f4887b05775ede2ad98b9b380b663c0929394c811648792ef20f0756b6bad50de\
        099fda3dd792ae5616df8837945c3cb4cd833fb9bf0db07243887c0a8fedba7030c428024be8572bca9398f563\
        b2a661574fd7faf130ac3d404dbe94b7e0ca06f440962616e1879d4f15895a10229f04969c26dbb9a1b733f734\
        fd2be1c88c7b20af178cd1d3fa116fba33a435b040155b5f5f28f0668b798810c2acb1faf0581e46cc71e9b07f\
        c9e4ebcd8a96a7d7d318d649e4468baa2ce2cdf9b1adf74f6a6b8b95a3eed5991934327ddfeb243e80db0c230e\
        d593df31dce1201e64430a27d39e6760dcf2086c1cb86bfb4e9211f18940b72d1a492a5b9109c0fdf4f5fa9fce\
        9e0ec199756ee5f8e69ba7ded6b7507facbc46df62adaa4546b3113a80e7ea40bab782194bfd006099f6a79bb8\
        19aad950497cae351fdc370756b86b3188e5c2cf71ed56fdb3683c9cc38facff80b0f2076d0f3b3a8605ca24d2\
        c8b6301601e23b50ea0940f7ba05f92ddd4a644cca6e420d6bfcd06caab9c695ba67b857bc57e1000b5935d0a8\
        79821217280#3966",
        Less,
    );
    // - rm == Nearest && shift == 0 first time in div_float_significands_general
    test(
        "3.494003589335061592114850452482839587826456952956376068177291318828682713177358264760784\
        874304744009907216335963217912223351596098015455390432300579562836209635380394751634810606\
        195414389573610568068623776330707730813932586805062287543490729241818555718410383777926489\
        343323917900958772688451527795643412381235039158654803215405274640835172391547486080681691\
        827169629800140882523056966137099117587677259077996268577162491409610597231997851472454718\
        312803304389117924206548413988545367622912505139619073824279361037770537659001099972259551\
        367613373821227769095845567435994753397767332318253172120529133505790353120177308303588487\
        770407205738279213609665027613843515660027068482463985188375646552261141849026408186728824\
        80307838589616979262426550353243006148083216398932181e-12",
        "0x3.d7797e014d587584d7875beed50257a2555726437bf03fdebac1110cae2c4b64f09f9a338bf2ca8b1fcf5\
        e0128d7f387a40893706e25c04e65fdd316e3fc348d2478632d632bae209325b6c681dde52405cd7f8d9707d7f\
        5d6de0abb73e130c41c21c4537ce41381fc43788187dab4fa280fa46503f1890d663ca441f49a6a7e2b712e710\
        4c826535fdf1c8ae0282d162e3d128a982e44f67c6104624863e7f3c2c60833df521e5bab88feddd4843e4b50b\
        81ba442bc612787ad38f640412f6cff81df9793590dfa6a0debdd7f2f6de7a089fc50d597d700dbeeecfc9d795\
        ceb9a69d05db5c520717ddd7e73fabaea4e2cb06b1e1874b8b541dfca2083cb277e4d1bbefa48c0a427afea0a5\
        87cd5085c2ba28c1cad42a97be72844e851abf698ac844915e9f5ac4af406a2c354aa055f3c0994b7932d1bdb7\
        b4999768f776148E-10#2560",
        "8.388557903692744678886673027706512178020882171440574341138664910770681048759242342335520\
        689291884051853125391850946672822384185374426629671262437149651332606541726962233658521936\
        440197380293004778812876511148916284206096731251130678261803308370362817534297504354768207\
        175302739282372189357904919163400327254111204148827422042723518774290057028465296755126014\
        371014512774422430414368313070504306047237723842986944935010614200247398223867859750512432\
        528263508555538801970472032683504663202558316239308702078808690323956381650112536926354687\
        819619269837340011822483859668579241157516055938780563664184620140501025776680716962470155\
        2e-35",
        "0x6.f80c88bef08546fc8a21f0f2152ee0612eebad2635acbe0d49ce7179b387d0719cd657923976ec2796026\
        5e330a5e71c0cd8417c2cf919556130f9b353cdf2435349f846c895ede372648bccd9c217f1bb7c3e4197c1806\
        c4744c8a05ddf4f67946a4972f056d84028e7023d956a95b2ff067c093002267f7015fecb9ca5ed8f58dde48d7\
        4510e965bfa6478370f4e71e5a240dabdd9a4d6e000d4af93eea8510c6c4e095290bce062925fd9a7245caff37\
        8b7be01d3b94b56154cbeb98c26f78338a98e416fa9acc3bd12c4953b058efdcdbe56335f952208a15166babaa\
        698da808f96df97655d3f5cdb4768e6370755a01515d4ad54f625432fc742e9121b7cce4fdb08E-29#2088",
        Nearest,
        "41652017300815899948547.94831484736938823572318695739742204735159366375620848030675947597\
        937407214848121136435440017047965685656179645007771422864559798518659544278806178160023554\
        020245599674536568425190916908552958650848910192845046055088066614557330847090320384899468\
        575304634050066348271499359839369427201019441174516439045175255034723132436872248790761539\
        506937527488424041602486298663492699846695028977559705152038731125314966620233281505474337\
        677248672173837029868825260085661764008715204976773612281854954844288452423998027475082981\
        779489389492756600188312999400831430516184077432877044587246391185262444911853744883006186\
        684049728461939709785134255700251964580618924591175050653894151676243677123400686218961075\
        24208104943935667403367286485079704207117845193523456",
        "0x8d1f5db9d3f145a7603.f2c4c307c343da5b63ef331aa97f5e951921921a937336258bc4ab65fdf9d715d36\
        ef6755e61dd29859283e35c618271ec076a196c3ddb06ce536bafe52ad10a521ebfdcda2a3839fce6eadd33d87\
        eba1d25c5eacfa66f0af4f1ce568be4792717319611eb807fe7fc0d855f2cf1b099f908a269208b3ee36d33e71\
        3912e0557515bf16566f8cc4c8c45fd6bb2ced1b3d3f27c9b272c6e5dfaacdd66335f658951d70cd7b3190aac8\
        b90d7e564b5c0ac68a04f4681a552c50de11c466e3ac1230d426fdc851e7d5705e73d7ad30a82c2febb82c46b4\
        93762b8d7c80e514c1fe29a64d4189fc176b72bb816f1223676b93d38dc33a2fd578eaf5fa512468b21e723d6c\
        d5595dac5bfd84c94e4826fc5b9aff74dec22c3cb43d7970a1359eb2642295a920a70da20a166db400602f0f4f\
        2aee9255f2251c#2560",
        Greater,
    );
    // - !round_helper_2 in div_float_significands_general
    test(
        "3.999999999999999999999999999999999999999999999999999999999999999999999999999447285212473\
        955543975273480780774427448575976676077991358482977909210124597604668289823519777773553500\
        1249731874464215297923136674027554116062077582682832144200801849365234375",
        "0x3.ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000#2090",
        "3.944304526105059027058642826413931148366032175545115023851394653320312500000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000038180485e-31",
        "0x8.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000001ffffffeE-26#2567",
        Nearest,
        "10141204801825835211973625643007.99999999999999999999999999999999999999999999859870153567\
        518292907627041671008386871973805812348422824293171611020891731413939851336181163787841796\
        874999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        99999999999999999999999999999999999999999999999018341217",
        "0x7fffffffffffffffffffffffff.fffffffffffffffffffffffffffffffffffff7ffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffe0000002#2567",
        Less,
    );
}

#[test]
fn div_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::ONE.div_round(THREE, Exact));
    assert_panic!(Float::ONE.div_round_val_ref(&THREE, Exact));
    assert_panic!(Float::ONE.div_round_ref_val(THREE, Exact));
    assert_panic!(Float::ONE.div_round_ref_ref(&THREE, Exact));

    assert_panic!({
        let mut x = Float::ONE;
        x.div_round_assign(THREE, Exact)
    });
    assert_panic!({
        let mut x = Float::ONE;
        x.div_round_assign_ref(&THREE, Exact)
    });
}

#[test]
fn test_div_prec_round() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (quotient, o) = x.clone().div_prec_round(y.clone(), prec, rm);
        assert!(quotient.is_valid());

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);
        assert_eq!(o, o_out);

        let (quotient_alt, o_alt) = x.clone().div_prec_round_val_ref(&y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = x.div_prec_round_ref_val(y.clone(), prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = x.div_prec_round_ref_ref(&y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_prec_round_assign(y.clone(), prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_prec_round_assign_ref(&y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o_out);

        let (quotient_alt, o_alt) = div_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_div_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient),
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
        "NaN", "NaN", "1.0", "0x1.0#1", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "1.0", "0x1.0#1", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "1.0", "0x1.0#1", 1, Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", 1, Down, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "-1.0", "-0x1.0#1", 1, Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-1.0", "-0x1.0#1", 1, Exact, "NaN", "NaN", Equal,
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
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
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
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
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
        Ceiling,
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
        Down,
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
        Up,
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
        Nearest,
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
        Exact,
        "NaN",
        "NaN",
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
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Up,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-1.0",
        "-0x1.0#1",
        1,
        Exact,
        "Infinity",
        "Infinity",
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
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "1.0", "0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, Exact, "-0.0", "-0x0.0", Equal,
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
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );

    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal,
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
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        1,
        Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        1,
        Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        1,
        Down,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        1,
        Up,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        1,
        Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        10,
        Floor,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        10,
        Down,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        10,
        Up,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1.0",
        "0x1.0#1",
        10,
        Exact,
        "123.0",
        "0x7b.0#10",
        Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        1,
        Floor,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        1,
        Ceiling,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        1,
        Down,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        1,
        Up,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        1,
        Nearest,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        10,
        Down,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        10,
        Up,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-1.0",
        "-0x1.0#1",
        10,
        Exact,
        "-123.0",
        "-0x7b.0#10",
        Equal,
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
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", 1, Floor, "0.008", "0x0.02#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", 1, Ceiling, "0.02", "0x0.04#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", 1, Down, "0.008", "0x0.02#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", 1, Up, "0.02", "0x0.04#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", 1, Nearest, "0.008", "0x0.02#1", Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "0.00812",
        "0x0.0214#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "0.00812",
        "0x0.0214#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "-0.02",
        "-0x0.04#1",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "-0.02",
        "-0x0.04#1",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "-0.008",
        "-0x0.02#1",
        Greater,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "-0.00812",
        "-0x0.0214#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "-0.00812",
        "-0x0.0214#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "-0.00813",
        "-0x0.0215#10",
        Less,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Floor, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Ceiling, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Down, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Up, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Nearest, "0.5", "0x0.8#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Exact, "0.5", "0x0.8#1", Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Exact,
        "0.5",
        "0x0.800#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.4497",
        "0x0.732#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.4502",
        "0x0.734#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "0.4497",
        "0x0.732#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "0.4502",
        "0x0.734#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.4502",
        "0x0.734#10",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-0.5",
        "-0x0.8#1",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.4497",
        "-0x0.732#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-0.4497",
        "-0x0.732#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-0.5",
        "-0x0.8#1",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.4497",
        "-0x0.732#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "-0.4497",
        "-0x0.732#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.4502",
        "-0x0.734#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.4497",
        "0x0.732#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.4502",
        "0x0.734#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "0.4497",
        "0x0.732#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "0.4502",
        "0x0.734#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.4502",
        "0x0.734#10",
        Greater,
    );

    // - (rm == Ceiling || rm == Up) && overflow in div_float_significands_long_by_short
    test(
        "2.493205886061172926",
        "0x2.7e42bdaed3d31b2#61",
        "7.459893537e20",
        "0x2.870ae94cE+17#33",
        5,
        Ceiling,
        "3.4e-21",
        "0x1.0E-17#5",
        Greater,
    );

    // - rm != Nearest third time in div_float_significands_genera
    test(
        "3.7563361266e88",
        "0x4.b87f4dfa0E+73#36",
        "6.769173652614128677797571270436826716e-13",
        "0xb.e8909656207637d3379c02628519c4E-11#123",
        63,
        Ceiling,
        "5.549179736559369991e100",
        "0x6.57b76abe8193e56E+83#63",
        Greater,
    );
    // - (rm == Up || rm == Ceiling) && (round_bit != 0 || sticky != 0) && carry in
    //   div_float_significands_general
    test(
        "4.70916604581e-30",
        "0x5.f8363584bE-25#39",
        "341290809831481093.63402342431195212374059",
        "0x4bc822eed1c5f05.a24f5bf051591756f951#139",
        1,
        Ceiling,
        "2.0e-47",
        "0x2.0E-39#1",
        Greater,
    );
    // - rm != Nearest && ((rm != Ceiling && rm != Up) || inex == 0) && (inex == 0 || rm != Exact)
    //   in div_float_significands_general
    // - goto_sub_1_ulp in div_float_significands_general
    test(
        "6265419941341407687.894108147333",
        "0x56f33e3db44da9c7.e4e44583e2#102",
        "28506201793260972591.2041505871822190859316631273737217979877602592305648246097",
        "0x18b9a5f7fa6ec6a2f.3443367f6825dd51709bc60beed373653861dd1a1c6c0422#256",
        63,
        Floor,
        "0.2197914680735399977",
        "0x0.384440ef50d090080#63",
        Less,
    );
    // - rm == Floor || rm == Down fourth time in div_float_significands_general
    // - (rm == Floor || rm == Down) fourth time && shift != 0 in div_float_significands_general
    test(
        "1.274876025e31",
        "0xa.0e9775E+25#28",
        "7.104011072486714881105976022274735719942619445087760266603169705559e-82",
        "0x5.6412fa517e8e5c9e2826903dbe9c6b4f020acbf4d07a5f83b6e4008E-68#222",
        126,
        Floor,
        "1.79458620199896394199805694868744557483e112",
        "0x1.dd946a676df629632baf4759d5af1088E+93#126",
        Less,
    );
    // - rm == Ceiling || rm == Up fourth time in div_float_significands_general
    // - (rm == Ceiling || rm == Up) fourth time && shift != 0 in div_float_significands_general
    test(
        "1.766000056026770292793619415e30",
        "0x1.64a410213aff5d6e713e280E+25#90",
        "3.8777897715163284337835091275987988e-9",
        "0x1.0a7acc91ecf72f35cdef4a25d008eE-7#116",
        63,
        Ceiling,
        "4.554140786585790019e38",
        "0x1.569d8fa574bc1b5cE+32#63",
        Greater,
    );
    // - (rm == Floor || rm == Down) fourth time && shift == 0 in div_float_significands_general
    test(
        "2.6837e8",
        "0xf.ffE+6#12",
        "33554432.0156249999999999",
        "0x2000000.03fffffffffff8#79",
        64,
        Floor,
        "7.9980468712756191975",
        "0x7.ff7ffff001000018#64",
        Less,
    );
    // - (rm == Ceiling || rm == Up) fourth time && shift == 0 in div_float_significands_general
    test(
        "0.00341799855229",
        "0x0.00e0007ffff8#37",
        "9444877080927366283263.8",
        "0x20001ffffffffffffff.c#76",
        64,
        Ceiling,
        "3.6188915144233536057e-25",
        "0x6.fffd0002bffd4008E-21#64",
        Greater,
    );
    // - cmp_s_r == Equal && shift == 0 in div_float_significands_general
    test(
        "6699743.0549682103909956055738813882394392365647386167825",
        "0x663adf.0e126589f2efed5e335a996cf4ea2b00000000000#186",
        "2.7716996157956048532084742e-18",
        "0x3.320fb3ad0fa6bc833b14E-15#82",
        64,
        Exact,
        "2.4171966604126677691e24",
        "0x1.ffdc7e57cc8990bcE+20#64",
        Equal,
    );
    // - (32-bit) len == 0 in sub_helper
    test(
        "30600.896887617100679",
        "0x7788.e59a6d47a2818#64",
        "2502994226528294594711254131620329726924950421806.930251136099954006921970343593452748426\
        707752157088293012113325450782206798004004382505336607707515733277073230960543511755657896\
        564954610313805343294199781328288665758818764110630501992193446364134574345717105834955361\
        7609386520869921223421139170307521302744186647065523683732731123797753054753663",
        "0x1b66e24958fe115307da9ef9c4f815e31129d412e.ee24f03e9dbbd6cdaa800176f4f7a592b62d2ecb1428c\
        19779d86494af5a809600728d36c503347da5f0182f00935e63a628c673a55a1e67334c368efde2d7a0fccbc62\
        0edc630bdd9fafc83b3088a3186480eefe67a7c19b91b866c079c0716d7b844259f598eeb31d6c0baf035c6bc4\
        6b11b5efad4f3ae582c7740#1151",
        32,
        Ceiling,
        "1.2225716132e-44",
        "0x4.5cbe7760E-37#32",
        Greater,
    );
    // - rm == Up || rm == Ceiling && carry in div_float_significands_general
    test(
        "2.454546732648863276547885977493137821487607756249724782555774558593552627360857928230048\
        942018665976138591785782565939256583959305212359451403757412054481726126552732229930964269\
        470905492198653519153594970703124999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        995761114627674791277020932519574469505017610728137410250159407014720169738824650046673749\
        059339474479527195633303962573451642912661719349979993453218111358455268175773121496346851\
        580476856645369527796626919786850732045813122714395701160155300736782415936562761674226693\
        028929972451809327089060718204876263534635026488050357020524924317959135992439056643999723\
        656960561739673936265216544476386688997222734833105296938067057205929214479610015975275932\
        718619380628277147130611593861916399934357066581330096178050556305086195135562253719174061\
        496425118601354010153670564319137109364713055282273218494183562236985834389291157056469689\
        9828846337814e-91",
        "0x7.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000E-76#4517",
        "0.015625000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000771048757510508716764719334683019132168263108153512725551350734409293871565\
        903519457201213361797567020084883497703807999422859208123581192925346248118136084792628172\
        988363113468168413497993606331107182531734570308778128025953654408272649725745457159283262\
        0134958821588759455527143",
        "0x0.0400000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffffffffffffff\
        fffffffffff8000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000#3655",
        2337,
        Ceiling,
        "1.570909908895272496990647025595608205752068963999823860835695717499873681510949074067231\
        322891946224728698742900842201124213733955335910048898404743714868304720993748627155817132\
        46137951500713825225830078125e-89",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000E-74#2337",
        Greater,
    );
}

#[test]
fn div_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).div_prec_round(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).div_prec_round_val_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).div_prec_round_ref_val(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).div_prec_round_ref_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.div_prec_round_assign(Float::two_prec(1), 0, Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.div_prec_round_assign_ref(&Float::two_prec(1), 0, Floor)
    });

    assert_panic!(Float::ONE.div_prec_round(THREE, 1, Exact));
    assert_panic!(Float::ONE.div_prec_round_val_ref(&THREE, 1, Exact));
    assert_panic!(Float::ONE.div_prec_round_ref_val(THREE, 1, Exact));
    assert_panic!(Float::ONE.div_prec_round_ref_ref(&THREE, 1, Exact));
    assert_panic!({
        let mut x = Float::ONE;
        x.div_prec_round_assign(THREE, 1, Exact)
    });
    assert_panic!({
        let mut x = Float::ONE;
        x.div_prec_round_assign_ref(&THREE, 1, Exact)
    });
}

#[test]
fn test_div_rational() {
    let test = |s, s_hex, t, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let quotient = x.clone() / y.clone();
        assert!(quotient.is_valid());

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let quotient_alt = x.clone() / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let quotient_alt = &x / y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let quotient_alt = &x / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );

        let mut quotient_alt = x.clone();
        quotient_alt /= y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let mut quotient_alt = x.clone();
        quotient_alt /= &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_div_rational(
                &rug::Float::exact_from(&x),
                &rug::Rational::from(&y)
            ))),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt =
            div_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt =
            div_rational_prec_round_direct(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
    };
    test("NaN", "NaN", "123", "NaN", "NaN");
    test("Infinity", "Infinity", "123", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "123", "-Infinity", "-Infinity");
    test("NaN", "NaN", "0", "NaN", "NaN");
    test("Infinity", "Infinity", "0", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "0", "-Infinity", "-Infinity");

    test("0.0", "0x0.0", "0", "NaN", "NaN");
    test("-0.0", "-0x0.0", "0", "NaN", "NaN");
    test("0.0", "0x0.0", "123", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "123", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "-123", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-123", "0.0", "0x0.0");
    test("0.0", "0x0.0", "1/3", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "1/3", "-0.0", "-0x0.0");
    test("123.0", "0x7b.0#7", "1", "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", "0", "Infinity", "Infinity");
    test("-123.0", "-0x7b.0#7", "0", "-Infinity", "-Infinity");

    test("1.0", "0x1.0#1", "2", "0.5", "0x0.8#1");
    test("1.0", "0x1.0#2", "2", "0.5", "0x0.8#2");
    test("1.0", "0x1.000#10", "2", "0.5", "0x0.800#10");
    test("1.0", "0x1.000#10", "3/2", "0.667", "0x0.aac#10");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "3/2",
        "0.666666666666666666666666666667",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab#100",
    );

    test("3.0", "0x3.0#2", "2", "1.5", "0x1.8#2");
    test("3.0", "0x3.00#10", "2", "1.5", "0x1.800#10");
    test("3.0", "0x3.00#10", "3/2", "2.0", "0x2.00#10");
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        "2.0",
        "0x2.0000000000000000000000000#100",
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
    );
}

#[test]
fn test_div_rational_prec() {
    let test = |s, s_hex, t, prec, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (quotient, o) = x.clone().div_rational_prec(y.clone(), prec);
        assert!(quotient.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let (quotient_alt, o_alt) = x.clone().div_rational_prec_val_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_prec_ref_val(y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_prec_ref_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_rational_prec_assign(y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_rational_prec_assign_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_direct(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (rug_quotient, rug_o) = rug_div_rational_prec(
            &rug::Float::exact_from(&x),
            &rug::Rational::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient)
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
    test("NaN", "NaN", "0", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "0", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "0", 1, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "123", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, "-0.0", "-0x0.0", Equal);
    test("123.0", "0x7b.0#7", "1", 1, "1.0e2", "0x8.0E+1#1", Greater);
    test("123.0", "0x7b.0#7", "1", 10, "123.0", "0x7b.0#10", Equal);
    test("123.0", "0x7b.0#7", "0", 1, "Infinity", "Infinity", Equal);
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        10,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("1.0", "0x1.0#1", "2", 1, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", 10, "0.5", "0x0.800#10", Equal);
    test("1.0", "0x1.000#10", "3/2", 1, "0.5", "0x0.8#1", Less);
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        10,
        "0.667",
        "0x0.aac#10",
        Greater,
    );

    test("3.0", "0x3.0#2", "2", 1, "2.0", "0x2.0#1", Greater);
    test("3.0", "0x3.0#2", "2", 10, "1.5", "0x1.800#10", Equal);
    test("3.0", "0x3.00#10", "3/2", 1, "2.0", "0x2.0#1", Equal);
    test("3.0", "0x3.00#10", "3/2", 10, "2.0", "0x2.00#10", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        10,
        "2.094",
        "0x2.18#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        1,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        10,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        1,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        10,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        10,
        "2.094",
        "0x2.18#10",
        Less,
    );
}

#[test]
fn div_rational_prec_fail() {
    assert_panic!(Float::NAN.div_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::NAN.div_rational_prec_val_ref(&Rational::ZERO, 0));
    assert_panic!(Float::NAN.div_rational_prec_ref_val(Rational::ZERO, 0));
    assert_panic!(Float::NAN.div_rational_prec_ref_ref(&Rational::ZERO, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.div_rational_prec_assign(Rational::ZERO, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.div_rational_prec_assign_ref(&Rational::ZERO, 0)
    });
}

#[test]
fn test_div_rational_round() {
    let test = |s, s_hex, t, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (quotient, o) = x.clone().div_rational_round(y.clone(), rm);
        assert!(quotient.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let (quotient_alt, o_alt) = x.clone().div_rational_round_val_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_round_ref_val(y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_round_ref_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_rational_round_assign(y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_rational_round_assign_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_div_rational_round(
                &rug::Float::exact_from(&x),
                &rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(rug_o, o);
        }

        let (quotient_alt, o_alt) =
            div_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_direct(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);
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

    test("NaN", "NaN", "0", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", Exact, "NaN", "NaN", Equal);

    test(
        "Infinity", "Infinity", "0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "0", Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", Up, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", Exact, "NaN", "NaN", Equal);

    test("-0.0", "-0x0.0", "0", Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", Up, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", Exact, "NaN", "NaN", Equal);

    test("0.0", "0x0.0", "123", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "123", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", Exact, "-0.0", "-0x0.0", Equal);

    test("0.0", "0x0.0", "-123", Floor, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", Ceiling, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", Down, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", Up, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", Nearest, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", Exact, "-0.0", "-0x0.0", Equal);

    test("-0.0", "-0x0.0", "-123", Floor, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", Ceiling, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", Down, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", Up, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", Exact, "0.0", "0x0.0", Equal);

    test("0.0", "0x0.0", "1/3", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "1/3", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", Exact, "-0.0", "-0x0.0", Equal);

    test("123.0", "0x7b.0#7", "1", Floor, "123.0", "0x7b.0#7", Equal);
    test(
        "123.0", "0x7b.0#7", "1", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test("123.0", "0x7b.0#7", "1", Down, "123.0", "0x7b.0#7", Equal);
    test("123.0", "0x7b.0#7", "1", Up, "123.0", "0x7b.0#7", Equal);
    test(
        "123.0", "0x7b.0#7", "1", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test("123.0", "0x7b.0#7", "1", Exact, "123.0", "0x7b.0#7", Equal);

    test(
        "123.0", "0x7b.0#7", "0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", Down, "Infinity", "Infinity", Equal,
    );
    test("123.0", "0x7b.0#7", "0", Up, "Infinity", "Infinity", Equal);
    test(
        "123.0", "0x7b.0#7", "0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("1.0", "0x1.0#1", "2", Floor, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", Ceiling, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", Down, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", Up, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", Nearest, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", Exact, "0.5", "0x0.8#1", Equal);

    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        Floor,
        "0.666",
        "0x0.aa8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        Ceiling,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        Down,
        "0.666",
        "0x0.aa8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        Up,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "3/2",
        Floor,
        "0.666666666666666666666666666666",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "3/2",
        Ceiling,
        "0.666666666666666666666666666667",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "3/2",
        Down,
        "0.666666666666666666666666666666",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaaa#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "3/2",
        Up,
        "0.666666666666666666666666666667",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "3/2",
        Nearest,
        "0.666666666666666666666666666667",
        "0x0.aaaaaaaaaaaaaaaaaaaaaaaab#100",
        Greater,
    );

    test("3.0", "0x3.0#2", "2", Floor, "1.5", "0x1.8#2", Equal);
    test("3.0", "0x3.0#2", "2", Ceiling, "1.5", "0x1.8#2", Equal);
    test("3.0", "0x3.0#2", "2", Down, "1.5", "0x1.8#2", Equal);
    test("3.0", "0x3.0#2", "2", Up, "1.5", "0x1.8#2", Equal);
    test("3.0", "0x3.0#2", "2", Nearest, "1.5", "0x1.8#2", Equal);
    test("3.0", "0x3.0#2", "2", Exact, "1.5", "0x1.8#2", Equal);

    test("3.0", "0x3.00#10", "3/2", Floor, "2.0", "0x2.00#10", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "3/2",
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("3.0", "0x3.00#10", "3/2", Down, "2.0", "0x2.00#10", Equal);
    test("3.0", "0x3.00#10", "3/2", Up, "2.0", "0x2.00#10", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "3/2",
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("3.0", "0x3.00#10", "3/2", Exact, "2.0", "0x2.00#10", Equal);

    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        Floor,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        Ceiling,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        Down,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        Up,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        Nearest,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        Exact,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        Floor,
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        Ceiling,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        Down,
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        Up,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        Nearest,
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        Floor,
        "-2.0943951023931957",
        "-0x2.182a4705ae6cc#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        Ceiling,
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        Down,
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        Up,
        "-2.0943951023931957",
        "-0x2.182a4705ae6cc#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        Nearest,
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        Floor,
        "-2.0943951023931957",
        "-0x2.182a4705ae6cc#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        Ceiling,
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        Down,
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        Up,
        "-2.0943951023931957",
        "-0x2.182a4705ae6cc#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        Nearest,
        "-2.0943951023931953",
        "-0x2.182a4705ae6ca#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        Floor,
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        Ceiling,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        Down,
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        Up,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        Nearest,
        "2.0943951023931953",
        "0x2.182a4705ae6ca#53",
        Less,
    );
}

#[test]
fn div_rational_round_fail() {
    const THREE: Rational = Rational::const_from_unsigned(3);
    assert_panic!(Float::ONE.div_rational_round(THREE, Exact));
    assert_panic!(Float::ONE.div_rational_round_val_ref(&THREE, Exact));
    assert_panic!(Float::ONE.div_rational_round_ref_val(THREE, Exact));
    assert_panic!(Float::ONE.div_rational_round_ref_ref(&THREE, Exact));
    assert_panic!({
        let mut x = Float::ONE;
        x.div_rational_round_assign(THREE, Exact)
    });
    assert_panic!({
        let mut x = Float::ONE;
        x.div_rational_round_assign_ref(&THREE, Exact)
    });
}

#[test]
fn test_div_rational_prec_round() {
    let test = |s, s_hex, t, prec, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (quotient, o) = x.clone().div_rational_prec_round(y.clone(), prec, rm);
        assert!(quotient.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let (quotient_alt, o_alt) = x.clone().div_rational_prec_round_val_ref(&y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_rational_prec_round_assign(y.clone(), prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let mut quotient_alt = x.clone();
        let o_alt = quotient_alt.div_rational_prec_round_assign_ref(&y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_naive_val_ref(x.clone(), &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_naive_ref_val(&x, y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_naive_ref_ref(&x, &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_direct(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_direct_val_ref(x.clone(), &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_direct_ref_val(&x, y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = div_rational_prec_round_direct_ref_ref(&x, &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_div_rational_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Rational::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient)
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

    test("NaN", "NaN", "0", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", 1, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0", 1, Exact, "NaN", "NaN", Equal);

    test(
        "Infinity", "Infinity", "0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("0.0", "0x0.0", "0", 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", 1, Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", 1, Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", 1, Up, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", 1, Exact, "NaN", "NaN", Equal);

    test("-0.0", "-0x0.0", "0", 1, Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", 1, Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", 1, Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", 1, Up, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "0", 1, Exact, "NaN", "NaN", Equal);

    test("0.0", "0x0.0", "123", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "123", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, Exact, "-0.0", "-0x0.0", Equal);

    test("0.0", "0x0.0", "-123", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, Down, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, Up, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, Exact, "-0.0", "-0x0.0", Equal);

    test("-0.0", "-0x0.0", "-123", 1, Floor, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, Down, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, Up, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, Exact, "0.0", "0x0.0", Equal);

    test("0.0", "0x0.0", "1/3", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "1/3", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, Exact, "-0.0", "-0x0.0", Equal);

    test(
        "123.0",
        "0x7b.0#7",
        "1",
        1,
        Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1",
        1,
        Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1",
        1,
        Down,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1",
        1,
        Up,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "1",
        1,
        Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "123.0", "0x7b.0#7", "0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("1.0", "0x1.0#1", "2", 1, Floor, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Ceiling, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Down, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Up, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Nearest, "0.5", "0x0.8#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Exact, "0.5", "0x0.8#1", Equal);

    test("1.0", "0x1.0#1", "2", 10, Floor, "0.5", "0x0.800#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        "2",
        10,
        Ceiling,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test("1.0", "0x1.0#1", "2", 10, Down, "0.5", "0x0.800#10", Equal);
    test("1.0", "0x1.0#1", "2", 10, Up, "0.5", "0x0.800#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        "2",
        10,
        Nearest,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test("1.0", "0x1.0#1", "2", 10, Exact, "0.5", "0x0.800#10", Equal);

    test("1.0", "0x1.000#10", "3/2", 1, Floor, "0.5", "0x0.8#1", Less);
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test("1.0", "0x1.000#10", "3/2", 1, Down, "0.5", "0x0.8#1", Less);
    test("1.0", "0x1.000#10", "3/2", 1, Up, "1.0", "0x1.0#1", Greater);
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Less,
    );

    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        10,
        Floor,
        "0.666",
        "0x0.aa8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        10,
        Ceiling,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        10,
        Down,
        "0.666",
        "0x0.aa8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        10,
        Up,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "3/2",
        10,
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );

    test("3.0", "0x3.0#2", "2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("3.0", "0x3.0#2", "2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("3.0", "0x3.0#2", "2", 1, Down, "1.0", "0x1.0#1", Less);
    test("3.0", "0x3.0#2", "2", 1, Up, "2.0", "0x2.0#1", Greater);
    test("3.0", "0x3.0#2", "2", 1, Nearest, "2.0", "0x2.0#1", Greater);

    test("3.0", "0x3.00#10", "3/2", 1, Floor, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "3/2",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test("3.0", "0x3.00#10", "3/2", 1, Down, "2.0", "0x2.0#1", Equal);
    test("3.0", "0x3.00#10", "3/2", 1, Up, "2.0", "0x2.0#1", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "3/2",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test("3.0", "0x3.00#10", "3/2", 1, Exact, "2.0", "0x2.0#1", Equal);

    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        10,
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "3/2",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        1,
        Up,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        10,
        Floor,
        "2.094",
        "0x2.18#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        10,
        Ceiling,
        "2.098",
        "0x2.19#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        10,
        Down,
        "2.094",
        "0x2.18#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        10,
        Up,
        "2.098",
        "0x2.19#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3/2",
        10,
        Nearest,
        "2.094",
        "0x2.18#10",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Floor,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Down,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Up,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Floor,
        "-2.098",
        "-0x2.19#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Ceiling,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Down,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Up,
        "-2.098",
        "-0x2.19#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Nearest,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        1,
        Floor,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        1,
        Down,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        1,
        Up,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        10,
        Floor,
        "-2.098",
        "-0x2.19#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        10,
        Ceiling,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        10,
        Down,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        10,
        Up,
        "-2.098",
        "-0x2.19#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3/2",
        10,
        Nearest,
        "-2.094",
        "-0x2.18#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Up,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Floor,
        "2.094",
        "0x2.18#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Ceiling,
        "2.098",
        "0x2.19#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Down,
        "2.094",
        "0x2.18#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Up,
        "2.098",
        "0x2.19#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3/2",
        10,
        Nearest,
        "2.094",
        "0x2.18#10",
        Less,
    );
}

#[test]
fn div_rational_prec_round_fail() {
    assert_panic!(Float::one_prec(1).div_rational_prec_round(Rational::ONE, 0, Exact));
    assert_panic!(Float::one_prec(1).div_rational_prec_round(
        Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).div_rational_prec_round_val_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).div_rational_prec_round_ref_val(
        Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).div_rational_prec_round_ref_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.div_rational_prec_round_assign(Rational::from_unsigneds(5u32, 8), 1, Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(5u32, 8), 1, Exact)
    });
}

#[test]
fn test_rational_div_float() {
    let test = |s, t, t_hex, out: &str, out_hex: &str| {
        let x = Rational::from_str(s).unwrap();

        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let quotient = x.clone() / y.clone();
        assert!(quotient.is_valid());

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let quotient_alt = x.clone() / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let quotient_alt = &x / y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        let quotient_alt = &x / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_rational_div_float(
                &rug::Rational::from(&x),
                &rug::Float::exact_from(&y),
            ))),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt = rational_div_float_prec_round_naive(
            x.clone(),
            y.clone(),
            y.significant_bits(),
            Nearest,
        )
        .0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt = rational_div_float_prec_round_direct(
            x.clone(),
            y.clone(),
            y.significant_bits(),
            Nearest,
        )
        .0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
    };
    test("123", "NaN", "NaN", "NaN", "NaN");
    test("123", "Infinity", "Infinity", "0.0", "0x0.0");
    test("123", "-Infinity", "-Infinity", "-0.0", "-0x0.0");
    test("0", "NaN", "NaN", "NaN", "NaN");
    test("0", "Infinity", "Infinity", "0.0", "0x0.0");
    test("0", "-Infinity", "-Infinity", "-0.0", "-0x0.0");

    test("0", "0.0", "0x0.0", "NaN", "NaN");
    test("0", "-0.0", "-0x0.0", "NaN", "NaN");
    test("123", "0.0", "0x0.0", "Infinity", "Infinity");
    test("123", "-0.0", "-0x0.0", "-Infinity", "-Infinity");
    test("-123", "0.0", "0x0.0", "-Infinity", "-Infinity");
    test("-123", "-0.0", "-0x0.0", "Infinity", "Infinity");
    test("1/3", "0.0", "0x0.0", "Infinity", "Infinity");
    test("1/3", "-0.0", "-0x0.0", "-Infinity", "-Infinity");
    test("-1/3", "0.0", "0x0.0", "-Infinity", "-Infinity");
    test("-1/3", "-0.0", "-0x0.0", "Infinity", "Infinity");
    test("1", "123.0", "0x7b.0#7", "0.0082", "0x0.0218#7");
    test("0", "123.0", "0x7b.0#7", "0.0", "0x0.0");
    test("0", "-123.0", "-0x7b.0#7", "-0.0", "-0x0.0");

    test("2", "1.0", "0x1.0#1", "2.0", "0x2.0#1");
    test("2", "1.0", "0x1.0#2", "2.0", "0x2.0#2");
    test("2", "1.0", "0x1.000#10", "2.0", "0x2.00#10");
    test("3/2", "1.0", "0x1.000#10", "1.5", "0x1.800#10");

    test("2", "3.0", "0x3.0#2", "0.8", "0x0.c#2");
    test("2", "3.0", "0x3.00#10", "0.667", "0x0.aac#10");
    test("3/2", "3.0", "0x3.00#10", "0.5", "0x0.800#10");

    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
    );
}

#[test]
fn test_rational_div_float_prec() {
    let test = |s, t, t_hex, prec, out: &str, out_hex: &str, o_out| {
        let x = Rational::from_str(s).unwrap();
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (quotient, o) = Float::rational_div_float_prec(x.clone(), y.clone(), prec);
        assert!(quotient.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let (quotient_alt, o_alt) = Float::rational_div_float_prec_val_ref(x.clone(), &y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = Float::rational_div_float_prec_ref_val(&x, y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = Float::rational_div_float_prec_ref_ref(&x, &y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (rug_quotient, rug_o) = rug_rational_div_float_prec(
            &rug::Rational::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(rug_o, o);
    };
    test("123", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("123", "Infinity", "Infinity", 1, "0.0", "0x0.0", Equal);
    test("123", "-Infinity", "-Infinity", 1, "-0.0", "-0x0.0", Equal);
    test("0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("0", "Infinity", "Infinity", 1, "0.0", "0x0.0", Equal);
    test("0", "-Infinity", "-Infinity", 1, "-0.0", "-0x0.0", Equal);

    test("0", "0.0", "0x0.0", 1, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal);
    test("123", "0.0", "0x0.0", 1, "Infinity", "Infinity", Equal);
    test("123", "-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-123", "0.0", "0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-123", "-0.0", "-0x0.0", 1, "Infinity", "Infinity", Equal);
    test("1/3", "0.0", "0x0.0", 1, "Infinity", "Infinity", Equal);
    test("1/3", "-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-1/3", "0.0", "0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-1/3", "-0.0", "-0x0.0", 1, "Infinity", "Infinity", Equal);
    test("1", "123.0", "0x7b.0#7", 1, "0.008", "0x0.02#1", Less);
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        10,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test("0", "123.0", "0x7b.0#7", 1, "0.0", "0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", 1, "-0.0", "-0x0.0", Equal);

    test("2", "1.0", "0x1.0#1", 1, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", 10, "2.0", "0x2.00#10", Equal);
    test("3/2", "1.0", "0x1.000#10", 1, "2.0", "0x2.0#1", Greater);
    test("3/2", "1.0", "0x1.000#10", 10, "1.5", "0x1.800#10", Equal);

    test("2", "3.0", "0x3.0#2", 1, "0.5", "0x0.8#1", Less);
    test("2", "3.0", "0x3.0#2", 10, "0.667", "0x0.aac#10", Greater);
    test("3/2", "3.0", "0x3.00#10", 1, "0.5", "0x0.8#1", Equal);
    test("3/2", "3.0", "0x3.00#10", 10, "0.5", "0x0.800#10", Equal);

    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
}

#[test]
fn rational_div_float_prec_fail() {
    assert_panic!(Float::rational_div_float_prec(
        Rational::ZERO,
        Float::NAN,
        0
    ));
    assert_panic!(Float::rational_div_float_prec_val_ref(
        Rational::ZERO,
        &Float::NAN,
        0
    ));
    assert_panic!(Float::rational_div_float_prec_ref_val(
        &Rational::ZERO,
        Float::NAN,
        0
    ));
    assert_panic!(Float::rational_div_float_prec_ref_ref(
        &Rational::ZERO,
        &Float::NAN,
        0
    ));
}

#[test]
fn test_rational_div_float_round() {
    let test = |s, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = Rational::from_str(s).unwrap();
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (quotient, o) = Float::rational_div_float_round(x.clone(), y.clone(), rm);
        assert!(quotient.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let (quotient_alt, o_alt) = Float::rational_div_float_round_val_ref(x.clone(), &y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = Float::rational_div_float_round_ref_val(&x, y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = Float::rational_div_float_round_ref_ref(&x, &y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_naive(x.clone(), y.clone(), y.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_direct(x.clone(), y.clone(), y.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_rational_div_float_round(
                &rug::Rational::exact_from(&x),
                &rug::Float::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("123", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test("123", "Infinity", "Infinity", Floor, "0.0", "0x0.0", Equal);
    test(
        "123", "Infinity", "Infinity", Ceiling, "0.0", "0x0.0", Equal,
    );
    test("123", "Infinity", "Infinity", Down, "0.0", "0x0.0", Equal);
    test("123", "Infinity", "Infinity", Up, "0.0", "0x0.0", Equal);
    test(
        "123", "Infinity", "Infinity", Nearest, "0.0", "0x0.0", Equal,
    );
    test("123", "Infinity", "Infinity", Exact, "0.0", "0x0.0", Equal);

    test(
        "123",
        "-Infinity",
        "-Infinity",
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("123", "-Infinity", "-Infinity", Up, "-0.0", "-0x0.0", Equal);
    test(
        "123",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test("0", "Infinity", "Infinity", Floor, "0.0", "0x0.0", Equal);
    test("0", "Infinity", "Infinity", Ceiling, "0.0", "0x0.0", Equal);
    test("0", "Infinity", "Infinity", Down, "0.0", "0x0.0", Equal);
    test("0", "Infinity", "Infinity", Up, "0.0", "0x0.0", Equal);
    test("0", "Infinity", "Infinity", Nearest, "0.0", "0x0.0", Equal);
    test("0", "Infinity", "Infinity", Exact, "0.0", "0x0.0", Equal);

    test(
        "0",
        "-Infinity",
        "-Infinity",
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("0", "-Infinity", "-Infinity", Down, "-0.0", "-0x0.0", Equal);
    test("0", "-Infinity", "-Infinity", Up, "-0.0", "-0x0.0", Equal);
    test(
        "0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("0", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", Down, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", Up, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal);

    test("0", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal);

    test("123", "0.0", "0x0.0", Floor, "Infinity", "Infinity", Equal);
    test(
        "123", "0.0", "0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test("123", "0.0", "0x0.0", Down, "Infinity", "Infinity", Equal);
    test("123", "0.0", "0x0.0", Up, "Infinity", "Infinity", Equal);
    test(
        "123", "0.0", "0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test("123", "0.0", "0x0.0", Exact, "Infinity", "Infinity", Equal);

    test(
        "123",
        "-0.0",
        "-0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("123", "-0.0", "-0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test(
        "123",
        "-0.0",
        "-0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-123",
        "0.0",
        "0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-123", "0.0", "0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test(
        "-123",
        "0.0",
        "0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-123", "-0.0", "-0x0.0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", Down, "Infinity", "Infinity", Equal,
    );
    test("-123", "-0.0", "-0x0.0", Up, "Infinity", "Infinity", Equal);
    test(
        "-123", "-0.0", "-0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", Exact, "Infinity", "Infinity", Equal,
    );

    test("1/3", "0.0", "0x0.0", Floor, "Infinity", "Infinity", Equal);
    test(
        "1/3", "0.0", "0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test("1/3", "0.0", "0x0.0", Down, "Infinity", "Infinity", Equal);
    test("1/3", "0.0", "0x0.0", Up, "Infinity", "Infinity", Equal);
    test(
        "1/3", "0.0", "0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test("1/3", "0.0", "0x0.0", Exact, "Infinity", "Infinity", Equal);

    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1/3", "-0.0", "-0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-1/3",
        "0.0",
        "0x0.0",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-1/3", "0.0", "0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-1/3", "-0.0", "-0x0.0", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", Down, "Infinity", "Infinity", Equal,
    );
    test("-1/3", "-0.0", "-0x0.0", Up, "Infinity", "Infinity", Equal);
    test(
        "-1/3", "-0.0", "-0x0.0", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "1",
        "123.0",
        "0x7b.0#7",
        Floor,
        "0.0081",
        "0x0.0210#7",
        Less,
    );
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );
    test("1", "123.0", "0x7b.0#7", Down, "0.0081", "0x0.0210#7", Less);
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        Up,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        Nearest,
        "0.0082",
        "0x0.0218#7",
        Greater,
    );

    test("0", "123.0", "0x7b.0#7", Floor, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", Ceiling, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", Down, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", Up, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", Nearest, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", Exact, "0.0", "0x0.0", Equal);

    test("0", "-123.0", "-0x7b.0#7", Floor, "-0.0", "-0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", Ceiling, "-0.0", "-0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", Down, "-0.0", "-0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", Up, "-0.0", "-0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", Nearest, "-0.0", "-0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", Exact, "-0.0", "-0x0.0", Equal);

    test("2", "1.0", "0x1.0#1", Floor, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", Ceiling, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", Down, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", Up, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", Nearest, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", Exact, "2.0", "0x2.0#1", Equal);

    test("2", "1.0", "0x1.0#2", Floor, "2.0", "0x2.0#2", Equal);
    test("2", "1.0", "0x1.0#2", Ceiling, "2.0", "0x2.0#2", Equal);
    test("2", "1.0", "0x1.0#2", Down, "2.0", "0x2.0#2", Equal);
    test("2", "1.0", "0x1.0#2", Up, "2.0", "0x2.0#2", Equal);
    test("2", "1.0", "0x1.0#2", Nearest, "2.0", "0x2.0#2", Equal);
    test("2", "1.0", "0x1.0#2", Exact, "2.0", "0x2.0#2", Equal);

    test("2", "1.0", "0x1.000#10", Floor, "2.0", "0x2.00#10", Equal);
    test("2", "1.0", "0x1.000#10", Ceiling, "2.0", "0x2.00#10", Equal);
    test("2", "1.0", "0x1.000#10", Down, "2.0", "0x2.00#10", Equal);
    test("2", "1.0", "0x1.000#10", Up, "2.0", "0x2.00#10", Equal);
    test("2", "1.0", "0x1.000#10", Nearest, "2.0", "0x2.00#10", Equal);
    test("2", "1.0", "0x1.000#10", Exact, "2.0", "0x2.00#10", Equal);

    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        Floor,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        Ceiling,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test("3/2", "1.0", "0x1.000#10", Down, "1.5", "0x1.800#10", Equal);
    test("3/2", "1.0", "0x1.000#10", Up, "1.5", "0x1.800#10", Equal);
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        Nearest,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        Exact,
        "1.5",
        "0x1.800#10",
        Equal,
    );

    test("2", "3.0", "0x3.0#2", Floor, "0.5", "0x0.8#2", Less);
    test("2", "3.0", "0x3.0#2", Ceiling, "0.8", "0x0.c#2", Greater);
    test("2", "3.0", "0x3.0#2", Down, "0.5", "0x0.8#2", Less);
    test("2", "3.0", "0x3.0#2", Up, "0.8", "0x0.c#2", Greater);
    test("2", "3.0", "0x3.0#2", Nearest, "0.8", "0x0.c#2", Greater);

    test("2", "3.0", "0x3.00#10", Floor, "0.666", "0x0.aa8#10", Less);
    test(
        "2",
        "3.0",
        "0x3.00#10",
        Ceiling,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test("2", "3.0", "0x3.00#10", Down, "0.666", "0x0.aa8#10", Less);
    test("2", "3.0", "0x3.00#10", Up, "0.667", "0x0.aac#10", Greater);
    test(
        "2",
        "3.0",
        "0x3.00#10",
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );

    test("3/2", "3.0", "0x3.00#10", Floor, "0.5", "0x0.800#10", Equal);
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        Ceiling,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test("3/2", "3.0", "0x3.00#10", Down, "0.5", "0x0.800#10", Equal);
    test("3/2", "3.0", "0x3.00#10", Up, "0.5", "0x0.800#10", Equal);
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        Nearest,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test("3/2", "3.0", "0x3.00#10", Exact, "0.5", "0x0.800#10", Equal);

    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
        Less,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "0.47746482927568606",
        "0x0.7a3b2292bab314#53",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
        Less,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "0.47746482927568606",
        "0x0.7a3b2292bab314#53",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
        Less,
    );

    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-0.47746482927568606",
        "-0x0.7a3b2292bab314#53",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-0.47746482927568606",
        "-0x0.7a3b2292bab314#53",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
        Greater,
    );

    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "-0.47746482927568606",
        "-0x0.7a3b2292bab314#53",
        Less,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
        Greater,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
        Greater,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "-0.47746482927568606",
        "-0x0.7a3b2292bab314#53",
        Less,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "-0.47746482927568601",
        "-0x0.7a3b2292bab310#53",
        Greater,
    );

    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "0.47746482927568606",
        "0x0.7a3b2292bab314#53",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "0.47746482927568606",
        "0x0.7a3b2292bab314#53",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "0.47746482927568601",
        "0x0.7a3b2292bab310#53",
        Less,
    );
}

#[test]
fn rational_div_float_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::rational_div_float_round(Rational::ONE, THREE, Exact));
    assert_panic!(Float::rational_div_float_round_val_ref(
        Rational::ONE,
        &THREE,
        Exact
    ));
    assert_panic!(Float::rational_div_float_round_ref_val(
        &Rational::ONE,
        THREE,
        Exact
    ));
    assert_panic!(Float::rational_div_float_round_ref_ref(
        &Rational::ONE,
        &THREE,
        Exact
    ));
}

#[test]
fn test_rational_div_float_prec_round() {
    let test = |s, t, t_hex, prec, rm, out: &str, out_hex: &str, o_out| {
        let x = Rational::from_str(s).unwrap();
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (quotient, o) = Float::rational_div_float_prec_round(x.clone(), y.clone(), prec, rm);
        assert!(quotient.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(quotient.to_string(), out);
        assert_eq!(to_hex_string(&quotient), out_hex);

        let (quotient_alt, o_alt) =
            Float::rational_div_float_prec_round_val_ref(x.clone(), &y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            Float::rational_div_float_prec_round_ref_val(&x, y.clone(), prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = Float::rational_div_float_prec_round_ref_ref(&x, &y, prec, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient),
            ComparableFloatRef(&quotient_alt)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_naive_val_ref(x.clone(), &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_naive_ref_val(&x, y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = rational_div_float_prec_round_naive_ref_ref(&x, &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_direct(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_direct_val_ref(x.clone(), &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_direct_ref_val(&x, y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = rational_div_float_prec_round_direct_ref_ref(&x, &y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_rational_div_float_prec_round(
                &rug::Rational::exact_from(&x),
                &rug::Float::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("123", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("123", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test(
        "123", "Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123", "Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123", "Infinity", "Infinity", 1, Down, "0.0", "0x0.0", Equal,
    );
    test("123", "Infinity", "Infinity", 1, Up, "0.0", "0x0.0", Equal);
    test(
        "123", "Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123", "Infinity", "Infinity", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "123",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("0", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("0", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test("0", "Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal);
    test(
        "0", "Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test("0", "Infinity", "Infinity", 1, Down, "0.0", "0x0.0", Equal);
    test("0", "Infinity", "Infinity", 1, Up, "0.0", "0x0.0", Equal);
    test(
        "0", "Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test("0", "Infinity", "Infinity", 1, Exact, "0.0", "0x0.0", Equal);

    test(
        "0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("0", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal);
    test("0", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal);

    test("0", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal);
    test("0", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal);

    test(
        "123", "0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123", "0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123", "0.0", "0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test("123", "0.0", "0x0.0", 1, Up, "Infinity", "Infinity", Equal);
    test(
        "123", "0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123", "0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "123",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-123",
        "0.0",
        "0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-123",
        "0.0",
        "0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-123", "-0.0", "-0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-123", "-0.0", "-0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "1/3", "0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "1/3", "0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "1/3", "0.0", "0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test("1/3", "0.0", "0x0.0", 1, Up, "Infinity", "Infinity", Equal);
    test(
        "1/3", "0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "1/3", "0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1/3",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-1/3",
        "0.0",
        "0x0.0",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1/3",
        "0.0",
        "0x0.0",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-1/3", "-0.0", "-0x0.0", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-1/3", "-0.0", "-0x0.0", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "1", "123.0", "0x7b.0#7", 1, Floor, "0.008", "0x0.02#1", Less,
    );
    test(
        "1", "123.0", "0x7b.0#7", 1, Ceiling, "0.02", "0x0.04#1", Greater,
    );
    test("1", "123.0", "0x7b.0#7", 1, Down, "0.008", "0x0.02#1", Less);
    test("1", "123.0", "0x7b.0#7", 1, Up, "0.02", "0x0.04#1", Greater);
    test(
        "1", "123.0", "0x7b.0#7", 1, Nearest, "0.008", "0x0.02#1", Less,
    );

    test(
        "1",
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "0.00812",
        "0x0.0214#10",
        Less,
    );
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "0.00812",
        "0x0.0214#10",
        Less,
    );
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );
    test(
        "1",
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "0.00813",
        "0x0.0215#10",
        Greater,
    );

    test("0", "123.0", "0x7b.0#7", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", 1, Down, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", 1, Up, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", "123.0", "0x7b.0#7", 1, Exact, "0.0", "0x0.0", Equal);

    test(
        "0",
        "-123.0",
        "-0x7b.0#7",
        1,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-123.0",
        "-0x7b.0#7",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("0", "-123.0", "-0x7b.0#7", 1, Down, "-0.0", "-0x0.0", Equal);
    test("0", "-123.0", "-0x7b.0#7", 1, Up, "-0.0", "-0x0.0", Equal);
    test(
        "0",
        "-123.0",
        "-0x7b.0#7",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "0",
        "-123.0",
        "-0x7b.0#7",
        1,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("2", "1.0", "0x1.0#1", 1, Floor, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", 1, Ceiling, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", 1, Down, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", 1, Up, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", 1, Nearest, "2.0", "0x2.0#1", Equal);
    test("2", "1.0", "0x1.0#1", 1, Exact, "2.0", "0x2.0#1", Equal);

    test("2", "1.0", "0x1.0#1", 10, Floor, "2.0", "0x2.00#10", Equal);
    test(
        "2",
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("2", "1.0", "0x1.0#1", 10, Down, "2.0", "0x2.00#10", Equal);
    test("2", "1.0", "0x1.0#1", 10, Up, "2.0", "0x2.00#10", Equal);
    test(
        "2",
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("2", "1.0", "0x1.0#1", 10, Exact, "2.0", "0x2.00#10", Equal);

    test("3/2", "1.0", "0x1.000#10", 1, Floor, "1.0", "0x1.0#1", Less);
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test("3/2", "1.0", "0x1.000#10", 1, Down, "1.0", "0x1.0#1", Less);
    test("3/2", "1.0", "0x1.000#10", 1, Up, "2.0", "0x2.0#1", Greater);
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );

    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        10,
        Floor,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        10,
        Ceiling,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        10,
        Down,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        10,
        Up,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        10,
        Nearest,
        "1.5",
        "0x1.800#10",
        Equal,
    );
    test(
        "3/2",
        "1.0",
        "0x1.000#10",
        10,
        Exact,
        "1.5",
        "0x1.800#10",
        Equal,
    );

    test("2", "3.0", "0x3.0#2", 1, Floor, "0.5", "0x0.8#1", Less);
    test("2", "3.0", "0x3.0#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("2", "3.0", "0x3.0#2", 1, Down, "0.5", "0x0.8#1", Less);
    test("2", "3.0", "0x3.0#2", 1, Up, "1.0", "0x1.0#1", Greater);
    test("2", "3.0", "0x3.0#2", 1, Nearest, "0.5", "0x0.8#1", Less);

    test(
        "2",
        "3.0",
        "0x3.0#2",
        10,
        Floor,
        "0.666",
        "0x0.aa8#10",
        Less,
    );
    test(
        "2",
        "3.0",
        "0x3.0#2",
        10,
        Ceiling,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test("2", "3.0", "0x3.0#2", 10, Down, "0.666", "0x0.aa8#10", Less);
    test(
        "2",
        "3.0",
        "0x3.0#2",
        10,
        Up,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test(
        "2",
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );

    test("3/2", "3.0", "0x3.00#10", 1, Floor, "0.5", "0x0.8#1", Equal);
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Equal,
    );
    test("3/2", "3.0", "0x3.00#10", 1, Down, "0.5", "0x0.8#1", Equal);
    test("3/2", "3.0", "0x3.00#10", 1, Up, "0.5", "0x0.8#1", Equal);
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Equal,
    );
    test("3/2", "3.0", "0x3.00#10", 1, Exact, "0.5", "0x0.8#1", Equal);

    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        10,
        Floor,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        10,
        Ceiling,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        10,
        Down,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        10,
        Up,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        10,
        Nearest,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        "3/2",
        "3.0",
        "0x3.00#10",
        10,
        Exact,
        "0.5",
        "0x0.800#10",
        Equal,
    );

    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Greater,
    );

    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.4771",
        "0x0.7a2#10",
        Less,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "0.4771",
        "0x0.7a2#10",
        Less,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
    test(
        "3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );

    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-0.5",
        "-0x0.8#1",
        Less,
    );

    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.4771",
        "-0x0.7a2#10",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-0.4771",
        "-0x0.7a2#10",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );

    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "-0.2",
        "-0x0.4#1",
        Greater,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "-0.5",
        "-0x0.8#1",
        Less,
    );
    test(
        "3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-0.5",
        "-0x0.8#1",
        Less,
    );

    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.4771",
        "-0x0.7a2#10",
        Greater,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "-0.4771",
        "-0x0.7a2#10",
        Greater,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );
    test(
        "-3/2",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.4775",
        "-0x0.7a4#10",
        Less,
    );

    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Greater,
    );

    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.4771",
        "0x0.7a2#10",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "0.4771",
        "0x0.7a2#10",
        Less,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
    test(
        "-3/2",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.4775",
        "0x0.7a4#10",
        Greater,
    );
}

#[test]
fn rational_div_float_prec_round_fail() {
    assert_panic!(Float::rational_div_float_prec_round(
        Rational::ONE,
        Float::one_prec(1),
        0,
        Floor
    ));
    assert_panic!(Float::rational_div_float_prec_round(
        Rational::ONE,
        Float::from(3),
        1,
        Exact
    ));
    assert_panic!(Float::rational_div_float_prec_round_val_ref(
        Rational::ONE,
        &Float::from(3),
        1,
        Exact
    ));
    assert_panic!(Float::rational_div_float_prec_round_ref_val(
        &Rational::ONE,
        Float::from(3),
        1,
        Exact
    ));
    assert_panic!(Float::rational_div_float_prec_round_ref_ref(
        &Rational::ONE,
        &Float::from(3),
        1,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn div_prec_round_properties_helper(x: Float, y: Float, prec: u64, rm: RoundingMode) {
    let (quotient, o) = x.clone().div_prec_round(y.clone(), prec, rm);
    assert!(quotient.is_valid());
    let (quotient_alt, o_alt) = x.clone().div_prec_round_val_ref(&y, prec, rm);
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);
    let (quotient_alt, o_alt) = x.div_prec_round_ref_val(y.clone(), prec, rm);
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);
    let (quotient_alt, o_alt) = x.div_prec_round_ref_ref(&y, prec, rm);
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.div_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.div_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
    assert_eq!(o_alt, o);

    let (quotient_alt, o_alt) = div_prec_round_naive(x.clone(), y.clone(), prec, rm);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_quotient, rug_o) = rug_div_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal && quotient.is_finite() && quotient != 0 {
        assert_eq!(
            ComparableFloatRef(
                &quotient
                    .mul_prec_round_ref_ref(&y, x.significant_bits(), Exact)
                    .0
            ),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(
                &x.div_prec_round_ref_ref(&quotient, y.significant_bits(), Exact)
                    .0
            ),
            ComparableFloatRef(&y)
        );
    }

    let r_quotient = if quotient.is_finite() && y.is_finite() {
        if quotient.is_normal() {
            assert_eq!(quotient.get_prec(), Some(prec));
        }
        let r_quotient = Rational::exact_from(&x) / Rational::exact_from(&y);
        assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
        if o == Less {
            let mut next = quotient.clone();
            next.increment();
            assert!(next > r_quotient);
        } else if o == Greater {
            let mut next = quotient.clone();
            next.decrement();
            assert!(next < r_quotient);
        }
        Some(r_quotient)
    } else {
        assert_eq!(o, Equal);
        None
    };

    match (
        r_quotient.is_some() && *r_quotient.as_ref().unwrap() >= 0u32,
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

    let (mut quotient_alt, mut o_alt) = x.div_prec_round_ref_val(-&y, prec, -rm);
    quotient_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(quotient_alt.abs_negative_zero()),
        ComparableFloat(quotient.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    let (mut quotient_alt, mut o_alt) = (-&x).div_prec_round_val_ref(&y, prec, -rm);
    quotient_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(
        ComparableFloat(quotient_alt.abs_negative_zero()),
        ComparableFloat(quotient.abs_negative_zero_ref())
    );
    assert_eq!(o_alt, o);

    let (quotient_alt, o_alt) = (-&x).div_prec_round(-&y, prec, rm);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.div_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(quotient.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.div_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn div_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(|(x, y, prec, rm)| {
        div_prec_round_properties_helper(x, y, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_float_unsigned_rounding_mode_quadruple_gen_var_4().test_properties_with_config(
        &config,
        |(x, y, prec, rm)| {
            div_prec_round_properties_helper(x, y, prec, rm);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 2048);
    float_float_unsigned_rounding_mode_quadruple_gen_var_4().test_properties_with_config(
        &config,
        |(x, y, prec, rm)| {
            div_prec_round_properties_helper(x, y, prec, rm);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (quotient, o) = x.div_prec_round_ref_val(Float::NAN, prec, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        let (quotient, o) = Float::NAN.div_prec_round_val_ref(&x, prec, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x.is_finite() {
                assert_eq!(
                    x.div_prec_round_ref_val(Float::INFINITY, prec, rm),
                    (
                        if x.is_sign_positive() {
                            Float::ZERO
                        } else {
                            Float::NEGATIVE_ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::INFINITY.div_prec_round_val_ref(&x, prec, rm),
                    (
                        if x.is_sign_positive() {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    x.div_prec_round_ref_val(Float::NEGATIVE_INFINITY, prec, rm),
                    (
                        if x.is_sign_positive() {
                            Float::NEGATIVE_ZERO
                        } else {
                            Float::ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.div_prec_round_val_ref(&x, prec, rm),
                    (
                        if x.is_sign_positive() {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
            }
            if x != 0 {
                assert_eq!(
                    x.div_prec_round_ref_val(Float::ZERO, prec, rm),
                    (
                        if x > 0 {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::ZERO.div_prec_round_val_ref(&x, prec, rm),
                    (
                        if x > 0 {
                            Float::ZERO
                        } else {
                            Float::NEGATIVE_ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    x.div_prec_round_ref_val(Float::ZERO, prec, rm),
                    (
                        if x > 0 {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO.div_prec_round_val_ref(&x, prec, rm),
                    (
                        if x > 0 {
                            Float::NEGATIVE_ZERO
                        } else {
                            Float::ZERO
                        },
                        Equal
                    )
                );
            }
        }
        if !x.is_negative_zero() {
            let (quotient, o) = x.div_prec_round_ref_val(Float::ONE, prec, rm);
            let mut quotient_alt = x.clone();
            let o_alt = quotient_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(quotient), ComparableFloat(quotient_alt));
            assert_eq!(o, o_alt);

            if rm != Exact {
                let (quotient, o) = Float::ONE.div_prec_round_val_ref(&x, prec, rm);
                let (quotient_alt, o_alt) = x.clone().reciprocal_prec_round(prec, rm);
                assert_eq!(ComparableFloat(quotient), ComparableFloat(quotient_alt));
                assert_eq!(o, o_alt);
            }
        }
    });
}

fn div_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (quotient, o) = x.clone().div_prec(y.clone(), prec);
    assert!(quotient.is_valid());
    let (quotient_alt, o_alt) = x.clone().div_prec_val_ref(&y, prec);
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);
    let (quotient_alt, o_alt) = x.div_prec_ref_val(y.clone(), prec);
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);
    let (quotient_alt, o_alt) = x.div_prec_ref_ref(&y, prec);
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.div_prec_assign(y.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.div_prec_assign_ref(&y, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
    assert_eq!(o_alt, o);

    let (quotient_alt, o_alt) = div_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    if o == Equal && quotient.is_finite() && quotient != 0 {
        assert_eq!(
            ComparableFloatRef(&quotient.mul_prec_ref_ref(&y, x.significant_bits()).0),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&x.div_prec_ref_ref(&quotient, y.significant_bits()).0),
            ComparableFloatRef(&y)
        );
    }

    let (quotient_alt, o_alt) = x.div_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    if quotient.is_finite() && y.is_finite() {
        if quotient.is_normal() {
            assert_eq!(quotient.get_prec(), Some(prec));
        }
        let r_quotient = Rational::exact_from(&x) / Rational::exact_from(&y);
        assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
        if o == Less {
            let mut next = quotient.clone();
            next.increment();
            assert!(next > r_quotient);
        } else if o == Greater {
            let mut next = quotient.clone();
            next.decrement();
            assert!(next < r_quotient);
        }
    } else {
        assert_eq!(o, Equal);
    }

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        let (mut quotient_alt, mut o_alt) = x.div_prec_ref_val(-&y, prec);
        quotient_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient),
        );
        assert_eq!(o_alt, o);

        let (mut quotient_alt, mut o_alt) = (-&x).div_prec_val_ref(&y, prec);
        quotient_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = (-x).div_prec(-y, prec);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);
    }
}

#[test]
fn div_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        div_prec_properties_helper(x, y, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_float_unsigned_triple_gen_var_1().test_properties_with_config(&config, |(x, y, prec)| {
        div_prec_properties_helper(x, y, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 2048);
    float_float_unsigned_triple_gen_var_1().test_properties_with_config(&config, |(x, y, prec)| {
        div_prec_properties_helper(x, y, prec);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (quotient, o) = x.div_prec_ref_val(Float::NAN, prec);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        let (quotient, o) = Float::NAN.div_prec_val_ref(&x, prec);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x.is_finite() {
                assert_eq!(
                    x.div_prec_ref_val(Float::INFINITY, prec),
                    (
                        if x.is_sign_positive() {
                            Float::ZERO
                        } else {
                            Float::NEGATIVE_ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::INFINITY.div_prec_val_ref(&x, prec),
                    (
                        if x.is_sign_positive() {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    x.div_prec_ref_val(Float::NEGATIVE_INFINITY, prec),
                    (
                        if x.is_sign_positive() {
                            Float::NEGATIVE_ZERO
                        } else {
                            Float::ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.div_prec_val_ref(&x, prec),
                    (
                        if x.is_sign_positive() {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
            }
            if x != 0 {
                assert_eq!(
                    x.div_prec_ref_val(Float::ZERO, prec),
                    (
                        if x > 0 {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::ZERO.div_prec_val_ref(&x, prec),
                    (
                        if x > 0 {
                            Float::ZERO
                        } else {
                            Float::NEGATIVE_ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    x.div_prec_ref_val(Float::NEGATIVE_ZERO, prec),
                    (
                        if x > 0 {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO.div_prec_val_ref(&x, prec),
                    (
                        if x > 0 {
                            Float::NEGATIVE_ZERO
                        } else {
                            Float::ZERO
                        },
                        Equal
                    )
                );
            }
        }
        if !x.is_negative_zero() {
            let (quotient, o) = x.div_prec_ref_val(Float::ONE, prec);
            let mut quotient_alt = x.clone();
            let o_alt = quotient_alt.set_prec(prec);
            assert_eq!(ComparableFloat(quotient), ComparableFloat(quotient_alt));
            assert_eq!(o, o_alt);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn div_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (quotient, o) = x.clone().div_round(y.clone(), rm);
    assert!(quotient.is_valid());
    let (quotient_alt, o_alt) = x.clone().div_round_val_ref(&y, rm);
    assert!(quotient_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let (quotient_alt, o_alt) = x.div_round_ref_val(y.clone(), rm);
    assert!(quotient_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let (quotient_alt, o_alt) = x.div_round_ref_ref(&y, rm);
    assert!(quotient_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.div_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.div_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
    assert_eq!(o_alt, o);

    let (quotient_alt, o_alt) = div_prec_round_naive(
        x.clone(),
        y.clone(),
        max(x.significant_bits(), y.significant_bits()),
        rm,
    );
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);
    let (quotient_alt, o_alt) =
        x.div_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), rm);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    assert_eq!(o_alt, o);

    if o == Equal && quotient.is_finite() && quotient != 0 {
        assert_eq!(quotient.mul_round_ref_ref(&y, Exact).0, x);
        assert_eq!(x.div_round_ref_ref(&quotient, Exact).0, y);
    }

    let r_quotient = if quotient.is_finite() && y.is_finite() {
        if x.is_normal() && y.is_normal() && quotient.is_normal() {
            assert_eq!(
                quotient.get_prec(),
                Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
            );
        }
        let r_quotient = Rational::exact_from(&x) / Rational::exact_from(&y);
        assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
        if o == Less {
            let mut next = quotient.clone();
            next.increment();
            assert!(next > r_quotient);
        } else if o == Greater {
            let mut next = quotient.clone();
            next.decrement();
            assert!(next < r_quotient);
        }
        Some(r_quotient)
    } else {
        assert_eq!(o, Equal);
        None
    };

    match (
        r_quotient.is_some() && *r_quotient.as_ref().unwrap() >= 0u32,
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

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_quotient, rug_o) =
            rug_div_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
        );
        assert_eq!(rug_o, o);
    }

    let (mut quotient_alt, mut o_alt) = x.div_round_ref_val(-&y, -rm);
    quotient_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(quotient_alt.abs_negative_zero()),
        ComparableFloat(quotient.abs_negative_zero_ref())
    );

    let (mut quotient_alt, mut o_alt) = (-&x).div_round_val_ref(&y, -rm);
    quotient_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(quotient_alt.abs_negative_zero()),
        ComparableFloat(quotient.abs_negative_zero_ref())
    );

    let (quotient_alt, o_alt) = (-&x).div_round(-&y, rm);
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.div_round_ref_ref(&y, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(quotient.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.div_round_ref_ref(&y, Exact));
    }
}

#[test]
fn div_round_properties() {
    float_float_rounding_mode_triple_gen_var_23().test_properties(|(x, y, rm)| {
        div_round_properties_helper(x, y, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_float_rounding_mode_triple_gen_var_23().test_properties_with_config(
        &config,
        |(x, y, rm)| {
            div_round_properties_helper(x, y, rm);
        },
    );

    float_float_rounding_mode_triple_gen_var_24().test_properties(|(x, y, rm)| {
        div_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_25().test_properties(|(x, y, rm)| {
        div_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_26().test_properties(|(x, y, rm)| {
        div_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_27().test_properties(|(x, y, rm)| {
        div_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_28().test_properties(|(x, y, rm)| {
        div_round_properties_helper(x, y, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (quotient, o) = x.div_round_ref_val(Float::NAN, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        let (quotient, o) = Float::NAN.div_round_val_ref(&x, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x.is_finite() {
                assert_eq!(
                    x.div_round_ref_val(Float::INFINITY, rm),
                    (
                        if x.is_sign_positive() {
                            Float::ZERO
                        } else {
                            Float::NEGATIVE_ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::INFINITY.div_round_val_ref(&x, rm),
                    (
                        if x.is_sign_positive() {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    x.div_round_ref_val(Float::NEGATIVE_INFINITY, rm),
                    (
                        if x.is_sign_positive() {
                            Float::NEGATIVE_ZERO
                        } else {
                            Float::ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.div_round_val_ref(&x, rm),
                    (
                        if x.is_sign_positive() {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
            }
            if x != 0 {
                assert_eq!(
                    x.div_round_ref_val(Float::ZERO, rm),
                    (
                        if x > 0 {
                            Float::INFINITY
                        } else {
                            Float::NEGATIVE_INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::ZERO.div_round_val_ref(&x, rm),
                    (
                        if x > 0 {
                            Float::ZERO
                        } else {
                            Float::NEGATIVE_ZERO
                        },
                        Equal
                    )
                );
                assert_eq!(
                    x.div_round_ref_val(Float::NEGATIVE_ZERO, rm),
                    (
                        if x > 0 {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO.div_round_val_ref(&x, rm),
                    (
                        if x > 0 {
                            Float::NEGATIVE_ZERO
                        } else {
                            Float::ZERO
                        },
                        Equal
                    )
                );
            }
        }
        if !x.is_negative_zero() {
            let (quotient, o) = x.div_round_ref_val(Float::ONE, rm);
            assert_eq!(ComparableFloatRef(&quotient), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn div_properties_helper_1(x: Float, y: Float) {
    let quotient = x.clone() / y.clone();
    assert!(quotient.is_valid());
    let quotient_alt = x.clone() / &y;
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let quotient_alt = &x / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let quotient_alt = &x / &y;
    assert!(quotient_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );

    let mut x_alt = x.clone();
    x_alt /= y.clone();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));

    let mut x_alt = x.clone();
    x_alt /= &y;
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));

    let quotient_alt = div_prec_round_naive(
        x.clone(),
        y.clone(),
        max(x.significant_bits(), y.significant_bits()),
        Nearest,
    )
    .0;
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let quotient_alt = x
        .div_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let quotient_alt = x
        .div_prec_ref_ref(&y, max(x.significant_bits(), y.significant_bits()))
        .0;
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );
    let (quotient_alt, o) = x.div_round_ref_ref(&y, Nearest);
    assert_eq!(
        ComparableFloatRef(&quotient_alt),
        ComparableFloatRef(&quotient)
    );

    if o == Equal && quotient.is_finite() && quotient != 0 {
        assert_eq!(&quotient * &y, x);
        assert_eq!(&x / &quotient, y);
    }

    if quotient.is_finite() && x.is_normal() && y.is_normal() && quotient.is_normal() {
        assert_eq!(
            quotient.get_prec(),
            Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
        );
        let r_quotient = Rational::exact_from(&x) / Rational::exact_from(&y);
        if quotient < r_quotient {
            let mut next = quotient.clone();
            next.increment();
            assert!(next > r_quotient);
        } else if quotient > r_quotient {
            let mut next = quotient.clone();
            next.decrement();
            assert!(next < r_quotient);
        }
    }

    let rug_quotient = rug_div(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_quotient)),
        ComparableFloatRef(&quotient),
    );

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        assert_eq!(
            ComparableFloatRef(&-(&x / -&y)),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(
            ComparableFloatRef(&-(-&x / &y)),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(
            ComparableFloatRef(&(-&x / -&y)),
            ComparableFloatRef(&quotient)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn div_properties_helper_2<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let quotient_1 = x / y;
        let quotient_2 = emulate_primitive_float_fn_2(|x, y, prec| x.div_prec(y, prec).0, x, y);
        assert_eq!(NiceFloat(quotient_1), NiceFloat(quotient_2));
    });
}

#[test]
fn div_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        div_properties_helper_1(x, y);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_pair_gen().test_properties_with_config(&config, |(x, y)| {
        div_properties_helper_1(x, y);
    });

    float_pair_gen_var_2().test_properties(|(x, y)| {
        div_properties_helper_1(x, y);
    });

    float_pair_gen_var_3().test_properties(|(x, y)| {
        div_properties_helper_1(x, y);
    });

    float_pair_gen_var_4().test_properties(|(x, y)| {
        div_properties_helper_1(x, y);
    });

    float_pair_gen_var_8().test_properties(|(x, y)| {
        div_properties_helper_1(x, y);
    });

    float_pair_gen_var_9().test_properties(|(x, y)| {
        div_properties_helper_1(x, y);
    });

    apply_fn_to_primitive_floats!(div_properties_helper_2);

    float_gen().test_properties(|x| {
        assert!((&x / Float::NAN).is_nan());
        assert!((Float::NAN / &x).is_nan());
        if !x.is_nan() {
            if x.is_finite() {
                assert_eq!(
                    &x / Float::INFINITY,
                    if x.is_sign_positive() {
                        Float::ZERO
                    } else {
                        Float::NEGATIVE_ZERO
                    }
                );
                assert_eq!(
                    Float::INFINITY / &x,
                    if x.is_sign_positive() {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    }
                );
                assert_eq!(
                    &x / Float::NEGATIVE_INFINITY,
                    if x.is_sign_positive() {
                        Float::NEGATIVE_ZERO
                    } else {
                        Float::ZERO
                    }
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY / &x,
                    if x.is_sign_positive() {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    }
                );
            }
            if x != 0 {
                assert_eq!(
                    &x / Float::ZERO,
                    if x.is_sign_positive() {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    }
                );
                assert_eq!(
                    Float::ZERO / &x,
                    if x.is_sign_positive() {
                        Float::ZERO
                    } else {
                        Float::NEGATIVE_ZERO
                    }
                );
                assert_eq!(
                    &x / Float::NEGATIVE_ZERO,
                    if x.is_sign_positive() {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    }
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO / &x,
                    if x.is_sign_positive() {
                        Float::NEGATIVE_ZERO
                    } else {
                        Float::ZERO
                    }
                );
            }
            assert_eq!(&x / Float::ONE, x);
        }
    });
}

#[test]
fn div_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(
        |(x, y, prec, rm)| {
            let (quotient, o) = x.clone().div_rational_prec_round(y.clone(), prec, rm);
            assert!(quotient.is_valid());
            let (quotient_alt, o_alt) = x.clone().div_rational_prec_round_val_ref(&y, prec, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);
            let (quotient_alt, o_alt) = x.div_rational_prec_round_ref_val(y.clone(), prec, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);
            let (quotient_alt, o_alt) = x.div_rational_prec_round_ref_ref(&y, prec, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.div_rational_prec_round_assign(y.clone(), prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.div_rational_prec_round_assign_ref(&y, prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                div_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                div_rational_prec_round_naive_val_ref(x.clone(), &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                div_rational_prec_round_naive_ref_val(&x, y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) = div_rational_prec_round_naive_ref_ref(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                div_rational_prec_round_direct(x.clone(), y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                div_rational_prec_round_direct_val_ref(x.clone(), &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                div_rational_prec_round_direct_ref_val(&x, y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) = div_rational_prec_round_direct_ref_ref(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_quotient, rug_o) = rug_div_rational_prec_round(
                    &rug::Float::exact_from(&x),
                    &rug::Rational::exact_from(&y),
                    prec,
                    rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_quotient)),
                    ComparableFloatRef(&quotient),
                );
                assert_eq!(rug_o, o);
            }

            if o == Equal && quotient.is_finite() && quotient != 0 {
                assert_eq!(
                    ComparableFloatRef(
                        &quotient
                            .mul_rational_prec_round_ref_ref(&y, x.significant_bits(), Exact)
                            .0
                    ),
                    ComparableFloatRef(&x)
                );
                // TODO additional test
            }

            let r_quotient = if quotient.is_finite() {
                if quotient.is_normal() {
                    assert_eq!(quotient.get_prec(), Some(prec));
                }
                let r_quotient = Rational::exact_from(&x) / &y;
                assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
                if o == Less {
                    let mut next = quotient.clone();
                    next.increment();
                    assert!(next > r_quotient);
                } else if o == Greater {
                    let mut next = quotient.clone();
                    next.decrement();
                    assert!(next < r_quotient);
                }
                Some(r_quotient)
            } else {
                assert_eq!(o, Equal);
                None
            };

            match (
                r_quotient.is_some() && *r_quotient.as_ref().unwrap() >= 0u32,
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

            let (mut quotient_alt, mut o_alt) = x.div_rational_prec_round_ref_val(-&y, prec, -rm);
            if y != 0 {
                quotient_alt.neg_assign();
            }
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(quotient_alt.abs_negative_zero()),
                ComparableFloat(quotient.abs_negative_zero_ref()),
            );
            assert_eq!(o_alt, o);

            let (mut quotient_alt, mut o_alt) =
                (-&x).div_rational_prec_round_val_ref(&y, prec, -rm);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(quotient_alt.abs_negative_zero()),
                ComparableFloat(quotient.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);

            if quotient != 0u32 && y != 0 {
                let (quotient_alt, o_alt) = (-&x).div_rational_prec_round(-&y, prec, rm);
                assert_eq!(
                    ComparableFloatRef(&quotient_alt),
                    ComparableFloatRef(&quotient)
                );
                assert_eq!(o_alt, o);
            }

            if o == Equal {
                for rm in exhaustive_rounding_modes() {
                    let (s, oo) = x.div_rational_prec_round_ref_ref(&y, prec, rm);
                    assert_eq!(
                        ComparableFloat(s.abs_negative_zero_ref()),
                        ComparableFloat(quotient.abs_negative_zero_ref())
                    );
                    assert_eq!(oo, Equal);
                }
            } else {
                assert_panic!(x.div_rational_prec_round_ref_ref(&y, prec, Exact));
            }
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_negative_zero() {
            let (quotient, o) = x.div_rational_prec_round_ref_val(Rational::ONE, prec, rm);
            let mut quotient_alt = x.clone();
            let o_alt = quotient_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(quotient), ComparableFloat(quotient_alt));
            assert_eq!(o, o_alt);
        }
        if !x.is_nan() && x != 0 {
            assert_eq!(
                x.div_rational_prec_round_ref_val(Rational::ZERO, prec, rm),
                (
                    if x > 0 {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    },
                    Equal
                )
            );
        }
    });

    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (quotient, o) = Float::NAN.div_rational_prec_round_val_ref(&x, prec, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(
            Float::INFINITY.div_rational_prec_round_val_ref(&x, prec, rm),
            (
                if x >= 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                },
                Equal
            )
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.div_rational_prec_round_val_ref(&x, prec, rm),
            (
                if x >= 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                },
                Equal
            )
        );
        if x != 0 {
            let (quotient, o) = Float::ZERO.div_rational_prec_round_val_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                })
            );
            assert_eq!(o, Equal);

            let (quotient, o) = Float::NEGATIVE_ZERO.div_rational_prec_round_val_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                })
            );
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn div_rational_prec_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (quotient, o) = x.clone().div_rational_prec(y.clone(), prec);
        assert!(quotient.is_valid());
        let (quotient_alt, o_alt) = x.clone().div_rational_prec_val_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);
        let (quotient_alt, o_alt) = x.div_rational_prec_ref_val(y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);
        let (quotient_alt, o_alt) = x.div_rational_prec_ref_ref(&y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.div_rational_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.div_rational_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_direct(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (rug_quotient, rug_o) = rug_div_rational_prec(
            &rug::Float::exact_from(&x),
            &rug::Rational::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
        );
        assert_eq!(rug_o, o);

        if o == Equal && quotient.is_finite() && quotient != 0 {
            assert_eq!(
                ComparableFloatRef(
                    &quotient
                        .mul_rational_prec_ref_ref(&y, x.significant_bits())
                        .0
                ),
                ComparableFloatRef(&x)
            );
            // TODO additional test
        }

        let (quotient_alt, o_alt) = x.div_rational_prec_round_ref_ref(&y, prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if quotient.is_finite() {
            if quotient.is_normal() {
                assert_eq!(quotient.get_prec(), Some(prec));
            }
            let r_quotient = Rational::exact_from(&x) / &y;
            assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
            if o == Less {
                let mut next = quotient.clone();
                next.increment();
                assert!(next > r_quotient);
            } else if o == Greater {
                let mut next = quotient.clone();
                next.decrement();
                assert!(next < r_quotient);
            }
        } else {
            assert_eq!(o, Equal);
        }

        if x != 0u32 && y != 0u32 {
            let (mut quotient_alt, mut o_alt) = x.div_rational_prec_ref_val(-&y, prec);
            quotient_alt.neg_assign();
            quotient_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (mut quotient_alt, mut o_alt) = (-&x).div_rational_prec_val_ref(&y, prec);
            quotient_alt.neg_assign();
            quotient_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) = (-x).div_rational_prec(-y, prec);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if !x.is_negative_zero() {
            let (quotient, o) = x.div_rational_prec_ref_val(Rational::ONE, prec);
            let mut quotient_alt = x.clone();
            let o_alt = quotient_alt.set_prec(prec);
            assert_eq!(ComparableFloat(quotient), ComparableFloat(quotient_alt));
            assert_eq!(o, o_alt);
        }

        if x.is_finite() && x != 0 {
            let (quotient, o) = x.div_rational_prec_ref_val(Rational::ZERO, prec);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                })
            );
            assert_eq!(o, Equal);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (quotient, o) = Float::NAN.div_rational_prec_val_ref(&x, prec);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::INFINITY.div_rational_prec_val_ref(&x, prec),
                (
                    if x >= 0 {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    },
                    Equal
                )
            );
            assert_eq!(
                Float::NEGATIVE_INFINITY.div_rational_prec_val_ref(&x, prec),
                (
                    if x >= 0 {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    },
                    Equal
                )
            );
        }

        if x != 0 {
            let (quotient, o) = Float::ZERO.div_rational_prec_val_ref(&x, prec);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                })
            );
            assert_eq!(o, Equal);
            let (quotient, o) = Float::NEGATIVE_ZERO.div_rational_prec_val_ref(&x, prec);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                })
            );
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn div_rational_round_properties() {
    float_rational_rounding_mode_triple_gen_var_5().test_properties(|(x, y, rm)| {
        let (quotient, o) = x.clone().div_rational_round(y.clone(), rm);
        assert!(quotient.is_valid());
        let (quotient_alt, o_alt) = x.clone().div_rational_round_val_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let (quotient_alt, o_alt) = x.div_rational_round_ref_val(y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let (quotient_alt, o_alt) = x.div_rational_round_ref_ref(&y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let mut x_alt = x.clone();
        let o_alt = x_alt.div_rational_round_assign(y.clone(), rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.div_rational_round_assign_ref(&y, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            div_rational_prec_round_direct(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) = x.div_rational_prec_round_ref_ref(&y, x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if o == Equal && quotient.is_finite() && quotient != 0 {
            assert_eq!(quotient.mul_rational_round_ref_ref(&y, Exact).0, x);
            // TODO additional test
        }

        let r_quotient = if quotient.is_finite() {
            if x.is_normal() && quotient.is_normal() {
                assert_eq!(quotient.get_prec(), Some(x.get_prec().unwrap()));
            }
            let r_quotient = Rational::exact_from(&x) / &y;
            assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
            if o == Less {
                let mut next = quotient.clone();
                next.increment();
                assert!(next > r_quotient);
            } else if o == Greater {
                let mut next = quotient.clone();
                next.decrement();
                assert!(next < r_quotient);
            }
            Some(r_quotient)
        } else {
            assert_eq!(o, Equal);
            None
        };

        match (
            r_quotient.is_some() && *r_quotient.as_ref().unwrap() >= 0u32,
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

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_div_rational_round(
                &rug::Float::exact_from(&x),
                &rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient),
            );
            assert_eq!(rug_o, o);
        }

        if y != 0 {
            let (mut quotient_alt, mut o_alt) = x.div_rational_round_ref_val(-&y, -rm);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(o_alt, o);
            assert_eq!(
                ComparableFloat(quotient_alt.abs_negative_zero_ref()),
                ComparableFloat(quotient.abs_negative_zero_ref())
            );
        }

        let (mut quotient_alt, mut o_alt) = (-&x).div_rational_round_val_ref(&y, -rm);
        quotient_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(quotient_alt.abs_negative_zero_ref()),
            ComparableFloat(quotient.abs_negative_zero_ref())
        );

        if x != 0 && y != 0 {
            let (quotient_alt, o_alt) = (-&x).div_rational_round(-&y, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient),
            );
            assert_eq!(o_alt, o);
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = x.div_rational_round_ref_ref(&y, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(quotient.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(x.div_rational_round_ref_ref(&y, Exact));
        }
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (quotient, o) = x.div_rational_round_ref_val(Rational::ONE, rm);
        assert_eq!(ComparableFloatRef(&quotient), ComparableFloatRef(&x));
        assert_eq!(o, Equal);

        if x.is_finite() && x != 0 {
            let (quotient, o) = x.div_rational_round_ref_val(Rational::ZERO, rm);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                })
            );
            assert_eq!(o, Equal);
        }
    });

    rational_rounding_mode_pair_gen_var_6().test_properties(|(x, rm)| {
        let (quotient, o) = Float::NAN.div_rational_round_val_ref(&x, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::INFINITY.div_rational_round_val_ref(&x, rm),
                (
                    if x >= 0 {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    },
                    Equal
                )
            );
            assert_eq!(
                Float::NEGATIVE_INFINITY.div_rational_round_val_ref(&x, rm),
                (
                    if x >= 0 {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    },
                    Equal
                )
            );
        }
    });
}

#[test]
fn div_rational_properties() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        let quotient = x.clone() / y.clone();
        assert!(quotient.is_valid());
        let quotient_alt = x.clone() / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = &x / y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = &x / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let mut x_alt = x.clone();
        x_alt /= y.clone();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));

        let mut x_alt = x.clone();
        x_alt /= &y;
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&quotient));

        let quotient_alt =
            div_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt =
            div_rational_prec_round_direct(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt = x
            .div_rational_prec_round_ref_ref(&y, x.significant_bits(), Nearest)
            .0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt = x.div_rational_prec_ref_ref(&y, x.significant_bits()).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let (quotient_alt, o) = x.div_rational_round_ref_ref(&y, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        if o == Equal && quotient.is_finite() && quotient != 0 {
            assert_eq!(&quotient * &y, x);
            // TODO additional test
        }

        if quotient.is_finite() && x.is_normal() && quotient.is_normal() {
            assert_eq!(quotient.get_prec(), Some(x.get_prec().unwrap()));
            let r_quotient = Rational::exact_from(&x) / &y;
            if quotient < r_quotient {
                let mut next = quotient.clone();
                next.increment();
                assert!(next > r_quotient);
            } else if quotient > r_quotient {
                let mut next = quotient.clone();
                next.decrement();
                assert!(next < r_quotient);
            }
        }

        let rug_quotient = rug_div_rational(&rug::Float::exact_from(&x), &rug::Rational::from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
        );

        if quotient != 0u32 {
            assert_eq!(
                ComparableFloatRef(&-(-&x / &y)),
                ComparableFloatRef(&quotient)
            );
            if y != 0 {
                assert_eq!(
                    ComparableFloatRef(&-(&x / -&y)),
                    ComparableFloatRef(&quotient)
                );
                assert_eq!(
                    ComparableFloatRef(&(-x / -y)),
                    ComparableFloatRef(&quotient)
                );
            }
        }
    });

    float_gen().test_properties(|x| {
        assert_eq!(
            ComparableFloatRef(&(&x / Rational::ONE)),
            ComparableFloatRef(&x)
        );
        if x.is_finite() && x != 0 {
            assert_eq!(
                ComparableFloat(&x / Rational::ZERO),
                ComparableFloat(if x.is_sign_positive() {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                }),
            );
        }
    });

    rational_gen().test_properties(|x| {
        assert!((&x / Float::NAN).is_nan());
        assert_eq!(
            &x / Float::INFINITY,
            if x >= 0 {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            }
        );
        assert_eq!(
            &x / Float::NEGATIVE_INFINITY,
            if x >= 0 {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            }
        );
        if x != 0 {
            assert_eq!(
                &x / Float::ZERO,
                if x > 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                }
            );
            assert_eq!(
                &x / Float::NEGATIVE_ZERO,
                if x > 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                }
            );
        }
    });
}

#[test]
fn rational_div_float_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(y, x, prec, rm)| {
            let (quotient, o) =
                Float::rational_div_float_prec_round(x.clone(), y.clone(), prec, rm);
            assert!(quotient.is_valid());
            let (quotient_alt, o_alt) =
                Float::rational_div_float_prec_round_val_ref(x.clone(), &y, prec, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);
            let (quotient_alt, o_alt) =
                Float::rational_div_float_prec_round_ref_val(&x, y.clone(), prec, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);
            let (quotient_alt, o_alt) =
                Float::rational_div_float_prec_round_ref_ref(&x, &y, prec, rm);
            assert!(quotient_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_naive(x.clone(), y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_naive_val_ref(x.clone(), &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_naive_ref_val(&x, y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_naive_ref_ref(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_direct(x.clone(), y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_direct_val_ref(x.clone(), &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_direct_ref_val(&x, y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) =
                rational_div_float_prec_round_direct_ref_ref(&x, &y, prec, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_quotient, rug_o) = rug_rational_div_float_prec_round(
                    &rug::Rational::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    prec,
                    rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_quotient)),
                    ComparableFloatRef(&quotient),
                );
                assert_eq!(rug_o, o);
            }

            let r_quotient = if quotient.is_finite() && y.is_finite() {
                if quotient.is_normal() {
                    assert_eq!(quotient.get_prec(), Some(prec));
                }
                let r_quotient = &x / Rational::exact_from(&y);
                assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
                if o == Less {
                    let mut next = quotient.clone();
                    next.increment();
                    assert!(next > r_quotient);
                } else if o == Greater {
                    let mut next = quotient.clone();
                    next.decrement();
                    assert!(next < r_quotient);
                }
                Some(r_quotient)
            } else {
                assert_eq!(o, Equal);
                None
            };

            match (
                r_quotient.is_some() && *r_quotient.as_ref().unwrap() >= 0u32,
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

            let (mut quotient_alt, mut o_alt) =
                Float::rational_div_float_prec_round_ref_val(&x, -&y, prec, -rm);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(quotient_alt.abs_negative_zero()),
                ComparableFloat(quotient.abs_negative_zero_ref()),
            );
            assert_eq!(o_alt, o);

            let (mut quotient_alt, mut o_alt) =
                Float::rational_div_float_prec_round_val_ref(-&x, &y, prec, -rm);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(quotient_alt.abs_negative_zero()),
                ComparableFloat(quotient.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);

            if quotient != 0u32 && y != 0u32 {
                let (quotient_alt, o_alt) =
                    Float::rational_div_float_prec_round(-&x, -&y, prec, rm);
                assert_eq!(
                    ComparableFloatRef(&quotient_alt),
                    ComparableFloatRef(&quotient)
                );
                assert_eq!(o_alt, o);
            }

            if o == Equal {
                for rm in exhaustive_rounding_modes() {
                    let (s, oo) = Float::rational_div_float_prec_round_ref_ref(&x, &y, prec, rm);
                    assert_eq!(
                        ComparableFloat(s.abs_negative_zero_ref()),
                        ComparableFloat(quotient.abs_negative_zero_ref())
                    );
                    assert_eq!(oo, Equal);
                }
            } else {
                assert_panic!(Float::rational_div_float_prec_round_ref_ref(
                    &x, &y, prec, Exact
                ));
            }
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_nan() && x != 0 {
            assert_eq!(
                Float::rational_div_float_prec_round_val_ref(Rational::ZERO, &x, prec, rm),
                (
                    if x > 0 {
                        Float::ZERO
                    } else {
                        Float::NEGATIVE_ZERO
                    },
                    Equal
                )
            );
        }
    });

    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (quotient, o) = Float::rational_div_float_prec_round_ref_val(&x, Float::NAN, prec, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(
            Float::rational_div_float_prec_round_ref_val(&x, Float::INFINITY, prec, rm),
            (
                if x >= 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                },
                Equal
            )
        );
        assert_eq!(
            Float::rational_div_float_prec_round_ref_val(&x, Float::NEGATIVE_INFINITY, prec, rm),
            (
                if x >= 0 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                },
                Equal
            )
        );
        if x != 0 {
            let (quotient, o) =
                Float::rational_div_float_prec_round_ref_val(&x, Float::ZERO, prec, rm);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                })
            );
            assert_eq!(o, Equal);

            let (quotient, o) =
                Float::rational_div_float_prec_round_ref_val(&x, Float::NEGATIVE_ZERO, prec, rm);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                })
            );
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn rational_div_float_prec_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(y, x, prec)| {
        let (quotient, o) = Float::rational_div_float_prec(x.clone(), y.clone(), prec);
        assert!(quotient.is_valid());
        let (quotient_alt, o_alt) = Float::rational_div_float_prec_val_ref(x.clone(), &y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);
        let (quotient_alt, o_alt) = Float::rational_div_float_prec_ref_val(&x, y.clone(), prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);
        let (quotient_alt, o_alt) = Float::rational_div_float_prec_ref_ref(&x, &y, prec);
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_direct(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (rug_quotient, rug_o) = rug_rational_div_float_prec(
            &rug::Rational::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
        );
        assert_eq!(rug_o, o);

        let (quotient_alt, o_alt) =
            Float::rational_div_float_prec_round_ref_ref(&x, &y, prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if quotient.is_finite() && y.is_finite() {
            if quotient.is_normal() {
                assert_eq!(quotient.get_prec(), Some(prec));
            }
            let r_quotient = &x / Rational::exact_from(&y);
            assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
            if o == Less {
                let mut next = quotient.clone();
                next.increment();
                assert!(next > r_quotient);
            } else if o == Greater {
                let mut next = quotient.clone();
                next.decrement();
                assert!(next < r_quotient);
            }
        } else {
            assert_eq!(o, Equal);
        }

        if x != 0u32 && y != 0u32 {
            let (mut quotient_alt, mut o_alt) =
                Float::rational_div_float_prec_ref_val(&x, -&y, prec);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (mut quotient_alt, mut o_alt) =
                Float::rational_div_float_prec_val_ref(-&x, &y, prec);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);

            let (quotient_alt, o_alt) = Float::rational_div_float_prec(-x, -y, prec);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient)
            );
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if x.is_finite() && x != 0 {
            let (quotient, o) = Float::rational_div_float_prec_val_ref(Rational::ZERO, &x, prec);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                })
            );
            assert_eq!(o, Equal);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (quotient, o) = Float::rational_div_float_prec_ref_val(&x, Float::NAN, prec);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::rational_div_float_prec_ref_val(&x, Float::INFINITY, prec),
                (
                    if x >= 0 {
                        Float::ZERO
                    } else {
                        Float::NEGATIVE_ZERO
                    },
                    Equal
                )
            );
            assert_eq!(
                Float::rational_div_float_prec_ref_val(&x, Float::NEGATIVE_INFINITY, prec),
                (
                    if x >= 0 {
                        Float::NEGATIVE_ZERO
                    } else {
                        Float::ZERO
                    },
                    Equal
                )
            );
        }

        if x != 0 {
            let (quotient, o) = Float::rational_div_float_prec_ref_val(&x, Float::ZERO, prec);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                })
            );
            assert_eq!(o, Equal);
            let (quotient, o) =
                Float::rational_div_float_prec_ref_val(&x, Float::NEGATIVE_ZERO, prec);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                })
            );
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn rational_div_float_round_properties() {
    float_rational_rounding_mode_triple_gen_var_6().test_properties(|(y, x, rm)| {
        let (quotient, o) = Float::rational_div_float_round(x.clone(), y.clone(), rm);
        assert!(quotient.is_valid());
        let (quotient_alt, o_alt) = Float::rational_div_float_round_val_ref(x.clone(), &y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let (quotient_alt, o_alt) = Float::rational_div_float_round_ref_val(&x, y.clone(), rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let (quotient_alt, o_alt) = Float::rational_div_float_round_ref_ref(&x, &y, rm);
        assert!(quotient_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_naive(x.clone(), y.clone(), y.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let (quotient_alt, o_alt) =
            rational_div_float_prec_round_direct(x.clone(), y.clone(), y.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_quotient, rug_o) = rug_rational_div_float_round(
                &rug::Rational::exact_from(&x),
                &rug::Float::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_quotient)),
                ComparableFloatRef(&quotient),
            );
            assert_eq!(rug_o, o);
        }

        let (quotient_alt, o_alt) =
            Float::rational_div_float_prec_round_ref_ref(&x, &y, y.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        assert_eq!(o_alt, o);

        let r_quotient = if quotient.is_finite() && y.is_finite() {
            if y.is_normal() && quotient.is_normal() {
                assert_eq!(quotient.get_prec(), Some(y.get_prec().unwrap()));
            }
            let r_quotient = &x / Rational::exact_from(&y);
            assert_eq!(quotient.partial_cmp(&r_quotient), Some(o));
            if o == Less {
                let mut next = quotient.clone();
                next.increment();
                assert!(next > r_quotient);
            } else if o == Greater {
                let mut next = quotient.clone();
                next.decrement();
                assert!(next < r_quotient);
            }
            Some(r_quotient)
        } else {
            assert_eq!(o, Equal);
            None
        };

        match (
            r_quotient.is_some() && *r_quotient.as_ref().unwrap() >= 0u32,
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

        if y != 0 {
            let (mut quotient_alt, mut o_alt) =
                Float::rational_div_float_round_ref_val(&x, -&y, -rm);
            quotient_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(o_alt, o);
            assert_eq!(
                ComparableFloat(quotient_alt.abs_negative_zero_ref()),
                ComparableFloat(quotient.abs_negative_zero_ref())
            );
        }

        let (mut quotient_alt, mut o_alt) = Float::rational_div_float_round_val_ref(-&x, &y, -rm);
        quotient_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(quotient_alt.abs_negative_zero_ref()),
            ComparableFloat(quotient.abs_negative_zero_ref())
        );

        if x != 0 && y != 0 {
            let (quotient_alt, o_alt) = Float::rational_div_float_round(-&x, -&y, rm);
            assert_eq!(
                ComparableFloatRef(&quotient_alt),
                ComparableFloatRef(&quotient),
            );
            assert_eq!(o_alt, o);
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = Float::rational_div_float_round_ref_ref(&x, &y, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(quotient.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(Float::rational_div_float_round_ref_ref(&x, &y, Exact));
        }
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        if x.is_finite() && x != 0 {
            let (quotient, o) = Float::rational_div_float_round_val_ref(Rational::ZERO, &x, rm);
            assert_eq!(
                ComparableFloat(quotient),
                ComparableFloat(if x >= 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                })
            );
            assert_eq!(o, Equal);
        }
    });

    rational_rounding_mode_pair_gen_var_6().test_properties(|(x, rm)| {
        let (quotient, o) = Float::rational_div_float_round_ref_val(&x, Float::NAN, rm);
        assert!(quotient.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::rational_div_float_round_ref_val(&x, Float::INFINITY, rm),
                (
                    if x >= 0 {
                        Float::ZERO
                    } else {
                        Float::NEGATIVE_ZERO
                    },
                    Equal
                )
            );
            assert_eq!(
                Float::rational_div_float_round_ref_val(&x, Float::NEGATIVE_INFINITY, rm),
                (
                    if x >= 0 {
                        Float::NEGATIVE_ZERO
                    } else {
                        Float::ZERO
                    },
                    Equal
                )
            );
        }
    });
}

#[test]
fn rational_div_float_properties() {
    float_rational_pair_gen().test_properties(|(y, x)| {
        let quotient = x.clone() / y.clone();
        assert!(quotient.is_valid());
        let quotient_alt = x.clone() / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = &x / y.clone();
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = &x / &y;
        assert!(quotient_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt = rational_div_float_prec_round_naive(
            x.clone(),
            y.clone(),
            y.significant_bits(),
            Nearest,
        )
        .0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = rational_div_float_prec_round_direct(
            x.clone(),
            y.clone(),
            y.significant_bits(),
            Nearest,
        )
        .0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        let quotient_alt =
            Float::rational_div_float_prec_round_ref_ref(&x, &y, y.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = Float::rational_div_float_prec_ref_ref(&x, &y, y.significant_bits()).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );
        let quotient_alt = Float::rational_div_float_round_ref_ref(&x, &y, Nearest).0;
        assert_eq!(
            ComparableFloatRef(&quotient_alt),
            ComparableFloatRef(&quotient)
        );

        if quotient.is_finite() && y.is_normal() && quotient.is_normal() {
            assert_eq!(quotient.get_prec(), Some(y.get_prec().unwrap()));
            let r_quotient = &x / Rational::exact_from(&y);
            if quotient < r_quotient {
                let mut next = quotient.clone();
                next.increment();
                assert!(next > r_quotient);
            } else if quotient > r_quotient {
                let mut next = quotient.clone();
                next.decrement();
                assert!(next < r_quotient);
            }
        }

        let rug_quotient =
            rug_rational_div_float(&rug::Rational::from(&x), &rug::Float::exact_from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_quotient)),
            ComparableFloatRef(&quotient),
        );

        if quotient != 0u32 {
            assert_eq!(
                ComparableFloatRef(&-(-&x / &y)),
                ComparableFloatRef(&quotient)
            );
            if y != 0 {
                assert_eq!(
                    ComparableFloatRef(&-(&x / -&y)),
                    ComparableFloatRef(&quotient)
                );
                assert_eq!(
                    ComparableFloatRef(&(-x / -y)),
                    ComparableFloatRef(&quotient)
                );
            }
        }
    });

    float_gen().test_properties(|x| {
        if x.is_finite() && x != 0 {
            assert_eq!(
                ComparableFloat(Rational::ZERO / &x),
                ComparableFloat(if x.is_sign_positive() {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                }),
            );
            assert_eq!(
                ComparableFloat(Rational::ZERO / &x),
                ComparableFloat(if x.is_sign_positive() {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                })
            );
        }
    });

    rational_gen().test_properties(|x| {
        assert!((Float::NAN / &x).is_nan());
        assert_eq!(
            Float::INFINITY / &x,
            if x >= 0 {
                Float::INFINITY
            } else {
                Float::NEGATIVE_INFINITY
            }
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY / &x,
            if x >= 0 {
                Float::NEGATIVE_INFINITY
            } else {
                Float::INFINITY
            }
        );
        if x != 0 {
            assert_eq!(
                Float::ZERO / &x,
                if x > 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                }
            );
            assert_eq!(
                Float::NEGATIVE_ZERO / &x,
                if x > 0 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                }
            );
        }
        let quotient_alt = Float::from_rational_prec_ref(&x, 1).0;
        assert_eq!(
            ComparableFloat(&x / Float::ONE),
            ComparableFloat(quotient_alt.clone())
        );
    });
}
