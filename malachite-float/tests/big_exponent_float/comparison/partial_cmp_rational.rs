// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf, Zero};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_rational_pair_gen, float_rational_pair_gen,
};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

#[test]
fn test_partial_cmp_rational() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let test = |x: BigExponentFloat, y: &str, out: Option<Ordering>| {
        let y = Rational::from_str(y).unwrap();
        assert_eq!(x.partial_cmp(&y), out);
        assert_eq!(y.partial_cmp(&x), out.map(Ordering::reverse));
        assert_eq!(x == y, out == Some(Equal));
        assert_eq!(y == x, out == Some(Equal));
    };
    test(BigExponentFloat::NAN, "0", None);
    test(BigExponentFloat::INFINITY, "0", Some(Greater));
    test(BigExponentFloat::NEGATIVE_INFINITY, "0", Some(Less));
    test(BigExponentFloat::ZERO, "0", Some(Equal));
    test(BigExponentFloat::ZERO, "1", Some(Less));
    test(BigExponentFloat::ONE, "1", Some(Equal));
    test(BigExponentFloat::ONE_HALF, "1", Some(Less));
    test(BigExponentFloat::ONE, "0", Some(Greater));
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "3",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "4",
        Some(Less),
    );
    test(parse_big_exponent_float_hex("0xff.0#8"), "255", Some(Equal));
    test(
        parse_big_exponent_float_hex("0xff.8#9"),
        "255",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("0x1.0E+40#1"),
        "1461501637330902918203684832716283019655932542976",
        Some(Equal),
    );
    test(
        parse_big_exponent_float_hex("0x1.0E+40#1"),
        "1461501637330902918203684832716283019655932542977",
        Some(Less),
    );
    test(huge.clone(), "0", Some(Greater));
    test(
        huge.clone(),
        "1461501637330902918203684832716283019655932542976",
        Some(Greater),
    );
    test(tiny.clone(), "0", Some(Greater));
    test(tiny.clone(), "1", Some(Less));
    test(neg_huge.clone(), "0", Some(Less));
    test(neg_tiny.clone(), "0", Some(Less));
    test(BigExponentFloat::NEGATIVE_INFINITY, "-1000", Some(Less));
    test(BigExponentFloat::NAN, "-1", None);
    test(BigExponentFloat::ZERO, "-1", Some(Greater));
    test(
        parse_big_exponent_float_hex("-0x3.243f6a8885a30#53"),
        "-3",
        Some(Less),
    );
    test(
        parse_big_exponent_float_hex("-0x3.243f6a8885a30#53"),
        "-4",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("-0xff.0#8"),
        "-255",
        Some(Equal),
    );
    test(huge.clone(), "-1", Some(Greater));
    test(
        neg_huge.clone(),
        "-1000000000000000000000000000000",
        Some(Less),
    );
    test(neg_tiny.clone(), "-1", Some(Greater));
    test(tiny.clone(), "-1", Some(Greater));
    test(BigExponentFloat::ONE_HALF, "1/2", Some(Equal));
    test(BigExponentFloat::ONE_HALF, "1/3", Some(Greater));
    test(
        parse_big_exponent_float_hex("0x0.55555555555554#53"),
        "1/3",
        Some(Less),
    );
    test(
        parse_big_exponent_float_hex("-0x0.55555555555554#53"),
        "-1/3",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "22/7",
        Some(Less),
    );
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "355/113",
        Some(Less),
    );
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "333/106",
        Some(Greater),
    );
    test(
        tiny.clone(),
        "1/1000000000000000000000000000000",
        Some(Less),
    );
    test(
        neg_tiny.clone(),
        "-1/1000000000000000000000000000000",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("0x1.0E-40#1"),
        "1/1461501637330902918203684832716283019655932542976",
        Some(Equal),
    );
    test(
        parse_big_exponent_float_hex("0x1.0E-40#1"),
        "1/1461501637330902918203684832716283019655932542975",
        Some(Less),
    );
}

// Compares a `BigExponentFloat` with a `Rational` by bracketing the `Rational` between its floor
// and ceiling at a precision at least that of the `BigExponentFloat`, without forming either
// exactly.
fn big_exponent_float_partial_cmp_rational_oracle(
    x: &BigExponentFloat,
    y: &Rational,
) -> Option<Ordering> {
    let prec = x.get_prec().unwrap_or(1);
    let lo = BigExponentFloat::from_rational_prec_round_ref(y, prec, Floor).0;
    let hi = BigExponentFloat::from_rational_prec_round_ref(y, prec, Ceiling).0;
    let lo_cmp = x.partial_cmp(&lo)?;
    if lo == hi {
        Some(lo_cmp)
    } else if lo_cmp != Greater {
        Some(Less)
    } else {
        assert_ne!(x.partial_cmp(&hi), Some(Less));
        Some(Greater)
    }
}

#[test]
fn partial_cmp_rational_properties() {
    big_exponent_float_rational_pair_gen().test_properties(|(x, y)| {
        let ord = x.partial_cmp(&y);
        assert_eq!(y.partial_cmp(&x).map(Ordering::reverse), ord);
        assert_eq!(ord, big_exponent_float_partial_cmp_rational_oracle(&x, &y));
    });

    // - agreement with `Float`
    float_rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).partial_cmp(&y),
            x.partial_cmp(&y)
        );
    });
}
