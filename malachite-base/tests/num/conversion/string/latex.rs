// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

#[test]
pub fn test_to_latex() {
    fn test_u<T: PrimitiveUnsigned + ToLatex>(x: T, out: &str) {
        assert_eq!(x.to_latex().to_string(), out);
    }
    test_u::<u8>(0, "0");
    test_u::<u8>(5, "5");
    test_u::<u8>(u8::MAX, "255");
    test_u::<u32>(123, "123");
    test_u::<u64>(u64::MAX, "18446744073709551615");
    test_u::<u128>(u128::MAX, "340282366920938463463374607431768211455");

    fn test_i<T: PrimitiveSigned + ToLatex>(x: T, out: &str) {
        assert_eq!(x.to_latex().to_string(), out);
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
        let s = x.to_latex().to_string();
        // The defining contract for primitive integers: the fragment is the `Display` output.
        assert_eq!(s, x.to_string());
        // It is a bare fragment: no math-mode delimiters, and nothing needing escaping.
        assert!(!s.is_empty());
        assert!(s.chars().all(|c| c.is_ascii_digit()));
    });
}

fn to_latex_helper_signed<T: PrimitiveSigned + ToLatex>() {
    signed_gen::<T>().test_properties(|x| {
        let s = x.to_latex().to_string();
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
