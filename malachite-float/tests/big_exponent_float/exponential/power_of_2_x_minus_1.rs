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
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_12, float_unsigned_pair_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_22,
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

fn power_of_2_x_minus_1_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().power_of_2_x_minus_1_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.power_of_2_x_minus_1_prec_round_ref(prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.power_of_2_x_minus_1_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().power_of_2_x_minus_1_prec(prec), &r));
        assert!(same_pair(&x.power_of_2_x_minus_1_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.power_of_2_x_minus_1_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn power_of_2_x_minus_1_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::power_of_2_x_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::power_of_2_x_minus_1_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::power_of_2_x_minus_1_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::power_of_2_x_minus_1_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_power_of_2_x_minus_1_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_2_x_minus_1_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_x_minus_1_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &power_of_2_x_minus_1_prec_round_all(&ninf, 10, rm),
            &(BigExponentFloat::NEGATIVE_ONE, Equal)
        ));
        assert!(same_pair(
            &power_of_2_x_minus_1_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
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
        let r = power_of_2_x_minus_1_prec_round_all(&hex(x_hex), prec, rm);
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
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "7.8203",
        "0x7.d2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "7.828",
        "0x7.d4",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "7.8203",
        "0x7.d2",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "7.828",
        "0x7.d4",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "7.828",
        "0x7.d4",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "7.824977827076286874738941696579",
        "0x7.d331bf3337c170dfcdee149f",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "-0.8867",
        "-0x0.e3",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "-0.88574",
        "-0x0.e2c",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Down,
        "-0.88574",
        "-0x0.e2c",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Up,
        "-0.8867",
        "-0x0.e3",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Nearest,
        "-0.8867",
        "-0x0.e3",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "-0.8866852677032391173608780681405",
        "-0x0.e2fdce42a16302e3a9273a804",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.414", "0x0.6a", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.41455", "0x0.6a2", Greater);
    test("0.50", "0x0.8", 10, Down, "0.414", "0x0.6a", Less);
    test("0.50", "0x0.8", 10, Up, "0.41455", "0x0.6a2", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.414", "0x0.6a", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.41421356237309504880168872420957",
        "0x0.6a09e667f3bcc908b2fb1366e8",
        Less,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-0.50", "-0x0.8", 10, Floor, "-0.2930", "-0x0.4b", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-0.29248", "-0x0.4ae", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "-0.29248", "-0x0.4ae", Greater);
    test("-0.50", "-0x0.8", 10, Up, "-0.2930", "-0x0.4b", Less);
    test("-0.50", "-0x0.8", 10, Nearest, "-0.2930", "-0x0.4b", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.29289321881345247559915563789502",
        "-0x0.4afb0ccc06219b7ba682764c88",
        Greater,
    );
    test("3.0", "0x3", 1, Nearest, "8.0", "0x8.0", Greater);
    test("3.0", "0x3", 10, Floor, "7.0", "0x7.0", Equal);
    test("3.0", "0x3", 10, Ceiling, "7.0", "0x7.0", Equal);
    test("3.0", "0x3", 10, Down, "7.0", "0x7.0", Equal);
    test("3.0", "0x3", 10, Up, "7.0", "0x7.0", Equal);
    test("3.0", "0x3", 10, Nearest, "7.0", "0x7.0", Equal);
    test("3.0", "0x3", 100, Nearest, "7.0", "0x7.0", Equal);
    test("-3.0", "-0x3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-3.0", "-0x3", 10, Floor, "-0.88", "-0x0.e", Equal);
    test("-3.0", "-0x3", 10, Ceiling, "-0.88", "-0x0.e", Equal);
    test("-3.0", "-0x3", 10, Down, "-0.88", "-0x0.e", Equal);
    test("-3.0", "-0x3", 10, Up, "-0.88", "-0x0.e", Equal);
    test("-3.0", "-0x3", 10, Nearest, "-0.88", "-0x0.e", Equal);
    test("-3.0", "-0x3", 100, Nearest, "-0.88", "-0x0.e", Equal);
    test("100.0", "0x64", 1, Nearest, "1.3e30", "0x1.0E+25", Greater);
    test("100.0", "0x64", 10, Floor, "1.2664e30", "0xf.fcE+24", Less);
    test("100.0", "0x64", 10, Ceiling, "1.3e30", "0x1.0E+25", Greater);
    test("100.0", "0x64", 10, Down, "1.2664e30", "0xf.fcE+24", Less);
    test("100.0", "0x64", 10, Up, "1.3e30", "0x1.0E+25", Greater);
    test("100.0", "0x64", 10, Nearest, "1.3e30", "0x1.0E+25", Greater);
    test(
        "100.0",
        "0x64",
        100,
        Nearest,
        "1267650600228229401496703205375.0",
        "0xfffffffffffffffffffffffff.0",
        Equal,
    );
    test("-100.0", "-0x64", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-100.0", "-0x64", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater,
    );
    test("-100.0", "-0x64", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-100.0", "-0x64", 10, Up, "-1.0", "-0x1.0", Less);
    test("-100.0", "-0x64", 10, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "-100.0",
        "-0x64",
        100,
        Nearest,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        1,
        Nearest,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Floor,
        "4.3590e3010299956",
        "0xf.fcE+2499999999",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "4.3590e3010299956",
        "0xf.fcE+2499999999",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "4.4e3010299956",
        "0x1.0E+2500000000",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Floor,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Up,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        10,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "-1.00000000e10",
        "-0x2540be400.0",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Floor,
        "4.7284e363922934236666733021124",
        "0xf.fcE+302231454903657293676543",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Down,
        "4.7284e363922934236666733021124",
        "0xf.fcE+302231454903657293676543",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Up,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Nearest,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "4.7e363922934236666733021124",
        "0x1.0E+302231454903657293676544",
        Greater,
    );
    test("-1.2e24", "-0x1.0E+20", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.2e24", "-0x1.0E+20", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.2e24",
        "-0x1.0E+20",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-1.2e24", "-0x1.0E+20", 10, Up, "-1.0", "-0x1.0", Less);
    test("-1.2e24", "-0x1.0E+20", 10, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "-1.2e24",
        "-0x1.0E+20",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        1,
        Nearest,
        "1.9e-121",
        "0x8.0E-101",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "2.6813e-121",
        "0xb.14E-101",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "2.685e-121",
        "0xb.18E-101",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "2.6813e-121",
        "0xb.14E-101",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "2.685e-121",
        "0xb.18E-101",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "2.685e-121",
        "0xb.18E-101",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "2.6842761672370447637332615355e-121",
        "0xb.17217f7d1cf79abc9e3b398E-101",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        1,
        Nearest,
        "-1.9e-121",
        "-0x8.0E-101",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Floor,
        "-2.685e-121",
        "-0xb.18E-101",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Ceiling,
        "-2.6813e-121",
        "-0xb.14E-101",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Down,
        "-2.6813e-121",
        "-0xb.14E-101",
        Greater,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Up,
        "-2.685e-121",
        "-0xb.18E-101",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        10,
        Nearest,
        "-2.685e-121",
        "-0xb.18E-101",
        Less,
    );
    test(
        "-3.9e-121",
        "-0x1.0E-100",
        100,
        Nearest,
        "-2.6842761672370447637332615355e-121",
        "-0xb.17217f7d1cf79abc9e3b398E-101",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "1.6e-361235995",
        "0x1.0E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "1.659e-361235995",
        "0x1.0aE-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "1.6622e-361235995",
        "0x1.0a8E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "1.659e-361235995",
        "0x1.0aE-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "1.6622e-361235995",
        "0x1.0a8E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "1.659e-361235995",
        "0x1.0aE-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "1.66011925137764267425722659811e-361235995",
        "0x1.0a2b23f3bab73681aed58d64E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "-1.6e-361235995",
        "-0x1.0E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "-1.6622e-361235995",
        "-0x1.0a8E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "-1.659e-361235995",
        "-0x1.0aE-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "-1.659e-361235995",
        "-0x1.0aE-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "-1.6622e-361235995",
        "-0x1.0a8E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "-1.659e-361235995",
        "-0x1.0aE-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "-1.66011925137764267425722659811e-361235995",
        "-0x1.0a2b23f3bab73681aed58d64E-300000000",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        1,
        Nearest,
        "6.0e404035620",
        "0x1.0E+335544320",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Floor,
        "6.0016e404035620",
        "0xf.fcE+335544319",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Ceiling,
        "6.0e404035620",
        "0x1.0E+335544320",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Down,
        "6.0016e404035620",
        "0xf.fcE+335544319",
        Less,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Up,
        "6.0e404035620",
        "0x1.0E+335544320",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        10,
        Nearest,
        "6.0e404035620",
        "0x1.0E+335544320",
        Greater,
    );
    test(
        "1.3e9",
        "0x5.0E+7",
        100,
        Nearest,
        "6.0e404035620",
        "0x1.0E+335544320",
        Greater,
    );
    test("-1.3e9", "-0x5.0E+7", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test(
        "-1.3e9",
        "-0x5.0E+7",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-1.3e9", "-0x5.0E+7", 10, Up, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.3e9", "-0x5.0E+7", 100, Nearest, "-1.0", "-0x1.0", Less);
    test("3.0", "0x3", 300, Exact, "7.0", "0x7.0", Equal);
    test("-3.0", "-0x3", 300, Exact, "-0.88", "-0x0.e", Equal);
    test(
        "100.0",
        "0x64",
        300,
        Exact,
        "1267650600228229401496703205375.0",
        "0xfffffffffffffffffffffffff.0",
        Equal,
    );
    test(
        "-100.0",
        "-0x64",
        300,
        Exact,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff",
        Equal,
    );
}

#[test]
fn test_power_of_2_x_minus_1_rational_prec_round() {
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &power_of_2_x_minus_1_rational_prec_round_all(&Rational::ZERO, 10, rm),
            &(BigExponentFloat::ZERO, Equal)
        ));
    }
    // The expected values agree with MPFR's wherever MPFR's exponent range allows.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r =
            power_of_2_x_minus_1_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.2598", "0x0.428", Less);
    test("1/3", 10, Ceiling, "0.26025", "0x0.42a", Greater);
    test("1/3", 10, Down, "0.2598", "0x0.428", Less);
    test("1/3", 10, Up, "0.26025", "0x0.42a", Greater);
    test("1/3", 10, Nearest, "0.2598", "0x0.428", Less);
    test(
        "1/3",
        100,
        Nearest,
        "0.2599210498948731647672106072783",
        "0x0.428a2f98d728ae223ddab715c",
        Greater,
    );
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4", Less);
    test("-1/3", 10, Floor, "-0.2065", "-0x0.34e", Less);
    test("-1/3", 10, Ceiling, "-0.20630", "-0x0.34d", Greater);
    test("-1/3", 10, Down, "-0.20630", "-0x0.34d", Greater);
    test("-1/3", 10, Up, "-0.2065", "-0x0.34e", Less);
    test("-1/3", 10, Nearest, "-0.20630", "-0x0.34d", Greater);
    test(
        "-1/3",
        100,
        Nearest,
        "-0.2062994740159002626241471803638",
        "-0x0.34d00ad6148e1bea7d3332a5e",
        Greater,
    );
    test("3/5", 1, Nearest, "0.50", "0x0.8", Less);
    test("3/5", 10, Floor, "0.516", "0x0.84", Less);
    test("3/5", 10, Ceiling, "0.51660", "0x0.844", Greater);
    test("3/5", 10, Down, "0.516", "0x0.84", Less);
    test("3/5", 10, Up, "0.51660", "0x0.844", Greater);
    test("3/5", 10, Nearest, "0.516", "0x0.84", Less);
    test(
        "3/5",
        100,
        Nearest,
        "0.51571656651039808234725980130654",
        "0x0.8406003b2ae5c6b99347320eb",
        Greater,
    );
    test("3", 1, Nearest, "8.0", "0x8.0", Greater);
    test("3", 10, Floor, "7.0", "0x7.0", Equal);
    test("3", 10, Ceiling, "7.0", "0x7.0", Equal);
    test("3", 10, Down, "7.0", "0x7.0", Equal);
    test("3", 10, Up, "7.0", "0x7.0", Equal);
    test("3", 10, Nearest, "7.0", "0x7.0", Equal);
    test("3", 100, Nearest, "7.0", "0x7.0", Equal);
    test("-3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-3", 10, Floor, "-0.88", "-0x0.e", Equal);
    test("-3", 10, Ceiling, "-0.88", "-0x0.e", Equal);
    test("-3", 10, Down, "-0.88", "-0x0.e", Equal);
    test("-3", 10, Up, "-0.88", "-0x0.e", Equal);
    test("-3", 10, Nearest, "-0.88", "-0x0.e", Equal);
    test("-3", 100, Nearest, "-0.88", "-0x0.e", Equal);
    test("100", 1, Nearest, "1.3e30", "0x1.0E+25", Greater);
    test("100", 10, Floor, "1.2664e30", "0xf.fcE+24", Less);
    test("100", 10, Ceiling, "1.3e30", "0x1.0E+25", Greater);
    test("100", 10, Down, "1.2664e30", "0xf.fcE+24", Less);
    test("100", 10, Up, "1.3e30", "0x1.0E+25", Greater);
    test("100", 10, Nearest, "1.3e30", "0x1.0E+25", Greater);
    test(
        "100",
        100,
        Nearest,
        "1267650600228229401496703205375.0",
        "0xfffffffffffffffffffffffff.0",
        Equal,
    );
    test("-100", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-100", 10, Floor, "-1.0", "-0x1.0", Less);
    test("-100", 10, Ceiling, "-0.99902", "-0x0.ffc", Greater);
    test("-100", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-100", 10, Up, "-1.0", "-0x1.0", Less);
    test("-100", 10, Nearest, "-1.0", "-0x1.0", Less);
    test(
        "-100",
        100,
        Nearest,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff",
        Equal,
    );
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
    test("-10000000000/3", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 10, Floor, "-1.0", "-0x1.0", Less);
    test(
        "-10000000000/3",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc",
        Greater,
    );
    test("-10000000000/3", 10, Down, "-0.99902", "-0x0.ffc", Greater);
    test("-10000000000/3", 10, Up, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 10, Nearest, "-1.0", "-0x1.0", Less);
    test("-10000000000/3", 100, Nearest, "-1.0", "-0x1.0", Less);
    test("1/10000000000", 1, Nearest, "5.8e-11", "0x4.0E-9", Less);
    test("1/10000000000", 10, Floor, "6.9235e-11", "0x4.c2E-9", Less);
    test(
        "1/10000000000",
        10,
        Ceiling,
        "6.935e-11",
        "0x4.c4E-9",
        Greater,
    );
    test("1/10000000000", 10, Down, "6.9235e-11", "0x4.c2E-9", Less);
    test("1/10000000000", 10, Up, "6.935e-11", "0x4.c4E-9", Greater);
    test(
        "1/10000000000",
        10,
        Nearest,
        "6.935e-11",
        "0x4.c4E-9",
        Greater,
    );
    test(
        "1/10000000000",
        100,
        Nearest,
        "6.9314718058396796011369723377813e-11",
        "0x4.c365bd0908848ce7456463c28E-9",
        Less,
    );
    test(
        "-1/10000000000",
        1,
        Nearest,
        "-5.8e-11",
        "-0x4.0E-9",
        Greater,
    );
    test(
        "-1/10000000000",
        10,
        Floor,
        "-6.935e-11",
        "-0x4.c4E-9",
        Less,
    );
    test(
        "-1/10000000000",
        10,
        Ceiling,
        "-6.9235e-11",
        "-0x4.c2E-9",
        Greater,
    );
    test(
        "-1/10000000000",
        10,
        Down,
        "-6.9235e-11",
        "-0x4.c2E-9",
        Greater,
    );
    test("-1/10000000000", 10, Up, "-6.935e-11", "-0x4.c4E-9", Less);
    test(
        "-1/10000000000",
        10,
        Nearest,
        "-6.935e-11",
        "-0x4.c4E-9",
        Less,
    );
    test(
        "-1/10000000000",
        100,
        Nearest,
        "-6.931471805359226587218770913118e-11",
        "-0x4.c365bd079d7f6bfc85b9a87aE-9",
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
    test("3", 300, Exact, "7.0", "0x7.0", Equal);
    test("-3", 300, Exact, "-0.88", "-0x0.e", Equal);
    test(
        "100",
        300,
        Exact,
        "1267650600228229401496703205375.0",
        "0xfffffffffffffffffffffffff.0",
        Equal,
    );
    test(
        "-100",
        300,
        Exact,
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff",
        Equal,
    );
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_prec_round_fail_1() {
    BigExponentFloat::ONE.power_of_2_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_prec_round_fail_2() {
    BigExponentFloat::from(0.5).power_of_2_x_minus_1_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_rational_prec_round_fail_1() {
    BigExponentFloat::power_of_2_x_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_rational_prec_round_fail_2() {
    BigExponentFloat::power_of_2_x_minus_1_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

fn power_of_2_x_minus_1_prec_round_properties_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) {
    let r = power_of_2_x_minus_1_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if !r.0.is_nan() {
        assert_eq!(r.0.partial_cmp(&0u32), x.partial_cmp(&0u32));
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else {
        // The result lies between the results rounded down and up.
        let floor = power_of_2_x_minus_1_prec_round_all(x, prec, Floor);
        let ceiling = power_of_2_x_minus_1_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0 || r.0.is_nan());
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &power_of_2_x_minus_1_prec_round_all(x, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = power_of_2_x_minus_1_prec_round_all(x, prec + 16, rm);
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
            &power_of_2_x_minus_1_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
    // At an integer n, the exact value is a `Rational`.
    if let Ok(n) = Integer::try_from(x)
        && n.unsigned_abs_ref() <= &1000u32
    {
        let n = i64::exact_from(&n);
        let exact = Rational::power_of_2(n) - Rational::ONE;
        assert!(same_pair(
            &BigExponentFloat::from_rational_prec_round(exact, prec, rm),
            &r
        ));
    }
}

#[test]
fn power_of_2_x_minus_1_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_12().test_properties(
        |(x, prec, rm)| {
            power_of_2_x_minus_1_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn power_of_2_x_minus_1_prec_round_float_agreement_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            if rm == Exact {
                continue;
            }
            let x = &x;
            let (f, fo) = x.clone().power_of_2_x_minus_1_prec_round(prec, rm);
            if float_result_comparable(&f, fo) {
                let r =
                    power_of_2_x_minus_1_prec_round_all(&float_to_big_exponent_float(x), prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}

#[test]
fn power_of_2_x_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_22().test_properties(|(x, prec, rm)| {
        let r = power_of_2_x_minus_1_rational_prec_round_all(&x, prec, rm);
        if r.0.is_zero() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        if rm == Exact {
            assert_eq!(r.1, Equal);
        } else if rm != Nearest {
            let (y, o) = power_of_2_x_minus_1_rational_prec_round_all(&x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    });
}

#[test]
fn power_of_2_x_minus_1_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_22().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let x = &x;
        let (f, fo) = Float::power_of_2_x_minus_1_rational_prec_round_ref(x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = power_of_2_x_minus_1_rational_prec_round_all(x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
