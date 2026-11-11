// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::{ConvertibleFrom, IntegerMantissaAndExponent};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{BigExponentFloat, ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::conversion::primitive_float_from_rational::FloatConversionError;
use std::cmp::Ordering::{self, *};

#[test]
fn test_try_from() {
    let test = |x: BigExponentFloat, out: Result<&str, FloatConversionError>| {
        let result = Float::try_from(&x);
        assert_eq!(
            result
                .as_ref()
                .map(|f| format!("{:#x}", ComparableFloatRef(f)))
                .map_err(|e| *e),
            out.map(ToString::to_string)
        );
        assert_eq!(Float::convertible_from(&x), out.is_ok());
        let result_alt = Float::try_from(x);
        assert_eq!(result_alt.is_ok(), out.is_ok());
    };
    test(BigExponentFloat::NAN, Ok("NaN"));
    test(BigExponentFloat::INFINITY, Ok("Infinity"));
    test(BigExponentFloat::NEGATIVE_INFINITY, Ok("-Infinity"));
    test(BigExponentFloat::ZERO, Ok("0x0.0"));
    test(BigExponentFloat::ONE, Ok("0x1.0#1"));
    test(parse_big_exponent_float_hex("-0x1.8#2"), Ok("-0x1.8#2"));
    // - the largest and smallest exponents of a `Float`
    test(
        big_exponent_float_power_of_2(Integer::from(Float::MAX_EXPONENT - 1)),
        Ok("0x4.0E+268435455#1"),
    );
    test(
        big_exponent_float_power_of_2(Integer::from(Float::MIN_EXPONENT - 1)),
        Ok("0x1.0E-268435456#1"),
    );
    // - just outside them
    test(
        big_exponent_float_power_of_2(Integer::from(Float::MAX_EXPONENT)),
        Err(FloatConversionError::Overflow),
    );
    test(
        big_exponent_float_power_of_2(Integer::from(Float::MIN_EXPONENT - 2)),
        Err(FloatConversionError::Underflow),
    );
}

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_from_big_exponent_float_prec_round() {
    let test = |x: &BigExponentFloat, prec, rm, out: &str, o: Ordering| {
        let (f, o_out) = Float::from_big_exponent_float_prec_round_ref(x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(format!("{:#x}", ComparableFloat(f)), out);
        assert_eq!(o_out, o);
        let (f_alt, o_alt) = Float::from_big_exponent_float_prec_round(x.clone(), prec, rm);
        assert_eq!(format!("{:#x}", ComparableFloat(f_alt)), out);
        assert_eq!(o_alt, o);
    };
    test(&BigExponentFloat::NAN, 1, Floor, "NaN", Equal);
    test(&BigExponentFloat::ZERO, 10, Floor, "0x0.0", Equal);
    let x = parse_big_exponent_float_hex("0x1.c#3");
    test(&x, 2, Floor, "0x1.8#2", Less);
    test(&x, 2, Ceiling, "0x2.0#2", Greater);
    test(&x, 10, Exact, "0x1.c00#10", Equal);
    // - overflow
    let big = big_exponent_float_power_of_2(Integer::from(1u64 << 40));
    test(&big, 2, Nearest, "Infinity", Greater);
    test(&big, 2, Up, "Infinity", Greater);
    test(&big, 2, Floor, "0x6.0E+268435455#2", Less);
    test(&big, 2, Down, "0x6.0E+268435455#2", Less);
    let neg_big = big_exponent_float_negate(big.clone());
    test(&neg_big, 2, Floor, "-Infinity", Less);
    test(&neg_big, 2, Ceiling, "-0x6.0E+268435455#2", Greater);
    // - rounding up carries into overflow
    let edge = parse_big_exponent_float_hex("0x7.0E+268435455#3");
    test(&edge, 2, Up, "Infinity", Greater);
    // - underflow
    let tiny = big_exponent_float_power_of_2(-Integer::from(1u64 << 40));
    test(&tiny, 2, Nearest, "0x0.0", Less);
    test(&tiny, 2, Floor, "0x0.0", Less);
    test(&tiny, 2, Up, "0x1.0E-268435456#2", Greater);
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    test(&neg_tiny, 2, Nearest, "-0x0.0", Greater);
    test(&neg_tiny, 2, Floor, "-0x1.0E-268435456#2", Less);
    // - exactly half of the smallest positive `Float` rounds to zero; more than half rounds up
    let half_min = big_exponent_float_power_of_2(Integer::from(Float::MIN_EXPONENT - 2));
    test(&half_min, 2, Nearest, "0x0.0", Less);
    let above_half_min = BigExponentFloat::from_integer_mantissa_and_exponent(
        Natural::from(3u32),
        Integer::from(Float::MIN_EXPONENT - 3),
    )
    .unwrap();
    test(&above_half_min, 2, Nearest, "0x1.0E-268435456#2", Greater);
}

#[test]
#[should_panic]
fn from_big_exponent_float_prec_round_fail_1() {
    Float::from_big_exponent_float_prec_round(BigExponentFloat::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn from_big_exponent_float_prec_round_fail_2() {
    let tiny = big_exponent_float_power_of_2(-Integer::from(1u64 << 40));
    Float::from_big_exponent_float_prec_round(tiny, 1, Exact);
}

#[test]
fn float_from_big_exponent_float_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let result = Float::try_from(&x);
        assert_eq!(Float::convertible_from(&x), result.is_ok());
        if let Ok(f) = result {
            assert!(f.is_valid());
            assert_eq!(
                format!("{:#x}", BigExponentFloat::from(&f)),
                format!("{x:#x}")
            );
            assert_eq!(f.get_prec(), x.get_prec());
        }
    });

    big_exponent_float_unsigned_rounding_mode_triple_gen_var_1().test_properties(
        |(x, prec, rm)| {
            // - `Exact` panics when the value overflows or underflows a `Float`
            if rm == Exact && !Float::convertible_from(&x) {
                return;
            }
            let (f, o) = Float::from_big_exponent_float_prec_round_ref(&x, prec, rm);
            assert!(f.is_valid());
            let (f_alt, o_alt) = Float::from_big_exponent_float_prec_round(x.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&f_alt));
            assert_eq!(o, o_alt);
            if rm == Nearest {
                let (f_alt, o_alt) = Float::from_big_exponent_float_prec_ref(&x, prec);
                assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&f_alt));
                assert_eq!(o, o_alt);
                let (f_alt, o_alt) = Float::from_big_exponent_float_prec(x.clone(), prec);
                assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&f_alt));
                assert_eq!(o, o_alt);
            }
            // - agreement with the conversion from the exact `Rational`, which handles overflow and
            //   underflow the same way
            if x.is_normal()
                && x.get_exponent().unwrap().unsigned_abs_ref() < &Natural::from(10000u32)
            {
                let q = Rational::try_from(&x).unwrap();
                let (g, o_g) = Float::from_rational_prec_round(q, prec, rm);
                assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&g));
                assert_eq!(o, o_g);
            }
        },
    );

    // - for each rounding mode, overflow and underflow agree with `from_rational_prec_round` at the
    //   edges of `Float`'s range
    for e in [
        Float::MAX_EXPONENT - 1,
        Float::MAX_EXPONENT,
        Float::MIN_EXPONENT - 1,
        Float::MIN_EXPONENT - 2,
        Float::MIN_EXPONENT - 3,
    ] {
        for m in [1u32, 3, 5, 7] {
            let x = BigExponentFloat::from_integer_mantissa_and_exponent(
                Natural::from(m),
                Integer::from(e) - Integer::from(2u32),
            )
            .unwrap();
            for x in [x.clone(), big_exponent_float_negate(x)] {
                for rm in ROUNDING_MODES {
                    for prec in [1, 2, 3] {
                        let (f, o) = Float::from_big_exponent_float_prec_round_ref(&x, prec, rm);
                        let (g, o_g) = Float::from_rational_prec_round(
                            Rational::try_from(&x).unwrap(),
                            prec,
                            rm,
                        );
                        assert_eq!(ComparableFloat(f), ComparableFloat(g), "{x:?} {prec} {rm}");
                        assert_eq!(o, o_g);
                    }
                }
            }
        }
    }
}
