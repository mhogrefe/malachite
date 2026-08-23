// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use malachite_base::num::arithmetic::traits::{NegAssign, PowerOf2};
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::float::arithmetic::product::{
    naive_product, naive_product_prec, naive_product_prec_round, naive_product_round,
};
use malachite_float::test_util::generators::{
    float_vec_gen, float_vec_gen_var_1, float_vec_rounding_mode_pair_gen_var_3,
    float_vec_rounding_mode_pair_gen_var_4, float_vec_unsigned_pair_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_3,
    float_vec_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn parse_hex_strings(xs_hex: &[&str]) -> Vec<Float> {
    xs_hex.iter().copied().map(parse_hex_string).collect()
}

#[test]
fn test_product() {
    let test = |xs_hex: &[&str], out: &str, out_hex: &str| {
        let xs = parse_hex_strings(xs_hex);

        let product = Float::product(xs.iter().cloned());
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);

        let product_alt = Float::product(xs.iter());
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        let product_alt = naive_product(&xs);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
    };
    test(&[], "1.0", "0x1.0#1");
    test(&["NaN"], "NaN", "NaN");
    test(&["0x1.0#1", "0x2.0#1", "0x3.0#2"], "6.0", "0x6.0#2");
    test(
        &[
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
        ],
        "1.0",
        "0x1.0#1",
    );
    test(
        &[
            "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2",
            "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2",
            "0x3.0#2", "0x3.0#2", "0x3.0#2", "0x3.0#2",
        ],
        "3.2e9",
        "0xc.0E+7#2",
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        "-14.959966287406718723278821172461",
        "-0xe.f5c059c1aca50e8b8e828902#100",
    );
}

#[test]
fn test_product_prec() {
    let test = |xs_hex: &[&str], prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);

        let (product, o) = Float::product_prec(&xs, prec);
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);
        assert_eq!(o, o_out);

        let (product_alt, o_alt) = naive_product_prec(&xs, prec);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
    };
    test(&[], 10, "1.0000", "0x1.000#10", Equal);
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        30,
        "-14.959966287",
        "-0xe.f5c059c#30",
        Greater,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        10,
        "105.00",
        "0x69.0#10",
        Equal,
    );
}

#[test]
fn product_prec_fail() {
    assert_panic!(Float::product_prec(&[Float::from(3), Float::from(4)], 0));
    assert_panic!(Float::product_prec_round(&[], 0, Floor));
}

#[test]
fn test_product_round() {
    let test = |xs_hex: &[&str], rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);

        let (product, o) = Float::product_round(&xs, rm);
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);
        assert_eq!(o, o_out);

        let (product_alt, o_alt) = naive_product_round(&xs, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
    };
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        Floor,
        "-14.959966287406718723278821172474",
        "-0xe.f5c059c1aca50e8b8e828903#100",
        Less,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        Ceiling,
        "-14.959966287406718723278821172461",
        "-0xe.f5c059c1aca50e8b8e828902#100",
        Greater,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        Down,
        "-14.959966287406718723278821172461",
        "-0xe.f5c059c1aca50e8b8e828902#100",
        Greater,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        Up,
        "-14.959966287406718723278821172474",
        "-0xe.f5c059c1aca50e8b8e828903#100",
        Less,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        Nearest,
        "-14.959966287406718723278821172461",
        "-0xe.f5c059c1aca50e8b8e828902#100",
        Greater,
    );
    // - an exact product under the Exact rounding mode
    test(
        &["0x3.00#10", "0x5.0#3", "0x7.0#3"],
        Exact,
        "105.00",
        "0x69.0#10",
        Equal,
    );
}

#[test]
fn product_round_fail() {
    assert_panic!(Float::product_round(
        &[Float::from(3), Float::from(5), Float::from(7)],
        Exact
    ));
}

