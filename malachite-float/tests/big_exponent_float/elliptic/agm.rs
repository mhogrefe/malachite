// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::elliptic::agm::agm_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_15,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_19,
    float_float_unsigned_rounding_mode_quadruple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_10,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_25,
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_1,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_zero() || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn agm_prec_round_all(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().agm_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone().agm_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.agm_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(&x.agm_prec_round_ref_ref(y, prec, rm), &r));
    let mut z = x.clone();
    let o = z.agm_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.agm_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().agm_prec(y.clone(), prec), &r));
        assert!(same_pair(&x.clone().agm_prec_val_ref(y, prec), &r));
        assert!(same_pair(&x.agm_prec_ref_val(y.clone(), prec), &r));
        assert!(same_pair(&x.agm_prec_ref_ref(y, prec), &r));
        let mut z = x.clone();
        let o = z.agm_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.agm_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn agm_rational_prec_round_all(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::agm_rational_prec_round(x.clone(), y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::agm_rational_prec_round_val_ref(x.clone(), y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::agm_rational_prec_round_ref_val(x, y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &BigExponentFloat::agm_rational_prec_round_ref_ref(x, y, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::agm_rational_prec(x.clone(), y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::agm_rational_prec_val_ref(x.clone(), y, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::agm_rational_prec_ref_val(x, y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::agm_rational_prec_ref_ref(x, y, prec),
            &r
        ));
    }
    r
}

fn agm_big_exponent_float_rational_prec_round_all(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .agm_big_exponent_float_rational_prec_round(y.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.clone()
            .agm_big_exponent_float_rational_prec_round_val_ref(y, prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.agm_big_exponent_float_rational_prec_round_ref_val(y.clone(), prec, rm),
        &r
    ));
    assert!(same_pair(
        &x.agm_big_exponent_float_rational_prec_round_ref_ref(y, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.agm_big_exponent_float_rational_prec_round_assign(y.clone(), prec, rm);
    assert!(same_pair(&(z, o), &r));
    let mut z = x.clone();
    let o = z.agm_big_exponent_float_rational_prec_round_assign_ref(y, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone()
                .agm_big_exponent_float_rational_prec(y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.clone()
                .agm_big_exponent_float_rational_prec_val_ref(y, prec),
            &r
        ));
        assert!(same_pair(
            &x.agm_big_exponent_float_rational_prec_ref_val(y.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &x.agm_big_exponent_float_rational_prec_ref_ref(y, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.agm_big_exponent_float_rational_prec_assign(y.clone(), prec);
        assert!(same_pair(&(z, o), &r));
        let mut z = x.clone();
        let o = z.agm_big_exponent_float_rational_prec_assign_ref(y, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

// Checks the properties of a rounded AGM `r` of arguments `x` and `y` that are shared by all three
// functions: a `NaN`, zero, or infinite result is exact; otherwise, the result is positive, has at
// most `prec` bits, and lies between the arguments rounded outward.
fn check_agm_result(
    r: &(BigExponentFloat, Ordering),
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    prec: u64,
) {
    if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0 > 0u32);
        assert!(r.0.get_prec().unwrap() <= prec);
        let (lo, hi) = if x < y { (x, y) } else { (y, x) };
        assert!(r.0 >= lo.round_to_prec_round_ref(prec, Floor).0);
        assert!(r.0 <= hi.round_to_prec_round_ref(prec, Ceiling).0);
    }
}

#[test]
fn test_agm_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                y: &str,
                y_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        assert_eq!(hex(y_hex).to_string(), y);
        let r = agm_prec_round_all(&hex(x_hex), &hex(y_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {y} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {y} {prec} {rm}");
        assert!(same_pair(
            &agm_prec_round_generic(&hex(x_hex), &hex(y_hex), prec, rm),
            &r
        ));
    };
    // Special cases follow `Float`'s.
    test("NaN", "NaN", "1.0", "0x1", 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", "NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "1.0", "0x1", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "1.0", "0x1", "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0", "Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-1.0", "-0x1", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "1.0",
        "0x1",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0", "1.0", "0x1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", "0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0", "-1.0", "-0x1", 10, Floor, "0.0", "0x0.0", Equal,
    );
    test("-1.0", "-0x1", "1.0", "0x1", 10, Floor, "NaN", "NaN", Equal);
    test(
        "-1.0", "-0x1", "-2.0", "-0x2", 10, Exact, "NaN", "NaN", Equal,
    );
    // AGM(x, x) = x.
    test("1.0", "0x1", "1.0", "0x1", 10, Exact, "1.0", "0x1.0", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "3.141",
        "0x3.24",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "3.1445",
        "0x3.25",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "3.141",
        "0x3.24",
        Less,
    );
    // AGM(24, 6).
    test("24.0", "0x18", "6.0", "0x6", 1, Floor, "8.0", "0x8.0", Less);
    test(
        "24.0", "0x18", "6.0", "0x6", 1, Ceiling, "16.0", "0x1.0E+1", Greater,
    );
    test("24.0", "0x18", "6.0", "0x6", 1, Down, "8.0", "0x8.0", Less);
    test(
        "24.0", "0x18", "6.0", "0x6", 1, Up, "16.0", "0x1.0E+1", Greater,
    );
    test(
        "24.0", "0x18", "6.0", "0x6", 1, Nearest, "16.0", "0x1.0E+1", Greater,
    );
    test(
        "24.0", "0x18", "6.0", "0x6", 5, Floor, "13.0", "0xd.0", Less,
    );
    test(
        "24.0", "0x18", "6.0", "0x6", 5, Ceiling, "13.5", "0xd.8", Greater,
    );
    test("24.0", "0x18", "6.0", "0x6", 5, Down, "13.0", "0xd.0", Less);
    test(
        "24.0", "0x18", "6.0", "0x6", 5, Up, "13.5", "0xd.8", Greater,
    );
    test(
        "24.0", "0x18", "6.0", "0x6", 5, Nearest, "13.5", "0xd.8", Greater,
    );
    test(
        "24.0", "0x18", "6.0", "0x6", 20, Floor, "13.45816", "0xd.754a", Less,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        20,
        Ceiling,
        "13.458176",
        "0xd.754b",
        Greater,
    );
    test(
        "24.0", "0x18", "6.0", "0x6", 20, Down, "13.45816", "0xd.754a", Less,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        20,
        Up,
        "13.458176",
        "0xd.754b",
        Greater,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        20,
        Nearest,
        "13.458176",
        "0xd.754b",
        Greater,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        100,
        Floor,
        "13.4581714817256154207668131570",
        "0xd.754ab9e9f8ac5a069236024",
        Less,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        100,
        Ceiling,
        "13.458171481725615420766813156976",
        "0xd.754ab9e9f8ac5a0692360241",
        Greater,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        100,
        Down,
        "13.4581714817256154207668131570",
        "0xd.754ab9e9f8ac5a069236024",
        Less,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        100,
        Up,
        "13.458171481725615420766813156976",
        "0xd.754ab9e9f8ac5a0692360241",
        Greater,
    );
    test(
        "24.0",
        "0x18",
        "6.0",
        "0x6",
        100,
        Nearest,
        "13.458171481725615420766813156976",
        "0xd.754ab9e9f8ac5a0692360241",
        Greater,
    );
    test(
        "1.0", "0x1", "2.0", "0x2", 10, Floor, "1.4551", "0x1.748", Less,
    );
    test(
        "1.0", "0x1", "2.0", "0x2", 10, Ceiling, "1.457", "0x1.75", Greater,
    );
    test(
        "1.0", "0x1", "2.0", "0x2", 10, Down, "1.4551", "0x1.748", Less,
    );
    test(
        "1.0", "0x1", "2.0", "0x2", 10, Up, "1.457", "0x1.75", Greater,
    );
    test(
        "1.0", "0x1", "2.0", "0x2", 10, Nearest, "1.457", "0x1.75", Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        10,
        Nearest,
        "2.9258",
        "0x2.ed",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "2.7182818284590451",
        "0x2.b7e151628aed2",
        100,
        Nearest,
        "2.926108551572304562575742331802",
        "0x2.ed1573358c89c9b00125a2958",
        Greater,
    );
    // Exponents beyond the range of a `Float`, but close to each other: AGM(24 * 2^N, 6 * 2^N) =
    // AGM(24, 6) * 2^N.
    test(
        "5.7e30102999566398119522",
        "0x18.0E+25000000000000000000",
        "1.4e30102999566398119522",
        "0x6.0E+25000000000000000000",
        20,
        Floor,
        "3.183282e30102999566398119522",
        "0xd.754aE+25000000000000000000",
        Less,
    );
    test(
        "5.7e30102999566398119522",
        "0x18.0E+25000000000000000000",
        "1.4e30102999566398119522",
        "0x6.0E+25000000000000000000",
        20,
        Ceiling,
        "3.1832860e30102999566398119522",
        "0xd.754bE+25000000000000000000",
        Greater,
    );
    test(
        "5.7e30102999566398119522",
        "0x18.0E+25000000000000000000",
        "1.4e30102999566398119522",
        "0x6.0E+25000000000000000000",
        20,
        Nearest,
        "3.1832860e30102999566398119522",
        "0xd.754bE+25000000000000000000",
        Greater,
    );
    test(
        "1.0e-30102999566398119520",
        "0x18.0E-25000000000000000000",
        "2.5e-30102999566398119521",
        "0x6.0E-25000000000000000000",
        20,
        Nearest,
        "5.6897963e-30102999566398119521",
        "0xd.754bE-25000000000000000000",
        Greater,
    );
    // The exponents differ by -2^30 + 1, the smallest exponent of a `Float`.
    test(
        "0.50",
        "0x0.8",
        "2.4e-323228497",
        "0x1.0E-268435456",
        20,
        Nearest,
        "1.055273e-9",
        "0x4.8849E-8",
        Greater,
    );
    // The exponents differ by -2^30, just outside that range: one AGM step is taken first.
    test(
        "0.50",
        "0x0.8",
        "1.2e-323228497",
        "0x0.8E-268435456",
        20,
        Nearest,
        "1.055273e-9",
        "0x4.8849E-8",
        Greater,
    );
    // Exponents differing by about 2^35: AGM(1, t) is about π / (2 ln(4/t)).
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Floor,
        "2.9e-11",
        "0x2.0E-9",
        Less,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Ceiling,
        "5.8e-11",
        "0x4.0E-9",
        Greater,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "5.8e-11",
        "0x4.0E-9",
        Greater,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "5.662e-11",
        "0x3.e4E-9",
        Less,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "5.6673e-11",
        "0x3.e5E-9",
        Greater,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Nearest,
        "5.6673e-11",
        "0x3.e5E-9",
        Greater,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Floor,
        "5.66545017700071975318456769448e-11",
        "0x3.e4ad316c65c691baa016b688E-9",
        Less,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Ceiling,
        "5.6654501770007197531845676944896e-11",
        "0x3.e4ad316c65c691baa016b6884E-9",
        Greater,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "5.66545017700071975318456769448e-11",
        "0x3.e4ad316c65c691baa016b688E-9",
        Less,
    );
    test(
        "1.0",
        "0x1",
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1000,
        Nearest,
        "5.665450177000719753184567694486605127990496578255219651319759702359763207033643217472172\
        688631411477634762504502985659870128894971022257128630642713148261378288695579935223642241\
        460381721113838186862673028376127444765958480081442110250645889616426610693101238957080954\
        72943229083733629404772113587109618e-11",
        "0x3.e4ad316c65c691baa016b68816a198f0957267c7a8fe7cfb89ef18b8f1d2cd2166965e075a2057aebf81c\
        d85ee48f4e5114cff1f614c5bb404b1c15cf87760b2b749269555293bb54e7a9aad333590c2b88494fd89a29e4\
        cb28798dd9d77e26235f879ee11f318793f745eea619609add42585b8a1d6772f1dc72aa25cE-9",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1.0",
        "0x1",
        20,
        Nearest,
        "5.6654514e-11",
        "0x3.e4ad4E-9",
        Greater,
    );
    test(
        "4.8e36123599479",
        "0x1.0E+30000000000",
        "3.6e12041199826",
        "0x1.0E+10000000000",
        20,
        Nearest,
        "1.3488012e36123599469",
        "0x1.f256aE+29999999991",
        Greater,
    );
    // Exponents differing by about 2^68.
    test(
        "3.2e-120411998265592478086",
        "0x1.0E-100000000000000000000",
        "1.0",
        "0x1",
        20,
        Nearest,
        "5.665450e-21",
        "0x1.ac11cE-17",
        Greater,
    );
    // Exponents differing by about 2^1098: at the first working precision, the error bound of the
    // more than 1000 AGM steps is too weak, and a higher working precision is needed.
    test(
        "2.8e-120411998265592478085495557889797210707275952584843416524170984450843275709769803794\
        770900847274468816273790876572398151637907152453409402399876933481878230025801185701677361\
        064727893724641177400473561159271433046861775812744771618541553879808095724339844985016160\
        1053250378485915383389273130690735929304786171174030525270193403709",
        "0x1.0E-1000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000",
        "1.0",
        "0x1",
        1,
        Nearest,
        "5.9e-331",
        "0x8.0E-275",
        Greater,
    );
}

#[test]
fn test_agm_rational_prec_round() {
    let test =
        |x: &str, y: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let r = agm_rational_prec_round_all(
                &Rational::from_str(x).unwrap(),
                &Rational::from_str(y).unwrap(),
                prec,
                rm,
            );
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {y} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {y} {prec} {rm}");
        };
    // Special cases follow `Float`'s.
    test("0", "1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", "0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0", "-1", 10, Floor, "0.0", "0x0.0", Equal);
    test("-1", "1", 10, Floor, "NaN", "NaN", Equal);
    test("1", "-1", 10, Exact, "NaN", "NaN", Equal);
    // AGM(x, x) = x.
    test("1", "1", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/3", "1/3", 10, Floor, "0.3330", "0x0.554", Less);
    test("1/3", "1/3", 10, Ceiling, "0.33350", "0x0.556", Greater);
    test("2/3", "1/5", 1, Floor, "0.25", "0x0.4", Less);
    test("2/3", "1/5", 1, Ceiling, "0.50", "0x0.8", Greater);
    test("2/3", "1/5", 1, Nearest, "0.50", "0x0.8", Greater);
    test("2/3", "1/5", 10, Floor, "0.398", "0x0.66", Less);
    test("2/3", "1/5", 10, Ceiling, "0.39893", "0x0.662", Greater);
    test("2/3", "1/5", 10, Nearest, "0.398", "0x0.66", Less);
    test(
        "2/3",
        "1/5",
        100,
        Floor,
        "0.39851137022003454646968546577215",
        "0x0.6604d7562ddfd5c2adcd41dec8",
        Less,
    );
    test(
        "2/3",
        "1/5",
        100,
        Ceiling,
        "0.3985113702200345464696854657725",
        "0x0.6604d7562ddfd5c2adcd41ded",
        Greater,
    );
    test(
        "2/3",
        "1/5",
        100,
        Nearest,
        "0.39851137022003454646968546577215",
        "0x0.6604d7562ddfd5c2adcd41dec8",
        Less,
    );
    test(
        "1/1000000",
        "1000000",
        20,
        Nearest,
        "54133.062",
        "0xd375.1",
        Less,
    );
    test("24", "6", 20, Nearest, "13.458176", "0xd.754b", Greater);
}

#[test]
fn test_agm_big_exponent_float_rational_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                y: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = agm_big_exponent_float_rational_prec_round_all(
            &hex(x_hex),
            &Rational::from_str(y).unwrap(),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {y} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {y} {prec} {rm}");
    };
    // Special cases follow `Float`'s.
    test("NaN", "NaN", "1", 10, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "1", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", "0", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", "-1", 10, Floor, "NaN", "NaN", Equal);
    test(
        "-Infinity",
        "-Infinity",
        "1",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0", "-1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", "0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", "1", 10, Floor, "NaN", "NaN", Equal);
    test("1.0", "0x1", "-1", 10, Floor, "NaN", "NaN", Equal);
    // AGM(x, x) = x.
    test("1.0", "0x1", "1", 10, Exact, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "1/2", 1, Exact, "0.50", "0x0.8", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "884279719003555/281474976710656",
        10,
        Floor,
        "3.141",
        "0x3.24",
        Less,
    );
    test("24.0", "0x18", "1/3", 1, Floor, "4.0", "0x4.0", Less);
    test("24.0", "0x18", "1/3", 1, Ceiling, "8.0", "0x8.0", Greater);
    test("24.0", "0x18", "1/3", 1, Nearest, "8.0", "0x8.0", Greater);
    test("24.0", "0x18", "1/3", 10, Floor, "6.656", "0x6.a8", Less);
    test(
        "24.0", "0x18", "1/3", 10, Ceiling, "6.6641", "0x6.aa", Greater,
    );
    test("24.0", "0x18", "1/3", 10, Nearest, "6.656", "0x6.a8", Less);
    test(
        "24.0",
        "0x18",
        "1/3",
        100,
        Floor,
        "6.6568740624918905960055565225339",
        "0x6.a828e607fe4ae642fac3581f8",
        Less,
    );
    test(
        "24.0",
        "0x18",
        "1/3",
        100,
        Ceiling,
        "6.65687406249189059600555652254",
        "0x6.a828e607fe4ae642fac3582",
        Greater,
    );
    test(
        "24.0",
        "0x18",
        "1/3",
        100,
        Nearest,
        "6.6568740624918905960055565225339",
        "0x6.a828e607fe4ae642fac3581f8",
        Less,
    );
    // A `BigExponentFloat` beyond the range of a `Float`.
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3",
        20,
        Floor,
        "2.053439e12041199816",
        "0x3.e4adE+9999999991",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3",
        20,
        Ceiling,
        "2.0534415e12041199816",
        "0x3.e4ad4E+9999999991",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        "3",
        20,
        Nearest,
        "2.0534415e12041199816",
        "0x3.e4ad4E+9999999991",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        "1/3",
        20,
        Nearest,
        "1.888484e-11",
        "0x1.4c39cE-9",
        Greater,
    );
}

#[test]
#[should_panic]
fn agm_prec_round_fail_1() {
    BigExponentFloat::ONE.agm_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn agm_prec_round_fail_2() {
    BigExponentFloat::ONE.agm_prec_round(BigExponentFloat::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn agm_rational_prec_round_fail_1() {
    BigExponentFloat::agm_rational_prec_round(Rational::ONE, Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn agm_rational_prec_round_fail_2() {
    BigExponentFloat::agm_rational_prec_round(Rational::ONE, Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn agm_big_exponent_float_rational_prec_round_fail_1() {
    BigExponentFloat::ONE.agm_big_exponent_float_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn agm_big_exponent_float_rational_prec_round_fail_2() {
    BigExponentFloat::ONE.agm_big_exponent_float_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
fn agm_prec_round_properties() {
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_15()
        .test_properties(|(x, y, prec, rm)| {
            let r = agm_prec_round_all(&x, &y, prec, rm);
            // `Float`'s AGM agrees with the implementation that does not use it, at any exponents.
            assert!(same_pair(&agm_prec_round_generic(&x, &y, prec, rm), &r));
            check_agm_result(&r, &x, &y, prec);
            assert!(same_pair(&agm_prec_round_all(&y, &x, prec, rm), &r));
            // Multiplying both arguments by 2 doubles the AGM.
            let (s, o) = (&x << 1u32).agm_prec_round(&y << 1u32, prec, rm);
            assert!(same(&s, &(&r.0 << 1u32)));
            assert_eq!(o, r.1);
            if big_exponent_float_exponent_is_small(&y)
                && let Ok(q) = Rational::try_from(&y)
            {
                assert!(same_pair(
                    &agm_big_exponent_float_rational_prec_round_all(&x, &q, prec, rm),
                    &r
                ));
            }
        });
}

#[test]
fn agm_prec_round_float_agreement_properties() {
    let test = |x: Float, y: Float, prec: u64, rm: RoundingMode| {
        let (f, fo) = x.agm_prec_round_ref_ref(&y, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = agm_prec_round_all(
                &float_to_big_exponent_float(&x),
                &float_to_big_exponent_float(&y),
                prec,
                rm,
            );
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    };
    float_float_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(|(x, y, prec, rm)| {
        test(x, y, prec, rm);
    });
    float_float_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, y, prec, rm)| {
            test(x, y, prec, rm);
        },
    );
}

#[test]
fn agm_rational_prec_round_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, prec, rm)| {
            let r = agm_rational_prec_round_all(&x, &y, prec, rm);
            if r.0.is_nan() || r.0.is_zero() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0 > 0u32);
                assert!(r.0.get_prec().unwrap() <= prec);
                let (lo, hi) = if x < y { (&x, &y) } else { (&y, &x) };
                assert!(r.0 >= BigExponentFloat::from_rational_prec_round_ref(lo, prec, Floor).0);
                assert!(r.0 <= BigExponentFloat::from_rational_prec_round_ref(hi, prec, Ceiling).0);
            }
            assert!(same_pair(
                &agm_rational_prec_round_all(&y, &x, prec, rm),
                &r
            ));
            if let Ok(xe) = BigExponentFloat::try_from(&x) {
                assert!(same_pair(
                    &agm_big_exponent_float_rational_prec_round_all(&xe, &y, prec, rm),
                    &r
                ));
                if let Ok(ye) = BigExponentFloat::try_from(&y) {
                    assert!(same_pair(&agm_prec_round_all(&xe, &ye, prec, rm), &r));
                }
            }
            let (f, fo) = Float::agm_rational_prec_round_ref_ref(&x, &y, prec, rm);
            if float_result_comparable(&f, fo) {
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}

#[test]
fn agm_big_exponent_float_rational_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_19().test_properties(
        |(x, y, prec, rm)| {
            let r = agm_big_exponent_float_rational_prec_round_all(&x, &y, prec, rm);
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0 > 0u32);
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            if let Ok(ye) = BigExponentFloat::try_from(&y) {
                assert!(same_pair(&agm_prec_round_all(&x, &ye, prec, rm), &r));
            }
            if x.is_finite() && big_exponent_float_exponent_is_small(&x) {
                let xq = Rational::try_from(&x).unwrap();
                assert!(same_pair(
                    &agm_rational_prec_round_all(&xq, &y, prec, rm),
                    &r
                ));
            }
        },
    );
}

#[test]
fn agm_big_exponent_float_rational_prec_round_float_agreement_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_25().test_properties(
        |(x, y, prec, rm)| {
            let (f, fo) = x.agm_float_rational_prec_round_ref_ref(&y, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = agm_big_exponent_float_rational_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &y,
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
