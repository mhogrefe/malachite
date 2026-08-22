// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::balanced_fold;
use malachite_base::test_util::generators::unsigned_vec_gen;

#[test]
fn test_balanced_fold() {
    // empty
    assert_eq!(
        balanced_fold(std::iter::empty::<u64>(), |_| false, |a, b| *a += b),
        None
    );
    // single
    assert_eq!(
        balanced_fold([5u64].into_iter(), |_| false, |a, b| *a += b),
        Some(5)
    );
    // the tree shape is balanced and order-preserving
    for n in 1usize..=33 {
        let s = balanced_fold(
            (0..n).map(|i| i.to_string()),
            |_| false,
            |a, b| {
                *a = format!("({a} {b})");
            },
        )
        .unwrap();
        // every element appears once, in order
        let mut last = None;
        for tok in s.split(['(', ')', ' ']) {
            if tok.is_empty() {
                continue;
            }
            let v: usize = tok.parse().unwrap();
            if let Some(prev) = last {
                assert_eq!(v, prev + 1);
            }
            last = Some(v);
        }
        assert_eq!(last, Some(n - 1));
    }
    // absorbing short-circuits and does not consume the rest
    let mut consumed = 0;
    let result = balanced_fold(
        [3u64, 4, 0, 7].iter().inspect(|_| consumed += 1).copied(),
        |&x| x == 0,
        |a, b| *a *= b,
    );
    assert_eq!(result, Some(0));
    assert_eq!(consumed, 3);
}

#[test]
fn balanced_fold_properties() {
    unsigned_vec_gen::<u64>().test_properties(|xs| {
        // For an associative, commutative operation, the balanced fold agrees with the linear
        // fold.
        let sum = balanced_fold(xs.iter().copied(), |_| false, |a, b| *a = a.wrapping_add(b));
        assert_eq!(
            sum,
            if xs.is_empty() {
                None
            } else {
                Some(xs.iter().fold(0u64, |a, b| a.wrapping_add(*b)))
            }
        );
        // Order preservation for a non-commutative operation: concatenation.
        let cat = balanced_fold(
            xs.iter().map(u64::to_string),
            |_| false,
            |a, b| a.push_str(&b),
        );
        assert_eq!(
            cat,
            if xs.is_empty() {
                None
            } else {
                Some(xs.iter().map(u64::to_string).collect::<String>())
            }
        );
    });
}
