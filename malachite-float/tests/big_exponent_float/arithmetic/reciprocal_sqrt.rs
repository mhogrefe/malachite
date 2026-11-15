// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::arithmetic::reciprocal_sqrt::reciprocal_sqrt_prec_round_generic;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_5,
    float_unsigned_rounding_mode_triple_gen_var_15,
};
use malachite_float::{BigExponentFloat, Float};
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

// Whether `x` is finite and nonzero.
const fn finite_nonzero(x: &BigExponentFloat) -> bool {
    x.is_finite() && !x.is_zero()
}

fn reciprocal_sqrt_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().reciprocal_sqrt_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.reciprocal_sqrt_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.reciprocal_sqrt_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().reciprocal_sqrt_prec(prec), &r));
        assert!(same_pair(&x.reciprocal_sqrt_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.reciprocal_sqrt_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn check(x: &str, x_hex: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering) {
    assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
    let x = &parse_big_exponent_float_hex(x_hex);
    assert_eq!(parse_big_exponent_float_hex(out_hex).to_string(), out);
    let out = &parse_big_exponent_float_hex(out_hex);
    let r = reciprocal_sqrt_prec_round_all(x, prec, rm);
    assert!(same(&r.0, out), "{x:?} {prec} {rm}: {:?} != {out:?}", r.0);
    assert_eq!(r.1, o);
    assert!(same_pair(
        &reciprocal_sqrt_prec_round_generic(x, prec, rm),
        &r
    ));
}

#[test]
fn test_reciprocal_sqrt_prec_round() {
    check("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    check("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    check("-4.0", "-0x4.0", 10, Floor, "NaN", "NaN", Equal);
    check("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    check("0.0", "0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    check("0.25", "0x0.4", 1, Exact, "2.0", "0x2.0", Equal);
    check("4.0", "0x4.0", 1, Exact, "0.50", "0x0.8", Equal);
    // An even exponent beyond the range of a `Float`: 1/sqrt(2^(2N)) = 2^(-N).
    check(
        "5.6e60205999132796239042",
        "0x1.0E+50000000000000000000",
        1,
        Exact,
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Equal,
    );
    check(
        "1.8e-60205999132796239043",
        "0x1.0E-50000000000000000000",
        1,
        Exact,
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Equal,
    );
    // An odd one: 1/sqrt(2^(2N + 1)) = 2^(-N - 1) * sqrt(2), and sqrt(2) rounds down to 1 and up to
    // 2 at 1 bit.
    check(
        "1.1e60205999132796239043",
        "0x2.0E+50000000000000000000",
        1,
        Floor,
        "2.1e-30102999566398119522",
        "0x8.0E-25000000000000000001",
        Less,
    );
    check(
        "1.1e60205999132796239043",
        "0x2.0E+50000000000000000000",
        1,
        Ceiling,
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Floor,
        "0.562",
        "0x0.9",
        Less,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        5,
        Ceiling,
        "0.594",
        "0x0.98",
        Greater,
    );
    check(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        20,
        Nearest,
        "0.5641899",
        "0x0.906ec",
        Greater,
    );
}

#[test]
#[should_panic]
fn reciprocal_sqrt_prec_round_fail_1() {
    BigExponentFloat::ONE.reciprocal_sqrt_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn reciprocal_sqrt_prec_round_fail_2() {
    BigExponentFloat::TWO.reciprocal_sqrt_prec_round(10, Exact);
}

#[test]
fn reciprocal_sqrt_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_5().test_properties(
        |(x, prec, rm)| {
            let r = reciprocal_sqrt_prec_round_all(&x, prec, rm);
            // The kernels agree with the kernel-free implementation, at any exponents.
            assert!(same_pair(
                &reciprocal_sqrt_prec_round_generic(&x, prec, rm),
                &r
            ));
            if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
                assert_eq!(r.1, Equal);
            } else {
                assert!(r.0.is_sign_positive());
                assert!(r.0.get_prec().unwrap() <= prec);
            }
            // Multiplying x by 4 halves the reciprocal square root.
            let (s, o) = (&x << 2u32).reciprocal_sqrt_prec_round(prec, rm);
            assert!(same(&s, &(&r.0 >> 1u32)));
            assert_eq!(o, r.1);
            if finite_nonzero(&x) && x.is_sign_positive() {
                // The function is decreasing, so r^2 * x is greater than 1 iff r is too large.
                assert_eq!(
                    ((&r.0).square() * &x).partial_cmp(&BigExponentFloat::ONE),
                    Some(r.1)
                );
            }
        },
    );
}

#[test]
fn reciprocal_sqrt_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_15().test_properties(|(x, prec, rm)| {
        let (f, fo) = x.reciprocal_sqrt_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = reciprocal_sqrt_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
