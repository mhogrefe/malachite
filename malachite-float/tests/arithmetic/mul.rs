// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::{
    max,
    Ordering::{self, *},
};
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
use malachite_float::arithmetic::mul::mul_rational_prec_round_naive;
use malachite_float::test_util::arithmetic::mul::{
    mul_prec_round_naive, rug_mul, rug_mul_rational, rug_mul_rational_round, rug_mul_round,
};
use malachite_float::test_util::common::{
    emulate_primitive_float_fn_2, parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_16, float_float_rounding_mode_triple_gen_var_17,
    float_float_rounding_mode_triple_gen_var_18, float_float_rounding_mode_triple_gen_var_19,
    float_float_rounding_mode_triple_gen_var_20, float_float_rounding_mode_triple_gen_var_21,
    float_float_rounding_mode_triple_gen_var_22,
    float_float_unsigned_rounding_mode_quadruple_gen_var_3, float_float_unsigned_triple_gen_var_1,
    float_gen, float_pair_gen, float_pair_gen_var_2, float_pair_gen_var_3, float_pair_gen_var_4,
    float_pair_gen_var_5, float_pair_gen_var_6, float_pair_gen_var_7, float_rational_pair_gen,
    float_rational_rounding_mode_triple_gen_var_4,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_3,
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
fn test_mul() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let product = x.clone() * y.clone();
        assert!(product.is_valid());

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);

        let product_alt = x.clone() * &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = &x * y.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = &x * &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        let mut product_alt = x.clone();
        product_alt *= y.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let mut product_alt = x.clone();
        product_alt *= &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_mul(
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&product),
            "{:#x} {:#x}",
            ComparableFloatRef(&Float::from(&rug_mul(
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&product)
        );

        let product_alt = mul_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            Nearest,
        )
        .0;
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
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
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("Infinity", "Infinity", "0.0", "0x0.0", "NaN", "NaN");
    test("Infinity", "Infinity", "-0.0", "-0x0.0", "NaN", "NaN");
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
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test("-Infinity", "-Infinity", "0.0", "0x0.0", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-0.0", "-0x0.0", "NaN", "NaN");
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
    test("0.0", "0x0.0", "Infinity", "Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "1.0", "0x1.0#1", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-1.0", "-0x1.0#1", "-0.0", "-0x0.0");

    test("-0.0", "-0x0.0", "NaN", "NaN", "NaN", "NaN");
    test("-0.0", "-0x0.0", "Infinity", "Infinity", "NaN", "NaN");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity", "NaN", "NaN");
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "1.0", "0x1.0#1", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-1.0", "-0x1.0#1", "0.0", "0x0.0");

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
    test("123.0", "0x7b.0#7", "0.0", "0x0.0", "0.0", "0x0.0");
    test("123.0", "0x7b.0#7", "-0.0", "-0x0.0", "-0.0", "-0x0.0");
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
    test("1.0", "0x1.0#1", "123.0", "0x7b.0#7", "123.0", "0x7b.0#7");
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        "-123.0",
        "-0x7b.0#7",
    );

    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "2.0", "0x2.0#1");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "2.0", "0x2.0#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#1", "2.0", "0x2.0#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#2", "2.0", "0x2.0#2");
    test("1.0", "0x1.000#10", "2.0", "0x2.00#10", "2.0", "0x2.00#10");

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
    );

    // yyy

    // - in mul_float_significands_same_prec_lt_w
    // - decrement_exp in mul_float_significands_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", "1.0", "0x1.0#1");
    // - !decrement_exp in mul_float_significands_same_prec_lt_w
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_lt_w
    // - rm == Nearest in mul_float_significands_same_prec_lt_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (product & shift_bit) == 0)) in
    //   mul_float_significands_same_prec_lt_w
    test("1.5", "0x1.8#2", "1.5", "0x1.8#2", "2.0", "0x2.0#2");
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (product & shift_bit) != 0) &&
    //   product.overflowing_add_assign(shift_bit) in mul_float_significands_same_prec_lt_w
    test("1.2", "0x1.4#3", "1.5", "0x1.8#3", "2.0", "0x2.0#3");
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (product & shift_bit) != 0) &&
    //   !product.overflowing_add_assign(shift_bit) in mul_float_significands_same_prec_lt_w
    test("1.2", "0x1.4#4", "1.4", "0x1.6#4", "1.8", "0x1.c#4");

    // - in mul_float_significands_same_prec_w
    // - decrement_exp in mul_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
    );
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_w
    // - rm == Nearest in mul_float_significands_same_prec_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && product.even())) in
    //   mul_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || product.even()) &&
    //   !product.overflowing_add_assign(1) in mul_float_significands_same_prec_w
    test(
        "3.2729513077064011786e-37",
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        "7.8519772600462495573e-34",
        "0x4.13b4f0d218450fb0E-28#64",
        "2.569913924134929736e-70",
        "0x1.c610823e5a4c0774E-58#64",
    );
    // - !decrement_exp in mul_float_significands_same_prec_w
    test(
        "3116635254961129.0696",
        "0xb1290314433e9.11d#64",
        "7.092177112370390978e-10",
        "0x3.0bcb09ebbb50e418E-8#64",
        "2210372.9222841977617",
        "0x21ba44.ec1ad133010#64",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || product.even()) &&
    //   product.overflowing_add_assign(1) in mul_float_significands_same_prec_w
    test(
        "1.9999999999999999998",
        "0x1.fffffffffffffffc#64",
        "2.0000000000000000002",
        "0x2.0000000000000004#64",
        "4.0",
        "0x4.0000000000000000#64",
    );
    // - in mul_float_significands_same_prec_gt_w_lt_2w
    // - l.wrapping_add(2) & (mask >> 2) <= 2 in mul_float_significands_same_prec_gt_w_lt_2w
    // - decrement_exp in mul_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
    );
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in mul_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (z_0 & shift_bit) == 0)) in
    //   mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
    );
    // - l.wrapping_add(2) & (mask >> 2) > 2 in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "1.00000000000000000022",
        "0x1.0000000000000004#65",
    );
    // - !decrement_exp in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.0001507756106295330606262754053",
        "0x0.0009e19851127b95dcf03f0cdc#91",
        "3458.565842843038054059107814",
        "0xd82.90db1399862ba513faf8#91",
        "0.5214673768571047372764465276",
        "0x0.857ee2d1883c6e783c18b1e#91",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && !overflow
    //   in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "119368.6474438890389479272222539538",
        "0x1d248.a5bee1f96ad66a5061314f7#109",
        "1.573235366444334767515689608501749e-6",
        "0x0.00001a64fe94215b4ea1a015983c92bc#109",
        "0.1877949778033513768632732912065661",
        "0x0.301354e804b87d40aa80cfe1472a#109",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && overflow
    //   in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "295147905179352825855.999999999999999997",
        "0xfffffffffffffffff.ffffffffffffffc#127",
        "0.000244140625000000000000000000000000000003",
        "0x0.00100000000000000000000000000000004#127",
        "72057594037927936.0",
        "0x100000000000000.000000000000000000#127",
    );
    // - in mul_float_significands_same_prec_gt_2w_lt_3w
    // - a0.wrapping_add(4) & (mask >> 2) <= 4 in mul_float_significands_same_prec_gt_2w_lt_3w
    // - decrement_exp in mul_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
    );
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest in mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && z_0 & shift_bit == 0)) in
    //   mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
    );
    // - a0.wrapping_add(4) & (mask >> 2) > 4 in mul_float_significands_same_prec_gt_2w_lt_3w
    // - !decrement_exp in mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) in
    //   mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && !overflow
    //   in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        "0.00497732823382322348141815797577421539704054126",
        "0x0.014631b5fc58aeb12d61fe8ebe2fa3511f34a8b#149",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && overflow in
    //   mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.9999999999999999999999999999999999999993",
        "0x1.ffffffffffffffffffffffffffffffffc#132",
        "2.0000000000000000000000000000000000000007",
        "0x2.000000000000000000000000000000004#132",
        "4.0",
        "0x4.000000000000000000000000000000000#132",
    );
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", "2.0", "0x2.0#3");

    // - in mul_float_significands_general
    // - xs_len <= 2 in mul_float_significands_general
    // - xs_len == 1 in mul_float_significands_general
    // - b1 == 0 first time in mul_float_significands_general
    // - !goto_full_multiply second time in mul_float_significands_general
    // - in round_helper_raw
    // - !increment_exp in mul_float_significands_general
    test("1.0", "0x1.0#1", "1.0", "0x1.0#2", "1.0", "0x1.0#2");
    // - xs_hi[0] & ulp != 0 in round_helper_raw
    // - increment in round_helper_raw
    // - increment_exp in mul_float_significands_general
    test("1.5", "0x1.8#2", "1.2", "0x1.4#3", "2.0", "0x2.0#3");
    // - b1 != 0 first time in mul_float_significands_general
    // - xs_hi[0] & ulp == 0 in round_helper_raw
    test("1.5", "0x1.8#2", "1.5", "0x1.8#3", "2.0", "0x2.0#3");
    // - !increment in round_helper_raw
    test("1.5", "0x1.8#2", "1.1", "0x1.2#4", "1.8", "0x1.c#4");
    // - xs_len != 1 && ys_len == 1 in mul_float_significands_general
    test(
        "21729783659306408649613509.686",
        "0x11f975eebbcb21a32ee0c5.af8#95",
        "4.140691354e21",
        "0xe.077a2d0E+17#30",
        "8.9976327319036285762104654629e46",
        "0xf.c2ad952d9bedaa8c340fb5cE+38#95",
    );
    // - xs_len <= 2 || ys_len <= MPFR_MUL_THRESHOLD in mul_float_significands_general
    // - goto_full_multiply second time in mul_float_significands_general
    // - b1 != 0 third time in mul_float_significands_general
    test(
        "3.29008365861415556134836580980448399733562188e-9",
        "0xe.217c389f8c9fd22042f5ed70da20cfb9f1ecE-8#146",
        "3719044561792922503530448846362960.3599330496921301151502834994",
        "0xb75cf116bc625ef1eab58f3c9950.5c2492852d5fb6817443c180#205",
        "12235967738412737409453496.5112769496822945381768578702376539566",
        "0xa1f1123e59edd22860db8.82e30bd21587183f065c6eea9383ac0#205",
    );
    // - b1 == 0 third time in mul_float_significands_general
    test(
        "0.152",
        "0x0.27#6",
        "0.000677250271462116637219538246676594845637468375",
        "0x0.002c625fc46315b1256e735851ab2413f0230460#149",
        "0.0001031748460430568314514140297671374960150830727",
        "0x0.0006c2fc96eb184dfcb3d3927471127f099555aa8#149",
    );
    // - xs_len != 1 && ys_len != 1 in mul_float_significands_general
    test(
        "3.600675849075089170455453502457566",
        "0x3.99c5e47746483e1e72f35153c9e#109",
        "7.042716356117671710991084962952371e-31",
        "0xe.48c9931f0a6ce474caf85ca810deE-26#116",
        "2.5358538695359015660810037702729442e-30",
        "0x3.36ee7f4e42c0edeaa7100c9586414E-25#116",
    );
    // - xs_len > 2 && ys_len > MPFR_MUL_THRESHOLD in mul_float_significands_general
    // - (xs[0] != 0 || xs[1] != 0) && (ys[0] != 0 || ys[1] != 0) in mul_float_significands_general
    // - out_prec > p - 5 in mul_float_significands_general
    // - out_prec > p - 5 + Limb::WIDTH || xs_len <= threshold + 1 in mul_float_significands_general
    // - in limbs_float_mul_high_same_length
    // - n >= MPFR_MULHIGH_TAB.len() in limbs_float_mul_high_same_length
    // - n <= MUL_FFT_THRESHOLD in limbs_float_mul_high_same_length
    // - n < MPFR_MULHIGH_TAB.len() in limbs_float_mul_high_same_length
    // - MPFR_MULHIGH_TAB[n] != -1 in limbs_float_mul_high_same_length
    // - k == Some(0) in limbs_float_mul_high_same_length
    // - in limbs_float_mul_high_same_length_basecase
    // - b1 != 0 second time in mul_float_significands_general
    // - !limbs_float_can_round in mul_float_significands_general
    // - goto_full_multiply first time in mul_float_significands_general
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
        "11691360406839692866118328.82160678934697449148561930623262244451953109076390937578282165\
        187539508530274103471810974014544850101997980642382746362536485174905201106566021759334671\
        038084134174395853704847978726074297598470143399522515598459659231845311084194915852728366\
        056422715218301245062324603756166217017490819881851715545101168897106488715462344820352976\
        259754023729090781152750099501504576567751861133292449402560554539891918302651799386455549\
        657664547443484522197757542024159371062990161647359586646799536699319711548827677391665966\
        287279197599493918294842951008826029536226825832854955685587903267919852682540903329229694\
        700736024127943619726831445160555949561747552032983921128094877425217970278847326632273418\
        407484077549236662570769286015075277220777908237436835596208190640803687131540850535115913\
        655954830695789298058473415611731503722972733488269283706021704532728574584684669676341549\
        292252804245357342748316615826715707101522402518116062329441651970107805707342168691626509\
        305758570985170639749549248022366152944291484173783280217974452532323709041672239778485037\
        294723948248639908579195768663906666012983129167763644004221520737884980923091163080707988\
        588422464078838824212171802847408608047867571127753377657964788586031325255106000694013614\
        527340915821283801356153178149699147203162830520911097467745909302616153231682263893483275\
        019567006794361076438485306321832725119559578113910632379303074353609691782388847871383450\
        484766163524999863199330839928739926251980847649975020069602105177714848358628397385350021\
        24095735959378409981273751636624594837614456921497122960535259532723503477033",
        "0x9abbdeae92a7cc75062b8.d254d2926ab47ec2646ac6102bc7f2c66e4adda59ec8c5ced87c449f85121ac94\
        7e79dc78b7313a18f2ce5e38db71d51c990df401764ed1949bc25e2614a9bdb3027efa1acf4ea0e322dfad232e\
        cbd31249cc1917bbdffcead4f455598e97f24397ef8bff64f45dfd672a81269c27e03d6cb02da64c33a0af7326\
        eb8e21ca7d9003144c8f62cd449d48953249142d7f5afd8112b58223fc36214a7e8c4b9a026eb8ba0dc0dca681\
        f9d3c37cf62540e73333f8e07151b6353a7d9d6ad54f935a33c049acbf7c6e4e80629250c603ad3a070a43d3f9\
        70a8c5d99edacc2b229a04d8bb9f8b71acb7611d4e6c4a3e4330466403e5422746189403976a41698df939ffc0\
        32b31da3e6493f1393fd89226bf6b4eadf296725de20aac599c96edd2a445c9d689fee07ae5e94997f00659560\
        27632aa2cbfceba0ad1dcfeb7fb56c3a038718044af027a05c55e79d87a581fe5887ab1630360a4ddf22f18f4d\
        250a5385c3b6ee351d8d368543ed65e80461de207d9d924e8595b146d24defa8dbd01358a44fdf639ae28a852e\
        7675001a1c34c22013db15e7527ad9b7064b49c21ba11b7ed42ac9a570c006de5a092e84450d553767a5e2bb57\
        f588dd1a63559f4b5bf0f0b1eda574f319b65c5c97023caf2862a4487c403143538931bdaba7e1fd57d1d07dc4\
        613f75769c70427af74a7af461be5b0126125aa40920e4e9b44a9e28616dc9ca7b77624028a0ed40aaed08486d\
        90086f51f501e47b6f5e0f7391a8320f07d62774d2c29f3abfc9f71514e3bc471ff2041ede5c0fb4ad39572ab8\
        6909d5a368ef66086eed3f37dad57f14ec6362b1b9567a687e771330ec416265c437c3e6d3c3136aedfe753577\
        319aed775fdefcbe6ff046d9f93acca0d253617b9b98b73f7e7fc043ad2e5f9ae06754e4df0f8#5329",
    );
    // - out_prec <= p - 5 + Limb::WIDTH && xs_len > threshold + 1 in mul_float_significands_general
    // - xs_len > len in mul_float_significands_general
    // - k < len << 1 in mul_float_significands_general
    // - b1 == 0 second time in mul_float_significands_general
    // - !goto_full_multiply first time in mul_float_significands_general
    test(
        "0.000505125560873134723723784766950092518997332731967301291698755707702902596942317437852\
        941942371960388246351631317879977557751125153730732708319124734660713981958784935850223006\
        317900579689594830566320680544405297096706330277372404179805722699177498153458710988448597\
        667865038489642478166853872591228283694673924060010690084057961646317421857676637869623134\
        589158734829608080525671151880734425152367375882305121472858495571639962287931732234443489\
        588015865732596365983920824372468289009819493599234552103162449485200505569599322042323513\
        171853918367347759762125056867136354464299283595190704290061720089955391730926571989543650\
        123824756",
        "0x0.00211a99c3aab8ff53e9907564934b7d8b29ea0189ec9cd3d1a74407e1344b68fcd1809f4234bfef08c0c\
        3f89af1fe509093292b5b157eded4009179fedceae065ab0dd9d53b35a4ce7f1d190711e92618f7d0b216ce0bd\
        09e9c6f6625bcc6003ad38389decfb93a34f29b624c9c3c24dd825a59d15423602847fb2fe248d2b631514d4ed\
        f610fd7088faa3f59c46a406f42343787142069c527e5b0aa730ef1f4054d887515ccd31c4f04063d57d8645a7\
        b70fd198011231617f8b64344b5d43eb951d0a8ebfe39606336b443b19074cd641a63b4656b6e71133e47c099e\
        1fed7bc661252f72ee2c68b5cf9db6ee0645b455fb007e9d294c3a5e091df7f92b6268fff2e65b109ec#2100",
        "1.594827021209513894575272853189101796908969657496052553026904808146944132849277331696165\
        366066982899056554189139962942976395082328192559670863685439930018276703216617917370998670\
        143663891402542565265723410284904517798491873969116076521737839809230852429183992755715781\
        991743918659448701680599805386273131215689278489918563344703836788128584757613316503273812\
        347176374414975252562793531554358724296424942636927017522952046066543039832692600611375088\
        044395866209370211970587133626417624012062511750942538348241762511210915523305672594074841\
        5529943020302456665999104875220004206872624343740835605816652e-79",
        "0x4.ba3e1591f6a0ae18e8a750107efc0ca278b3954588d15a3695387227774da4319070b249d295916c24713\
        070f060a0df67ad5c2e8cc350d13189bcbf73f9af1e33fe42bf44fe99164bf802f7a5f41fe2d28377f1932b019\
        a00373f4400d62f20693ce9fbe239afbc3c804face7c59fb40fe03be617f14dd6880b31ff429280aafe8ea5d94\
        c3349efe67683ab81b8b5f4d00925073051d872bde743109c8e7c79da2c5fff9f489b12c271aa83405fe1d99c2\
        9f846c6b1c71dec233f13240569b3015559ed53bda719283585ed3f94bdeaea4d8438fe90e633bb97ee755eb88\
        113450187e9b05d8e6040fa53e6a10b6dcc2f01463a3937f5a90E-66#1988",
        "8.055878935840864338761112052964131694842078144914087985110265043132694138227764882097467\
        485603629304908508159498533271385147841989466565220242546894501115782558008761861379571521\
        691942027516784202151149343065118307142127390742346695428089929852244559471690807529077258\
        667004534037323942848543095527661248910413746226405479291587579546107337446543923963731652\
        263604834014601264189816875633028978567676911887046995279630281027980542407933148412976200\
        257662826354908783390381610432026516882234795071151533161797811544609495678329417798803859\
        814384620434259660366163853727351148214688205975873187006097686502581917099916584703699137\
        7927e-83",
        "0x9.c7fc20220e43b5ac1717bf3fc467495e7a66c020cb393887e4bf4a3ad3eb4f20763507aea5b673c5aff25\
        8525ec5ac9bbc9f5c976983bce6231282e951654f346e3e5f7779ecdeae718176d55b03e21297ae13b32728a38\
        30c19eb847a199cde2fd0f3aa2330282fc433000c5f99b14022cd881da6b7caa98d3563644c4710ec876c4b61c\
        b15fdd4cc7d712644bad3a66e844700eafebabf02dead4fd71d95a14ef9c6a12db9092cf7b4723f45cb086b401\
        8123e185be43126baa17ab68fd32a0834023652a003d24211dd87066d2d5b1f0f1993c0dd9756329316927dd98\
        425ba5d470abc9065c6ff724d0e16c68152314d36e37d235fe078572e42d94540fe4418a9440633E-69#2100",
    );
    // - MPFR_MULHIGH_TAB[n] == -1 in limbs_float_mul_high_same_length
    // - k.is_none() in limbs_float_mul_high_same_length
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
        "5.713957232680308157039243396854396365207579750474509016836068310655183904589399398864949\
        369194427543435776465300666190725186454505340683874976519507548279996486761744005041118935\
        948747589743699350970808906395296086217435990867715483224265504817855574934867156151545781\
        106814865537845359234760816964504042844539777879186641559132734209465852287196566458935778\
        830300266229578325422723437672411534085537936547389438107265754858071594639985760821104661\
        940648845072022419465943642338099526948876829384943161258695569354371632958134560717841147\
        840574265292252548747529841143166165808068071464187327019844718180901133673499079508336221\
        530763161764985656092293178070670798408664087435197003752408586649867017671653317949434498\
        7804832149608135310831412140057788625216612318872425368846162e-33",
        "0x1.dab26e4414e016457a3b64d6dc588a9f2a4cbec7acd1a5b9033e00168a27a1ac3f669e39332cdbace29d9\
        b5d727df53a041f44f1389d6fda544d07799d35a62fbf327a55fca19ed803e80954753bfb12dcfda81be99eda5\
        2d7a382f6dd0ac26ab4b55fa3587bc812156b0ddb289357e7300ea3dafa43cee4f98c4068d3ffbeceea4a01ecc\
        74a09b85289f4c37000efab1e07926e01496cf6855b4a1f08915fff56a1737ff1d109fb6f3011fb249b88db85f\
        52980590101cbaa0e8af5872ded15ef0f39ec69d2d19d45c2972793e60bbb6eee9b673f24b3318973256a8e9bc\
        90d40f033e145dc56a6caa7e542a2701c0a41519fd0ef8d97ed0cef3e4296de50e877edeae80c0dd91f9082539\
        71e8998ef2a91d865baa5aa565e308789af81b128e8d0e832f5368e689962335ca7077ad875ea20eae1f2fc375\
        a972b8e7b8e285bbe3bdb0E-27#2587",
    );
    // - xs_len <= len in mul_float_significands_general
    // - limbs_float_can_round in mul_float_significands_general
    test(
        "2791656999165.493196894581080462482671477586959884065253358835066410305633620155930725019\
        490863414181814174955842522364865024774379310519058136940369290858518517313460149216061827\
        231421960818181308820010416988884966114071973567666857448623899431042502611223036510238920\
        700493077403107131238955287441775846760482666068674778697270732066126184067657756561516984\
        251159820991198761865419427067747698722303345927460124328666182849055120607788400162794652\
        0878157328543259919881",
        "0x289fbb948fd.7e4226d4b6e97c4ccb64c2a072193f46e582b56e431d1d873706fd936c5e3328fd5b164ebe6\
        f6c3319057366a2de168eaef87f67644a657027f176fb9d5deaa62912e24d9b1eebf67a642c7ebc91e95eba928\
        d2c47f45ab981a3053c2a055e2a36102f8c4b2c72f24edbf87122ddff42408c1c95b6eccee7c0a49a25e2fe2fd\
        67a0dfc9d33ccad11d273f88f3e5a176da6b5989e6fc247727a58af5ef0f3a863e35d59edceb1ed0401bb2ce98\
        77227cdff6442212230b17fb43a061516c#1558",
        "2418611879271263.303076595070873673412348926925363768438166046534014463089969421888804109\
        398849144724043021994927400943502019398552005514731676646297869532250197280910814394530400\
        370821284006890204265425587260634300563572770382580394115150082129924900080499709146475799\
        383767996765000283568589472197582158015457869515972738606414178902063354061536898088295371\
        641586286472920893106777705320869713042818435525510767840203306234890813618032983465299395\
        730347727963352100742341245602143",
        "0x897b6f5e9db5f.4d966d80032f1615c15b6d17e02b100d2d97b52de1b561199b3dbeda9276bc25c5519d480\
        c2c20f52155b94074ad9db0bd7a97dae0337b738b6709c5fdd1bbcf1492bef975c393313e477790485d0a4ed0f\
        8790d9d469d65c29122cd019f7ff423e0f74c0585cac3aa85ceee8cbd256a201e23dd78f6dc402aa36b5a053ea\
        c1a3b2c435ae84ee9942bec1d88e41507bee030c35cd3f18b2f7a5c9193976d7418a633e08ca4923be03a2dc93\
        6ed64c6ff04ace32f44aa72f58bf488b9bf31c4c03f#1596",
        "6751934781032429031553446805.854936199324254228337806577049209187291041079391491758652444\
        360751832908394550062618692018471995917689775126406303977393164343146088816585483538887454\
        027840163444916716206473736192766508247978473183426254779355642987433462433675113794019449\
        629515518752711445976665575431349720824019816455254142652806744349728110370084378693365847\
        376950483953541149499814845310185214082773221620162084856311318860928357827312318714828967\
        076794760533141841485387490188815",
        "0x15d111cde75ec690f0fb3395.dadd194843a336fd8b04fc1edf686f76aa52a00d00db09b133bcedaa832e64\
        e3cbe80e635c5bb60e81bb4cb3839dc581a1f683d480b6757136ace3d394264c5f969b75cb4d985b4b2b1096d2\
        a53a6d6570c7f7c917bed4f2a9c8f7bcdea046337457665b27e5e01bdbce6f20c6d5bc9e463f055cca9b0c53a2\
        f1f5b6d7211ac001a5fa99daf3378815db2c7bc77c6d6b76f815ae464fdc70ae82752dbe2b5683cc1ebc18275d\
        4685cbd35d58b70dbda84d2297ff9a12d20f2283678e#1596",
    );
    // - out_prec <= p - 5 in mul_float_significands_general
    test(
        "7.449979045041702122541548973189889723858373710052883535770432677692650998750696470703091\
        744255868088920360332810996981821635561095981209970543165990341865465399250649125818133550\
        345620227288485003961432095280508748897878683743133302349812547742789930883805941444651209\
        784236601329970891014476723035066837590220224883282604585022340111695027909185310636007609\
        680407882966239499164648695501638941698641158401193435378725738878261400237405551514713128\
        836982995938875007304023367379668047357772772649359285993284186179803252277e-21",
        "0x2.32e78d682285335802e277865ca4c9a80daafb30e4f67daedb2f466845a2fba5d9b1c96b068c42999ae0b\
        463fb94e578b359027630c01a995d88a74b72186c316ec30a8b4238fb27273b57cc5e72fa518fa14032e99af11\
        e1d5ccfc1217be504bf8dae0d1ec7e885fc95b51f444d68ed47eb588644910e4196d65104707dcd1c488a22e9c\
        1ef05ce85b3823fd6aa2bb7ad7d14fa1da7a628e1ba344d98ad7e95bf7e01883afc84273614fb8387e4035b2d9\
        ad867bc2be83eaabaa44df896348f52d4972bc5e8cf58f1cfd421cc30920d9f5504a043149274f5E-17#1738",
        "4.689598630661113758052361654518074651900718126313070505518580809308545656249466966444734\
        190644625007707301196274229099309347916384836732990153197811248723616418486524781737017155\
        222220872736570797023388780311017887508289881051954626904239986311860650999175535050592519\
        124458883175586921775053060798371665433755864916689805023189965297596140362143269558535567\
        196585839368205277548820051354051306095222442551993791324926248616628216018709202221063695\
        130497558241752004759655263919857509980778615592531001107310020150056419e-20",
        "0xd.d75c4c85ded4dada979685c90590eb9f191595aa1862df62f276fa75d5d790d2c00d187fe126b806534fd\
        0379dfcf600aeee23bbe00ed5955cc2e11b7de967da0c347d3ca03714f55b714dbe2c9ad31dbd7b6fb9a639fa4\
        da6cec918f3fc77bbca5ded28446186fe8b4224339c0722fed5f69b2d07571b12f4cce6fece51fc63ab46ea7fe\
        5c1f350ed6aab2c76a1e817bf652a9c77a95e3e3c107ed65e9ace86dfec217fb9dcbba544e3d84b0f3cac6c221\
        176c57f1db9bc306f22db85308dab755243a502d385e1b0a31f2f8c2d53c98729cfe52f82289cE-17#1730",
        "3.493741152808155821061780436862260285420312736969910829282351541785101293399126489805777\
        980187532032743650816868281893520821921299562434265222876601442289649987508199259232837158\
        559068445030886433472805552651673239073165198885867344396236299229232184802565958836951408\
        637905144866851014091996571320623346167534428281388860566996500707311335172658622534949254\
        838518529528764394710349462443386828046760610922770501616700810546292498376696602122491857\
        8540047731481736443609736067164917894729323015869507020379275317039780244947e-40",
        "0x1.e6f4d9986216e186a3b15da557e0677276dd590e3d92a71ff89974ad2abac1a1e2e3dc761157b14cfef04\
        285b10c4bf2f72117bbdbdfd5ead95a5ed69f62f69682442a59a3f3c8a5a7b41238f0a029b2d53c54a08053451\
        35f5fc0a67464283bade3997992eea966997d442d69987820628966dfcaa6d25ca0be2df6ed92e5fa5201002da\
        70c6447c3a8a64ebd34fc04156c8dbd224d772ba1059982a5aa333d301a708dc568330bdead12245127d520b32\
        b4e7d9ae221552ab3d3f5f2f139c90118fe830b6adc84392948585b40267beb9fd60cce93fbed140E-33#1738",
    );
    // - xs[0] == 0 && xs[1] == 0 || ys[0] == 0 && ys[1] == 0 in mul_float_significands_general
    // - xs[xs_offset] == 0 in mul_float_significands_general
    // - ys[ys_offset] == 0 in mul_float_significands_general
    test(
        "4.44089209850062616169452667236328125e-16",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000E-13#2587",
        "6.938893903907228377647697925567627621275394370123920756842177449687616281576087749649243\
        449560881562252974476503855378704420289043275360540974361356347799301147460937500000000000\
        428087763478203718701697872488500162623204828539850902982930587576291943616315378569169353\
        840957917417166522314320167278698048638040260019989045715363127222769643922793712163996851\
        225018060026136483495536368092685586468682354268058778363979622204058653592623442621394947\
        3239996744456973724770155e-18",
        "0x8.0000000000000000000000000003fff000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000007fffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fe00000000000000000000000000000000000E-15#1571",
        "3.081487911019577364889564708135884006379343333980678546187704758135879626468817238227001\
        919863959483626636205013163029555487554952753086708205626001498966504653625325671484347367\
        558282010814620352468694740583510310926487120637351903244174605756585424927747220355720149\
        705262186320261062950329818867477826244378862350354462458385252797818784708103173459291235\
        769538479528034217611825504020777438216019896385615152551542382665798426796321259955655479\
        300773291504754679549289638268112862439028560817297535629039872109954348969232817790940901\
        772925125639499661447304917621775611916484032847611462965461660088172016929375670794716891\
        215288121297398166137896866022540415673603027116093396582728756514564426357106145459651571\
        0636500728956428014332817951864653686798458732591874037159276e-33",
        "0x1.00000000000000000000000000007ffe00000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000fffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffc000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000E-27#2587",
    );
    // - xs[xs_offset] != 0 in mul_float_significands_general
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
        "1.577721810442023610823457130565572459346412870218046009540557861328124999999781992456191\
        582683140605249728137786969774247203293102032637435297463867968227730622206111553159326620\
        257748727154917100482837417166916131758491078680030585867783733868391940878208953914079279\
        677271330675596139325733702207799069583415985107421875000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000001527219401e-30",
        "0x1.ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000007ffffff8E-25#2567",
    );
    // - ys[ys_offset] != 0 in mul_float_significands_general
    test(
        "3.552713678800500929355621337890625000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000185653473271011701515143789632334288014147578747413968011889263434165\
        454031003144878632958696545103828379944287431463737771314691776232764018029744516848435369\
        906941507005331339097431189921414095915636548343770290467007831930064077297413615608072068\
        77626957251331627162116234578193867366731943186452308784033031e-15",
        "0x1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000003fffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffE-12#1693",
        "131071.9999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999931137299504668061215619580033350122879947175743013377219720120678876536\
        628643562946337394277177628286886258713868752206334184193331360816755408598724051586961746\
        685495576886231160425736851019484652776925452038003143395027472798288565900387988918967170\
        024140096215282631707814129061246845336549291242702672707242154281040996134116689237194330\
        449329247059466983779402165943011772702916829761392820351107526820413839207822952585392346\
        328868866364548445701244431290159307431187837005854773744122693182105660536394809595147776\
        165023610906858577556273686957594781817444085841499342583068122729709267291",
        "0x1ffff.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffff80000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffc0000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000#2930",
        "4.656612873077392578125000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000243339720485780457409929267946893197985903514415810436152543495368429\
        343907516442055321546973277589582124346598388556070043505489984826143425305512049232107258\
        133592644143603087675038065556842214758248268059151846952539521827810268928481776834311731\
        044052486308720793745225556140061733844701779522496735016181906708106564202059526039890815\
        919178696765989061598664073635409369732679923290228161362302668920197866351570235145652815\
        456445767888399222146130209012884306181795683184194476165898983356347814975458163826496169\
        408195862006279548822773547655683272930717806964759394377376260006554734212838956379338694\
        525902352837865322055484704841599472083306918235788680977972181745262154636e-10",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000007fffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffefffffffffffffffffffdffffffffffffffffffffff\
        fffffffffffffffffff80000000000000000000000000000000008000000000000000000000000000000000000\
        000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffdfffffffffffc0000000000000000004000000000000000000000000000000000000000000000000000000\
        00080000000000000E-8#2930",
    );
    test(
        "12962368.00910437392826314193446550944389165836262015621514962112057577196019471161937054\
        243429475140935082492484312458697139062031326336320065697050792374770657213779646419924892\
        886900897092216274606911219143642483069838857046000551685989850622347518286427108428594195\
        826642547794926862213089792293539194572777838215978068770604450325322408918968488729618206\
        83239161602000658143319869428",
        "0xc5ca40.0254aa0c45bef8b7e5bdc2c8d0bc3f4ef77512b0c665764acef9797020852473fed6cf5aef7ab76f\
        af4e7ccca7674b7b9a7d4e3adf7dbd2f9c51a93c8b500f9c6e799811cc9793ad3189530f5202900aed04c32d98\
        72075f4a8efb28f1e3d9e2065d70be0df4f07d46d4f2f48b95a6fcd718d2f233dbbaff41296fe46efd00783168\
        f9da607a651780d29e9357c1d8e9abdd149e2d7ed04e74488fff360#1283",
        "24678125950611051118469790273997953293778953666881566874553684447233373952852192.03220118\
        824573657786267791510531425783997288827451260529832388036858875235909753308200751110173525\
        014519161498781309811156911488470099945303550175018037705473098495478288721507648423524330\
        255884870759432606723320831807670552582364423881930823507999308667398128596287742877235432\
        154091338959855861235089997932459245142831892",
        "0xd51fdc4e02742b528c90f8a31146dcba3e1273bfdd1198ff1ec486c97e52f8c0e0.083e564a6864272a5d52\
        74cbdf67db38b72e576e7cfe8be5418f583cf4173a82939f37d7b6b399cada03434a24aef04a65323c333b33cf\
        622583787d8ec1f247c4f691c3a7862628783f4322923b6bb9434902754e348fb74eb6364e294832597785e445\
        1994ee48803ea5771d85e7f226166cdd1853477f296076996b97965596455eb50129#1336",
        "319886950346849155967554337687681180827634107405044558954903133134156000777816460853971.0\
        571286460940042331570995057120143779176892645683546163312322051333969186064059757999062545\
        749424646225780605680908775358827191384406475576071818545005626321659954666901446716884049\
        318512517659256499382137861718683137930417158199590725586222075459373218000006096544204046\
        600400863088080659996213042623116710817104536",
        "0xa4a9e4f5c54093e3388df17b55e5724251f90cd6c231443e4bc0d5770919a2c1878982d3.0e9ffba2a37526\
        7ffbc880a862170226c003dcfce47863efaa79493245f971642ca10409a10ad584de1597974f309b61313528bd\
        1512f14b2397a5630d41d067d57d9b327d751eacb350f8e13428d4508849e4464775e72f3b4fb63d6bef2d97b6\
        f0118aa022becb98c8c88fdcb280078abcbe4c3b8423dc1dfc91e6b09ea9115cd0cd#1336",
    );
}

