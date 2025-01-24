// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::traits::Zero;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_q::exhaustive::exhaustive_rationals_with_denominator_inclusive_range;
use malachite_q::Rational;
use std::str::FromStr;

fn helper(d: &str, a: &str, b: &str, out: &str) {
    assert_eq!(
        exhaustive_rationals_with_denominator_inclusive_range(
            Natural::from_str(d).unwrap(),
            Rational::from_str(a).unwrap(),
            Rational::from_str(b).unwrap()
        )
        .collect_vec()
        .to_debug_string(),
        out
    );
}

#[test]
fn test_exhaustive_rationals_with_denominator_inclusive_range() {
    helper("1", "1", "1", "[1]");
    helper("2", "1", "1", "[]");
    helper("3", "1", "1", "[]");
    helper("4", "1", "1", "[]");
    helper("5", "1", "1", "[]");
    helper("6", "1", "1", "[]");

    helper("1", "0", "1", "[0, 1]");
    helper("2", "0", "1", "[1/2]");
    helper("3", "0", "1", "[1/3, 2/3]");
    helper("4", "0", "1", "[1/4, 3/4]");
    helper("5", "0", "1", "[1/5, 2/5, 3/5, 4/5]");
    helper("6", "0", "1", "[1/6, 5/6]");

    helper("1", "0", "1/2", "[0]");
    helper("2", "0", "1/2", "[1/2]");
    helper("3", "0", "1/2", "[1/3]");
    helper("4", "0", "1/2", "[1/4]");
    helper("5", "0", "1/2", "[1/5, 2/5]");
    helper("6", "0", "1/2", "[1/6]");

    helper("1", "1/3", "1/2", "[]");
    helper("2", "1/3", "1/2", "[1/2]");
    helper("3", "1/3", "1/2", "[1/3]");
    helper("4", "1/3", "1/2", "[]");
    helper("5", "1/3", "1/2", "[2/5]");
    helper("6", "1/3", "1/2", "[]");

    helper("1", "-1/2", "-1/3", "[]");
    helper("2", "-1/2", "-1/3", "[-1/2]");
    helper("3", "-1/2", "-1/3", "[-1/3]");
    helper("4", "-1/2", "-1/3", "[]");
    helper("5", "-1/2", "-1/3", "[-2/5]");
    helper("6", "-1/2", "-1/3", "[]");

    helper("1", "-1/2", "1/3", "[0]");
    helper("2", "-1/2", "1/3", "[-1/2]");
    helper("3", "-1/2", "1/3", "[1/3, -1/3]");
    helper("4", "-1/2", "1/3", "[1/4, -1/4]");
    helper("5", "-1/2", "1/3", "[1/5, -1/5, -2/5]");
    helper("6", "-1/2", "1/3", "[1/6, -1/6]");

    // [e, π), roughly
    helper("1", "268876667/98914198", "245850922/78256779", "[3]");
    helper("2", "268876667/98914198", "245850922/78256779", "[]");
    helper("3", "268876667/98914198", "245850922/78256779", "[]");
    helper("4", "268876667/98914198", "245850922/78256779", "[11/4]");
    helper("5", "268876667/98914198", "245850922/78256779", "[14/5]");
    helper("6", "268876667/98914198", "245850922/78256779", "[17/6]");
    helper("7", "268876667/98914198", "245850922/78256779", "[20/7]");
    helper(
        "8",
        "268876667/98914198",
        "245850922/78256779",
        "[23/8, 25/8]",
    );
    helper(
        "9",
        "268876667/98914198",
        "245850922/78256779",
        "[25/9, 26/9, 28/9]",
    );
    helper(
        "10",
        "268876667/98914198",
        "245850922/78256779",
        "[29/10, 31/10]",
    );
    helper(
        "100",
        "268876667/98914198",
        "245850922/78256779",
        "[273/100, 277/100, 279/100, 281/100, 283/100, 287/100, 289/100, 291/100, 293/100, \
        297/100, 299/100, 301/100, 303/100, 307/100, 309/100, 311/100, 313/100]",
    );
}

#[test]
#[should_panic]
fn exhaustive_rationals_with_denominator_inclusive_range_fail_1() {
    exhaustive_rationals_with_denominator_inclusive_range(
        Natural::ZERO,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(1u32, 2),
    );
}

#[test]
#[should_panic]
fn exhaustive_rationals_with_denominator_inclusive_range_fail_2() {
    exhaustive_rationals_with_denominator_inclusive_range(
        Natural::ONE,
        Rational::from_unsigneds(1u32, 2),
        Rational::from_unsigneds(1u32, 3),
    );
}
