// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_11, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_21,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn power_of_2_of_big_exponent_float_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::power_of_2_of_big_exponent_float_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::power_of_2_of_big_exponent_float_prec_round_ref(x, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.power_of_2_of_big_exponent_float_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::power_of_2_of_big_exponent_float_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::power_of_2_of_big_exponent_float_prec_ref(x, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.power_of_2_of_big_exponent_float_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn power_of_2_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::power_of_2_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::power_of_2_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::power_of_2_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::power_of_2_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_power_of_2_of_big_exponent_float_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&ninf, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_of_big_exponent_float_prec_round_all(&zero, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = power_of_2_of_big_exponent_float_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "8.0",
        "0x8.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "8.812",
        "0x8.d",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "8.8281",
        "0x8.d4",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "8.812",
        "0x8.d",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "8.8281",
        "0x8.d4",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "8.8281",
        "0x8.d4",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "8.8249778270762868747389416965789",
        "0x8.d331bf3337c170dfcdee149f",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "0.12",
        "0x0.2",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "0.113",
        "0x0.1d",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "0.11340",
        "0x0.1d08",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Down,
        "0.113",
        "0x0.1d",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Up,
        "0.11340",
        "0x0.1d08",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Nearest,
        "0.113",
        "0x0.1d",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "0.11331473229676088263912193185925",
        "0x0.1d0231bd5e9cfd1c56d8c57fba",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "1.0", "0x1.0", Less);
    test("0.50", "0x0.8", 10, Floor, "1.414", "0x1.6a", Less);
    test("0.50", "0x0.8", 10, Ceiling, "1.4160", "0x1.6a8", Greater);
    test("0.50", "0x0.8", 10, Down, "1.414", "0x1.6a", Less);
    test("0.50", "0x0.8", 10, Up, "1.4160", "0x1.6a8", Greater);
    test("0.50", "0x0.8", 10, Nearest, "1.414", "0x1.6a", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "1.4142135623730950488016887242092",
        "0x1.6a09e667f3bcc908b2fb1366e",
        Less,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test("-0.50", "-0x0.8", 10, Floor, "0.7070", "0x0.b5", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "0.70801", "0x0.b54", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "0.7070", "0x0.b5", Less);
    test("-0.50", "-0x0.8", 10, Up, "0.70801", "0x0.b54", Greater);
    test("-0.50", "-0x0.8", 10, Nearest, "0.7070", "0x0.b5", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "0.70710678118654752440084436210459",
        "0x0.b504f333f9de6484597d89b37",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "8.0", "0x8.0", Equal);
    test("3.0", "0x3", 10, Floor, "8.0", "0x8.0", Equal);
    test("3.0", "0x3", 10, Ceiling, "8.0", "0x8.0", Equal);
    test("3.0", "0x3", 10, Down, "8.0", "0x8.0", Equal);
    test("3.0", "0x3", 10, Up, "8.0", "0x8.0", Equal);
    test("3.0", "0x3", 10, Nearest, "8.0", "0x8.0", Equal);
    test("3.0", "0x3", 100, Nearest, "8.0", "0x8.0", Equal);
    test("-3.0", "-0x3", 1, Nearest, "0.12", "0x0.2", Equal);
    test("-3.0", "-0x3", 10, Floor, "0.12", "0x0.2", Equal);
    test("-3.0", "-0x3", 10, Ceiling, "0.12", "0x0.2", Equal);
    test("-3.0", "-0x3", 10, Down, "0.12", "0x0.2", Equal);
    test("-3.0", "-0x3", 10, Up, "0.12", "0x0.2", Equal);
    test("-3.0", "-0x3", 10, Nearest, "0.12", "0x0.2", Equal);
    test("-3.0", "-0x3", 100, Nearest, "0.12", "0x0.2", Equal);
    test("100.0", "0x64", 1, Nearest, "1.3e30", "0x1.0E+25", Equal);
    test("100.0", "0x64", 10, Floor, "1.3e30", "0x1.0E+25", Equal);
    test("100.0", "0x64", 10, Ceiling, "1.3e30", "0x1.0E+25", Equal);
    test("100.0", "0x64", 10, Down, "1.3e30", "0x1.0E+25", Equal);
    test("100.0", "0x64", 10, Up, "1.3e30", "0x1.0E+25", Equal);
    test("100.0", "0x64", 10, Nearest, "1.3e30", "0x1.0E+25", Equal);
    test("100.0", "0x64", 100, Nearest, "1.3e30", "0x1.0E+25", Equal);
    test("-100.0", "-0x64", 1, Nearest, "7.9e-31", "0x1.0E-25", Equal);
    test("-100.0", "-0x64", 10, Floor, "7.9e-31", "0x1.0E-25", Equal);
    test(
        "-100.0",
        "-0x64",
        10,
        Ceiling,
        "7.9e-31",
        "0x1.0E-25",
        Equal,
    );
    test("-100.0", "-0x64", 10, Down, "7.9e-31", "0x1.0E-25", Equal);
    test("-100.0", "-0x64", 10, Up, "7.9e-31", "0x1.0E-25", Equal);
    test(
        "-100.0",
        "-0x64",
        10,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Equal,
    );
    test(
        "-100.0",
        "-0x64",
        100,
        Nearest,
        "7.9e-31",
        "0x1.0E-25",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        1,
        Nearest,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Floor,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        1,
        Nearest,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Floor,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Ceiling,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Down,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Up,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Nearest,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        100,
        Nearest,
        "2.3e-3010299957",
        "0x1.0E-2500000000",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Floor,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Down,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Up,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Nearest,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        1,
        Nearest,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Floor,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Down,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Up,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Nearest,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "2.1e-363922934236666733021125",
        "0x1.0E-302231454903657293676544",
        Equal,
    );
    test("3.9e-121", "0x1.0E-100", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.9e-121", "0x1.0E-100", 10, Floor, "1.0", "0x1.0", Less);
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 10, Down, "1.0", "0x1.0", Less);
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 10, Nearest, "1.0", "0x1.0", Less);
    test("3.9e-121", "0x1.0E-100", 100, Nearest, "1.0", "0x1.0", Less);
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test("-3.9e-121", "-0x1.0E-100", 10, Up, "1.0", "0x1.0", Greater);
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "0.99902",
        "0x0.ffc",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        1,
        Nearest,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Floor,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Ceiling,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Down,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Up,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Nearest,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        100,
        Nearest,
        "6.0e404035620",
        "0x1.0E+335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        1,
        Nearest,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Floor,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Ceiling,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Down,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Up,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Nearest,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        100,
        Nearest,
        "1.7e-404035621",
        "0x1.0E-335544320",
        Equal,
    );
    test("3.0", "0x3", 300, Exact, "8.0", "0x8.0", Equal);
    test("-3.0", "-0x3", 300, Exact, "0.12", "0x0.2", Equal);
    test("100.0", "0x64", 300, Exact, "1.3e30", "0x1.0E+25", Equal);
    test("-100.0", "-0x64", 300, Exact, "7.9e-31", "0x1.0E-25", Equal);
}