#[test]
fn test_mul_prec() {
    let test = |s, s_hex, t, t_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (product, o) = x.clone().mul_prec(y.clone(), prec);
        assert!(product.is_valid());

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);
        assert_eq!(o, o_out);

        let (product_alt, o_alt) = x.clone().mul_prec_val_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = x.mul_prec_ref_val(y.clone(), prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = x.mul_prec_ref_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_prec_assign(y.clone(), prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_prec_assign_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = mul_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product),
        );
        assert_eq!(o_alt, o);
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
        "Infinity", "Infinity", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal,
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
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "NaN",
        "NaN",
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
        "0.0", "0x0.0", "Infinity", "Infinity", 1, "NaN", "NaN", Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "1.0", "0x1.0#1", 1, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "-1.0", "-0x1.0#1", 1, "-0.0", "-0x0.0", Equal,
    );

    test("-0.0", "-0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, "NaN", "NaN", Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, "0.0", "0x0.0", Equal);
    test(
        "-0.0", "-0x0.0", "1.0", "0x1.0#1", 1, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, "0.0", "0x0.0", Equal,
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
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal,
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
        "1.0",
        "0x1.0#1",
        10,
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
        "-123.0",
        "-0x7b.0#10",
        Equal,
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
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        "2.0",
        "0x2.00#10",
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
        "4.445",
        "0x4.72#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-4.445",
        "-0x4.72#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "-4.445",
        "-0x4.72#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "4.445",
        "0x4.72#10",
        Greater,
    );

    // yyy
    test(
        "1.4134592e-8",
        "0x3.cb5260E-7#24",
        "3.7164162e-37",
        "0x7.e768d0E-31#24",
        2,
        "6.0e-45",
        "0x2.0E-37#2",
        Greater,
    );
    test(
        "1.5", "0x1.8#2", "1.5", "0x1.8#2", 3, "2.0", "0x2.0#3", Less,
    );
}

