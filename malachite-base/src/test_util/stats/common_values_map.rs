// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::ToDebugString;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;

macro_rules! impl_common_values_map {
    ($CommonOrdered: ident, $StringTrait: ident, $to_string: ident, $common_values_map: ident) => {
        #[derive(Eq, PartialEq)]
        struct $CommonOrdered<T: $StringTrait + Eq> {
            x: T,
            frequency: usize,
        }

        impl<T: $StringTrait + Eq> PartialOrd for $CommonOrdered<T> {
            fn partial_cmp(&self, other: &$CommonOrdered<T>) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<T: $StringTrait + Eq> Ord for $CommonOrdered<T> {
            fn cmp(&self, other: &$CommonOrdered<T>) -> Ordering {
                other.frequency.cmp(&self.frequency).then_with(|| {
                    let x = self.x.$to_string();
                    let y = other.x.$to_string();
                    x.len().cmp(&y.len()).then_with(|| x.cmp(&y))
                })
            }
        }

        /// Takes the first `limit` values of an iterator `xs`. Groups them into (unique value,
        /// frequency) pairs. Takes the top `n` pairs, preferring ones with high frequency. To break
        /// ties, prefers values with low `$to_string` representations, where the strings are
        /// shortlex-ordered (ordered by length, then lexicographically).
        pub fn $common_values_map<I: Iterator>(
            limit: usize,
            n: usize,
            xs: I,
        ) -> Vec<(I::Item, usize)>
        where
            I::Item: $StringTrait + Eq + Hash,
        {
            assert_ne!(n, 0);
            let mut frequencies: HashMap<I::Item, usize> = HashMap::new();
            for x in xs.take(limit) {
                *frequencies.entry(x).or_insert(0) += 1;
            }
            let mut top_n = BinaryHeap::new();
            let mut full = false;
            for (x, frequency) in frequencies {
                top_n.push($CommonOrdered { x, frequency });
                if full {
                    top_n.pop();
                } else if top_n.len() == n {
                    full = true;
                }
            }
            let mut common_values = Vec::new();
            while let Some(pair) = top_n.pop() {
                common_values.push((pair.x, pair.frequency));
            }
            common_values.reverse();
            common_values
        }
    };
}
impl_common_values_map!(CommonOrdered, Display, to_string, common_values_map);
impl_common_values_map!(
    CommonOrderedDebug,
    Debug,
    to_debug_string,
    common_values_map_debug
);
