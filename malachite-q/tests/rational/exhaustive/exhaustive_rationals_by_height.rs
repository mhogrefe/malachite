// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::exhaustive::{
    exhaustive_negative_rationals_by_height, exhaustive_non_negative_rationals_by_height,
    exhaustive_nonzero_rationals_by_height, exhaustive_positive_rationals_by_height,
    exhaustive_rationals_by_height,
};
use std::collections::HashSet;

#[test]
fn test_exhaustive_positive_rationals_by_height() {
    // This 20-term prefix exercises every branch of the stepping engine:
    // - a fraction below one followed by its reciprocal (1/2 -> 2 and every such pair)
    // - a height exhausted, moving to the next (3/2 -> 1/4: within height 3, the denominator
    //   reaches the numerator)
    // - the next coprime pair within a height (1/3 -> ... -> 2/3: denominator 2 against 3)
    // - a non-coprime pair skipped (4/3 -> 1/5 skips 2/4; 5/4 -> 1/6 skips 2/5's mates 5/5)
    assert_eq!(
        exhaustive_positive_rationals_by_height()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[1, 1/2, 2, 1/3, 3, 2/3, 3/2, 1/4, 4, 3/4, 4/3, 1/5, 5, 2/5, 5/2, 3/5, 5/3, 4/5, 5/4, \
        1/6]"
    );
}

#[test]
fn test_exhaustive_non_negative_rationals_by_height() {
    assert_eq!(
        exhaustive_non_negative_rationals_by_height()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[0, 1, 1/2, 2, 1/3, 3, 2/3, 3/2, 1/4, 4, 3/4, 4/3, 1/5, 5, 2/5, 5/2, 3/5, 5/3, 4/5, 5/4]"
    );
}

#[test]
fn test_exhaustive_negative_rationals_by_height() {
    assert_eq!(
        exhaustive_negative_rationals_by_height()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[-1, -1/2, -2, -1/3, -3, -2/3, -3/2, -1/4, -4, -3/4, -4/3, -1/5, -5, -2/5, -5/2, -3/5, \
        -5/3, -4/5, -5/4, -1/6]"
    );
}

#[test]
fn test_exhaustive_nonzero_rationals_by_height() {
    // - both arms of the sign interleaver: each positive term is pulled and cached, then its
    //   negative is emitted from the cache
    assert_eq!(
        exhaustive_nonzero_rationals_by_height()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3, -3, 2/3, -2/3, 3/2, -3/2, 1/4, -1/4, 4, -4, 3/4, \
        -3/4]"
    );
}

#[test]
fn test_exhaustive_rationals_by_height() {
    assert_eq!(
        exhaustive_rationals_by_height()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[0, 1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3, -3, 2/3, -2/3, 3/2, -3/2, 1/4, -1/4, 4, -4, \
        3/4]"
    );
}

const N: usize = 3000;

#[test]
fn exhaustive_rationals_by_height_properties() {
    // Every positive Rational appears exactly once, and the heights never decrease.
    let xs = exhaustive_positive_rationals_by_height()
        .take(N)
        .collect_vec();
    let mut seen = HashSet::new();
    let mut previous_height = Natural::from(1u32);
    for x in &xs {
        assert!(x.is_valid());
        assert!(*x > 0u32);
        assert!(seen.insert(x.clone()), "{x} was generated twice");
        let height = x.to_height();
        assert!(height >= previous_height, "the height of {x} decreased");
        previous_height = height;
    }
    // Every positive Rational whose height is below the last full height class appears. The classes
    // are complete, so this checks that nothing is skipped.
    let complete_height = xs.last().unwrap().to_height();
    for x in &xs {
        let h = x.to_height();
        if h < complete_height {
            let reciprocal = Rational::from_naturals(x.to_denominator(), x.to_numerator());
            assert!(seen.contains(&reciprocal), "{reciprocal} was skipped");
        }
    }

    // The other four iterators are the positive one shifted, negated, or interleaved.
    assert_eq!(
        exhaustive_non_negative_rationals_by_height()
            .take(N)
            .skip(1)
            .collect_vec(),
        xs[..N - 1]
    );
    assert_eq!(
        exhaustive_negative_rationals_by_height()
            .take(N)
            .map(Abs::abs)
            .collect_vec(),
        xs
    );
    let nonzero = exhaustive_nonzero_rationals_by_height()
        .take(N << 1)
        .collect_vec();
    for (i, x) in xs.iter().enumerate() {
        assert_eq!(nonzero[i << 1], *x);
        assert_eq!(nonzero[(i << 1) + 1], -x);
    }
    let all = exhaustive_rationals_by_height().take(N).collect_vec();
    assert_eq!(all[0], 0u32);
    assert_eq!(all[1..], nonzero[..N - 1]);
}