#[test]
fn test_product_prec_round() {
    let test = |xs_hex: &[&str],
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);

        let (product, o) = Float::product_prec_round(&xs, prec, rm);
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        assert_eq!(to_hex_string(&product), out_hex);
        assert_eq!(o, o_out);

        let (product_alt, o_alt) = naive_product_prec_round(&xs, prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
    };
    // - the singular rules: any NaN, or a zero times an infinity, gives NaN; infinities otherwise
    //   give an infinity whose sign is the XOR of all the input signs
    test(
        &["NaN", "0x1.0#1", "0x2.0#1"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["Infinity", "0x2.0#1", "Infinity"],
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        &["Infinity", "0x2.0#1", "-Infinity"],
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        &["Infinity", "0x0.0", "0x1.0#1"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["-Infinity", "-0x2.0#1", "-Infinity"],
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    // - zero results: the sign is the XOR of all the input signs
    test(
        &["0x0.0", "0x1.0#1", "-0x2.0#1"],
        5,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        &["-0x0.0", "-0x1.0#1", "-0x2.0#1"],
        5,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        &["-0x0.0", "-0x0.0", "0x3.0#2"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    // - delegation paths: one or two inputs
    test(&["0x3.0#2"], 10, Nearest, "3.0000", "0x3.00#10", Equal);
    test(
        &["0x3.0#2", "0x5.0#3"],
        10,
        Nearest,
        "15.000",
        "0xf.00#10",
        Equal,
    );
    // - the exact path: products whose odd parts are small, including under the Exact rounding mode
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        10,
        Nearest,
        "105.00",
        "0x69.0#10",
        Equal,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        10,
        Exact,
        "105.00",
        "0x69.0#10",
        Equal,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        3,
        Floor,
        "96.0",
        "0x6.0E+1#3",
        Less,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        3,
        Ceiling,
        "1.1e2",
        "0x7.0E+1#3",
        Greater,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        3,
        Down,
        "96.0",
        "0x6.0E+1#3",
        Less,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        3,
        Up,
        "1.1e2",
        "0x7.0E+1#3",
        Greater,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x7.0#3"],
        3,
        Nearest,
        "1.1e2",
        "0x7.0E+1#3",
        Greater,
    );
    // - inputs that are all powers of 2 have trivial odd parts
    test(
        &["0x4.0#1", "0x0.08#1", "0x2.0#1"],
        5,
        Nearest,
        "0.250",
        "0x0.40#5",
        Equal,
    );
    test(
        &["0x4.0#1", "0x0.08#1", "0x2.0#1"],
        5,
        Exact,
        "0.250",
        "0x0.40#5",
        Equal,
    );
    // - exponent bookkeeping through intermediates far outside the i32 exponent range
    test(
        &["0x1.0E+250000000#1", "0x2.0E-250000000#1", "0x3.0#2"],
        5,
        Nearest,
        "6.00",
        "0x6.0#5",
        Equal,
    );
    test(
        &["0x1.0E+250000000#1", "0x2.0E-250000000#1", "0x3.0#2"],
        5,
        Exact,
        "6.00",
        "0x6.0#5",
        Equal,
    );
    // - the truncated Ziv path
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        20,
        Floor,
        "-14.959976",
        "-0xe.f5c1#20",
        Less,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        20,
        Ceiling,
        "-14.959961",
        "-0xe.f5c0#20",
        Greater,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        20,
        Down,
        "-14.959961",
        "-0xe.f5c0#20",
        Greater,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        20,
        Up,
        "-14.959976",
        "-0xe.f5c1#20",
        Less,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        20,
        Nearest,
        "-14.959961",
        "-0xe.f5c0#20",
        Greater,
    );
    // - near-boundary factors of the form 1 ± 2^-k
    test(
        &["0x1.00001000#30", "0x0.ffffffff80#40", "0x3.0#2", "0x1.000000000000000000000040#95"],
        64,
        Floor,
        "3.00000286067370292020",
        "0x3.00002ffe7fffe800#64",
        Less,
    );
    test(
        &["0x1.00001000#30", "0x0.ffffffff80#40", "0x3.0#2", "0x1.000000000000000000000040#95"],
        64,
        Ceiling,
        "3.00000286067370292042",
        "0x3.00002ffe7fffe804#64",
        Greater,
    );
    test(
        &["0x1.00001000#30", "0x0.ffffffff80#40", "0x3.0#2", "0x1.000000000000000000000040#95"],
        64,
        Down,
        "3.00000286067370292020",
        "0x3.00002ffe7fffe800#64",
        Less,
    );
    test(
        &["0x1.00001000#30", "0x0.ffffffff80#40", "0x3.0#2", "0x1.000000000000000000000040#95"],
        64,
        Up,
        "3.00000286067370292042",
        "0x3.00002ffe7fffe804#64",
        Greater,
    );
    test(
        &["0x1.00001000#30", "0x0.ffffffff80#40", "0x3.0#2", "0x1.000000000000000000000040#95"],
        64,
        Nearest,
        "3.00000286067370292020",
        "0x3.00002ffe7fffe800#64",
        Less,
    );
    // - overflow and underflow saturation, in both signs
    test(
        &["0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Floor,
        "2.03e323228496",
        "0x7.cE+268435455#5",
        Less,
    );
    test(
        &["0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Down,
        "2.03e323228496",
        "0x7.cE+268435455#5",
        Less,
    );
    test(
        &["0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["-0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["-0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Ceiling,
        "-2.03e323228496",
        "-0x7.cE+268435455#5",
        Greater,
    );
    test(
        &["-0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Down,
        "-2.03e323228496",
        "-0x7.cE+268435455#5",
        Greater,
    );
    test(
        &["-0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["-0x2.0E+268435455#1", "0x2.0E+268435455#1", "0x2.0#1"],
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Ceiling,
        "2.38e-323228497",
        "0x1.0E-268435456#5",
        Greater,
    );
    test(
        &["0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Up,
        "2.38e-323228497",
        "0x1.0E-268435456#5",
        Greater,
    );
    test(
        &["0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["-0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Floor,
        "-2.38e-323228497",
        "-0x1.0E-268435456#5",
        Less,
    );
    test(
        &["-0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        &["-0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        &["-0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Up,
        "-2.38e-323228497",
        "-0x1.0E-268435456#5",
        Less,
    );
    test(
        &["-0x1.0E-268435456#1", "0x1.0E-268435456#1", "0x2.0#1"],
        5,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    // - long input lists exercise the i128 exponent accumulation
    test(
        &[
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
            "0x2.0E+268435455#1",
        ],
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &[
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
            "0x1.0E-268435456#1",
        ],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // Step-4 branch-coverage rows.
    // - truncations collapsing the accumulator onto an exactly representable value: can_round can
    //   never accept these, and the one-sided bump path decides them
    test(
        &[
            "0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Floor,
        "3.00",
        "0x3.0#5",
        Less,
    );
    test(
        &[
            "0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Down,
        "3.00",
        "0x3.0#5",
        Less,
    );
    test(
        &[
            "0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Nearest,
        "3.00",
        "0x3.0#5",
        Less,
    );
    test(
        &[
            "0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Ceiling,
        "3.12",
        "0x3.2#5",
        Greater,
    );
    test(
        &[
            "0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Up,
        "3.12",
        "0x3.2#5",
        Greater,
    );
    // - the same shape with a negative product
    test(
        &[
            "-0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Ceiling,
        "-3.00",
        "-0x3.0#5",
        Greater,
    );
    test(
        &[
            "-0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Up,
        "-3.12",
        "-0x3.2#5",
        Less,
    );
    test(
        &[
            "-0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Nearest,
        "-3.00",
        "-0x3.0#5",
        Greater,
    );
    test(
        &[
            "-0x1.00000000000000000000000000000000000000000000000001#201",
            "0x1.000000000000000000000000000000000000000000000000000000000000000000000000001#301",
            "0x3.0#2",
        ],
        5,
        Floor,
        "-3.12",
        "-0x3.2#5",
        Less,
    );
    // - a rounding carry in the exact path: 31 rounds up to 32 = 2^5
    test(
        &["0x1f.0#5", "0x1.0#1", "0x1.0#1"],
        4,
        Up,
        "32.0",
        "0x20.0#4",
        Greater,
    );
    test(
        &["0x1f.0#5", "0x1.0#1", "0x1.0#1"],
        4,
        Nearest,
        "32.0",
        "0x20.0#4",
        Greater,
    );
    test(
        &["0x1f.0#5", "0x1.0#1", "0x1.0#1"],
        4,
        Floor,
        "30.0",
        "0x1e.0#4",
        Less,
    );
    // - Ziv-path overflow and underflow saturation, in both signs
    test(
        &["0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Floor,
        "2.07e323228496",
        "0x7.eE+268435455#6",
        Less,
    );
    test(
        &["0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Down,
        "2.07e323228496",
        "0x7.eE+268435455#6",
        Less,
    );
    test(
        &["0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["-0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["-0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Ceiling,
        "-2.07e323228496",
        "-0x7.eE+268435455#6",
        Greater,
    );
    test(
        &["-0x5.5555555555555558E+268435449#64", "0x5.5555555555555558E+268435449#64", "0x3.0#2"],
        6,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Ceiling,
        "2.38e-323228497",
        "0x1.00E-268435456#6",
        Greater,
    );
    test(
        &["0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Up,
        "2.38e-323228497",
        "0x1.00E-268435456#6",
        Greater,
    );
    test(
        &["0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["-0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Floor,
        "-2.38e-323228497",
        "-0x1.00E-268435456#6",
        Less,
    );
    test(
        &["-0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        &["-0x5.5555555555555558E-268435451#64", "0x5.5555555555555558E-268435451#64", "0x3.0#2"],
        6,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    // - a zero times a negative infinity
    test(
        &["-Infinity", "0x0.0", "0x2.0#1"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
fn product_prec_round_fail() {
    assert_panic!(Float::product_prec_round(&[Float::from(3)], 0, Floor));
    assert_panic!(Float::product_prec_round(
        &[Float::from(3), Float::from(5), Float::from(7)],
        5,
        Exact
    ));
}

#[test]
fn product_prec_round_extreme_lengths() {
    // Over a thousand maximal exponents overflow the +-2^40 shift clamp, which saturates
    // identically to any other out-of-range exponent.
    let xs = vec![Float::power_of_2(1073741822i64); 1100];
    let (p, o) = Float::product_prec_round(&xs, 5, Nearest);
    assert_eq!(p, Float::INFINITY);
    assert_eq!(o, Greater);
    let (p_alt, o_alt) = naive_product_prec_round(&xs, 5, Nearest);
    assert_eq!(ComparableFloat(p_alt), ComparableFloat(p));
    assert_eq!(o_alt, o);
    let (p, o) = Float::product_prec_round(&xs, 5, Down);
    assert_eq!(p.to_string(), "2.03e323228496");
    assert_eq!(to_hex_string(&p), "0x7.cE+268435455#5");
    assert_eq!(o, Less);
    let (p_alt, o_alt) = naive_product_prec_round(&xs, 5, Down);
    assert_eq!(ComparableFloat(p_alt), ComparableFloat(p));
    assert_eq!(o_alt, o);
    let xs = vec![Float::power_of_2(-1073741824i64); 1100];
    let (p, o) = Float::product_prec_round(&xs, 5, Nearest);
    assert_eq!(ComparableFloat(p.clone()), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (p_alt, o_alt) = naive_product_prec_round(&xs, 5, Nearest);
    assert_eq!(ComparableFloat(p_alt), ComparableFloat(p));
    assert_eq!(o_alt, o);
    let (p, o) = Float::product_prec_round(&xs, 5, Up);
    assert_eq!(p.to_string(), "2.38e-323228497");
    assert_eq!(to_hex_string(&p), "0x1.0E-268435456#5");
    assert_eq!(o, Greater);
    let (p_alt, o_alt) = naive_product_prec_round(&xs, 5, Up);
    assert_eq!(ComparableFloat(p_alt), ComparableFloat(p));
    assert_eq!(o_alt, o);
}

const EXPONENT_GATE: i64 = 1 << 16;

fn in_gate(x: &Float) -> bool {
    x.get_exponent()
        .is_none_or(|e| i64::from(e).abs() < EXPONENT_GATE)
}

#[allow(clippy::needless_pass_by_value)]
fn product_prec_round_properties_helper(xs: Vec<Float>, prec: u64, rm: RoundingMode) {
    let (product, o) = Float::product_prec_round(&xs, prec, rm);
    assert!(product.is_valid());

    // reversal invariance
    let reversed: Vec<Float> = xs.iter().rev().cloned().collect();
    let (product_alt, o_alt) = Float::product_prec_round(&reversed, prec, rm);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    // the naive exact-accumulation oracle; since it keeps its intermediate values normalized, it
    // needs no exponent gate, even for extreme inputs
    let (product_alt, o_alt) = naive_product_prec_round(&xs, prec, rm);
    assert_eq!(
        ComparableFloat(product_alt),
        ComparableFloat(product.clone())
    );
    assert_eq!(o_alt, o);

    // the singular rules
    let any_nan = xs.iter().any(Float::is_nan);
    let any_inf = xs.iter().any(Float::is_infinite);
    let any_zero = xs.iter().any(|x| *x == 0u32);
    if any_nan || (any_inf && any_zero) {
        assert!(product.is_nan());
        assert_eq!(o, Equal);
        return;
    }
    let positive = xs
        .iter()
        .filter(|x| x.is_sign_negative())
        .count()
        .is_multiple_of(2);
    if any_inf {
        assert_eq!(
            product,
            if positive {
                Float::INFINITY
            } else {
                Float::NEGATIVE_INFINITY
            }
        );
        assert_eq!(o, Equal);
        return;
    }
    if any_zero {
        assert_eq!(product, Float::ZERO);
        assert_ne!(product.is_sign_negative(), positive);
        assert_eq!(o, Equal);
        return;
    }

    // appending a 1 changes nothing
    let mut with_one = xs.clone();
    with_one.push(Float::ONE);
    let (product_alt, o_alt) = Float::product_prec_round(&with_one, prec, rm);
    assert_eq!(
        ComparableFloatRef(&product_alt),
        ComparableFloatRef(&product)
    );
    assert_eq!(o_alt, o);

    // negating one input negates the product, with the rounding mode reflected
    if !xs.is_empty() {
        let mut negated = xs.clone();
        negated[0].neg_assign();
        let (product_alt, o_alt) = Float::product_prec_round(&negated, prec, -rm);
        assert_eq!(
            ComparableFloat(product_alt),
            ComparableFloat(-product.clone())
        );
        assert_eq!(o_alt, o.reverse());
    }

    // consistency with the delegations
    if xs.len() == 1 {
        let (product_alt, o_alt) = Float::from_float_prec_round_ref(&xs[0], prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
    } else if xs.len() == 2 {
        let (product_alt, o_alt) = xs[0].mul_prec_round_ref_ref(&xs[1], prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);
    }

    if xs.iter().all(in_gate) {
        // the complete exact-Rational oracle
        let exact = Rational::product(xs.iter().map(Rational::exact_from));
        let (product_alt, o_alt) = Float::from_rational_prec_round(exact, prec, rm);
        assert_eq!(
            ComparableFloat(product_alt),
            ComparableFloat(product.clone())
        );
        assert_eq!(o_alt, o);

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (p, oo) = Float::product_prec_round(&xs, prec, rm);
                assert_eq!(ComparableFloat(p), ComparableFloat(product.clone()));
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(Float::product_prec_round(&xs, prec, Exact));
        }
    }
}

#[test]
fn product_prec_round_properties() {
    float_vec_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(xs, prec, rm)| {
        product_prec_round_properties_helper(xs, prec, rm);
    });

    float_vec_unsigned_rounding_mode_triple_gen_var_4().test_properties(|(xs, prec, rm)| {
        product_prec_round_properties_helper(xs, prec, rm);
    });
}

#[test]
fn product_prec_properties() {
    float_vec_unsigned_pair_gen_var_1().test_properties(|(xs, prec)| {
        let (product, o) = Float::product_prec(&xs, prec);
        assert!(product.is_valid());
        let (product_alt, o_alt) = Float::product_prec_round(&xs, prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = naive_product_prec(&xs, prec);
        assert_eq!(
            ComparableFloat(product_alt),
            ComparableFloat(product.clone())
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn product_round_properties() {
    let helper = |xs: &[Float], rm: RoundingMode| {
        let (product, o) = Float::product_round(xs, rm);
        assert!(product.is_valid());
        let prec = xs
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        let (product_alt, o_alt) = Float::product_prec_round(xs, prec, rm);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );
        assert_eq!(o_alt, o);

        let (product_alt, o_alt) = naive_product_round(xs, rm);
        assert_eq!(
            ComparableFloat(product_alt),
            ComparableFloat(product.clone())
        );
        assert_eq!(o_alt, o);
    };
    float_vec_rounding_mode_pair_gen_var_3().test_properties(|(xs, rm)| {
        helper(&xs, rm);
    });

    float_vec_rounding_mode_pair_gen_var_4().test_properties(|(xs, rm)| {
        helper(&xs, rm);
    });
}

#[test]
fn product_properties() {
    let helper = |xs: Vec<Float>| {
        let product = Float::product(xs.iter().cloned());
        assert!(product.is_valid());
        let product_alt = Float::product(xs.iter());
        assert!(product_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&product),
            ComparableFloatRef(&product_alt)
        );

        let prec = xs
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        let (product_alt, o) = Float::product_prec_round(&xs, prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&product_alt),
            ComparableFloatRef(&product)
        );

        let product_alt = naive_product(&xs);
        assert_eq!(
            ComparableFloat(product_alt),
            ComparableFloat(product.clone())
        );

        // a product of no Floats, or of all-positive exactly-representable Floats, is exact
        if xs.is_empty() {
            assert_eq!(o, Equal);
        }
    };
    float_vec_gen().test_properties(helper);
    float_vec_gen_var_1().test_properties(helper);
}
