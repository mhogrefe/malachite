// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};

#[test]
pub fn test_to_typst() {
    let mut frags = Vec::new();
    fn test_u<T: PrimitiveUnsigned + ToTypst>(x: T, out: &str, frags: &mut Vec<String>) {
        assert_eq!(x.to_typst_string(), out);
        frags.push(out.to_string());
    }
    test_u::<u8>(0, "0", &mut frags);
    test_u::<u8>(5, "5", &mut frags);
    test_u::<u8>(u8::MAX, "255", &mut frags);
    test_u::<u32>(123, "123", &mut frags);
    test_u::<u64>(u64::MAX, "18446744073709551615", &mut frags);
    test_u::<u128>(
        u128::MAX,
        "340282366920938463463374607431768211455",
        &mut frags,
    );

    fn test_i<T: PrimitiveSigned + ToTypst>(x: T, out: &str, frags: &mut Vec<String>) {
        assert_eq!(x.to_typst_string(), out);
        frags.push(out.to_string());
    }
    test_i::<i8>(0, "0", &mut frags);
    test_i::<i8>(-5, "-5", &mut frags);
    test_i::<i16>(-45, "-45", &mut frags);
    test_i::<i64>(i64::MIN, "-9223372036854775808", &mut frags);
    test_i::<i128>(
        i128::MIN,
        "-170141183460469231731687303715884105728",
        &mut frags,
    );
    assert_typst_compiles(&frags);
}

#[test]
pub fn test_to_typst_primitive_float() {
    let mut frags = Vec::new();
    let mut test = |s: String, out: &str| {
        assert_eq!(s, out);
        frags.push(s);
    };
    test(f64::NAN.to_typst_string(), r#""NaN""#);
    test(f64::INFINITY.to_typst_string(), "infinity");
    test(f64::NEGATIVE_INFINITY.to_typst_string(), "-infinity");
    test(0.0f64.to_typst_string(), "0.0");
    test((-0.0f64).to_typst_string(), "-0.0");
    test(1.0f64.to_typst_string(), "1.0");
    test((-1.0f64).to_typst_string(), "-1.0");
    test(0.00123f64.to_typst_string(), "0.00123");
    test(1.0e16f64.to_typst_string(), "1.0 times 10^(16)");
    test(
        f32::MIN_POSITIVE_SUBNORMAL.to_typst_string(),
        "1.0 times 10^(-45)",
    );
    test(
        f64::MAX.to_typst_string(),
        "1.7976931348623157 times 10^(308)",
    );
    assert_typst_compiles(&frags);
}

#[test]
pub fn test_to_typst_embedding() {
    // A fragment carries no delimiters of its own, so it can be substituted into a group.
    assert_eq!(format!("x^({})", 10u8.to_typst()), "x^(10)");
    assert_eq!(format!("x^({})", (-3i8).to_typst()), "x^(-3)");
}

#[test]
fn to_typst_properties() {
    let mut frags = Vec::new();
    unsigned_gen::<u64>().test_properties(|x| {
        let s = x.to_typst_string();
        assert_eq!(s, x.to_string());
        frags.push(s);
    });
    signed_gen::<i64>().test_properties(|x| {
        let s = x.to_typst_string();
        assert_eq!(s, x.to_string());
        frags.push(s);
    });
    assert_typst_compiles(&frags);
}

#[test]
fn to_typst_primitive_float_properties() {
    let mut frags = Vec::new();
    primitive_float_gen::<f64>().test_properties(|x| {
        let s = x.to_typst_string();
        assert!(!s.is_empty());
        // No bare exponent marker survives; it has all become ` times 10^(...)`. (A plain
        // `contains('e')` would not do: `times` has an `e` of its own. Ryu's marker is always
        // preceded by a digit.)
        assert!(
            !s.as_bytes()
                .windows(2)
                .any(|w| w[0].is_ascii_digit() && w[1] == b'e')
        );
        frags.push(s);
    });
    assert_typst_compiles(&frags);
}
