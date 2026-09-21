// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use std::str::FromStr;

#[test]
pub fn test_to_latex() {
    fn test_u<T: PrimitiveUnsigned + ToLatex>(x: T, out: &str) {
        assert_eq!(x.to_latex_string(), out);
    }
    test_u::<u8>(0, "0");
    test_u::<u8>(5, "5");
    test_u::<u8>(u8::MAX, "255");
    test_u::<u32>(123, "123");
    test_u::<u64>(u64::MAX, "18446744073709551615");
    test_u::<u128>(u128::MAX, "340282366920938463463374607431768211455");

    fn test_i<T: PrimitiveSigned + ToLatex>(x: T, out: &str) {
        assert_eq!(x.to_latex_string(), out);
    }
    test_i::<i8>(0, "0");
    test_i::<i8>(-5, "-5");
    test_i::<i16>(-45, "-45");
    test_i::<i64>(i64::MIN, "-9223372036854775808");
    test_i::<i128>(i128::MIN, "-170141183460469231731687303715884105728");
}

#[test]
pub fn test_to_latex_embedding() {
    // A fragment carries no delimiters of its own, so it can be substituted into a group.
    assert_eq!(format!("x^{{{}}}", 10u8.to_latex()), "x^{10}");
    assert_eq!(format!("x^{{{}}}", (-3i8).to_latex()), "x^{-3}");
}

fn to_latex_helper_unsigned<T: PrimitiveUnsigned + ToLatex>() {
    unsigned_gen::<T>().test_properties(|x| {
        let s = x.to_latex_string();
        // The defining contract for primitive integers: the fragment is the `Display` output.
        assert_eq!(s, x.to_string());
        // It is a bare fragment: no math-mode delimiters, and nothing needing escaping.
        assert!(!s.is_empty());
        assert!(s.chars().all(|c| c.is_ascii_digit()));
    });
}

fn to_latex_helper_signed<T: PrimitiveSigned + ToLatex>() {
    signed_gen::<T>().test_properties(|x| {
        let s = x.to_latex_string();
        assert_eq!(s, x.to_string());
        assert!(!s.is_empty());
        assert!(
            s.strip_prefix('-')
                .unwrap_or(&s)
                .chars()
                .all(|c| c.is_ascii_digit())
        );
        assert_eq!(s.starts_with('-'), x < T::ZERO);
    });
}

#[test]
fn to_latex_properties() {
    apply_fn_to_unsigneds!(to_latex_helper_unsigned);
    apply_fn_to_signeds!(to_latex_helper_signed);
}

#[test]
pub fn test_to_latex_primitive_float() {
    fn test<T: PrimitiveFloat + ToLatex>(x: T, out: &str) {
        assert_eq!(x.to_latex_string(), out);
    }
    // specials
    test::<f64>(f64::NAN, r"\text{NaN}");
    test::<f64>(f64::INFINITY, r"\infty");
    test::<f64>(f64::NEGATIVE_INFINITY, r"-\infty");
    test::<f32>(f32::NAN, r"\text{NaN}");
    test::<f32>(f32::INFINITY, r"\infty");
    test::<f32>(f32::NEGATIVE_INFINITY, r"-\infty");
    // zeros keep their signs
    test::<f64>(0.0, "0.0");
    test::<f64>(-0.0, "-0.0");
    test::<f32>(0.0, "0.0");
    test::<f32>(-0.0, "-0.0");
    // finite values whose shortest representation has no exponent pass through
    test::<f64>(1.0, "1.0");
    test::<f64>(-1.0, "-1.0");
    test::<f64>(0.5, "0.5");
    test::<f64>(100.0, "100.0");
    test::<f64>(0.00123, "0.00123");
    test::<f64>(core::f64::consts::PI, "3.141592653589793");
    // and those that do have one get the LaTeX form, with a braced exponent
    test::<f64>(1.0e16, r"1.0 \times 10^{16}");
    test::<f64>(1.0e-7, r"1.0 \times 10^{-7}");
    test::<f64>(-4.5e-9, r"-4.5 \times 10^{-9}");
    test::<f64>(f64::MAX, r"1.7976931348623157 \times 10^{308}");
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, r"5.0 \times 10^{-324}");
    test::<f32>(f32::MAX, r"3.4028235 \times 10^{38}");
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, r"1.0 \times 10^{-45}");
}

// Rewrites a fragment back into the `NiceFloat` string it came from, so that the transform can be
// checked for losslessness.
fn undo_latex(s: &str) -> String {
    const TIMES: &str = r" \times 10^";
    if let Some(i) = s.find(TIMES) {
        let mantissa = &s[..i];
        let exponent = &s[i + TIMES.len()..];
        let exponent = exponent
            .strip_prefix('{')
            .and_then(|e| e.strip_suffix('}'))
            .unwrap_or(exponent);
        format!("{mantissa}e{exponent}")
    } else {
        s.to_string()
    }
}

fn to_latex_helper_primitive_float<T: PrimitiveFloat + ToLatex>()
where
    NiceFloat<T>: FromStr,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let s = x.to_latex_string();
        assert!(!s.is_empty());
        assert_eq!(s.matches('{').count(), s.matches('}').count());
        if x.is_nan() {
            assert_eq!(s, r"\text{NaN}");
        } else if x.is_infinite() {
            assert_eq!(
                s,
                if x.is_sign_positive() {
                    r"\infty"
                } else {
                    r"-\infty"
                }
            );
        } else {
            // No bare exponent marker survives; it has all become ` \times 10^`. (A plain
            // `contains('e')` would not do: `\times` has an `e` of its own. Ryu's marker is always
            // preceded by a digit.)
            assert!(
                !s.as_bytes()
                    .windows(2)
                    .any(|w| w[0].is_ascii_digit() && w[1] == b'e')
            );
            // The rewrite is lossless: undoing it recovers the `NiceFloat` string exactly, and that
            // string parses back to the very same float.
            let undone = undo_latex(&s);
            assert_eq!(undone, NiceFloat(x).to_string());
            assert_eq!(
                NiceFloat::<T>::from_str(&undone).ok().unwrap(),
                NiceFloat(x)
            );
            // An exponent is braced unless it is a single digit, and a single digit is positive.
            if let Some(i) = s.find(r" \times 10^") {
                let exponent = &s[i + r" \times 10^".len()..];
                if exponent.starts_with('{') {
                    assert!(exponent.ends_with('}'));
                } else {
                    assert_eq!(exponent.len(), 1);
                    assert!(exponent.chars().next().unwrap().is_ascii_digit());
                }
            }
        }
    });
}

#[test]
fn to_latex_primitive_float_properties() {
    apply_fn_to_primitive_floats!(to_latex_helper_primitive_float);
}
