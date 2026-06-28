// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::GelfondsConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_gelfonds_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::gelfonds_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gelfonds_constant_prec() {
    test_gelfonds_constant_prec_helper(1, "2.0e1", "0x1.0E+1#1", Less);
    test_gelfonds_constant_prec_helper(2, "24.0", "0x18.0#2", Greater);
    test_gelfonds_constant_prec_helper(3, "24.0", "0x18.0#3", Greater);
    test_gelfonds_constant_prec_helper(4, "24.0", "0x18.0#4", Greater);
    test_gelfonds_constant_prec_helper(5, "23.0", "0x17.0#5", Less);
    test_gelfonds_constant_prec_helper(6, "23.0", "0x17.0#6", Less);
    test_gelfonds_constant_prec_helper(7, "23.2", "0x17.4#7", Greater);
    test_gelfonds_constant_prec_helper(8, "23.1", "0x17.2#8", Less);
    test_gelfonds_constant_prec_helper(9, "23.12", "0x17.2#9", Less);
    test_gelfonds_constant_prec_helper(10, "23.16", "0x17.28#10", Greater);
    test_gelfonds_constant_prec_helper(
        100,
        "23.14069263277926900572908636794",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );

    let gelfonds_constant_f32 = Float::gelfonds_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(gelfonds_constant_f32.to_string(), "23.140692");
    assert_eq!(to_hex_string(&gelfonds_constant_f32), "0x17.24046#24");
    assert_eq!(gelfonds_constant_f32, f32::GELFONDS_CONSTANT);

    let gelfonds_constant_f64 = Float::gelfonds_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(gelfonds_constant_f64.to_string(), "23.14069263277927");
    assert_eq!(
        to_hex_string(&gelfonds_constant_f64),
        "0x17.24046eb0933a#53"
    );
    assert_eq!(gelfonds_constant_f64, f64::GELFONDS_CONSTANT);
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_fail_1() {
    Float::gelfonds_constant_prec(0);
}

