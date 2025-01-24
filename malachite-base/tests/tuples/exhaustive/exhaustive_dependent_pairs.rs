// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use itertools::{chain, Itertools};
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_dependent_pairs_stop_after_empty_ys,
    ExhaustiveDependentPairsYsGenerator,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{once, repeat, Cloned};
use std::slice::Iter;

#[derive(Clone, Debug)]
struct ExhaustiveGeneratorFromMap<X: Clone + Eq + Hash, Y: 'static + Clone> {
    map: HashMap<X, &'static [Y]>,
}

impl<X: Clone + Eq + Hash, Y: 'static + Clone>
    ExhaustiveDependentPairsYsGenerator<X, Y, Cloned<Iter<'static, Y>>>
    for ExhaustiveGeneratorFromMap<X, Y>
{
    #[inline]
    fn get_ys(&self, x: &X) -> Cloned<Iter<'static, Y>> {
        self.map[x].iter().cloned()
    }
}

fn exhaustive_dependent_pairs_finite_ys_helper<I: Clone + Iterator, Y>(
    xs: I,
    map: HashMap<I::Item, &'static [Y]>,
    out_ruler: &[(I::Item, Y)],
    out_normal_normal: &[(I::Item, Y)],
    out_tiny_normal: &[(I::Item, Y)],
) where
    I::Item: Clone + Debug + Eq + Hash,
    Y: Clone + Debug + Eq,
{
    let xss_ruler = exhaustive_dependent_pairs(
        ruler_sequence(),
        xs.clone(),
        ExhaustiveGeneratorFromMap { map: map.clone() },
    )
    .take(20)
    .collect_vec();
    assert_eq!(xss_ruler.as_slice(), out_ruler);
    let xss_normal_normal = exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        xs.clone(),
        ExhaustiveGeneratorFromMap { map: map.clone() },
    )
    .take(20)
    .collect_vec();
    assert_eq!(xss_normal_normal.as_slice(), out_normal_normal);
    let xss_tiny_normal = exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ),
        xs,
        ExhaustiveGeneratorFromMap { map },
    )
    .take(20)
    .collect_vec();
    assert_eq!(xss_tiny_normal.as_slice(), out_tiny_normal);
}

