// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{integer_gen, integer_unsigned_pair_gen_var_6};
use malachite_q::Rational;
use std::cmp::{max, Ordering::*};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_from_integer() {
    let test = |s, out, out_hex| {
        let u = Integer::from_str(s).unwrap();

        let x = Float::from(u.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let x = Float::from(&u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let rug_x = rug::Float::with_val(
            max(1, u32::exact_from(u.significant_bits())),
            rug::Integer::from(&u),
        );
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test("0", "0.0", "0x0.0");
    test("1", "1.0", "0x1.0#1");
    test("123", "123.0", "0x7b.0#7");
    test("1000000000000", "1000000000000.0", "0xe8d4a51000.0#40");
    test("-1", "-1.0", "-0x1.0#1");
    test("-123", "-123.0", "-0x7b.0#7");
    test("-1000000000000", "-1000000000000.0", "-0xe8d4a51000.0#40");
}

#[test]
fn test_from_integer_prec() {
    let test = |s, prec, out, out_hex, out_o| {
        let u = Integer::from_str(s).unwrap();

        let (x, o) = Float::from_integer_prec(u.clone(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_integer_prec_ref(&u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Integer::from(&u));
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 20, "0.0", "0x0.0", Equal);

    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0", "0x1.000#10", Equal);
    test("1", 20, "1.0", "0x1.00000#20", Equal);

    test("123", 1, "1.0e2", "0x8.0E+1#1", Greater);
    test("123", 10, "123.0", "0x7b.0#10", Equal);
    test("123", 20, "123.0", "0x7b.0000#20", Equal);

    test("1000000000000", 1, "1.0e12", "0x1.0E+10#1", Greater);
    test("1000000000000", 10, "9.997e11", "0xe.8cE+9#10", Less);
    test("1000000000000", 20, "9.999997e11", "0xe.8d4aE+9#20", Less);

    test("-1", 1, "-1.0", "-0x1.0#1", Equal);
    test("-1", 10, "-1.0", "-0x1.000#10", Equal);
    test("-1", 20, "-1.0", "-0x1.00000#20", Equal);

    test("-123", 1, "-1.0e2", "-0x8.0E+1#1", Less);
    test("-123", 10, "-123.0", "-0x7b.0#10", Equal);
    test("-123", 20, "-123.0", "-0x7b.0000#20", Equal);

    test("-1000000000000", 1, "-1.0e12", "-0x1.0E+10#1", Less);
    test("-1000000000000", 10, "-9.997e11", "-0xe.8cE+9#10", Greater);
    test(
        "-1000000000000",
        20,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
}

#[test]
fn from_integer_prec_fail() {
    assert_panic!(Float::from_integer_prec(Integer::ZERO, 0));
    assert_panic!(Float::from_integer_prec(Integer::ONE, 0));
    assert_panic!(Float::from_integer_prec(Integer::NEGATIVE_ONE, 0));
}

#[test]
fn from_integer_prec_ref_fail() {
    assert_panic!(Float::from_integer_prec_ref(&Integer::ZERO, 0));
    assert_panic!(Float::from_integer_prec_ref(&Integer::ONE, 0));
    assert_panic!(Float::from_integer_prec_ref(&Integer::NEGATIVE_ONE, 0));
}

#[test]
fn test_from_integer_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Integer::from_str(s).unwrap();

        let (x, o) = Float::from_integer_prec_round(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_integer_prec_round_ref(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Integer::from(&u), rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(rug_o, out_o);
        }
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);

    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);

    test("0", 20, Floor, "0.0", "0x0.0", Equal);
    test("0", 20, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 20, Down, "0.0", "0x0.0", Equal);
    test("0", 20, Up, "0.0", "0x0.0", Equal);
    test("0", 20, Nearest, "0.0", "0x0.0", Equal);
    test("0", 20, Exact, "0.0", "0x0.0", Equal);

    test("1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test("1", 20, Floor, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Ceiling, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Down, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Up, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Nearest, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Exact, "1.0", "0x1.00000#20", Equal);

    test("123", 1, Floor, "6.0e1", "0x4.0E+1#1", Less);
    test("123", 1, Ceiling, "1.0e2", "0x8.0E+1#1", Greater);
    test("123", 1, Down, "6.0e1", "0x4.0E+1#1", Less);
    test("123", 1, Up, "1.0e2", "0x8.0E+1#1", Greater);
    test("123", 1, Nearest, "1.0e2", "0x8.0E+1#1", Greater);

    test("123", 10, Floor, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Ceiling, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Down, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Up, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Nearest, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Exact, "123.0", "0x7b.0#10", Equal);

    test("123", 20, Floor, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Ceiling, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Down, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Up, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Nearest, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Exact, "123.0", "0x7b.0000#20", Equal);

    test("1000000000000", 1, Floor, "5.0e11", "0x8.0E+9#1", Less);
    test(
        "1000000000000",
        1,
        Ceiling,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );
    test("1000000000000", 1, Down, "5.0e11", "0x8.0E+9#1", Less);
    test("1000000000000", 1, Up, "1.0e12", "0x1.0E+10#1", Greater);
    test(
        "1000000000000",
        1,
        Nearest,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );

    test("1000000000000", 10, Floor, "9.997e11", "0xe.8cE+9#10", Less);
    test(
        "1000000000000",
        10,
        Ceiling,
        "1.001e12",
        "0xe.90E+9#10",
        Greater,
    );
    test("1000000000000", 10, Down, "9.997e11", "0xe.8cE+9#10", Less);
    test("1000000000000", 10, Up, "1.001e12", "0xe.90E+9#10", Greater);
    test(
        "1000000000000",
        10,
        Nearest,
        "9.997e11",
        "0xe.8cE+9#10",
        Less,
    );

    test(
        "1000000000000",
        20,
        Floor,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test(
        "1000000000000",
        20,
        Ceiling,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test(
        "1000000000000",
        20,
        Down,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test(
        "1000000000000",
        20,
        Up,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test(
        "1000000000000",
        20,
        Nearest,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );

    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Down, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Up, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test("-1", 10, Floor, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Ceiling, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Down, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Up, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Nearest, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Exact, "-1.0", "-0x1.000#10", Equal);

    test("-1", 20, Floor, "-1.0", "-0x1.00000#20", Equal);
    test("-1", 20, Ceiling, "-1.0", "-0x1.00000#20", Equal);
    test("-1", 20, Down, "-1.0", "-0x1.00000#20", Equal);
    test("-1", 20, Up, "-1.0", "-0x1.00000#20", Equal);
    test("-1", 20, Nearest, "-1.0", "-0x1.00000#20", Equal);
    test("-1", 20, Exact, "-1.0", "-0x1.00000#20", Equal);

    test("-123", 1, Floor, "-1.0e2", "-0x8.0E+1#1", Less);
    test("-123", 1, Ceiling, "-6.0e1", "-0x4.0E+1#1", Greater);
    test("-123", 1, Down, "-6.0e1", "-0x4.0E+1#1", Greater);
    test("-123", 1, Up, "-1.0e2", "-0x8.0E+1#1", Less);
    test("-123", 1, Nearest, "-1.0e2", "-0x8.0E+1#1", Less);

    test("-123", 10, Floor, "-123.0", "-0x7b.0#10", Equal);
    test("-123", 10, Ceiling, "-123.0", "-0x7b.0#10", Equal);
    test("-123", 10, Down, "-123.0", "-0x7b.0#10", Equal);
    test("-123", 10, Up, "-123.0", "-0x7b.0#10", Equal);
    test("-123", 10, Nearest, "-123.0", "-0x7b.0#10", Equal);
    test("-123", 10, Exact, "-123.0", "-0x7b.0#10", Equal);

    test("-123", 20, Floor, "-123.0", "-0x7b.0000#20", Equal);
    test("-123", 20, Ceiling, "-123.0", "-0x7b.0000#20", Equal);
    test("-123", 20, Down, "-123.0", "-0x7b.0000#20", Equal);
    test("-123", 20, Up, "-123.0", "-0x7b.0000#20", Equal);
    test("-123", 20, Nearest, "-123.0", "-0x7b.0000#20", Equal);
    test("-123", 20, Exact, "-123.0", "-0x7b.0000#20", Equal);

    test("-1000000000000", 1, Floor, "-1.0e12", "-0x1.0E+10#1", Less);
    test(
        "-1000000000000",
        1,
        Ceiling,
        "-5.0e11",
        "-0x8.0E+9#1",
        Greater,
    );
    test("-1000000000000", 1, Down, "-5.0e11", "-0x8.0E+9#1", Greater);
    test("-1000000000000", 1, Up, "-1.0e12", "-0x1.0E+10#1", Less);
    test(
        "-1000000000000",
        1,
        Nearest,
        "-1.0e12",
        "-0x1.0E+10#1",
        Less,
    );

    test(
        "-1000000000000",
        10,
        Floor,
        "-1.001e12",
        "-0xe.90E+9#10",
        Less,
    );
    test(
        "-1000000000000",
        10,
        Ceiling,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Greater,
    );
    test(
        "-1000000000000",
        10,
        Down,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Greater,
    );
    test("-1000000000000", 10, Up, "-1.001e12", "-0xe.90E+9#10", Less);
    test(
        "-1000000000000",
        10,
        Nearest,
        "-9.997e11",
        "-0xe.8cE+9#10",
        Greater,
    );

    test(
        "-1000000000000",
        20,
        Floor,
        "-1.000001e12",
        "-0xe.8d4bE+9#20",
        Less,
    );
    test(
        "-1000000000000",
        20,
        Ceiling,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
    test(
        "-1000000000000",
        20,
        Down,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
    test(
        "-1000000000000",
        20,
        Up,
        "-1.000001e12",
        "-0xe.8d4bE+9#20",
        Less,
    );
    test(
        "-1000000000000",
        20,
        Nearest,
        "-9.999997e11",
        "-0xe.8d4aE+9#20",
        Greater,
    );
}

#[test]
fn from_integer_prec_round_fail() {
    assert_panic!(Float::from_integer_prec_round(Integer::ZERO, 0, Floor));
    assert_panic!(Float::from_integer_prec_round(Integer::ONE, 0, Floor));
    assert_panic!(Float::from_integer_prec_round(
        Integer::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::from_integer_prec_round(
        Integer::NEGATIVE_ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_integer_prec_round(
        Integer::from(-123),
        1,
        Exact
    ));
}

#[test]
fn from_integer_prec_round_ref_fail() {
    assert_panic!(Float::from_integer_prec_round_ref(&Integer::ZERO, 0, Floor));
    assert_panic!(Float::from_integer_prec_round_ref(&Integer::ONE, 0, Floor));
    assert_panic!(Float::from_integer_prec_round_ref(
        &Integer::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::from_integer_prec_round_ref(
        &Integer::NEGATIVE_ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_integer_prec_round_ref(
        &Integer::from(-123),
        1,
        Exact
    ));
}

#[test]
fn from_integer_properties() {
    integer_gen().test_properties(|n| {
        let float_n = Float::from(n.clone());
        assert!(float_n.is_valid());

        let float_n_alt = Float::from(&n);
        assert!(float_n_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(float_n, n);

        let rug_n = rug::Float::with_val(
            max(1, u32::exact_from(n.significant_bits())),
            rug::Integer::from(&n),
        );
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&Float::from(&rug_n))
        );

        assert_eq!(
            float_n.get_prec(),
            if n == 0u32 {
                None
            } else {
                Some(n.significant_bits())
            }
        );
        assert_eq!(Integer::exact_from(&float_n), n);
        let bits = max(1, n.significant_bits());
        let (f, o) = Float::from_integer_prec(n, bits);
        assert_eq!(ComparableFloat(f), ComparableFloat(float_n));
        assert_eq!(o, Equal);
    });
}

#[test]
fn from_integer_prec_properties() {
    integer_unsigned_pair_gen_var_6().test_properties(|(n, prec)| {
        let (float_n, o) = Float::from_integer_prec(n.clone(), prec);
        assert!(float_n.is_valid());

        let (float_n_alt, o_alt) = Float::from_integer_prec_ref(&n, prec);
        assert!(float_n_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_n.partial_cmp(&n), Some(o));

        let rug_n = rug::Float::with_val(u32::exact_from(prec), rug::Integer::from(&n));
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&Float::from(&rug_n))
        );

        assert_eq!(
            float_n.get_prec(),
            if n == 0u32 { None } else { Some(prec) }
        );
    });
}

#[test]
fn from_integer_prec_round_properties() {
    integer_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(n, prec, rm)| {
        let (float_n, o) = Float::from_integer_prec_round(n.clone(), prec, rm);
        assert!(float_n.is_valid());

        let (float_n_alt, o_alt) = Float::from_integer_prec_round_ref(&n, prec, rm);
        assert!(float_n_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_n.partial_cmp(&n), Some(o));
        match (n >= 0, rm) {
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
            let (rug_n, rug_o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Integer::from(&n), rm);
            assert_eq!(
                ComparableFloatRef(&float_n),
                ComparableFloatRef(&Float::from(&rug_n))
            );
            assert_eq!(rug_o, o);
        }

        assert_eq!(
            float_n.get_prec(),
            if n == 0u32 { None } else { Some(prec) }
        );
    });

    integer_unsigned_pair_gen_var_6().test_properties(|(n, prec)| {
        let floor = Float::from_integer_prec_round_ref(&n, prec, Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= n);
        if r_floor != 0u32 {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > n);
        }
        let (floor_n_alt, o_alt) =
            Float::from_integer_prec_round_ref(&n, prec, if n >= 0 { Down } else { Up });
        assert_eq!(
            ComparableFloatRef(&floor_n_alt),
            ComparableFloatRef(&floor.0)
        );
        assert_eq!(o_alt, floor.1);

        let ceiling = Float::from_integer_prec_round_ref(&n, prec, Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= n);
        if r_ceiling != 0u32 {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < n);
        }
        let (ceiling_n_alt, o_alt) =
            Float::from_integer_prec_round_ref(&n, prec, if n >= 0 { Up } else { Down });
        assert_eq!(
            ComparableFloatRef(&ceiling_n_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(o_alt, ceiling.1);

        let nearest = Float::from_integer_prec_round_ref(&n, prec, Nearest);
        let r_nearest = Rational::exact_from(&nearest.0);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        if r_nearest != 0u32 {
            assert!((r_nearest - Rational::from(&n))
                .le_abs(&(Rational::exact_from(nearest.0.ulp().unwrap()) >> 1)));
        }
    });
}