fn test_gelfonds_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::gelfonds_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gelfonds_constant_prec_round() {
    test_gelfonds_constant_prec_round_helper(1, Floor, "2.0e1", "0x1.0E+1#1", Less);
    test_gelfonds_constant_prec_round_helper(1, Ceiling, "3.0e1", "0x2.0E+1#1", Greater);
    test_gelfonds_constant_prec_round_helper(1, Down, "2.0e1", "0x1.0E+1#1", Less);
    test_gelfonds_constant_prec_round_helper(1, Up, "3.0e1", "0x2.0E+1#1", Greater);
    test_gelfonds_constant_prec_round_helper(1, Nearest, "2.0e1", "0x1.0E+1#1", Less);

    test_gelfonds_constant_prec_round_helper(2, Floor, "16.0", "0x10.0#2", Less);
    test_gelfonds_constant_prec_round_helper(2, Ceiling, "24.0", "0x18.0#2", Greater);
    test_gelfonds_constant_prec_round_helper(2, Down, "16.0", "0x10.0#2", Less);
    test_gelfonds_constant_prec_round_helper(2, Up, "24.0", "0x18.0#2", Greater);
    test_gelfonds_constant_prec_round_helper(2, Nearest, "24.0", "0x18.0#2", Greater);

    test_gelfonds_constant_prec_round_helper(3, Floor, "20.0", "0x14.0#3", Less);
    test_gelfonds_constant_prec_round_helper(3, Ceiling, "24.0", "0x18.0#3", Greater);
    test_gelfonds_constant_prec_round_helper(3, Down, "20.0", "0x14.0#3", Less);
    test_gelfonds_constant_prec_round_helper(3, Up, "24.0", "0x18.0#3", Greater);
    test_gelfonds_constant_prec_round_helper(3, Nearest, "24.0", "0x18.0#3", Greater);

    test_gelfonds_constant_prec_round_helper(4, Floor, "22.0", "0x16.0#4", Less);
    test_gelfonds_constant_prec_round_helper(4, Ceiling, "24.0", "0x18.0#4", Greater);
    test_gelfonds_constant_prec_round_helper(4, Down, "22.0", "0x16.0#4", Less);
    test_gelfonds_constant_prec_round_helper(4, Up, "24.0", "0x18.0#4", Greater);
    test_gelfonds_constant_prec_round_helper(4, Nearest, "24.0", "0x18.0#4", Greater);

    test_gelfonds_constant_prec_round_helper(5, Floor, "23.0", "0x17.0#5", Less);
    test_gelfonds_constant_prec_round_helper(5, Ceiling, "24.0", "0x18.0#5", Greater);
    test_gelfonds_constant_prec_round_helper(5, Down, "23.0", "0x17.0#5", Less);
    test_gelfonds_constant_prec_round_helper(5, Up, "24.0", "0x18.0#5", Greater);
    test_gelfonds_constant_prec_round_helper(5, Nearest, "23.0", "0x17.0#5", Less);

    test_gelfonds_constant_prec_round_helper(6, Floor, "23.0", "0x17.0#6", Less);
    test_gelfonds_constant_prec_round_helper(6, Ceiling, "23.5", "0x17.8#6", Greater);
    test_gelfonds_constant_prec_round_helper(6, Down, "23.0", "0x17.0#6", Less);
    test_gelfonds_constant_prec_round_helper(6, Up, "23.5", "0x17.8#6", Greater);
    test_gelfonds_constant_prec_round_helper(6, Nearest, "23.0", "0x17.0#6", Less);

    test_gelfonds_constant_prec_round_helper(7, Floor, "23.0", "0x17.0#7", Less);
    test_gelfonds_constant_prec_round_helper(7, Ceiling, "23.2", "0x17.4#7", Greater);
    test_gelfonds_constant_prec_round_helper(7, Down, "23.0", "0x17.0#7", Less);
    test_gelfonds_constant_prec_round_helper(7, Up, "23.2", "0x17.4#7", Greater);
    test_gelfonds_constant_prec_round_helper(7, Nearest, "23.2", "0x17.4#7", Greater);

    test_gelfonds_constant_prec_round_helper(8, Floor, "23.1", "0x17.2#8", Less);
    test_gelfonds_constant_prec_round_helper(8, Ceiling, "23.2", "0x17.4#8", Greater);
    test_gelfonds_constant_prec_round_helper(8, Down, "23.1", "0x17.2#8", Less);
    test_gelfonds_constant_prec_round_helper(8, Up, "23.2", "0x17.4#8", Greater);
    test_gelfonds_constant_prec_round_helper(8, Nearest, "23.1", "0x17.2#8", Less);

    test_gelfonds_constant_prec_round_helper(9, Floor, "23.12", "0x17.2#9", Less);
    test_gelfonds_constant_prec_round_helper(9, Ceiling, "23.19", "0x17.3#9", Greater);
    test_gelfonds_constant_prec_round_helper(9, Down, "23.12", "0x17.2#9", Less);
    test_gelfonds_constant_prec_round_helper(9, Up, "23.19", "0x17.3#9", Greater);
    test_gelfonds_constant_prec_round_helper(9, Nearest, "23.12", "0x17.2#9", Less);

    test_gelfonds_constant_prec_round_helper(10, Floor, "23.12", "0x17.20#10", Less);
    test_gelfonds_constant_prec_round_helper(10, Ceiling, "23.16", "0x17.28#10", Greater);
    test_gelfonds_constant_prec_round_helper(10, Down, "23.12", "0x17.20#10", Less);
    test_gelfonds_constant_prec_round_helper(10, Up, "23.16", "0x17.28#10", Greater);
    test_gelfonds_constant_prec_round_helper(10, Nearest, "23.16", "0x17.28#10", Greater);

    test_gelfonds_constant_prec_round_helper(
        100,
        Floor,
        "23.14069263277926900572908636794",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Ceiling,
        "23.14069263277926900572908636796",
        "0x17.24046eb093399ecda7489f9c#100",
        Greater,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Down,
        "23.14069263277926900572908636794",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Up,
        "23.14069263277926900572908636796",
        "0x17.24046eb093399ecda7489f9c#100",
        Greater,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Nearest,
        "23.14069263277926900572908636794",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_round_fail_1() {
    Float::gelfonds_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_round_fail_2() {
    Float::gelfonds_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_round_fail_3() {
    Float::gelfonds_constant_prec_round(1000, Exact);
}

#[test]
fn gelfonds_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (gelfonds_constant, o) = Float::gelfonds_constant_prec(prec);
        assert!(gelfonds_constant.is_valid());
        assert_eq!(gelfonds_constant.get_prec(), Some(prec));
        assert_eq!(gelfonds_constant.get_exponent(), Some(5));
        assert_ne!(o, Equal);
        if o == Less {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Ceiling);
            let mut next_upper = gelfonds_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gelfonds_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gelfonds_constant.is_power_of_2() {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Floor);
            let mut next_lower = gelfonds_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gelfonds_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&gelfonds_constant_alt),
            ComparableFloatRef(&gelfonds_constant)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn gelfonds_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (gelfonds_constant, o) = Float::gelfonds_constant_prec_round(prec, rm);
        assert!(gelfonds_constant.is_valid());
        assert_eq!(gelfonds_constant.get_prec(), Some(prec));
        // e^pi is in [16, 32), so the result has exponent 5 unless it rounds up to 32.
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up) => 6,
            _ => 5,
        };
        assert_eq!(gelfonds_constant.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Ceiling);
            let mut next_upper = gelfonds_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gelfonds_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gelfonds_constant.is_power_of_2() {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Floor);
            let mut next_lower = gelfonds_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gelfonds_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::gelfonds_constant_prec_round(prec, Exact));
    });

    test_constant(Float::gelfonds_constant_prec_round, 10000);
}