#[test]
fn test_exhaustive_dependent_pairs() {
    exhaustive_dependent_pairs_finite_ys_helper(
        [1, 2, 3].iter().copied(),
        hashmap! {
            1 => &[100, 101, 102][..],
            2 => &[200, 201, 202][..],
            3 => &[300, 301, 302][..]
        },
        &[(1, 100), (2, 200), (1, 101), (3, 300), (1, 102), (2, 201), (2, 202), (3, 301), (3, 302)],
        &[(1, 100), (2, 200), (1, 101), (2, 201), (3, 300), (1, 102), (3, 301), (3, 302), (2, 202)],
        &[(1, 100), (2, 200), (3, 300), (1, 101), (1, 102), (2, 201), (3, 301), (3, 302), (2, 202)],
    );
    exhaustive_dependent_pairs_finite_ys_helper(
        ["cat", "dog", "mouse", "dog", "cat"].iter().copied(),
        hashmap! { "cat" => &[2, 3, 4][..], "dog" => &[20][..], "mouse" => &[30, 40][..] },
        &[
            ("cat", 2),
            ("dog", 20),
            ("cat", 3),
            ("mouse", 30),
            ("cat", 4),
            ("mouse", 40),
            ("dog", 20),
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
        ],
        &[
            ("cat", 2),
            ("dog", 20),
            ("cat", 3),
            ("mouse", 30),
            ("dog", 20),
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
            ("mouse", 40),
            ("cat", 4),
        ],
        &[
            ("cat", 2),
            ("dog", 20),
            ("mouse", 30),
            ("dog", 20),
            ("cat", 3),
            ("mouse", 40),
            ("cat", 2),
            ("cat", 4),
            ("cat", 3),
            ("cat", 4),
        ],
    );
    exhaustive_dependent_pairs_finite_ys_helper(
        [1, 2, 3, 2, 3, 2, 2].iter().copied(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[(1, 100), (3, 300), (1, 101), (3, 300), (1, 102), (3, 301), (3, 302), (3, 301), (3, 302)],
        &[(1, 100), (3, 300), (1, 101), (3, 301), (3, 300), (1, 102), (3, 301), (3, 302), (3, 302)],
        &[(1, 100), (3, 300), (3, 300), (1, 101), (1, 102), (3, 301), (3, 301), (3, 302), (3, 302)],
    );
    exhaustive_dependent_pairs_finite_ys_helper(
        [].iter().copied(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
        &[],
        &[],
    );
    exhaustive_dependent_pairs_finite_ys_helper(
        [2, 2, 2, 2, 2].iter().copied(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
        &[],
        &[],
    );
}

fn exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper<I: Clone + Iterator, Y>(
    xs: I,
    map: HashMap<I::Item, &'static [Y]>,
    out_ruler: &[(I::Item, Y)],
    out_normal_normal: &[(I::Item, Y)],
    out_tiny_normal: &[(I::Item, Y)],
) where
    I::Item: Clone + Debug + Eq + Hash,
    Y: Clone + Debug + Eq,
{
    let xss_ruler = exhaustive_dependent_pairs_stop_after_empty_ys(
        ruler_sequence(),
        xs.clone(),
        ExhaustiveGeneratorFromMap { map: map.clone() },
    )
    .take(20)
    .collect_vec();
    assert_eq!(xss_ruler.as_slice(), out_ruler);
    let xss_normal_normal = exhaustive_dependent_pairs_stop_after_empty_ys(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        xs.clone(),
        ExhaustiveGeneratorFromMap { map: map.clone() },
    )
    .take(20)
    .collect_vec();
    assert_eq!(xss_normal_normal.as_slice(), out_normal_normal);
    let xss_tiny_normal = exhaustive_dependent_pairs_stop_after_empty_ys(
        bit_distributor_sequence(
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ),
        xs,
        ExhaustiveGeneratorFromMap { map },
    )
    .take(20)
    .collect_vec();
    assert_eq!(xss_tiny_normal.as_slice(), out_tiny_normal);
}

#[test]
fn test_exhaustive_dependent_pairs_stop_after_empty_ys() {
    exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper(
        [1, 2, 3].iter().copied(),
        hashmap! {
            1 => &[100, 101, 102][..],
            2 => &[200, 201, 202][..],
            3 => &[300, 301, 302][..]
        },
        &[(1, 100), (2, 200), (1, 101), (3, 300), (1, 102), (2, 201), (2, 202), (3, 301), (3, 302)],
        &[(1, 100), (2, 200), (1, 101), (2, 201), (3, 300), (1, 102), (3, 301), (3, 302), (2, 202)],
        &[(1, 100), (2, 200), (3, 300), (1, 101), (1, 102), (2, 201), (3, 301), (3, 302), (2, 202)],
    );
    exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper(
        ["cat", "dog", "mouse", "dog", "cat"].iter().copied(),
        hashmap! { "cat" => &[2, 3, 4][..], "dog" => &[20][..], "mouse" => &[30, 40][..] },
        &[
            ("cat", 2),
            ("dog", 20),
            ("cat", 3),
            ("mouse", 30),
            ("cat", 4),
            ("mouse", 40),
            ("dog", 20),
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
        ],
        &[
            ("cat", 2),
            ("dog", 20),
            ("cat", 3),
            ("mouse", 30),
            ("dog", 20),
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
            ("mouse", 40),
            ("cat", 4),
        ],
        &[
            ("cat", 2),
            ("dog", 20),
            ("mouse", 30),
            ("dog", 20),
            ("cat", 3),
            ("mouse", 40),
            ("cat", 2),
            ("cat", 4),
            ("cat", 3),
            ("cat", 4),
        ],
    );
    // Notice difference from `exhaustive_dependent_pairs`
    exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper(
        [1, 2, 3, 2, 3, 2, 2].iter().copied(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[(1, 100)],
        &[(1, 100)],
        &[(1, 100)],
    );
    exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper(
        [].iter().copied(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
        &[],
        &[],
    );
    exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper(
        [2, 2, 2, 2, 2].iter().copied(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
        &[],
        &[],
    );
    // With `exhaustive_dependent_pairs` this would hang
    exhaustive_dependent_pairs_finite_ys_stop_after_empty_ys_helper(
        chain(once(3), repeat(2)),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[(3, 300)],
        &[(3, 300)],
        &[(3, 300)],
    );
}

#[derive(Clone, Debug)]
struct MultiplesGeneratorHelper {
    u: u64,
    step: u64,
}

impl Iterator for MultiplesGeneratorHelper {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.u;
        self.u += self.step;
        Some(next)
    }
}

#[derive(Clone, Debug)]
struct MultiplesGenerator {}

impl ExhaustiveDependentPairsYsGenerator<u64, u64, MultiplesGeneratorHelper>
    for MultiplesGenerator
{
    #[inline]
    fn get_ys(&self, x: &u64) -> MultiplesGeneratorHelper {
        MultiplesGeneratorHelper { u: *x, step: *x }
    }
}

#[test]
fn test_exhaustive_dependent_pairs_infinite() {
    let xs = exhaustive_positive_primitive_ints::<u64>();
    let xss_ruler = exhaustive_dependent_pairs(ruler_sequence(), xs.clone(), MultiplesGenerator {})
        .take(50)
        .collect_vec();
    assert_eq!(
        xss_ruler.as_slice(),
        &[
            (1, 1),
            (2, 2),
            (1, 2),
            (3, 3),
            (1, 3),
            (2, 4),
            (1, 4),
            (4, 4),
            (1, 5),
            (2, 6),
            (1, 6),
            (3, 6),
            (1, 7),
            (2, 8),
            (1, 8),
            (5, 5),
            (1, 9),
            (2, 10),
            (1, 10),
            (3, 9),
            (1, 11),
            (2, 12),
            (1, 12),
            (4, 8),
            (1, 13),
            (2, 14),
            (1, 14),
            (3, 12),
            (1, 15),
            (2, 16),
            (1, 16),
            (6, 6),
            (1, 17),
            (2, 18),
            (1, 18),
            (3, 15),
            (1, 19),
            (2, 20),
            (1, 20),
            (4, 12),
            (1, 21),
            (2, 22),
            (1, 22),
            (3, 18),
            (1, 23),
            (2, 24),
            (1, 24),
            (5, 10),
            (1, 25),
            (2, 26)
        ]
    );
    let xss_normal_normal = exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        xs.clone(),
        MultiplesGenerator {},
    )
    .take(50)
    .collect_vec();
    assert_eq!(
        xss_normal_normal.as_slice(),
        &[
            (1, 1),
            (2, 2),
            (1, 2),
            (2, 4),
            (3, 3),
            (4, 4),
            (3, 6),
            (4, 8),
            (1, 3),
            (2, 6),
            (1, 4),
            (2, 8),
            (3, 9),
            (4, 12),
            (3, 12),
            (4, 16),
            (5, 5),
            (6, 6),
            (5, 10),
            (6, 12),
            (7, 7),
            (8, 8),
            (7, 14),
            (8, 16),
            (5, 15),
            (6, 18),
            (5, 20),
            (6, 24),
            (7, 21),
            (8, 24),
            (7, 28),
            (8, 32),
            (1, 5),
            (2, 10),
            (1, 6),
            (2, 12),
            (3, 15),
            (4, 20),
            (3, 18),
            (4, 24),
            (1, 7),
            (2, 14),
            (1, 8),
            (2, 16),
            (3, 21),
            (4, 28),
            (3, 24),
            (4, 32),
            (5, 25),
            (6, 30)
        ]
    );
    let xss_tiny_normal = exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ),
        xs,
        MultiplesGenerator {},
    )
    .take(50)
    .collect_vec();
    assert_eq!(
        xss_tiny_normal.as_slice(),
        &[
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (1, 2),
            (2, 4),
            (3, 6),
            (4, 8),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (5, 10),
            (6, 12),
            (7, 14),
            (8, 16),
            (1, 3),
            (2, 6),
            (3, 9),
            (4, 12),
            (1, 4),
            (2, 8),
            (3, 12),
            (4, 16),
            (5, 15),
            (6, 18),
            (7, 21),
            (8, 24),
            (5, 20),
            (6, 24),
            (7, 28),
            (8, 32),
            (1, 5),
            (2, 10),
            (3, 15),
            (4, 20),
            (1, 6),
            (2, 12),
            (3, 18),
            (4, 24),
            (5, 25),
            (6, 30),
            (7, 35),
            (8, 40),
            (5, 30),
            (6, 36),
            (7, 42),
            (8, 48),
            (1, 7),
            (2, 14)
        ]
    );
}
