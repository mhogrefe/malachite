// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Square};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::sqrt::sqrt_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen_var_3, big_exponent_float_unsigned_rounding_mode_triple_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_13,
    rational_unsigned_rounding_mode_triple_gen_var_3,
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
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn sqrt_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().sqrt_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.sqrt_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.sqrt_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().sqrt_prec(prec), &r));
        assert!(same_pair(&x.sqrt_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.sqrt_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_sqrt_prec_round() {
    let check = |x: &str, x_hex: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let out = &parse_big_exponent_float_hex(out_hex);
        let r = sqrt_prec_round_all(x, prec, rm);
        assert!(same(&r.0, out), "{x:?} {prec} {rm}: {:?} != {out:?}", r.0);
        assert_eq!(r.1, o);
    };
    check("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    check("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    check("-4.0", "-0x4.0", 10, Floor, "NaN", "NaN", Equal);
    check(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    check("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    check("2.25", "0x2.4", 2, Exact, "1.5", "0x1.8", Equal);
    check("0.25", "0x0.4", 1, Exact, "0.50", "0x0.8", Equal);
    // An even and an odd exponent, each beyond the range of a `Float`.
    check(
        "5.6e60205999132796239042",
        "0x1.0E+50000000000000000000",
        1,
        Exact,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Equal,
    );
    check(
        "1.8e-60205999132796239043",
        "0x1.0E-50000000000000000000",
        1,
        Exact,
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Equal,
    );
    check(
        "1.1e60205999132796239043",
        "0x2.0E+50000000000000000000",
        1,
        Floor,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "1.8",
        "0x1.c",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "1.772453",
        "0x1.c5bf8",
        Less,
    );
}

#[test]
#[should_panic]
fn sqrt_prec_round_fail_1() {
    BigExponentFloat::ONE.sqrt_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_prec_round_fail_2() {
    BigExponentFloat::TWO.sqrt_prec_round(10, Exact);
}

#[test]
fn sqrt_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_4().test_properties(
        |(x, prec, rm)| {
            let r = sqrt_prec_round_all(&x, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(&sqrt_prec_round_generic(&x, prec, rm), &r));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.is_sign_positive());
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // Multiplying x by 4 doubles the square root.
            let (s, o) = (&x << 2u32).sqrt_prec_round(prec, rm);
            assert!(same(&s, &(&r.0 << 1u32)));
            assert_eq!(o, r.1);
            if big_exponent_float_exponent_is_small(&x)
                && x.is_finite()
                && !x.is_zero()
                && x.is_sign_positive()
            {
                // The square root is monotonic, so comparing squares gives the ordering.
                let q = Rational::exact_from(&x);
                assert_eq!(Rational::exact_from(&r.0).square().cmp(&q), r.1);
            }
        },
    );
}

#[test]
fn sqrt_prec_round_properties_2() {
    big_exponent_float_gen_var_3().test_properties(|x| {
        if x.is_finite() && !x.is_zero() {
            // The square root of an exact square is exact.
            let prec = x.get_prec().unwrap();
            assert!(same_pair(
                &(&x).square().sqrt_prec_round(prec, Exact),
                &((&x).abs(), Equal)
            ));
        }
    });
}

#[test]
fn sqrt_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_13().test_properties(|(x, prec, rm)| {
        let (f, fo) = x.sqrt_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = sqrt_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

fn sqrt_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::sqrt_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::sqrt_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::sqrt_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::sqrt_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_sqrt_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
        let r = sqrt_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert!(
            same(&r.0, &parse_big_exponent_float_hex(out_hex)),
            "{x} {prec} {rm}: {:?}",
            r.0
        );
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1/3", 10, Exact, "NaN", "NaN", Equal);
    test("9/4", 2, Exact, "1.5", "0x1.8", Equal);
    test("1/4", 1, Exact, "0.50", "0x0.8", Equal);
    test("3/5", 1, Nearest, "1.0", "0x1.0", Greater);
    test("3/5", 10, Floor, "0.77441", "0x0.c64", Less);
    test("3/5", 10, Ceiling, "0.7754", "0x0.c68", Greater);
    test(
        "3/5",
        100,
        Nearest,
        "0.77459666924148337703585307995660",
        "0x0.c64bf7a1fb923e09790030699",
        Greater,
    );
    test("22/7", 1, Nearest, "2.0", "0x2.0", Greater);
    test("22/7", 10, Floor, "1.7715", "0x1.c58", Less);
    test("22/7", 10, Ceiling, "1.773", "0x1.c6", Greater);
    test(
        "22/7",
        100,
        Nearest,
        "1.7728105208558366565904631364929",
        "0x1.c5d6e909149e8e4e59f04b68e",
        Greater,
    );
    test(
        "1000000000000000000000000000001",
        1,
        Nearest,
        "1.1e15",
        "0x4.0E+12",
        Greater,
    );
    test(
        "1000000000000000000000000000001",
        10,
        Floor,
        "9.9946e14",
        "0x3.8dE+12",
        Less,
    );
    test(
        "1000000000000000000000000000001",
        10,
        Ceiling,
        "1.001e15",
        "0x3.8eE+12",
        Greater,
    );
    test(
        "1000000000000000000000000000001",
        100,
        Nearest,
        "1000000000000000.0000000000000009",
        "0x38d7ea4c68000.0000000000004",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        1,
        Nearest,
        "3.6e-12",
        "0x4.0E-10",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        10,
        Floor,
        "3.162e-12",
        "0x3.7aE-10",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Ceiling,
        "3.1655e-12",
        "0x3.7bE-10",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        100,
        Nearest,
        "3.1622776601683793319988935444332e-12",
        "0x3.7a1a1eafe816fb2fb8d09d43cE-10",
        Greater,
    );
}

#[test]
fn test_sqrt_rational_prec_round_huge() {
    // sqrt(3 * 2^(2^31)) = sqrt(3) * 2^(2^30), whose exponent is beyond `Float`'s range.
    let x = Rational::from(3u32) << (1u64 << 31);
    for rm in [Floor, Ceiling, Nearest] {
        let (y, o) = BigExponentFloat::from(3u32).sqrt_prec_round(20, rm);
        let r = sqrt_rational_prec_round_all(&x, 20, rm);
        assert!(same(&r.0, &(y << (1u64 << 30))));
        assert_eq!(r.1, o);
        assert!(*r.0.get_exponent().unwrap() > Float::MAX_EXPONENT);
    }
}

#[test]
#[should_panic]
fn sqrt_rational_prec_round_fail_1() {
    BigExponentFloat::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 0, Floor);
}

#[test]
#[should_panic]
fn sqrt_rational_prec_round_fail_2() {
    BigExponentFloat::sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 10, Exact);
}

#[test]
fn sqrt_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(x, prec, rm)| {
        let r = sqrt_rational_prec_round_all(&x, prec, rm);
        if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
            assert_eq!(r.1, Equal);
        } else {
            assert!(r.0.get_prec().unwrap() <= prec);
        }
        // The square root is the root of order 2.
        assert!(same_pair(
            &BigExponentFloat::root_u_rational_prec_round_ref(&x, 2, prec, rm),
            &r
        ));
        // Multiplying x by 2^(2000) multiplies the root by 2^(1000).
        if r.0.is_finite() && !r.0.is_zero() {
            let (y, o) = sqrt_rational_prec_round_all(&(&x << 2000u32), prec, rm);
            assert!(same(&y, &(&r.0 << 1000u32)));
            assert_eq!(o, r.1);
        }
        // A dyadic x gives the root of the equal `BigExponentFloat`.
        if let Ok(x_bef) = BigExponentFloat::try_from(&x) {
            assert!(same_pair(&x_bef.sqrt_prec_round(prec, rm), &r));
        }
        // `Float`'s function agrees wherever its result neither overflows nor underflows (at 0,
        // `Float` has signed zeros and infinities).
        if x != 0u32 {
            let (f, fo) = Float::sqrt_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&f, fo) {
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        }
    });
}
