// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::strings::typst::ToTypst;
use malachite_float::test_util::generators::float_gen;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

// Turns the power of ten back into the `e` that `Display` writes, so that a fragment can be
// compared with it directly.
fn undo_power_of_ten(s: &str) -> String {
    const TIMES: &str = " times 10^";
    s.find(TIMES).map_or_else(
        || s.to_string(),
        |i| {
            let mantissa = &s[..i];
            let exponent = &s[i + TIMES.len()..];
            let exponent = exponent
                .strip_prefix('(')
                .and_then(|e| e.strip_suffix(')'))
                .unwrap_or(exponent);
            format!("{mantissa}e{exponent}")
        },
    )
}

#[test]
fn test_float_to_typst() {
    let test = |x: Float, out: &str| assert_eq!(x.to_typst_string(), out);
    // the specials, as the primitive floats write them
    test(Float::NAN, r#""NaN""#);
    test(Float::INFINITY, "infinity");
    test(Float::NEGATIVE_INFINITY, "-infinity");
    // the two zeros are kept apart, as `Display` keeps them apart
    test(Float::ZERO, "0.0");
    test(Float::NEGATIVE_ZERO, "-0.0");
    // ordinary values are written as `Display` writes them
    test(Float::ONE, "1.0");
    test(Float::NEGATIVE_ONE, "-1.0");
    test(Float::from(1.5), "1.5");
    test(Float::from(255), "255.0");
    // an exponent is lifted into a real power of ten
    test(Float::power_of_2(100u64), "1.3 times 10^(30)");
    test(Float::power_of_2(-100i64), "7.9 times 10^(-31)");
    // the digit count follows the precision, not the value
    test(Float::one_prec(100), "1.0000000000000000000000000000000");
}

#[test]
fn float_to_typst_properties() {
    float_gen().test_properties(|x| {
        let s = x.to_typst_string();
        assert!(!s.is_empty());
        // Undoing the power of ten gives back exactly what `Display` writes, for everything but the
        // specials, which have spellings of their own.
        if !x.is_nan() && !x.is_infinite() {
            assert_eq!(undo_power_of_ten(&s), x.to_string());
            // No bare exponent marker survives. (A plain `contains('e')` would not do: the
            // separator has an `e` of its own. The marker is always preceded by a digit.)
            assert!(
                !s.as_bytes()
                    .windows(2)
                    .any(|w| w[0].is_ascii_digit() && w[1] == b'e')
            );
        }
        // The wrappers tell more `Float`s apart than the usual comparisons do, but they do not
        // change what the value is.
        assert_eq!(ComparableFloat(x.clone()).to_typst_string(), s);
        assert_eq!(ComparableFloatRef(&x).to_typst_string(), s);
    });
}