#[test]
fn test_power_of_2_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_2_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ONE, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = power_of_2_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.2598", "0x1.428", Less);
    test("1/3", 10, Ceiling, "1.262", "0x1.43", Greater);
    test("1/3", 10, Down, "1.2598", "0x1.428", Less);
    test("1/3", 10, Up, "1.262", "0x1.43", Greater);
    test("1/3", 10, Nearest, "1.2598", "0x1.428", Less);
    test(
        "1/3",
        100,
        Nearest,
        "1.259921049894873164767210607278",
        "0x1.428a2f98d728ae223ddab715c",
        Greater,
    );
    test("-1/3", 1, Nearest, "1.0", "0x1.0", Greater);
    test("-1/3", 10, Floor, "0.7930", "0x0.cb", Less);
    test("-1/3", 10, Ceiling, "0.79395", "0x0.cb4", Greater);
    test("-1/3", 10, Down, "0.7930", "0x0.cb", Less);
    test("-1/3", 10, Up, "0.79395", "0x0.cb4", Greater);
    test("-1/3", 10, Nearest, "0.79395", "0x0.cb4", Greater);
    test(
        "-1/3",
        100,
        Nearest,
        "0.7937005259840997373758528196362",
        "0x0.cb2ff529eb71e41582cccd5a2",
        Greater,
    );
    test("3/5", 1, Nearest, "2.0", "0x2.0", Greater);
    test("3/5", 10, Floor, "1.516", "0x1.84", Less);
    test("3/5", 10, Ceiling, "1.5176", "0x1.848", Greater);
    test("3/5", 10, Down, "1.516", "0x1.84", Less);
    test("3/5", 10, Up, "1.5176", "0x1.848", Greater);
    test("3/5", 10, Nearest, "1.516", "0x1.84", Less);
    test(
        "3/5",
        100,
        Nearest,
        "1.5157165665103980823472598013058",
        "0x1.8406003b2ae5c6b99347320ea",
        Less,
    );
    test("3", 1, Nearest, "8.0", "0x8.0", Equal);
    test("3", 10, Floor, "8.0", "0x8.0", Equal);
    test("3", 10, Ceiling, "8.0", "0x8.0", Equal);
    test("3", 10, Down, "8.0", "0x8.0", Equal);
    test("3", 10, Up, "8.0", "0x8.0", Equal);
    test("3", 10, Nearest, "8.0", "0x8.0", Equal);
    test("3", 100, Nearest, "8.0", "0x8.0", Equal);
    test("-3", 1, Nearest, "0.12", "0x0.2", Equal);
    test("-3", 10, Floor, "0.12", "0x0.2", Equal);
    test("-3", 10, Ceiling, "0.12", "0x0.2", Equal);
    test("-3", 10, Down, "0.12", "0x0.2", Equal);
    test("-3", 10, Up, "0.12", "0x0.2", Equal);
    test("-3", 10, Nearest, "0.12", "0x0.2", Equal);
    test("-3", 100, Nearest, "0.12", "0x0.2", Equal);
    test("100", 1, Nearest, "1.3e30", "0x1.0E+25", Equal);
    test("100", 10, Floor, "1.3e30", "0x1.0E+25", Equal);
    test("100", 10, Ceiling, "1.3e30", "0x1.0E+25", Equal);
    test("100", 10, Down, "1.3e30", "0x1.0E+25", Equal);
    test("100", 10, Up, "1.3e30", "0x1.0E+25", Equal);
    test("100", 10, Nearest, "1.3e30", "0x1.0E+25", Equal);
    test("100", 100, Nearest, "1.3e30", "0x1.0E+25", Equal);
    test("-100", 1, Nearest, "7.9e-31", "0x1.0E-25", Equal);
    test("-100", 10, Floor, "7.9e-31", "0x1.0E-25", Equal);
    test("-100", 10, Ceiling, "7.9e-31", "0x1.0E-25", Equal);
    test("-100", 10, Down, "7.9e-31", "0x1.0E-25", Equal);
    test("-100", 10, Up, "7.9e-31", "0x1.0E-25", Equal);
    test("-100", 10, Nearest, "7.9e-31", "0x1.0E-25", Equal);
    test("-100", 100, Nearest, "7.9e-31", "0x1.0E-25", Equal);
    test(
        "10000000000/3",
        1,
        Nearest,
        "6.0e1003433318",
        "0x2.0E+833333333",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Floor,
        "7.5837e1003433318",
        "0x2.85E+833333333",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Ceiling,
        "7.596e1003433318",
        "0x2.86E+833333333",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Down,
        "7.5837e1003433318",
        "0x2.85E+833333333",
        Less,
    );
    test(
        "10000000000/3",
        10,
        Up,
        "7.596e1003433318",
        "0x2.86E+833333333",
        Greater,
    );
    test(
        "10000000000/3",
        10,
        Nearest,
        "7.5837e1003433318",
        "0x2.85E+833333333",
        Less,
    );
    test(
        "10000000000/3",
        100,
        Nearest,
        "7.584680958596010496687463397264e1003433318",
        "0x2.85145f31ae515c447bb56e2b8E+833333333",
        Greater,
    );
    test(
        "-10000000000/3",
        1,
        Nearest,
        "1.7e-1003433319",
        "0x8.0E-833333334",
        Greater,
    );
    test(
        "-10000000000/3",
        10,
        Floor,
        "1.317e-1003433319",
        "0x6.58E-833333334",
        Less,
    );
    test(
        "-10000000000/3",
        10,
        Ceiling,
        "1.3189e-1003433319",
        "0x6.5aE-833333334",
        Greater,
    );
    test(
        "-10000000000/3",
        10,
        Down,
        "1.317e-1003433319",
        "0x6.58E-833333334",
        Less,
    );
    test(
        "-10000000000/3",
        10,
        Up,
        "1.3189e-1003433319",
        "0x6.5aE-833333334",
        Greater,
    );
    test(
        "-10000000000/3",
        10,
        Nearest,
        "1.3189e-1003433319",
        "0x6.5aE-833333334",
        Greater,
    );
    test(
        "-10000000000/3",
        100,
        Nearest,
        "1.318447019009628293628743816325e-1003433319",
        "0x6.597fa94f5b8f20ac16666ad1E-833333334",
        Greater,
    );
    test("1/10000000000", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/10000000000", 10, Floor, "1.0", "0x1.0", Less);
    test("1/10000000000", 10, Ceiling, "1.0020", "0x1.008", Greater);
    test("1/10000000000", 10, Down, "1.0", "0x1.0", Less);
    test("1/10000000000", 10, Up, "1.0020", "0x1.008", Greater);
    test("1/10000000000", 10, Nearest, "1.0", "0x1.0", Less);
    test(
        "1/10000000000",
        100,
        Nearest,
        "1.000000000069314718058396796012",
        "0x1.000000004c365bd0908848ce8",
        Greater,
    );
    test("-1/10000000000", 1, Nearest, "1.0", "0x1.0", Greater);
    test("-1/10000000000", 10, Floor, "0.99902", "0x0.ffc", Less);
    test("-1/10000000000", 10, Ceiling, "1.0", "0x1.0", Greater);
    test("-1/10000000000", 10, Down, "0.99902", "0x0.ffc", Less);
    test("-1/10000000000", 10, Up, "1.0", "0x1.0", Greater);
    test("-1/10000000000", 10, Nearest, "1.0", "0x1.0", Greater);
    test(
        "-1/10000000000",
        100,
        Nearest,
        "0.99999999993068528194640773412744",
        "0x0.ffffffffb3c9a42f862809403",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        1,
        Nearest,
        "1.3e121307644745555577673708",
        "0x2.0E+100743818301219097892181",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Floor,
        "1.6788e121307644745555577673708",
        "0x2.85E+100743818301219097892181",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Ceiling,
        "1.681e121307644745555577673708",
        "0x2.86E+100743818301219097892181",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Down,
        "1.6788e121307644745555577673708",
        "0x2.85E+100743818301219097892181",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Up,
        "1.681e121307644745555577673708",
        "0x2.86E+100743818301219097892181",
        Greater,
    );
    test(
        "1208925819614629174706176/3",
        10,
        Nearest,
        "1.6788e121307644745555577673708",
        "0x2.85E+100743818301219097892181",
        Less,
    );
    test(
        "1208925819614629174706176/3",
        100,
        Nearest,
        "1.678985353921004680273469717807e121307644745555577673708",
        "0x2.85145f31ae515c447bb56e2b8E+100743818301219097892181",
        Greater,
    );
    test("3", 300, Exact, "8.0", "0x8.0", Equal);
    test("-3", 300, Exact, "0.12", "0x0.2", Equal);
    test("100", 300, Exact, "1.3e30", "0x1.0E+25", Equal);
    test("-100", 300, Exact, "7.9e-31", "0x1.0E-25", Equal);
}