#[test]
fn mul_prec_fail() {
    assert_panic!(Float::NAN.mul_prec(Float::NAN, 0));
    assert_panic!(Float::NAN.mul_prec_val_ref(&Float::NAN, 0));
    assert_panic!(Float::NAN.mul_prec_ref_val(Float::NAN, 0));
    assert_panic!(Float::NAN.mul_prec_ref_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.mul_prec_assign(Float::NAN, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.mul_prec_assign_ref(&Float::NAN, 0)
    });
}

#[test]
fn test_mul_round() {
    let test = |s, s_hex, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (product, o) = x.clone().mul_round(y.clone(), rm);
        assert!(product.is_valid());

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);
        assert_eq!(o, o_out);

        let (product_alt, o_alt) = x.clone().mul_round_val_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = x.mul_round_ref_val(y.clone(), rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = x.mul_round_ref_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_round_assign(y.clone(), rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_round_assign_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_product, rug_o) =
                rug_mul_round(rug::Float::exact_from(&x), rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_product)),
                ComparableFloatRef(&product),
            );
            assert_eq!(rug_o, o);
        }

        let (product_alt, o_alt) = mul_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
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
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal,
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
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
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
        "-Infinity",
        "-Infinity",
        Down,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
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
        "-Infinity",
        "-Infinity",
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Exact,
        "NaN",
        "NaN",
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
        "0.0", "0x0.0", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
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

    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Exact, "-0.0", "-0x0.0", Equal,
    );

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
        "-0.0", "-0x0.0", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Exact, "0.0", "0x0.0", Equal,
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
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", Floor, "0.0", "0x0.0", Equal,
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
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Exact, "-0.0", "-0x0.0", Equal,
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
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", Floor, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", Ceiling, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", Down, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", Up, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", Nearest, "123.0", "0x7b.0#7", Equal,
    );
    test(
        "1.0", "0x1.0#1", "123.0", "0x7b.0#7", Exact, "123.0", "0x7b.0#7", Equal,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Floor,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Down,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Up,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Nearest,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        Exact,
        "-123.0",
        "-0x7b.0#7",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Floor, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Ceiling, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Down, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Up, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Nearest, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Exact, "2.0", "0x2.0#1", Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Ceiling, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Down, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Up, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Nearest, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Exact, "2.0", "0x2.0#2", Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Ceiling, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Down, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Up, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Nearest, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Exact, "2.0", "0x2.0#2", Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Ceiling, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Down, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Up, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Nearest, "2.0", "0x2.0#2", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Exact, "2.0", "0x2.0#2", Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "4.442882938158367",
        "0x4.7160c6b758b94#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "4.442882938158367",
        "0x4.7160c6b758b94#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-4.442882938158367",
        "-0x4.7160c6b758b94#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-4.442882938158367",
        "-0x4.7160c6b758b94#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "-4.442882938158367",
        "-0x4.7160c6b758b94#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "-4.442882938158367",
        "-0x4.7160c6b758b94#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "-4.442882938158366",
        "-0x4.7160c6b758b90#53",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "4.442882938158367",
        "0x4.7160c6b758b94#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "4.442882938158367",
        "0x4.7160c6b758b94#53",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "4.442882938158366",
        "0x4.7160c6b758b90#53",
        Less,
    );

    // yyy

    // - rm == Floor || rm == Down in mul_float_significands_same_prec_lt_w
    test(
        "1.5", "0x1.8#2", "1.5", "0x1.8#2", Down, "2.0", "0x2.0#2", Less,
    );
    // - rm == Ceiling || rm == Up in mul_float_significands_same_prec_lt_w
    // - (rm == Ceiling || rm == Up) && !product.overflowing_add_assign(shift_bit) in
    //   mul_float_significands_same_prec_lt_w
    test(
        "1.5", "0x1.8#2", "1.5", "0x1.8#2", Up, "3.0", "0x3.0#2", Greater,
    );
    // - (rm == Ceiling || rm == Up) && product.overflowing_add_assign(shift_bit) in
    //   mul_float_significands_same_prec_lt_w
    test(
        "1.2", "0x1.4#3", "1.5", "0x1.8#3", Up, "2.0", "0x2.0#3", Greater,
    );
    // - rm == Floor || rm == Down in mul_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Down,
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        Less,
    );
    // - rm == Ceiling || rm == Up in mul_float_significands_same_prec_w
    // - (rm == Ceiling || rm == Up) && !product.overflowing_add_assign(1) in
    //   mul_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Up,
        "1.0000000000000000003",
        "0x1.0000000000000006#64",
        Greater,
    );
    // - (rm == Ceiling || rm == Up) && product.overflowing_add_assign(1) in
    //   mul_float_significands_same_prec_w
    test(
        "1.9999999999999999998",
        "0x1.fffffffffffffffc#64",
        "2.0000000000000000002",
        "0x2.0000000000000004#64",
        Up,
        "4.0",
        "0x4.0000000000000000#64",
        Greater,
    );
    // - rm == Floor || rm == Down in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Down,
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Less,
    );
    // - rm == Ceiling || rm == Up in mul_float_significands_same_prec_gt_w_lt_2w
    // - (rm == Ceiling || rm == Up) && !overflow in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Up,
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        Greater,
    );
    // - (rm == Ceiling || rm == Up) && overflow in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "295147905179352825855.999999999999999997",
        "0xfffffffffffffffff.ffffffffffffffc#127",
        "0.000244140625000000000000000000000000000003",
        "0x0.00100000000000000000000000000000004#127",
        Up,
        "72057594037927936.0",
        "0x100000000000000.000000000000000000#127",
        Greater,
    );
    // - rm == Floor || rm == Down in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Down,
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
        Less,
    );
    // - rm == Ceiling || rm == Up in mul_float_significands_same_prec_gt_2w_lt_3w
    // - (rm == Ceiling || rm == Up) && z_2 != 0 in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Up,
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        Greater,
    );
    // - (rm == Ceiling || rm == Up) && z_2 == 0 in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.9999999999999999999999999999999999999993",
        "0x1.ffffffffffffffffffffffffffffffffc#132",
        "2.0000000000000000000000000000000000000007",
        "0x2.000000000000000000000000000000004#132",
        Up,
        "4.0",
        "0x4.000000000000000000000000000000000#132",
        Greater,
    );

    // - in mul_float_significands_same_prec_lt_w
    // - decrement_exp in mul_float_significands_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_lt_w
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal,
    );
    // - !decrement_exp in mul_float_significands_same_prec_lt_w
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_lt_w
    // - rm == Nearest in mul_float_significands_same_prec_lt_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (product & shift_bit) == 0)) in
    //   mul_float_significands_same_prec_lt_w
    test(
        "1.5", "0x1.8#2", "1.5", "0x1.8#2", Nearest, "2.0", "0x2.0#2", Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (product & shift_bit) != 0) &&
    //   product.overflowing_add_assign(shift_bit) in mul_float_significands_same_prec_lt_w
    test(
        "1.2", "0x1.4#3", "1.5", "0x1.8#3", Nearest, "2.0", "0x2.0#3", Greater,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (product & shift_bit) != 0) &&
    //   !product.overflowing_add_assign(shift_bit) in mul_float_significands_same_prec_lt_w
    test(
        "1.2", "0x1.4#4", "1.4", "0x1.6#4", Nearest, "1.8", "0x1.c#4", Greater,
    );

    // - in mul_float_significands_same_prec_w
    // - decrement_exp in mul_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_w
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
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_w
    // - rm == Nearest in mul_float_significands_same_prec_w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && product.even())) in
    //   mul_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Nearest,
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || product.even()) &&
    //   !product.overflowing_add_assign(1) in mul_float_significands_same_prec_w
    test(
        "3.2729513077064011786e-37",
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        "7.8519772600462495573e-34",
        "0x4.13b4f0d218450fb0E-28#64",
        Nearest,
        "2.569913924134929736e-70",
        "0x1.c610823e5a4c0774E-58#64",
        Greater,
    );
    // - !decrement_exp in mul_float_significands_same_prec_w
    test(
        "3116635254961129.0696",
        "0xb1290314433e9.11d#64",
        "7.092177112370390978e-10",
        "0x3.0bcb09ebbb50e418E-8#64",
        Nearest,
        "2210372.9222841977617",
        "0x21ba44.ec1ad133010#64",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || product.even()) &&
    //   product.overflowing_add_assign(1) in mul_float_significands_same_prec_w
    test(
        "1.9999999999999999998",
        "0x1.fffffffffffffffc#64",
        "2.0000000000000000002",
        "0x2.0000000000000004#64",
        Nearest,
        "4.0",
        "0x4.0000000000000000#64",
        Greater,
    );
    // - in mul_float_significands_same_prec_gt_w_lt_2w
    // - l.wrapping_add(2) & (mask >> 2) <= 2 in mul_float_significands_same_prec_gt_w_lt_2w
    // - decrement_exp in mul_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_gt_w_lt_2w
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
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in mul_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (z_0 & shift_bit) == 0)) in
    //   mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Less,
    );
    // - l.wrapping_add(2) & (mask >> 2) > 2 in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        Nearest,
        "1.00000000000000000022",
        "0x1.0000000000000004#65",
        Less,
    );
    // - !decrement_exp in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.0001507756106295330606262754053",
        "0x0.0009e19851127b95dcf03f0cdc#91",
        "3458.565842843038054059107814",
        "0xd82.90db1399862ba513faf8#91",
        Nearest,
        "0.5214673768571047372764465276",
        "0x0.857ee2d1883c6e783c18b1e#91",
        Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && !overflow
    //   in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "119368.6474438890389479272222539538",
        "0x1d248.a5bee1f96ad66a5061314f7#109",
        "1.573235366444334767515689608501749e-6",
        "0x0.00001a64fe94215b4ea1a015983c92bc#109",
        Nearest,
        "0.1877949778033513768632732912065661",
        "0x0.301354e804b87d40aa80cfe1472a#109",
        Greater,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (z_0 & shift_bit) != 0) && overflow
    //   in mul_float_significands_same_prec_gt_w_lt_2w
    test(
        "295147905179352825855.999999999999999997",
        "0xfffffffffffffffff.ffffffffffffffc#127",
        "0.000244140625000000000000000000000000000003",
        "0x0.00100000000000000000000000000000004#127",
        Nearest,
        "72057594037927936.0",
        "0x100000000000000.000000000000000000#127",
        Greater,
    );
    // - in mul_float_significands_same_prec_gt_2w_lt_3w
    // - a0.wrapping_add(4) & (mask >> 2) <= 4 in mul_float_significands_same_prec_gt_2w_lt_3w
    // - decrement_exp in mul_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        Nearest,
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest in mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && z_0 & shift_bit == 0)) in
    //   mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        Nearest,
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
        Less,
    );
    // - a0.wrapping_add(4) & (mask >> 2) > 4 in mul_float_significands_same_prec_gt_2w_lt_3w
    // - !decrement_exp in mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) in
    //   mul_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && !overflow
    //   in mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        Nearest,
        "0.00497732823382322348141815797577421539704054126",
        "0x0.014631b5fc58aeb12d61fe8ebe2fa3511f34a8b#149",
        Greater,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || z_0 & shift_bit != 0) && overflow in
    //   mul_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.9999999999999999999999999999999999999993",
        "0x1.ffffffffffffffffffffffffffffffffc#132",
        "2.0000000000000000000000000000000000000007",
        "0x2.000000000000000000000000000000004#132",
        Nearest,
        "4.0",
        "0x4.000000000000000000000000000000000#132",
        Greater,
    );
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Nearest, "2.0", "0x2.0#3", Greater,
    );

    // - in mul_float_significands_general
    // - xs_len <= 2 in mul_float_significands_general
    // - xs_len == 1 in mul_float_significands_general
    // - b1 == 0 first time in mul_float_significands_general
    // - !goto_full_multiply second time in mul_float_significands_general
    // - in round_helper_raw
    // - !increment_exp in mul_float_significands_general
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#2", Nearest, "1.0", "0x1.0#2", Equal,
    );
    // - xs_hi[0] & ulp != 0 in round_helper_raw
    // - increment in round_helper_raw
    // - increment_exp in mul_float_significands_general
    test(
        "1.5", "0x1.8#2", "1.2", "0x1.4#3", Nearest, "2.0", "0x2.0#3", Greater,
    );
    // - b1 != 0 first time in mul_float_significands_general
    // - xs_hi[0] & ulp == 0 in round_helper_raw
    test(
        "1.5", "0x1.8#2", "1.5", "0x1.8#3", Nearest, "2.0", "0x2.0#3", Less,
    );
    // - !increment in round_helper_raw
    test(
        "1.5", "0x1.8#2", "1.1", "0x1.2#4", Nearest, "1.8", "0x1.c#4", Greater,
    );
    // - xs_len != 1 && ys_len == 1 in mul_float_significands_general
    test(
        "21729783659306408649613509.686",
        "0x11f975eebbcb21a32ee0c5.af8#95",
        "4.140691354e21",
        "0xe.077a2d0E+17#30",
        Nearest,
        "8.9976327319036285762104654629e46",
        "0xf.c2ad952d9bedaa8c340fb5cE+38#95",
        Less,
    );
    // - xs_len <= 2 || ys_len <= MPFR_MUL_THRESHOLD in mul_float_significands_general
    // - goto_full_multiply second time in mul_float_significands_general
    // - b1 != 0 third time in mul_float_significands_general
    test(
        "3.29008365861415556134836580980448399733562188e-9",
        "0xe.217c389f8c9fd22042f5ed70da20cfb9f1ecE-8#146",
        "3719044561792922503530448846362960.3599330496921301151502834994",
        "0xb75cf116bc625ef1eab58f3c9950.5c2492852d5fb6817443c180#205",
        Nearest,
        "12235967738412737409453496.5112769496822945381768578702376539566",
        "0xa1f1123e59edd22860db8.82e30bd21587183f065c6eea9383ac0#205",
        Less,
    );
    // - b1 == 0 third time in mul_float_significands_general
    test(
        "0.152",
        "0x0.27#6",
        "0.000677250271462116637219538246676594845637468375",
        "0x0.002c625fc46315b1256e735851ab2413f0230460#149",
        Nearest,
        "0.0001031748460430568314514140297671374960150830727",
        "0x0.0006c2fc96eb184dfcb3d3927471127f099555aa8#149",
        Less,
    );
    // - xs_len != 1 && ys_len != 1 in mul_float_significands_general
    test(
        "3.600675849075089170455453502457566",
        "0x3.99c5e47746483e1e72f35153c9e#109",
        "7.042716356117671710991084962952371e-31",
        "0xe.48c9931f0a6ce474caf85ca810deE-26#116",
        Nearest,
        "2.5358538695359015660810037702729442e-30",
        "0x3.36ee7f4e42c0edeaa7100c9586414E-25#116",
        Less,
    );
    // - xs_len > 2 && ys_len > MPFR_MUL_THRESHOLD in mul_float_significands_general
    // - (xs[0] != 0 || xs[1] != 0) && (ys[0] != 0 || ys[1] != 0) in mul_float_significands_general
    // - out_prec > p - 5 in mul_float_significands_general
    // - out_prec > p - 5 + Limb::WIDTH || xs_len <= threshold + 1 in mul_float_significands_general
    // - in limbs_float_mul_high_same_length
    // - n >= MPFR_MULHIGH_TAB.len() in limbs_float_mul_high_same_length
    // - n <= MUL_FFT_THRESHOLD in limbs_float_mul_high_same_length
    // - n < MPFR_MULHIGH_TAB.len() in limbs_float_mul_high_same_length
    // - MPFR_MULHIGH_TAB[n] != -1 in limbs_float_mul_high_same_length
    // - k == Some(0) in limbs_float_mul_high_same_length
    // - in limbs_float_mul_high_same_length_basecase
    // - b1 != 0 second time in mul_float_significands_general
    // - !limbs_float_can_round in mul_float_significands_general
    // - goto_full_multiply first time in mul_float_significands_general
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
        "11691360406839692866118328.82160678934697449148561930623262244451953109076390937578282165\
        187539508530274103471810974014544850101997980642382746362536485174905201106566021759334671\
        038084134174395853704847978726074297598470143399522515598459659231845311084194915852728366\
        056422715218301245062324603756166217017490819881851715545101168897106488715462344820352976\
        259754023729090781152750099501504576567751861133292449402560554539891918302651799386455549\
        657664547443484522197757542024159371062990161647359586646799536699319711548827677391665966\
        287279197599493918294842951008826029536226825832854955685587903267919852682540903329229694\
        700736024127943619726831445160555949561747552032983921128094877425217970278847326632273418\
        407484077549236662570769286015075277220777908237436835596208190640803687131540850535115913\
        655954830695789298058473415611731503722972733488269283706021704532728574584684669676341549\
        292252804245357342748316615826715707101522402518116062329441651970107805707342168691626509\
        305758570985170639749549248022366152944291484173783280217974452532323709041672239778485037\
        294723948248639908579195768663906666012983129167763644004221520737884980923091163080707988\
        588422464078838824212171802847408608047867571127753377657964788586031325255106000694013614\
        527340915821283801356153178149699147203162830520911097467745909302616153231682263893483275\
        019567006794361076438485306321832725119559578113910632379303074353609691782388847871383450\
        484766163524999863199330839928739926251980847649975020069602105177714848358628397385350021\
        24095735959378409981273751636624594837614456921497122960535259532723503477033",
        "0x9abbdeae92a7cc75062b8.d254d2926ab47ec2646ac6102bc7f2c66e4adda59ec8c5ced87c449f85121ac94\
        7e79dc78b7313a18f2ce5e38db71d51c990df401764ed1949bc25e2614a9bdb3027efa1acf4ea0e322dfad232e\
        cbd31249cc1917bbdffcead4f455598e97f24397ef8bff64f45dfd672a81269c27e03d6cb02da64c33a0af7326\
        eb8e21ca7d9003144c8f62cd449d48953249142d7f5afd8112b58223fc36214a7e8c4b9a026eb8ba0dc0dca681\
        f9d3c37cf62540e73333f8e07151b6353a7d9d6ad54f935a33c049acbf7c6e4e80629250c603ad3a070a43d3f9\
        70a8c5d99edacc2b229a04d8bb9f8b71acb7611d4e6c4a3e4330466403e5422746189403976a41698df939ffc0\
        32b31da3e6493f1393fd89226bf6b4eadf296725de20aac599c96edd2a445c9d689fee07ae5e94997f00659560\
        27632aa2cbfceba0ad1dcfeb7fb56c3a038718044af027a05c55e79d87a581fe5887ab1630360a4ddf22f18f4d\
        250a5385c3b6ee351d8d368543ed65e80461de207d9d924e8595b146d24defa8dbd01358a44fdf639ae28a852e\
        7675001a1c34c22013db15e7527ad9b7064b49c21ba11b7ed42ac9a570c006de5a092e84450d553767a5e2bb57\
        f588dd1a63559f4b5bf0f0b1eda574f319b65c5c97023caf2862a4487c403143538931bdaba7e1fd57d1d07dc4\
        613f75769c70427af74a7af461be5b0126125aa40920e4e9b44a9e28616dc9ca7b77624028a0ed40aaed08486d\
        90086f51f501e47b6f5e0f7391a8320f07d62774d2c29f3abfc9f71514e3bc471ff2041ede5c0fb4ad39572ab8\
        6909d5a368ef66086eed3f37dad57f14ec6362b1b9567a687e771330ec416265c437c3e6d3c3136aedfe753577\
        319aed775fdefcbe6ff046d9f93acca0d253617b9b98b73f7e7fc043ad2e5f9ae06754e4df0f8#5329",
        Greater,
    );
    // - out_prec <= p - 5 + Limb::WIDTH && xs_len > threshold + 1 in mul_float_significands_general
    // - xs_len > len in mul_float_significands_general
    // - k < len << 1 in mul_float_significands_general
    // - b1 == 0 second time in mul_float_significands_general
    // - !goto_full_multiply first time in mul_float_significands_general
    test(
        "0.000505125560873134723723784766950092518997332731967301291698755707702902596942317437852\
        941942371960388246351631317879977557751125153730732708319124734660713981958784935850223006\
        317900579689594830566320680544405297096706330277372404179805722699177498153458710988448597\
        667865038489642478166853872591228283694673924060010690084057961646317421857676637869623134\
        589158734829608080525671151880734425152367375882305121472858495571639962287931732234443489\
        588015865732596365983920824372468289009819493599234552103162449485200505569599322042323513\
        171853918367347759762125056867136354464299283595190704290061720089955391730926571989543650\
        123824756",
        "0x0.00211a99c3aab8ff53e9907564934b7d8b29ea0189ec9cd3d1a74407e1344b68fcd1809f4234bfef08c0c\
        3f89af1fe509093292b5b157eded4009179fedceae065ab0dd9d53b35a4ce7f1d190711e92618f7d0b216ce0bd\
        09e9c6f6625bcc6003ad38389decfb93a34f29b624c9c3c24dd825a59d15423602847fb2fe248d2b631514d4ed\
        f610fd7088faa3f59c46a406f42343787142069c527e5b0aa730ef1f4054d887515ccd31c4f04063d57d8645a7\
        b70fd198011231617f8b64344b5d43eb951d0a8ebfe39606336b443b19074cd641a63b4656b6e71133e47c099e\
        1fed7bc661252f72ee2c68b5cf9db6ee0645b455fb007e9d294c3a5e091df7f92b6268fff2e65b109ec#2100",
        "1.594827021209513894575272853189101796908969657496052553026904808146944132849277331696165\
        366066982899056554189139962942976395082328192559670863685439930018276703216617917370998670\
        143663891402542565265723410284904517798491873969116076521737839809230852429183992755715781\
        991743918659448701680599805386273131215689278489918563344703836788128584757613316503273812\
        347176374414975252562793531554358724296424942636927017522952046066543039832692600611375088\
        044395866209370211970587133626417624012062511750942538348241762511210915523305672594074841\
        5529943020302456665999104875220004206872624343740835605816652e-79",
        "0x4.ba3e1591f6a0ae18e8a750107efc0ca278b3954588d15a3695387227774da4319070b249d295916c24713\
        070f060a0df67ad5c2e8cc350d13189bcbf73f9af1e33fe42bf44fe99164bf802f7a5f41fe2d28377f1932b019\
        a00373f4400d62f20693ce9fbe239afbc3c804face7c59fb40fe03be617f14dd6880b31ff429280aafe8ea5d94\
        c3349efe67683ab81b8b5f4d00925073051d872bde743109c8e7c79da2c5fff9f489b12c271aa83405fe1d99c2\
        9f846c6b1c71dec233f13240569b3015559ed53bda719283585ed3f94bdeaea4d8438fe90e633bb97ee755eb88\
        113450187e9b05d8e6040fa53e6a10b6dcc2f01463a3937f5a90E-66#1988",
        Nearest,
        "8.055878935840864338761112052964131694842078144914087985110265043132694138227764882097467\
        485603629304908508159498533271385147841989466565220242546894501115782558008761861379571521\
        691942027516784202151149343065118307142127390742346695428089929852244559471690807529077258\
        667004534037323942848543095527661248910413746226405479291587579546107337446543923963731652\
        263604834014601264189816875633028978567676911887046995279630281027980542407933148412976200\
        257662826354908783390381610432026516882234795071151533161797811544609495678329417798803859\
        814384620434259660366163853727351148214688205975873187006097686502581917099916584703699137\
        7927e-83",
        "0x9.c7fc20220e43b5ac1717bf3fc467495e7a66c020cb393887e4bf4a3ad3eb4f20763507aea5b673c5aff25\
        8525ec5ac9bbc9f5c976983bce6231282e951654f346e3e5f7779ecdeae718176d55b03e21297ae13b32728a38\
        30c19eb847a199cde2fd0f3aa2330282fc433000c5f99b14022cd881da6b7caa98d3563644c4710ec876c4b61c\
        b15fdd4cc7d712644bad3a66e844700eafebabf02dead4fd71d95a14ef9c6a12db9092cf7b4723f45cb086b401\
        8123e185be43126baa17ab68fd32a0834023652a003d24211dd87066d2d5b1f0f1993c0dd9756329316927dd98\
        425ba5d470abc9065c6ff724d0e16c68152314d36e37d235fe078572e42d94540fe4418a9440633E-69#2100",
        Greater,
    );
    // - MPFR_MULHIGH_TAB[n] == -1 in limbs_float_mul_high_same_length
    // - k.is_none() in limbs_float_mul_high_same_length
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
        "5.713957232680308157039243396854396365207579750474509016836068310655183904589399398864949\
        369194427543435776465300666190725186454505340683874976519507548279996486761744005041118935\
        948747589743699350970808906395296086217435990867715483224265504817855574934867156151545781\
        106814865537845359234760816964504042844539777879186641559132734209465852287196566458935778\
        830300266229578325422723437672411534085537936547389438107265754858071594639985760821104661\
        940648845072022419465943642338099526948876829384943161258695569354371632958134560717841147\
        840574265292252548747529841143166165808068071464187327019844718180901133673499079508336221\
        530763161764985656092293178070670798408664087435197003752408586649867017671653317949434498\
        7804832149608135310831412140057788625216612318872425368846162e-33",
        "0x1.dab26e4414e016457a3b64d6dc588a9f2a4cbec7acd1a5b9033e00168a27a1ac3f669e39332cdbace29d9\
        b5d727df53a041f44f1389d6fda544d07799d35a62fbf327a55fca19ed803e80954753bfb12dcfda81be99eda5\
        2d7a382f6dd0ac26ab4b55fa3587bc812156b0ddb289357e7300ea3dafa43cee4f98c4068d3ffbeceea4a01ecc\
        74a09b85289f4c37000efab1e07926e01496cf6855b4a1f08915fff56a1737ff1d109fb6f3011fb249b88db85f\
        52980590101cbaa0e8af5872ded15ef0f39ec69d2d19d45c2972793e60bbb6eee9b673f24b3318973256a8e9bc\
        90d40f033e145dc56a6caa7e542a2701c0a41519fd0ef8d97ed0cef3e4296de50e877edeae80c0dd91f9082539\
        71e8998ef2a91d865baa5aa565e308789af81b128e8d0e832f5368e689962335ca7077ad875ea20eae1f2fc375\
        a972b8e7b8e285bbe3bdb0E-27#2587",
        Less,
    );
    // - xs_len <= len in mul_float_significands_general
    // - limbs_float_can_round in mul_float_significands_general
    test(
        "2791656999165.493196894581080462482671477586959884065253358835066410305633620155930725019\
        490863414181814174955842522364865024774379310519058136940369290858518517313460149216061827\
        231421960818181308820010416988884966114071973567666857448623899431042502611223036510238920\
        700493077403107131238955287441775846760482666068674778697270732066126184067657756561516984\
        251159820991198761865419427067747698722303345927460124328666182849055120607788400162794652\
        0878157328543259919881",
        "0x289fbb948fd.7e4226d4b6e97c4ccb64c2a072193f46e582b56e431d1d873706fd936c5e3328fd5b164ebe6\
        f6c3319057366a2de168eaef87f67644a657027f176fb9d5deaa62912e24d9b1eebf67a642c7ebc91e95eba928\
        d2c47f45ab981a3053c2a055e2a36102f8c4b2c72f24edbf87122ddff42408c1c95b6eccee7c0a49a25e2fe2fd\
        67a0dfc9d33ccad11d273f88f3e5a176da6b5989e6fc247727a58af5ef0f3a863e35d59edceb1ed0401bb2ce98\
        77227cdff6442212230b17fb43a061516c#1558",
        "2418611879271263.303076595070873673412348926925363768438166046534014463089969421888804109\
        398849144724043021994927400943502019398552005514731676646297869532250197280910814394530400\
        370821284006890204265425587260634300563572770382580394115150082129924900080499709146475799\
        383767996765000283568589472197582158015457869515972738606414178902063354061536898088295371\
        641586286472920893106777705320869713042818435525510767840203306234890813618032983465299395\
        730347727963352100742341245602143",
        "0x897b6f5e9db5f.4d966d80032f1615c15b6d17e02b100d2d97b52de1b561199b3dbeda9276bc25c5519d480\
        c2c20f52155b94074ad9db0bd7a97dae0337b738b6709c5fdd1bbcf1492bef975c393313e477790485d0a4ed0f\
        8790d9d469d65c29122cd019f7ff423e0f74c0585cac3aa85ceee8cbd256a201e23dd78f6dc402aa36b5a053ea\
        c1a3b2c435ae84ee9942bec1d88e41507bee030c35cd3f18b2f7a5c9193976d7418a633e08ca4923be03a2dc93\
        6ed64c6ff04ace32f44aa72f58bf488b9bf31c4c03f#1596",
        Nearest,
        "6751934781032429031553446805.854936199324254228337806577049209187291041079391491758652444\
        360751832908394550062618692018471995917689775126406303977393164343146088816585483538887454\
        027840163444916716206473736192766508247978473183426254779355642987433462433675113794019449\
        629515518752711445976665575431349720824019816455254142652806744349728110370084378693365847\
        376950483953541149499814845310185214082773221620162084856311318860928357827312318714828967\
        076794760533141841485387490188815",
        "0x15d111cde75ec690f0fb3395.dadd194843a336fd8b04fc1edf686f76aa52a00d00db09b133bcedaa832e64\
        e3cbe80e635c5bb60e81bb4cb3839dc581a1f683d480b6757136ace3d394264c5f969b75cb4d985b4b2b1096d2\
        a53a6d6570c7f7c917bed4f2a9c8f7bcdea046337457665b27e5e01bdbce6f20c6d5bc9e463f055cca9b0c53a2\
        f1f5b6d7211ac001a5fa99daf3378815db2c7bc77c6d6b76f815ae464fdc70ae82752dbe2b5683cc1ebc18275d\
        4685cbd35d58b70dbda84d2297ff9a12d20f2283678e#1596",
        Greater,
    );
    // - out_prec <= p - 5 in mul_float_significands_general
    test(
        "7.449979045041702122541548973189889723858373710052883535770432677692650998750696470703091\
        744255868088920360332810996981821635561095981209970543165990341865465399250649125818133550\
        345620227288485003961432095280508748897878683743133302349812547742789930883805941444651209\
        784236601329970891014476723035066837590220224883282604585022340111695027909185310636007609\
        680407882966239499164648695501638941698641158401193435378725738878261400237405551514713128\
        836982995938875007304023367379668047357772772649359285993284186179803252277e-21",
        "0x2.32e78d682285335802e277865ca4c9a80daafb30e4f67daedb2f466845a2fba5d9b1c96b068c42999ae0b\
        463fb94e578b359027630c01a995d88a74b72186c316ec30a8b4238fb27273b57cc5e72fa518fa14032e99af11\
        e1d5ccfc1217be504bf8dae0d1ec7e885fc95b51f444d68ed47eb588644910e4196d65104707dcd1c488a22e9c\
        1ef05ce85b3823fd6aa2bb7ad7d14fa1da7a628e1ba344d98ad7e95bf7e01883afc84273614fb8387e4035b2d9\
        ad867bc2be83eaabaa44df896348f52d4972bc5e8cf58f1cfd421cc30920d9f5504a043149274f5E-17#1738",
        "4.689598630661113758052361654518074651900718126313070505518580809308545656249466966444734\
        190644625007707301196274229099309347916384836732990153197811248723616418486524781737017155\
        222220872736570797023388780311017887508289881051954626904239986311860650999175535050592519\
        124458883175586921775053060798371665433755864916689805023189965297596140362143269558535567\
        196585839368205277548820051354051306095222442551993791324926248616628216018709202221063695\
        130497558241752004759655263919857509980778615592531001107310020150056419e-20",
        "0xd.d75c4c85ded4dada979685c90590eb9f191595aa1862df62f276fa75d5d790d2c00d187fe126b806534fd\
        0379dfcf600aeee23bbe00ed5955cc2e11b7de967da0c347d3ca03714f55b714dbe2c9ad31dbd7b6fb9a639fa4\
        da6cec918f3fc77bbca5ded28446186fe8b4224339c0722fed5f69b2d07571b12f4cce6fece51fc63ab46ea7fe\
        5c1f350ed6aab2c76a1e817bf652a9c77a95e3e3c107ed65e9ace86dfec217fb9dcbba544e3d84b0f3cac6c221\
        176c57f1db9bc306f22db85308dab755243a502d385e1b0a31f2f8c2d53c98729cfe52f82289cE-17#1730",
        Nearest,
        "3.493741152808155821061780436862260285420312736969910829282351541785101293399126489805777\
        980187532032743650816868281893520821921299562434265222876601442289649987508199259232837158\
        559068445030886433472805552651673239073165198885867344396236299229232184802565958836951408\
        637905144866851014091996571320623346167534428281388860566996500707311335172658622534949254\
        838518529528764394710349462443386828046760610922770501616700810546292498376696602122491857\
        8540047731481736443609736067164917894729323015869507020379275317039780244947e-40",
        "0x1.e6f4d9986216e186a3b15da557e0677276dd590e3d92a71ff89974ad2abac1a1e2e3dc761157b14cfef04\
        285b10c4bf2f72117bbdbdfd5ead95a5ed69f62f69682442a59a3f3c8a5a7b41238f0a029b2d53c54a08053451\
        35f5fc0a67464283bade3997992eea966997d442d69987820628966dfcaa6d25ca0be2df6ed92e5fa5201002da\
        70c6447c3a8a64ebd34fc04156c8dbd224d772ba1059982a5aa333d301a708dc568330bdead12245127d520b32\
        b4e7d9ae221552ab3d3f5f2f139c90118fe830b6adc84392948585b40267beb9fd60cce93fbed140E-33#1738",
        Greater,
    );
    // - xs[0] == 0 && xs[1] == 0 || ys[0] == 0 && ys[1] == 0 in mul_float_significands_general
    // - xs[xs_offset] == 0 in mul_float_significands_general
    // - ys[ys_offset] == 0 in mul_float_significands_general
    test(
        "4.44089209850062616169452667236328125e-16",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000E-13#2587",
        "6.938893903907228377647697925567627621275394370123920756842177449687616281576087749649243\
        449560881562252974476503855378704420289043275360540974361356347799301147460937500000000000\
        428087763478203718701697872488500162623204828539850902982930587576291943616315378569169353\
        840957917417166522314320167278698048638040260019989045715363127222769643922793712163996851\
        225018060026136483495536368092685586468682354268058778363979622204058653592623442621394947\
        3239996744456973724770155e-18",
        "0x8.0000000000000000000000000003fff000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000007fffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fe00000000000000000000000000000000000E-15#1571",
        Nearest,
        "3.081487911019577364889564708135884006379343333980678546187704758135879626468817238227001\
        919863959483626636205013163029555487554952753086708205626001498966504653625325671484347367\
        558282010814620352468694740583510310926487120637351903244174605756585424927747220355720149\
        705262186320261062950329818867477826244378862350354462458385252797818784708103173459291235\
        769538479528034217611825504020777438216019896385615152551542382665798426796321259955655479\
        300773291504754679549289638268112862439028560817297535629039872109954348969232817790940901\
        772925125639499661447304917621775611916484032847611462965461660088172016929375670794716891\
        215288121297398166137896866022540415673603027116093396582728756514564426357106145459651571\
        0636500728956428014332817951864653686798458732591874037159276e-33",
        "0x1.00000000000000000000000000007ffe00000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000fffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffc000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000E-27#2587",
        Equal,
    );
    // - xs[xs_offset] != 0 in mul_float_significands_general
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
        "1.577721810442023610823457130565572459346412870218046009540557861328124999999781992456191\
        582683140605249728137786969774247203293102032637435297463867968227730622206111553159326620\
        257748727154917100482837417166916131758491078680030585867783733868391940878208953914079279\
        677271330675596139325733702207799069583415985107421875000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000001527219401e-30",
        "0x1.ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000007ffffff8E-25#2567",
        Greater,
    );
    // - ys[ys_offset] != 0 in mul_float_significands_general
    test(
        "3.552713678800500929355621337890625000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000185653473271011701515143789632334288014147578747413968011889263434165\
        454031003144878632958696545103828379944287431463737771314691776232764018029744516848435369\
        906941507005331339097431189921414095915636548343770290467007831930064077297413615608072068\
        77626957251331627162116234578193867366731943186452308784033031e-15",
        "0x1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000003fffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffE-12#1693",
        "131071.9999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999931137299504668061215619580033350122879947175743013377219720120678876536\
        628643562946337394277177628286886258713868752206334184193331360816755408598724051586961746\
        685495576886231160425736851019484652776925452038003143395027472798288565900387988918967170\
        024140096215282631707814129061246845336549291242702672707242154281040996134116689237194330\
        449329247059466983779402165943011772702916829761392820351107526820413839207822952585392346\
        328868866364548445701244431290159307431187837005854773744122693182105660536394809595147776\
        165023610906858577556273686957594781817444085841499342583068122729709267291",
        "0x1ffff.fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        fffffffffffffffffff80000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffc0000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000#2930",
        Nearest,
        "4.656612873077392578125000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000243339720485780457409929267946893197985903514415810436152543495368429\
        343907516442055321546973277589582124346598388556070043505489984826143425305512049232107258\
        133592644143603087675038065556842214758248268059151846952539521827810268928481776834311731\
        044052486308720793745225556140061733844701779522496735016181906708106564202059526039890815\
        919178696765989061598664073635409369732679923290228161362302668920197866351570235145652815\
        456445767888399222146130209012884306181795683184194476165898983356347814975458163826496169\
        408195862006279548822773547655683272930717806964759394377376260006554734212838956379338694\
        525902352837865322055484704841599472083306918235788680977972181745262154636e-10",
        "0x2.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000007fffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffefffffffffffffffffffdffffffffffffffffffffff\
        fffffffffffffffffff80000000000000000000000000000000008000000000000000000000000000000000000\
        000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffdfffffffffffc0000000000000000004000000000000000000000000000000000000000000000000000000\
        00080000000000000E-8#2930",
        Greater,
    );
    test(
        "12962368.00910437392826314193446550944389165836262015621514962112057577196019471161937054\
        243429475140935082492484312458697139062031326336320065697050792374770657213779646419924892\
        886900897092216274606911219143642483069838857046000551685989850622347518286427108428594195\
        826642547794926862213089792293539194572777838215978068770604450325322408918968488729618206\
        83239161602000658143319869428",
        "0xc5ca40.0254aa0c45bef8b7e5bdc2c8d0bc3f4ef77512b0c665764acef9797020852473fed6cf5aef7ab76f\
        af4e7ccca7674b7b9a7d4e3adf7dbd2f9c51a93c8b500f9c6e799811cc9793ad3189530f5202900aed04c32d98\
        72075f4a8efb28f1e3d9e2065d70be0df4f07d46d4f2f48b95a6fcd718d2f233dbbaff41296fe46efd00783168\
        f9da607a651780d29e9357c1d8e9abdd149e2d7ed04e74488fff360#1283",
        "24678125950611051118469790273997953293778953666881566874553684447233373952852192.03220118\
        824573657786267791510531425783997288827451260529832388036858875235909753308200751110173525\
        014519161498781309811156911488470099945303550175018037705473098495478288721507648423524330\
        255884870759432606723320831807670552582364423881930823507999308667398128596287742877235432\
        154091338959855861235089997932459245142831892",
        "0xd51fdc4e02742b528c90f8a31146dcba3e1273bfdd1198ff1ec486c97e52f8c0e0.083e564a6864272a5d52\
        74cbdf67db38b72e576e7cfe8be5418f583cf4173a82939f37d7b6b399cada03434a24aef04a65323c333b33cf\
        622583787d8ec1f247c4f691c3a7862628783f4322923b6bb9434902754e348fb74eb6364e294832597785e445\
        1994ee48803ea5771d85e7f226166cdd1853477f296076996b97965596455eb50129#1336",
        Nearest,
        "319886950346849155967554337687681180827634107405044558954903133134156000777816460853971.0\
        571286460940042331570995057120143779176892645683546163312322051333969186064059757999062545\
        749424646225780605680908775358827191384406475576071818545005626321659954666901446716884049\
        318512517659256499382137861718683137930417158199590725586222075459373218000006096544204046\
        600400863088080659996213042623116710817104536",
        "0xa4a9e4f5c54093e3388df17b55e5724251f90cd6c231443e4bc0d5770919a2c1878982d3.0e9ffba2a37526\
        7ffbc880a862170226c003dcfce47863efaa79493245f971642ca10409a10ad584de1597974f309b61313528bd\
        1512f14b2397a5630d41d067d57d9b327d751eacb350f8e13428d4508849e4464775e72f3b4fb63d6bef2d97b6\
        f0118aa022becb98c8c88fdcb280078abcbe4c3b8423dc1dfc91e6b09ea9115cd0cd#1336",
        Greater,
    );
}

