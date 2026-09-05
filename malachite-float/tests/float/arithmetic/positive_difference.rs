// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::assert_panic;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{primitive_float_gen, primitive_float_pair_gen};
use malachite_float::float::arithmetic::positive_difference::{
    primitive_float_positive_difference, primitive_float_positive_difference_rational,
    primitive_float_rational_positive_difference_float,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::positive_difference::*;
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_42,
    float_float_unsigned_rounding_mode_quadruple_gen_var_22,
    float_float_unsigned_rounding_mode_quadruple_gen_var_23, float_float_unsigned_triple_gen_var_1,
    float_pair_gen, float_pair_gen_var_10, float_rational_pair_gen,
    float_rational_rounding_mode_triple_gen_var_22, float_rational_rounding_mode_triple_gen_var_23,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_23,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_24,
    float_rational_unsigned_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

#[allow(clippy::needless_pass_by_value)]
fn positive_difference_prec_round_properties_helper(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (d, o) = x.positive_difference_prec_round_ref_ref(&y, prec, rm);
    assert!(d.is_valid());
    let (d2, o2) = x
        .clone()
        .positive_difference_prec_round(y.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
    assert_eq!(o2, o);
    let (d2, o2) = x
        .clone()
        .positive_difference_prec_round_val_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
    assert_eq!(o2, o);
    let (d2, o2) = x.positive_difference_prec_round_ref_val(y.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
    assert_eq!(o2, o);
    let mut x2 = x.clone();
    let o2 = x2.positive_difference_prec_round_assign(y.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
    assert_eq!(o2, o);
    let mut x2 = x.clone();
    let o2 = x2.positive_difference_prec_round_assign_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
    assert_eq!(o2, o);

    if d.is_normal() {
        assert_eq!(d.get_prec(), Some(prec));
    }

    // the definition: x - y when x > y, +0 when x <= y, NaN when incomparable
    match x.partial_cmp(&y) {
        Some(Greater) => {
            let (expected, expected_o) = x.sub_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(ComparableFloatRef(&d), ComparableFloatRef(&expected));
            assert_eq!(o, expected_o);
            if !extreme {
                assert!(d > 0u32);
            }
        }
        Some(_) => {
            assert_eq!(ComparableFloat(d.clone()), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
        }
        None => {
            assert!(d.is_nan());
            assert_eq!(o, Equal);
        }
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_d, rug_o) = rug_positive_difference_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_d)),
            ComparableFloatRef(&d)
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.positive_difference_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(d.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.positive_difference_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn positive_difference_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_22().test_properties(
        |(x, y, prec, rm)| {
            positive_difference_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );

    float_float_unsigned_rounding_mode_quadruple_gen_var_23().test_properties(
        |(x, y, prec, rm)| {
            positive_difference_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );
}

#[test]
fn positive_difference_shorthand_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (d, o) = x.positive_difference_prec_round_ref_ref(&y, prec, Nearest);
        let (d2, o2) = x.positive_difference_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.clone().positive_difference_prec(y.clone(), prec);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_prec_assign(y.clone(), prec);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_prec_assign_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    });

    float_float_rounding_mode_triple_gen_var_42().test_properties(|(x, y, rm)| {
        let prec = max(x.significant_bits(), y.significant_bits());
        let (d, o) = x.positive_difference_prec_round_ref_ref(&y, prec, rm);
        let (d2, o2) = x.positive_difference_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.clone().positive_difference_round(y.clone(), rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.clone().positive_difference_round_val_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.positive_difference_round_ref_val(y.clone(), rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_round_assign(y.clone(), rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_round_assign_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    });

    float_pair_gen().test_properties(|(x, y)| {
        let (d, o) = x.positive_difference_ref_ref(&y);
        let (d2, o2) = x.positive_difference_round_ref_ref(&y, Nearest);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.clone().positive_difference(y.clone());
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.clone().positive_difference_val_ref(&y);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.positive_difference_ref_val(y.clone());
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_assign(y.clone());
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_assign_ref(&y);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        // dim(x, y) or dim(y, x) is zero (or NaN); both are zero only when x == y
        let (e, _) = y.positive_difference_ref_ref(&x);
        if !d.is_nan() {
            assert!(d == 0u32 || e == 0u32);
        }
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        let (d, o) = x.positive_difference_ref_ref(&y);
        let (d2, o2) = x.positive_difference_round_ref_ref(&y, Nearest);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    });
}

// The emulated primitive-float positive difference: for finite operands it is `x - y` when `x > y`
// (which the primitive subtraction computes exactly rounded) and a positive zero otherwise; NaN
// inputs give NaN.
#[test]
fn primitive_float_positive_difference_properties() {
    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        let d = primitive_float_positive_difference(x, y);
        if x.is_nan() || y.is_nan() {
            assert!(d.is_nan());
        } else if x > y {
            assert_eq!(NiceFloat(d), NiceFloat(x - y));
        } else {
            assert_eq!(NiceFloat(d), NiceFloat(0.0));
        }
    });

    primitive_float_pair_gen::<f32>().test_properties(|(x, y)| {
        let d = primitive_float_positive_difference(x, y);
        if x.is_nan() || y.is_nan() {
            assert!(d.is_nan());
        } else if x > y {
            assert_eq!(NiceFloat(d), NiceFloat(x - y));
        } else {
            assert_eq!(NiceFloat(d), NiceFloat(0.0));
        }
    });
}

#[test]
fn positive_difference_fail() {
    assert_panic!(Float::from(3u32).positive_difference_prec_round(Float::ONE, 0, Nearest));
    assert_panic!(Float::from(3u32).positive_difference_prec_round_ref_ref(
        &Float::ONE,
        0,
        Nearest
    ));
    assert_panic!(Float::from(3u32).positive_difference_prec(Float::ONE, 0));
    // Exact with an inexact difference
    assert_panic!(parse_hex_string("0x3.0#2").positive_difference_prec_round(
        parse_hex_string("0x0.8#1"),
        1,
        Exact
    ));
}

#[test]
fn test_positive_difference() {
    let test =
        |s, s_hex, t, t_hex, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let y = parse_hex_string(t_hex);
            assert_eq!(y.to_string(), t);

            let (d, o) = x.positive_difference_prec_round_ref_ref(&y, prec, rm);
            assert!(d.is_valid());
            assert_eq!(d.to_string(), out);
            assert_eq!(to_hex_string(&d), out_hex);
            assert_eq!(o, o_out);

            let (d2, o2) = x
                .clone()
                .positive_difference_prec_round(y.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
            assert_eq!(o2, o);
            let mut x2 = x.clone();
            let o2 = x2.positive_difference_prec_round_assign(y.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
            assert_eq!(o2, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_d, rug_o) = rug_positive_difference_prec_round(
                    &rug::Float::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_d)),
                    ComparableFloatRef(&d)
                );
                assert_eq!(rug_o, o);
            }
        };
    // - either operand NaN: NaN
    test(
        "NaN", "NaN", "1.0", "0x1.0#1", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal,
    );
    // - x <= y: a positive zero, even for equal infinities, equal values, and zero pairs of either
    //   sign order
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "Infinity", "Infinity", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "1.0", "0x1.0#1", "3.0", "0x3.0#2", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "3.0", "0x3.0#2", "3.0", "0x3.0#2", 10, Nearest, "0.0", "0x0.0", Equal,
    );
    // - x > y: the difference, exactly as sub computes it
    test(
        "Infinity", "Infinity", "1.0", "0x1.0#1", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-Infinity",
        "-Infinity",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-3.0",
        "-0x3.0#2",
        10,
        Nearest,
        "4.0000",
        "0x4.00#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        "-3.0",
        "-0x3.0#2",
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    // - an inexact difference under each rounding direction, and Exact when representable
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Floor, "2.0", "0x2.0#1", Less,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Ceiling, "4.0", "0x4.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 1, Nearest, "4.0", "0x4.0#1", Greater,
    );
    test(
        "10.0", "0xa.0#3", "7.0", "0x7.0#3", 2, Exact, "3.0", "0x3.0#2", Equal,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn positive_difference_rational_prec_round_properties_helper(
    x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
    reversed: bool,
) {
    type F = fn(&Float, &Rational, u64, RoundingMode) -> (Float, Ordering);
    let f: F = if reversed {
        |x, y, prec, rm| {
            Float::rational_positive_difference_float_prec_round_ref_ref(y, x, prec, rm)
        }
    } else {
        Float::positive_difference_rational_prec_round_ref_ref
    };
    let (d, o) = f(&x, &y, prec, rm);
    assert!(d.is_valid());

    if reversed {
        let (d2, o2) =
            Float::rational_positive_difference_float_prec_round(y.clone(), x.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) =
            Float::rational_positive_difference_float_prec_round_val_ref(y.clone(), &x, prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) =
            Float::rational_positive_difference_float_prec_round_ref_val(&y, x.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    } else {
        let (d2, o2) = x
            .clone()
            .positive_difference_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x
            .clone()
            .positive_difference_rational_prec_round_val_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.positive_difference_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_rational_prec_round_assign(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_rational_prec_round_assign_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    }

    if d.is_normal() {
        assert_eq!(d.get_prec(), Some(prec));
    }

    // the definition: the exact difference when the first operand is larger, a positive zero
    // otherwise, NaN for NaN
    let c = if reversed {
        x.partial_cmp(&y).map(Ordering::reverse)
    } else {
        x.partial_cmp(&y)
    };
    match c {
        None => {
            assert!(d.is_nan());
            assert_eq!(o, Equal);
        }
        Some(Greater) => {
            if reversed {
                let (expected, expected_o) = {
                    let (s, so) = x.sub_rational_prec_round_ref_ref(&y, prec, -rm);
                    (-s, so.reverse())
                };
                assert_eq!(ComparableFloatRef(&d), ComparableFloatRef(&expected));
                assert_eq!(o, expected_o);
            } else {
                let (expected, expected_o) = x.sub_rational_prec_round_ref_ref(&y, prec, rm);
                assert_eq!(ComparableFloatRef(&d), ComparableFloatRef(&expected));
                assert_eq!(o, expected_o);
            }
        }
        Some(_) => {
            assert_eq!(ComparableFloat(d.clone()), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
        }
    }

    if o == Equal {
        for rm2 in exhaustive_rounding_modes() {
            let (s, oo) = f(&x, &y, prec, rm2);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(d.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(f(&x, &y, prec, Exact));
    }
}

#[test]
fn positive_difference_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_23().test_properties(
        |(x, y, prec, rm)| {
            positive_difference_rational_prec_round_properties_helper(x, y, prec, rm, false);
        },
    );
}

#[test]
fn rational_positive_difference_float_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_24().test_properties(
        |(x, y, prec, rm)| {
            positive_difference_rational_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );
}

#[test]
fn positive_difference_rational_shorthand_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (d, o) = x.positive_difference_rational_prec_round_ref_ref(&y, prec, Nearest);
        let (d2, o2) = x.positive_difference_rational_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_rational_prec_assign_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d, o) =
            Float::rational_positive_difference_float_prec_round_ref_ref(&y, &x, prec, Nearest);
        let (d2, o2) = Float::rational_positive_difference_float_prec_ref_ref(&y, &x, prec);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_22().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (d, o) = x.positive_difference_rational_prec_round_ref_ref(&y, prec, rm);
        let (d2, o2) = x.positive_difference_rational_round_ref_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_rational_round_assign_ref(&y, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    });

    float_rational_rounding_mode_triple_gen_var_23().test_properties(|(x, y, rm)| {
        let prec = x.significant_bits();
        let (d, o) = Float::rational_positive_difference_float_prec_round_ref_ref(&y, &x, prec, rm);
        let (d2, o2) = Float::rational_positive_difference_float_round_ref_ref(&y, &x, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
    });

    float_rational_pair_gen().test_properties(|(x, y)| {
        let (d, o) = x.positive_difference_rational_ref_ref(&y);
        let (d2, o2) = x.positive_difference_rational_round_ref_ref(&y, Nearest);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (d2, o2) = x.clone().positive_difference_rational(y.clone());
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_rational_assign_ref(&y);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let (r, ro) = Float::rational_positive_difference_float_ref_ref(&y, &x);
        let (r2, ro2) = Float::rational_positive_difference_float_round_ref_ref(&y, &x, Nearest);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(ro2, ro);
        // at most one direction is positive; with a dyadic Rational both agree with the Float-Float
        // function
        if !d.is_nan() {
            assert!(d == 0u32 || r == 0u32);
        }
        // with an exactly convertible Rational, the mixed function agrees with the Float-Float
        // function at the same precision
        if let Ok(yf) = Float::try_from(y.clone()) {
            let prec = x.significant_bits();
            let (d2, o2) = x.positive_difference_rational_prec_round_ref_ref(&y, prec, Nearest);
            let (expected, expected_o) =
                x.positive_difference_prec_round_ref_val(yf, prec, Nearest);
            assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&expected));
            assert_eq!(o2, expected_o);
        }
    });
}

#[test]
fn primitive_float_positive_difference_rational_properties() {
    primitive_float_gen::<f64>().test_properties(|x| {
        for y in [
            Rational::from_signeds(22i64, 7i64),
            Rational::from_signeds(-22i64, 7i64),
            Rational::from_signeds(1i64, 3i64) << 200u32,
        ] {
            let d = primitive_float_positive_difference_rational(x, &y);
            let rev = primitive_float_rational_positive_difference_float(&y, x);
            if x.is_nan() {
                assert!(d.is_nan());
                assert!(rev.is_nan());
            } else {
                match x.partial_cmp(&y).unwrap() {
                    Greater => {
                        assert!(d > 0.0);
                        assert_eq!(NiceFloat(rev), NiceFloat(0.0));
                    }
                    Less => {
                        assert_eq!(NiceFloat(d), NiceFloat(0.0));
                        assert!(rev > 0.0);
                    }
                    Equal => {
                        assert_eq!(NiceFloat(d), NiceFloat(0.0));
                        assert_eq!(NiceFloat(rev), NiceFloat(0.0));
                    }
                }
            }
        }
    });
}

#[test]
fn positive_difference_rational_fail() {
    assert_panic!(Float::from(3u32).positive_difference_rational_prec_round(
        Rational::from_signeds(1i32, 3i32),
        0,
        Nearest
    ));
    assert_panic!(Float::rational_positive_difference_float_prec_round(
        Rational::from_signeds(1i32, 3i32),
        Float::from(3u32),
        0,
        Nearest
    ));
    // Exact with an inexact difference
    assert_panic!(Float::from(3u32).positive_difference_rational_prec_round(
        Rational::from_signeds(1i32, 3i32),
        2,
        Exact
    ));
}

#[test]
fn test_positive_difference_rational() {
    let test = |s,
                s_hex,
                t: &str,
                prec,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering,
                rev_out: &str,
                rev_hex: &str,
                rev_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = t.parse::<Rational>().unwrap();

        let (d, o) = x.positive_difference_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!(to_hex_string(&d), out_hex);
        assert_eq!(o, o_out);
        let (d2, o2) = x
            .clone()
            .positive_difference_rational_prec_round(y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&d2), ComparableFloatRef(&d));
        assert_eq!(o2, o);
        let mut x2 = x.clone();
        let o2 = x2.positive_difference_rational_prec_round_assign_ref(&y, prec, rm);
        assert_eq!(ComparableFloatRef(&x2), ComparableFloatRef(&d));
        assert_eq!(o2, o);

        let (r, ro) =
            Float::rational_positive_difference_float_prec_round_ref_ref(&y, &x, prec, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), rev_out);
        assert_eq!(to_hex_string(&r), rev_hex);
        assert_eq!(ro, rev_o);
        let (r2, ro2) =
            Float::rational_positive_difference_float_prec_round(y.clone(), x.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&r2), ComparableFloatRef(&r));
        assert_eq!(ro2, ro);
    };
    // - a NaN Float is NaN in both directions
    test(
        "NaN", "NaN", "1/3", 10, Nearest, "NaN", "NaN", Equal, "NaN", "NaN", Equal,
    );
    // - infinities compare exactly; the smaller side is a positive zero
    test(
        "Infinity", "Infinity", "1/3", 10, Nearest, "Infinity", "Infinity", Equal, "0.0", "0x0.0",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1/3",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - zero ties (of either Float sign, against the unsigned Rational zero): positive zeros
    test(
        "0.0", "0x0.0", "0", 10, Nearest, "0.0", "0x0.0", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0", 10, Nearest, "0.0", "0x0.0", Equal, "0.0", "0x0.0", Equal,
    );
    // - the exact difference on the winning side, correctly rounded; zero on the other
    test(
        "3.0",
        "0x3.0#2",
        "1/3",
        10,
        Nearest,
        "2.6680",
        "0x2.ab#10",
        Greater,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "3.0", "0x3.0#2", "22/7", 2, Floor, "0.0", "0x0.0", Equal, "0.12", "0x0.2#2", Less,
    );
    test(
        "3.0", "0x3.0#2", "22/7", 2, Ceiling, "0.0", "0x0.0", Equal, "0.19", "0x0.3#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "22/7",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
        "2.1445",
        "0x2.25#10",
        Greater,
    );
    // - negative operands on both sides
    test(
        "-1.0",
        "-0x1.0#1",
        "-22/7",
        10,
        Nearest,
        "2.1445",
        "0x2.25#10",
        Greater,
        "0.0",
        "0x0.0",
        Equal,
    );
    // - equal values: positive zeros both ways
    test(
        "1.0", "0x1.0#1", "1", 10, Nearest, "0.0", "0x0.0", Equal, "0.0", "0x0.0", Equal,
    );
    test(
        "4.0", "0x4.0#1", "22/7", 4, Nearest, "0.875", "0x0.e#4", Greater, "0.0", "0x0.0", Equal,
    );
}