#[test]
#[should_panic]
fn power_of_2_of_big_exponent_float_prec_round_fail_1() {
    BigExponentFloat::power_of_2_of_big_exponent_float_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_of_big_exponent_float_prec_round_fail_2() {
    BigExponentFloat::power_of_2_of_big_exponent_float_prec_round(
        BigExponentFloat::from(0.5),
        10,
        Exact,
    );
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_fail_1() {
    BigExponentFloat::power_of_2_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_fail_2() {
    BigExponentFloat::power_of_2_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn power_of_2_of_big_exponent_float_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = power_of_2_of_big_exponent_float_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert!(r.0 >= 0u32);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = power_of_2_of_big_exponent_float_prec_round_all(x, prec, Floor);
        let ceiling = power_of_2_of_big_exponent_float_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &power_of_2_of_big_exponent_float_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = power_of_2_of_big_exponent_float_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(
            &power_of_2_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
    // At an integer n, the exact value is a `Rational`.
    if let Ok(n) = Integer::try_from(x)
        && n.unsigned_abs_ref() <= &1000u32
    {
        let n = i64::exact_from(&n);
        let exact = Rational::power_of_2(n);
        assert!(same_pair(
            &BigExponentFloat::from_rational_prec_round(exact, prec, rm),
            &r
        ));
    }
    // 2^(x + k) = 2^x * 2^k for any integer k, even beyond `Float`'s exponent range.
    if x.is_finite() && exponent_is_moderate(x) {
        for k in [
            Integer::ONE,
            Integer::NEGATIVE_ONE,
            Integer::from_str("100000000000000000000").unwrap(),
        ] {
            let s = power_of_2_of_big_exponent_float_prec_round_all(
                &(x + BigExponentFloat::from(k.clone())),
                prec,
                rm,
            );
            assert!(same_pair(
                &s,
                &(&r.0 * big_exponent_float_power_of_2(k), r.1)
            ));
        }
    }
}

#[test]
fn power_of_2_of_big_exponent_float_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_11().test_properties(
        |(x, prec, rm)| {
            power_of_2_of_big_exponent_float_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn power_of_2_of_big_exponent_float_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = Float::power_of_2_of_float_prec_round_ref(x, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = power_of_2_of_big_exponent_float_prec_round_all(
                    &float_to_big_exponent_float(x),
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn power_of_2_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        let r = power_of_2_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = power_of_2_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn power_of_2_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::power_of_2_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = power_of_2_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
