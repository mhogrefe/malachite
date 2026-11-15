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
};
use malachite_float::{BigExponentFloat, Float};
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
