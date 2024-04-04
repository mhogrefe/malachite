use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::test_util::arithmetic::add::{
    add_prec_round_naive, add_rational_prec_round_naive,
};
use malachite_float::test_util::arithmetic::sub::{
    rug_sub, rug_sub_rational, rug_sub_rational_round, rug_sub_round,
};
use malachite_float::test_util::common::{
    emulate_primitive_float_fn_2, parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_10, float_float_rounding_mode_triple_gen_var_11,
    float_float_rounding_mode_triple_gen_var_12, float_float_rounding_mode_triple_gen_var_13,
    float_float_rounding_mode_triple_gen_var_14, float_float_rounding_mode_triple_gen_var_15,
    float_float_rounding_mode_triple_gen_var_2,
    float_float_unsigned_rounding_mode_quadruple_gen_var_2, float_float_unsigned_triple_gen_var_1,
    float_gen, float_pair_gen, float_pair_gen_var_2, float_pair_gen_var_3, float_pair_gen_var_4,
    float_pair_gen_var_5, float_pair_gen_var_6, float_pair_gen_var_7, float_rational_pair_gen,
    float_rational_rounding_mode_triple_gen_var_2,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_2,
    float_rational_unsigned_triple_gen_var_1, float_rounding_mode_pair_gen,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_1,
    rational_rounding_mode_pair_gen_var_6, rational_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use malachite_q::Rational;
use std::cmp::{max, Ordering};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let diff = x.clone() - y.clone();
        assert!(diff.is_valid());

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);

        let diff_alt = x.clone() - &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));

        let diff_alt = &x - y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));

        let diff_alt = &x - &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));

        let mut diff_alt = x.clone();
        diff_alt -= y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        let mut diff_alt = x.clone();
        diff_alt -= &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sub(
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&diff)
        );

        let diff_alt = add_prec_round_naive(
            x.clone(),
            -&y,
            max(x.significant_bits(), y.significant_bits()),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    };
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "NaN", "NaN");
    test("NaN", "NaN", "-Infinity", "-Infinity", "NaN", "NaN");
    test("NaN", "NaN", "0.0", "0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "-0.0", "-0x0.0", "NaN", "NaN");

    test("Infinity", "Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test("Infinity", "Infinity", "Infinity", "Infinity", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", "Infinity", "Infinity",
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", "Infinity", "Infinity",
    );

    test("-Infinity", "-Infinity", "NaN", "NaN", "NaN", "NaN");
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
        "Infinity",
        "Infinity",
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
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
    );

    test("0.0", "0x0.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");

    test("-0.0", "-0x0.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", "NaN", "NaN", "NaN", "NaN");
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("123.0", "0x7b.0#7", "-0.0", "-0x0.0", "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", "0.0", "0x0.0", "123.0", "0x7b.0#7");

    test("NaN", "NaN", "-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        "Infinity",
        "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "-123.0", "-0x7b.0#7", "123.0", "0x7b.0#7");
    test("-0.0", "-0x0.0", "-123.0", "-0x7b.0#7", "123.0", "0x7b.0#7");

    test("1.0", "0x1.0#1", "-2.0", "-0x2.0#1", "4.0", "0x4.0#1");
    test("1.0", "0x1.0#1", "-2.0", "-0x2.0#2", "3.0", "0x3.0#2");
    test("1.0", "0x1.0#2", "-2.0", "-0x2.0#1", "3.0", "0x3.0#2");
    test("1.0", "0x1.0#2", "-2.0", "-0x2.0#2", "3.0", "0x3.0#2");
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        "3.0",
        "0x3.00#10",
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1.727379091216698",
        "0x1.ba35842091e63#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-4.555806215962888",
        "-0x4.8e4950f0795fc#53",
    );

    test("1.0", "0x1.0#1", "-0.0002", "-0x0.001#1", "1.0", "0x1.0#1");
    // - in sub_float_significands_same_prec_lt_w
    // - x_exp == y_exp in sub_float_significands_same_prec_lt_w
    // - x == y in sub_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", "0.0", "0x0.0");
    // - x_exp < y_exp in sub_float_significands_same_prec_lt_w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_lt_w
    // - leading_zeros != 0 in sub_float_significands_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "-1.0", "-0x1.0#1");
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_lt_w
    // - neg in sub_float_significands_same_prec_lt_w
    // - rm == Nearest in sub_float_significands_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & shift_bit) != 0) in
    //   sub_float_significands_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && sticky_bit != 0 && diff == 0 in
    //   sub_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "4.0", "0x4.0#1", "-4.0", "-0x4.0#1");
    // - !neg in sub_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "0.2", "0x0.4#1", "1.0", "0x1.0#1");
    // - x < y in sub_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#2", "1.5", "0x1.8#2", "-0.5", "-0x0.8#2");
    // - x > y in sub_float_significands_same_prec_lt_w
    test("1.5", "0x1.8#2", "1.0", "0x1.0#2", "0.5", "0x0.8#2");
    // - leading_zeros == 0 in sub_float_significands_same_prec_lt_w
    test("1.5", "0x1.8#2", "0.5", "0x0.8#2", "1.0", "0x1.0#2");
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff & shift_bit) == 0)) in
    //   sub_float_significands_same_prec_lt_w
    test("2.0", "0x2.0#2", "0.8", "0x0.c#2", "1.0", "0x1.0#2");
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & shift_bit) != 0) && diff !=
    //   0 in sub_float_significands_same_prec_lt_w
    test("1.5", "0x1.8#2", "0.1", "0x0.2#2", "1.5", "0x1.8#2");
    // - exp_diff >= Limb::WIDTH in sub_float_significands_same_prec_lt_w
    // - x <= HIGH_BIT in sub_float_significands_same_prec_lt_w
    test(
        "1.0e9",
        "0x4.0E+7#1",
        "6.0e-11",
        "0x4.0E-9#1",
        "1.0e9",
        "0x4.0E+7#1",
    );
    // - x > HIGH_BIT in sub_float_significands_same_prec_lt_w
    test(
        "9.2047171e-27",
        "0x2.d945d78E-22#27",
        "1.43189635e33",
        "0x4.69912aE+27#27",
        "-1.43189635e33",
        "-0x4.69912aE+27#27",
    );

    // - in sub_float_significands_same_prec_w
    // - x_exp == y_exp in sub_float_significands_same_prec_w
    // - x_exp == y_exp && a0 == 0 in sub_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        "0.0",
        "0x0.0",
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_w
    // - x_exp < y_exp in sub_float_significands_same_prec_w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_w
    // - a0 != 0 in sub_float_significands_same_prec_w
    // - leading_zeros != 0 in sub_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "2.0",
        "0x2.0000000000000000#64",
        "-1.0",
        "-0x1.0000000000000000#64",
    );
    // - a0 > x in sub_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "-1.084202172485504434e-19",
        "-0x2.0000000000000000E-16#64",
    );
    // - a0 <= x in sub_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0",
        "0x1.0000000000000000#64",
        "1.084202172485504434e-19",
        "0x2.0000000000000000E-16#64",
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_w
    // - neg in sub_float_significands_same_prec_w
    // - rm == Nearest in sub_float_significands_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & 1) != 0) in
    //   sub_float_significands_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & 1) != 0) &&
    //   !diff.overflowing_add_assign(1) in sub_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "4.0",
        "0x4.0000000000000000#64",
        "-3.0",
        "-0x3.0000000000000000#64",
    );
    // - !neg in sub_float_significands_same_prec_w
    test(
        "4.0",
        "0x4.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "3.0",
        "0x3.0000000000000000#64",
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff & 1) == 0)) in
    //   sub_float_significands_same_prec_w
    test(
        "1.0000000000000000003",
        "0x1.0000000000000006#64",
        "4.0",
        "0x4.0000000000000000#64",
        "-2.9999999999999999996",
        "-0x2.fffffffffffffff8#64",
    );
    // - leading_zeros == 0 in sub_float_significands_same_prec_w
    test(
        "3.2729513077064011786e-37",
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        "7.8519772600462495573e-34",
        "0x4.13b4f0d218450fb0E-28#64",
        "-7.848704308738543156e-34",
        "-0x4.13459164c75d56b8E-28#64",
    );
    // - exp_diff >= Limb::WIDTH in sub_float_significands_same_prec_w
    // - x > HIGH_BIT in sub_float_significands_same_prec_w
    test(
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
        "2.9347251290514630352e-45",
        "0x1.0c11b075f03d6daeE-37#64",
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
    );
    // - x <= HIGH_BIT in sub_float_significands_same_prec_w
    // - exp_diff != Limb::WIDTH || y <= HIGH_BIT in sub_float_significands_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & 1) != 0) &&
    //   diff.overflowing_add_assign(1) in sub_float_significands_same_prec_w
    test(
        "8355840.0624923708378",
        "0x7f8000.0fff8000ff8#64",
        "2.3384026197294446691e49",
        "0x1.0000000000000000E+41#64",
        "-2.3384026197294446691e49",
        "-0x1.0000000000000000E+41#64",
    );
    // - x_exp != y_exp && a0 == 0 in sub_float_significands_same_prec_w
    test(
        "63.999999999999999997",
        "0x3f.ffffffffffffffc#64",
        "64.0",
        "0x40.000000000000000#64",
        "-3.4694469519536141888e-18",
        "-0x4.0000000000000000E-15#64",
    );
    // - exp_diff == Limb::WIDTH && y > HIGH_BIT in sub_float_significands_same_prec_w
    test(
        "4.656578456163629198e-10",
        "0x1.ffff07fffffffffeE-8#64",
        "4294967296.0",
        "0x100000000.00000000#64",
        "-4294967295.9999999995",
        "-0xffffffff.fffffffe#64",
    );

    // - in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp in sub_float_significands_same_prec_gt_w_lt_2w
    // - a1 == 0 && a0 == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        "0.0",
        "0x0.0",
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp < y_exp in sub_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp != y_exp && a1 != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp != y_exp && leading_zeros != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "2.0",
        "0x2.0000000000000000#65",
        "-1.0",
        "-0x1.0000000000000000#65",
    );
    // - (a1 != 0 || a0 != 0) && a1 >= x_1 in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp && a1 == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp && leading_zeros == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "-5.42101086242752217e-20",
        "-0x1.0000000000000000E-16#65",
    );
    // - (a1 != 0 || a0 != 0) && a1 < x_1 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.0",
        "0x1.0000000000000000#65",
        "5.42101086242752217e-20",
        "0x1.0000000000000000E-16#65",
    );
    // - x_exp == y_exp && leading_zeros != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.00000000000000000#66",
        "1.00000000000000000003",
        "0x1.00000000000000008#66",
        "-2.710505431213761085e-20",
        "-0x8.0000000000000000E-17#66",
    );
    // - x_exp == y_exp && a1 != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        "-1.084202172485504434e-19",
        "-0x2.0000000000000000E-16#65",
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - neg in sub_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in sub_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   !overflow in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "4.0",
        "0x4.0000000000000000#65",
        "-3.0",
        "-0x3.0000000000000000#65",
    );
    // - !neg in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.0",
        "0x4.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "3.0",
        "0x3.0000000000000000#65",
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff_0 & shift_bit) == 0)) in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        "4.0",
        "0x4.0000000000000000#65",
        "-2.9999999999999999998",
        "-0x2.fffffffffffffffc#65",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && a1 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "18.63123034252626794758647",
        "0x12.a1984fcd64a8ae228eef#85",
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
    );
    // - x_exp != y_exp && leading_zeros == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.0001507756106295330606262754053",
        "0x0.0009e19851127b95dcf03f0cdc#91",
        "3458.565842843038054059107814",
        "0xd82.90db1399862ba513faf8#91",
        "-3458.565692067427424526047188",
        "-0xd82.90d132013519297e1e08#91",
    );
    // - exp_diff >= Limb::WIDTH * 2 in sub_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff >= Limb::WIDTH * 2 && a1 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
        "1.14823551075108882469e-96",
        "0x2.73dea72af3fe6314E-80#65",
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
    );
    // - exp_diff == Limb::WIDTH in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "19585.2851423168986928116147584507795",
        "0x4c81.48ff163dc91a0d4bd90309b0f8#116",
        "372369974082165972902790.766638151683",
        "0x4eda377c7f0d747fa386.c44265dd58#116",
        "-372369974082165972883205.481495834785",
        "-0x4eda377c7f0d747f5705.7b434f9f90#116",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && a1 < HIGH_BIT in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   overflow in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "9.9035203142830421991929938e27",
        "0x2.000000000000000000000E+23#85",
        "16.0",
        "0x10.00000000000000000000#85",
        "9.9035203142830421991929938e27",
        "0x2.000000000000000000000E+23#85",
    );
    // - exp_diff >= Limb::WIDTH * 2 && a1 < HIGH_BIT in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "5.3455294200288159103345444e-51",
        "0x1.ffffffffc000000000000E-42#83",
        "8.0",
        "0x8.00000000000000000000#83",
        "-8.0",
        "-0x8.00000000000000000000#83",
    );
    // - x_exp != y_exp && a1 == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.00000000000000000000084702",
        "0x4.000000000000000003fffc#89",
        "3.999999999999999999999999994",
        "0x3.fffffffffffffffffffffe#89",
        "8.47026484905764768539612568e-22",
        "0x3.fffe000000000000000000E-18#89",
    );

    // - in sub_float_significands_same_prec_2w
    // - x_exp == y_exp in sub_float_significands_same_prec_2w
    // - a1 == 0 && a0 == 0 in sub_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "0.0",
        "0x0.0",
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_2w
    // - x_exp < y_exp in sub_float_significands_same_prec_2w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_2w
    // - a1 != 0 second time in sub_float_significands_same_prec_2w
    // - a1 != 0 third time in sub_float_significands_same_prec_2w
    // - x_exp != y_exp && leading_zeros != 0 in sub_float_significands_same_prec_2w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        "-1.0",
        "-0x1.00000000000000000000000000000000#128",
    );
    // - (a1 != 0 || a0 != 0) && a1 >= x_1 in sub_float_significands_same_prec_2w
    // - a1 == 0 first time in sub_float_significands_same_prec_2w
    // - x_exp == y_exp && leading_zeros != 0 in sub_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "-5.87747175411143753984368268611122838909e-39",
        "-0x2.00000000000000000000000000000000E-32#128",
    );
    // - (a1 != 0 || a0 != 0) && a1 < x_1 in sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "5.87747175411143753984368268611122838909e-39",
        "0x2.00000000000000000000000000000000E-32#128",
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_2w
    // - neg in sub_float_significands_same_prec_2w
    // - rm == Nearest in sub_float_significands_same_prec_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & 1) != 0) && !overflow in
    //   sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        "-3.0",
        "-0x3.00000000000000000000000000000000#128",
    );
    // - !neg in sub_float_significands_same_prec_2w
    test(
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "3.0",
        "0x3.00000000000000000000000000000000#128",
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff_0 & 1) == 0)) in
    //   sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000018",
        "0x1.00000000000000000000000000000006#128",
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        "-2.99999999999999999999999999999999999998",
        "-0x2.fffffffffffffffffffffffffffffff8#128",
    );
    // - x_exp != y_exp && leading_zeros == 0 in sub_float_significands_same_prec_2w
    test(
        "1.91698663575347889601435178329077738407e-37",
        "0x4.13b4f0d218450fb6f5f6d50e7b8f6eb0E-31#128",
        "8.0669110092962644724944820639408319804e-34",
        "0x4.3046c1d7c0f5b6be39df2b03f3e49a70E-28#128",
        "-8.06499402266051099359846771215754120301e-34",
        "-0x4.30058688b3d4326d3e6fcb96a2fce178E-28#128",
    );
    // - exp_diff >= Limb::WIDTH * 2 in sub_float_significands_same_prec_2w
    // - x_1 > HIGH_BIT || x_0 > 0 in sub_float_significands_same_prec_2w
    test(
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        "5.07801249136957145270807726205511855421e-45",
        "0x1.cfd8608b7c32de2bbcfecf8bcf8a2d00E-37#128",
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_2w
    // - a1 >= HIGH_BIT in sub_float_significands_same_prec_2w
    test(
        "4354249796990942.35435357526597783143164",
        "0xf782ac869b7de.5ab6ea78fcf0cc5079f#128",
        "8.03239453825726512240307053405256016022e-10",
        "0x3.732bce7aa121827a284545a25f32dc68E-8#128",
        "4354249796990942.35435357446273837760591",
        "0xf782ac869b7de.5ab6ea7589c4fdd5d8d#128",
    );
    // - a1 != 0 first time in sub_float_significands_same_prec_2w
    test(
        "852777855057.113455599443829872557360137",
        "0xc68d856851.1d0b6d1928c98779f28bdd#128",
        "869223500084.503694559376384046083491558",
        "0xca61c20934.80f2206bb1d7d5cad69caa#128",
        "-16445645027.39023895993255417352613142066",
        "-0x3d43ca0e3.63e6b352890e4e50e410cd00#128",
    );
    // - exp_diff == Limb::WIDTH in sub_float_significands_same_prec_2w
    test(
        "1.057437459917463716438672572710788562701e-17",
        "0xc.310127aae1df1a1cb12f60c4d339d76E-15#128",
        "148.0549133677002965445211858794413066474",
        "0x94.0e0ecd6e62d0a8c7c7c2a633277e3e#128",
        "-148.054913367700296533946811280266669483",
        "-0x94.0e0ecd6e62d0a804b7b02b85098c9c#128",
    );
    // - x_1 <= HIGH_BIT && x_0 <= 0 in sub_float_significands_same_prec_2w
    // - exp_diff != TWICE_WIDTH || tst in sub_float_significands_same_prec_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & 1) != 0) && overflow in
    //   sub_float_significands_same_prec_2w
    test(
        "3.83123885216472214589586756787577295905e53",
        "0x4.00000000000000000000000000000000E+44#128",
        "9.09494703466994132423639742106012939545e-13",
        "0x1.00000008000000000000007fffffff80E-10#128",
        "3.83123885216472214589586756787577295905e53",
        "0x4.00000000000000000000000000000000E+44#128",
    );
    // - a1 < HIGH_BIT in sub_float_significands_same_prec_2w
    test(
        "0.000473141670219947088192358501321236008969",
        "0x0.001f01fffffe001ffe000000000003ffff0#128",
        "144115188075855872.000000000007275846592",
        "0x200000000000000.0000000007fff80000#128",
        "-144115188075855871.9995268583370558995037",
        "-0x1ffffffffffffff.ffe0fe000801f7e002#128",
    );
    // - a1 == 0 second  time in sub_float_significands_same_prec_2w
    test(
        "1.192092895507812500000000000000000918348e-7",
        "0x2.0000000000000000000000000007fffcE-6#128",
        "1.19209289550781249999999999999999632658e-7",
        "0x1.ffffffffffffffffffffffffffe00000E-6#128",
        "4.5917678014072389539175224798764807294e-40",
        "0x2.7fffc000000000000000000000000000E-33#128",
    );
    // - exp_diff == TWICE_WIDTH && !tst in sub_float_significands_same_prec_2w
    test(
        "1024.0",
        "0x400.000000000000000000000000000000#128",
        "5.97151130219911582898625687582100437209e-36",
        "0x7.f00000001fffffffffffffffe0000000E-30#128",
        "1023.999999999999999999999999999999999994",
        "0x3ff.fffffffffffffffffffffffffffff8#128",
    );
    // - x_exp == y_exp && leading_zeros == 0 in sub_float_significands_same_prec_2w
    test(
        "0.249999999999999999999999999974756451033",
        "0x0.3ffffffffffffffffffffffe000000000#128",
        "0.2499999999999999999864474728691747435412",
        "0x0.3fffffffffffffffc0000001ffffffffc#128",
        "1.35525271055817074916830884337875729072e-20",
        "0x3.ffffffc0000000040000000000000000E-17#128",
    );
    // - a1 == 0 third time in sub_float_significands_same_prec_2w
    test(
        "8.0",
        "0x8.0000000000000000000000000000000#128",
        "7.99999999999999999999999999999999999998",
        "0x7.fffffffffffffffffffffffffffffff8#128",
        "2.35098870164457501593747307444449135564e-38",
        "0x8.0000000000000000000000000000000E-32#128",
    );

    // - in sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp in sub_float_significands_same_prec_gt_2w_lt_3w
    // - a2 == 0 && a1 == 0 && a0 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "0.0",
        "0x0.0",
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && sticky_bit == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && a2 != 0 first time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && leading_zeros != 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        "-1.0",
        "-0x1.00000000000000000000000000000000#129",
    );
    // - x_exp >= y_exp in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
    );
    // - (a2 != 0 || a1 != 0 || a0 != 0) && a2 >= x_2 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp && a2 == 0 first time in sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp && a2 == 0 second time in sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp && leading_zeros == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "-2.93873587705571876992184134305561419455e-39",
        "-0x1.00000000000000000000000000000000E-32#129",
    );
    // - (a2 != 0 || a1 != 0 || a0 != 0) && a2 < x_2 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "2.93873587705571876992184134305561419455e-39",
        "0x1.00000000000000000000000000000000E-32#129",
    );
    // - x_exp == y_exp && leading_zeros != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000#130",
        "1.000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000008#130",
        "-1.469367938527859384960920671527807097273e-39",
        "-0x8.00000000000000000000000000000000E-33#130",
    );
    // - x_exp == y_exp && a2 != 0 second time in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
        "-5.87747175411143753984368268611122838909e-39",
        "-0x2.00000000000000000000000000000000E-32#129",
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    // - neg in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   overflow in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) && diff_1
    //   == 0 && diff_0 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && diff_2 != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        "-3.0",
        "-0x3.00000000000000000000000000000000#129",
    );
    // - !neg in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "3.0",
        "0x3.00000000000000000000000000000000#129",
    );
    // - rm == Nearest && round_bit == 0 || (sticky_bit == 0 && (diff_0 & shift_bit) == 0) in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        "-2.999999999999999999999999999999999999988",
        "-0x2.fffffffffffffffffffffffffffffffc#129",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   (diff_1 != 0 || diff_0 != 0) in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.000000000000000000000000000000000000006",
        "0x2.00000000000000000000000000000002#129",
        "0.500000000000000000000000000000000000001",
        "0x0.800000000000000000000000000000008#129",
        "1.500000000000000000000000000000000000006",
        "0x1.80000000000000000000000000000002#129",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   !overflow in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.000000000000000000000000000000000000003",
        "0x2.00000000000000000000000000000001#130",
        "0.5000000000000000000000000000000000000007",
        "0x0.800000000000000000000000000000004#130",
        "1.500000000000000000000000000000000000003",
        "0x1.800000000000000000000000000000010#130",
    );
    // - TWICE_WIDTH <= exp_diff < THRICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - TWICE_WIDTH < exp_diff < THRICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - TWICE_WIDTH <= exp_diff < THRICE_WIDTH && a2 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        "-245906107849378561117126906.9059035528266331265",
        "-0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
    );
    // - exp_diff >= THRICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff >= THRICE_WIDTH && a2 >= HIGH_BIT in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        "6.8892360159362421595728818935378487832685754059e-50",
        "0x1.9c693c182df3035eef00d41638bbdd942f4d498E-41#155",
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
    );
    // - exp_diff < Limb::WIDTH && leading_zeros == 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.9709672065181108960570410290811793724062284431352e-48",
        "0x7.43dc113e95ca123693650af31435eac45c0e7a680E-40#165",
        "5.2183974782595301717751266872943662193587933931613e-47",
        "0x4.c4457ca8b3429511981a96eb0c2de4fdb8c43bea4E-39#165",
        "-4.7213007576077190821694225843862482821181705488478e-47",
        "-0x4.5007bb94c9e5f3ee2ee4463bdaea865173035443cE-39#165",
    );
    // - exp_diff < Limb::WIDTH && sticky_bit != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "8.264811372870109665580646414654919646529224699888394e-20",
        "0x1.864b7049feb3dcfe49ea910db778157cbe9c2021b44E-16#171",
        "0.0007337187065343299500100945131574173571435306249470578",
        "0x0.003015c1d959ec54ab97dc58b77c22566586c06119b810#171",
        "-0.0007337187065343298673619807844563207013370664783978612",
        "-0x0.003015c1d959ec53254c6c0eb8c845581b9c2f53623ff8#171",
    );
    // - Limb::WIDTH <= exp_diff < TWICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && sticky_bit != 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && a2 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.2850537238606374652351877988811796373898773e-22",
        "0x2.0607fd4819748c532aad3528693c1e3c1966E-18#146",
        "978.49328809934495391839880801989439981236569",
        "0x3d2.7e4820fe314caadcb9a156bef2f1c8e53c#146",
        "-978.49328809934495391839837951452201374861917",
        "-0x3d2.7e4820fe314caadcb79b4ec1aad85458e9#146",
    );
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && sticky_bit == 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "0.4575080253178526499352273198671695352442",
        "0x0.751f3ef665d0ca4dfa2d089979c4e9600#130",
        "184366716174337778394.1535133791267987587",
        "0x9fe9a278b38ab22da.274ca71ed919c918#130",
        "-184366716174337778393.6960053538089461089",
        "-0x9fe9a278b38ab22d9.b22d68287348fecc#130",
    );
    // - x_exp == y_exp && a2 != 0 first time in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "229.1456159244630209666077998586332154002628588961962846344",
        "0xe5.254715d158717849f7198986a38cb415eeea3464b1df38#189",
        "175.1335582002789888688278442018847623084238889142385801004",
        "0xaf.2230dec64f958583522e37252cf610378914f3127d0bb0#189",
        "54.01205772418403209777995565674845309183896998195770453405",
        "0x36.0316370b08dbf2c6a4eb52617696a3de65d5415234d388#189",
    );
    // - exp_diff == TWICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "563971925627753843356041629019151473018178607215.42",
        "0x62c960337e963a378ba6626ea422d8a5e623986f.6c#165",
        "1301375421.83361702516620516356439489325145225661938",
        "0x4d9169bd.d567ece47a47ef60371d48c969ba8765d4#165",
        "563971925627753843356041629019151473016877231793.59",
        "0x62c960337e963a378ba6626ea422d8a598922eb1.98#165",
    );
    // - exp_diff == Limb::WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "226.9305090753243797994707628568605406194",
        "0xe2.ee35d7bf263fda8c632644ad7c49d98#130",
        "4262448175090788889452.984188256984861391",
        "0xe71159efd3a67e736c.fbf3c2f8db72fb8#130",
        "-4262448175090788889226.053679181660481592",
        "-0xe71159efd3a67e728a.0dbdeb39b533210#130",
    );
    // - TWICE_WIDTH <= exp_diff < THRICE_WIDTH && a2 < HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && diff_2 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "0.0001220703125",
        "0x0.0008000000000000000000000000000000000000#145",
        "8.2855746158774225568154012162196749515653053e-50",
        "0x1.f0000fc000000000000003fffffffffffffcE-41#145",
        "0.0001220703125",
        "0x0.0008000000000000000000000000000000000000#145",
    );
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && a2 < HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "0.0156250000000000000000000000252435487789932816416198",
        "0x0.040000000000000000000001ffffffc00000000003f#167",
        "2.11758236813575084767080625169910490512847748461541e-22",
        "0xf.ffffffffffffffffffffffffffffffffff0000000E-19#167",
        "0.0156249999999999999997882417884299736942262010164499",
        "0x0.03ffffffffffffffff000001ffffffc00000000003f0#167",
    );
    // - exp_diff >= THRICE_WIDTH && a2 < HIGH_BIT in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.028440348325753776346855739098344065614209916020987e62",
        "0x4.000000000000000000000000000000000000000000E+51#168",
        "0.00781238079073842683897073489057921223679527623088422",
        "0x0.01fffe000007e000000038000007fffffffffe000000#168",
        "1.028440348325753776346855739098344065614209916020987e62",
        "0x4.000000000000000000000000000000000000000000E+51#168",
    );
    // - exp_diff < Limb::WIDTH && a2 == 0 first time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && a2 != 0 second time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "32767.9999999999999999999999999999999999999999996",
        "0x7fff.fffffffffffffffffffffffffffffffffff8#156",
        "32768.0000000000000000069388939039072268369037424",
        "0x8000.000000000000007ffffffffffff80000000#156",
        "-6.93889390390722683690374277451135137549581608853e-18",
        "-0x7.ffffffffffff800000008000000000000000000E-15#156",
    );
    // - exp_diff < Limb::WIDTH && a2 == 0 second time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "137438953471.99999999999999999999999999999",
        "0x1fffffffff.ffffffffffffffffffffffff#133",
        "137438953472.0",
        "0x2000000000.000000000000000000000000#133",
        "-1.2621774483536188886587657044524579674771e-29",
        "-0x1.000000000000000000000000000000000E-24#133",
    );

    // - in sub_float_significands_same_prec_ge_3w
    // - x_exp == y_exp in sub_float_significands_same_prec_ge_3w
    // - k < 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "0.0",
        "0x0.0",
    );
    // - x_exp > y_exp in sub_float_significands_same_prec_ge_3w
    // - exp_diff == 1 in sub_float_significands_same_prec_ge_3w
    // - !goto_sub_d1_no_lose && !goto_sub_d1_lose in sub_float_significands_same_prec_ge_3w
    // - limb < HIGH_BIT in sub_float_significands_same_prec_ge_3w
    // - exp_diff == 0 in sub_float_significands_same_prec_ge_3w
    // - goto_exact_normalize in sub_float_significands_same_prec_ge_3w
    // - limb != 0 in sub_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && limb != 0 && leading_zeros == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
    );
    // - k >= 0 && xs[k] < ys[k] in sub_float_significands_same_prec_ge_3w
    // - !goto_exact_normalize in sub_float_significands_same_prec_ge_3w
    // - limb == 0 in sub_float_significands_same_prec_ge_3w
    // - out[usize::exact_from(k)] != 0 in sub_float_significands_same_prec_ge_3w
    // - limb == 0 && leading_zeros != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "-3.186183822264904554057760795535423611182209110385237572148e-58",
        "-0x2.000000000000000000000000000000000000000000000000E-48#192",
    );
    // - k >= 0 && xs[k] >= ys[k] in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "3.186183822264904554057760795535423611182209110385237572148e-58",
        "0x2.000000000000000000000000000000000000000000000000E-48#192",
    );
    // - exp_diff == 0 && limb != 0 && leading_zeros != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "0.9999999999999999999999999999999999999999999999999999999997",
        "0x0.fffffffffffffffffffffffffffffffffffffffffffffffe#192",
    );
    // - 2 <= exp_diff < prec in sub_float_significands_same_prec_ge_3w
    // - dm != 0 && m == 0 in sub_float_significands_same_prec_ge_3w
    // - sx != 0 in sub_float_significands_same_prec_ge_3w
    // - !out[n - 1].get_highest_bit() in sub_float_significands_same_prec_ge_3w
    // - round_bit == 0 in sub_float_significands_same_prec_ge_3w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "0.5",
        "0x0.800000000000000000000000000000000000000000000000#192",
        "1.5",
        "0x1.800000000000000000000000000000000000000000000000#192",
    );
    // - limb == 0 && leading_zeros == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#193",
        "1.0000000000000000000000000000000000000000000000000000000002",
        "0x1.000000000000000000000000000000000000000000000001#193",
        "-1.5930919111324522770288803977677118055911045551926187860739e-58",
        "-0x1.000000000000000000000000000000000000000000000000E-48#193",
    );
    // - sx == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#193",
        "0.5",
        "0x0.8000000000000000000000000000000000000000000000000#193",
        "1.5",
        "0x1.800000000000000000000000000000000000000000000000#193",
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_ge_3w
    // - out_power_of_2 || round_bit == 0 in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest && !out_power_of_2 first time in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest && (round_bit == 0 || (round_bit != 0 && sticky_bit == 0 && (out[0] &
    //   shift_bit == 0 || prec == 1))) in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        "-3.0",
        "-0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - out[usize::exact_from(k)] == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000002",
        "0x1.000000000000000000000000000000000000000000000001#193",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#193",
        "-1.5930919111324522770288803977677118055911045551926187860739e-58",
        "-0x1.000000000000000000000000000000000000000000000000E-48#193",
    );
    // - round_bit != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000006",
        "0x1.000000000000000000000000000000000000000000000004#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        "-2.9999999999999999999999999999999999999999999999999999999994",
        "-0x2.fffffffffffffffffffffffffffffffffffffffffffffffc#192",
    );
    // - rm == Nearest && round_bit != 0 && (round_bit == 0 || sticky_bit != 0 || (out[0] &
    //   shift_bit != 0 && prec == 1)) in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest && !out_power_of_2 second time in sub_float_significands_same_prec_ge_3w
    test(
        "1.000000000000000000000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000000000000000000006#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        "-2.9999999999999999999999999999999999999999999999999999999987",
        "-0x2.fffffffffffffffffffffffffffffffffffffffffffffff8#192",
    );
    // - sticky_bit_2 == 0 && k > 0 third time in sub_float_significands_same_prec_ge_3w
    test(
        "0.5",
        "0x0.8000000000000000000000000000000000000000000000000#193",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#193",
        "-3.5",
        "-0x3.800000000000000000000000000000000000000000000000#193",
    );
    // - sticky_bit_2 != 0 || k <= 0 third time in sub_float_significands_same_prec_ge_3w
    test(
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        "0.5000000000000000000000000000000000000000000000000000000002",
        "0x0.800000000000000000000000000000000000000000000001#192",
        "3.5",
        "0x3.800000000000000000000000000000000000000000000000#192",
    );
    // - dm != 0 && m != 0 in sub_float_significands_same_prec_ge_3w
    // - out[n - 1].get_highest_bit() in sub_float_significands_same_prec_ge_3w
    test(
        "7.28057116938384227432903448367767196428679514765398378973101e-48",
        "0xa.a3fc2da1f20fb2d9771f86d3c16a444cd62d5d139e3935f24E-40#198",
        "3.5123473778825578958968695187657587760357139395948269588971e-27",
        "0x1.1646de419a6dbd3466f3081403a87d719b7a765a1ec69e4658E-22#198",
        "-3.51234737788255789588958894759637493376138490511114928693281e-27",
        "-0x1.1646de419a6dbd345c4f0be661b66dbec20356d34b05340208E-22#198",
    );
    // - exp_diff >= prec in sub_float_significands_same_prec_ge_3w
    // - exp_diff > prec in sub_float_significands_same_prec_ge_3w
    // - exp_diff != prec + 1 in sub_float_significands_same_prec_ge_3w
    test(
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        "0.023991386767031193042066748710708351501952890752924613005724",
        "0x0.06244cad8cd272134e34b325815ad281733f2c06231a0ee744#193",
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
    );
    // - dm == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "6.442552350746554109885349691592991892989624685631192235549e-6",
        "0x0.00006c168d38e231899f0fc85d1888549d5177bdceaee72e15060#192",
        "1476808010161862576835936576709144.7975622615653024045505082",
        "0x48cff00a780a50d34bb694ada218.cc2d0a55f25f9f9126258#192",
        "-1476808010161862576835936576709144.7975558190129516579963983",
        "-0x48cff00a780a50d34bb694ada218.cc2c9e3f6526bd5f9c868#192",
    );
    // - exp_diff == prec + 1 in sub_float_significands_same_prec_ge_3w
    // - sticky_bit_2 != 0 || k <= 0 second time in sub_float_significands_same_prec_ge_3w
    test(
        "29249291732025621624535078208.59212499364958152526111994335",
        "0x5e827271f9e9d261e7cb5540.979580eade814aae28ae9d3c8#192",
        "1.7056859397843570965021420438616279890690624515282312011749e-30",
        "0x2.2986d80d04731f28b49380410e3f4711dc2cc5f113c594a0E-25#192",
        "29249291732025621624535078208.59212499364958152526111994335",
        "0x5e827271f9e9d261e7cb5540.979580eade814aae28ae9d3c8#192",
    );
    // - limb > HIGH_BIT in sub_float_significands_same_prec_ge_3w
    // - y0 != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1958139908729.1847354007541959640287427302874567071533816044",
        "0x1c7ea3bd279.2f4ad1b8a7190307771c7e5767590237208b90#192",
        "688604646855.2266320591592881661509057171162297144871538944",
        "0xa054090dc7.3a048f025074a63da83a500a235d2b8fd9766d#192",
        "1269535261873.9581033415949077978778370131712269926662277102",
        "0x1279632c4b1.f54642b656a45cc9cee22e4d43fbd6a7471524#192",
    );
    // - y0 == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2473299914875391681216.10395653853096422269749051583697615",
        "0x8613ee70797f97a2c0.1a9ce54d32239153edab6ff15dad1c0#193",
        "7716446651886500012256.87778108230244207876352217543002925",
        "0x1a24f3592677b2760e0.e0b642d189564d3b4c797ad9c9cde8#193",
        "-5243146737011108331040.7738245437714778560660316595930531",
        "-0x11c3b4721edfb8fbe20.c6195d845732bbe75ece0ae86c20cc#193",
    );
    // - exp_diff == prec in sub_float_significands_same_prec_ge_3w
    // - sticky_bit_2 != 0 || k <= 0 first time in sub_float_significands_same_prec_ge_3w
    test(
        "4.0635838402455207229400698207668893925379768151364313942222e-23",
        "0x3.1202ecf10ff40b477337957dede18bd7b746884ec977474eE-19#194",
        "1174582238252884689829665592721065057.76655867827770290150723",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d4#194",
        "-1174582238252884689829665592721065057.76655867827770290150718",
        "-0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d0#194",
    );
    // - rm == Nearest && out_power_of_2 first time in sub_float_significands_same_prec_ge_3w
    test(
        "22300745198530623141535718272648361505980416.0",
        "0x1000000000000000000000000000000000000.000000000000#192",
        "8.470329472543003387009805160583577341645369940072346001613e-22",
        "0x3.ffffffffffffffe000000003ff0000003ffffffc00000000E-18#192",
        "22300745198530623141535718272648361505980416.0",
        "0x1000000000000000000000000000000000000.000000000000#192",
    );
    // - sticky_bit_2 == 0 && k > 0 second time in sub_float_significands_same_prec_ge_3w
    test(
        "1.6849966666969146159452711670928107852024276704905067469395e66",
        "0xf.ffffffffffff01fffff00000000000000000000000000000E+54#193",
        "33554432.00000000000000044408920985006261616945266723632812",
        "0x2000000.0000000000001ffffffffffffffffffffffffffffe#193",
        "1.6849966666969146159452711670928107852024276704905067469395e66",
        "0xf.ffffffffffff01fffff00000000000000000000000000000E+54#193",
    );
    // - limb == HIGH_BIT in sub_float_significands_same_prec_ge_3w
    // - l >= 0 first time in sub_float_significands_same_prec_ge_3w
    // - xs[l] != yl_shifted in sub_float_significands_same_prec_ge_3w
    // - l >= 0 && xs[l] <= yl_shifted in sub_float_significands_same_prec_ge_3w
    // - goto_sub_d1_no_lose || goto_sub_d1_lose in sub_float_significands_same_prec_ge_3w
    test(
        "2047.9999999999999999999999999999747564510329303127198868805",
        "0x7ff.fffffffffffffffffffffffe00000000003c000003fffc#193",
        "4095.99999999999999988897769753748434595763683319091796875",
        "0xfff.fffffffffffff800000000000000000000000000000000#193",
        "-2047.9999999999999998889776975375095895066039028781980818695",
        "-0x7ff.fffffffffffff80000000001ffffffffffc3fffffc0004#193",
    );
    // - sticky_bit_2 == 0 && k > 0 first time in sub_float_significands_same_prec_ge_3w
    test(
        "0.0002442598197376355528831482438462803411504065058668775410457",
        "0x0.001001fff000000000000003ffff00000001fffffff07fffffe#192",
        "5.834076822994820350447560050418866475553361427251759468222e-62",
        "0x1.8000000000000000000000000000000000001c00007ffffeE-51#192",
        "0.00024425981973763555288314824384628034115040650586687754104564",
        "0x0.001001fff000000000000003ffff00000001fffffff07fffffc#192",
    );
    // - l >= 0 && xs[l] > yl_shifted in sub_float_significands_same_prec_ge_3w
    test(
        "127.99999999999999999999998841947063540272586186468301133273",
        "0x7f.fffffffffffffffffff1fffff00007ffffffffffffffff8#192",
        "255.99999999999999999999999979320484686174308128214782699291",
        "0xff.ffffffffffffffffffffc00000000000000000000000ff#192",
        "-128.0000000000000000000000113737342114590172194174648156602",
        "-0x80.0000000000000000000dc0000ffff80000000000000100#192",
    );
    // - out_power_of_2 && round_bit != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "81129638414606681695789005144064.0",
        "0x400000000000000000000000000.0000000000000000000000#193",
        "6.46392625738094777466974989455420015132331189664152496326e-27",
        "0x2.001ffffffffe0000001fffff000000ffffff80000000000eE-22#193",
        "81129638414606681695789005144063.999999999999999999999999994",
        "0x3ffffffffffffffffffffffffff.fffffffffffffffffffffe#193",
    );
    // - rm == Nearest && out_power_of_2 second time in sub_float_significands_same_prec_ge_3w
    test(
        "9.5367431640625e-7",
        "0x0.00001000000000000000000000000000000000000000000000000#192",
        "7.596455102175746880546879414772134233793171691679642324806e-65",
        "0x8.00000ffffffffffe000000000000000000000000001ffffE-54#192",
        "9.536743164062499999999999999999999999999999999999999999998e-7",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffE-6#192",
    );
    // - xs[l] == yl_shifted in sub_float_significands_same_prec_ge_3w
    test(
        "1180591620717411303423.9999999999999999999999999999999999998",
        "0x3fffffffffffffffff.ffffffffffffffffffffffffffffffc#192",
        "2361183241434822606847.9999999999999999930619531290400259223",
        "0x7fffffffffffffffff.ffffffffffffff8003ffffffffffff8#192",
        "-1180591620717411303423.9999999999999999930619531290400259225",
        "-0x3fffffffffffffffff.ffffffffffffff8003ffffffffffffc#192",
    );
    // - l < 0 first time in sub_float_significands_same_prec_ge_3w
    // - l < 0 second time in sub_float_significands_same_prec_ge_3w
    // - yl_shifted != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "32767.999999999999999999999999999999999999999999999999999995",
        "0x7fff.ffffffffffffffffffffffffffffffffffffffffffff8#192",
        "65535.99999999999999999999999999999999999999999999999999999",
        "0xffff.ffffffffffffffffffffffffffffffffffffffffffff#192",
        "-32767.999999999999999999999999999999999999999999999999999995",
        "-0x7fff.ffffffffffffffffffffffffffffffffffffffffffff8#192",
    );
    // - yl_shifted == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "34359738367.999999999999999999999999999999999999999999999989",
        "0x7ffffffff.fffffffffffffffffffffffffffffffffffffff0#192",
        "68719476735.99999999999999999999999999999999999999999999999",
        "0xfffffffff.fffffffffffffffffffffffffffffffffffffff#192",
        "-34359738368.0",
        "-0x800000000.000000000000000000000000000000000000000#192",
    );

    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", "-0.5", "-0x0.8#2");
    test("0.5", "0x0.8#1", "3.0", "0x3.0#2", "-2.0", "-0x2.0#2");
    test("0.2", "0x0.4#1", "4.0", "0x4.0#2", "-4.0", "-0x4.0#2");
    test(
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        "1.07183972513958531257713938927815e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222827281009532032311766811006",
        "0x0.00f5402c0bbf2e1505ed1467de9fe#107",
    );
    test(
        "2589062031404.0",
        "0x25ad01f682c.0#43",
        "4351572166934988.581719655389852344796925751245753159273257621838622031922945531041952618\
        810238932316",
        "0xf75bb593981cc.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
        "-4348983104903584.58171965538985234479692575124575315927325762183862203192294553104195261\
        8810238932316",
        "-0xf7360891a19a0.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
    );

    // - in sub_float_significands_general
    // - in exponent_shift_compare
    // - sdiff_exp >= 0 in exponent_shift_compare
    // - diff_exp == 0 first time in exponent_shift_compare
    // - xi >= 0 && yi >= 0 && xs[xi] == ys[yi] in exponent_shift_compare
    // - xi < 0 in exponent_shift_compare
    // - xi < 0 && yi < 0 in exponent_shift_compare
    // - sign == Equal in sub_float_significands_general
    test("1.0", "0x1.0#1", "1.0", "0x1.0#2", "0.0", "0x0.0");
    // - diff_exp != 0 first time in exponent_shift_compare
    // - diff_exp < Limb::WIDTH in exponent_shift_compare
    // - diff_exp != 0 second time in exponent_shift_compare
    // - (yi < 0 && lasty == 0) || high_dif != 0 || dif != 1 in exponent_shift_compare
    // - high_dif == 0 in exponent_shift_compare
    // - dif.is_power_of_2() in exponent_shift_compare
    // - yi < 0 && lasty == 0 in exponent_shift_compare
    // - sign != Equal in sub_float_significands_general
    // - sign != Less in sub_float_significands_general
    // - !neg in sub_float_significands_general
    // - max(out_prec, x_prec) + 2 > exp_diff in sub_float_significands_general
    // - shift_x != 0 in sub_float_significands_general
    // - shift_y == 0 in sub_float_significands_general
    // - cancel >= exp_diff in sub_float_significands_general
    // - out_len + cancel1 <= xs_len in sub_float_significands_general
    // - out_len + cancel2 > 0 in sub_float_significands_general
    // - cancel2 >= 0 in sub_float_significands_general
    // - out_len + cancel2 <= ys_len in sub_float_significands_general
    // - rm == Nearest in sub_float_significands_general
    // - rm == Nearest && carry <= Limb::power_of_2(sh - 1) && (0 >= carry || carry >=
    //   Limb::power_of_2(sh - 1)) in sub_float_significands_general
    // - !goto_truncate in sub_float_significands_general
    // - ixs_len <= 0 && iys_len <= 0 in sub_float_significands_general
    // - !goto_truncate && !goto_end_of_sub second time in sub_float_significands_general
    // - rm != Nearest || cmp_low == 0 in sub_float_significands_general
    // - !goto_end_of_sub in sub_float_significands_general
    // - out[out_len - 1] >> (Limb::WIDTH - 1) != 0 in sub_float_significands_general
    // - cancel != 0 in sub_float_significands_general
    test("2.0", "0x2.0#1", "1.0", "0x1.0#2", "1.0", "0x1.0#2");
    // - sdiff_exp < 0 in exponent_shift_compare
    // - sign == Less in sub_float_significands_general
    // - neg in sub_float_significands_general
    test("1.0", "0x1.0#2", "2.0", "0x2.0#1", "-1.0", "-0x1.0#2");
    // - xi < 0 || yi < 0 || xs[xi] != ys[yi] in exponent_shift_compare
    // - xi >= 0 in exponent_shift_compare
    // - yi >= 0 second time in exponent_shift_compare
    // - xs[xi] < ys[yi] in exponent_shift_compare
    // - diff_exp == 0 second time in exponent_shift_compare
    // - shift_y != 0 in sub_float_significands_general
    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", "-0.5", "-0x0.8#2");
    // - xs[xi] >= ys[yi] in exponent_shift_compare
    test("1.5", "0x1.8#2", "1.0", "0x1.0#1", "0.5", "0x0.8#2");
    // - shift_x == 0 in sub_float_significands_general
    // - cancel < exp_diff in sub_float_significands_general
    // - ixs_len > 0 || iys_len > 0 in sub_float_significands_general
    // - ixs_len <= 0 in sub_float_significands_general
    // - iys_len condition in sub_float_significands_general
    // - cmp_low == 0 first time in sub_float_significands_general
    // - rm == Nearest && (sh != 0 || k != 0) in sub_float_significands_general
    // - cmp_low == 0 second time in sub_float_significands_general
    // - !goto_truncate && !goto_end_of_sub first time in sub_float_significands_general
    // - cancel == 0 in sub_float_significands_general
    test("0.5", "0x0.8#1", "1.5", "0x1.8#2", "-1.0", "-0x1.0#2");
    // - !dif.is_power_of_2() in exponent_shift_compare
    test("0.5", "0x0.8#1", "2.0", "0x2.0#2", "-1.5", "-0x1.8#2");
    // - cmp_low != 0 first time in sub_float_significands_general
    // - cmp_low > 0 in sub_float_significands_general
    // - cmp_low > 0 && rm == Nearest in sub_float_significands_general
    // - cmp_low > 0 && rm == Nearest && xx == yy in sub_float_significands_general
    // - rm == Nearest && cmp_low != 0 in sub_float_significands_general
    // - (out[0] >> sh) & 1 == 0 in sub_float_significands_general
    test("0.5", "0x0.8#1", "3.0", "0x3.0#2", "-2.0", "-0x2.0#2");
    // - (out[0] >> sh) & 1 != 0 in sub_float_significands_general
    // - cmp_low >= 0 in sub_float_significands_general
    // - cmp_low >= 0 && !carry in sub_float_significands_general
    test("4.0", "0x4.0#1", "1.2", "0x1.4#3", "3.0", "0x3.0#3");
    // - cmp_low >= 0 && carry in sub_float_significands_general
    test("4.0", "0x4.0#1", "0.5", "0x0.8#2", "4.0", "0x4.0#2");
    // - rm == Nearest && carry > Limb::power_of_2(sh - 1) in sub_float_significands_general
    // - goto_truncate in sub_float_significands_general
    test("3.0", "0x3.0#2", "0.2", "0x0.4#1", "3.0", "0x3.0#2");
    // - rm == Nearest && carry <= Limb::power_of_2(sh - 1) && 0 < carry && carry <
    //   Limb::power_of_2(sh - 1) in sub_float_significands_general
    test("4.0", "0x4.0#1", "0.8", "0x0.c#2", "3.0", "0x3.0#2");
    // - max(out_prec, x_prec) + 2 <= exp_diff in sub_float_significands_general
    // - in round_helper
    // - dest_prec >= x_prec in round_helper
    // - !increment_exp in sub_float_significands_general
    // - inexact == 0 && rm != Down && rm != Floor in sub_float_significands_general
    test("0.2", "0x0.4#1", "4.0", "0x4.0#2", "-4.0", "-0x4.0#2");
    // - diff_exp >= Limb::WIDTH in exponent_shift_compare first time
    test(
        "8.82188e11",
        "0xc.d668E+9#18",
        "9.75459983374e122",
        "0x1.79c17f063aE+102#40",
        "-9.75459983374e122",
        "-0x1.79c17f063aE+102#40",
    );
    // - cancel2 < 0 in sub_float_significands_general
    // - out_len - neg_cancel2 <= ys_len in sub_float_significands_general
    test(
        "3.29008365861415556134836580980448399733562188e-9",
        "0xe.217c389f8c9fd22042f5ed70da20cfb9f1ecE-8#146",
        "3719044561792922503530448846362960.3599330496921301151502834994",
        "0xb75cf116bc625ef1eab58f3c9950.5c2492852d5fb6817443c180#205",
        "-3719044561792922503530448846362960.359933046402046456536127938",
        "-0xb75cf116bc625ef1eab58f3c9950.5c2492770be37de1e7a3ef60#205",
    );
    // - out_len + cancel1 > xs_len in sub_float_significands_general
    // - cancel1 < xs_len in sub_float_significands_general
    test(
        "1.07183972513958531257713938927815e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        "-0.00374222827281009532032311766811006",
        "-0x0.00f5402c0bbf2e1505ed1467de9fe#107",
    );
    // - out_len + cancel2 > ys_len in sub_float_significands_general
    test(
        "2589062031404.0",
        "0x25ad01f682c.0#43",
        "4351572166934988.581719655389852344796925751245753159273257621838622031922945531041952618\
        810238932316",
        "0xf75bb593981cc.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
        "-4348983104903584.58171965538985234479692575124575315927325762183862203192294553104195261\
        8810238932316",
        "-0xf7360891a19a0.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
    );
    // - yi >= 0 && lasty != 0 in exponent_shift_compare
    // - xi < 0 fourth time in exponent_shift_compare
    // - lasty == 0 in exponent_shift_compare
    // - yi < 0 || ys[yi] != 0 in exponent_shift_compare
    // - yi >= 0 fourth time in exponent_shift_compare
    test(
        "0.002",
        "0x0.008#2",
        "1.107886492190627864290739752375593855464628579e-38",
        "0x3.c51af197224960473945f6944424f855697e2E-32#149",
        "0.001953124999999999999999999999999999988921135078",
        "0x0.007ffffffffffffffffffffffffffffc3ae50e68#149",
    );
    // - cmp_low > 0 && rm == Nearest && xx < yy in sub_float_significands_general
    // - goto_truncate || goto_end_of_sub first time in sub_float_significands_general
    // - goto_truncate || goto_end_of_sub second time in sub_float_significands_general
    test(
        "1.521287e-9",
        "0x6.88ac4E-8#21",
        "6.842391932190563625e-20",
        "0x1.431f7157e61b20d0E-16#62",
        "1.5212870962270854807e-9",
        "0x6.88ac3ffebce08eaE-8#62",
    );
    // - out_len - neg_cancel2 > ys_len in sub_float_significands_general
    test(
        "0.00678514868524062",
        "0x0.01bcabe7b39c71#49",
        "492541199943575879969.43922949854802247248767794847124160758",
        "0x1ab361dbc0e97d4121.7071582bb3c5bd22b8f59cfc93c72a98#194",
        "-492541199943575879969.43244434986278185350564935536210679008",
        "-0x1ab361dbc0e97d4121.6eb4ac4400294c22b8f59cfc93c72a98#194",
    );
    // - rm == Nearest && sh == 0 && k == 0 in sub_float_significands_general
    // - rm == Nearest && cmp_low >= 0 in sub_float_significands_general
    // - rm == Nearest && cmp_low >= 0 && yy < half in sub_float_significands_general
    // - rm == Nearest && cmp_low >= 0 && cmp_low <= 0 in sub_float_significands_general
    test(
        "5.7505515877842013577e-7",
        "0x9.a5d7d56cabed47dE-6#64",
        "1.1758894e-14",
        "0x3.4f515E-12#22",
        "5.7505514701952590309e-7",
        "0x9.a5d7d21d5a9d47dE-6#64",
    );
    // - rm == Nearest && cmp_low < 0 in sub_float_significands_general
    // - rm == Nearest && cmp_low < 0 && yy < half in sub_float_significands_general
    // - cmp_low < 0 in sub_float_significands_general first time
    // - cmp_low < 0 && rm == Nearest in sub_float_significands_general
    // - rm == Nearest && (xx > yy || sh > 0 || cmp_low == -1) in sub_float_significands_general
    test(
        "8319983682.218895978935307677994592087137128849954503237724",
        "0x1efe8e042.3809911ec0c7f99b114d2930720001b00aa46846#192",
        "1.88392800575e35",
        "0x2.4487b7174E+29#37",
        "-188392800574747474298435817696599997.78110402106469232200542",
        "-0x24487b7173fffffffffffe10171fbd.c7f66ee13f380664eec#192",
    );
    // - rm == Nearest && cmp_low < 0 && yy >= half in sub_float_significands_general
    // - rm == Nearest && xx < yy && sh <= 0 && cmp_low != -1 in sub_float_significands_general
    // - goto_end_of_sub in sub_float_significands_general
    test(
        "2.36288970224581301467472547462526069521e-27",
        "0xb.b3518c72d51185c09977eb6e009c2c0E-23#128",
        "3.413020751e-12",
        "0x3.c0ae105E-10#30",
        "-3.413020751029435178256669624768773569e-12",
        "-0x3.c0ae104fffff44cae738d2aee7a3f668E-10#128",
    );
    // - xi >= 0 fourth time in exponent_shift_compare
    // - diff_exp >= Limb::WIDTH in exponent_shift_compare second time
    // - xs[xi] == yy in exponent_shift_compare
    test(
        "1125899906842624.0",
        "0x4000000000000.000000000000#98",
        "1.166815364554e-61",
        "0x2.ffffffff80E-51#39",
        "1125899906842624.0",
        "0x4000000000000.000000000000#98",
    );
    // - 0 < diff_exp < Limb::WIDTH && yi >= 0 in exponent_shift_compare
    // - xs[xi] != yy in exponent_shift_compare
    test(
        "9671406556917033397649408.0",
        "0x800000000000000000000.0000#99",
        "65536.015624999999999999946",
        "0x10000.03ffffffffffffff00#87",
        "9671406556917033397583871.98438",
        "0x7fffffffffffffffeffff.fc00#99",
    );
    // - diff_exp == 0 && yi >= 0 in exponent_shift_compare
    test(
        "1.3877787807814456370485565165946e-17",
        "0xf.ffffffffffffe007ffffffff8E-15#101",
        "128.0000000000000000000000000008077935669463159990585083341",
        "0x80.00000000000000000000003ffffffffffffe0000000000#190",
        "-127.9999999999999999861222121929933371964607508331127668586",
        "-0x7f.ffffffffffffff00000000400001ff7ffffe0008000000#190",
    );
    // - rm == Nearest && xx == yy && sh <= 0 && cmp_low != -1 in sub_float_significands_general
    // - cmp_low < 0 in sub_float_significands_general second time
    test(
        "2.220581574517834801066783605693002897371795957735742517101e-16",
        "0x1.0003fffffe00000000007ffffffffffffffffffffff000feE-13#192",
        "8.881784e-16",
        "0x4.00000E-13#21",
        "-6.661202622483417522322269739033559602628204042264257482898e-16",
        "-0x2.fffc000001ffffffffff80000000000000000000000fff00E-13#192",
    );
    // - diff_exp < Limb::WIDTH && yi < 0 in exponent_shift_compare
    test(
        "-1.8e-12",
        "-0x1.fcE-10#7",
        "-524288.0000000000017621525",
        "-0x80000.0000000001f00078#81",
        "524287.9999999999999573739",
        "0x7ffff.fffffffffff40078#81",
    );
    // - (yi >= 0 || lasty != 0) && high_dif == 0 && dif == 1 in exponent_shift_compare
    // - xi >= 0 third time in exponent_shift_compare
    // - yi >= 0 && diff_exp != 0 in exponent_shift_compare
    // - high_dif != 0 in exponent_shift_compare
    // - dif == 0 in exponent_shift_compare
    test(
        "3.99999999999999999957",
        "0x3.fffffffffffffff80#67",
        "4.0",
        "0x4.00000000000000000#68",
        "-4.33680868994201773603e-19",
        "-0x8.0000000000000000E-16#68",
    );
    // - xi < 0 third time in exponent_shift_compare
    // - cancel1 >= xs_len in sub_float_significands_general
    test(
        "6.77626357803440271254657930615627554e-21",
        "0x1.ffffffffffffffffffffc01fff800E-17#117",
        "7.0e-21",
        "0x2.0E-17#4",
        "-6.99280860154738521672154778865541105e-46",
        "-0x3.fe000800000000000000000000000E-38#117",
    );
    // - yi >= 0 && diff_exp == 0 in exponent_shift_compare
    // - dif != 0 in exponent_shift_compare
    test(
        "1.45519152351431153846750277125465800054371356958e-11",
        "0x1.00000001ffffffffffffffffffffffffffffffcE-9#155",
        "1.455191523514311538467502771254658000526607875496523229114700566610230516799191954282190\
        364e-11",
        "0x1.00000001fffffffffffffffffffffffc0000000000000000000000000000fffffffffff87f8E-9#301",
        "1.710569408086637569000134230861426729009957349416545362986887684243433631257558931055548\
        3967e-49",
        "0x3.ffffffbfffffffffffffffffffff00000000000780800000000000000000000000000000000E-41#301",
    );
    // - xs[xi] == 0 in exponent_shift_compare
    // - xi < 0 second time in exponent_shift_compare
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        "0.0",
        "0x0.0",
    );
    // - xi >= 0 second time in exponent_shift_compare
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        "0.0",
        "0x0.0",
    );
}