#[test]
fn mul_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.mul_round(THREE, Exact));
    assert_panic!(THREE.mul_round_val_ref(&THREE, Exact));
    assert_panic!(THREE.mul_round_ref_val(THREE, Exact));
    assert_panic!(THREE.mul_round_ref_ref(&THREE, Exact));

    assert_panic!({
        let mut x = THREE;
        x.mul_round_assign(THREE, Exact)
    });
    assert_panic!({
        let mut x = THREE;
        x.mul_round_assign_ref(&THREE, Exact)
    });
}

#[test]
fn test_mul_prec_round() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (product, o) = x.clone().mul_prec_round(y.clone(), prec, rm);
        assert!(product.is_valid());

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);
        assert_eq!(o, o_out);

        let (product_alt, o_alt) = x.clone().mul_prec_round_val_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = x.mul_prec_round_ref_val(y.clone(), prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = x.mul_prec_round_ref_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_prec_round_assign(y.clone(), prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_prec_round_assign_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o_out);

        let (product_alt, o_alt) = mul_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
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
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal,
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
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Exact,
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
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
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
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Floor,
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
        Ceiling,
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
        Down,
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
        Up,
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
        Nearest,
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
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "NaN",
        "NaN",
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
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
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

    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Exact, "-0.0", "-0x0.0", Equal,
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
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Exact, "-0.0", "-0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
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
        "-0.0", "-0x0.0", "-1.0", "-0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal,
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
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Down, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Up, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Exact, "-0.0", "-0x0.0", Equal,
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
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Exact,
        "123.0",
        "0x7b.0#10",
        Equal,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Down,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "123.0",
        "0x7b.0#7",
        10,
        Exact,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Floor, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Ceiling, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Down, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Up, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Nearest, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Exact, "2.0", "0x2.0#1", Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Floor, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Ceiling, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Down, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Up, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Nearest, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Exact, "2.0", "0x2.0#1", Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Floor, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Ceiling, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Down, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Up, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Nearest, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Exact, "2.0", "0x2.0#1", Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Floor, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Ceiling, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Down, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Up, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Nearest, "2.0", "0x2.0#1", Equal,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Exact, "2.0", "0x2.0#1", Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Exact,
        "2.0",
        "0x2.0#1",
        Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Up,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
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
        Floor,
        "4.44",
        "0x4.70#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "4.445",
        "0x4.72#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "4.44",
        "0x4.70#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "4.445",
        "0x4.72#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "4.445",
        "0x4.72#10",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-8.0",
        "-0x8.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "-8.0",
        "-0x8.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-4.445",
        "-0x4.72#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-4.44",
        "-0x4.70#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-4.44",
        "-0x4.70#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-4.445",
        "-0x4.72#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-4.445",
        "-0x4.72#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "-8.0",
        "-0x8.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "-8.0",
        "-0x8.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "-4.445",
        "-0x4.72#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-4.44",
        "-0x4.70#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "-4.44",
        "-0x4.70#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "-4.445",
        "-0x4.72#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-4.445",
        "-0x4.72#10",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "4.44",
        "0x4.70#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "4.445",
        "0x4.72#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "4.44",
        "0x4.70#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "4.445",
        "0x4.72#10",
        Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "4.445",
        "0x4.72#10",
        Greater,
    );
}

