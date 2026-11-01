// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::conversion::digits::foer_digits::FoerDigit;
use malachite_q::test_util::generators::rational_natural_pair_gen_var_2;
use std::str::FromStr;

// Renders digits as in `08[(3)[(3)`: a repeating digit is parenthesized, and `[` marks the start of
// each repetition of the repeating part.
fn render(digits: impl Iterator<Item = FoerDigit>) -> String {
    let mut s = String::new();
    for d in digits {
        if d.period_start {
            s.push('[');
        }
        if d.repeating {
            s.push_str(&format!("({})", d.digit));
        } else {
            s.push_str(&d.digit.to_string());
        }
    }
    s
}

#[test]
fn test_foer_digits() {
    let test = |x, base, take, before_out, non_repeating_len, terminating, after_out| {
        let x = Rational::from_str(x).unwrap();
        let base = Natural::from(base);
        let (before_point, after_point) = x.foer_digits(&base);
        assert_eq!(before_point.to_debug_string(), before_out);
        assert_eq!(after_point.non_repeating_len(), non_repeating_len);
        assert_eq!(after_point.is_terminating(), terminating);
        assert_eq!(render(after_point.take(take)), after_out);
    };
    test("3", 10u32, 5, "[3]", 0, true, "");
    test("1/8", 10u32, 5, "[]", 3, true, "125");
    test("25/12", 10u32, 6, "[2]", 2, false, "08[(3)[(3)[(3)[(3)");
    test(
        "1/7",
        10u32,
        14,
        "[]",
        0,
        false,
        "[(1)(4)(2)(8)(5)(7)[(1)(4)(2)(8)(5)(7)[(1)(4)",
    );
    test(
        "-22/7",
        10u32,
        8,
        "[3]",
        0,
        false,
        "[(1)(4)(2)(8)(5)(7)[(1)(4)",
    );
    // - base with several prime factors
    test("5/24", 6u32, 6, "[]", 3, true, "113");
    // - power-of-2 base
    test("1/6", 2u32, 7, "[]", 1, false, "0[(0)(1)[(0)(1)[(0)(1)");
    test("1/6", 4u32, 4, "[]", 1, false, "0[(2)[(2)[(2)");
}

#[test]
#[should_panic]
fn foer_digits_fail_1() {
    Rational::ONE.foer_digits(&Natural::ONE);
}

#[test]
#[should_panic]
fn foer_digits_fail_2() {
    Rational::ONE.foer_digits(&Natural::ZERO);
}

#[test]
fn foer_digits_properties() {
    rational_natural_pair_gen_var_2().test_properties(|(x, base)| {
        let (before_point, after_point) = x.foer_digits(&base);
        let (before_point_alt, foer) = x.to_digits(&base);
        assert_eq!(before_point, before_point_alt);
        let (non_repeating, repeating) = foer.slices_ref();
        let k = non_repeating.len();
        let period = repeating.len();
        assert_eq!(after_point.non_repeating_len(), u64::try_from(k).unwrap());
        assert_eq!(after_point.is_terminating(), period == 0);
        let digits: Vec<FoerDigit> = after_point.take(k + 2 * period + 1).collect();
        assert_eq!(
            digits.len(),
            if period == 0 { k } else { k + 2 * period + 1 }
        );
        for (i, d) in digits.iter().enumerate() {
            let expected = if i < k {
                &non_repeating[i]
            } else {
                &repeating[(i - k) % period]
            };
            assert_eq!(&d.digit, expected);
            assert_eq!(d.repeating, i >= k);
            assert_eq!(d.period_start, i >= k && (i - k) % period == 0);
        }
    });
}
