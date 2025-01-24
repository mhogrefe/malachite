// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::Infinity;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_5, signed_unsigned_pair_gen_var_19, signed_unsigned_pair_gen_var_20,
    unsigned_gen, unsigned_gen_var_5,
};
use malachite_float::test_util::arithmetic::power_of_2::{
    power_of_2_i64_naive, power_of_2_prec_naive, power_of_2_prec_round_naive, power_of_2_u64_naive,
};
use malachite_float::test_util::arithmetic::shl_round::rug_shl_prec_round;
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::{
    signed_unsigned_rounding_mode_triple_gen_var_5, signed_unsigned_rounding_mode_triple_gen_var_6,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_power_of_2_prec_round() {
    let test = |i: i64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out| {
        let (p, o) = Float::power_of_2_prec_round(i, prec, rm);
        assert!(p.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);

        let (p_alt, o_alt) = power_of_2_prec_round_naive(i, prec, rm);
        assert_eq!(p_alt.to_string(), out);
        assert_eq!(to_hex_string(&p_alt), out_hex);
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_p, o) = rug_shl_prec_round(&rug::Float::with_val(1, 1), i, prec, rm);
            let p = Float::exact_from(&rug_p);
            assert_eq!(p.to_string(), out);
            assert_eq!(to_hex_string(&p), out_hex);
            assert_eq!(o, o_out);
        }
    };
    test(0, 1, Floor, "1.0", "0x1.0#1", Equal);
    test(0, 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test(0, 1, Down, "1.0", "0x1.0#1", Equal);
    test(0, 1, Up, "1.0", "0x1.0#1", Equal);
    test(0, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(0, 1, Exact, "1.0", "0x1.0#1", Equal);

    test(
        0,
        100,
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        0,
        100,
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        0,
        100,
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        0,
        100,
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        0,
        100,
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        0,
        100,
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test(1, 1, Floor, "2.0", "0x2.0#1", Equal);
    test(1, 1, Ceiling, "2.0", "0x2.0#1", Equal);
    test(1, 1, Down, "2.0", "0x2.0#1", Equal);
    test(1, 1, Up, "2.0", "0x2.0#1", Equal);
    test(1, 1, Nearest, "2.0", "0x2.0#1", Equal);
    test(1, 1, Exact, "2.0", "0x2.0#1", Equal);

    test(
        1,
        100,
        Floor,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        1,
        100,
        Ceiling,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        1,
        100,
        Down,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        1,
        100,
        Up,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        1,
        100,
        Nearest,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );
    test(
        1,
        100,
        Exact,
        "2.0",
        "0x2.0000000000000000000000000#100",
        Equal,
    );

    test(100, 1, Floor, "1.0e30", "0x1.0E+25#1", Equal);
    test(100, 1, Ceiling, "1.0e30", "0x1.0E+25#1", Equal);
    test(100, 1, Down, "1.0e30", "0x1.0E+25#1", Equal);
    test(100, 1, Up, "1.0e30", "0x1.0E+25#1", Equal);
    test(100, 1, Nearest, "1.0e30", "0x1.0E+25#1", Equal);
    test(100, 1, Exact, "1.0e30", "0x1.0E+25#1", Equal);

    test(
        100,
        100,
        Floor,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );
    test(
        100,
        100,
        Ceiling,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );
    test(
        100,
        100,
        Down,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );
    test(
        100,
        100,
        Up,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );
    test(
        100,
        100,
        Nearest,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );
    test(
        100,
        100,
        Exact,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );

    test(1073741822, 1, Floor, "too_big", "0x4.0E+268435455#1", Equal);
    test(
        1073741822,
        1,
        Ceiling,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test(1073741822, 1, Down, "too_big", "0x4.0E+268435455#1", Equal);
    test(1073741822, 1, Up, "too_big", "0x4.0E+268435455#1", Equal);
    test(
        1073741822,
        1,
        Nearest,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test(1073741822, 1, Exact, "too_big", "0x4.0E+268435455#1", Equal);

    test(
        1073741822,
        100,
        Floor,
        "too_big",
        "0x4.0000000000000000000000000E+268435455#100",
        Equal,
    );
    test(
        1073741822,
        100,
        Ceiling,
        "too_big",
        "0x4.0000000000000000000000000E+268435455#100",
        Equal,
    );
    test(
        1073741822,
        100,
        Down,
        "too_big",
        "0x4.0000000000000000000000000E+268435455#100",
        Equal,
    );
    test(
        1073741822,
        100,
        Up,
        "too_big",
        "0x4.0000000000000000000000000E+268435455#100",
        Equal,
    );
    test(
        1073741822,
        100,
        Nearest,
        "too_big",
        "0x4.0000000000000000000000000E+268435455#100",
        Equal,
    );
    test(
        1073741822,
        100,
        Exact,
        "too_big",
        "0x4.0000000000000000000000000E+268435455#100",
        Equal,
    );

    test(1073741823, 1, Floor, "too_big", "0x4.0E+268435455#1", Less);
    test(1073741823, 1, Ceiling, "Infinity", "Infinity", Greater);
    test(1073741823, 1, Down, "too_big", "0x4.0E+268435455#1", Less);
    test(1073741823, 1, Up, "Infinity", "Infinity", Greater);
    test(1073741823, 1, Nearest, "Infinity", "Infinity", Greater);

    test(
        1073741823,
        100,
        Floor,
        "too_big",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Less,
    );
    test(1073741823, 100, Ceiling, "Infinity", "Infinity", Greater);
    test(
        1073741823,
        100,
        Down,
        "too_big",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Less,
    );
    test(1073741823, 100, Up, "Infinity", "Infinity", Greater);
    test(1073741823, 100, Nearest, "Infinity", "Infinity", Greater);

    test(-1, 1, Floor, "0.5", "0x0.8#1", Equal);
    test(-1, 1, Ceiling, "0.5", "0x0.8#1", Equal);
    test(-1, 1, Down, "0.5", "0x0.8#1", Equal);
    test(-1, 1, Up, "0.5", "0x0.8#1", Equal);
    test(-1, 1, Nearest, "0.5", "0x0.8#1", Equal);
    test(-1, 1, Exact, "0.5", "0x0.8#1", Equal);

    test(
        -1,
        100,
        Floor,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        -1,
        100,
        Ceiling,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        -1,
        100,
        Down,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        -1,
        100,
        Up,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        -1,
        100,
        Nearest,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        -1,
        100,
        Exact,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );

    test(-100, 1, Floor, "8.0e-31", "0x1.0E-25#1", Equal);
    test(-100, 1, Ceiling, "8.0e-31", "0x1.0E-25#1", Equal);
    test(-100, 1, Down, "8.0e-31", "0x1.0E-25#1", Equal);
    test(-100, 1, Up, "8.0e-31", "0x1.0E-25#1", Equal);
    test(-100, 1, Nearest, "8.0e-31", "0x1.0E-25#1", Equal);
    test(-100, 1, Exact, "8.0e-31", "0x1.0E-25#1", Equal);

    test(
        -1073741824,
        1,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test(
        -1073741824,
        1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test(
        -1073741824,
        1,
        Down,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test(-1073741824, 1, Up, "too_small", "0x1.0E-268435456#1", Equal);
    test(
        -1073741824,
        1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test(
        -1073741824,
        1,
        Exact,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );

    test(
        -1073741824,
        100,
        Floor,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Equal,
    );
    test(
        -1073741824,
        100,
        Ceiling,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Equal,
    );
    test(
        -1073741824,
        100,
        Down,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Equal,
    );
    test(
        -1073741824,
        100,
        Up,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Equal,
    );
    test(
        -1073741824,
        100,
        Nearest,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Equal,
    );
    test(
        -1073741824,
        100,
        Exact,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Equal,
    );

    test(-1073741825, 1, Floor, "0.0", "0x0.0", Less);
    test(
        -1073741825,
        1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(-1073741825, 1, Down, "0.0", "0x0.0", Less);
    test(
        -1073741825,
        1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(-1073741825, 1, Nearest, "0.0", "0x0.0", Less);

    test(-1073741825, 100, Floor, "0.0", "0x0.0", Less);
    test(
        -1073741825,
        100,
        Ceiling,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(-1073741825, 100, Down, "0.0", "0x0.0", Less);
    test(
        -1073741825,
        100,
        Up,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(-1073741825, 100, Nearest, "0.0", "0x0.0", Less);
}

#[test]
fn power_of_2_prec_round_fail() {
    assert_panic!(Float::power_of_2_prec_round(0, 0, Floor));
    assert_panic!(Float::power_of_2_prec_round(1073741823, 1, Exact));
    assert_panic!(Float::power_of_2_prec_round(1073741823, 100, Exact));
    assert_panic!(Float::power_of_2_prec_round(-1073741825, 1, Exact));
    assert_panic!(Float::power_of_2_prec_round(-1073741825, 100, Exact));
}

fn power_of_2_prec_round_properties_helper(i: i64, prec: u64, rm: RoundingMode) {
    let (p, o) = Float::power_of_2_prec_round(i, prec, rm);
    assert!(p.is_valid());
    assert!(p.is_finite() || p == Float::INFINITY || p.is_positive_zero());
    assert!(p >= 0u32);
    if p.is_normal() {
        assert_eq!(p.get_prec(), Some(prec));
    } else {
        assert_eq!(o, if p > 0 { Greater } else { Less });
    }

    if i.lt_abs(&1_000_000_000) {
        let (p_alt, o_alt) = power_of_2_prec_round_naive(i, prec, rm);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
    }

    if rm == Exact {
        assert_eq!(o, Equal);
    } else {
        let rm = rug_round_try_from_rounding_mode(rm).unwrap();
        if i32::convertible_from(i) {
            let (rug_p, rug_o) = rug_shl_prec_round(&rug::Float::with_val(1, 1), i, prec, rm);
            assert_eq!(
                ComparableFloat(Float::exact_from(&rug_p)),
                ComparableFloat(p)
            );
            assert_eq!(rug_o, o);
        }
    }
}

#[test]
fn power_of_2_prec_round_properties() {
    signed_unsigned_rounding_mode_triple_gen_var_5().test_properties(|(i, prec, rm)| {
        power_of_2_prec_round_properties_helper(i, prec, rm);
    });

    signed_unsigned_rounding_mode_triple_gen_var_6().test_properties(|(i, prec, rm)| {
        power_of_2_prec_round_properties_helper(i, prec, rm);
    });
}

#[test]
fn test_power_of_2_prec() {
    let test = |i: i64, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let (p, o) = Float::power_of_2_prec(i, prec);
        assert!(p.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);

        let (p_alt, o_alt) = power_of_2_prec_naive(i, prec);
        assert_eq!(p_alt.to_string(), out);
        assert_eq!(to_hex_string(&p_alt), out_hex);
        assert_eq!(o_alt, o_out);

        let rug_p = rug::Float::with_val(u32::exact_from(prec), 1) << i32::exact_from(i);
        let p = Float::exact_from(&rug_p);
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
    };
    test(0, 1, "1.0", "0x1.0#1", Equal);
    test(0, 100, "1.0", "0x1.0000000000000000000000000#100", Equal);
    test(1, 1, "2.0", "0x2.0#1", Equal);
    test(1, 100, "2.0", "0x2.0000000000000000000000000#100", Equal);
    test(100, 1, "1.0e30", "0x1.0E+25#1", Equal);
    test(
        100,
        100,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Equal,
    );
    test(1073741822, 1, "too_big", "0x4.0E+268435455#1", Equal);
    test(1073741823, 1, "Infinity", "Infinity", Greater);

    test(-1, 1, "0.5", "0x0.8#1", Equal);
    test(-1, 100, "0.5", "0x0.8000000000000000000000000#100", Equal);
    test(-100, 1, "8.0e-31", "0x1.0E-25#1", Equal);
    test(
        -100,
        100,
        "7.88860905221011805411728565283e-31",
        "0x1.0000000000000000000000000E-25#100",
        Equal,
    );
    test(-1073741824, 1, "too_small", "0x1.0E-268435456#1", Equal);
    test(-1073741825, 1, "0.0", "0x0.0", Less);
}

#[test]
#[should_panic]
fn power_of_2_prec_fail() {
    Float::power_of_2_prec(0, 0);
}

fn power_of_2_prec_properties_helper(i: i64, prec: u64) {
    let (p, o) = Float::power_of_2_prec(i, prec);
    assert!(p.is_valid());
    assert!(p.is_finite() || p == Float::INFINITY || p.is_positive_zero());
    assert!(p >= 0u32);
    assert_eq!(Float::power_of_2(i), p);
    let (p_alt, o_alt) = Float::power_of_2_prec_round(i, prec, Nearest);
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o, o_alt);
    if p.is_normal() {
        assert_eq!(o, Equal);
        assert_eq!(p.get_prec(), Some(prec));
        assert!(p.is_power_of_2());
    } else {
        assert_eq!(o, if p > 0 { Greater } else { Less });
    }

    if i.lt_abs(&1_000_000_000) {
        let (p_alt, o_alt) = power_of_2_prec_naive(i, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (p_alt, o_alt) = Float::power_of_2_prec_round(i, prec, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, Equal);
        }
    }
    if let Ok(i) = i32::try_from(i) {
        let rug_p = rug::Float::with_val(u32::exact_from(prec), 1) << i;
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    }
}

#[test]
fn power_of_2_prec_properties() {
    signed_unsigned_pair_gen_var_19::<i64, u64>().test_properties(|(i, prec)| {
        power_of_2_prec_properties_helper(i, prec);
    });

    signed_unsigned_pair_gen_var_20::<i64, u64>().test_properties(|(i, prec)| {
        power_of_2_prec_properties_helper(i, prec);
    });
}

#[test]
fn test_power_of_2() {
    let test = |i: i64, out: &str, out_hex: &str| {
        let p = Float::power_of_2(i);
        assert!(p.is_valid());

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);

        let p_alt = power_of_2_i64_naive(i);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let rug_p = rug::Float::with_val(1, 1) << i32::exact_from(i);
        let p = Float::exact_from(&rug_p);
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
    };
    test(0, "1.0", "0x1.0#1");
    test(1, "2.0", "0x2.0#1");
    test(2, "4.0", "0x4.0#1");
    test(3, "8.0", "0x8.0#1");
    test(1073741822, "too_big", "0x4.0E+268435455#1");
    test(1073741823, "Infinity", "Infinity");

    test(-1, "0.5", "0x0.8#1");
    test(-2, "0.2", "0x0.4#1");
    test(-3, "0.1", "0x0.2#1");
    test(-100, "8.0e-31", "0x1.0E-25#1");
    test(-1073741824, "too_small", "0x1.0E-268435456#1");
    test(-1073741825, "0.0", "0x0.0");

    let test = |u: u64, out: &str, out_hex: &str| {
        let p = Float::power_of_2(u);
        assert!(p.is_valid());

        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);

        let p_alt = power_of_2_u64_naive(u);
        assert_eq!(p_alt.to_string(), out);
        assert_eq!(to_hex_string(&p_alt), out_hex);

        let rug_p = rug::Float::with_val(1, 1) << u32::exact_from(u);
        let p = Float::exact_from(&rug_p);
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
    };
    test(0, "1.0", "0x1.0#1");
    test(1, "2.0", "0x2.0#1");
    test(2, "4.0", "0x4.0#1");
    test(3, "8.0", "0x8.0#1");
    test(100, "1.0e30", "0x1.0E+25#1");
    test(1073741822, "too_big", "0x4.0E+268435455#1");
    test(1073741823, "Infinity", "Infinity");
}

fn power_of_2_properties_signed_helper(i: i64) {
    let p = Float::power_of_2(i);
    assert!(p.is_valid());
    assert!(p.is_finite() || p == Float::INFINITY || p.is_positive_zero());
    assert!(p >= 0u32);
    if p.is_normal() {
        assert!(p.is_power_of_2());
        assert_eq!(p.get_prec(), Some(1));
    }
    assert_eq!(
        ComparableFloatRef(&Float::power_of_2_prec(i, 1).0),
        ComparableFloatRef(&p)
    );
    if i >= 0 {
        assert_eq!(
            ComparableFloatRef(&Float::power_of_2(i.unsigned_abs())),
            ComparableFloatRef(&p)
        );
    }

    if i.lt_abs(&1_000_000_000) {
        let p_alt = power_of_2_i64_naive(i);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    }

    if let Ok(i) = u32::try_from(i) {
        let rug_p = rug::Float::with_val(1, 1) << i;
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    }
}

fn power_of_2_properties_unsigned_helper(u: u64) {
    let p = Float::power_of_2(u);
    assert!(p.is_valid());
    assert!(p.is_finite() || p == Float::INFINITY || p.is_positive_zero());
    assert!(p >= 0u32);
    if p.is_normal() {
        assert!(p.is_power_of_2());
        assert_eq!(p.get_prec(), Some(1));
    }

    if u < 1_000_000_000 {
        let p_alt = power_of_2_u64_naive(u);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    }

    if let Ok(u) = u32::try_from(u) {
        let rug_p = rug::Float::with_val(1, 1) << u;
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    }
}

#[test]
fn power_of_2_properties() {
    signed_gen_var_5().test_properties(|i| {
        power_of_2_properties_signed_helper(i);
    });

    signed_gen().test_properties(|i| {
        power_of_2_properties_signed_helper(i);
    });

    unsigned_gen_var_5().test_properties(|u| {
        power_of_2_properties_unsigned_helper(u);
    });

    unsigned_gen().test_properties(|u| {
        power_of_2_properties_unsigned_helper(u);
    });
}