#[test]
fn mul_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).mul_prec_round(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).mul_prec_round_val_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).mul_prec_round_ref_val(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).mul_prec_round_ref_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.mul_prec_round_assign(Float::two_prec(1), 0, Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.mul_prec_round_assign_ref(&Float::two_prec(1), 0, Floor)
    });

    assert_panic!(THREE.mul_prec_round(THREE, 1, Exact));
    assert_panic!(THREE.mul_prec_round_val_ref(&THREE, 1, Exact));
    assert_panic!(THREE.mul_prec_round_ref_val(THREE, 1, Exact));
    assert_panic!(THREE.mul_prec_round_ref_ref(&THREE, 1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.mul_prec_round_assign(THREE, 1, Exact)
    });
    assert_panic!({
        let mut x = THREE;
        x.mul_prec_round_assign_ref(&THREE, 1, Exact)
    });
}

#[test]
fn test_mul_rational() {
    let test = |s, s_hex, t, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let product = x.clone() * y.clone();
        assert!(product.is_valid());

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);

        let product_alt = x.clone() * &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = &x * y.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = &x * &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        let product_alt = y.clone() * x.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = y.clone() * &x;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = &y * x.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let product_alt = &y * &x;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        let mut product_alt = x.clone();
        product_alt *= y.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        let mut product_alt = x.clone();
        product_alt *= &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_mul_rational(
                rug::Float::exact_from(&x),
                rug::Rational::from(&y)
            ))),
            ComparableFloatRef(&product)
        );

        let product_alt =
            mul_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
    };
    test("NaN", "NaN", "123", "NaN", "NaN");
    test("Infinity", "Infinity", "123", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "123", "-Infinity", "-Infinity");
    test("NaN", "NaN", "0", "NaN", "NaN");
    test("Infinity", "Infinity", "0", "NaN", "NaN");
    test("-Infinity", "-Infinity", "0", "NaN", "NaN");

    test("0.0", "0x0.0", "0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "123", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "123", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "-123", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "-123", "0.0", "0x0.0");
    test("0.0", "0x0.0", "1/3", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "1/3", "-0.0", "-0x0.0");
    test("123.0", "0x7b.0#7", "1", "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", "0", "0.0", "0x0.0");
    test("-123.0", "-0x7b.0#7", "0", "-0.0", "-0x0.0");

    test("1.0", "0x1.0#1", "2", "2.0", "0x2.0#1");
    test("1.0", "0x1.0#2", "2", "2.0", "0x2.0#2");
    test("1.0", "0x1.000#10", "2", "2.0", "0x2.00#10");
    test("1.0", "0x1.000#10", "1/3", "0.3335", "0x0.556#10");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
    );

    test("3.0", "0x3.0#2", "2", "6.0", "0x6.0#2");
    test("3.0", "0x3.00#10", "2", "6.0", "0x6.00#10");
    test("3.0", "0x3.00#10", "1/3", "1.0", "0x1.000#10");
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
    );
}