#[test]
fn test_sub_prec() {
    let test = |s, s_hex, t, t_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (diff, o) = x.clone().sub_prec(y.clone(), prec);
        assert!(diff.is_valid());

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.clone().sub_prec_val_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = x.sub_prec_ref_val(y.clone(), prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = x.sub_prec_ref_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_prec_assign(y.clone(), prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_prec_assign_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = add_prec_round_naive(x.clone(), -y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test("NaN", "NaN", "NaN", "NaN", 1, "NaN", "NaN", Ordering::Equal);
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "-2.0",
        "-0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "-4.0",
        "-0x4.0#1",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        1,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "0.5",
        "0x0.8#1",
        "4.0",
        "0x4.0#1",
        2,
        "-4.0",
        "-0x4.0#2",
        Ordering::Less,
    );
    test(
        "8.829e30",
        "0x6.f70E+25#13",
        "0.05475",
        "0x0.0e04#10",
        36,
        "8.8289883602e30",
        "0x6.f70000000E+25#36",
        Ordering::Greater,
    );
    test(
        "1.6390041924e12",
        "0x1.7d9c433e8E+10#34",
        "1.8272611248130303984e40",
        "0x3.5b2c8ea9f3a386bE+33#62",
        40,
        "-1.827261124813e40",
        "-0x3.5b2c8ea9f4E+33#40",
        Ordering::Less,
    );
    test(
        "5.96031697199505378e-9",
        "0x1.9997014b4948928E-7#60",
        "0.000014",
        "0x0.0000f#4",
        3,
        "-0.000013",
        "-0x0.0000e#3",
        Ordering::Greater,
    );
    test(
        "8.7741695e-21",
        "0x2.96f51f0E-17#27",
        "1.86647e10",
        "0x4.5880E+8#16",
        11,
        "-1.866e10",
        "-0x4.58E+8#11",
        Ordering::Greater,
    );
    // - dest_prec < x_prec in round_helper
    // - sh != 0 in round_helper
    // - sh != 0 && (rm == Nearest || rb == 0) in round_helper
    // - sh != 0 && (rm == Nearest || rb == 0) && (n == 0 || sb != 0) in round_helper
    // - rm == Nearest in round_helper
    // - rm == Nearest && rb != 0 && sb != 0 in round_helper
    // - !increment second time in round_helper
    // - inexact != 0 && inexact != MPFR_EVEN_INEX in sub_float_significands_general
    test(
        "1.6390041924e12",
        "0x1.7d9c433e8E+10#34",
        "1.8272611248130303984e40",
        "0x3.5b2c8ea9f3a386bE+33#62",
        40,
        "-1.827261124813e40",
        "-0x3.5b2c8ea9f4E+33#40",
        Ordering::Less,
    );
    // - rm == Nearest && rb == 0 in round_helper
    test(
        "13104.5238818416080254535",
        "0x3330.861d1ed0acba8a3a#77",
        "2.854e-35",
        "0x2.5fE-29#10",
        17,
        "13104.5",
        "0x3330.8#17",
        Ordering::Less,
    );
    // - out_len + cancel2 <= 0 in sub_float_significands_general
    test(
        "1.73414747294406e-17",
        "0x1.3fe4cc8cf520E-14#48",
        "5095194424.1679374580403939884785489",
        "0x12fb27f38.2afdf3020e8eaac84a7ec#116",
        62,
        "-5095194424.167937458",
        "-0x12fb27f38.2afdf300#62",
        Ordering::Greater,
    );
    // - rm == Nearest && rb != 0 && sb == 0 in round_helper
    // - xs[o] & ulp != 0 in round_helper
    // - increment first time in round_helper
    // - increment_exp in sub_float_significands_general
    // - inexact != 0 || rm == Down || rm == Floor && (inexact == 0 || inexact == MPFR_EVEN_INEX) in
    //   sub_float_significands_general
    // - !out[out_len - 1].get_highest_bit() in sub_float_significands_general
    test(
        "5.96031697199505378e-9",
        "0x1.9997014b4948928E-7#60",
        "0.000014",
        "0x0.0000f#4",
        3,
        "-0.000013",
        "-0x0.0000e#3",
        Ordering::Greater,
    );
    // - xs[o] & ulp == 0 in round_helper
    test(
        "8.7741695e-21",
        "0x2.96f51f0E-17#27",
        "1.86647e10",
        "0x4.5880E+8#16",
        11,
        "-1.866e10",
        "-0x4.58E+8#11",
        Ordering::Greater,
    );
    // - !increment first time in round_helper
    // - out[out_len - 1].get_highest_bit() in sub_float_significands_general
    test(
        "1.3482e25",
        "0xb.27E+20#12",
        "1.12202588817e-11",
        "0xc.5638dc08E-10#36",
        11,
        "1.348e25",
        "0xb.26E+20#11",
        Ordering::Less,
    );
    // - increment second time in round_helper
    test(
        "4.53892e18",
        "0x3.efd8E+15#16",
        "3412.3125291",
        "0xd54.5001e8#33",
        1,
        "5.0e18",
        "0x4.0E+15#1",
        Ordering::Greater,
    );
    // - ixs_len > 0 in sub_float_significands_general
    test(
        "6058.05208272591415306446968882359605946955168456454",
        "0x17aa.0d554b247ce1b6ab28ba39c8d5992a74c7ac91a#169",
        "0.000144566892208323",
        "0x0.0009796e12f9784#47",
        64,
        "6058.0519381590219448",
        "0x17aa.0d4bd1b669e84#64",
        Ordering::Greater,
    );
    // - sh == 0 in round_helper
    // - sh == 0 && (rm == Nearest || rb == 0) in round_helper
    // - sh == 0 && (rm == Nearest || rb == 0) && (n == 0 || sb != 0) in round_helper
    test(
        "3.6596517369110659089355442395654891585e48",
        "0x2.810875a0ca3206afd8c6cf841941830E+40#123",
        "1545.699550397407201099813420288295",
        "0x609.b315bc1ec48a143a74bd53048#109",
        64,
        "3.6596517369110659089e48",
        "0x2.810875a0ca3206b0E+40#64",
        Ordering::Greater,
    );
    // - rm == Nearest && cmp_low >= 0 && cmp_low > 0 in sub_float_significands_general
    // - cmp_low > 0 && rm == Nearest && xx > yy in sub_float_significands_general
    // - cmp_low == 2 && rm == Nearest && xx > yy in sub_float_significands_general
    // - cmp_low == 2 && rm == Nearest && xx > yy && !carry in sub_float_significands_general
    test(
        "2.80915429604669102593383052436808885401854724410738829e-11",
        "0x1.ee310ffa09a06a6361f52c2cd8a9569a780b775dc213E-9#177",
        "519241072363118296470.333928838103121952666621563036",
        "0x1c25eadc41d4907d96.557c5c3ed81cab65dab0cf920#166",
        64,
        "-5.1924107236311829648e20",
        "-0x1.c25eadc41d4907daE+17#64",
        Ordering::Less,
    );
    // - cmp_low > 0 && cmp_low != 2 && rm == Nearest && xx > yy in sub_float_significands_general
    test(
        "559935046210054011882951826578284118061013900.5853448",
        "0x191bbd3588c78488c2f4d122814d5fb34edb8c.95d928#170",
        "3.027932e11",
        "0x4.67fe2E+9#22",
        63,
        "5.599350462100540119e44",
        "0x1.91bbd3588c78488cE+37#63",
        Ordering::Less,
    );
    // - sh != 0 && (rm == Nearest || rb == 0) && n != 0 && sb == 0 in round_helper
    test(
        "7184368264698708563285024670194469326968686224.86386349506591",
        "0x1422880c600dc4fd90a02f1814859aafd658690.dd2628738430#198",
        "1.0296060328202e-24",
        "0x1.3ea5cb49bdaE-20#44",
        61,
        "7.184368264698708565e45",
        "0x1.422880c600dc4feE+38#61",
        Ordering::Greater,
    );
    // - not iys_len condition in sub_float_significands_general
    test(
        "0.005406335446698931371156460215539762984788400068",
        "0x0.01624f41ef4d9361bd396d1b5ff5c84cbeacdc10#153",
        "268934084586659427574577798723115132232.14094768348",
        "0xca52d21696ccb42e28dd0ff5c50ba548.241525bafe#167",
        62,
        "-2.689340845866594276e38",
        "-0xc.a52d21696ccb430E+31#62",
        Ordering::Less,
    );
    // - out[out_len - 1] >> (Limb::WIDTH - 1) == 0 in sub_float_significands_general
    test(
        "64.0",
        "0x40.000000000000000000000000#101",
        "3.388131587461699328349286349380665493965593642335603946234186645761911188725141218987660\
        374e-21",
        "0xf.fffff007f8000000007ffc00ffff00007fffffffffffffff80003ffff800001fffffffff80E-18#298",
        32,
        "64.0",
        "0x40.0000000#32",
        Ordering::Greater,
    );
    // - xi < 0 && yi >= 0 in exponent_shift_compare
    // - yi < 0 second time in exponent_shift_compare
    // - xs[xi] != 0 in exponent_shift_compare
    test(
        "8.0",
        "0x8.00#10",
        "8.000000000000000000001799531422609112",
        "0x8.0000000000000000087f8000000000#121",
        146,
        "-1.79953142260911170668274960959820418793242425e-21",
        "-0x8.7f8000000000000000000000000000000000E-18#146",
        Ordering::Equal,
    );
    // - yi >= 0 && ys[yi] == 0 in exponent_shift_compare
    // - yi < 0 fourth time in exponent_shift_compare
    test(
        "65536.0",
        "0x10000.0000000000000000000#90",
        "1.0e5",
        "0x2.0E+4#2",
        146,
        "-65536.0",
        "-0x10000.000000000000000000000000000000000#146",
        Ordering::Equal,
    );
    // - rm == Nearest && cmp_low >= 0 && yy >= half in sub_float_significands_general
    test(
        "3004622760.998488672936478517",
        "0xb316e7a8.ff9cf423b0491b4#90",
        "1355498858579258.8596",
        "0x4d0d1abf5853a.dc0ec#70",
        64,
        "-1355495853956497.8611",
        "-0x4d0d0f8de9d91.dc70#64",
        Ordering::Greater,
    );
    // - cmp_low == 2 && rm == Nearest && xx > yy && carry in sub_float_significands_general
    test(
        "2596143477507256672744148570210303.750244140624999888977803416602752745179193772357882662\
        973853959140044204",
        "0x7ffff000000000000000001fffff.c00ffffffffff800007ffffffffffffffe000001ffffffff80000001ff\
        fc#349",
        "4.2351647e-22",
        "0x1.ffffffcE-18#27",
        18,
        "2.59615e33",
        "0x8.0000E+27#18",
        Ordering::Greater,
    );
    // - lasty != 0 in exponent_shift_compare
    test(
        "6.2230152778611417071440640537801242403342227837862e-61",
        "0xf.ffffffffffffffffffffffffffffff1ffff3fffeE-51#163",
        "6.0e-61",
        "0x1.0E-50#4",
        11,
        "-2.56e-98",
        "-0xe.00E-82#11",
        Ordering::Greater,
    );
    // - sh == 0 && (rm == Nearest || rb == 0) && n != 0 && sb == 0 in round_helper
    test(
        "1.56676e-64",
        "0x1.07ff8E-53#18",
        "127.906249999999999993061106096092771622352302164",
        "0x7f.e7ffffffffffff8000000000000000000001ff8#160",
        64,
        "-127.906249999999999993",
        "-0x7f.e7ffffffffffff8#64",
        Ordering::Greater,
    );
    // - yi < 0 third time in exponent_shift_compare
    test(
        "6.9e-18",
        "0x8.0E-15#6",
        "6.9388939039072283773e-18",
        "0x7.fffffffffffffff8E-15#64",
        86,
        "3.7615819226313200254999569e-37",
        "0x8.000000000000000000000E-31#86",
        Ordering::Equal,
    );
}

#[test]
fn sub_prec_fail() {
    assert_panic!(Float::NAN.sub_prec(Float::NAN, 0));
    assert_panic!(Float::NAN.sub_prec_val_ref(&Float::NAN, 0));
    assert_panic!(Float::NAN.sub_prec_ref_val(Float::NAN, 0));
    assert_panic!(Float::NAN.sub_prec_ref_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.sub_prec_assign(Float::NAN, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.sub_prec_assign_ref(&Float::NAN, 0)
    });
}

#[test]
fn test_sub_round() {
    let test = |s, s_hex, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (diff, o) = x.clone().sub_round(y.clone(), rm);
        assert!(diff.is_valid());

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.clone().sub_round_val_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = x.sub_round_ref_val(y.clone(), rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = x.sub_round_ref_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_round_assign(y.clone(), rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_round_assign_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_diff, rug_o) =
                rug_sub_round(rug::Float::exact_from(&x), rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_diff)),
                ComparableFloatRef(&diff),
            );
            assert_eq!(rug_o, o);
        }

        let (diff_alt, o_alt) = add_prec_round_naive(
            x.clone(),
            -&y,
            max(x.significant_bits(), y.significant_bits()),
            rm,
        );
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "4.555806215962889",
        "0x4.8e4950f079600#53",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "4.555806215962889",
        "0x4.8e4950f079600#53",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Nearest,
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
        Ordering::Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Nearest,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Exact,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Nearest,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Exact,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "-4.555806215962889",
        "-0x4.8e4950f079600#53",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "-4.555806215962888",
        "-0x4.8e4950f0795fc#53",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "-4.555806215962888",
        "-0x4.8e4950f0795fc#53",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "-4.555806215962889",
        "-0x4.8e4950f079600#53",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "-4.555806215962889",
        "-0x4.8e4950f079600#53",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        RoundingMode::Floor,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        RoundingMode::Ceiling,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        RoundingMode::Down,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        RoundingMode::Up,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );

    // Note different behavior for Floor
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.5",
        "0x0.8#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Down,
        "-1.0",
        "-0x1.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    // - in sub_float_significands_same_prec_lt_w
    // - x_exp == y_exp in sub_float_significands_same_prec_lt_w
    // - x == y in sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp < y_exp in sub_float_significands_same_prec_lt_w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_lt_w
    // - leading_zeros != 0 in sub_float_significands_same_prec_lt_w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.0#1",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_lt_w
    // - neg in sub_float_significands_same_prec_lt_w
    // - rm == Nearest in sub_float_significands_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & shift_bit) != 0) in
    //   sub_float_significands_same_prec_lt_w
    // - rm == Nearest && round_bit != 0 && sticky_bit != 0 && diff == 0 in
    //   sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "4.0",
        "0x4.0#1",
        RoundingMode::Nearest,
        "-4.0",
        "-0x4.0#1",
        Ordering::Less,
    );
    // - !neg in sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "0.2",
        "0x0.4#1",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Greater,
    );
    // - x < y in sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#2",
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "-0.5",
        "-0x0.8#2",
        Ordering::Equal,
    );
    // - x > y in sub_float_significands_same_prec_lt_w
    test(
        "1.5",
        "0x1.8#2",
        "1.0",
        "0x1.0#2",
        RoundingMode::Nearest,
        "0.5",
        "0x0.8#2",
        Ordering::Equal,
    );
    // - leading_zeros == 0 in sub_float_significands_same_prec_lt_w
    test(
        "1.5",
        "0x1.8#2",
        "0.5",
        "0x0.8#2",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#2",
        Ordering::Equal,
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff & shift_bit) == 0)) in
    //   sub_float_significands_same_prec_lt_w
    test(
        "2.0",
        "0x2.0#2",
        "0.8",
        "0x0.c#2",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#2",
        Ordering::Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & shift_bit) != 0) && diff !=
    //   0 in sub_float_significands_same_prec_lt_w
    test(
        "1.5",
        "0x1.8#2",
        "0.1",
        "0x0.2#2",
        RoundingMode::Nearest,
        "1.5",
        "0x1.8#2",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH in sub_float_significands_same_prec_lt_w
    // - x <= HIGH_BIT in sub_float_significands_same_prec_lt_w
    test(
        "1.0e9",
        "0x4.0E+7#1",
        "6.0e-11",
        "0x4.0E-9#1",
        RoundingMode::Nearest,
        "1.0e9",
        "0x4.0E+7#1",
        Ordering::Greater,
    );
    // - x > HIGH_BIT in sub_float_significands_same_prec_lt_w
    test(
        "9.2047171e-27",
        "0x2.d945d78E-22#27",
        "1.43189635e33",
        "0x4.69912aE+27#27",
        RoundingMode::Nearest,
        "-1.43189635e33",
        "-0x4.69912aE+27#27",
        Ordering::Less,
    );

    // - rm == Floor || rm == Down in sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "0.2",
        "0x0.4#1",
        RoundingMode::Down,
        "0.5",
        "0x0.8#1",
        Ordering::Less,
    );
    // - rm == Ceiling || rm == Up in sub_float_significands_same_prec_lt_w
    // - (rm == Ceiling || rm == Up) && diff == 0 in sub_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "0.2",
        "0x0.4#1",
        RoundingMode::Up,
        "1.0",
        "0x1.0#1",
        Ordering::Greater,
    );
    // - (rm == Ceiling || rm == Up) && diff != 0 in sub_float_significands_same_prec_lt_w
    test(
        "2.0",
        "0x2.0#2",
        "0.8",
        "0x0.c#2",
        RoundingMode::Up,
        "1.5",
        "0x1.8#2",
        Ordering::Greater,
    );

    // - in sub_float_significands_same_prec_w
    // - x_exp == y_exp in sub_float_significands_same_prec_w
    // - x_exp == y_exp && a0 == 0 in sub_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_w
    // - x_exp < y_exp in sub_float_significands_same_prec_w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_w
    // - a0 != 0 in sub_float_significands_same_prec_w
    // - leading_zeros != 0 in sub_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "2.0",
        "0x2.0000000000000000#64",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.0000000000000000#64",
        Ordering::Equal,
    );
    // - a0 > x in sub_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        RoundingMode::Nearest,
        "-1.084202172485504434e-19",
        "-0x2.0000000000000000E-16#64",
        Ordering::Equal,
    );
    // - a0 <= x in sub_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "1.0",
        "0x1.0000000000000000#64",
        RoundingMode::Nearest,
        "1.084202172485504434e-19",
        "0x2.0000000000000000E-16#64",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_w
    // - neg in sub_float_significands_same_prec_w
    // - rm == Nearest in sub_float_significands_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & 1) != 0) in
    //   sub_float_significands_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & 1) != 0) &&
    //   !diff.overflowing_add_assign(1) in sub_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "4.0",
        "0x4.0000000000000000#64",
        RoundingMode::Nearest,
        "-3.0",
        "-0x3.0000000000000000#64",
        Ordering::Less,
    );
    // - !neg in sub_float_significands_same_prec_w
    test(
        "4.0",
        "0x4.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0000000000000000#64",
        Ordering::Greater,
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff & 1) == 0)) in
    //   sub_float_significands_same_prec_w
    test(
        "1.0000000000000000003",
        "0x1.0000000000000006#64",
        "4.0",
        "0x4.0000000000000000#64",
        RoundingMode::Nearest,
        "-2.9999999999999999996",
        "-0x2.fffffffffffffff8#64",
        Ordering::Greater,
    );
    // - leading_zeros == 0 in sub_float_significands_same_prec_w
    test(
        "3.2729513077064011786e-37",
        "0x6.f5f6d50e7b8f6eb0E-31#64",
        "7.8519772600462495573e-34",
        "0x4.13b4f0d218450fb0E-28#64",
        RoundingMode::Nearest,
        "-7.848704308738543156e-34",
        "-0x4.13459164c75d56b8E-28#64",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH in sub_float_significands_same_prec_w
    // - x > HIGH_BIT in sub_float_significands_same_prec_w
    test(
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
        "2.9347251290514630352e-45",
        "0x1.0c11b075f03d6daeE-37#64",
        RoundingMode::Nearest,
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
        Ordering::Greater,
    );
    // - x <= HIGH_BIT in sub_float_significands_same_prec_w
    // - exp_diff != Limb::WIDTH || y <= HIGH_BIT in sub_float_significands_same_prec_w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff & 1) != 0) &&
    //   diff.overflowing_add_assign(1) in sub_float_significands_same_prec_w
    test(
        "8355840.0624923708378",
        "0x7f8000.0fff8000ff8#64",
        "2.3384026197294446691e49",
        "0x1.0000000000000000E+41#64",
        RoundingMode::Nearest,
        "-2.3384026197294446691e49",
        "-0x1.0000000000000000E+41#64",
        Ordering::Less,
    );
    // - x_exp != y_exp && a0 == 0 in sub_float_significands_same_prec_w
    test(
        "63.999999999999999997",
        "0x3f.ffffffffffffffc#64",
        "64.0",
        "0x40.000000000000000#64",
        RoundingMode::Nearest,
        "-3.4694469519536141888e-18",
        "-0x4.0000000000000000E-15#64",
        Ordering::Equal,
    );
    // - exp_diff == Limb::WIDTH && y > HIGH_BIT in sub_float_significands_same_prec_w
    test(
        "4.656578456163629198e-10",
        "0x1.ffff07fffffffffeE-8#64",
        "4294967296.0",
        "0x100000000.00000000#64",
        RoundingMode::Nearest,
        "-4294967295.9999999995",
        "-0xffffffff.fffffffe#64",
        Ordering::Greater,
    );

    // - rm == Floor || rm == Down in sub_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "4.0",
        "0x4.0000000000000000#64",
        RoundingMode::Down,
        "-2.9999999999999999998",
        "-0x2.fffffffffffffffc#64",
        Ordering::Greater,
    );
    // - rm == Ceiling || rm == Up in sub_float_significands_same_prec_w
    // - (rm == Ceiling || rm == Up) && !diff.overflowing_add_assign(1) in
    //   sub_float_significands_same_prec_w
    test(
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "4.0",
        "0x4.0000000000000000#64",
        RoundingMode::Up,
        "-3.0",
        "-0x3.0000000000000000#64",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && diff.overflowing_add_assign(1) in
    //   sub_float_significands_same_prec_w
    test(
        "7.737125245533626718e25",
        "0x4.0000000000000000E+21#64",
        "3.4410713482205469792e-21",
        "0x1.03fffffffffc0000E-17#64",
        RoundingMode::Up,
        "7.737125245533626718e25",
        "0x4.0000000000000000E+21#64",
        Ordering::Greater,
    );

    // - in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp in sub_float_significands_same_prec_gt_w_lt_2w
    // - a1 == 0 && a0 == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp < y_exp in sub_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp != y_exp && a1 != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp != y_exp && leading_zeros != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "2.0",
        "0x2.0000000000000000#65",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.0000000000000000#65",
        Ordering::Equal,
    );
    // - (a1 != 0 || a0 != 0) && a1 >= x_1 in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp && a1 == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp && leading_zeros == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        RoundingMode::Nearest,
        "-5.42101086242752217e-20",
        "-0x1.0000000000000000E-16#65",
        Ordering::Equal,
    );
    // - (a1 != 0 || a0 != 0) && a1 < x_1 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "1.0",
        "0x1.0000000000000000#65",
        RoundingMode::Nearest,
        "5.42101086242752217e-20",
        "0x1.0000000000000000E-16#65",
        Ordering::Equal,
    );
    // - x_exp == y_exp && leading_zeros != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.00000000000000000#66",
        "1.00000000000000000003",
        "0x1.00000000000000008#66",
        RoundingMode::Nearest,
        "-2.710505431213761085e-20",
        "-0x8.0000000000000000E-17#66",
        Ordering::Equal,
    );
    // - x_exp == y_exp && a1 != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000011",
        "0x1.0000000000000002#65",
        RoundingMode::Nearest,
        "-1.084202172485504434e-19",
        "-0x2.0000000000000000E-16#65",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_gt_w_lt_2w
    // - neg in sub_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in sub_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   !overflow in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "4.0",
        "0x4.0000000000000000#65",
        RoundingMode::Nearest,
        "-3.0",
        "-0x3.0000000000000000#65",
        Ordering::Less,
    );
    // - !neg in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.0",
        "0x4.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0000000000000000#65",
        Ordering::Greater,
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff_0 & shift_bit) == 0)) in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        "4.0",
        "0x4.0000000000000000#65",
        RoundingMode::Nearest,
        "-2.9999999999999999998",
        "-0x2.fffffffffffffffc#65",
        Ordering::Greater,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && a1 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "18.63123034252626794758647",
        "0x12.a1984fcd64a8ae228eef#85",
        RoundingMode::Nearest,
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        Ordering::Greater,
    );
    // - x_exp != y_exp && leading_zeros == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.0001507756106295330606262754053",
        "0x0.0009e19851127b95dcf03f0cdc#91",
        "3458.565842843038054059107814",
        "0xd82.90db1399862ba513faf8#91",
        RoundingMode::Nearest,
        "-3458.565692067427424526047188",
        "-0xd82.90d132013519297e1e08#91",
        Ordering::Less,
    );
    // - exp_diff >= Limb::WIDTH * 2 in sub_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff >= Limb::WIDTH * 2 && a1 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
        "1.14823551075108882469e-96",
        "0x2.73dea72af3fe6314E-80#65",
        RoundingMode::Nearest,
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
        Ordering::Greater,
    );
    // - exp_diff == Limb::WIDTH in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "19585.2851423168986928116147584507795",
        "0x4c81.48ff163dc91a0d4bd90309b0f8#116",
        "372369974082165972902790.766638151683",
        "0x4eda377c7f0d747fa386.c44265dd58#116",
        RoundingMode::Nearest,
        "-372369974082165972883205.481495834785",
        "-0x4eda377c7f0d747f5705.7b434f9f90#116",
        Ordering::Less,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && a1 < HIGH_BIT in
    //   sub_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   overflow in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "9.9035203142830421991929938e27",
        "0x2.000000000000000000000E+23#85",
        "16.0",
        "0x10.00000000000000000000#85",
        RoundingMode::Nearest,
        "9.9035203142830421991929938e27",
        "0x2.000000000000000000000E+23#85",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH * 2 && a1 < HIGH_BIT in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "5.3455294200288159103345444e-51",
        "0x1.ffffffffc000000000000E-42#83",
        "8.0",
        "0x8.00000000000000000000#83",
        RoundingMode::Nearest,
        "-8.0",
        "-0x8.00000000000000000000#83",
        Ordering::Less,
    );
    // - x_exp != y_exp && a1 == 0 in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.00000000000000000000084702",
        "0x4.000000000000000003fffc#89",
        "3.999999999999999999999999994",
        "0x3.fffffffffffffffffffffe#89",
        RoundingMode::Nearest,
        "8.47026484905764768539612568e-22",
        "0x3.fffe000000000000000000E-18#89",
        Ordering::Equal,
    );

    // - rm == Floor || rm == Down in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "4.0",
        "0x4.0000000000000000#65",
        RoundingMode::Down,
        "-2.9999999999999999999",
        "-0x2.fffffffffffffffe#65",
        Ordering::Greater,
    );
    // - rm == Ceiling || rm == Up in sub_float_significands_same_prec_gt_w_lt_2w
    // - (rm == Ceiling || rm == Up) && !overflow in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "4.0",
        "0x4.0000000000000000#65",
        RoundingMode::Up,
        "-3.0",
        "-0x3.0000000000000000#65",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && overflow in sub_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.5986723171183083380727715984266450731e-36",
        "0x2.1fffffffffff00000000000000ffff8E-30#123",
        "2475880078570760549798248448.0",
        "0x80000000000000000000000.00000000#123",
        RoundingMode::Up,
        "-2475880078570760549798248448.0",
        "-0x80000000000000000000000.00000000#123",
        Ordering::Less,
    );

    // - in sub_float_significands_same_prec_2w
    // - x_exp == y_exp in sub_float_significands_same_prec_2w
    // - a1 == 0 && a0 == 0 in sub_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_2w
    // - x_exp < y_exp in sub_float_significands_same_prec_2w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_2w
    // - a1 != 0 second time in sub_float_significands_same_prec_2w
    // - a1 != 0 third time in sub_float_significands_same_prec_2w
    // - x_exp != y_exp && leading_zeros != 0 in sub_float_significands_same_prec_2w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.00000000000000000000000000000000#128",
        Ordering::Equal,
    );
    // - (a1 != 0 || a0 != 0) && a1 >= x_1 in sub_float_significands_same_prec_2w
    // - a1 == 0 first time in sub_float_significands_same_prec_2w
    // - x_exp == y_exp && leading_zeros != 0 in sub_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        RoundingMode::Nearest,
        "-5.87747175411143753984368268611122838909e-39",
        "-0x2.00000000000000000000000000000000E-32#128",
        Ordering::Equal,
    );
    // - (a1 != 0 || a0 != 0) && a1 < x_1 in sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "5.87747175411143753984368268611122838909e-39",
        "0x2.00000000000000000000000000000000E-32#128",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_2w
    // - neg in sub_float_significands_same_prec_2w
    // - rm == Nearest in sub_float_significands_same_prec_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & 1) != 0) && !overflow in
    //   sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "-3.0",
        "-0x3.00000000000000000000000000000000#128",
        Ordering::Less,
    );
    // - !neg in sub_float_significands_same_prec_2w
    test(
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00000000000000000000000000000000#128",
        Ordering::Greater,
    );
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (diff_0 & 1) == 0)) in
    //   sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000018",
        "0x1.00000000000000000000000000000006#128",
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "-2.99999999999999999999999999999999999998",
        "-0x2.fffffffffffffffffffffffffffffff8#128",
        Ordering::Greater,
    );
    // - x_exp != y_exp && leading_zeros == 0 in sub_float_significands_same_prec_2w
    test(
        "1.91698663575347889601435178329077738407e-37",
        "0x4.13b4f0d218450fb6f5f6d50e7b8f6eb0E-31#128",
        "8.0669110092962644724944820639408319804e-34",
        "0x4.3046c1d7c0f5b6be39df2b03f3e49a70E-28#128",
        RoundingMode::Nearest,
        "-8.06499402266051099359846771215754120301e-34",
        "-0x4.30058688b3d4326d3e6fcb96a2fce178E-28#128",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH * 2 in sub_float_significands_same_prec_2w
    // - x_1 > HIGH_BIT || x_0 > 0 in sub_float_significands_same_prec_2w
    test(
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        "5.07801249136957145270807726205511855421e-45",
        "0x1.cfd8608b7c32de2bbcfecf8bcf8a2d00E-37#128",
        RoundingMode::Nearest,
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        Ordering::Greater,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in sub_float_significands_same_prec_2w
    // - a1 >= HIGH_BIT in sub_float_significands_same_prec_2w
    test(
        "4354249796990942.35435357526597783143164",
        "0xf782ac869b7de.5ab6ea78fcf0cc5079f#128",
        "8.03239453825726512240307053405256016022e-10",
        "0x3.732bce7aa121827a284545a25f32dc68E-8#128",
        RoundingMode::Nearest,
        "4354249796990942.35435357446273837760591",
        "0xf782ac869b7de.5ab6ea7589c4fdd5d8d#128",
        Ordering::Greater,
    );
    // - a1 != 0 first time in sub_float_significands_same_prec_2w
    test(
        "852777855057.113455599443829872557360137",
        "0xc68d856851.1d0b6d1928c98779f28bdd#128",
        "869223500084.503694559376384046083491558",
        "0xca61c20934.80f2206bb1d7d5cad69caa#128",
        RoundingMode::Nearest,
        "-16445645027.39023895993255417352613142066",
        "-0x3d43ca0e3.63e6b352890e4e50e410cd00#128",
        Ordering::Equal,
    );
    // - exp_diff == Limb::WIDTH in sub_float_significands_same_prec_2w
    test(
        "1.057437459917463716438672572710788562701e-17",
        "0xc.310127aae1df1a1cb12f60c4d339d76E-15#128",
        "148.0549133677002965445211858794413066474",
        "0x94.0e0ecd6e62d0a8c7c7c2a633277e3e#128",
        RoundingMode::Nearest,
        "-148.054913367700296533946811280266669483",
        "-0x94.0e0ecd6e62d0a804b7b02b85098c9c#128",
        Ordering::Greater,
    );
    // - x_1 <= HIGH_BIT && x_0 <= 0 in sub_float_significands_same_prec_2w
    // - exp_diff != TWICE_WIDTH || tst in sub_float_significands_same_prec_2w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & 1) != 0) && overflow in
    //   sub_float_significands_same_prec_2w
    test(
        "3.83123885216472214589586756787577295905e53",
        "0x4.00000000000000000000000000000000E+44#128",
        "9.09494703466994132423639742106012939545e-13",
        "0x1.00000008000000000000007fffffff80E-10#128",
        RoundingMode::Nearest,
        "3.83123885216472214589586756787577295905e53",
        "0x4.00000000000000000000000000000000E+44#128",
        Ordering::Greater,
    );
    // - a1 < HIGH_BIT in sub_float_significands_same_prec_2w
    test(
        "0.000473141670219947088192358501321236008969",
        "0x0.001f01fffffe001ffe000000000003ffff0#128",
        "144115188075855872.000000000007275846592",
        "0x200000000000000.0000000007fff80000#128",
        RoundingMode::Nearest,
        "-144115188075855871.9995268583370558995037",
        "-0x1ffffffffffffff.ffe0fe000801f7e002#128",
        Ordering::Less,
    );
    // - a1 == 0 second  time in sub_float_significands_same_prec_2w
    test(
        "1.192092895507812500000000000000000918348e-7",
        "0x2.0000000000000000000000000007fffcE-6#128",
        "1.19209289550781249999999999999999632658e-7",
        "0x1.ffffffffffffffffffffffffffe00000E-6#128",
        RoundingMode::Nearest,
        "4.5917678014072389539175224798764807294e-40",
        "0x2.7fffc000000000000000000000000000E-33#128",
        Ordering::Equal,
    );
    // - exp_diff == TWICE_WIDTH && !tst in sub_float_significands_same_prec_2w
    test(
        "1024.0",
        "0x400.000000000000000000000000000000#128",
        "5.97151130219911582898625687582100437209e-36",
        "0x7.f00000001fffffffffffffffe0000000E-30#128",
        RoundingMode::Nearest,
        "1023.999999999999999999999999999999999994",
        "0x3ff.fffffffffffffffffffffffffffff8#128",
        Ordering::Less,
    );
    // - x_exp == y_exp && leading_zeros == 0 in sub_float_significands_same_prec_2w
    test(
        "0.249999999999999999999999999974756451033",
        "0x0.3ffffffffffffffffffffffe000000000#128",
        "0.2499999999999999999864474728691747435412",
        "0x0.3fffffffffffffffc0000001ffffffffc#128",
        RoundingMode::Nearest,
        "1.35525271055817074916830884337875729072e-20",
        "0x3.ffffffc0000000040000000000000000E-17#128",
        Ordering::Equal,
    );
    // - a1 == 0 third time in sub_float_significands_same_prec_2w
    test(
        "8.0",
        "0x8.0000000000000000000000000000000#128",
        "7.99999999999999999999999999999999999998",
        "0x7.fffffffffffffffffffffffffffffff8#128",
        RoundingMode::Nearest,
        "2.35098870164457501593747307444449135564e-38",
        "0x8.0000000000000000000000000000000E-32#128",
        Ordering::Equal,
    );

    // - rm == Floor || rm == Down in sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        RoundingMode::Down,
        "-2.99999999999999999999999999999999999999",
        "-0x2.fffffffffffffffffffffffffffffffc#128",
        Ordering::Greater,
    );
    // - rm == Ceiling || rm == Up in sub_float_significands_same_prec_2w
    // - (rm == Ceiling || rm == Up) && !overflow in sub_float_significands_same_prec_2w
    test(
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "4.0",
        "0x4.00000000000000000000000000000000#128",
        RoundingMode::Up,
        "-3.0",
        "-0x3.00000000000000000000000000000000#128",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && overflow in sub_float_significands_same_prec_2w
    test(
        "77371252455336267181195264.0",
        "0x4000000000000000000000.00000000000#128",
        "4.93038066207960562897601098365736701192e-32",
        "0x1.00000003e00000000001ff000000001eE-26#128",
        RoundingMode::Up,
        "77371252455336267181195264.0",
        "0x4000000000000000000000.00000000000#128",
        Ordering::Greater,
    );

    // - rm == Floor || rm == Down in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        RoundingMode::Down,
        "-2.999999999999999999999999999999999999994",
        "-0x2.fffffffffffffffffffffffffffffffe#129",
        Ordering::Greater,
    );
    // - rm == Ceiling || rm == Up in sub_float_significands_same_prec_gt_2w_lt_3w
    // - (rm == Ceiling || rm == Up) && overflow in sub_float_significands_same_prec_gt_2w_lt_3w
    // - (rm == Ceiling || rm == Up) && diff_1 == 0 && diff_0 == 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - (rm == Ceiling || rm == Up) && diff_2 != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        RoundingMode::Up,
        "-3.0",
        "-0x3.00000000000000000000000000000000#129",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && !overflow in sub_float_significands_same_prec_gt_2w_lt_3w
    // - (rm == Ceiling || rm == Up) && (diff_1 != 0 || diff_0 != 0) in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        RoundingMode::Up,
        "-2.999999999999999999999999999999999999994",
        "-0x2.fffffffffffffffffffffffffffffffe#129",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && diff_2 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "3.982729777831130692572200994412995874342180298e-59",
        "0x3.ffffffffffffffffffffffff800000000000000E-49#155",
        "9.31322574615478515625e-10",
        "0x4.00000000000000000000000000000000000000E-8#155",
        RoundingMode::Floor,
        "-9.31322574615478515625e-10",
        "-0x4.00000000000000000000000000000000000000E-8#155",
        Ordering::Less,
    );

    // - in sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp in sub_float_significands_same_prec_gt_2w_lt_3w
    // - a2 == 0 && a1 == 0 && a0 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp != y_exp in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && sticky_bit == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && a2 != 0 first time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && leading_zeros != 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.00000000000000000000000000000000#129",
        Ordering::Equal,
    );
    // - x_exp >= y_exp in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        Ordering::Equal,
    );
    // - (a2 != 0 || a1 != 0 || a0 != 0) && a2 >= x_2 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp && a2 == 0 first time in sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp && a2 == 0 second time in sub_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp == y_exp && leading_zeros == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        RoundingMode::Nearest,
        "-2.93873587705571876992184134305561419455e-39",
        "-0x1.00000000000000000000000000000000E-32#129",
        Ordering::Equal,
    );
    // - (a2 != 0 || a1 != 0 || a0 != 0) && a2 < x_2 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "2.93873587705571876992184134305561419455e-39",
        "0x1.00000000000000000000000000000000E-32#129",
        Ordering::Equal,
    );
    // - x_exp == y_exp && leading_zeros != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000#130",
        "1.000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000008#130",
        RoundingMode::Nearest,
        "-1.469367938527859384960920671527807097273e-39",
        "-0x8.00000000000000000000000000000000E-33#130",
        Ordering::Equal,
    );
    // - x_exp == y_exp && a2 != 0 second time in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#129",
        RoundingMode::Nearest,
        "-5.87747175411143753984368268611122838909e-39",
        "-0x2.00000000000000000000000000000000E-32#129",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    // - neg in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   overflow in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) && diff_1
    //   == 0 && diff_0 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && diff_2 != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "-3.0",
        "-0x3.00000000000000000000000000000000#129",
        Ordering::Less,
    );
    // - !neg in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00000000000000000000000000000000#129",
        Ordering::Greater,
    );
    // - rm == Nearest && round_bit == 0 || (sticky_bit == 0 && (diff_0 & shift_bit) == 0) in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        "4.0",
        "0x4.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "-2.999999999999999999999999999999999999988",
        "-0x2.fffffffffffffffffffffffffffffffc#129",
        Ordering::Greater,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   (diff_1 != 0 || diff_0 != 0) in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.000000000000000000000000000000000000006",
        "0x2.00000000000000000000000000000002#129",
        "0.500000000000000000000000000000000000001",
        "0x0.800000000000000000000000000000008#129",
        RoundingMode::Nearest,
        "1.500000000000000000000000000000000000006",
        "0x1.80000000000000000000000000000002#129",
        Ordering::Greater,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (diff_0 & shift_bit) != 0) &&
    //   !overflow in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.000000000000000000000000000000000000003",
        "0x2.00000000000000000000000000000001#130",
        "0.5000000000000000000000000000000000000007",
        "0x0.800000000000000000000000000000004#130",
        RoundingMode::Nearest,
        "1.500000000000000000000000000000000000003",
        "0x1.800000000000000000000000000000010#130",
        Ordering::Greater,
    );
    // - TWICE_WIDTH <= exp_diff < THRICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - TWICE_WIDTH < exp_diff < THRICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - TWICE_WIDTH <= exp_diff < THRICE_WIDTH && a2 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        RoundingMode::Nearest,
        "-245906107849378561117126906.9059035528266331265",
        "-0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        Ordering::Less,
    );
    // - exp_diff >= THRICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff >= THRICE_WIDTH && a2 >= HIGH_BIT in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        "6.8892360159362421595728818935378487832685754059e-50",
        "0x1.9c693c182df3035eef00d41638bbdd942f4d498E-41#155",
        RoundingMode::Nearest,
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        Ordering::Greater,
    );
    // - exp_diff < Limb::WIDTH && leading_zeros == 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.9709672065181108960570410290811793724062284431352e-48",
        "0x7.43dc113e95ca123693650af31435eac45c0e7a680E-40#165",
        "5.2183974782595301717751266872943662193587933931613e-47",
        "0x4.c4457ca8b3429511981a96eb0c2de4fdb8c43bea4E-39#165",
        RoundingMode::Nearest,
        "-4.7213007576077190821694225843862482821181705488478e-47",
        "-0x4.5007bb94c9e5f3ee2ee4463bdaea865173035443cE-39#165",
        Ordering::Equal,
    );
    // - exp_diff < Limb::WIDTH && sticky_bit != 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "8.264811372870109665580646414654919646529224699888394e-20",
        "0x1.864b7049feb3dcfe49ea910db778157cbe9c2021b44E-16#171",
        "0.0007337187065343299500100945131574173571435306249470578",
        "0x0.003015c1d959ec54ab97dc58b77c22566586c06119b810#171",
        RoundingMode::Nearest,
        "-0.0007337187065343298673619807844563207013370664783978612",
        "-0x0.003015c1d959ec53254c6c0eb8c845581b9c2f53623ff8#171",
        Ordering::Greater,
    );
    // - Limb::WIDTH <= exp_diff < TWICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && sticky_bit != 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && a2 >= HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.2850537238606374652351877988811796373898773e-22",
        "0x2.0607fd4819748c532aad3528693c1e3c1966E-18#146",
        "978.49328809934495391839880801989439981236569",
        "0x3d2.7e4820fe314caadcb9a156bef2f1c8e53c#146",
        RoundingMode::Nearest,
        "-978.49328809934495391839837951452201374861917",
        "-0x3d2.7e4820fe314caadcb79b4ec1aad85458e9#146",
        Ordering::Less,
    );
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && sticky_bit == 0 in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "0.4575080253178526499352273198671695352442",
        "0x0.751f3ef665d0ca4dfa2d089979c4e9600#130",
        "184366716174337778394.1535133791267987587",
        "0x9fe9a278b38ab22da.274ca71ed919c918#130",
        RoundingMode::Nearest,
        "-184366716174337778393.6960053538089461089",
        "-0x9fe9a278b38ab22d9.b22d68287348fecc#130",
        Ordering::Less,
    );
    // - x_exp == y_exp && a2 != 0 first time in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "229.1456159244630209666077998586332154002628588961962846344",
        "0xe5.254715d158717849f7198986a38cb415eeea3464b1df38#189",
        "175.1335582002789888688278442018847623084238889142385801004",
        "0xaf.2230dec64f958583522e37252cf610378914f3127d0bb0#189",
        RoundingMode::Nearest,
        "54.01205772418403209777995565674845309183896998195770453405",
        "0x36.0316370b08dbf2c6a4eb52617696a3de65d5415234d388#189",
        Ordering::Equal,
    );
    // - exp_diff == TWICE_WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "563971925627753843356041629019151473018178607215.42",
        "0x62c960337e963a378ba6626ea422d8a5e623986f.6c#165",
        "1301375421.83361702516620516356439489325145225661938",
        "0x4d9169bd.d567ece47a47ef60371d48c969ba8765d4#165",
        RoundingMode::Nearest,
        "563971925627753843356041629019151473016877231793.59",
        "0x62c960337e963a378ba6626ea422d8a598922eb1.98#165",
        Ordering::Greater,
    );
    // - exp_diff == Limb::WIDTH in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "226.9305090753243797994707628568605406194",
        "0xe2.ee35d7bf263fda8c632644ad7c49d98#130",
        "4262448175090788889452.984188256984861391",
        "0xe71159efd3a67e736c.fbf3c2f8db72fb8#130",
        RoundingMode::Nearest,
        "-4262448175090788889226.053679181660481592",
        "-0xe71159efd3a67e728a.0dbdeb39b533210#130",
        Ordering::Less,
    );
    // - TWICE_WIDTH <= exp_diff < THRICE_WIDTH && a2 < HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - rm == Nearest && diff_2 == 0 in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "0.0001220703125",
        "0x0.0008000000000000000000000000000000000000#145",
        "8.2855746158774225568154012162196749515653053e-50",
        "0x1.f0000fc000000000000003fffffffffffffcE-41#145",
        RoundingMode::Nearest,
        "0.0001220703125",
        "0x0.0008000000000000000000000000000000000000#145",
        Ordering::Greater,
    );
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH && a2 < HIGH_BIT in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "0.0156250000000000000000000000252435487789932816416198",
        "0x0.040000000000000000000001ffffffc00000000003f#167",
        "2.11758236813575084767080625169910490512847748461541e-22",
        "0xf.ffffffffffffffffffffffffffffffffff0000000E-19#167",
        RoundingMode::Nearest,
        "0.0156249999999999999997882417884299736942262010164499",
        "0x0.03ffffffffffffffff000001ffffffc00000000003f0#167",
        Ordering::Less,
    );
    // - exp_diff >= THRICE_WIDTH && a2 < HIGH_BIT in sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.028440348325753776346855739098344065614209916020987e62",
        "0x4.000000000000000000000000000000000000000000E+51#168",
        "0.00781238079073842683897073489057921223679527623088422",
        "0x0.01fffe000007e000000038000007fffffffffe000000#168",
        RoundingMode::Nearest,
        "1.028440348325753776346855739098344065614209916020987e62",
        "0x4.000000000000000000000000000000000000000000E+51#168",
        Ordering::Greater,
    );
    // - exp_diff < Limb::WIDTH && a2 == 0 first time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && a2 != 0 second time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "32767.9999999999999999999999999999999999999999996",
        "0x7fff.fffffffffffffffffffffffffffffffffff8#156",
        "32768.0000000000000000069388939039072268369037424",
        "0x8000.000000000000007ffffffffffff80000000#156",
        RoundingMode::Nearest,
        "-6.93889390390722683690374277451135137549581608853e-18",
        "-0x7.ffffffffffff800000008000000000000000000E-15#156",
        Ordering::Equal,
    );
    // - exp_diff < Limb::WIDTH && a2 == 0 second time in
    //   sub_float_significands_same_prec_gt_2w_lt_3w
    test(
        "137438953471.99999999999999999999999999999",
        "0x1fffffffff.ffffffffffffffffffffffff#133",
        "137438953472.0",
        "0x2000000000.000000000000000000000000#133",
        RoundingMode::Nearest,
        "-1.2621774483536188886587657044524579674771e-29",
        "-0x1.000000000000000000000000000000000E-24#133",
        Ordering::Equal,
    );

    // - in sub_float_significands_same_prec_ge_3w
    // - x_exp == y_exp in sub_float_significands_same_prec_ge_3w
    // - k < 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp > y_exp in sub_float_significands_same_prec_ge_3w
    // - exp_diff == 1 in sub_float_significands_same_prec_ge_3w
    // - !goto_sub_d1_no_lose && !goto_sub_d1_lose in sub_float_significands_same_prec_ge_3w
    // - limb < HIGH_BIT in sub_float_significands_same_prec_ge_3w
    // - exp_diff == 0 in sub_float_significands_same_prec_ge_3w
    // - goto_exact_normalize in sub_float_significands_same_prec_ge_3w
    // - limb != 0 in sub_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && limb != 0 && leading_zeros == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - k >= 0 && xs[k] < ys[k] in sub_float_significands_same_prec_ge_3w
    // - !goto_exact_normalize in sub_float_significands_same_prec_ge_3w
    // - limb == 0 in sub_float_significands_same_prec_ge_3w
    // - out[usize::exact_from(k)] != 0 in sub_float_significands_same_prec_ge_3w
    // - limb == 0 && leading_zeros != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Nearest,
        "-3.186183822264904554057760795535423611182209110385237572148e-58",
        "-0x2.000000000000000000000000000000000000000000000000E-48#192",
        Ordering::Equal,
    );
    // - k >= 0 && xs[k] >= ys[k] in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "3.186183822264904554057760795535423611182209110385237572148e-58",
        "0x2.000000000000000000000000000000000000000000000000E-48#192",
        Ordering::Equal,
    );
    // - exp_diff == 0 && limb != 0 && leading_zeros != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Nearest,
        "0.9999999999999999999999999999999999999999999999999999999997",
        "0x0.fffffffffffffffffffffffffffffffffffffffffffffffe#192",
        Ordering::Equal,
    );
    // - 2 <= exp_diff < prec in sub_float_significands_same_prec_ge_3w
    // - dm != 0 && m == 0 in sub_float_significands_same_prec_ge_3w
    // - sx != 0 in sub_float_significands_same_prec_ge_3w
    // - !out[n - 1].get_highest_bit() in sub_float_significands_same_prec_ge_3w
    // - round_bit == 0 in sub_float_significands_same_prec_ge_3w
    // - round_bit == 0 && sticky_bit == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "0.5",
        "0x0.800000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "1.5",
        "0x1.800000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - limb == 0 && leading_zeros == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#193",
        "1.0000000000000000000000000000000000000000000000000000000002",
        "0x1.000000000000000000000000000000000000000000000001#193",
        RoundingMode::Nearest,
        "-1.5930919111324522770288803977677118055911045551926187860739e-58",
        "-0x1.000000000000000000000000000000000000000000000000E-48#193",
        Ordering::Equal,
    );
    // - sx == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#193",
        "0.5",
        "0x0.8000000000000000000000000000000000000000000000000#193",
        RoundingMode::Nearest,
        "1.5",
        "0x1.800000000000000000000000000000000000000000000000#193",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in sub_float_significands_same_prec_ge_3w
    // - out_power_of_2 || round_bit == 0 in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest && !out_power_of_2 first time in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest && (round_bit == 0 || (round_bit != 0 && sticky_bit == 0 && (out[0] &
    //   shift_bit == 0 || prec == 1))) in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "-3.0",
        "-0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Less,
    );
    // - out[usize::exact_from(k)] == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000002",
        "0x1.000000000000000000000000000000000000000000000001#193",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#193",
        RoundingMode::Nearest,
        "-1.5930919111324522770288803977677118055911045551926187860739e-58",
        "-0x1.000000000000000000000000000000000000000000000000E-48#193",
        Ordering::Equal,
    );
    // - round_bit != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000006",
        "0x1.000000000000000000000000000000000000000000000004#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "-2.9999999999999999999999999999999999999999999999999999999994",
        "-0x2.fffffffffffffffffffffffffffffffffffffffffffffffc#192",
        Ordering::Equal,
    );
    // - rm == Nearest && round_bit != 0 && (round_bit == 0 || sticky_bit != 0 || (out[0] &
    //   shift_bit != 0 && prec == 1)) in sub_float_significands_same_prec_ge_3w
    // - rm == Nearest && !out_power_of_2 second time in sub_float_significands_same_prec_ge_3w
    test(
        "1.000000000000000000000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000000000000000000006#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "-2.9999999999999999999999999999999999999999999999999999999987",
        "-0x2.fffffffffffffffffffffffffffffffffffffffffffffff8#192",
        Ordering::Greater,
    );
    // - sticky_bit_2 == 0 && k > 0 third time in sub_float_significands_same_prec_ge_3w
    test(
        "0.5",
        "0x0.8000000000000000000000000000000000000000000000000#193",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#193",
        RoundingMode::Nearest,
        "-3.5",
        "-0x3.800000000000000000000000000000000000000000000000#193",
        Ordering::Equal,
    );
    // - sticky_bit_2 != 0 || k <= 0 third time in sub_float_significands_same_prec_ge_3w
    test(
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        "0.5000000000000000000000000000000000000000000000000000000002",
        "0x0.800000000000000000000000000000000000000000000001#192",
        RoundingMode::Nearest,
        "3.5",
        "0x3.800000000000000000000000000000000000000000000000#192",
        Ordering::Greater,
    );
    // - dm != 0 && m != 0 in sub_float_significands_same_prec_ge_3w
    // - out[n - 1].get_highest_bit() in sub_float_significands_same_prec_ge_3w
    test(
        "7.28057116938384227432903448367767196428679514765398378973101e-48",
        "0xa.a3fc2da1f20fb2d9771f86d3c16a444cd62d5d139e3935f24E-40#198",
        "3.5123473778825578958968695187657587760357139395948269588971e-27",
        "0x1.1646de419a6dbd3466f3081403a87d719b7a765a1ec69e4658E-22#198",
        RoundingMode::Nearest,
        "-3.51234737788255789588958894759637493376138490511114928693281e-27",
        "-0x1.1646de419a6dbd345c4f0be661b66dbec20356d34b05340208E-22#198",
        Ordering::Greater,
    );
    // - exp_diff >= prec in sub_float_significands_same_prec_ge_3w
    // - exp_diff > prec in sub_float_significands_same_prec_ge_3w
    // - exp_diff != prec + 1 in sub_float_significands_same_prec_ge_3w
    test(
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        "0.023991386767031193042066748710708351501952890752924613005724",
        "0x0.06244cad8cd272134e34b325815ad281733f2c06231a0ee744#193",
        RoundingMode::Nearest,
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        Ordering::Greater,
    );
    // - dm == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "6.442552350746554109885349691592991892989624685631192235549e-6",
        "0x0.00006c168d38e231899f0fc85d1888549d5177bdceaee72e15060#192",
        "1476808010161862576835936576709144.7975622615653024045505082",
        "0x48cff00a780a50d34bb694ada218.cc2d0a55f25f9f9126258#192",
        RoundingMode::Nearest,
        "-1476808010161862576835936576709144.7975558190129516579963983",
        "-0x48cff00a780a50d34bb694ada218.cc2c9e3f6526bd5f9c868#192",
        Ordering::Less,
    );
    // - exp_diff == prec + 1 in sub_float_significands_same_prec_ge_3w
    // - sticky_bit_2 != 0 || k <= 0 second time in sub_float_significands_same_prec_ge_3w
    test(
        "29249291732025621624535078208.59212499364958152526111994335",
        "0x5e827271f9e9d261e7cb5540.979580eade814aae28ae9d3c8#192",
        "1.7056859397843570965021420438616279890690624515282312011749e-30",
        "0x2.2986d80d04731f28b49380410e3f4711dc2cc5f113c594a0E-25#192",
        RoundingMode::Nearest,
        "29249291732025621624535078208.59212499364958152526111994335",
        "0x5e827271f9e9d261e7cb5540.979580eade814aae28ae9d3c8#192",
        Ordering::Greater,
    );
    // - limb > HIGH_BIT in sub_float_significands_same_prec_ge_3w
    // - y0 != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "1958139908729.1847354007541959640287427302874567071533816044",
        "0x1c7ea3bd279.2f4ad1b8a7190307771c7e5767590237208b90#192",
        "688604646855.2266320591592881661509057171162297144871538944",
        "0xa054090dc7.3a048f025074a63da83a500a235d2b8fd9766d#192",
        RoundingMode::Nearest,
        "1269535261873.9581033415949077978778370131712269926662277102",
        "0x1279632c4b1.f54642b656a45cc9cee22e4d43fbd6a7471524#192",
        Ordering::Greater,
    );
    // - y0 == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "2473299914875391681216.10395653853096422269749051583697615",
        "0x8613ee70797f97a2c0.1a9ce54d32239153edab6ff15dad1c0#193",
        "7716446651886500012256.87778108230244207876352217543002925",
        "0x1a24f3592677b2760e0.e0b642d189564d3b4c797ad9c9cde8#193",
        RoundingMode::Nearest,
        "-5243146737011108331040.7738245437714778560660316595930531",
        "-0x11c3b4721edfb8fbe20.c6195d845732bbe75ece0ae86c20cc#193",
        Ordering::Equal,
    );
    // - exp_diff == prec in sub_float_significands_same_prec_ge_3w
    // - sticky_bit_2 != 0 || k <= 0 first time in sub_float_significands_same_prec_ge_3w
    test(
        "4.0635838402455207229400698207668893925379768151364313942222e-23",
        "0x3.1202ecf10ff40b477337957dede18bd7b746884ec977474eE-19#194",
        "1174582238252884689829665592721065057.76655867827770290150723",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d4#194",
        RoundingMode::Nearest,
        "-1174582238252884689829665592721065057.76655867827770290150718",
        "-0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d0#194",
        Ordering::Greater,
    );
    // - rm == Nearest && out_power_of_2 first time in sub_float_significands_same_prec_ge_3w
    test(
        "22300745198530623141535718272648361505980416.0",
        "0x1000000000000000000000000000000000000.000000000000#192",
        "8.470329472543003387009805160583577341645369940072346001613e-22",
        "0x3.ffffffffffffffe000000003ff0000003ffffffc00000000E-18#192",
        RoundingMode::Nearest,
        "22300745198530623141535718272648361505980416.0",
        "0x1000000000000000000000000000000000000.000000000000#192",
        Ordering::Greater,
    );
    // - sticky_bit_2 == 0 && k > 0 second time in sub_float_significands_same_prec_ge_3w
    test(
        "1.6849966666969146159452711670928107852024276704905067469395e66",
        "0xf.ffffffffffff01fffff00000000000000000000000000000E+54#193",
        "33554432.00000000000000044408920985006261616945266723632812",
        "0x2000000.0000000000001ffffffffffffffffffffffffffffe#193",
        RoundingMode::Nearest,
        "1.6849966666969146159452711670928107852024276704905067469395e66",
        "0xf.ffffffffffff01fffff00000000000000000000000000000E+54#193",
        Ordering::Greater,
    );
    // - limb == HIGH_BIT in sub_float_significands_same_prec_ge_3w
    // - l >= 0 first time in sub_float_significands_same_prec_ge_3w
    // - xs[l] != yl_shifted in sub_float_significands_same_prec_ge_3w
    // - l >= 0 && xs[l] <= yl_shifted in sub_float_significands_same_prec_ge_3w
    // - goto_sub_d1_no_lose || goto_sub_d1_lose in sub_float_significands_same_prec_ge_3w
    test(
        "2047.9999999999999999999999999999747564510329303127198868805",
        "0x7ff.fffffffffffffffffffffffe00000000003c000003fffc#193",
        "4095.99999999999999988897769753748434595763683319091796875",
        "0xfff.fffffffffffff800000000000000000000000000000000#193",
        RoundingMode::Nearest,
        "-2047.9999999999999998889776975375095895066039028781980818695",
        "-0x7ff.fffffffffffff80000000001ffffffffffc3fffffc0004#193",
        Ordering::Equal,
    );
    // - sticky_bit_2 == 0 && k > 0 first time in sub_float_significands_same_prec_ge_3w
    test(
        "0.0002442598197376355528831482438462803411504065058668775410457",
        "0x0.001001fff000000000000003ffff00000001fffffff07fffffe#192",
        "5.834076822994820350447560050418866475553361427251759468222e-62",
        "0x1.8000000000000000000000000000000000001c00007ffffeE-51#192",
        RoundingMode::Nearest,
        "0.00024425981973763555288314824384628034115040650586687754104564",
        "0x0.001001fff000000000000003ffff00000001fffffff07fffffc#192",
        Ordering::Less,
    );
    // - l >= 0 && xs[l] > yl_shifted in sub_float_significands_same_prec_ge_3w
    test(
        "127.99999999999999999999998841947063540272586186468301133273",
        "0x7f.fffffffffffffffffff1fffff00007ffffffffffffffff8#192",
        "255.99999999999999999999999979320484686174308128214782699291",
        "0xff.ffffffffffffffffffffc00000000000000000000000ff#192",
        RoundingMode::Nearest,
        "-128.0000000000000000000000113737342114590172194174648156602",
        "-0x80.0000000000000000000dc0000ffff80000000000000100#192",
        Ordering::Less,
    );
    // - out_power_of_2 && round_bit != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "81129638414606681695789005144064.0",
        "0x400000000000000000000000000.0000000000000000000000#193",
        "6.46392625738094777466974989455420015132331189664152496326e-27",
        "0x2.001ffffffffe0000001fffff000000ffffff80000000000eE-22#193",
        RoundingMode::Nearest,
        "81129638414606681695789005144063.999999999999999999999999994",
        "0x3ffffffffffffffffffffffffff.fffffffffffffffffffffe#193",
        Ordering::Greater,
    );
    // - rm == Nearest && out_power_of_2 second time in sub_float_significands_same_prec_ge_3w
    test(
        "9.5367431640625e-7",
        "0x0.00001000000000000000000000000000000000000000000000000#192",
        "7.596455102175746880546879414772134233793171691679642324806e-65",
        "0x8.00000ffffffffffe000000000000000000000000001ffffE-54#192",
        RoundingMode::Nearest,
        "9.536743164062499999999999999999999999999999999999999999998e-7",
        "0xf.fffffffffffffffffffffffffffffffffffffffffffffffE-6#192",
        Ordering::Less,
    );
    // - xs[l] == yl_shifted in sub_float_significands_same_prec_ge_3w
    test(
        "1180591620717411303423.9999999999999999999999999999999999998",
        "0x3fffffffffffffffff.ffffffffffffffffffffffffffffffc#192",
        "2361183241434822606847.9999999999999999930619531290400259223",
        "0x7fffffffffffffffff.ffffffffffffff8003ffffffffffff8#192",
        RoundingMode::Nearest,
        "-1180591620717411303423.9999999999999999930619531290400259225",
        "-0x3fffffffffffffffff.ffffffffffffff8003ffffffffffffc#192",
        Ordering::Equal,
    );
    // - l < 0 first time in sub_float_significands_same_prec_ge_3w
    // - l < 0 second time in sub_float_significands_same_prec_ge_3w
    // - yl_shifted != 0 in sub_float_significands_same_prec_ge_3w
    test(
        "32767.999999999999999999999999999999999999999999999999999995",
        "0x7fff.ffffffffffffffffffffffffffffffffffffffffffff8#192",
        "65535.99999999999999999999999999999999999999999999999999999",
        "0xffff.ffffffffffffffffffffffffffffffffffffffffffff#192",
        RoundingMode::Nearest,
        "-32767.999999999999999999999999999999999999999999999999999995",
        "-0x7fff.ffffffffffffffffffffffffffffffffffffffffffff8#192",
        Ordering::Equal,
    );
    // - yl_shifted == 0 in sub_float_significands_same_prec_ge_3w
    test(
        "34359738367.999999999999999999999999999999999999999999999989",
        "0x7ffffffff.fffffffffffffffffffffffffffffffffffffff0#192",
        "68719476735.99999999999999999999999999999999999999999999999",
        "0xfffffffff.fffffffffffffffffffffffffffffffffffffff#192",
        RoundingMode::Nearest,
        "-34359738368.0",
        "-0x800000000.000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );

    // - neg in sub_float_significands_same_prec_ge_3w
    // - rm == Down || rm == Floor in sub_float_significands_same_prec_ge_3w
    // - (rm == Down || rm == Floor) && !out_power_of_2 in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Down,
        "-2.9999999999999999999999999999999999999999999999999999999994",
        "-0x2.fffffffffffffffffffffffffffffffffffffffffffffffc#192",
        Ordering::Greater,
    );
    // - !neg in sub_float_significands_same_prec_ge_3w
    test(
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Down,
        "2.9999999999999999999999999999999999999999999999999999999994",
        "0x2.fffffffffffffffffffffffffffffffffffffffffffffffc#192",
        Ordering::Less,
    );
    // - rm == Up || rm == Ceiling in sub_float_significands_same_prec_ge_3w
    test(
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "4.0",
        "0x4.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Up,
        "-3.0",
        "-0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Less,
    );
    // - (rm == Down || rm == Floor) && out_power_of_2 in sub_float_significands_same_prec_ge_3w
    test(
        "5.392603845001725202291044579550746038551121674218581060912e-33",
        "0x1.c0000000fffffffffffffffffffffffffffffffffffffffeE-27#192",
        "1329227995784915872903807060280344576.0",
        "0x1000000000000000000000000000000.000000000000000000#192",
        RoundingMode::Ceiling,
        "-1329227995784915872903807060280344575.9999999999999999999998",
        "-0xffffffffffffffffffffffffffffff.ffffffffffffffffff#192",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "-0.5",
        "-0x0.8#2",
        Ordering::Equal,
    );
    test(
        "0.5",
        "0x0.8#1",
        "3.0",
        "0x3.0#2",
        RoundingMode::Nearest,
        "-2.0",
        "-0x2.0#2",
        Ordering::Greater,
    );
    test(
        "0.2",
        "0x0.4#1",
        "4.0",
        "0x4.0#2",
        RoundingMode::Nearest,
        "-4.0",
        "-0x4.0#2",
        Ordering::Less,
    );
    test(
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        "1.07183972513958531257713938927815e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        RoundingMode::Nearest,
        "0.00374222827281009532032311766811006",
        "0x0.00f5402c0bbf2e1505ed1467de9fe#107",
        Ordering::Less,
    );
    test(
        "2589062031404.0",
        "0x25ad01f682c.0#43",
        "4351572166934988.581719655389852344796925751245753159273257621838622031922945531041952618\
        810238932316",
        "0xf75bb593981cc.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
        RoundingMode::Nearest,
        "-4348983104903584.58171965538985234479692575124575315927325762183862203192294553104195261\
        8810238932316",
        "-0xf7360891a19a0.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
        Ordering::Equal,
    );

    // - in sub_float_significands_general
    // - in exponent_shift_compare
    // - sdiff_exp >= 0 in exponent_shift_compare
    // - diff_exp == 0 first time in exponent_shift_compare
    // - xi >= 0 && yi >= 0 && xs[xi] == ys[yi] in exponent_shift_compare
    // - xi < 0 in exponent_shift_compare
    // - xi < 0 && yi < 0 in exponent_shift_compare
    // - sign == Equal in sub_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - diff_exp != 0 first time in exponent_shift_compare
    // - diff_exp < Limb::WIDTH in exponent_shift_compare
    // - diff_exp != 0 second time in exponent_shift_compare
    // - (yi < 0 && lasty == 0) || high_dif != 0 || dif != 1 in exponent_shift_compare
    // - high_dif == 0 in exponent_shift_compare
    // - dif.is_power_of_2() in exponent_shift_compare
    // - yi < 0 && lasty == 0 in exponent_shift_compare
    // - sign != Equal in sub_float_significands_general
    // - sign != Less in sub_float_significands_general
    // - !neg in sub_float_significands_general
    // - max(out_prec, x_prec) + 2 > exp_diff in sub_float_significands_general
    // - shift_x != 0 in sub_float_significands_general
    // - shift_y == 0 in sub_float_significands_general
    // - cancel >= exp_diff in sub_float_significands_general
    // - out_len + cancel1 <= xs_len in sub_float_significands_general
    // - out_len + cancel2 > 0 in sub_float_significands_general
    // - cancel2 >= 0 in sub_float_significands_general
    // - out_len + cancel2 <= ys_len in sub_float_significands_general
    // - rm == Nearest in sub_float_significands_general
    // - rm == Nearest && carry <= Limb::power_of_2(sh - 1) && (0 >= carry || carry >=
    //   Limb::power_of_2(sh - 1)) in sub_float_significands_general
    // - !goto_truncate in sub_float_significands_general
    // - ixs_len <= 0 && iys_len <= 0 in sub_float_significands_general
    // - !goto_truncate && !goto_end_of_sub second time in sub_float_significands_general
    // - rm != Nearest || cmp_low == 0 in sub_float_significands_general
    // - !goto_end_of_sub in sub_float_significands_general
    // - out[out_len - 1] >> (Limb::WIDTH - 1) != 0 in sub_float_significands_general
    // - cancel != 0 in sub_float_significands_general
    test(
        "2.0",
        "0x2.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#2",
        Ordering::Equal,
    );
    // - sdiff_exp < 0 in exponent_shift_compare
    // - sign == Less in sub_float_significands_general
    // - neg in sub_float_significands_general
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.0#2",
        Ordering::Equal,
    );
    // - xi < 0 || yi < 0 || xs[xi] != ys[yi] in exponent_shift_compare
    // - xi >= 0 in exponent_shift_compare
    // - yi >= 0 second time in exponent_shift_compare
    // - xs[xi] < ys[yi] in exponent_shift_compare
    // - diff_exp == 0 second time in exponent_shift_compare
    // - shift_y != 0 in sub_float_significands_general
    test(
        "1.0",
        "0x1.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "-0.5",
        "-0x0.8#2",
        Ordering::Equal,
    );
    // - xs[xi] >= ys[yi] in exponent_shift_compare
    test(
        "1.5",
        "0x1.8#2",
        "1.0",
        "0x1.0#1",
        RoundingMode::Nearest,
        "0.5",
        "0x0.8#2",
        Ordering::Equal,
    );
    // - shift_x == 0 in sub_float_significands_general
    // - cancel < exp_diff in sub_float_significands_general
    // - ixs_len > 0 || iys_len > 0 in sub_float_significands_general
    // - ixs_len <= 0 in sub_float_significands_general
    // - iys_len condition in sub_float_significands_general
    // - cmp_low == 0 first time in sub_float_significands_general
    // - rm == Nearest && (sh != 0 || k != 0) in sub_float_significands_general
    // - cmp_low == 0 second time in sub_float_significands_general
    // - !goto_truncate && !goto_end_of_sub first time in sub_float_significands_general
    // - cancel == 0 in sub_float_significands_general
    test(
        "0.5",
        "0x0.8#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "-1.0",
        "-0x1.0#2",
        Ordering::Equal,
    );
    // - !dif.is_power_of_2() in exponent_shift_compare
    test(
        "0.5",
        "0x0.8#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Nearest,
        "-1.5",
        "-0x1.8#2",
        Ordering::Equal,
    );
    // - cmp_low != 0 first time in sub_float_significands_general
    // - cmp_low > 0 in sub_float_significands_general
    // - cmp_low > 0 && rm == Nearest in sub_float_significands_general
    // - cmp_low > 0 && rm == Nearest && xx == yy in sub_float_significands_general
    // - rm == Nearest && cmp_low != 0 in sub_float_significands_general
    // - (out[0] >> sh) & 1 == 0 in sub_float_significands_general
    test(
        "0.5",
        "0x0.8#1",
        "3.0",
        "0x3.0#2",
        RoundingMode::Nearest,
        "-2.0",
        "-0x2.0#2",
        Ordering::Greater,
    );
    // - (out[0] >> sh) & 1 != 0 in sub_float_significands_general
    // - cmp_low >= 0 in sub_float_significands_general
    // - cmp_low >= 0 && !carry in sub_float_significands_general
    test(
        "4.0",
        "0x4.0#1",
        "1.2",
        "0x1.4#3",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#3",
        Ordering::Greater,
    );
    // - cmp_low >= 0 && carry in sub_float_significands_general
    test(
        "4.0",
        "0x4.0#1",
        "0.5",
        "0x0.8#2",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#2",
        Ordering::Greater,
    );
    // - rm == Nearest && carry > Limb::power_of_2(sh - 1) in sub_float_significands_general
    // - goto_truncate in sub_float_significands_general
    test(
        "3.0",
        "0x3.0#2",
        "0.2",
        "0x0.4#1",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#2",
        Ordering::Greater,
    );
    // - rm == Nearest && carry <= Limb::power_of_2(sh - 1) && 0 < carry && carry <
    //   Limb::power_of_2(sh - 1) in sub_float_significands_general
    test(
        "4.0",
        "0x4.0#1",
        "0.8",
        "0x0.c#2",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#2",
        Ordering::Less,
    );
    // - max(out_prec, x_prec) + 2 <= exp_diff in sub_float_significands_general
    // - in round_helper
    // - dest_prec >= x_prec in round_helper
    // - !increment_exp in sub_float_significands_general
    // - inexact == 0 && rm != Down && rm != Floor in sub_float_significands_general
    test(
        "0.2",
        "0x0.4#1",
        "4.0",
        "0x4.0#2",
        RoundingMode::Nearest,
        "-4.0",
        "-0x4.0#2",
        Ordering::Less,
    );
    // - diff_exp >= Limb::WIDTH in exponent_shift_compare first time
    test(
        "8.82188e11",
        "0xc.d668E+9#18",
        "9.75459983374e122",
        "0x1.79c17f063aE+102#40",
        RoundingMode::Nearest,
        "-9.75459983374e122",
        "-0x1.79c17f063aE+102#40",
        Ordering::Less,
    );
    // - cancel2 < 0 in sub_float_significands_general
    // - out_len - neg_cancel2 <= ys_len in sub_float_significands_general
    test(
        "3.29008365861415556134836580980448399733562188e-9",
        "0xe.217c389f8c9fd22042f5ed70da20cfb9f1ecE-8#146",
        "3719044561792922503530448846362960.3599330496921301151502834994",
        "0xb75cf116bc625ef1eab58f3c9950.5c2492852d5fb6817443c180#205",
        RoundingMode::Nearest,
        "-3719044561792922503530448846362960.359933046402046456536127938",
        "-0xb75cf116bc625ef1eab58f3c9950.5c2492770be37de1e7a3ef60#205",
        Ordering::Less,
    );
    // - out_len + cancel1 > xs_len in sub_float_significands_general
    // - cancel1 < xs_len in sub_float_significands_general
    test(
        "1.07183972513958531257713938927815e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        RoundingMode::Nearest,
        "-0.00374222827281009532032311766811006",
        "-0x0.00f5402c0bbf2e1505ed1467de9fe#107",
        Ordering::Greater,
    );
    // - out_len + cancel2 > ys_len in sub_float_significands_general
    test(
        "2589062031404.0",
        "0x25ad01f682c.0#43",
        "4351572166934988.581719655389852344796925751245753159273257621838622031922945531041952618\
        810238932316",
        "0xf75bb593981cc.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
        RoundingMode::Nearest,
        "-4348983104903584.58171965538985234479692575124575315927325762183862203192294553104195261\
        8810238932316",
        "-0xf7360891a19a0.94eb944f56fd85d744c7e812bf078ed9a4f5d3086fdab71e98a907840097016cb76fdc\
        #330",
        Ordering::Equal,
    );
    // - yi >= 0 && lasty != 0 in exponent_shift_compare
    // - xi < 0 fourth time in exponent_shift_compare
    // - lasty == 0 in exponent_shift_compare
    // - yi < 0 || ys[yi] != 0 in exponent_shift_compare
    // - yi >= 0 fourth time in exponent_shift_compare
    test(
        "0.002",
        "0x0.008#2",
        "1.107886492190627864290739752375593855464628579e-38",
        "0x3.c51af197224960473945f6944424f855697e2E-32#149",
        RoundingMode::Nearest,
        "0.001953124999999999999999999999999999988921135078",
        "0x0.007ffffffffffffffffffffffffffffc3ae50e68#149",
        Ordering::Less,
    );
    // - cmp_low > 0 && rm == Nearest && xx < yy in sub_float_significands_general
    // - goto_truncate || goto_end_of_sub first time in sub_float_significands_general
    // - goto_truncate || goto_end_of_sub second time in sub_float_significands_general
    test(
        "1.521287e-9",
        "0x6.88ac4E-8#21",
        "6.842391932190563625e-20",
        "0x1.431f7157e61b20d0E-16#62",
        RoundingMode::Nearest,
        "1.5212870962270854807e-9",
        "0x6.88ac3ffebce08eaE-8#62",
        Ordering::Less,
    );
    // - out_len - neg_cancel2 > ys_len in sub_float_significands_general
    test(
        "0.00678514868524062",
        "0x0.01bcabe7b39c71#49",
        "492541199943575879969.43922949854802247248767794847124160758",
        "0x1ab361dbc0e97d4121.7071582bb3c5bd22b8f59cfc93c72a98#194",
        RoundingMode::Nearest,
        "-492541199943575879969.43244434986278185350564935536210679008",
        "-0x1ab361dbc0e97d4121.6eb4ac4400294c22b8f59cfc93c72a98#194",
        Ordering::Equal,
    );
    // - rm == Nearest && sh == 0 && k == 0 in sub_float_significands_general
    // - rm == Nearest && cmp_low >= 0 in sub_float_significands_general
    // - rm == Nearest && cmp_low >= 0 && yy < half in sub_float_significands_general
    // - rm == Nearest && cmp_low >= 0 && cmp_low <= 0 in sub_float_significands_general
    test(
        "5.7505515877842013577e-7",
        "0x9.a5d7d56cabed47dE-6#64",
        "1.1758894e-14",
        "0x3.4f515E-12#22",
        RoundingMode::Nearest,
        "5.7505514701952590309e-7",
        "0x9.a5d7d21d5a9d47dE-6#64",
        Ordering::Equal,
    );
    // - rm == Nearest && cmp_low < 0 in sub_float_significands_general
    // - rm == Nearest && cmp_low < 0 && yy < half in sub_float_significands_general
    // - cmp_low < 0 in sub_float_significands_general first time
    // - cmp_low < 0 && rm == Nearest in sub_float_significands_general
    // - rm == Nearest && (xx > yy || sh > 0 || cmp_low == -1) in sub_float_significands_general
    test(
        "8319983682.218895978935307677994592087137128849954503237724",
        "0x1efe8e042.3809911ec0c7f99b114d2930720001b00aa46846#192",
        "1.88392800575e35",
        "0x2.4487b7174E+29#37",
        RoundingMode::Nearest,
        "-188392800574747474298435817696599997.78110402106469232200542",
        "-0x24487b7173fffffffffffe10171fbd.c7f66ee13f380664eec#192",
        Ordering::Less,
    );
    // - rm == Nearest && cmp_low < 0 && yy >= half in sub_float_significands_general
    // - rm == Nearest && xx < yy && sh <= 0 && cmp_low != -1 in sub_float_significands_general
    // - goto_end_of_sub in sub_float_significands_general
    test(
        "2.36288970224581301467472547462526069521e-27",
        "0xb.b3518c72d51185c09977eb6e009c2c0E-23#128",
        "3.413020751e-12",
        "0x3.c0ae105E-10#30",
        RoundingMode::Nearest,
        "-3.413020751029435178256669624768773569e-12",
        "-0x3.c0ae104fffff44cae738d2aee7a3f668E-10#128",
        Ordering::Greater,
    );
    // - xi >= 0 fourth time in exponent_shift_compare
    // - diff_exp >= Limb::WIDTH in exponent_shift_compare second time
    // - xs[xi] == yy in exponent_shift_compare
    test(
        "1125899906842624.0",
        "0x4000000000000.000000000000#98",
        "1.166815364554e-61",
        "0x2.ffffffff80E-51#39",
        RoundingMode::Nearest,
        "1125899906842624.0",
        "0x4000000000000.000000000000#98",
        Ordering::Greater,
    );
    // - 0 < diff_exp < Limb::WIDTH && yi >= 0 in exponent_shift_compare
    // - xs[xi] != yy in exponent_shift_compare
    test(
        "9671406556917033397649408.0",
        "0x800000000000000000000.0000#99",
        "65536.015624999999999999946",
        "0x10000.03ffffffffffffff00#87",
        RoundingMode::Nearest,
        "9671406556917033397583871.98438",
        "0x7fffffffffffffffeffff.fc00#99",
        Ordering::Less,
    );
    // - diff_exp == 0 && yi >= 0 in exponent_shift_compare
    test(
        "1.3877787807814456370485565165946e-17",
        "0xf.ffffffffffffe007ffffffff8E-15#101",
        "128.0000000000000000000000000008077935669463159990585083341",
        "0x80.00000000000000000000003ffffffffffffe0000000000#190",
        RoundingMode::Nearest,
        "-127.9999999999999999861222121929933371964607508331127668586",
        "-0x7f.ffffffffffffff00000000400001ff7ffffe0008000000#190",
        Ordering::Equal,
    );
    // - rm == Nearest && xx == yy && sh <= 0 && cmp_low != -1 in sub_float_significands_general
    // - cmp_low < 0 in sub_float_significands_general second time
    test(
        "2.220581574517834801066783605693002897371795957735742517101e-16",
        "0x1.0003fffffe00000000007ffffffffffffffffffffff000feE-13#192",
        "8.881784e-16",
        "0x4.00000E-13#21",
        RoundingMode::Nearest,
        "-6.661202622483417522322269739033559602628204042264257482898e-16",
        "-0x2.fffc000001ffffffffff80000000000000000000000fff00E-13#192",
        Ordering::Greater,
    );
    // - diff_exp < Limb::WIDTH && yi < 0 in exponent_shift_compare
    test(
        "-1.8e-12",
        "-0x1.fcE-10#7",
        "-524288.0000000000017621525",
        "-0x80000.0000000001f00078#81",
        RoundingMode::Nearest,
        "524287.9999999999999573739",
        "0x7ffff.fffffffffff40078#81",
        Ordering::Equal,
    );
    // - (yi >= 0 || lasty != 0) && high_dif == 0 && dif == 1 in exponent_shift_compare
    // - xi >= 0 third time in exponent_shift_compare
    // - yi >= 0 && diff_exp != 0 in exponent_shift_compare
    // - high_dif != 0 in exponent_shift_compare
    // - dif == 0 in exponent_shift_compare
    test(
        "3.99999999999999999957",
        "0x3.fffffffffffffff80#67",
        "4.0",
        "0x4.00000000000000000#68",
        RoundingMode::Nearest,
        "-4.33680868994201773603e-19",
        "-0x8.0000000000000000E-16#68",
        Ordering::Equal,
    );
    // - xi < 0 third time in exponent_shift_compare
    // - cancel1 >= xs_len in sub_float_significands_general
    test(
        "6.77626357803440271254657930615627554e-21",
        "0x1.ffffffffffffffffffffc01fff800E-17#117",
        "7.0e-21",
        "0x2.0E-17#4",
        RoundingMode::Nearest,
        "-6.99280860154738521672154778865541105e-46",
        "-0x3.fe000800000000000000000000000E-38#117",
        Ordering::Equal,
    );
    // - yi >= 0 && diff_exp == 0 in exponent_shift_compare
    // - dif != 0 in exponent_shift_compare
    test(
        "1.45519152351431153846750277125465800054371356958e-11",
        "0x1.00000001ffffffffffffffffffffffffffffffcE-9#155",
        "1.455191523514311538467502771254658000526607875496523229114700566610230516799191954282190\
        364e-11",
        "0x1.00000001fffffffffffffffffffffffc0000000000000000000000000000fffffffffff87f8E-9#301",
        RoundingMode::Nearest,
        "1.710569408086637569000134230861426729009957349416545362986887684243433631257558931055548\
        3967e-49",
        "0x3.ffffffbfffffffffffffffffffff00000000000780800000000000000000000000000000000E-41#301",
        Ordering::Equal,
    );
    // - xs[xi] == 0 in exponent_shift_compare
    // - xi < 0 second time in exponent_shift_compare
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - xi >= 0 second time in exponent_shift_compare
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // - rm != Nearest in sub_float_significands_general
    // - rm != Nearest && carry == 0 in sub_float_significands_general
    test(
        "2.0",
        "0x2.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Down,
        "1.0",
        "0x1.0#2",
        Ordering::Equal,
    );
    // - rm != Nearest && carry != 0 in sub_float_significands_general
    // - rm == Floor || rm == Down in sub_float_significands_general
    test(
        "0.5",
        "0x0.8#1",
        "3.0",
        "0x3.0#2",
        RoundingMode::Down,
        "-2.0",
        "-0x2.0#2",
        Ordering::Greater,
    );
    // - rm == Ceiling || rm == Up in sub_float_significands_general
    // - (rm == Ceiling || rm == Up) && !carry in sub_float_significands_general
    test(
        "0.5",
        "0x0.8#1",
        "3.0",
        "0x3.0#2",
        RoundingMode::Up,
        "-3.0",
        "-0x3.0#2",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && carry in sub_float_significands_general
    test(
        "4.0",
        "0x4.0#1",
        "0.5",
        "0x0.8#2",
        RoundingMode::Up,
        "4.0",
        "0x4.0#2",
        Ordering::Greater,
    );
    // - cmp_low < 0 && (rm == Floor || rm == Down) in sub_float_significands_general
    test(
        "0.1952943266615587806218370459",
        "0x0.31fecf1a1b1180be748fe5b8#91",
        "5.04217616231508488430478600129900999076e-13",
        "0x8.decb552f6cf9a70a64d0c0d7a367802E-11#127",
        RoundingMode::Down,
        "0.195294326661054563005605537435615043117",
        "0x0.31fecf1a1a8394092199161d8f59b2f38#127",
        Ordering::Less,
    );
    // - cmp_low < 0 && (rm == Ceiling || rm == Up) in sub_float_significands_general
    test(
        "1.1718977744e31",
        "0x9.3ea0f304E+25#34",
        "1910333504687680.9305",
        "0x6c9702df30e40.ee38#64",
        RoundingMode::Ceiling,
        "1.171897774363652748e31",
        "0x9.3ea0f303ffff937E+25#64",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000E-7#69",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "7.0e-9",
        "0x2.0E-7#2",
        "7.450580596923828125e-9",
        "0x2.00000000000000000000000000000000000000000000000000E-7#200",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
}

#[test]
fn sub_round_fail() {
    assert_panic!(
        Float::one_prec(1).sub_round(Float::from_unsigned_prec(4u8, 1).0, RoundingMode::Exact)
    );
    assert_panic!(Float::one_prec(1)
        .sub_round_val_ref(&Float::from_unsigned_prec(4u8, 1).0, RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .sub_round_ref_val(Float::from_unsigned_prec(4u8, 1).0, RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .sub_round_ref_ref(&Float::from_unsigned_prec(4u8, 1).0, RoundingMode::Exact));

    assert_panic!(
        parse_hex_string("0x1.0#1").sub_round(parse_hex_string("0x0.001#1"), RoundingMode::Exact)
    );
    assert_panic!(parse_hex_string("0x1.0#1")
        .sub_round_val_ref(&parse_hex_string("0x0.001#1"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x1.0#1")
        .sub_round_ref_val(parse_hex_string("0x0.001#1"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x1.0#1")
        .sub_round_ref_ref(&parse_hex_string("0x0.001#1"), RoundingMode::Exact));

    assert_panic!(parse_hex_string("0x1.0000000000000000#64").sub_round(
        parse_hex_string("0x2.0000000000000002#64"),
        RoundingMode::Exact
    ));
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64").sub_round_val_ref(
            &parse_hex_string("0x2.0000000000000002#64"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64").sub_round_ref_val(
            parse_hex_string("0x2.0000000000000002#64"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64").sub_round_ref_ref(
            &parse_hex_string("0x2.0000000000000002#64"),
            RoundingMode::Exact
        )
    );

    assert_panic!(parse_hex_string("0x1.0000000000000000#65").sub_round(
        parse_hex_string("0x2.0000000000000001#65"),
        RoundingMode::Exact
    ));
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65").sub_round_val_ref(
            &parse_hex_string("0x2.0000000000000001#65"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65").sub_round_ref_val(
            parse_hex_string("0x2.0000000000000001#65"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65").sub_round_ref_ref(
            &parse_hex_string("0x2.0000000000000001#65"),
            RoundingMode::Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").sub_round(
            parse_hex_string("0x2.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").sub_round_val_ref(
            &parse_hex_string("0x2.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").sub_round_ref_val(
            parse_hex_string("0x2.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").sub_round_ref_ref(
            &parse_hex_string("0x2.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").sub_round(
            parse_hex_string("0x2.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").sub_round_val_ref(
            &parse_hex_string("0x2.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").sub_round_ref_val(
            parse_hex_string("0x2.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").sub_round_ref_ref(
            &parse_hex_string("0x2.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192").sub_round(
            parse_hex_string("0x2.000000000000000000000000000000000000000000000002#192"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .sub_round_val_ref(
                &parse_hex_string("0x2.000000000000000000000000000000000000000000000002#192"),
                RoundingMode::Exact
            )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .sub_round_ref_val(
                parse_hex_string("0x2.000000000000000000000000000000000000000000000002#192"),
                RoundingMode::Exact
            )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .sub_round_ref_ref(
                &parse_hex_string("0x2.000000000000000000000000000000000000000000000002#192"),
                RoundingMode::Exact
            )
    );

    assert_panic!(
        parse_hex_string("0x0.8#1").sub_round(parse_hex_string("0x3.0#2"), RoundingMode::Exact)
    );
    assert_panic!(parse_hex_string("0x0.8#1")
        .sub_round_val_ref(&parse_hex_string("0x3.0#2"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x0.8#1")
        .sub_round_ref_val(parse_hex_string("0x3.0#2"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x0.8#1")
        .sub_round_ref_ref(&parse_hex_string("0x3.0#2"), RoundingMode::Exact));

    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_round_assign(Float::from_unsigned_prec(4u8, 1).0, RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_round_assign_ref(&Float::from_unsigned_prec(4u8, 1).0, RoundingMode::Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.sub_round_assign(parse_hex_string("0x0.001#1"), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.sub_round_assign_ref(&parse_hex_string("0x0.001#1"), RoundingMode::Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#64");
        x.sub_round_assign(
            parse_hex_string("0x2.0000000000000002#64"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#64");
        x.sub_round_assign_ref(
            &parse_hex_string("0x2.0000000000000002#64"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#65");
        x.sub_round_assign(
            parse_hex_string("0x2.0000000000000001#65"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#65");
        x.sub_round_assign_ref(
            &parse_hex_string("0x2.0000000000000001#65"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#128");
        x.sub_round_assign(
            parse_hex_string("0x2.00000000000000000000000000000002#128"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#128");
        x.sub_round_assign_ref(
            &parse_hex_string("0x2.00000000000000000000000000000002#128"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#129");
        x.sub_round_assign(
            parse_hex_string("0x2.00000000000000000000000000000003#129"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#129");
        x.sub_round_assign_ref(
            &parse_hex_string("0x2.00000000000000000000000000000003#129"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192");
        x.sub_round_assign(
            parse_hex_string("0x2.000000000000000000000000000000000000000000000002#192"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192");
        x.sub_round_assign_ref(
            &parse_hex_string("0x2.000000000000000000000000000000000000000000000002#192"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x0.8#1");
        x.sub_round_assign(parse_hex_string("0x3.0#2"), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x0.8#1");
        x.sub_round_assign_ref(&parse_hex_string("0x3.0#2"), RoundingMode::Exact)
    });
}

#[test]
fn test_sub_prec_round() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (diff, o) = x.clone().sub_prec_round(y.clone(), prec, rm);
        assert!(diff.is_valid());

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);
        assert_eq!(o, o_out);

        let (diff_alt, o_alt) = x.clone().sub_prec_round_val_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = x.sub_prec_round_ref_val(y.clone(), prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = x.sub_prec_round_ref_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_prec_round_assign(y.clone(), prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_prec_round_assign_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o_out);

        let (diff_alt, o_alt) = add_prec_round_naive(x.clone(), -&y, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123.0",
        "-0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#1",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2.0",
        "-0x2.0#2",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2.0",
        "-0x2.00#10",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "4.56",
        "0x4.90#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "4.56",
        "0x4.90#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "-1.729",
        "-0x1.ba8#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "-1.729",
        "-0x1.ba8#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "1.729",
        "0x1.ba8#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "1.729",
        "0x1.ba8#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "-4.56",
        "-0x4.90#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "-4.56",
        "-0x4.90#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        1,
        RoundingMode::Floor,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        1,
        RoundingMode::Ceiling,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        1,
        RoundingMode::Down,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        1,
        RoundingMode::Up,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        1,
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        RoundingMode::Floor,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        RoundingMode::Ceiling,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        RoundingMode::Down,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        RoundingMode::Up,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        RoundingMode::Nearest,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-0.0002",
        "-0x0.001#1",
        20,
        RoundingMode::Exact,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - rm != Nearest in round_helper
    // - (rm == Up || rm == Ceiling) && sb | rb != 0 in round_helper
    // - !increment third time in round_helper
    test(
        "13104.5238818416080254535",
        "0x3330.861d1ed0acba8a3a#77",
        "2.854e-35",
        "0x2.5fE-29#10",
        17,
        RoundingMode::Ceiling,
        "13104.6",
        "0x3330.a#17",
        Ordering::Greater,
    );
    // - sh != 0 && (rm != Nearest && rb != 0) in round_helper
    // - rm == Down || rm == Floor in round_helper
    test(
        "2.8979948183270175762296398212780973e-12",
        "0x3.2fb688bd98f0271a53708f0554568E-10#115",
        "8.270488650862e23",
        "0xa.f2268a8074E+19#42",
        2,
        RoundingMode::Down,
        "-6.0e23",
        "-0x8.0E+19#2",
        Ordering::Greater,
    );
    // - (rm == Up || rm == Ceiling) && sb | rb == 0 in round_helper
    test(
        "1.503764257662314e22",
        "0x3.2f31367a9800E+18#48",
        "7.3839e-20",
        "0x1.5cb2E-16#17",
        39,
        RoundingMode::Ceiling,
        "1.503764257662e22",
        "0x3.2f31367a98E+18#39",
        Ordering::Greater,
    );
    // - increment third time in round_helper
    test(
        "4.70916604581e-30",
        "0x5.f8363584bE-25#39",
        "341290809831481093.63402342431195212374059",
        "0x4bc822eed1c5f05.a24f5bf051591756f951#139",
        1,
        RoundingMode::Floor,
        "-6.0e17",
        "-0x8.0E+14#1",
        Ordering::Less,
    );
    // - cmp_low > 0 && (rm == Floor || rm == Down) in sub_float_significands_general
    test(
        "559935046210054011882951826578284118061013900.5853448",
        "0x191bbd3588c78488c2f4d122814d5fb34edb8c.95d928#170",
        "3.027932e11",
        "0x4.67fe2E+9#22",
        63,
        RoundingMode::Down,
        "5.599350462100540119e44",
        "0x1.91bbd3588c78488cE+37#63",
        Ordering::Less,
    );
    // - cmp_low > 0 && (rm == Ceiling || rm == Up) in sub_float_significands_general
    // - cmp_low > 0 && (rm == Ceiling || rm == Up) && !carry in sub_float_significands_general
    test(
        "1.3111820218254081035114504135472568116036464005e-6",
        "0x0.000015ff7be10e865ada82cd25acef5baa9c89c25f4#152",
        "2.51465891601e-20",
        "0x7.6c05c64a8E-17#38",
        128,
        RoundingMode::Ceiling,
        "1.311182021825382956922290308381837960614e-6",
        "0x0.000015ff7be10e85e41a26687dacef5baa9ca#128",
        Ordering::Greater,
    );
    // - cmp_low > 0 && (rm == Ceiling || rm == Up) && carry in sub_float_significands_general
    test(
        "1.19e-7",
        "0x2.0E-6#6",
        "2722258925226302905881161717745111269376.0000001187",
        "0x7ffffff800000000000003fe7c0000000.000001fe0#164",
        28,
        RoundingMode::Up,
        "-2.72225894e39",
        "-0x8.000000E+32#28",
        Ordering::Less,
    );
}

#[test]
fn sub_prec_round_fail() {
    assert_panic!(Float::one_prec(1).sub_prec_round(Float::two_prec(1), 0, RoundingMode::Floor));
    assert_panic!(Float::one_prec(1).sub_prec_round_val_ref(
        &Float::two_prec(1),
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::one_prec(1).sub_prec_round_ref_val(
        Float::two_prec(1),
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::one_prec(1).sub_prec_round_ref_ref(
        &Float::two_prec(1),
        0,
        RoundingMode::Floor
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_prec_round_assign(Float::two_prec(1), 0, RoundingMode::Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_prec_round_assign_ref(&Float::two_prec(1), 0, RoundingMode::Floor)
    });

    assert_panic!(Float::one_prec(1).sub_prec_round(-Float::two_prec(1), 1, RoundingMode::Exact));
    assert_panic!(Float::one_prec(1).sub_prec_round_val_ref(
        &-Float::two_prec(1),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).sub_prec_round_ref_val(
        -Float::two_prec(1),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).sub_prec_round_ref_ref(
        &-Float::two_prec(1),
        1,
        RoundingMode::Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_prec_round_assign(-Float::two_prec(1), 1, RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_prec_round_assign_ref(&-Float::two_prec(1), 1, RoundingMode::Exact)
    });
}

#[test]
fn test_sub_rational() {
    let test = |s, s_hex, t, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let diff = x.clone() - y.clone();
        assert!(diff.is_valid());

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);

        let diff_alt = x.clone() - &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        let diff_alt = &x - y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        let diff_alt = &x - &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));

        let diff_alt = y.clone() - x.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloat(-&diff), ComparableFloat(diff_alt));
        let diff_alt = y.clone() - &x;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloat(-&diff), ComparableFloat(diff_alt));
        let diff_alt = &y - x.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloat(-&diff), ComparableFloat(diff_alt));
        let diff_alt = &y - &x;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloat(-&diff), ComparableFloat(diff_alt));

        let mut diff_alt = x.clone();
        diff_alt -= y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        let mut diff_alt = x.clone();
        diff_alt -= &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sub_rational(
                rug::Float::exact_from(&x),
                rug::Rational::from(&y)
            ))),
            ComparableFloatRef(&diff)
        );

        let diff_alt = add_rational_prec_round_naive(
            x.clone(),
            -&y,
            x.significant_bits(),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    };
    test("NaN", "NaN", "-123", "NaN", "NaN");
    test("Infinity", "Infinity", "-123", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-123", "-Infinity", "-Infinity");
    test("0.0", "0x0.0", "0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "-123", "1.0e2", "0x8.0E+1#1");
    test("-0.0", "-0x0.0", "-123", "1.0e2", "0x8.0E+1#1");
    test("0.0", "0x0.0", "-1/3", "0.2", "0x0.4#1");
    test("-0.0", "-0x0.0", "-1/3", "0.2", "0x0.4#1");
    test("123.0", "0x7b.0#7", "0", "123.0", "0x7b.0#7");

    test("1.0", "0x1.0#1", "-2", "4.0", "0x4.0#1");
    test("1.0", "0x1.0#2", "-2", "3.0", "0x3.0#2");
    test("1.0", "0x1.000#10", "-2", "3.0", "0x3.00#10");
    test("1.0", "0x1.000#10", "-1/3", "1.334", "0x1.558#10");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "-1/3",
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
    );

    test("1.0", "0x1.0#1", "-1/50000", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "1", "0.0", "0x0.0");
}

#[test]
fn test_sub_rational_prec() {
    let test = |s, s_hex, t, prec, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (diff, o) = x.clone().sub_rational_prec(y.clone(), prec);
        assert!(diff.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);

        let (diff_alt, o_alt) = x.clone().sub_rational_prec_val_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_prec_ref_val(y.clone(), prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_prec_ref_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_rational_prec_assign(y.clone(), prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_rational_prec_assign_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), -&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test("NaN", "NaN", "-123", 1, "NaN", "NaN", Ordering::Equal);
    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test("0.0", "0x0.0", "0", 1, "0.0", "0x0.0", Ordering::Equal);
    test("-0.0", "-0x0.0", "0", 1, "-0.0", "-0x0.0", Ordering::Equal);
    test(
        "0.0",
        "0x0.0",
        "-123",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test("0.0", "0x0.0", "-1/3", 1, "0.2", "0x0.4#1", Ordering::Less);
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        1,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test("1.0", "0x1.0#1", "-2", 2, "3.0", "0x3.0#2", Ordering::Equal);
    test(
        "1.0",
        "0x1.0#2",
        "-2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test("1.0", "0x1.0#2", "-2", 2, "3.0", "0x3.0#2", Ordering::Equal);
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        100,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        10,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );
    test("1.0", "0x1.0#1", "1", 10, "0.0", "0x0.0", Ordering::Equal);
}

#[test]
fn sub_rational_prec_fail() {
    assert_panic!(Float::NAN.sub_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::NAN.sub_rational_prec_val_ref(&Rational::ZERO, 0));
    assert_panic!(Float::NAN.sub_rational_prec_ref_val(Rational::ZERO, 0));
    assert_panic!(Float::NAN.sub_rational_prec_ref_ref(&Rational::ZERO, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.sub_rational_prec_assign(Rational::ZERO, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.sub_rational_prec_assign_ref(&Rational::ZERO, 0)
    });
}

#[test]
fn test_sub_rational_round() {
    let test = |s, s_hex, t, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (diff, o) = x.clone().sub_rational_round(y.clone(), rm);
        assert!(diff.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);

        let (diff_alt, o_alt) = x.clone().sub_rational_round_val_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_round_ref_val(y.clone(), rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_round_ref_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_rational_round_assign(y.clone(), rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_rational_round_assign_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_diff, rug_o) = rug_sub_rational_round(
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_diff)),
                ComparableFloatRef(&diff)
            );
            assert_eq!(rug_o, o);
        }

        let (diff_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), -&y, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "-123",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-123",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-123",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-123",
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-123",
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-1/3",
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        RoundingMode::Nearest,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        RoundingMode::Nearest,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2",
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "-2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2",
        RoundingMode::Ceiling,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2",
        RoundingMode::Down,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2",
        RoundingMode::Up,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "-2",
        RoundingMode::Exact,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-2",
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-2",
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        RoundingMode::Floor,
        "1.332",
        "0x1.550#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        RoundingMode::Ceiling,
        "1.334",
        "0x1.558#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        RoundingMode::Down,
        "1.332",
        "0x1.550#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        RoundingMode::Up,
        "1.334",
        "0x1.558#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        RoundingMode::Nearest,
        "1.334",
        "0x1.558#10",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "-1/3",
        RoundingMode::Floor,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "-1/3",
        RoundingMode::Ceiling,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "-1/3",
        RoundingMode::Down,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "-1/3",
        RoundingMode::Up,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "-1/3",
        RoundingMode::Nearest,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Floor,
        "3.4749259869231262",
        "0x3.7994bfdddaf84#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Ceiling,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Down,
        "3.4749259869231262",
        "0x3.7994bfdddaf84#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Up,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Nearest,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Floor,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Ceiling,
        "2.8082593202564601",
        "0x2.ceea1533304dc#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Down,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Up,
        "2.8082593202564601",
        "0x2.ceea1533304dc#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Nearest,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Ordering::Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Floor,
        "-2.8082593202564601",
        "-0x2.ceea1533304dc#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Ceiling,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Down,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Up,
        "-2.8082593202564601",
        "-0x2.ceea1533304dc#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Nearest,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Ordering::Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Floor,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Ceiling,
        "-3.4749259869231262",
        "-0x3.7994bfdddaf84#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Down,
        "-3.4749259869231262",
        "-0x3.7994bfdddaf84#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Up,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Nearest,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        RoundingMode::Floor,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        RoundingMode::Ceiling,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        RoundingMode::Down,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        RoundingMode::Up,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
}

#[test]
fn sub_rational_round_fail() {
    assert_panic!(Float::one_prec(1)
        .sub_rational_round(-Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .sub_rational_round_val_ref(&-Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .sub_rational_round_ref_val(-Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .sub_rational_round_ref_ref(&-Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_rational_round_assign(-Rational::from_unsigneds(1u32, 3), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_rational_round_assign_ref(&-Rational::from_unsigneds(1u32, 3), RoundingMode::Exact)
    });
}

#[test]
fn test_sub_rational_prec_round() {
    let test = |s, s_hex, t, prec, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (diff, o) = x.clone().sub_rational_prec_round(y.clone(), prec, rm);
        assert!(diff.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(diff.to_string(), out);
        assert_eq!(to_hex_string(&diff), out_hex);

        let (diff_alt, o_alt) = x.clone().sub_rational_prec_round_val_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_rational_prec_round_assign(y.clone(), prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let mut diff_alt = x.clone();
        let o_alt = diff_alt.sub_rational_prec_round_assign_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = add_rational_prec_round_naive(x.clone(), -&y, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "-123",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-123",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-123",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-123",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-123",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-123",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-123",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-123",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "0.0",
        "0x0.0",
        "-1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        1,
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        1,
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        1,
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "-1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        1,
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        1,
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        1,
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-2",
        2,
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        2,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        2,
        RoundingMode::Down,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        2,
        RoundingMode::Up,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        2,
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-2",
        2,
        RoundingMode::Exact,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        100,
        RoundingMode::Floor,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        100,
        RoundingMode::Ceiling,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        100,
        RoundingMode::Down,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        100,
        RoundingMode::Up,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "-1/3",
        100,
        RoundingMode::Nearest,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Floor,
        "3.473",
        "0x3.79#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Ceiling,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Down,
        "3.473",
        "0x3.79#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Up,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Nearest,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Floor,
        "2.805",
        "0x2.ce#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Ceiling,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Down,
        "2.805",
        "0x2.ce#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Up,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Nearest,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Floor,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Ceiling,
        "-2.805",
        "-0x2.ce#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Down,
        "-2.805",
        "-0x2.ce#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Up,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Nearest,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Floor,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Ceiling,
        "-3.473",
        "-0x3.79#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Down,
        "-3.473",
        "-0x3.79#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Up,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Nearest,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        10,
        RoundingMode::Floor,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        10,
        RoundingMode::Ceiling,
        "1.002",
        "0x1.008#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        10,
        RoundingMode::Down,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        10,
        RoundingMode::Up,
        "1.002",
        "0x1.008#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1/50000",
        10,
        RoundingMode::Nearest,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1",
        10,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        10,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        10,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        10,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        10,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1",
        10,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
}

#[test]
fn sub_rational_prec_round_fail() {
    assert_panic!(Float::one_prec(1).sub_rational_prec_round(
        Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).sub_rational_prec_round_val_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).sub_rational_prec_round_ref_val(
        Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).sub_rational_prec_round_ref_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_rational_prec_round_assign(Rational::from_unsigneds(5u32, 8), 1, RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sub_rational_prec_round_assign_ref(
            &Rational::from_unsigneds(5u32, 8),
            1,
            RoundingMode::Exact,
        )
    });
}

#[test]
fn sub_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(|(x, y, prec, rm)| {
        let (diff, o) = x.clone().sub_prec_round(y.clone(), prec, rm);
        assert!(diff.is_valid());
        let (diff_alt, o_alt) = x.clone().sub_prec_round_val_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_prec_round_ref_val(y.clone(), prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_prec_round_ref_ref(&y, prec, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_prec_round_assign(y.clone(), prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_prec_round_assign_ref(&y, prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = add_prec_round_naive(x.clone(), -&y, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let r_diff = if diff.is_finite() {
            if diff.is_normal() {
                assert_eq!(diff.get_prec(), Some(prec));
            }
            let r_diff = Rational::exact_from(&x) - Rational::exact_from(&y);
            assert_eq!(diff.partial_cmp(&r_diff), Some(o));
            if o == Ordering::Less {
                let mut next = diff.clone();
                next.increment();
                assert!(next > r_diff);
            } else if o == Ordering::Greater {
                let mut next = diff.clone();
                next.decrement();
                assert!(next < r_diff);
            }
            Some(r_diff)
        } else {
            assert_eq!(o, Ordering::Equal);
            None
        };

        match (r_diff.is_some() && *r_diff.as_ref().unwrap() >= 0u32, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }

        let (mut diff_alt, mut o_alt) = y.sub_prec_round_ref_ref(&x, prec, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff_alt.abs_negative_zero_ref()),
            ComparableFloat(diff.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.add_prec_round_ref_val(-&y, prec, rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let (mut diff_alt, mut o_alt) = (-&x).add_prec_round_val_ref(&y, prec, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff_alt.abs_negative_zero()),
            ComparableFloat(diff.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (mut diff_alt, mut o_alt) = (-x).sub_prec_round(-y, prec, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff_alt.abs_negative_zero()),
            ComparableFloat(diff.abs_negative_zero())
        );
        assert_eq!(o_alt, o);
    });

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (diff, o) = x.sub_prec_round_ref_val(Float::NAN, prec, rm);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        let (diff, o) = Float::NAN.sub_prec_round_val_ref(&x, prec, rm);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        if !x.is_nan() {
            if x != Float::INFINITY {
                assert_eq!(
                    x.sub_prec_round_ref_val(Float::INFINITY, prec, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::INFINITY.sub_prec_round_val_ref(&x, prec, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
            }
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.sub_prec_round_ref_val(Float::NEGATIVE_INFINITY, prec, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.sub_prec_round_val_ref(&x, prec, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (diff, o) = x.sub_prec_round_ref_val(Float::ZERO, prec, rm);
            let mut diff_alt = x.clone();
            let o_alt = diff_alt.set_prec_round(prec, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat(diff_alt.abs_negative_zero())
            );
            assert_eq!(o, o_alt);

            let (diff, o) = Float::ZERO.sub_prec_round_val_ref(&x, prec, rm);
            let mut diff_alt = -&x;
            let o_alt = diff_alt.set_prec_round(prec, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat(diff_alt.abs_negative_zero())
            );
            assert_eq!(o, o_alt);
        }
        if rm != RoundingMode::Floor || !x.is_positive_zero() {
            let (diff, o) = x.sub_prec_round_ref_val(Float::NEGATIVE_ZERO, prec, rm);
            let mut diff_alt = x.clone();
            let o_alt = diff_alt.set_prec_round(prec, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat(diff_alt.abs_negative_zero())
            );
            assert_eq!(o, o_alt);

            let (diff, o) = Float::NEGATIVE_ZERO.sub_prec_round_val_ref(&x, prec, rm);
            let mut diff_alt = -&x;
            let o_alt = diff_alt.set_prec_round(prec, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat(diff_alt.abs_negative_zero())
            );
            assert_eq!(o, o_alt);
        }
    });
}

#[test]
fn sub_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (diff, o) = x.clone().sub_prec(y.clone(), prec);
        assert!(diff.is_valid());
        let (diff_alt, o_alt) = x.clone().sub_prec_val_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_prec_ref_val(y.clone(), prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_prec_ref_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = add_prec_round_naive(x.clone(), -&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_prec_round_ref_ref(&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if diff.is_finite() {
            if diff.is_normal() {
                assert_eq!(diff.get_prec(), Some(prec));
            }
            let r_diff = Rational::exact_from(&x) - Rational::exact_from(&y);
            assert_eq!(diff.partial_cmp(&r_diff), Some(o));
            if o == Ordering::Less {
                let mut next = diff.clone();
                next.increment();
                assert!(next > r_diff);
            } else if o == Ordering::Greater {
                let mut next = diff.clone();
                next.decrement();
                assert!(next < r_diff);
            }
        } else {
            assert_eq!(o, Ordering::Equal);
        }

        let (mut diff_alt, mut o_alt) = y.sub_prec_ref_ref(&x, prec);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff_alt.abs_negative_zero_ref()),
            ComparableFloat(diff.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.add_prec_ref_val(-&y, prec);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
            let (mut diff_alt, mut o_alt) = (-&x).add_prec_val_ref(&y, prec);
            diff_alt.neg_assign();
            diff_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let (mut diff_alt, mut o_alt) = (-x).sub_prec(-y, prec);
            diff_alt.neg_assign();
            diff_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (diff, o) = x.sub_prec_ref_val(Float::NAN, prec);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        let (diff, o) = Float::NAN.sub_prec_val_ref(&x, prec);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        if !x.is_nan() {
            if x != Float::INFINITY {
                assert_eq!(
                    x.sub_prec_ref_val(Float::INFINITY, prec),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::INFINITY.sub_prec_val_ref(&x, prec),
                    (Float::INFINITY, Ordering::Equal)
                );
            }
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.sub_prec_ref_val(Float::NEGATIVE_INFINITY, prec),
                    (Float::INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.sub_prec_val_ref(&x, prec),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (diff, o) = x.sub_prec_ref_val(Float::ZERO, prec);
            let mut diff_alt = x.clone();
            let o_alt = diff_alt.set_prec(prec);
            assert_eq!(ComparableFloat(diff), ComparableFloat(diff_alt));
            assert_eq!(o, o_alt);

            let (diff, o) = Float::ZERO.sub_prec_val_ref(&x, prec);
            let mut diff_alt = -&x;
            let o_alt = diff_alt.set_prec(prec);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat(diff_alt.abs_negative_zero())
            );
            assert_eq!(o, o_alt);
        }
        let (diff, o) = x.sub_prec_ref_val(Float::NEGATIVE_ZERO, prec);
        let mut diff_alt = x.clone();
        let o_alt = diff_alt.set_prec(prec);
        assert_eq!(
            ComparableFloat(diff.abs_negative_zero()),
            ComparableFloat(diff_alt.abs_negative_zero())
        );
        assert_eq!(o, o_alt);

        let (diff, o) = Float::NEGATIVE_ZERO.sub_prec_val_ref(&x, prec);
        let mut diff_alt = -x;
        let o_alt = diff_alt.set_prec(prec);
        assert_eq!(ComparableFloat(diff), ComparableFloat(diff_alt));
        assert_eq!(o, o_alt);
    });
}

fn sub_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (diff, o) = x.clone().sub_round(y.clone(), rm);
    assert!(diff.is_valid());
    let (diff_alt, o_alt) = x.clone().sub_round_val_ref(&y, rm);
    assert!(diff_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let (diff_alt, o_alt) = x.sub_round_ref_val(y.clone(), rm);
    assert!(diff_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let (diff_alt, o_alt) = x.sub_round_ref_ref(&y, rm);
    assert!(diff_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.sub_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    let (diff_alt, o_alt) = add_prec_round_naive(
        x.clone(),
        -&y,
        max(x.significant_bits(), y.significant_bits()),
        rm,
    );
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);
    let (diff_alt, o_alt) =
        x.sub_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), rm);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    assert_eq!(o_alt, o);

    let r_diff = if diff.is_finite() {
        if x.is_normal() && y.is_normal() && diff.is_normal() {
            assert_eq!(
                diff.get_prec(),
                Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
            );
        }
        let r_diff = Rational::exact_from(&x) - Rational::exact_from(&y);
        assert_eq!(diff.partial_cmp(&r_diff), Some(o));
        if o == Ordering::Less {
            let mut next = diff.clone();
            next.increment();
            assert!(next > r_diff);
        } else if o == Ordering::Greater {
            let mut next = diff.clone();
            next.decrement();
            assert!(next < r_diff);
        }
        Some(r_diff)
    } else {
        assert_eq!(o, Ordering::Equal);
        None
    };
    match (r_diff.is_some() && *r_diff.as_ref().unwrap() >= 0u32, rm) {
        (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
            assert_ne!(o, Ordering::Greater)
        }
        (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
            assert_ne!(o, Ordering::Less)
        }
        (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
        _ => {}
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_diff, rug_o) =
            rug_sub_round(rug::Float::exact_from(&x), rug::Float::exact_from(&y), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_diff)),
            ComparableFloatRef(&diff),
        );
        assert_eq!(rug_o, o);
    }

    let (mut diff_alt, mut o_alt) = y.sub_round_ref_ref(&x, -rm);
    diff_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&diff_alt.abs_negative_zero()),
        ComparableFloatRef(&diff.abs_negative_zero_ref())
    );

    let (diff_alt, o_alt) = x.add_round_ref_val(-&y, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

    let (mut diff_alt, mut o_alt) = (-&x).add_round_val_ref(&y, -rm);
    diff_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(diff_alt.abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero_ref())
    );

    let (mut diff_alt, mut o_alt) = (-x).sub_round(-y, -rm);
    diff_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(diff_alt.abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero())
    );
}

#[test]
fn sub_round_properties() {
    float_float_rounding_mode_triple_gen_var_2().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_10().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_11().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_12().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_13().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_14().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_15().test_properties(|(x, y, rm)| {
        sub_round_properties_helper(x, y, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (diff, o) = x.sub_round_ref_val(Float::NAN, rm);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        let (diff, o) = Float::NAN.sub_round_val_ref(&x, rm);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        if !x.is_nan() {
            if x != Float::INFINITY {
                assert_eq!(
                    x.sub_round_ref_val(Float::INFINITY, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::INFINITY.sub_round_val_ref(&x, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
            }
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.sub_round_ref_val(Float::NEGATIVE_INFINITY, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.sub_round_val_ref(&x, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (diff, o) = x.sub_round_ref_val(Float::ZERO, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero_ref()),
                ComparableFloat(x.abs_negative_zero_ref())
            );
            assert_eq!(o, Ordering::Equal);
            let (diff, o) = Float::ZERO.sub_round_val_ref(&x, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat((-&x).abs_negative_zero())
            );
            assert_eq!(o, Ordering::Equal);
        }
        if rm != RoundingMode::Floor || !x.is_positive_zero() {
            let (diff, o) = x.sub_round_ref_val(Float::NEGATIVE_ZERO, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero_ref()),
                ComparableFloat(x.abs_negative_zero_ref())
            );
            assert_eq!(o, Ordering::Equal);
            let (diff, o) = Float::NEGATIVE_ZERO.sub_round_val_ref(&x, rm);
            assert_eq!(
                ComparableFloat(diff.abs_negative_zero()),
                ComparableFloat((-&x).abs_negative_zero())
            );
            assert_eq!(o, Ordering::Equal);
        }
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn sub_properties_helper_1<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let diff_1 = x - y;
        let diff_2 = emulate_primitive_float_fn_2(|x, y, prec| x.sub_prec(y, prec).0, x, y);
        assert_eq!(NiceFloat(diff_1), NiceFloat(diff_2));
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sub_properties_helper_2(x: Float, y: Float) {
    let diff = x.clone() - y.clone();
    assert!(diff.is_valid());
    let diff_alt = x.clone() - &y;
    assert!(diff_alt.is_valid());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = &x - y.clone();
    assert!(diff_alt.is_valid());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = &x - &y;
    assert!(diff_alt.is_valid());
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

    let mut x_alt = x.clone();
    x_alt -= y.clone();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));

    let mut x_alt = x.clone();
    x_alt -= &y;
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));

    let diff_alt = add_prec_round_naive(
        x.clone(),
        -&y,
        max(x.significant_bits(), y.significant_bits()),
        RoundingMode::Nearest,
    )
    .0;
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = x
        .sub_prec_round_ref_ref(
            &y,
            max(x.significant_bits(), y.significant_bits()),
            RoundingMode::Nearest,
        )
        .0;
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = x
        .sub_prec_ref_ref(&y, max(x.significant_bits(), y.significant_bits()))
        .0;
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    let diff_alt = x.sub_round_ref_ref(&y, RoundingMode::Nearest).0;
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

    if diff.is_finite() && x.is_normal() && y.is_normal() && diff.is_normal() {
        assert_eq!(
            diff.get_prec(),
            Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
        );
        let r_diff = Rational::exact_from(&x) - Rational::exact_from(&y);
        if diff < r_diff {
            let mut next = diff.clone();
            next.increment();
            assert!(next > r_diff);
        } else if diff > r_diff {
            let mut next = diff.clone();
            next.decrement();
            assert!(next < r_diff);
        }
    }

    let rug_diff = rug_sub(rug::Float::exact_from(&x), rug::Float::exact_from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_diff)),
        ComparableFloatRef(&diff),
    );

    let diff_alt = -(&y - &x);
    assert_eq!(
        ComparableFloat(diff_alt.abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero_ref())
    );

    let diff_alt = &x + -&y;
    assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

    let diff_alt = -(-&x + &y);
    assert_eq!(
        ComparableFloat(diff_alt.abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero_ref())
    );

    let diff_alt = -(-&x - -&y);
    assert_eq!(
        ComparableFloat(diff_alt.abs_negative_zero()),
        ComparableFloat(diff.abs_negative_zero())
    );
}

#[test]
fn sub_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    float_pair_gen_var_2().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    float_pair_gen_var_3().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    float_pair_gen_var_4().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    float_pair_gen_var_5().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    float_pair_gen_var_6().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    float_pair_gen_var_7().test_properties(|(x, y)| {
        sub_properties_helper_2(x, y);
    });

    apply_fn_to_primitive_floats!(sub_properties_helper_1);

    float_gen().test_properties(|x| {
        assert!((&x - Float::NAN).is_nan());
        assert!((Float::NAN - &x).is_nan());
        if !x.is_nan() {
            if x != Float::INFINITY {
                assert_eq!(&x - Float::INFINITY, Float::NEGATIVE_INFINITY);
                assert_eq!(Float::INFINITY - &x, Float::INFINITY);
            }
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(&x - Float::NEGATIVE_INFINITY, Float::INFINITY);
                assert_eq!(Float::NEGATIVE_INFINITY - &x, Float::NEGATIVE_INFINITY);
            }
        }
        assert_eq!(
            ComparableFloatRef(&(&x - Float::ZERO)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloat((-(Float::ZERO - &x)).abs_negative_zero()),
            ComparableFloat(x.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloat((&x - Float::NEGATIVE_ZERO).abs_negative_zero()),
            ComparableFloat(x.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloatRef(&-(Float::NEGATIVE_ZERO - &x)),
            ComparableFloatRef(&x)
        );
    });
}

#[test]
fn sub_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, y, prec, rm)| {
            let (diff, o) = x.clone().sub_rational_prec_round(y.clone(), prec, rm);
            assert!(diff.is_valid());
            let (diff_alt, o_alt) = x.clone().sub_rational_prec_round_val_ref(&y, prec, rm);
            assert!(diff_alt.is_valid());
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let (diff_alt, o_alt) = x.sub_rational_prec_round_ref_val(y.clone(), prec, rm);
            assert!(diff_alt.is_valid());
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
            let (diff_alt, o_alt) = x.sub_rational_prec_round_ref_ref(&y, prec, rm);
            assert!(diff_alt.is_valid());
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.sub_rational_prec_round_assign(y.clone(), prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.sub_rational_prec_round_assign_ref(&y, prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let (diff_alt, o_alt) = add_rational_prec_round_naive(x.clone(), -&y, prec, rm);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let r_diff = if diff.is_finite() {
                if diff.is_normal() {
                    assert_eq!(diff.get_prec(), Some(prec));
                }
                let r_diff = Rational::exact_from(&x) - &y;
                assert_eq!(diff.partial_cmp(&r_diff), Some(o));
                if o == Ordering::Less {
                    let mut next = diff.clone();
                    next.increment();
                    assert!(next > r_diff);
                } else if o == Ordering::Greater {
                    let mut next = diff.clone();
                    next.decrement();
                    assert!(next < r_diff);
                }
                Some(r_diff)
            } else {
                assert_eq!(o, Ordering::Equal);
                None
            };

            match (r_diff.is_some() && *r_diff.as_ref().unwrap() >= 0u32, rm) {
                (_, RoundingMode::Floor)
                | (true, RoundingMode::Down)
                | (false, RoundingMode::Up) => {
                    assert_ne!(o, Ordering::Greater)
                }
                (_, RoundingMode::Ceiling)
                | (true, RoundingMode::Up)
                | (false, RoundingMode::Down) => {
                    assert_ne!(o, Ordering::Less)
                }
                (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
                _ => {}
            }

            let (diff_alt, o_alt) = x.add_rational_prec_round_ref_val(-&y, prec, rm);
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let (mut diff_alt, mut o_alt) = (-&x).add_rational_prec_round_val_ref(&y, prec, -rm);
            diff_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(diff_alt.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);

            let (mut diff_alt, mut o_alt) = (-x).sub_rational_prec_round(-y, prec, -rm);
            diff_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(diff_alt.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero())
            );
            assert_eq!(o_alt, o);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_negative_zero() {
            let (diff, o) = x.sub_rational_prec_round_ref_val(Rational::ZERO, prec, rm);
            let mut diff_alt = x.clone();
            let o_alt = diff_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(diff), ComparableFloat(diff_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (diff, o) = Float::NAN.sub_rational_prec_round_val_ref(&x, prec, rm);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        assert_eq!(
            Float::INFINITY.sub_rational_prec_round_val_ref(&x, prec, rm),
            (Float::INFINITY, Ordering::Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.sub_rational_prec_round_val_ref(&x, prec, rm),
            (Float::NEGATIVE_INFINITY, Ordering::Equal)
        );

        let (diff, o) = Float::ZERO.sub_rational_prec_round_val_ref(&x, prec, rm);
        let (mut diff_alt, mut o_alt) = Float::from_rational_prec_round_ref(&x, prec, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff.abs_negative_zero()),
            ComparableFloat(diff_alt.abs_negative_zero())
        );
        assert_eq!(o, o_alt);

        let (diff, o) = Float::NEGATIVE_ZERO.sub_rational_prec_round_val_ref(&x, prec, rm);
        let (mut diff_alt, mut o_alt) = Float::from_rational_prec_round_ref(&x, prec, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(ComparableFloat(diff), ComparableFloat(diff_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn sub_rational_prec_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (diff, o) = x.clone().sub_rational_prec(y.clone(), prec);
        assert!(diff.is_valid());
        let (diff_alt, o_alt) = x.clone().sub_rational_prec_val_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_rational_prec_ref_val(y.clone(), prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_rational_prec_ref_ref(&y, prec);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_rational_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_rational_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), -&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let (diff_alt, o_alt) = x.sub_rational_prec_round_ref_ref(&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if diff.is_finite() {
            if diff.is_normal() {
                assert_eq!(diff.get_prec(), Some(prec));
            }
            let r_diff = Rational::exact_from(&x) - &y;
            assert_eq!(diff.partial_cmp(&r_diff), Some(o));
            if o == Ordering::Less {
                let mut next = diff.clone();
                next.increment();
                assert!(next > r_diff);
            } else if o == Ordering::Greater {
                let mut next = diff.clone();
                next.decrement();
                assert!(next < r_diff);
            }
        } else {
            assert_eq!(o, Ordering::Equal);
        }

        let (diff_alt, o_alt) = x.add_rational_prec_ref_val(-&y, prec);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        if (x != 0u32 && y != 0u32) || x.is_sign_positive() {
            let (mut diff_alt, mut o_alt) = (-&x).add_rational_prec_val_ref(&y, prec);
            diff_alt.neg_assign();
            diff_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);

            let (mut diff_alt, mut o_alt) = (-x).sub_rational_prec(-y, prec);
            diff_alt.neg_assign();
            diff_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if !x.is_negative_zero() {
            let (diff, o) = x.sub_rational_prec_ref_val(Rational::ZERO, prec);
            let mut diff_alt = x.clone();
            let o_alt = diff_alt.set_prec(prec);
            assert_eq!(ComparableFloat(diff), ComparableFloat(diff_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (diff, o) = Float::NAN.sub_rational_prec_val_ref(&x, prec);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        assert_eq!(
            Float::INFINITY.sub_rational_prec_val_ref(&x, prec),
            (Float::INFINITY, Ordering::Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.sub_rational_prec_val_ref(&x, prec),
            (Float::NEGATIVE_INFINITY, Ordering::Equal)
        );
        let (diff, o) = Float::ZERO.sub_rational_prec_val_ref(&x, prec);
        let (mut diff_alt, mut o_alt) = Float::from_rational_prec_ref(&x, prec);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff.abs_negative_zero()),
            ComparableFloat(diff_alt.abs_negative_zero())
        );
        assert_eq!(o, o_alt);

        let (diff, o) = Float::NEGATIVE_ZERO.sub_rational_prec_val_ref(&x, prec);
        let (mut diff_alt, mut o_alt) = Float::from_rational_prec_ref(&x, prec);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(ComparableFloat(diff), ComparableFloat(diff_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn sub_rational_round_properties() {
    float_rational_rounding_mode_triple_gen_var_2().test_properties(|(x, y, rm)| {
        let (diff, o) = x.clone().sub_rational_round(y.clone(), rm);
        assert!(diff.is_valid());
        let (diff_alt, o_alt) = x.clone().sub_rational_round_val_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let (diff_alt, o_alt) = x.sub_rational_round_ref_val(y.clone(), rm);
        assert!(diff_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let (diff_alt, o_alt) = x.sub_rational_round_ref_ref(&y, rm);
        assert!(diff_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_rational_round_assign(y.clone(), rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sub_rational_round_assign_ref(&y, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), -&y, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);
        let (diff_alt, o_alt) = x.sub_rational_prec_round_ref_ref(&y, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        assert_eq!(o_alt, o);

        let r_diff = if diff.is_finite() {
            if x.is_normal() && diff.is_normal() {
                assert_eq!(diff.get_prec(), Some(x.get_prec().unwrap()));
            }
            let r_diff = Rational::exact_from(&x) - &y;
            assert_eq!(diff.partial_cmp(&r_diff), Some(o));
            if o == Ordering::Less {
                let mut next = diff.clone();
                next.increment();
                assert!(next > r_diff);
            } else if o == Ordering::Greater {
                let mut next = diff.clone();
                next.decrement();
                assert!(next < r_diff);
            }
            Some(r_diff)
        } else {
            assert_eq!(o, Ordering::Equal);
            None
        };

        match (r_diff.is_some() && *r_diff.as_ref().unwrap() >= 0u32, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_diff, rug_o) = rug_sub_rational_round(
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_diff)),
                ComparableFloatRef(&diff)
            );
            assert_eq!(rug_o, o);
        }

        let (diff_alt, o_alt) = x.add_rational_round_ref_val(-&y, rm);
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        let (mut diff_alt, mut o_alt) = (-&x).add_rational_round_val_ref(&y, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(diff_alt.abs_negative_zero()),
            ComparableFloat(diff.abs_negative_zero_ref())
        );

        let (mut diff_alt, mut o_alt) = (-x).sub_rational_round(-y, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(diff_alt.abs_negative_zero()),
            ComparableFloat(diff.abs_negative_zero())
        );
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        if !x.is_negative_zero() {
            let (diff, o) = x.sub_rational_round_ref_val(Rational::ZERO, rm);
            assert_eq!(ComparableFloat(diff), ComparableFloat(x));
            assert_eq!(o, Ordering::Equal);
        }
    });

    rational_rounding_mode_pair_gen_var_6().test_properties(|(x, rm)| {
        let (diff, o) = Float::NAN.sub_rational_round_val_ref(&x, rm);
        assert!(diff.is_nan());
        assert_eq!(o, Ordering::Equal);

        assert_eq!(
            Float::INFINITY.sub_rational_round_val_ref(&x, rm),
            (Float::INFINITY, Ordering::Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.sub_rational_round_val_ref(&x, rm),
            (Float::NEGATIVE_INFINITY, Ordering::Equal)
        );
        let (diff, o) = Float::ZERO.sub_rational_round_val_ref(&x, rm);
        let (mut diff_alt, mut o_alt) = Float::from_rational_prec_round_ref(&x, 1, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(diff.abs_negative_zero()),
            ComparableFloat(diff_alt.abs_negative_zero())
        );
        assert_eq!(o, o_alt);

        let (diff, o) = Float::NEGATIVE_ZERO.sub_rational_round_val_ref(&x, rm);
        let (mut diff_alt, mut o_alt) = Float::from_rational_prec_round_ref(&x, 1, -rm);
        diff_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(ComparableFloatRef(&diff), ComparableFloatRef(&diff_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn sub_rational_properties() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        let diff = x.clone() - y.clone();
        assert!(diff.is_valid());
        let diff_alt = x.clone() - &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = &x - y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = &x - &y;
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        let diff_alt = -(y.clone() - x.clone());
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = -(y.clone() - &x);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = -(&y - x.clone());
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = -(&y - &x);
        assert!(diff_alt.is_valid());
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        let mut x_alt = x.clone();
        x_alt -= y.clone();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));

        let mut x_alt = x.clone();
        x_alt -= &y;
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&diff));

        let diff_alt = add_rational_prec_round_naive(
            x.clone(),
            -&y,
            x.significant_bits(),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = x
            .sub_rational_prec_round_ref_ref(&y, x.significant_bits(), RoundingMode::Nearest)
            .0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = x.sub_rational_prec_ref_ref(&y, x.significant_bits()).0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
        let diff_alt = x.sub_rational_round_ref_ref(&y, RoundingMode::Nearest).0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        if diff.is_finite() && x.is_normal() && diff.is_normal() {
            assert_eq!(diff.get_prec(), Some(x.get_prec().unwrap()));
            let r_diff = Rational::exact_from(&x) - &y;
            if diff < r_diff {
                let mut next = diff.clone();
                next.increment();
                assert!(next > r_diff);
            } else if diff > r_diff {
                let mut next = diff.clone();
                next.decrement();
                assert!(next < r_diff);
            }
        }

        let rug_diff = rug_sub_rational(rug::Float::exact_from(&x), rug::Rational::from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_diff)),
            ComparableFloatRef(&diff),
        );

        let diff_alt = &x + -&y;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));

        if (x != 0u32 && y != 0u32) || x.is_sign_positive() {
            let diff_alt = -(-&x + &y);
            assert_eq!(
                ComparableFloat(diff_alt.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );

            let diff_alt = -(-&x - -&y);
            assert_eq!(
                ComparableFloat(diff_alt.abs_negative_zero()),
                ComparableFloat(diff.abs_negative_zero_ref())
            );
        }

        let diff_alt = add_rational_prec_round_naive(
            x.clone(),
            -&y,
            x.significant_bits(),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&diff_alt), ComparableFloatRef(&diff));
    });

    float_gen().test_properties(|x| {
        assert_eq!(
            ComparableFloatRef(&(&x - Rational::ZERO)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&-(Rational::ZERO - &x)),
            ComparableFloatRef(&x)
        );
    });

    rational_gen().test_properties(|x| {
        assert!((&x - Float::NAN).is_nan());
        assert!((Float::NAN - &x).is_nan());
        assert_eq!(&x - Float::INFINITY, Float::NEGATIVE_INFINITY);
        assert_eq!(Float::INFINITY - &x, Float::INFINITY);
        assert_eq!(&x - Float::NEGATIVE_INFINITY, Float::INFINITY);
        assert_eq!(Float::NEGATIVE_INFINITY - &x, Float::NEGATIVE_INFINITY);
        let diff_alt = Float::from_rational_prec_ref(&x, 1).0;
        assert_eq!(
            ComparableFloat((&x - Float::ZERO).abs_negative_zero()),
            ComparableFloat(diff_alt.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloat((Float::ZERO - &x).abs_negative_zero()),
            ComparableFloat((-&diff_alt).abs_negative_zero())
        );
        assert_eq!(
            ComparableFloat(&x - Float::NEGATIVE_ZERO),
            ComparableFloat(diff_alt.clone())
        );
        assert_eq!(
            ComparableFloat(Float::NEGATIVE_ZERO - &x),
            ComparableFloat(-diff_alt)
        );
    });
}