#[test]
fn test_mul_rational_prec() {
    let test = |s, s_hex, t, prec, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (product, o) = x.clone().mul_rational_prec(y.clone(), prec);
        assert!(product.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);

        let (product_alt, o_alt) = x.clone().mul_rational_prec_val_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_prec_ref_val(y.clone(), prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_prec_ref_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_rational_prec_assign(y.clone(), prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_rational_prec_assign_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) =
            mul_rational_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
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
    test("Infinity", "Infinity", "0", 1, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "0", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "0", 1, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "123", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "123", 1, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", "-123", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", "-123", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "1/3", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "1/3", 1, "-0.0", "-0x0.0", Equal);
    test("123.0", "0x7b.0#7", "1", 1, "1.0e2", "0x8.0E+1#1", Greater);
    test("123.0", "0x7b.0#7", "0", 1, "0.0", "0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", 1, "-0.0", "-0x0.0", Equal);

    test("1.0", "0x1.0#1", "2", 1, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", 10, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.0#2", "2", 1, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#2", "2", 10, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "2", 1, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.000#10", "2", 10, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "1/3", 1, "0.2", "0x0.4#1", Less);
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        10,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        1,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        10,
        "0.3335",
        "0x0.556#10",
        Greater,
    );

    test("3.0", "0x3.0#2", "2", 1, "8.0", "0x8.0#1", Greater);
    test("3.0", "0x3.0#2", "2", 10, "6.0", "0x6.00#10", Equal);
    test("3.0", "0x3.00#10", "2", 1, "8.0", "0x8.0#1", Greater);
    test("3.0", "0x3.00#10", "2", 10, "6.0", "0x6.00#10", Equal);
    test("3.0", "0x3.00#10", "1/3", 1, "1.0", "0x1.0#1", Equal);
    test("3.0", "0x3.00#10", "1/3", 10, "1.0", "0x1.000#10", Equal);
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        "1.0",
        "0x1.000#10",
        Equal,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        1,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        1,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
}

#[test]
fn mul_rational_prec_fail() {
    assert_panic!(Float::NAN.mul_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::NAN.mul_rational_prec_val_ref(&Rational::ZERO, 0));
    assert_panic!(Float::NAN.mul_rational_prec_ref_val(Rational::ZERO, 0));
    assert_panic!(Float::NAN.mul_rational_prec_ref_ref(&Rational::ZERO, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.mul_rational_prec_assign(Rational::ZERO, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.mul_rational_prec_assign_ref(&Rational::ZERO, 0)
    });
}

#[test]
fn test_mul_rational_round() {
    let test = |s, s_hex, t, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (product, o) = x.clone().mul_rational_round(y.clone(), rm);
        assert!(product.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);

        let (product_alt, o_alt) = x.clone().mul_rational_round_val_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_round_ref_val(y.clone(), rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_round_ref_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_rational_round_assign(y.clone(), rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_rational_round_assign_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_product, rug_o) = rug_mul_rational_round(
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_product)),
                ComparableFloatRef(&product)
            );
            assert_eq!(rug_o, o);
        }

        let (product_alt, o_alt) =
            mul_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
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

    test("Infinity", "Infinity", "0", Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", Down, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", Up, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", Exact, "NaN", "NaN", Equal);

    test("-Infinity", "-Infinity", "0", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", Exact, "NaN", "NaN", Equal);

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

    test("123.0", "0x7b.0#7", "0", Floor, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", Ceiling, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", Down, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", Up, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", Nearest, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", Exact, "0.0", "0x0.0", Equal);

    test("-123.0", "-0x7b.0#7", "0", Floor, "-0.0", "-0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", Down, "-0.0", "-0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", Up, "-0.0", "-0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", Nearest, "-0.0", "-0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", Exact, "-0.0", "-0x0.0", Equal);

    test("1.0", "0x1.0#1", "2", Floor, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", Ceiling, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", Down, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", Up, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", Nearest, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", Exact, "2.0", "0x2.0#1", Equal);

    test("1.0", "0x1.0#2", "2", Floor, "2.0", "0x2.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Ceiling, "2.0", "0x2.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Down, "2.0", "0x2.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Up, "2.0", "0x2.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Nearest, "2.0", "0x2.0#2", Equal);
    test("1.0", "0x1.0#2", "2", Exact, "2.0", "0x2.0#2", Equal);

    test("1.0", "0x1.000#10", "2", Floor, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "2", Ceiling, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "2", Down, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "2", Up, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "2", Nearest, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.000#10", "2", Exact, "2.0", "0x2.00#10", Equal);

    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        Floor,
        "0.333",
        "0x0.554#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        Ceiling,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        Down,
        "0.333",
        "0x0.554#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        Up,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        Nearest,
        "0.3335",
        "0x0.556#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Floor,
        "0.3333333333333333333333333333331",
        "0x0.55555555555555555555555550#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Ceiling,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Down,
        "0.3333333333333333333333333333331",
        "0x0.55555555555555555555555550#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Up,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        Nearest,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );

    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Floor,
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Ceiling,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Down,
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Up,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        Nearest,
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Floor,
        "-1.0471975511965979",
        "-0x1.0c152382d7366#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Ceiling,
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Down,
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Up,
        "-1.0471975511965979",
        "-0x1.0c152382d7366#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        Nearest,
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Floor,
        "-1.0471975511965979",
        "-0x1.0c152382d7366#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Ceiling,
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Down,
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Up,
        "-1.0471975511965979",
        "-0x1.0c152382d7366#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        Nearest,
        "-1.0471975511965976",
        "-0x1.0c152382d7365#53",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Floor,
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Ceiling,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Down,
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Up,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        Nearest,
        "1.0471975511965976",
        "0x1.0c152382d7365#53",
        Less,
    );
}

#[test]
fn mul_rational_round_fail() {
    const THREE_F: Float = Float::const_from_unsigned(3);
    const THREE_Q: Rational = Rational::const_from_unsigned(3);
    assert_panic!(THREE_F.mul_rational_round(THREE_Q, Exact));
    assert_panic!(THREE_F.mul_rational_round_val_ref(&THREE_Q, Exact));
    assert_panic!(THREE_F.mul_rational_round_ref_val(THREE_Q, Exact));
    assert_panic!(THREE_F.mul_rational_round_ref_ref(&THREE_Q, Exact));
    assert_panic!({
        let mut x = THREE_F;
        x.mul_rational_round_assign(THREE_Q, Exact)
    });
    assert_panic!({
        let mut x = THREE_F;
        x.mul_rational_round_assign_ref(&THREE_Q, Exact)
    });
}

#[test]
fn test_mul_rational_prec_round() {
    let test = |s, s_hex, t, prec, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (product, o) = x.clone().mul_rational_prec_round(y.clone(), prec, rm);
        assert!(product.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);

        let (product_alt, o_alt) = x.clone().mul_rational_prec_round_val_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_rational_prec_round_assign(y.clone(), prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let mut product_alt = x.clone();
        let o_alt = product_alt.mul_rational_prec_round_assign_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = mul_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
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

    test("Infinity", "Infinity", "0", 1, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", 1, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", 1, Down, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", 1, Up, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "0", 1, Exact, "NaN", "NaN", Equal);

    test("-Infinity", "-Infinity", "0", 1, Floor, "NaN", "NaN", Equal);
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test("-Infinity", "-Infinity", "0", 1, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", "0", 1, Up, "NaN", "NaN", Equal);
    test(
        "-Infinity",
        "-Infinity",
        "0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("-Infinity", "-Infinity", "0", 1, Exact, "NaN", "NaN", Equal);

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

    test("123.0", "0x7b.0#7", "0", 1, Floor, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", 1, Down, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", 1, Up, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("123.0", "0x7b.0#7", "0", 1, Exact, "0.0", "0x0.0", Equal);

    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("-123.0", "-0x7b.0#7", "0", 1, Down, "-0.0", "-0x0.0", Equal);
    test("-123.0", "-0x7b.0#7", "0", 1, Up, "-0.0", "-0x0.0", Equal);
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        "0",
        1,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );

    test("1.0", "0x1.0#1", "2", 1, Floor, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Ceiling, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Down, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Up, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Nearest, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", "2", 1, Exact, "2.0", "0x2.0#1", Equal);

    test("1.0", "0x1.0#1", "2", 10, Floor, "2.0", "0x2.00#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        "2",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("1.0", "0x1.0#1", "2", 10, Down, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.0#1", "2", 10, Up, "2.0", "0x2.00#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        "2",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("1.0", "0x1.0#1", "2", 10, Exact, "2.0", "0x2.00#10", Equal);

    test("1.0", "0x1.0#2", "2", 1, Floor, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#2", "2", 1, Ceiling, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#2", "2", 1, Down, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#2", "2", 1, Up, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#2", "2", 1, Nearest, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#2", "2", 1, Exact, "2.0", "0x2.0#1", Equal);

    test("1.0", "0x1.0#2", "2", 10, Floor, "2.0", "0x2.00#10", Equal);
    test(
        "1.0",
        "0x1.0#2",
        "2",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("1.0", "0x1.0#2", "2", 10, Down, "2.0", "0x2.00#10", Equal);
    test("1.0", "0x1.0#2", "2", 10, Up, "2.0", "0x2.00#10", Equal);
    test(
        "1.0",
        "0x1.0#2",
        "2",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("1.0", "0x1.0#2", "2", 10, Exact, "2.0", "0x2.00#10", Equal);

    test("1.0", "0x1.000#10", "2", 1, Floor, "2.0", "0x2.0#1", Equal);
    test(
        "1.0",
        "0x1.000#10",
        "2",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test("1.0", "0x1.000#10", "2", 1, Down, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.000#10", "2", 1, Up, "2.0", "0x2.0#1", Equal);
    test(
        "1.0",
        "0x1.000#10",
        "2",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Equal,
    );
    test("1.0", "0x1.000#10", "2", 1, Exact, "2.0", "0x2.0#1", Equal);

    test(
        "1.0",
        "0x1.000#10",
        "2",
        10,
        Floor,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        10,
        Ceiling,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        10,
        Down,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test("1.0", "0x1.000#10", "2", 10, Up, "2.0", "0x2.00#10", Equal);
    test(
        "1.0",
        "0x1.000#10",
        "2",
        10,
        Nearest,
        "2.0",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    );

    test("1.0", "0x1.000#10", "1/3", 1, Floor, "0.2", "0x0.4#1", Less);
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test("1.0", "0x1.000#10", "1/3", 1, Down, "0.2", "0x0.4#1", Less);
    test("1.0", "0x1.000#10", "1/3", 1, Up, "0.5", "0x0.8#1", Greater);
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        1,
        Nearest,
        "0.2",
        "0x0.4#1",
        Less,
    );

    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        10,
        Floor,
        "0.333",
        "0x0.554#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        10,
        Ceiling,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        10,
        Down,
        "0.333",
        "0x0.554#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        10,
        Up,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        10,
        Nearest,
        "0.3335",
        "0x0.556#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        1,
        Down,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        1,
        Up,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        1,
        Nearest,
        "0.2",
        "0x0.4#1",
        Less,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        10,
        Floor,
        "0.333",
        "0x0.554#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        10,
        Ceiling,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        10,
        Down,
        "0.333",
        "0x0.554#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        10,
        Up,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        10,
        Nearest,
        "0.3335",
        "0x0.556#10",
        Greater,
    );

    test("3.0", "0x3.0#2", "2", 1, Floor, "4.0", "0x4.0#1", Less);
    test("3.0", "0x3.0#2", "2", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("3.0", "0x3.0#2", "2", 1, Down, "4.0", "0x4.0#1", Less);
    test("3.0", "0x3.0#2", "2", 1, Up, "8.0", "0x8.0#1", Greater);
    test("3.0", "0x3.0#2", "2", 1, Nearest, "8.0", "0x8.0#1", Greater);

    test("3.0", "0x3.0#2", "2", 10, Floor, "6.0", "0x6.00#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        "2",
        10,
        Ceiling,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test("3.0", "0x3.0#2", "2", 10, Down, "6.0", "0x6.00#10", Equal);
    test("3.0", "0x3.0#2", "2", 10, Up, "6.0", "0x6.00#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        "2",
        10,
        Nearest,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test("3.0", "0x3.0#2", "2", 10, Exact, "6.0", "0x6.00#10", Equal);

    test("3.0", "0x3.00#10", "2", 1, Floor, "4.0", "0x4.0#1", Less);
    test(
        "3.0",
        "0x3.00#10",
        "2",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test("3.0", "0x3.00#10", "2", 1, Down, "4.0", "0x4.0#1", Less);
    test("3.0", "0x3.00#10", "2", 1, Up, "8.0", "0x8.0#1", Greater);
    test(
        "3.0",
        "0x3.00#10",
        "2",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Greater,
    );

    test(
        "3.0",
        "0x3.00#10",
        "2",
        10,
        Floor,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "2",
        10,
        Ceiling,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test("3.0", "0x3.00#10", "2", 10, Down, "6.0", "0x6.00#10", Equal);
    test("3.0", "0x3.00#10", "2", 10, Up, "6.0", "0x6.00#10", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "2",
        10,
        Nearest,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "2",
        10,
        Exact,
        "6.0",
        "0x6.00#10",
        Equal,
    );

    test("3.0", "0x3.00#10", "1/3", 1, Floor, "1.0", "0x1.0#1", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test("3.0", "0x3.00#10", "1/3", 1, Down, "1.0", "0x1.0#1", Equal);
    test("3.0", "0x3.00#10", "1/3", 1, Up, "1.0", "0x1.0#1", Equal);
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test("3.0", "0x3.00#10", "1/3", 1, Exact, "1.0", "0x1.0#1", Equal);

    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        10,
        Floor,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        10,
        Ceiling,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        10,
        Down,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        10,
        Up,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.00#10",
        "1/3",
        10,
        Exact,
        "1.0",
        "0x1.000#10",
        Equal,
    );

    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        1,
        Exact,
        "1.0",
        "0x1.0#1",
        Equal,
    );

    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        Floor,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        Ceiling,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        Down,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        Up,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0000000000000000000000000#100",
        "1/3",
        10,
        Exact,
        "1.0",
        "0x1.000#10",
        Equal,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Floor,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Ceiling,
        "1.049",
        "0x1.0c8#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Down,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Up,
        "1.049",
        "0x1.0c8#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        Nearest,
        "1.047",
        "0x1.0c0#10",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Down,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Up,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Floor,
        "-1.049",
        "-0x1.0c8#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Ceiling,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Down,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Up,
        "-1.049",
        "-0x1.0c8#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Nearest,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        1,
        Down,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        1,
        Up,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Floor,
        "-1.049",
        "-0x1.0c8#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Ceiling,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Down,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Up,
        "-1.049",
        "-0x1.0c8#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        Nearest,
        "-1.047",
        "-0x1.0c0#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Floor,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Ceiling,
        "1.049",
        "0x1.0c8#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Down,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Up,
        "1.049",
        "0x1.0c8#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        Nearest,
        "1.047",
        "0x1.0c0#10",
        Less,
    );
}

#[test]
fn mul_rational_prec_round_fail() {
    assert_panic!(Float::one_prec(1).mul_rational_prec_round(
        Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).mul_rational_prec_round_val_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).mul_rational_prec_round_ref_val(
        Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!(Float::one_prec(1).mul_rational_prec_round_ref_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.mul_rational_prec_round_assign(Rational::from_unsigneds(5u32, 8), 1, Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(5u32, 8), 1, Exact)
    });
}

#[test]
fn mul_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(|(x, y, prec, rm)| {
        let (product, o) = x.clone().mul_prec_round(y.clone(), prec, rm);
        assert!(product.is_valid());
        let (product_alt, o_alt) = x.clone().mul_prec_round_val_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
        let (product_alt, o_alt) = x.mul_prec_round_ref_val(y.clone(), prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
        let (product_alt, o_alt) = x.mul_prec_round_ref_ref(&y, prec, rm);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.mul_prec_round_assign(y.clone(), prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.mul_prec_round_assign_ref(&y, prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = mul_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let r_product = if product.is_finite() {
            if product.is_normal() {
                assert_eq!(product.get_prec(), Some(prec));
            }
            let r_product = Rational::exact_from(&x) * Rational::exact_from(&y);
            assert_eq!(product.partial_cmp(&r_product), Some(o));
            if o == Less {
                let mut next = product.clone();
                next.increment();
                assert!(next > r_product);
            } else if o == Greater {
                let mut next = product.clone();
                next.decrement();
                assert!(next < r_product);
            }
            Some(r_product)
        } else {
            assert_eq!(o, Equal);
            None
        };

        match (
            r_product.is_some() && *r_product.as_ref().unwrap() >= 0u32,
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

        let (product_alt, o_alt) = y.mul_prec_round_ref_ref(&x, prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let (mut product_alt, mut o_alt) = x.mul_prec_round_ref_val(-&y, prec, -rm);
        product_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(product_alt.abs_negative_zero()),
            ComparableFloat(product.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (mut product_alt, mut o_alt) = (-&x).mul_prec_round_val_ref(&y, prec, -rm);
        product_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(product_alt.abs_negative_zero()),
            ComparableFloat(product.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = (-&x).mul_prec_round(-&y, prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = x.mul_prec_round_ref_ref(&y, prec, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(product.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(x.mul_prec_round_ref_ref(&y, prec, Exact));
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (product, o) = x.mul_prec_round_ref_val(Float::NAN, prec, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        let (product, o) = Float::NAN.mul_prec_round_val_ref(&x, prec, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x != 0 {
                assert_eq!(
                    x.mul_prec_round_ref_val(Float::INFINITY, prec, rm),
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
                    Float::INFINITY.mul_prec_round_val_ref(&x, prec, rm),
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
                    x.mul_prec_round_ref_val(Float::NEGATIVE_INFINITY, prec, rm),
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
                    Float::NEGATIVE_INFINITY.mul_prec_round_val_ref(&x, prec, rm),
                    (
                        if x > 0 {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
            }
            if x.is_finite() {
                assert_eq!(
                    x.mul_prec_round_ref_val(Float::ZERO, prec, rm),
                    (Float::ZERO, Equal)
                );
                assert_eq!(
                    Float::ZERO.mul_prec_round_val_ref(&x, prec, rm),
                    (Float::ZERO, Equal)
                );
                assert_eq!(
                    x.mul_prec_round_ref_val(Float::NEGATIVE_ZERO, prec, rm),
                    (Float::ZERO, Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO.mul_prec_round_val_ref(&x, prec, rm),
                    (Float::ZERO, Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (product, o) = x.mul_prec_round_ref_val(Float::ONE, prec, rm);
            let mut product_alt = x.clone();
            let o_alt = product_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
            assert_eq!(o, o_alt);

            let (product, o) = Float::ONE.mul_prec_round_val_ref(&x, prec, rm);
            let mut product_alt = x.clone();
            let o_alt = product_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
            assert_eq!(o, o_alt);
        }
    });
}

fn mul_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (product, o) = x.clone().mul_prec(y.clone(), prec);
    assert!(product.is_valid());
    let (product_alt, o_alt) = x.clone().mul_prec_val_ref(&y, prec);
    assert!(product_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);
    let (product_alt, o_alt) = x.mul_prec_ref_val(y.clone(), prec);
    assert!(product_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);
    let (product_alt, o_alt) = x.mul_prec_ref_ref(&y, prec);
    assert!(product_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.mul_prec_assign(y.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.mul_prec_assign_ref(&y, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
    assert_eq!(o_alt, o);

    let (product_alt, o_alt) = mul_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    let (product_alt, o_alt) = x.mul_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    if product.is_finite() {
        if product.is_normal() {
            assert_eq!(product.get_prec(), Some(prec));
        }
        let r_product = Rational::exact_from(&x) * Rational::exact_from(&y);
        assert_eq!(product.partial_cmp(&r_product), Some(o));
        if o == Less {
            let mut next = product.clone();
            next.increment();
            assert!(next > r_product);
        } else if o == Greater {
            let mut next = product.clone();
            next.decrement();
            assert!(next < r_product);
        }
    } else {
        assert_eq!(o, Equal);
    }

    let (product_alt, o_alt) = y.mul_prec_ref_ref(&x, prec);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        let (mut product_alt, mut o_alt) = x.mul_prec_ref_val(-&y, prec);
        product_alt.neg_assign();
        product_alt.abs_negative_zero_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let (mut product_alt, mut o_alt) = (-&x).mul_prec_val_ref(&y, prec);
        product_alt.neg_assign();
        product_alt.abs_negative_zero_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = (-x).mul_prec(-y, prec);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
    }
}

#[test]
fn mul_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        mul_prec_properties_helper(x, y, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_float_unsigned_triple_gen_var_1().test_properties_with_config(&config, |(x, y, prec)| {
        mul_prec_properties_helper(x, y, prec);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (product, o) = x.mul_prec_ref_val(Float::NAN, prec);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        let (product, o) = Float::NAN.mul_prec_val_ref(&x, prec);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x != 0 {
                assert_eq!(
                    x.mul_prec_ref_val(Float::INFINITY, prec),
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
                    Float::INFINITY.mul_prec_val_ref(&x, prec),
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
                    x.mul_prec_ref_val(Float::NEGATIVE_INFINITY, prec),
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
                    Float::NEGATIVE_INFINITY.mul_prec_val_ref(&x, prec),
                    (
                        if x > 0 {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
            }
            if x.is_finite() {
                assert_eq!(x.mul_prec_ref_val(Float::ZERO, prec), (Float::ZERO, Equal));
                assert_eq!(Float::ZERO.mul_prec_val_ref(&x, prec), (Float::ZERO, Equal));
                assert_eq!(
                    x.mul_prec_ref_val(Float::NEGATIVE_ZERO, prec),
                    (Float::ZERO, Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO.mul_prec_val_ref(&x, prec),
                    (Float::ZERO, Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (product, o) = x.mul_prec_ref_val(Float::ONE, prec);
            let mut product_alt = x.clone();
            let o_alt = product_alt.set_prec(prec);
            assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
            assert_eq!(o, o_alt);

            let (product, o) = Float::ONE.mul_prec_val_ref(&x, prec);
            let mut product_alt = x.clone();
            let o_alt = product_alt.set_prec(prec);
            assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
            assert_eq!(o, o_alt);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn mul_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (product, o) = x.clone().mul_round(y.clone(), rm);
    assert!(product.is_valid());
    let (product_alt, o_alt) = x.clone().mul_round_val_ref(&y, rm);
    assert!(product_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let (product_alt, o_alt) = x.mul_round_ref_val(y.clone(), rm);
    assert!(product_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let (product_alt, o_alt) = x.mul_round_ref_ref(&y, rm);
    assert!(product_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.mul_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.mul_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
    assert_eq!(o_alt, o);

    let (product_alt, o_alt) = mul_prec_round_naive(
        x.clone(),
        y.clone(),
        max(x.significant_bits(), y.significant_bits()),
        rm,
    );
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);
    let (product_alt, o_alt) =
        x.mul_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), rm);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    let r_product = if product.is_finite() {
        if x.is_normal() && y.is_normal() && product.is_normal() {
            assert_eq!(
                product.get_prec(),
                Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
            );
        }
        let r_product = Rational::exact_from(&x) * Rational::exact_from(&y);
        assert_eq!(product.partial_cmp(&r_product), Some(o));
        if o == Less {
            let mut next = product.clone();
            next.increment();
            assert!(next > r_product);
        } else if o == Greater {
            let mut next = product.clone();
            next.decrement();
            assert!(next < r_product);
        }
        Some(r_product)
    } else {
        assert_eq!(o, Equal);
        None
    };

    match (
        r_product.is_some() && *r_product.as_ref().unwrap() >= 0u32,
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
        let (rug_product, rug_o) =
            rug_mul_round(rug::Float::exact_from(&x), rug::Float::exact_from(&y), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_product)),
            ComparableFloatRef(&product),
        );
        assert_eq!(rug_o, o);
    }

    let (product_alt, o_alt) = y.mul_round_ref_ref(&x, rm);
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );

    let (mut product_alt, mut o_alt) = x.mul_round_ref_val(-&y, -rm);
    product_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(product_alt.abs_negative_zero()),
        ComparableFloat(product.abs_negative_zero_ref())
    );

    let (mut product_alt, mut o_alt) = (-&x).mul_round_val_ref(&y, -rm);
    product_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(product_alt.abs_negative_zero()),
        ComparableFloat(product.abs_negative_zero_ref())
    );

    let (product_alt, o_alt) = (-&x).mul_round(-&y, rm);
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.mul_round_ref_ref(&y, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(product.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.mul_round_ref_ref(&y, Exact));
    }
}

#[test]
fn mul_round_properties() {
    float_float_rounding_mode_triple_gen_var_16().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_17().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_18().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_19().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_20().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_21().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_22().test_properties(|(x, y, rm)| {
        mul_round_properties_helper(x, y, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (product, o) = x.mul_round_ref_val(Float::NAN, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        let (product, o) = Float::NAN.mul_round_val_ref(&x, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        if !x.is_nan() {
            if x != 0 {
                assert_eq!(
                    x.mul_round_ref_val(Float::INFINITY, rm),
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
                    Float::INFINITY.mul_round_val_ref(&x, rm),
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
                    x.mul_round_ref_val(Float::NEGATIVE_INFINITY, rm),
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
                    Float::NEGATIVE_INFINITY.mul_round_val_ref(&x, rm),
                    (
                        if x > 0 {
                            Float::NEGATIVE_INFINITY
                        } else {
                            Float::INFINITY
                        },
                        Equal
                    )
                );
            }
            if x.is_finite() {
                assert_eq!(x.mul_round_ref_val(Float::ZERO, rm), (Float::ZERO, Equal));
                assert_eq!(Float::ZERO.mul_round_val_ref(&x, rm), (Float::ZERO, Equal));
                assert_eq!(
                    x.mul_round_ref_val(Float::NEGATIVE_ZERO, rm),
                    (Float::ZERO, Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_ZERO.mul_round_val_ref(&x, rm),
                    (Float::ZERO, Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (product, o) = x.mul_round_ref_val(Float::ONE, rm);
            assert_eq!(ComparableFloatRef(&product), ComparableFloatRef(&x));
            assert_eq!(o, Equal);

            let (product, o) = Float::ONE.mul_round_val_ref(&x, rm);
            assert_eq!(ComparableFloatRef(&product), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn mul_properties_helper_1(x: Float, y: Float) {
    let product = x.clone() * y.clone();
    assert!(product.is_valid());
    let product_alt = x.clone() * &y;
    assert!(product_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let product_alt = &x * y.clone();
    assert!(product_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let product_alt = &x * &y;
    assert!(product_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );

    let mut x_alt = x.clone();
    x_alt *= y.clone();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));

    let mut x_alt = x.clone();
    x_alt *= &y;
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));

    let product_alt = mul_prec_round_naive(
        x.clone(),
        y.clone(),
        max(x.significant_bits(), y.significant_bits()),
        Nearest,
    )
    .0;
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let product_alt = x
        .mul_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let product_alt = x
        .mul_prec_ref_ref(&y, max(x.significant_bits(), y.significant_bits()))
        .0;
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    let product_alt = x.mul_round_ref_ref(&y, Nearest).0;
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );

    if product.is_finite() && x.is_normal() && y.is_normal() && product.is_normal() {
        assert_eq!(
            product.get_prec(),
            Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
        );
        let r_product = Rational::exact_from(&x) * Rational::exact_from(&y);
        if product < r_product {
            let mut next = product.clone();
            next.increment();
            assert!(next > r_product);
        } else if product > r_product {
            let mut next = product.clone();
            next.decrement();
            assert!(next < r_product);
        }
    }

    let rug_product = rug_mul(rug::Float::exact_from(&x), rug::Float::exact_from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_product)),
        ComparableFloatRef(&product),
    );

    let product_alt = &y * &x;
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        assert_eq!(ComparableFloatRef(&(&y * &x)), ComparableFloatRef(&product));
        assert_eq!(
            ComparableFloatRef(&-(&x * -&y)),
            ComparableFloatRef(&product)
        );
        assert_eq!(
            ComparableFloatRef(&-(-&x * &y)),
            ComparableFloatRef(&product)
        );
        assert_eq!(
            ComparableFloatRef(&(-&x * -&y)),
            ComparableFloatRef(&product)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn mul_properties_helper_2<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let product_1 = x * y;
        let product_2 = emulate_primitive_float_fn_2(|x, y, prec| x.mul_prec(y, prec).0, x, y);
        assert_eq!(NiceFloat(product_1), NiceFloat(product_2));
    });
}

#[test]
fn mul_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_pair_gen().test_properties_with_config(&config, |(x, y)| {
        mul_properties_helper_1(x, y);
    });

    float_pair_gen_var_2().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    float_pair_gen_var_3().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    float_pair_gen_var_4().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    float_pair_gen_var_5().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    float_pair_gen_var_6().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    float_pair_gen_var_7().test_properties(|(x, y)| {
        mul_properties_helper_1(x, y);
    });

    apply_fn_to_primitive_floats!(mul_properties_helper_2);

    float_gen().test_properties(|x| {
        assert!((&x * Float::NAN).is_nan());
        assert!((Float::NAN * &x).is_nan());
        if !x.is_nan() {
            if x != 0 {
                assert_eq!(
                    &x * Float::INFINITY,
                    if x > 0 {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    }
                );
                assert_eq!(
                    Float::INFINITY * &x,
                    if x > 0 {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    }
                );
                assert_eq!(
                    &x * Float::NEGATIVE_INFINITY,
                    if x > 0 {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    }
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY * &x,
                    if x > 0 {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    }
                );
            }
            if x.is_finite() {
                assert_eq!(&x * Float::ZERO, Float::ZERO);
                assert_eq!(Float::ZERO * &x, Float::ZERO);
                assert_eq!(&x * Float::NEGATIVE_ZERO, Float::ZERO);
                assert_eq!(Float::NEGATIVE_ZERO * &x, Float::ZERO);
            }
            assert_eq!(&x * Float::ONE, x);
            assert_eq!(Float::ONE * &x, x);
        }
    });
}

#[test]
fn mul_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(x, y, prec, rm)| {
            let (product, o) = x.clone().mul_rational_prec_round(y.clone(), prec, rm);
            assert!(product.is_valid());
            let (product_alt, o_alt) = x.clone().mul_rational_prec_round_val_ref(&y, prec, rm);
            assert!(product_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);
            let (product_alt, o_alt) = x.mul_rational_prec_round_ref_val(y.clone(), prec, rm);
            assert!(product_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);
            let (product_alt, o_alt) = x.mul_rational_prec_round_ref_ref(&y, prec, rm);
            assert!(product_alt.is_valid());
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.mul_rational_prec_round_assign(y.clone(), prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.mul_rational_prec_round_assign_ref(&y, prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
            assert_eq!(o_alt, o);

            let (product_alt, o_alt) =
                mul_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);

            let r_product = if product.is_finite() {
                if product.is_normal() {
                    assert_eq!(product.get_prec(), Some(prec));
                }
                let r_product = Rational::exact_from(&x) * &y;
                assert_eq!(product.partial_cmp(&r_product), Some(o));
                if o == Less {
                    let mut next = product.clone();
                    next.increment();
                    assert!(next > r_product);
                } else if o == Greater {
                    let mut next = product.clone();
                    next.decrement();
                    assert!(next < r_product);
                }
                Some(r_product)
            } else {
                assert_eq!(o, Equal);
                None
            };

            match (
                r_product.is_some() && *r_product.as_ref().unwrap() >= 0u32,
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

            let (mut product_alt, mut o_alt) = x.mul_rational_prec_round_ref_val(-&y, prec, -rm);
            product_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(product_alt.abs_negative_zero()),
                ComparableFloat(product.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);

            let (mut product_alt, mut o_alt) = (-&x).mul_rational_prec_round_val_ref(&y, prec, -rm);
            product_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(product_alt.abs_negative_zero()),
                ComparableFloat(product.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);

            if product != 0u32 {
                let (product_alt, o_alt) = (-&x).mul_rational_prec_round(-&y, prec, rm);
                assert_eq!(
                    ComparableFloatRef(&product_alt),
                    ComparableFloatRef(&product)
                );
                assert_eq!(o_alt, o);
            }

            if o == Equal {
                for rm in exhaustive_rounding_modes() {
                    let (s, oo) = x.mul_rational_prec_round_ref_ref(&y, prec, rm);
                    assert_eq!(
                        ComparableFloat(s.abs_negative_zero_ref()),
                        ComparableFloat(product.abs_negative_zero_ref())
                    );
                    assert_eq!(oo, Equal);
                }
            } else {
                assert_panic!(x.mul_rational_prec_round_ref_ref(&y, prec, Exact));
            }
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_negative_zero() {
            let (product, o) = x.mul_rational_prec_round_ref_val(Rational::ONE, prec, rm);
            let mut product_alt = x.clone();
            let o_alt = product_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (product, o) = Float::NAN.mul_rational_prec_round_val_ref(&x, prec, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::INFINITY.mul_rational_prec_round_val_ref(&x, prec, rm),
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
                Float::NEGATIVE_INFINITY.mul_rational_prec_round_val_ref(&x, prec, rm),
                (
                    if x > 0 {
                        Float::NEGATIVE_INFINITY
                    } else {
                        Float::INFINITY
                    },
                    Equal
                )
            );
        }

        let (product, o) = Float::ZERO.mul_rational_prec_round_val_ref(&x, prec, rm);
        assert_eq!(
            ComparableFloat(product),
            ComparableFloat(if x >= 0 {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            })
        );
        assert_eq!(o, Equal);

        let (product, o) = Float::NEGATIVE_ZERO.mul_rational_prec_round_val_ref(&x, prec, rm);
        assert_eq!(
            ComparableFloat(product),
            ComparableFloat(if x >= 0 {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            })
        );
        assert_eq!(o, Equal);
    });
}

#[test]
fn mul_rational_prec_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (product, o) = x.clone().mul_rational_prec(y.clone(), prec);
        assert!(product.is_valid());
        let (product_alt, o_alt) = x.clone().mul_rational_prec_val_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
        let (product_alt, o_alt) = x.mul_rational_prec_ref_val(y.clone(), prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
        let (product_alt, o_alt) = x.mul_rational_prec_ref_ref(&y, prec);
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.mul_rational_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.mul_rational_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) =
            mul_rational_prec_round_naive(x.clone(), y.clone(), prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = x.mul_rational_prec_round_ref_ref(&y, prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        if product.is_finite() {
            if product.is_normal() {
                assert_eq!(product.get_prec(), Some(prec));
            }
            let r_product = Rational::exact_from(&x) * &y;
            assert_eq!(product.partial_cmp(&r_product), Some(o));
            if o == Less {
                let mut next = product.clone();
                next.increment();
                assert!(next > r_product);
            } else if o == Greater {
                let mut next = product.clone();
                next.decrement();
                assert!(next < r_product);
            }
        } else {
            assert_eq!(o, Equal);
        }

        if x != 0u32 && y != 0u32 {
            let (mut product_alt, mut o_alt) = x.mul_rational_prec_ref_val(-&y, prec);
            product_alt.neg_assign();
            product_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);

            let (mut product_alt, mut o_alt) = (-&x).mul_rational_prec_val_ref(&y, prec);
            product_alt.neg_assign();
            product_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);

            let (product_alt, o_alt) = (-x).mul_rational_prec(-y, prec);
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product)
            );
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if !x.is_negative_zero() {
            let (product, o) = x.mul_rational_prec_ref_val(Rational::ONE, prec);
            let mut product_alt = x.clone();
            let o_alt = product_alt.set_prec(prec);
            assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
            assert_eq!(o, o_alt);
        }

        if x.is_finite() && !x.is_negative_zero() {
            let (product, o) = x.mul_rational_prec_ref_val(Rational::ZERO, prec);
            assert_eq!(
                ComparableFloat(product),
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
        let (product, o) = Float::NAN.mul_rational_prec_val_ref(&x, prec);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::INFINITY.mul_rational_prec_val_ref(&x, prec),
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
                Float::NEGATIVE_INFINITY.mul_rational_prec_val_ref(&x, prec),
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

        let (product, o) = Float::ONE.mul_rational_prec_val_ref(&x, prec);
        let (product_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
        assert_eq!(o, o_alt);

        let (product, o) = Float::ZERO.mul_rational_prec_val_ref(&x, prec);
        assert_eq!(
            ComparableFloat(product),
            ComparableFloat(if x >= 0 {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            })
        );
        assert_eq!(o, Equal);

        let (product, o) = Float::NEGATIVE_ZERO.mul_rational_prec_val_ref(&x, prec);
        assert_eq!(
            ComparableFloat(product),
            ComparableFloat(if x >= 0 {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            })
        );
        assert_eq!(o, Equal);
    });
}

#[test]
fn mul_rational_round_properties() {
    float_rational_rounding_mode_triple_gen_var_4().test_properties(|(x, y, rm)| {
        let (product, o) = x.clone().mul_rational_round(y.clone(), rm);
        assert!(product.is_valid());
        let (product_alt, o_alt) = x.clone().mul_rational_round_val_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let (product_alt, o_alt) = x.mul_rational_round_ref_val(y.clone(), rm);
        assert!(product_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let (product_alt, o_alt) = x.mul_rational_round_ref_ref(&y, rm);
        assert!(product_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );

        let mut x_alt = x.clone();
        let o_alt = x_alt.mul_rational_round_assign(y.clone(), rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.mul_rational_round_assign_ref(&y, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) =
            mul_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
        let (product_alt, o_alt) = x.mul_rational_prec_round_ref_ref(&y, x.significant_bits(), rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let r_product = if product.is_finite() {
            if x.is_normal() && product.is_normal() {
                assert_eq!(product.get_prec(), Some(x.get_prec().unwrap()));
            }
            let r_product = Rational::exact_from(&x) * &y;
            assert_eq!(product.partial_cmp(&r_product), Some(o));
            if o == Less {
                let mut next = product.clone();
                next.increment();
                assert!(next > r_product);
            } else if o == Greater {
                let mut next = product.clone();
                next.decrement();
                assert!(next < r_product);
            }
            Some(r_product)
        } else {
            assert_eq!(o, Equal);
            None
        };

        match (
            r_product.is_some() && *r_product.as_ref().unwrap() >= 0u32,
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
            let (rug_product, rug_o) = rug_mul_rational_round(
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_product)),
                ComparableFloatRef(&product),
            );
            assert_eq!(rug_o, o);
        }

        let (mut product_alt, mut o_alt) = x.mul_rational_round_ref_val(-&y, -rm);
        product_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(product_alt.abs_negative_zero_ref()),
            ComparableFloat(product.abs_negative_zero_ref())
        );

        let (mut product_alt, mut o_alt) = (-&x).mul_rational_round_val_ref(&y, -rm);
        product_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(product_alt.abs_negative_zero_ref()),
            ComparableFloat(product.abs_negative_zero_ref())
        );

        if x != 0 && y != 0 {
            let (product_alt, o_alt) = (-&x).mul_rational_round(-&y, rm);
            assert_eq!(
                ComparableFloatRef(&product_alt),
                ComparableFloatRef(&product),
            );
            assert_eq!(o_alt, o);
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = x.mul_rational_round_ref_ref(&y, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(product.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(x.mul_rational_round_ref_ref(&y, Exact));
        }
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (product, o) = x.mul_rational_round_ref_val(Rational::ONE, rm);
        assert_eq!(ComparableFloatRef(&product), ComparableFloatRef(&x));
        assert_eq!(o, Equal);

        if x.is_finite() && !x.is_negative_zero() {
            let (product, o) = x.mul_rational_round_ref_val(Rational::ZERO, rm);
            assert_eq!(
                ComparableFloat(product),
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
        let (product, o) = Float::NAN.mul_rational_round_val_ref(&x, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        if x != 0 {
            assert_eq!(
                Float::INFINITY.mul_rational_round_val_ref(&x, rm),
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
                Float::NEGATIVE_INFINITY.mul_rational_round_val_ref(&x, rm),
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
        let (product, o) = Float::ONE.mul_rational_round_val_ref(&x, rm);
        let (product_alt, o_alt) = Float::from_rational_prec_round_ref(&x, 1, rm);
        assert_eq!(ComparableFloat(product), ComparableFloat(product_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn mul_rational_properties() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        let product = x.clone() * y.clone();
        assert!(product.is_valid());
        let product_alt = x.clone() * &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = &x * y.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = &x * &y;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );

        let product_alt = y.clone() * x.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = y.clone() * &x;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = &y * x.clone();
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = &y * &x;
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );

        let mut x_alt = x.clone();
        x_alt *= y.clone();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));

        let mut x_alt = x.clone();
        x_alt *= &y;
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&product));

        let product_alt =
            mul_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), Nearest).0;
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = x
            .mul_rational_prec_round_ref_ref(&y, x.significant_bits(), Nearest)
            .0;
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = x.mul_rational_prec_ref_ref(&y, x.significant_bits()).0;
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        let product_alt = x.mul_rational_round_ref_ref(&y, Nearest).0;
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );

        if product.is_finite() && x.is_normal() && product.is_normal() {
            assert_eq!(product.get_prec(), Some(x.get_prec().unwrap()));
            let r_product = Rational::exact_from(&x) * &y;
            if product < r_product {
                let mut next = product.clone();
                next.increment();
                assert!(next > r_product);
            } else if product > r_product {
                let mut next = product.clone();
                next.decrement();
                assert!(next < r_product);
            }
        }

        let rug_product = rug_mul_rational(rug::Float::exact_from(&x), rug::Rational::from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_product)),
            ComparableFloatRef(&product),
        );

        if product != 0u32 {
            assert_eq!(
                ComparableFloatRef(&-(-&x * &y)),
                ComparableFloatRef(&product)
            );
            assert_eq!(
                ComparableFloatRef(&-(&x * -&y)),
                ComparableFloatRef(&product)
            );
            assert_eq!(ComparableFloatRef(&(-x * -y)), ComparableFloatRef(&product));
        }
    });

    float_gen().test_properties(|x| {
        assert_eq!(
            ComparableFloatRef(&(&x * Rational::ONE)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&(Rational::ONE * &x)),
            ComparableFloatRef(&x)
        );
        if x.is_finite() {
            assert_eq!(
                ComparableFloat(&x * Rational::ZERO),
                ComparableFloat(if x.is_sign_positive() {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                }),
            );
            assert_eq!(
                ComparableFloat(Rational::ZERO * &x),
                ComparableFloat(if x.is_sign_positive() {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                })
            );
        }
    });

    rational_gen().test_properties(|x| {
        assert!((&x * Float::NAN).is_nan());
        assert!((Float::NAN * &x).is_nan());
        if x != 0 {
            assert_eq!(
                &x * Float::INFINITY,
                if x > 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                }
            );
            assert_eq!(
                Float::INFINITY * &x,
                if x > 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                }
            );
            assert_eq!(
                &x * Float::NEGATIVE_INFINITY,
                if x > 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                }
            );
            assert_eq!(
                Float::NEGATIVE_INFINITY * &x,
                if x > 0 {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                }
            );
        }
        let product_alt = Float::from_rational_prec_ref(&x, 1).0;
        assert_eq!(
            ComparableFloat(&x * Float::ONE),
            ComparableFloat(product_alt.clone())
        );
        assert_eq!(
            ComparableFloat(Float::ONE * &x),
            ComparableFloat(product_alt.clone())
        );
        assert_eq!(
            ComparableFloat((&x * Float::ZERO).abs_negative_zero()),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(
            ComparableFloat((Float::ZERO * &x).abs_negative_zero()),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(
            ComparableFloat((&x * Float::NEGATIVE_ZERO).abs_negative_zero()),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(
            ComparableFloat((Float::NEGATIVE_ZERO * &x).abs_negative_zero()),
            ComparableFloat(Float::ZERO)
        );
    });
}
