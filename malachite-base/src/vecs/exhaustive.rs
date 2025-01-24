// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use crate::iterators::iterator_cache::IteratorCache;
use crate::num::arithmetic::traits::CheckedPow;
use crate::num::conversion::traits::{ExactFrom, SaturatingFrom, WrappingFrom};
use crate::num::exhaustive::{
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
    PrimitiveIntIncreasingRange,
};
use crate::num::iterators::{ruler_sequence, RulerSequence};
use crate::num::logic::traits::SignificantBits;
use crate::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_dependent_pairs_stop_after_empty_ys,
    lex_dependent_pairs_stop_after_empty_ys, ExhaustiveDependentPairs,
    ExhaustiveDependentPairsYsGenerator, LexDependentPairs,
};
use crate::vecs::{exhaustive_vec_permutations, ExhaustiveVecPermutations};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{
    max, min,
    Ordering::{self, *},
};
use core::iter::{empty, once, FromIterator, Once, Zip};
use core::marker::PhantomData;
use core::mem::take;
use core::ops::RangeFrom;
use itertools::{repeat_n, Itertools};

#[doc(hidden)]
pub fn validate_oi_map<I: Iterator<Item = usize>>(max_input_index: usize, xs: I) {
    let mut oi_unique = hashbrown::HashSet::new();
    oi_unique.extend(xs);
    let oi_sorted_unique = oi_unique.into_iter().sorted().collect_vec();
    assert_eq!(oi_sorted_unique.len(), max_input_index + 1);
    assert_eq!(*oi_sorted_unique.first().unwrap(), 0);
    assert_eq!(*oi_sorted_unique.last().unwrap(), max_input_index);
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LexFixedLengthVecsOutput {
    pub input_index: usize,
    pub counter: usize,
}

/// Defines lexicographic fixed-length [`Vec`] generators.
///
/// Malachite provides [`lex_vecs_length_2`] and [`lex_vecs_fixed_length_2_inputs`], but you can
/// also define `lex_vecs_length_3`, `lex_vecs_length_4`, and so on, and
/// `lex_vecs_fixed_length_3_inputs`, `lex_vecs_fixed_length_4_inputs`, and so on, in your program
/// using the code below. The documentation for [`lex_vecs_length_2`] and
/// [`lex_vecs_fixed_length_2_inputs`] describes these other functions as well.
///
/// See usage examples [here](self#lex_vecs_length_2) and
/// [here](self#lex_vecs_fixed_length_2_inputs).
///
/// ```
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_vecs_fixed_length;
/// use malachite_base::vecs::exhaustive::{validate_oi_map, LexFixedLengthVecsOutput};
///
/// lex_vecs_fixed_length!(
///     (pub(crate)),
///     LexFixedLengthVecs3Inputs,
///     lex_vecs_fixed_length_3_inputs,
///     lex_vecs_length_3,
///     [0, I, xs, xs_outputs],
///     [1, J, ys, ys_outputs],
///     [2, K, zs, zs_outputs]
/// );
/// lex_vecs_fixed_length!(
///     (pub(crate)),
///     LexFixedLengthVecs4Inputs,
///     lex_vecs_fixed_length_4_inputs,
///     lex_vecs_length_4,
///     [0, I, xs, xs_outputs],
///     [1, J, ys, ys_outputs],
///     [2, K, zs, zs_outputs],
///     [3, L, ws, ws_outputs]
/// );
/// lex_vecs_fixed_length!(
///     (pub(crate)),
///     LexFixedLengthVecs5Inputs,
///     lex_vecs_fixed_length_5_inputs,
///     lex_vecs_length_5,
///     [0, I, xs, xs_outputs],
///     [1, J, ys, ys_outputs],
///     [2, K, zs, zs_outputs],
///     [3, L, ws, ws_outputs],
///     [4, M, vs, vs_outputs]
/// );
/// lex_vecs_fixed_length!(
///     (pub(crate)),
///     LexFixedLengthVecs6Inputs,
///     lex_vecs_fixed_length_6_inputs,
///     lex_vecs_length_6,
///     [0, I, xs, xs_outputs],
///     [1, J, ys, ys_outputs],
///     [2, K, zs, zs_outputs],
///     [3, L, ws, ws_outputs],
///     [4, M, vs, vs_outputs],
///     [5, N, us, us_outputs]
/// );
/// lex_vecs_fixed_length!(
///     (pub(crate)),
///     LexFixedLengthVecs7Inputs,
///     lex_vecs_fixed_length_7_inputs,
///     lex_vecs_length_7,
///     [0, I, xs, xs_outputs],
///     [1, J, ys, ys_outputs],
///     [2, K, zs, zs_outputs],
///     [3, L, ws, ws_outputs],
///     [4, M, vs, vs_outputs],
///     [5, N, us, us_outputs],
///     [6, O, ts, ts_outputs]
/// );
/// lex_vecs_fixed_length!(
///     (pub(crate)),
///     LexFixedLengthVecs8Inputs,
///     lex_vecs_fixed_length_8_inputs,
///     lex_vecs_length_8,
///     [0, I, xs, xs_outputs],
///     [1, J, ys, ys_outputs],
///     [2, K, zs, zs_outputs],
///     [3, L, ws, ws_outputs],
///     [4, M, vs, vs_outputs],
///     [5, N, us, us_outputs],
///     [6, O, ts, ts_outputs],
///     [7, P, ss, ss_outputs]
/// );
/// ```
#[macro_export]
macro_rules! lex_vecs_fixed_length {
    (
        ($($vis:tt)*),
        $exhaustive_struct: ident,
        $exhaustive_custom_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_outputs: ident]),*
    ) => {
        /// This documentation applies not only to `LexFixedLengthVecs2Inputs`, but also to
        /// `LexFixedLengthVecs3Inputs`, `LexFixedLengthVecs4Inputs`, and so on. See
        /// [`lex_vecs_fixed_length`] for more information.
        ///
        /// Generates all [`Vec`]s of a given length with elements from $m$ iterators, in
        /// lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// The fixed length $n$ of the [`Vec`]s is greater than or equal to $m$.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<T: Clone, $($it: Iterator<Item = T>,)*> {
            first: bool,
            done: bool,
            $(
                $xs: IteratorCache<$it>,
                $xs_outputs: Vec<usize>,
            )*
            outputs: Vec<LexFixedLengthVecsOutput>,
        }

        impl<T: Clone, $($it: Iterator<Item = T>,)*> $exhaustive_struct<T, $($it,)*> {
            fn increment_counters(&mut self) -> bool {
                for (i, output) in self.outputs.iter_mut().enumerate().rev() {
                    output.counter += 1;
                    let no_carry = match output.input_index {
                        $(
                            $i => self.$xs.get(output.counter).is_some(),
                        )*
                        _ => unreachable!(),
                    };
                    if no_carry {
                        return false;
                    } else if i == 0 {
                        return true;
                    }
                    output.counter = 0;
                }
                false
            }
        }

        impl<T: Clone, $($it: Iterator<Item = T>,)*> Iterator for $exhaustive_struct<T, $($it,)*>
        {
            type Item = Vec<T>;

            fn next(&mut self) -> Option<Vec<T>> {
                if self.done {
                    None
                } else if self.first {
                    self.first = false;
                    let mut next = vec![None; self.outputs.len()];
                    $(
                        if let Some(x) = self.$xs.get(0) {
                            for &output_index in &self.$xs_outputs {
                                next[output_index] = Some(x.clone());
                            }
                        } else {
                            self.done = true;
                            return None;
                        }
                    )*
                    Some(next.into_iter().map(Option::unwrap).collect())
                } else {
                    if self.increment_counters() {
                        self.done = true;
                        return None;
                    }
                    let mut next = Vec::with_capacity(self.outputs.len());
                    for &output in &self.outputs {
                        let x = match output.input_index {
                            $(
                                $i => self.$xs.get(output.counter),
                            )*
                            _ => unreachable!(),
                        };
                        next.push(x.unwrap().clone());
                    }
                    Some(next)
                }
            }
        }

        /// This documentation applies not only to `lex_vecs_fixed_length_2_inputs`, but also to
        /// `lex_vecs_fixed_length_3_inputs`, `lex_vecs_fixed_length_4_inputs`, and so on. See
        /// [`lex_vecs_fixed_length`] for more information.
        ///
        /// Generates all length-$n$ [`Vec`]s with elements from $m$ iterators, where $m \leq n$, in
        /// lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// The `output_to_input_map` parameter defines which iterators are mapped to which slot in
        /// the output [`Vec`]s. The length of the output [`Vec`]s, $n$, is specified by the length
        /// of `output_to_input_map`.
        ///
        /// The $i$th element of `output_to_input_map` is an index from 0 to $m-1$ which specifies
        /// which iterator the $i$th output slot is populated with. Together, the elements must
        /// include all indices from 0 to $m-1$, inclusive, possibly with repetitions.
        ///
        /// Let `xs` be the input iterator mapped to the first slot of the output [`Vec`]s. All the
        /// input iterators, except possibly `xs`, must be finite. If `xs` is finite, the output
        /// length is the product of the lengths of all the input iterators. If `xs` is infinite,
        /// the output is also infinite.
        ///
        /// If any of the input iterators is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#lex_vecs_fixed_length_2_inputs).
        #[allow(dead_code)]
        $($vis)* fn $exhaustive_custom_fn<T: Clone, $($it: Iterator<Item = T>,)*>(
            $($xs: $it,)*
            output_to_input_map: &[usize],
        ) -> $exhaustive_struct<T, $($it,)*> {
            $(
                let _max_input_index = $i;
            )*
            validate_oi_map(_max_input_index, output_to_input_map.iter().cloned());
            $(
                let $xs_outputs = output_to_input_map
                    .iter()
                    .enumerate()
                    .filter_map(|(o, i)| if *i == $i { Some(o) } else { None })
                    .collect();
            )*
            $exhaustive_struct {
                first: true,
                done: false,
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_outputs,
                )*
                outputs: output_to_input_map
                    .iter()
                    .map(|&i| LexFixedLengthVecsOutput {
                        input_index: i,
                        counter: 0,
                    })
                    .collect(),
            }
        }

        /// This documentation applies not only to `lex_vecs_length_2`, but also to
        /// `lex_vecs_length_3`, `lex_vecs_length_4`, and so on. See [`lex_vecs_fixed_length`] for
        /// more information.
        ///
        /// Generates all length-$n$ [`Vec`]s with elements from $n$ iterators, in lexicographic
        /// order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// All of `ys`, `zs`, ... (but not necessarily `xs`) must be finite. If `xs` is finite, the
        /// output length is the product of the lengths of all the input iterators. If `xs` is
        /// infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#lex_vecs_length_2).
        #[allow(dead_code)]
        #[inline]
        $($vis)* fn $exhaustive_1_to_1_fn<T: Clone, $($it: Iterator<Item = T>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<T, $($it,)*> {
            $exhaustive_custom_fn($($xs,)* &[$($i,)*])
        }
    }
}

lex_vecs_fixed_length!(
    (pub),
    LexFixedLengthVecs2Inputs,
    lex_vecs_fixed_length_2_inputs,
    lex_vecs_length_2,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs]
);

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct LexFixedLengthVecsFromSingleG<I: Iterator>
where
    I::Item: Clone,
{
    first: bool,
    done: bool,
    xs: IteratorCache<I>,
    counters: Vec<usize>,
    phantom: PhantomData<*const I::Item>,
}

impl<I: Iterator> LexFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    fn increment_counters(&mut self) -> bool {
        for (i, counter) in self.counters.iter_mut().enumerate().rev() {
            *counter += 1;
            if self.xs.get(*counter).is_some() {
                return false;
            } else if i == 0 {
                return true;
            }
            *counter = 0;
        }
        false
    }
}

impl<I: Iterator> Iterator for LexFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if self.done {
            None
        } else if self.first {
            self.first = false;
            if let Some(x) = self.xs.get(0) {
                Some(repeat_n(x, self.counters.len()).cloned().collect())
            } else {
                self.done = true;
                None
            }
        } else if self.increment_counters() {
            self.done = true;
            None
        } else {
            let xs = &mut self.xs;
            Some(
                self.counters
                    .iter()
                    .map(|&c| xs.get(c).unwrap().clone())
                    .collect(),
            )
        }
    }
}

fn lex_vecs_fixed_length_from_single_g<I: Iterator>(
    len: u64,
    xs: I,
) -> LexFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    LexFixedLengthVecsFromSingleG {
        first: true,
        done: false,
        xs: IteratorCache::new(xs),
        counters: vec![0; usize::exact_from(len)],
        phantom: PhantomData,
    }
}

/// Generates all [`Vec`]s of a given length with elements from a single iterator, in lexicographic
/// order.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// This `struct` is created by the [`lex_vecs_fixed_length_from_single`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub enum LexFixedLengthVecsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    Zero(Once<Vec<I::Item>>),
    One(I),
    GreaterThanOne(LexFixedLengthVecsFromSingleG<I>),
}

impl<I: Iterator> Iterator for LexFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            LexFixedLengthVecsFromSingle::Zero(ref mut xs) => xs.next(),
            LexFixedLengthVecsFromSingle::One(ref mut xs) => xs.next().map(|x| vec![x]),
            LexFixedLengthVecsFromSingle::GreaterThanOne(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all [`Vec`]s of a given length with elements from a single iterator, in lexicographic
/// order.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// `xs` must be finite.
///
/// The output length is $k^n$, where $k$ is `xs.count()` and $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_vecs_fixed_length_from_single;
///
/// let xss = lex_vecs_fixed_length_from_single(2, 0..4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[0, 0],
///         &[0, 1],
///         &[0, 2],
///         &[0, 3],
///         &[1, 0],
///         &[1, 1],
///         &[1, 2],
///         &[1, 3],
///         &[2, 0],
///         &[2, 1],
///         &[2, 2],
///         &[2, 3],
///         &[3, 0],
///         &[3, 1],
///         &[3, 2],
///         &[3, 3]
///     ]
/// );
/// ```
pub fn lex_vecs_fixed_length_from_single<I: Iterator>(
    len: u64,
    xs: I,
) -> LexFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    match len {
        0 => LexFixedLengthVecsFromSingle::Zero(once(Vec::new())),
        1 => LexFixedLengthVecsFromSingle::One(xs),
        len => LexFixedLengthVecsFromSingle::GreaterThanOne(lex_vecs_fixed_length_from_single_g(
            len, xs,
        )),
    }
}

/// Defines exhaustive fixed-length [`Vec`] generators.
///
/// Malachite provides [`exhaustive_vecs_length_2`] and [`exhaustive_vecs_fixed_length_2_inputs`],
/// but you can also define `exhaustive_vecs_length_3`, `exhaustive_vecs_length_4`, and so on, and
/// `exhaustive_vecs_fixed_length_3_inputs`, `exhaustive_vecs_fixed_length_4_inputs`, and so on, in
/// your program using the code below. The documentation for [`exhaustive_vecs_length_2`] and
/// [`exhaustive_vecs_fixed_length_2_inputs`] describes these other functions as well.
///
/// See usage examples [here](self#exhaustive_vecs_length_2) and
/// [here](self#exhaustive_vecs_fixed_length_2_inputs).
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::exhaustive_vecs_fixed_length;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use malachite_base::vecs::exhaustive::validate_oi_map;
/// use std::cmp::max;
///
/// exhaustive_vecs_fixed_length!(
///     (pub(crate)),
///     ExhaustiveFixedLengthVecs3Inputs,
///     exhaustive_vecs_fixed_length_3_inputs,
///     exhaustive_vecs_length_3,
///     [0, I, xs, xs_done, xs_outputs],
///     [1, J, ys, ys_done, ys_outputs],
///     [2, K, zs, zs_done, zs_outputs]
/// );
/// exhaustive_vecs_fixed_length!(
///     (pub(crate)),
///     ExhaustiveFixedLengthVecs4Inputs,
///     exhaustive_vecs_fixed_length_4_inputs,
///     exhaustive_vecs_length_4,
///     [0, I, xs, xs_done, xs_outputs],
///     [1, J, ys, ys_done, ys_outputs],
///     [2, K, zs, zs_done, zs_outputs],
///     [3, L, ws, ws_done, ws_outputs]
/// );
/// exhaustive_vecs_fixed_length!(
///     (pub(crate)),
///     ExhaustiveFixedLengthVecs5Inputs,
///     exhaustive_vecs_fixed_length_5_inputs,
///     exhaustive_vecs_length_5,
///     [0, I, xs, xs_done, xs_outputs],
///     [1, J, ys, ys_done, ys_outputs],
///     [2, K, zs, zs_done, zs_outputs],
///     [3, L, ws, ws_done, ws_outputs],
///     [4, M, vs, vs_done, vs_outputs]
/// );
/// exhaustive_vecs_fixed_length!(
///     (pub(crate)),
///     ExhaustiveFixedLengthVecs6Inputs,
///     exhaustive_vecs_fixed_length_6_inputs,
///     exhaustive_vecs_length_6,
///     [0, I, xs, xs_done, xs_outputs],
///     [1, J, ys, ys_done, ys_outputs],
///     [2, K, zs, zs_done, zs_outputs],
///     [3, L, ws, ws_done, ws_outputs],
///     [4, M, vs, vs_done, vs_outputs],
///     [5, N, us, us_done, us_outputs]
/// );
/// exhaustive_vecs_fixed_length!(
///     (pub(crate)),
///     ExhaustiveFixedLengthVecs7,
///     exhaustive_vecs_fixed_length_7_inputs,
///     exhaustive_vecs_length_7,
///     [0, I, xs, xs_done, xs_outputs],
///     [1, J, ys, ys_done, ys_outputs],
///     [2, K, zs, zs_done, zs_outputs],
///     [3, L, ws, ws_done, ws_outputs],
///     [4, M, vs, vs_done, vs_outputs],
///     [5, N, us, us_done, us_outputs],
///     [6, O, ts, ts_done, ts_outputs]
/// );
/// exhaustive_vecs_fixed_length!(
///     (pub(crate)),
///     ExhaustiveFixedLengthVecs8Inputs,
///     exhaustive_vecs_fixed_length_8_inputs,
///     exhaustive_vecs_length_8,
///     [0, I, xs, xs_done, xs_outputs],
///     [1, J, ys, ys_done, ys_outputs],
///     [2, K, zs, zs_done, zs_outputs],
///     [3, L, ws, ws_done, ws_outputs],
///     [4, M, vs, vs_done, vs_outputs],
///     [5, N, us, us_done, us_outputs],
///     [6, O, ts, ts_done, ts_outputs],
///     [7, P, ss, ss_done, ss_outputs]
/// );
/// ```
#[macro_export]
macro_rules! exhaustive_vecs_fixed_length {
    (
        ($($vis:tt)*),
        $exhaustive_struct: ident,
        $exhaustive_custom_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_done: ident, $outputs: ident]),*
    ) => {
        /// This documentation applies not only to `ExhaustiveFixedLengthVecs2Inputs`, but also to
        /// `ExhaustiveFixedLengthVecs3Inputs`, `ExhaustiveFixedLengthVecs4Inputs`, and so on. See
        /// [`exhaustive_vecs_fixed_length`] for more information.
        ///
        /// Generates all [`Vec`]s of a given length with elements from $m$ iterators.
        ///
        /// The fixed length $n$ of the [`Vec`]s is greater than or equal to $m$.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<T: Clone, $($it: Iterator<Item=T>,)*> {
            i: u64,
            len: u64,
            limit: Option<u64>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
                $outputs: Vec<usize>,
            )*
        }

        impl<T: Clone, $($it: Iterator<Item=T>,)*> $exhaustive_struct<T, $($it,)*> {
            fn try_getting_limit(&mut self) {
                let mut all_limits_known = true;
                $(
                    if let Some(xs_len) = self.$xs.known_len() {
                        if xs_len == 0 {
                            self.limit = Some(0);
                            return;
                        }
                    } else {
                        all_limits_known = false;
                    }
                )*
                if !all_limits_known {
                    return;
                }
                let mut product = 1u64;
                $(
                    let xs_len = u64::exact_from(self.$xs.known_len().unwrap());
                    for _ in 0..self.$outputs.len() {
                        if let Some(new_product) = product.checked_mul(xs_len) {
                            product = new_product;
                        } else {
                            return;
                        }
                    }
                )*
                self.limit = Some(product);
            }
        }

        impl<T: Clone, $($it: Iterator<Item=T>,)*> Iterator for $exhaustive_struct<T, $($it,)*> {
            type Item = Vec<T>;

            fn next(&mut self) -> Option<Vec<T>> {
                if Some(self.i) == self.limit {
                    None
                } else {
                    if self.i == u64::MAX {
                        panic!("Too many iterations");
                    }
                    loop {
                        let mut all_are_valid = true;
                        $(
                            for &output_index in &self.$outputs {
                                if self.$xs.get(
                                    self.distributor.get_output(output_index)
                                ).is_none() {
                                    all_are_valid = false;
                                    break;
                                }
                            }
                            if !all_are_valid {
                                if self.i == 0 {
                                    self.limit = Some(0);
                                    return None;
                                } else if !self.$xs_done {
                                    self.try_getting_limit();
                                    if Some(self.i) == self.limit {
                                        return None;
                                    }
                                    self.$xs_done = true;
                                    let xs_len = self.$xs.known_len().unwrap();
                                    // xs_len > 0 at this point
                                    self.distributor.set_max_bits(
                                        &self.$outputs,
                                        max(
                                            1,
                                            usize::wrapping_from((xs_len - 1).significant_bits())
                                        )
                                    );
                                } else {
                                    self.distributor.increment_counter();
                                }
                                continue;
                            }
                        )*
                        break;
                    }
                    let mut out = vec![None; usize::exact_from(self.len)];
                    $(
                        for &output_index in &self.$outputs {
                            let x = self.$xs.get(self.distributor.get_output(output_index));
                            out[output_index] = Some(x.unwrap().clone());
                        }
                    )*
                    self.i += 1;
                    self.distributor.increment_counter();
                    Some(out.into_iter().map(Option::unwrap).collect())
                }
            }
        }

        /// This documentation applies not only to `exhaustive_vecs_fixed_length_2_inputs`, but also
        /// to `exhaustive_vecs_fixed_length_3_inputs`, `exhaustive_vecs_fixed_length_4_inputs`, and
        /// so on. See [`exhaustive_vecs_fixed_length`] for more information.
        ///
        /// Generates all [`Vec`]s of a given length with elements from $m$ iterators, where $m \leq
        /// n$.
        ///
        /// The `output_types` parameter defines which iterators are mapped to which slot in the
        /// output [`Vec`]s, and how quickly each output slot advances through its iterator. The
        /// length of the output [`Vec`]s, $n$, is specified by the length of `output_types`.
        ///
        /// The $i$th element of `output_types` is a pair of [`BitDistributorOutputType`] and
        /// `usize`. The [`BitDistributorOutputType`] determines how quickly the $i$th output slot
        /// advances through its iterator; see the [`BitDistributor`] documentation for a
        /// description of the different types. The `usize` is an index from 0 to $m-1$ which
        /// specifies which iterator the $i$th output slot is populated with. Together, the `usize`s
        /// must include all indices from 0 to $m-1$, inclusive, possibly with repetitions.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Panics
        /// Panics if the `usize`s in `output_types` do not include all indices from 0 to $m-1$,
        /// inclusive, possibly with repetitions. In particular, the length of `output_types` must
        /// be at least $m$.
        ///
        /// # Examples
        /// See [here](self#exhaustive_vecs_fixed_length_2_inputs).
        #[allow(dead_code)]
        $($vis)* fn $exhaustive_custom_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
            output_types: &[(BitDistributorOutputType, usize)],
        ) -> $exhaustive_struct<T, $($it,)*> {
            $(
                let _max_input_index = $i;
            )*
            let output_to_input_map = output_types.iter().map(|(_, i)| *i).collect_vec();
            validate_oi_map(_max_input_index, output_to_input_map.iter().cloned());
            $exhaustive_struct {
                i: 0,
                len: u64::exact_from(output_types.len()),
                limit: None,
                distributor: BitDistributor::new(output_types.iter().map(|(ot, _)| *ot)
                    .collect_vec().as_slice()),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                    $outputs: output_types.iter().enumerate()
                        .filter_map(|(o, (_, i))| if *i == $i { Some(o) } else { None }).collect(),
                )*
            }
        }

        /// This documentation applies not only to `exhaustive_vecs_length_2`, but also to
        /// `exhaustive_vecs_length_3`, `exhaustive_vecs_length_4`, and so on. See
        /// [`exhaustive_vecs_fixed_length`] for more information.
        ///
        /// Generates all length-$n$ [`Vec`]s with elements from $n$ iterators.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_vecs_length_2).
        #[allow(dead_code)]
        #[inline]
        $($vis)* fn $exhaustive_1_to_1_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
        ) -> $exhaustive_struct<T, $($it,)*> {
            $exhaustive_custom_fn(
                $($xs,)*
                &[$((BitDistributorOutputType::normal(1), $i),)*]
            )
        }
    }
}

exhaustive_vecs_fixed_length!(
    (pub),
    ExhaustiveFixedLengthVecs2Inputs,
    exhaustive_vecs_fixed_length_2_inputs,
    exhaustive_vecs_length_2,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs]
);

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExhaustiveFixedLengthVecs1InputG<I: Iterator>
where
    I::Item: Clone,
{
    i: u64,
    len: u64,
    limit: Option<u64>,
    distributor: BitDistributor,
    xs: IteratorCache<I>,
    xs_done: bool,
    phantom: PhantomData<*const I::Item>,
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecs1InputG<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if Some(self.i) == self.limit {
            None
        } else {
            assert!(self.i != u64::MAX, "Too many iterations");
            loop {
                let mut all_are_valid = true;
                for i in 0..usize::exact_from(self.len) {
                    if self.xs.get(self.distributor.get_output(i)).is_none() {
                        all_are_valid = false;
                        break;
                    }
                }
                if all_are_valid {
                    break;
                } else if !self.xs_done {
                    let xs_len = self.xs.known_len().unwrap();
                    self.limit = CheckedPow::checked_pow(u64::exact_from(xs_len), self.len);
                    if Some(self.i) == self.limit {
                        return None;
                    }
                    self.xs_done = true;
                    // xs_len > 0 at this point
                    self.distributor.set_max_bits(
                        &[0],
                        max(1, usize::wrapping_from((xs_len - 1).significant_bits())),
                    );
                } else {
                    self.distributor.increment_counter();
                }
            }
            let out = (0..usize::exact_from(self.len))
                .map(|i| self.xs.get(self.distributor.get_output(i)).unwrap().clone())
                .collect();
            self.i += 1;
            self.distributor.increment_counter();
            Some(out)
        }
    }
}

fn exhaustive_vecs_fixed_length_1_input_g<I: Iterator>(
    xs: I,
    output_types: &[BitDistributorOutputType],
) -> ExhaustiveFixedLengthVecs1InputG<I>
where
    I::Item: Clone,
{
    ExhaustiveFixedLengthVecs1InputG {
        i: 0,
        len: u64::exact_from(output_types.len()),
        limit: None,
        distributor: BitDistributor::new(output_types),
        xs: IteratorCache::new(xs),
        xs_done: false,
        phantom: PhantomData,
    }
}

/// Generates all [`Vec`]s of a given length with elements from a single iterator.
///
/// This `struct` is created by [`exhaustive_vecs_fixed_length_from_single`]; see its documentation
/// for more.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum ExhaustiveFixedLengthVecs1Input<I: Iterator>
where
    I::Item: Clone,
{
    Zero(Once<Vec<I::Item>>),
    One(I),
    GreaterThanOne(ExhaustiveFixedLengthVecs1InputG<I>),
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecs1Input<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            ExhaustiveFixedLengthVecs1Input::Zero(ref mut xs) => xs.next(),
            ExhaustiveFixedLengthVecs1Input::One(ref mut xs) => xs.next().map(|x| vec![x]),
            ExhaustiveFixedLengthVecs1Input::GreaterThanOne(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all length-$n$ [`Vec`]s with elements from a single iterator.
///
/// This function differs from [`exhaustive_vecs_fixed_length_from_single`] in that different
/// [`BitDistributorOutputType`]s may be specified for each output element.
///
/// The $i$th element of `output_types` is a [`BitDistributorOutputType`] that determines how
/// quickly the $i$th output slot advances through the iterator; see the [`BitDistributor`]
/// documentation for a description of the different types. The length of the output [`Vec`]s, $n$,
/// is specified by the length of `output_types`.
///
/// If `xs` is finite, the output length is $k^n$, where $k$ is `xs.count()` and $n$ is `len`. If
/// `xs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_fixed_length_1_input;
///
/// // We are generating length-3 [`Vec`]s of chars using one input iterator, which produces all
/// // ASCII chars. The third element has a tiny output type, so it will grow more slowly than the
/// // other two elements (though it doesn't look that way from the first few [`Vec`]s).
/// let xss = exhaustive_vecs_fixed_length_1_input(
///     exhaustive_ascii_chars(),
///     &[
///         BitDistributorOutputType::normal(1),
///         BitDistributorOutputType::normal(1),
///         BitDistributorOutputType::tiny(),
///     ],
/// );
/// let xss_prefix = xss.take(20).collect_vec();
/// assert_eq!(
///     xss_prefix
///         .iter()
///         .map(Vec::as_slice)
///         .collect_vec()
///         .as_slice(),
///     &[
///         &['a', 'a', 'a'],
///         &['a', 'a', 'b'],
///         &['a', 'a', 'c'],
///         &['a', 'a', 'd'],
///         &['a', 'b', 'a'],
///         &['a', 'b', 'b'],
///         &['a', 'b', 'c'],
///         &['a', 'b', 'd'],
///         &['a', 'a', 'e'],
///         &['a', 'a', 'f'],
///         &['a', 'a', 'g'],
///         &['a', 'a', 'h'],
///         &['a', 'b', 'e'],
///         &['a', 'b', 'f'],
///         &['a', 'b', 'g'],
///         &['a', 'b', 'h'],
///         &['b', 'a', 'a'],
///         &['b', 'a', 'b'],
///         &['b', 'a', 'c'],
///         &['b', 'a', 'd']
///     ]
/// );
/// ```
pub fn exhaustive_vecs_fixed_length_1_input<I: Iterator>(
    xs: I,
    output_types: &[BitDistributorOutputType],
) -> ExhaustiveFixedLengthVecs1Input<I>
where
    I::Item: Clone,
{
    match output_types.len() {
        0 => ExhaustiveFixedLengthVecs1Input::Zero(once(Vec::new())),
        1 => ExhaustiveFixedLengthVecs1Input::One(xs),
        _ => ExhaustiveFixedLengthVecs1Input::GreaterThanOne(
            exhaustive_vecs_fixed_length_1_input_g(xs, output_types),
        ),
    }
}

/// Generates all [`Vec`]s of a given length with elements from a single iterator.
///
/// If `xs` is finite, the output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$ is `len`.
/// If `xs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_fixed_length_from_single;
///
/// let xss = exhaustive_vecs_fixed_length_from_single(2, 0..4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[0, 0],
///         &[0, 1],
///         &[1, 0],
///         &[1, 1],
///         &[0, 2],
///         &[0, 3],
///         &[1, 2],
///         &[1, 3],
///         &[2, 0],
///         &[2, 1],
///         &[3, 0],
///         &[3, 1],
///         &[2, 2],
///         &[2, 3],
///         &[3, 2],
///         &[3, 3]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs_fixed_length_from_single<I: Iterator>(
    len: u64,
    xs: I,
) -> ExhaustiveFixedLengthVecs1Input<I>
where
    I::Item: Clone,
{
    exhaustive_vecs_fixed_length_1_input(
        xs,
        &vec![BitDistributorOutputType::normal(1); usize::exact_from(len)],
    )
}

#[derive(Clone, Debug)]
struct LexVecsGenerator<Y: Clone, J: Clone + Iterator<Item = Y>> {
    ys: J,
}

impl<Y: Clone, J: Clone + Iterator<Item = Y>>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<Y>, LexFixedLengthVecsFromSingle<J>>
    for LexVecsGenerator<Y, J>
{
    #[inline]
    fn get_ys(&self, &x: &u64) -> LexFixedLengthVecsFromSingle<J> {
        lex_vecs_fixed_length_from_single(x, self.ys.clone())
    }
}

#[inline]
const fn shortlex_vecs_from_element_iterator_helper<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
) -> LexDependentPairs<u64, Vec<T>, LexVecsGenerator<T, J>, I, LexFixedLengthVecsFromSingle<J>> {
    lex_dependent_pairs_stop_after_empty_ys(xs, LexVecsGenerator { ys })
}

/// Generates all [`Vec`]s with elements from a specified iterator and with lengths from another
/// iterator.
#[derive(Clone, Debug)]
pub struct ShortlexVecs<T: Clone, I: Iterator<Item = u64>, J: Clone + Iterator<Item = T>>(
    LexDependentPairs<u64, Vec<T>, LexVecsGenerator<T, J>, I, LexFixedLengthVecsFromSingle<J>>,
);

impl<T: Clone, I: Iterator<Item = u64>, J: Clone + Iterator<Item = T>> Iterator
    for ShortlexVecs<T, I, J>
{
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Vec<T>> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all [`Vec`]s with elements from a specified iterator and with lengths from another
/// iterator.
///
/// The length-generating iterator is `xs`, and the element-generating iterator is `ys`.
///
/// If the provided lengths are $\ell_0, \ell_1, \ell_2, \ldots$, then first all [`Vec`]s with
/// length $\ell_0$ will be generated, in lexicographic order; then all [`Vec`]s with length
/// $\ell_2$, and so on. If the lengths iterator has repetitions, then the generated [`Vec`]s will
/// be repeated too.
///
/// `ys` must be finite; if it's infinite, the output will never get past the first nonzero $\ell$.
///
/// There's one quirk if `ys` is empty: then the iterator will stop as soon as it encounters a
/// nonzero $\ell$, even if there are zeros later on. This prevents the iterator hanging when given
/// an empty `ys` and lengths $0, 1, 2, \ldots$.
///
/// If `ys` is empty, the output length is the amount of zeros generated by `xs` before the first
/// nonzero length. If `ys` is nonempty and `xs` is infinite, the output is infinite. Finally, if
/// `ys` is nonempty and `xs` is finite, the output length is
/// $$
/// \sum_{k=0}^{m-1} n^{\ell_k},
/// $$
/// where $n$ is `ys.count()` and $m$ is `xs.count()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::nevers::nevers;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_from_length_iterator;
///
/// let xss = shortlex_vecs_from_length_iterator([2, 1, 2].iter().cloned(), exhaustive_bools())
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[false, false][..],
///         &[false, true],
///         &[true, false],
///         &[true, true],
///         &[false],
///         &[true],
///         &[false, false],
///         &[false, true],
///         &[true, false],
///         &[true, true]
///     ]
/// );
///
/// let xss =
///     shortlex_vecs_from_length_iterator([0, 0, 1, 0].iter().cloned(), nevers()).collect_vec();
/// // Stops after first empty ys
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[&[], &[]]
/// );
/// ```
#[inline]
pub const fn shortlex_vecs_from_length_iterator<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
) -> ShortlexVecs<T, I, J> {
    ShortlexVecs(shortlex_vecs_from_element_iterator_helper(xs, ys))
}

/// Generates [`Vec`]s with elements from a specified iterator, in shortlex order.
///
/// Shortlex order means that the [`Vec`]s are output from shortest to longest, and [`Vec`]s of the
/// same length are output in lexicographic order with respect to the ordering of the [`Vec`]
/// elements specified by the input iterator.
///
/// `xs` must be finite; if it's infinite, only [`Vec`]s of length 0 and 1 are ever produced.
///
/// If `xs` is empty, the output length is 1; otherwise, the output is infinite.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs;
///
/// let bss = shortlex_vecs(exhaustive_bools()).take(20).collect_vec();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[false],
///         &[true],
///         &[false, false],
///         &[false, true],
///         &[true, false],
///         &[true, true],
///         &[false, false, false],
///         &[false, false, true],
///         &[false, true, false],
///         &[false, true, true],
///         &[true, false, false],
///         &[true, false, true],
///         &[true, true, false],
///         &[true, true, true],
///         &[false, false, false, false],
///         &[false, false, false, true],
///         &[false, false, true, false],
///         &[false, false, true, true],
///         &[false, true, false, false]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_vecs<I: Clone + Iterator>(
    xs: I,
) -> ShortlexVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    shortlex_vecs_from_length_iterator(exhaustive_unsigneds(), xs)
}

/// Generates all [`Vec`]s with a minimum length and with elements from a specified iterator, in
/// shortlex order.
///
/// Shortlex order means that the [`Vec`]s are output from shortest to longest, and [`Vec`]s of the
/// same length are output in lexicographic order with respect to the ordering of the [`Vec`]
/// elements specified by the input iterator.
///
/// `xs` must be finite; if it's infinite, only [`Vec`]s of length `min_length` (or 0 and 1, if
/// `min_length` is 0) are ever produced.
///
/// If `xs` is empty and `min_length` is 0, the output length is 1; if `xs` is empty and
/// `min_length` is greater than 0, the output is empty; otherwise, the output is infinite.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_min_length;
///
/// let bss = shortlex_vecs_min_length(2, exhaustive_bools())
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[false, false][..],
///         &[false, true],
///         &[true, false],
///         &[true, true],
///         &[false, false, false],
///         &[false, false, true],
///         &[false, true, false],
///         &[false, true, true],
///         &[true, false, false],
///         &[true, false, true],
///         &[true, true, false],
///         &[true, true, true],
///         &[false, false, false, false],
///         &[false, false, false, true],
///         &[false, false, true, false],
///         &[false, false, true, true],
///         &[false, true, false, false],
///         &[false, true, false, true],
///         &[false, true, true, false],
///         &[false, true, true, true]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_vecs_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    shortlex_vecs_from_length_iterator(
        primitive_int_increasing_inclusive_range(min_length, u64::MAX),
        xs,
    )
}

/// Generates all [`Vec`]s with lengths in $[a, b)$ and with elements from a specified iterator, in
/// shortlex order.
///
/// Shortlex order means that the [`Vec`]s are output from shortest to longest, and [`Vec`]s of the
/// same length are output in lexicographic order with respect to the ordering of the [`Vec`]
/// elements specified by the input iterator.
///
/// `xs` must be finite; if it's infinite and $a < b$, only [`Vec`]s of length `a` (or 0 and 1, if
/// `a` is 0) are ever produced.
///
/// The output length is
/// $$
/// \sum_{k=a}^{b-1} n^k,
/// $$
/// where $k$ is `xs.count()`.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_length_range;
///
/// let bss = shortlex_vecs_length_range(2, 4, exhaustive_bools()).collect_vec();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[false, false][..],
///         &[false, true],
///         &[true, false],
///         &[true, true],
///         &[false, false, false],
///         &[false, false, true],
///         &[false, true, false],
///         &[false, true, true],
///         &[true, false, false],
///         &[true, false, true],
///         &[true, true, false],
///         &[true, true, true]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_vecs_length_range<I: Clone + Iterator>(
    a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    if a > b {
        b = a;
    }
    shortlex_vecs_from_length_iterator(primitive_int_increasing_range(a, b), xs)
}

/// Generates all [`Vec`]s with lengths in $[a, b]$ and with elements from a specified iterator, in
/// shortlex order.
///
/// Shortlex order means that the [`Vec`]s are output from shortest to longest, and [`Vec`]s of the
/// same length are output in lexicographic order with respect to the ordering of the [`Vec`]
/// elements specified by the input iterator.
///
/// `xs` must be finite; if it's infinite, only [`Vec`]s of length `a` (or 0 and 1, if `a` is 0) are
/// ever produced.
///
/// The output length is
/// $$
/// \sum_{k=a}^b n^k,
/// $$
/// where $k$ is `xs.count()`.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_length_inclusive_range;
///
/// let bss = shortlex_vecs_length_inclusive_range(2, 3, exhaustive_bools()).collect_vec();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[false, false][..],
///         &[false, true],
///         &[true, false],
///         &[true, true],
///         &[false, false, false],
///         &[false, false, true],
///         &[false, true, false],
///         &[false, true, true],
///         &[true, false, false],
///         &[true, false, true],
///         &[true, true, false],
///         &[true, true, true]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_vecs_length_inclusive_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    if a > b {
        a = 1;
        b = 0;
    }
    shortlex_vecs_from_length_iterator(primitive_int_increasing_range(a, b.saturating_add(1)), xs)
}

#[doc(hidden)]
#[derive(Clone, Debug)]
struct ExhaustiveVecsGenerator<Y: Clone, J: Clone + Iterator<Item = Y>> {
    ys: J,
}

impl<Y: Clone, J: Clone + Iterator<Item = Y>>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<Y>, ExhaustiveFixedLengthVecs1Input<J>>
    for ExhaustiveVecsGenerator<Y, J>
{
    #[inline]
    fn get_ys(&self, &x: &u64) -> ExhaustiveFixedLengthVecs1Input<J> {
        exhaustive_vecs_fixed_length_1_input(
            self.ys.clone(),
            &vec![BitDistributorOutputType::normal(1); usize::exact_from(x)],
        )
    }
}

#[inline]
const fn exhaustive_vecs_from_element_iterator_helper<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
) -> ExhaustiveDependentPairs<
    u64,
    Vec<T>,
    RulerSequence<usize>,
    ExhaustiveVecsGenerator<T, J>,
    I,
    ExhaustiveFixedLengthVecs1Input<J>,
> {
    exhaustive_dependent_pairs_stop_after_empty_ys(
        ruler_sequence(),
        xs,
        ExhaustiveVecsGenerator { ys },
    )
}

/// Generates all [`Vec`]s with elements from a specified iterator and with lengths from another
/// iterator.
#[derive(Clone, Debug)]
pub struct ExhaustiveVecs<T: Clone, I: Iterator<Item = u64>, J: Clone + Iterator<Item = T>>(
    ExhaustiveDependentPairs<
        u64,
        Vec<T>,
        RulerSequence<usize>,
        ExhaustiveVecsGenerator<T, J>,
        I,
        ExhaustiveFixedLengthVecs1Input<J>,
    >,
);

impl<T: Clone, I: Iterator<Item = u64>, J: Clone + Iterator<Item = T>> Iterator
    for ExhaustiveVecs<T, I, J>
{
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Vec<T>> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all [`Vec`]s with elements from a specified iterator and with lengths from another
/// iterator.
///
/// The length-generating iterator is `xs`, and the element-generating iterator is `ys`.
///
/// If the lengths iterator has repetitions, then the generated [`Vec`]s will be repeated too.
///
/// There's one quirk if `ys` is empty: then the iterator will stop at some point after it
/// encounters a nonzero $\ell$, even if there are zeros later on. This prevents the iterator
/// hanging when given an empty `ys` and lengths $0, 1, 2, \ldots$.
///
/// - If `ys` is empty, the output length is finite.
/// - If `ys` is infinite, the output length is infinite.
/// - If `ys` is nonempty and finite, and `xs` is infinite, the output is infinite.
/// - If `ys` is nonempty and finite, and `xs` is finite, the output length is
///   $$
///   \sum_{k=0}^{m-1} n^{\ell_k},
///   $$
///   where $n$ is `ys.count()` and $m$ is `xs.count()`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::nevers::nevers;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_from_length_iterator;
///
/// let xss = exhaustive_vecs_from_length_iterator([2, 1, 2].iter().cloned(), exhaustive_bools())
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[false, false][..],
///         &[false],
///         &[false, true],
///         &[false, false],
///         &[true, false],
///         &[true],
///         &[true, true],
///         &[false, true],
///         &[true, false],
///         &[true, true]
///     ]
/// );
///
/// let xss =
///     exhaustive_vecs_from_length_iterator([0, 0, 1, 0].iter().cloned(), nevers()).collect_vec();
/// // Stops at some point after first empty ys
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[&[], &[]]
/// );
/// ```
#[inline]
pub const fn exhaustive_vecs_from_length_iterator<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    lengths: I,
    xs: J,
) -> ExhaustiveVecs<T, I, J> {
    ExhaustiveVecs(exhaustive_vecs_from_element_iterator_helper(lengths, xs))
}

/// Generates all [`Vec`]s with elements from a specified iterator.
///
/// If `xs` is empty, the output length is 1; otherwise, the output is infinite.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs;
///
/// let xss = exhaustive_vecs(exhaustive_unsigneds::<u32>())
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[0],
///         &[1],
///         &[0, 0, 0],
///         &[2],
///         &[0, 0],
///         &[3],
///         &[0, 0, 0, 0],
///         &[4],
///         &[0, 1],
///         &[5],
///         &[0, 0, 1],
///         &[6],
///         &[1, 0],
///         &[7],
///         &[0, 0, 0, 0, 0],
///         &[8],
///         &[1, 1],
///         &[9],
///         &[0, 1, 0]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs<I: Clone + Iterator>(
    xs: I,
) -> ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    exhaustive_vecs_from_length_iterator(exhaustive_unsigneds(), xs)
}

/// Generates all [`Vec`]s with a minimum length and with elements from a specified iterator.
///
/// If `xs` is empty and `min_length` is 0, the output length is 1; if `xs` is empty and
/// `min_length` is greater than 0, the output is empty; otherwise, the output is infinite.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_min_length;
///
/// let xss = exhaustive_vecs_min_length(2, exhaustive_unsigneds::<u32>())
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[0, 0][..],
///         &[0, 0, 0],
///         &[0, 1],
///         &[0, 0, 0, 0],
///         &[1, 0],
///         &[0, 0, 1],
///         &[1, 1],
///         &[0, 0, 0, 0, 0],
///         &[0, 2],
///         &[0, 1, 0],
///         &[0, 3],
///         &[0, 0, 0, 1],
///         &[1, 2],
///         &[0, 1, 1],
///         &[1, 3],
///         &[0, 0, 0, 0, 0, 0],
///         &[2, 0],
///         &[1, 0, 0],
///         &[2, 1],
///         &[0, 0, 1, 0]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    exhaustive_vecs_from_length_iterator(
        primitive_int_increasing_inclusive_range(min_length, u64::MAX),
        xs,
    )
}

/// Generates all [`Vec`]s with lengths in $[a, b)$ and with elements from a specified iterator.
///
/// - If $a \geq b$, the output length is 0.
/// - If $a = 0$ and $b = 1$, the output length is 1.
/// - If $a < b$, $b > 1$, and `xs` is infinite, the output length is infinite.
/// - If `xs` is finite, the output length is
///   $$
///   \sum_{k=a}^{b-1} n^k,
///   $$
///   where $k$ is `xs.count()`.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_length_range;
///
/// let xss = exhaustive_vecs_length_range(2, 4, exhaustive_unsigneds::<u32>())
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[0, 0][..],
///         &[0, 0, 0],
///         &[0, 1],
///         &[1, 0],
///         &[1, 1],
///         &[0, 0, 1],
///         &[0, 2],
///         &[0, 1, 0],
///         &[0, 3],
///         &[0, 1, 1],
///         &[1, 2],
///         &[1, 3],
///         &[2, 0],
///         &[1, 0, 0],
///         &[2, 1],
///         &[3, 0],
///         &[3, 1],
///         &[1, 0, 1],
///         &[2, 2],
///         &[2, 3]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs_length_range<I: Clone + Iterator>(
    a: u64,
    mut b: u64,
    xs: I,
) -> ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    if a > b {
        b = a;
    }
    exhaustive_vecs_from_length_iterator(primitive_int_increasing_range(a, b), xs)
}

/// Generates all [`Vec`]s with lengths in $[a, b]$ and with elements from a specified iterator.
///
/// - If $a > b$, the output length is 0.
/// - If $a = b = 0$, the output length is 1.
/// - If $a < b$, $b > 0$, and `xs` is infinite, the output length is infinite.
/// - If `xs` is finite, the output length is
///   $$
///   \sum_{k=a}^b n^k,
///   $$
///   where $k$ is `xs.count()`.
///
/// The lengths of the output [`Vec`]s grow logarithmically.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_length_inclusive_range;
///
/// let xss = exhaustive_vecs_length_inclusive_range(2, 4, exhaustive_unsigneds::<u32>())
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[0, 0][..],
///         &[0, 0, 0],
///         &[0, 1],
///         &[0, 0, 0, 0],
///         &[1, 0],
///         &[0, 0, 1],
///         &[1, 1],
///         &[0, 2],
///         &[0, 3],
///         &[0, 1, 0],
///         &[1, 2],
///         &[0, 0, 0, 1],
///         &[1, 3],
///         &[0, 1, 1],
///         &[2, 0],
///         &[1, 0, 0],
///         &[2, 1],
///         &[1, 0, 1],
///         &[3, 0],
///         &[0, 0, 1, 0]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs_length_inclusive_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    if a > b {
        a = 1;
        b = 0;
    }
    exhaustive_vecs_from_length_iterator(primitive_int_increasing_range(a, b.saturating_add(1)), xs)
}

/// Generates all collections of elements from an iterator, where the collections are of a fixed
/// length, have no repetitions, and are ordered the same way as in the iterator.
#[derive(Clone, Debug)]
pub struct LexFixedLengthOrderedUniqueCollections<I: Iterator, C: FromIterator<I::Item>>
where
    I::Item: Clone,
{
    first: bool,
    done: bool,
    xs: IteratorCache<I>,
    indices: Vec<usize>,
    phantom_i: PhantomData<*const I::Item>,
    phantom_c: PhantomData<*const C>,
}

impl<I: Iterator, C: FromIterator<I::Item>> LexFixedLengthOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    pub fn new(k: u64, xs: I) -> LexFixedLengthOrderedUniqueCollections<I, C> {
        LexFixedLengthOrderedUniqueCollections {
            first: true,
            done: false,
            xs: IteratorCache::new(xs),
            indices: (0..usize::exact_from(k)).collect(),
            phantom_i: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

#[doc(hidden)]
pub fn fixed_length_ordered_unique_indices_helper(
    n: usize,
    k: usize,
    indices: &mut [usize],
) -> bool {
    let mut expected_j = n - 1;
    let mut i = k - 1;
    // Find longest suffix of the form [..., n - 3, n - 2, n - 1]. After this loop, i is the index
    // right before this longest suffix.
    loop {
        if expected_j != indices[i] {
            break;
        }
        if i == 0 {
            return true;
        }
        i -= 1;
        expected_j -= 1;
    }
    let mut j = indices[i] + 1;
    for index in &mut indices[i..] {
        *index = j;
        j += 1;
    }
    false
}

impl<I: Iterator, C: FromIterator<I::Item>> Iterator
    for LexFixedLengthOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.done {
            return None;
        }
        let k = self.indices.len();
        if self.first {
            self.first = false;
            self.xs.get(k);
            if let Some(n) = self.xs.known_len() {
                if n < k {
                    self.done = true;
                    return None;
                }
            }
        } else {
            if k == 0 {
                self.done = true;
                return None;
            }
            if let Some(n) = self.xs.known_len() {
                if fixed_length_ordered_unique_indices_helper(n, k, &mut self.indices) {
                    self.done = true;
                    return None;
                }
            } else {
                *self.indices.last_mut().unwrap() += 1;
            }
        }
        if let Some(&last_index) = self.indices.last() {
            // Give known len a chance to be set
            self.xs.get(last_index + 1);
        }
        Some(
            self.indices
                .iter()
                .map(|&i| self.xs.assert_get(i).clone())
                .collect(),
        )
    }
}

/// Generates [`Vec`]s of a given length with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the same way as
/// they are in the source iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_fixed_length;
///
/// let xss = lex_ordered_unique_vecs_fixed_length(4, 1..=6).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2, 3, 4],
///         &[1, 2, 3, 5],
///         &[1, 2, 3, 6],
///         &[1, 2, 4, 5],
///         &[1, 2, 4, 6],
///         &[1, 2, 5, 6],
///         &[1, 3, 4, 5],
///         &[1, 3, 4, 6],
///         &[1, 3, 5, 6],
///         &[1, 4, 5, 6],
///         &[2, 3, 4, 5],
///         &[2, 3, 4, 6],
///         &[2, 3, 5, 6],
///         &[2, 4, 5, 6],
///         &[3, 4, 5, 6]
///     ]
/// );
/// ```
#[inline]
pub fn lex_ordered_unique_vecs_fixed_length<I: Iterator>(
    k: u64,
    xs: I,
) -> LexFixedLengthOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    LexFixedLengthOrderedUniqueCollections::new(k, xs)
}

/// Generates all collections of elements from an iterator in shortlex order, where the collections
/// have no repetitions and are ordered the same way as in the iterator.
#[derive(Clone)]
pub struct ShortlexOrderedUniqueCollections<I: Clone + Iterator, C: FromIterator<I::Item>>
where
    I::Item: Clone,
{
    current_len: u64,
    max_len: u64,
    xs: I,
    current_xss: LexFixedLengthOrderedUniqueCollections<I, C>,
}

impl<I: Clone + Iterator, C: FromIterator<I::Item>> ShortlexOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    pub(crate) fn new(a: u64, b: u64, xs: I) -> ShortlexOrderedUniqueCollections<I, C> {
        ShortlexOrderedUniqueCollections {
            current_len: a,
            max_len: b,
            xs: xs.clone(),
            current_xss: LexFixedLengthOrderedUniqueCollections::new(a, xs),
        }
    }
}

impl<I: Clone + Iterator, C: FromIterator<I::Item>> Iterator
    for ShortlexOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.current_len > self.max_len {
            return None;
        }
        if let Some(next) = self.current_xss.next() {
            Some(next)
        } else {
            self.current_len += 1;
            if self.current_len > self.max_len {
                return None;
            }
            self.current_xss = LexFixedLengthOrderedUniqueCollections {
                first: true,
                done: false,
                xs: IteratorCache::new(self.xs.clone()),
                indices: (0..usize::exact_from(self.current_len)).collect(),
                phantom_i: PhantomData,
                phantom_c: PhantomData,
            };
            if let Some(next) = self.current_xss.next() {
                Some(next)
            } else {
                // Prevent any further iteration
                self.max_len = 0;
                self.current_len = 1;
                None
            }
        }
    }
}

/// Generates [`Vec`]s with elements from a single iterator, such that each [`Vec`] has no repeated
/// elements, and the elements in each [`Vec`] are ordered the same way as they are in the source
/// iterator.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length 2 and above will never be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`Vec`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_ordered_unique_vecs;
///
/// let xss = shortlex_ordered_unique_vecs(1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[1],
///         &[2],
///         &[3],
///         &[4],
///         &[1, 2],
///         &[1, 3],
///         &[1, 4],
///         &[2, 3],
///         &[2, 4],
///         &[3, 4],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 4],
///         &[2, 3, 4],
///         &[1, 2, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_ordered_unique_vecs<I: Clone + Iterator>(
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    shortlex_ordered_unique_vecs_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the same way as
/// they are in the source iterator.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length `\max(2, \ell + 1)` and
/// above will never be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_ordered_unique_vecs_min_length;
///
/// let xss = shortlex_ordered_unique_vecs_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[1, 4],
///         &[2, 3],
///         &[2, 4],
///         &[3, 4],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 4],
///         &[2, 3, 4],
///         &[1, 2, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_ordered_unique_vecs_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    shortlex_ordered_unique_vecs_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b)$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the
/// same way as they are in the source iterator.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length `\max(2, a + 1)` and above
/// will never be generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^{b - 1} \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_ordered_unique_vecs_length_range;
///
/// let xss = shortlex_ordered_unique_vecs_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[1, 4],
///         &[2, 3],
///         &[2, 4],
///         &[3, 4],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 4],
///         &[2, 3, 4],
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_ordered_unique_vecs_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    shortlex_ordered_unique_vecs_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b]$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the
/// same way as they are in the source iterator.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length `\max(2, a + 1)` and above
/// will never be generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_ordered_unique_vecs_length_inclusive_range;
///
/// let xss = shortlex_ordered_unique_vecs_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[1, 4],
///         &[2, 3],
///         &[2, 4],
///         &[3, 4],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 4],
///         &[2, 3, 4],
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_ordered_unique_vecs_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    ShortlexOrderedUniqueCollections::new(a, b, xs)
}

/// Generates all collections of elements from an iterator in lexicographic order, where the
/// collections have no repetitions and are ordered the same way as in the iterator.
#[derive(Clone, Debug)]
pub struct LexOrderedUniqueCollections<I: Iterator, C: FromIterator<I::Item>>
where
    I::Item: Clone,
{
    done: bool,
    first: bool,
    min_len: usize,
    max_len: usize,
    xs: IteratorCache<I>,
    indices: Vec<usize>,
    phantom_i: PhantomData<*const I::Item>,
    phantom_c: PhantomData<*const C>,
}

impl<I: Iterator, C: FromIterator<I::Item>> LexOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    pub(crate) fn new(a: u64, b: u64, xs: I) -> LexOrderedUniqueCollections<I, C> {
        LexOrderedUniqueCollections {
            done: a > b,
            first: true,
            min_len: usize::exact_from(a),
            max_len: usize::exact_from(b),
            xs: IteratorCache::new(xs),
            indices: (0..usize::exact_from(a)).collect(),
            phantom_i: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<I: Iterator, C: FromIterator<I::Item>> Iterator for LexOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.done {
            return None;
        }
        let k = self.indices.len();
        if self.first {
            self.first = false;
            self.xs.get(k);
            if let Some(n) = self.xs.known_len() {
                if n < k {
                    self.done = true;
                    return None;
                }
            }
        } else if k == 0 {
            if self.xs.get(0).is_none() {
                self.done = true;
                return None;
            }
            self.indices.push(0);
        } else {
            let last_i = *self.indices.last().unwrap();
            let next_i = last_i + 1;
            if k < self.max_len && self.xs.get(next_i).is_some() {
                // For example, if xs is [0, 1, 2, 3] and max_len is 4, then the next set of indices
                // after [0, 1] is [0, 1, 2].
                self.indices.push(next_i);
            } else if k == self.min_len {
                // For example, if xs is [0, 1, 2, 3] and min_len is 2, then the next set of indices
                // after [1, 3] is [2, 3].
                if let Some(n) = self.xs.known_len() {
                    if fixed_length_ordered_unique_indices_helper(n, k, &mut self.indices) {
                        self.done = true;
                        return None;
                    }
                } else {
                    *self.indices.last_mut().unwrap() += 1;
                }
            } else if self.xs.get(next_i).is_some() {
                // For example, if xs is [0, 1, 2, 3] and max_len is 3, then the next set of indices
                // after [1, 2, 3] is [1, 2, 4].
                *self.indices.last_mut().unwrap() = next_i;
            } else {
                let x = self.indices.pop();
                if let Some(last) = self.indices.last_mut() {
                    // For example, if xs is [0, 1, 2, 3] and max_len is 3, then the next set of
                    // indices after [0, 1, 2] is [0, 1, 3].
                    *last += 1;
                } else {
                    let next_x = x.unwrap() + 1;
                    if self.xs.get(next_x).is_none() {
                        // For example, if xs is [0, 1, 2, 3], then nothing comes after the indices
                        // [3].
                        self.done = true;
                        return None;
                    }
                    // For example, if xs is [0, 1, 2, 3] and max_len is 1, then the next set of
                    // indices after [0] is [1].
                    self.indices.push(next_x);
                }
            }
        }
        if let Some(&last_index) = self.indices.last() {
            // Give known len a chance to be set
            self.xs.get(last_index + 1);
        }
        Some(
            self.indices
                .iter()
                .map(|&i| self.xs.assert_get(i).clone())
                .collect(),
        )
    }
}

/// Generates [`Vec`]s with elements from a single iterator, such that each [`Vec`] has no repeated
/// elements, and the elements in each [`Vec`] are ordered the same way as they are in the source
/// iterator.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`Vec`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs;
///
/// let xss = lex_ordered_unique_vecs(1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[1],
///         &[1, 2],
///         &[1, 2, 3],
///         &[1, 2, 3, 4],
///         &[1, 2, 4],
///         &[1, 3],
///         &[1, 3, 4],
///         &[1, 4],
///         &[2],
///         &[2, 3],
///         &[2, 3, 4],
///         &[2, 4],
///         &[3],
///         &[3, 4],
///         &[4]
///     ]
/// );
/// ```
#[inline]
pub fn lex_ordered_unique_vecs<I: Clone + Iterator>(
    xs: I,
) -> LexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    lex_ordered_unique_vecs_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the same way as
/// they are in the source iterator.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_min_length;
///
/// let xss = lex_ordered_unique_vecs_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 2, 3],
///         &[1, 2, 3, 4],
///         &[1, 2, 4],
///         &[1, 3],
///         &[1, 3, 4],
///         &[1, 4],
///         &[2, 3],
///         &[2, 3, 4],
///         &[2, 4],
///         &[3, 4],
///     ]
/// );
/// ```
#[inline]
pub fn lex_ordered_unique_vecs_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    lex_ordered_unique_vecs_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b)$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the
/// same way as they are in the source iterator.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^{b - 1} \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_length_range;
///
/// let xss = lex_ordered_unique_vecs_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3],
///         &[1, 3, 4],
///         &[1, 4],
///         &[2, 3],
///         &[2, 3, 4],
///         &[2, 4],
///         &[3, 4],
///     ]
/// );
/// ```
#[inline]
pub fn lex_ordered_unique_vecs_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    lex_ordered_unique_vecs_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b]$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the
/// same way as they are in the source iterator.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs_length_inclusive_range;
///
/// let xss = lex_ordered_unique_vecs_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3],
///         &[1, 3, 4],
///         &[1, 4],
///         &[2, 3],
///         &[2, 3, 4],
///         &[2, 4],
///         &[3, 4],
///     ]
/// );
/// ```
#[inline]
pub fn lex_ordered_unique_vecs_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> LexOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    LexOrderedUniqueCollections::new(a, b, xs)
}

/// This function is used for iterating through all bit patterns with a specified number of minimum
/// and maximum `true` bits.
///
/// Given an existing bit pattern, and a reference `bit_count`, which must equal the number of
/// `true`s in the pattern, mutates the pattern into the next pattern with a valid number of `true`
/// bits. See the unit tests for many examples.
///
/// # Worst-case complexity
/// $$
/// T(k) = O(k)
/// $$
///
/// $$
/// M(k) = O(k)
/// $$
///
/// where $T$ is time, $M$ is additional memory, and $k$ is the length of `pattern`. The memory
/// usage is only linear when the pattern vector needs to be reallocated, which happens rarely.
///
/// # Panics
/// Panics if `max_bits` is zero. (However, `min_bits` may be zero.)
///
/// # Examples
/// ```
/// use malachite_base::vecs::exhaustive::next_bit_pattern;
///
/// // Suppose we are generating all bit patterns with 2 to 4 true bits, inclusive. Suppose our
/// // current pattern is `1111000`. Then, the lexicographically next largest valid pattern is
/// // `10000001`. (All patterns of the form `1111xxx`, where the `x`s are not all zero, have too
/// // many ones. That brings us to `10000000`, which has too few ones, and then `10000001`.)
/// //
/// // The patterns are represented "in reverse", with least-significant bits appearing first.
/// let mut pattern = vec![false, false, false, true, true, true, true];
/// let mut bit_count = 4;
/// next_bit_pattern(&mut pattern, &mut bit_count, 2, 4);
/// assert_eq!(
///     pattern,
///     &[true, false, false, false, false, false, false, true]
/// );
/// assert_eq!(bit_count, 2);
/// ```
pub fn next_bit_pattern(
    pattern: &mut Vec<bool>,
    bit_count: &mut usize,
    min_bits: usize,
    max_bits: usize,
) {
    assert_ne!(max_bits, 0);
    match pattern.first() {
        None => {
            pattern.push(true);
            *bit_count = 1;
        }
        Some(&false) => {
            if *bit_count < max_bits {
                pattern[0] = true;
                *bit_count += 1;
            } else {
                let leading_false_count = pattern.iter().take_while(|&&b| !b).count();
                let true_after_false_count = pattern[leading_false_count..]
                    .iter()
                    .take_while(|&&b| b)
                    .count();
                let tf_count = leading_false_count + true_after_false_count;
                if tf_count == pattern.len() {
                    for b in &mut *pattern {
                        *b = false;
                    }
                    pattern.push(true);
                    *bit_count = 1;
                } else {
                    for b in &mut pattern[leading_false_count..tf_count] {
                        *b = false;
                    }
                    pattern[tf_count] = true;
                    *bit_count -= true_after_false_count - 1;
                }
                if *bit_count < min_bits {
                    let diff = min_bits - *bit_count;
                    for b in &mut pattern[..diff] {
                        *b = true;
                    }
                    *bit_count += diff;
                }
            }
        }
        Some(&true) => {
            let leading_true_count = pattern.iter().take_while(|&&b| b).count();
            for b in &mut pattern[..leading_true_count] {
                *b = false;
            }
            if leading_true_count == pattern.len() {
                pattern.push(true);
            } else {
                pattern[leading_true_count] = true;
            }
            *bit_count -= leading_true_count - 1;
            if *bit_count < min_bits {
                let diff = min_bits - *bit_count;
                for b in &mut pattern[..diff] {
                    *b = true;
                }
                *bit_count += diff;
            }
        }
    }
}

#[derive(Clone)]
#[doc(hidden)]
pub struct ExhaustiveOrderedUniqueCollectionsGreaterThanOne<I: Iterator, C: FromIterator<I::Item>>
where
    I::Item: Clone,
{
    done: bool,
    first: bool,
    min_bits: usize,
    max_bits: usize,
    xs: IteratorCache<I>,
    pattern: Vec<bool>,
    bit_count: usize,
    phantom: PhantomData<*const C>,
}

impl<I: Iterator, C: FromIterator<I::Item>> Iterator
    for ExhaustiveOrderedUniqueCollectionsGreaterThanOne<I, C>
where
    I::Item: Clone,
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.done {
            return None;
        } else if self.first {
            self.first = false;
        } else {
            next_bit_pattern(
                &mut self.pattern,
                &mut self.bit_count,
                self.min_bits,
                self.max_bits,
            );
        }
        if !self.pattern.is_empty() && self.xs.get(self.pattern.len() - 1).is_none() {
            self.done = true;
            return None;
        }
        Some(
            self.pattern
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| {
                    if b {
                        Some(self.xs.assert_get(i).clone())
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

/// Generates all collections of elements from an iterator, where the collections have no
/// repetitions and are ordered the same way as in the iterator.
#[derive(Clone)]
pub enum ExhaustiveOrderedUniqueCollections<I: Iterator, C: FromIterator<I::Item>>
where
    I::Item: Clone,
{
    None,
    Zero(bool),
    ZeroOne(bool, I),
    One(I),
    GreaterThanOne(ExhaustiveOrderedUniqueCollectionsGreaterThanOne<I, C>),
}

impl<I: Iterator, C: FromIterator<I::Item>> ExhaustiveOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    pub(crate) fn new(a: u64, b: u64, xs: I) -> ExhaustiveOrderedUniqueCollections<I, C> {
        match (a, b) {
            (a, b) if a > b => ExhaustiveOrderedUniqueCollections::None,
            (0, 0) => ExhaustiveOrderedUniqueCollections::Zero(false),
            (0, 1) => ExhaustiveOrderedUniqueCollections::ZeroOne(true, xs),
            (1, 1) => ExhaustiveOrderedUniqueCollections::One(xs),
            (a, b) => ExhaustiveOrderedUniqueCollections::GreaterThanOne(
                ExhaustiveOrderedUniqueCollectionsGreaterThanOne {
                    done: false,
                    first: true,
                    min_bits: usize::saturating_from(a),
                    max_bits: usize::saturating_from(b),
                    xs: IteratorCache::new(xs),
                    pattern: vec![true; usize::saturating_from(a)],
                    bit_count: usize::saturating_from(a),
                    phantom: PhantomData,
                },
            ),
        }
    }
}

impl<I: Iterator, C: FromIterator<I::Item>> Iterator for ExhaustiveOrderedUniqueCollections<I, C>
where
    I::Item: Clone,
{
    type Item = C;

    fn next(&mut self) -> Option<C> {
        match self {
            ExhaustiveOrderedUniqueCollections::None => None,
            ExhaustiveOrderedUniqueCollections::Zero(ref mut done) => {
                if *done {
                    None
                } else {
                    *done = true;
                    Some(empty().collect())
                }
            }
            ExhaustiveOrderedUniqueCollections::ZeroOne(ref mut first, ref mut xs) => {
                if *first {
                    *first = false;
                    Some(empty().collect())
                } else {
                    xs.next().map(|x| once(x).collect())
                }
            }
            ExhaustiveOrderedUniqueCollections::One(ref mut xs) => {
                xs.next().map(|x| once(x).collect())
            }
            ExhaustiveOrderedUniqueCollections::GreaterThanOne(ref mut xs) => xs.next(),
        }
    }
}

/// Generates [`Vec`]s of a given length with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the same way as
/// they are in the source iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is $\binom{n}{k}$.
///
/// If $k$ is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_fixed_length;
///
/// let xss = exhaustive_ordered_unique_vecs_fixed_length(4, 1..=6).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2, 3, 4],
///         &[1, 2, 3, 5],
///         &[1, 2, 4, 5],
///         &[1, 3, 4, 5],
///         &[2, 3, 4, 5],
///         &[1, 2, 3, 6],
///         &[1, 2, 4, 6],
///         &[1, 3, 4, 6],
///         &[2, 3, 4, 6],
///         &[1, 2, 5, 6],
///         &[1, 3, 5, 6],
///         &[2, 3, 5, 6],
///         &[1, 4, 5, 6],
///         &[2, 4, 5, 6],
///         &[3, 4, 5, 6]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_vecs_fixed_length<I: Iterator>(
    k: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    exhaustive_ordered_unique_vecs_length_inclusive_range(k, k, xs)
}

/// Generates [`Vec`]s with elements from a single iterator, such that each [`Vec`] has no repeated
/// elements, and the elements in each [`Vec`] are ordered the same way as they are in the source
/// iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $2^n$.
///
/// If `xs` is empty, the output consists of a single empty [`Vec`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs;
///
/// let xss = exhaustive_ordered_unique_vecs(1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[1],
///         &[2],
///         &[1, 2],
///         &[3],
///         &[1, 3],
///         &[2, 3],
///         &[1, 2, 3],
///         &[4],
///         &[1, 4],
///         &[2, 4],
///         &[1, 2, 4],
///         &[3, 4],
///         &[1, 3, 4],
///         &[2, 3, 4],
///         &[1, 2, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_vecs<I: Iterator>(
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    exhaustive_ordered_unique_vecs_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the same way as
/// they are in the source iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_min_length;
///
/// let xss = exhaustive_ordered_unique_vecs_min_length(2, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[2, 3],
///         &[1, 2, 3],
///         &[1, 4],
///         &[2, 4],
///         &[1, 2, 4],
///         &[3, 4],
///         &[1, 3, 4],
///         &[2, 3, 4],
///         &[1, 2, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_vecs_min_length<I: Iterator>(
    min_length: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    exhaustive_ordered_unique_vecs_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b)$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the
/// same way as they are in the source iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^{b - 1} \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_length_range;
///
/// let xss = exhaustive_ordered_unique_vecs_length_range(2, 4, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[2, 3],
///         &[1, 2, 3],
///         &[1, 4],
///         &[2, 4],
///         &[1, 2, 4],
///         &[3, 4],
///         &[1, 3, 4],
///         &[2, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_vecs_length_range<I: Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    if a >= b {
        ExhaustiveOrderedUniqueCollections::None
    } else {
        exhaustive_ordered_unique_vecs_length_inclusive_range(a, b - 1, xs)
    }
}

/// Generates [`Vec`]s, with lengths in a range $[a, b]$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements, and the elements in each [`Vec`] are ordered the
/// same way as they are in the source iterator.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \binom{n}{i}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_length_inclusive_range;
///
/// let xss = exhaustive_ordered_unique_vecs_length_inclusive_range(2, 3, 1..=4).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[2, 3],
///         &[1, 2, 3],
///         &[1, 4],
///         &[2, 4],
///         &[1, 2, 4],
///         &[3, 4],
///         &[1, 3, 4],
///         &[2, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_ordered_unique_vecs_length_inclusive_range<I: Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>
where
    I::Item: Clone,
{
    ExhaustiveOrderedUniqueCollections::new(a, b, xs)
}

fn fixed_length_unique_indices_helper(indices: &mut [usize], used: &mut [bool]) -> bool {
    let n = used.len();
    let k = indices.len();
    assert!(k <= n);
    for i in (0..k).rev() {
        let x = indices[i];
        used[x] = false;
        for y in x + 1..n {
            if !used[y] {
                indices[i] = y;
                used[y] = true;
                let mut p = 0;
                for j in &mut indices[i + 1..k] {
                    while used[p] {
                        p += 1;
                    }
                    *j = p;
                    used[p] = true;
                }
                return false;
            }
        }
    }
    true
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct UniqueIndices {
    pub done: bool,
    first: bool,
    indices: Vec<usize>,
    pub used: Vec<bool>,
}

impl UniqueIndices {
    #[doc(hidden)]
    pub fn get_n(&self) -> usize {
        self.used.len()
    }

    #[doc(hidden)]
    pub fn increment_n(&mut self) {
        self.used.push(false);
    }
}

impl Iterator for UniqueIndices {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            None
        } else if self.first {
            self.first = false;
            Some(self.indices.clone())
        } else if fixed_length_unique_indices_helper(&mut self.indices, &mut self.used) {
            self.done = true;
            None
        } else {
            Some(self.indices.clone())
        }
    }
}

#[doc(hidden)]
pub fn unique_indices(n: usize, k: usize) -> UniqueIndices {
    UniqueIndices {
        done: n == 0 && k != 0,
        first: true,
        indices: (0..k).collect_vec(),
        used: repeat_n(true, k)
            .chain(repeat_n(false, n - k))
            .collect_vec(),
    }
}

/// Generates all [`Vec`]s of elements from an iterator, where the [`Vec`]s are of a fixed length
/// and have no repetitions.
#[derive(Clone)]
pub struct LexUniqueVecsFixedLength<I: Iterator>
where
    I::Item: Clone,
{
    first: bool,
    xs: IteratorCache<I>,
    indices: UniqueIndices,
}

impl<I: Iterator> Iterator for LexUniqueVecsFixedLength<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if !self.indices.used.is_empty() && self.xs.get(self.indices.get_n() - 1).is_none() {
                self.indices.done = true;
            }
            self.first = false;
        }
        if self.xs.get(self.indices.get_n()).is_some() {
            self.indices.increment_n();
        }
        self.indices.next().map(|indices| {
            indices
                .into_iter()
                .map(|i| self.xs.assert_get(i).clone())
                .collect_vec()
        })
    }
}

/// Generates [`Vec`]s of a given length with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is
/// $$
/// (n)_ k = \prod_ {i=0}^{k-1}(n - i) = frac{n!}{(n-k)!}.
/// $$
///
/// If $k$ is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_unique_vecs_fixed_length;
///
/// let xss = lex_unique_vecs_fixed_length(4, 1..=6)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2, 3, 4],
///         &[1, 2, 3, 5],
///         &[1, 2, 3, 6],
///         &[1, 2, 4, 3],
///         &[1, 2, 4, 5],
///         &[1, 2, 4, 6],
///         &[1, 2, 5, 3],
///         &[1, 2, 5, 4],
///         &[1, 2, 5, 6],
///         &[1, 2, 6, 3],
///         &[1, 2, 6, 4],
///         &[1, 2, 6, 5],
///         &[1, 3, 2, 4],
///         &[1, 3, 2, 5],
///         &[1, 3, 2, 6],
///         &[1, 3, 4, 2],
///         &[1, 3, 4, 5],
///         &[1, 3, 4, 6],
///         &[1, 3, 5, 2],
///         &[1, 3, 5, 4],
///     ]
/// );
/// ```
#[inline]
pub fn lex_unique_vecs_fixed_length<I: Iterator>(k: u64, xs: I) -> LexUniqueVecsFixedLength<I>
where
    I::Item: Clone,
{
    let k = usize::exact_from(k);
    LexUniqueVecsFixedLength {
        first: true,
        xs: IteratorCache::new(xs),
        // Initial n is k, but will grow to reach actual n (or forever, if n is infinite)
        indices: unique_indices(k, k),
    }
}

/// Generates all [`Vec`]s of elements from an iterator in shortlex order, where the [`Vec`]s have
/// no repetitions.
#[derive(Clone)]
pub struct ShortlexUniqueVecs<I: Clone + Iterator>
where
    I::Item: Clone,
{
    current_len: u64,
    max_len: u64,
    xs: I,
    current_xss: LexUniqueVecsFixedLength<I>,
}

impl<I: Clone + Iterator> ShortlexUniqueVecs<I>
where
    I::Item: Clone,
{
    fn new(a: u64, b: u64, xs: I) -> ShortlexUniqueVecs<I> {
        ShortlexUniqueVecs {
            current_len: a,
            max_len: b,
            xs: xs.clone(),
            current_xss: lex_unique_vecs_fixed_length(a, xs),
        }
    }
}

impl<I: Clone + Iterator> Iterator for ShortlexUniqueVecs<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if self.current_len > self.max_len {
            return None;
        }
        if let Some(next) = self.current_xss.next() {
            Some(next)
        } else {
            self.current_len += 1;
            if self.current_len > self.max_len {
                return None;
            }
            self.current_xss = lex_unique_vecs_fixed_length(self.current_len, self.xs.clone());
            if let Some(next) = self.current_xss.next() {
                Some(next)
            } else {
                // Prevent any further iteration
                self.max_len = 0;
                self.current_len = 1;
                None
            }
        }
    }
}

/// Generates [`Vec`]s with elements from a single iterator, such that each [`Vec`] has no repeated
/// elements.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length 2 and above will never be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_ {k=0}^n \frac{n!}{k!}
/// $$
/// $$
/// = \\begin{cases}
///     1 & \text{if} \\quad n = 0, \\\\
///     2 & \text{if} \\quad n = 1, \\\\
///     \operatorname{round}(en!) & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// See <https://oeis.org/A000522>.
///
/// If `xs` is empty, the output consists of a single empty [`Vec`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_unique_vecs;
///
/// let xss = shortlex_unique_vecs(1..=4).take(20).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[1],
///         &[2],
///         &[3],
///         &[4],
///         &[1, 2],
///         &[1, 3],
///         &[1, 4],
///         &[2, 1],
///         &[2, 3],
///         &[2, 4],
///         &[3, 1],
///         &[3, 2],
///         &[3, 4],
///         &[4, 1],
///         &[4, 2],
///         &[4, 3],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_unique_vecs<I: Clone + Iterator>(xs: I) -> ShortlexUniqueVecs<I>
where
    I::Item: Clone,
{
    shortlex_unique_vecs_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length `\max(2, \ell + 1)` and
/// above will never be generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_ {k=\ell}^n \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_unique_vecs_min_length;
///
/// let xss = shortlex_unique_vecs_min_length(2, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[1, 4],
///         &[2, 1],
///         &[2, 3],
///         &[2, 4],
///         &[3, 1],
///         &[3, 2],
///         &[3, 4],
///         &[4, 1],
///         &[4, 2],
///         &[4, 3],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2],
///         &[1, 3, 4],
///         &[1, 4, 2],
///         &[1, 4, 3],
///         &[2, 1, 3],
///         &[2, 1, 4]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_unique_vecs_min_length<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
) -> ShortlexUniqueVecs<I>
where
    I::Item: Clone,
{
    shortlex_unique_vecs_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b)$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length `\max(2, a + 1)` and above
/// will never be generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^{b - 1} \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_unique_vecs_length_range;
///
/// let xss = shortlex_unique_vecs_length_range(2, 4, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[1, 4],
///         &[2, 1],
///         &[2, 3],
///         &[2, 4],
///         &[3, 1],
///         &[3, 2],
///         &[3, 4],
///         &[4, 1],
///         &[4, 2],
///         &[4, 3],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2],
///         &[1, 3, 4],
///         &[1, 4, 2],
///         &[1, 4, 3],
///         &[2, 1, 3],
///         &[2, 1, 4]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_unique_vecs_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> ShortlexUniqueVecs<I>
where
    I::Item: Clone,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    shortlex_unique_vecs_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b]$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements.
///
/// The [`Vec`]s are generated in order of increasing length, and within each length they are
/// ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, [`Vec`]s of length `\max(2, a + 1)` and above
/// will never be generated.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::shortlex_unique_vecs_length_inclusive_range;
///
/// let xss = shortlex_unique_vecs_length_inclusive_range(2, 3, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[1, 4],
///         &[2, 1],
///         &[2, 3],
///         &[2, 4],
///         &[3, 1],
///         &[3, 2],
///         &[3, 4],
///         &[4, 1],
///         &[4, 2],
///         &[4, 3],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2],
///         &[1, 3, 4],
///         &[1, 4, 2],
///         &[1, 4, 3],
///         &[2, 1, 3],
///         &[2, 1, 4]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_unique_vecs_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexUniqueVecs<I>
where
    I::Item: Clone,
{
    ShortlexUniqueVecs::new(a, b, xs)
}

fn compare_indexed_vecs_lex<T>(xs: &[(usize, T)], ys: &[(usize, T)]) -> Ordering {
    let xs_len = xs.len();
    let ys_len = ys.len();
    for i in 0..min(xs_len, ys_len) {
        let o = xs[i].0.cmp(&ys[i].0);
        if o != Equal {
            return o;
        }
    }
    xs_len.cmp(&ys_len)
}

/// Generates all collections of elements from an iterator in lexicographic order, where the
/// collections have no repetitions.
#[derive(Clone)]
pub struct LexUniqueVecs<I: Clone + Iterator>
where
    I::Item: Clone,
{
    done: bool,
    first: bool,
    min: usize,
    max: usize,
    xs_for_prefix: I,
    xs: I,
    phase_1_vec: Option<Vec<I::Item>>,
    xsss: Vec<LexUniqueVecsFixedLength<Zip<RangeFrom<usize>, I>>>,
    next_xss: Vec<Option<Vec<(usize, I::Item)>>>,
}

impl<I: Clone + Iterator> Iterator for LexUniqueVecs<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if self.done {
            return None;
        }
        if self.first {
            self.first = false;
            return self.phase_1_vec.clone();
        }
        if let Some(prefix) = self.phase_1_vec.as_mut() {
            if prefix.len() < self.max {
                if let Some(x) = self.xs_for_prefix.next() {
                    prefix.push(x);
                    return Some(prefix.clone());
                }
                self.max = prefix.len();
            }
        }
        if self.phase_1_vec.is_some() {
            for k in self.min..=self.max {
                let mut xss =
                    lex_unique_vecs_fixed_length(u64::exact_from(k), (0..).zip(self.xs.clone()));
                // Skip over first Vec of length k, which was already generated in phase 1
                xss.next();
                self.next_xss.push(xss.next());
                self.xsss.push(xss);
            }
            self.phase_1_vec = None;
        }
        let mut min_i = None;
        let mut i_done = None;
        for i in 0..self.next_xss.len() {
            let choose = if let Some(xs) = &self.next_xss[i] {
                if let Some(min_i) = min_i {
                    let ys: &Option<Vec<(usize, I::Item)>> = &self.next_xss[min_i];
                    compare_indexed_vecs_lex(xs, ys.as_ref().unwrap()) == Less
                } else {
                    true
                }
            } else {
                i_done = Some(i);
                false
            };
            if choose {
                min_i = Some(i);
            }
        }
        if let Some(i) = min_i {
            self.next_xss.push(self.xsss[i].next());
            let xs = self
                .next_xss
                .swap_remove(i)
                .map(|xs| xs.into_iter().map(|p| p.1).collect());
            if let Some(i_done) = i_done {
                self.xsss.remove(i_done);
                self.next_xss.remove(i_done);
            }
            xs
        } else {
            self.done = true;
            None
        }
    }
}

/// Generates [`Vec`]s with elements from a single iterator, such that each [`Vec`] has no repeated
/// elements.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_ {k=0}^n \frac{n!}{k!}
/// $$
/// $$
/// = \\begin{cases}
///     1 & \text{if} \\quad n = 0, \\\\
///     2 & \text{if} \\quad n = 1, \\\\
///     \operatorname{round}(en!) & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// See <https://oeis.org/A000522>.
///
/// If `xs` is empty, the output consists of a single empty [`Vec`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_unique_vecs;
///
/// let xss = lex_unique_vecs(1..=4).take(20).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[1],
///         &[1, 2],
///         &[1, 2, 3],
///         &[1, 2, 3, 4],
///         &[1, 2, 4],
///         &[1, 2, 4, 3],
///         &[1, 3],
///         &[1, 3, 2],
///         &[1, 3, 2, 4],
///         &[1, 3, 4],
///         &[1, 3, 4, 2],
///         &[1, 4],
///         &[1, 4, 2],
///         &[1, 4, 2, 3],
///         &[1, 4, 3],
///         &[1, 4, 3, 2],
///         &[2],
///         &[2, 1],
///         &[2, 1, 3]
///     ]
/// );
/// ```
#[inline]
pub fn lex_unique_vecs<I: Clone + Iterator>(xs: I) -> LexUniqueVecs<I>
where
    I::Item: Clone,
{
    lex_unique_vecs_length_inclusive_range(0, u64::MAX, xs)
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_unique_vecs_min_length;
///
/// let xss = lex_unique_vecs_min_length(2, 1..=4).take(20).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 2, 3],
///         &[1, 2, 3, 4],
///         &[1, 2, 4],
///         &[1, 2, 4, 3],
///         &[1, 3],
///         &[1, 3, 2],
///         &[1, 3, 2, 4],
///         &[1, 3, 4],
///         &[1, 3, 4, 2],
///         &[1, 4],
///         &[1, 4, 2],
///         &[1, 4, 2, 3],
///         &[1, 4, 3],
///         &[1, 4, 3, 2],
///         &[2, 1],
///         &[2, 1, 3],
///         &[2, 1, 3, 4],
///         &[2, 1, 4],
///         &[2, 1, 4, 3]
///     ]
/// );
/// ```
#[inline]
pub fn lex_unique_vecs_min_length<I: Clone + Iterator>(min_length: u64, xs: I) -> LexUniqueVecs<I>
where
    I::Item: Clone,
{
    lex_unique_vecs_length_inclusive_range(min_length, u64::MAX, xs)
}

/// Generates [`Vec`]s, with lengths in a range $[a, b)$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^{b - 1} \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_unique_vecs_length_range;
///
/// let xss = lex_unique_vecs_length_range(2, 4, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3],
///         &[1, 3, 2],
///         &[1, 3, 4],
///         &[1, 4],
///         &[1, 4, 2],
///         &[1, 4, 3],
///         &[2, 1],
///         &[2, 1, 3],
///         &[2, 1, 4],
///         &[2, 3],
///         &[2, 3, 1],
///         &[2, 3, 4],
///         &[2, 4],
///         &[2, 4, 1],
///         &[2, 4, 3],
///         &[3, 1],
///         &[3, 1, 2]
///     ]
/// );
/// ```
#[inline]
pub fn lex_unique_vecs_length_range<I: Clone + Iterator>(
    mut a: u64,
    mut b: u64,
    xs: I,
) -> LexUniqueVecs<I>
where
    I::Item: Clone,
{
    if b == 0 {
        // Transform an empty (x, 0) range into (2, 1), which is also empty but doesn't cause
        // overflow
        a = 2;
        b = 1;
    }
    lex_unique_vecs_length_inclusive_range(a, b - 1, xs)
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements.
///
/// The [`Vec`]s are ordered lexicographically with respect to the order of the element iterator.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// The iterator should be finite; if it is infinite, only prefixes of the iterator will be
/// generated.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_{i=\ell}^n \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_unique_vecs_min_length;
///
/// let xss = lex_unique_vecs_min_length(2, 1..=4).take(20).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 2, 3],
///         &[1, 2, 3, 4],
///         &[1, 2, 4],
///         &[1, 2, 4, 3],
///         &[1, 3],
///         &[1, 3, 2],
///         &[1, 3, 2, 4],
///         &[1, 3, 4],
///         &[1, 3, 4, 2],
///         &[1, 4],
///         &[1, 4, 2],
///         &[1, 4, 2, 3],
///         &[1, 4, 3],
///         &[1, 4, 3, 2],
///         &[2, 1],
///         &[2, 1, 3],
///         &[2, 1, 3, 4],
///         &[2, 1, 4],
///         &[2, 1, 4, 3]
///     ]
/// );
/// ```
#[inline]
pub fn lex_unique_vecs_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> LexUniqueVecs<I>
where
    I::Item: Clone,
{
    let a = usize::exact_from(a);
    let b = usize::exact_from(b);
    let mut xs_for_prefix = xs.clone();
    let phase_1_vec = (&mut xs_for_prefix).take(a).collect_vec();
    LexUniqueVecs {
        done: a > b || phase_1_vec.len() < a,
        first: true,
        min: a,
        max: b,
        xs_for_prefix,
        xs,
        phase_1_vec: Some(phase_1_vec),
        xsss: Vec::new(),
        next_xss: Vec::new(),
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct ExhaustiveUniqueVecs2<I: Iterator>
where
    I::Item: Clone,
{
    next: Option<(I::Item, I::Item)>,
    ps: ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>,
}

impl<I: Iterator> Iterator for ExhaustiveUniqueVecs2<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if self.next.is_some() {
            let (a, b) = take(&mut self.next).unwrap();
            Some(vec![a, b])
        } else if let Some(p) = self.ps.next() {
            self.next = Some((p[1].clone(), p[0].clone()));
            Some(p)
        } else {
            None
        }
    }
}

fn exhaustive_unique_vecs_2<I: Iterator>(xs: I) -> ExhaustiveUniqueVecs2<I>
where
    I::Item: Clone,
{
    ExhaustiveUniqueVecs2 {
        next: None,
        ps: exhaustive_ordered_unique_vecs_fixed_length(2, xs),
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExhaustiveUniqueVecsGenerator<T: Clone, I: Iterator<Item = T>> {
    phantom_t: PhantomData<T>,
    phantom_i: PhantomData<I>,
}

impl<T: Clone, I: Iterator<Item = T>> ExhaustiveUniqueVecsGenerator<T, I> {
    #[doc(hidden)]
    #[inline]
    pub const fn new() -> ExhaustiveUniqueVecsGenerator<T, I> {
        ExhaustiveUniqueVecsGenerator {
            phantom_i: PhantomData,
            phantom_t: PhantomData,
        }
    }
}

impl<T: Clone, I: Iterator<Item = T>>
    ExhaustiveDependentPairsYsGenerator<Vec<T>, Vec<T>, ExhaustiveVecPermutations<T>>
    for ExhaustiveUniqueVecsGenerator<T, I>
{
    #[inline]
    fn get_ys(&self, xs: &Vec<T>) -> ExhaustiveVecPermutations<T> {
        exhaustive_vec_permutations(xs.clone())
    }
}

/// Generates all fixed-length [`Vec`]s of elements from an iterator, where the [`Vec`]s have no
/// repetitions and are ordered the same way as in the iterator.
#[derive(Clone)]
pub enum ExhaustiveUniqueVecsFixedLength<I: Iterator>
where
    I::Item: Clone,
{
    Zero(bool),
    One(I),
    Two(ExhaustiveUniqueVecs2<I>),
    GreaterThanTwo(
        ExhaustiveDependentPairs<
            Vec<I::Item>,
            Vec<I::Item>,
            RulerSequence<usize>,
            ExhaustiveUniqueVecsGenerator<I::Item, I>,
            ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>,
            ExhaustiveVecPermutations<I::Item>,
        >,
    ),
}

impl<I: Iterator> Iterator for ExhaustiveUniqueVecsFixedLength<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            ExhaustiveUniqueVecsFixedLength::Zero(done) => {
                if *done {
                    None
                } else {
                    *done = true;
                    Some(Vec::new())
                }
            }
            ExhaustiveUniqueVecsFixedLength::One(xs) => xs.next().map(|x| vec![x]),
            ExhaustiveUniqueVecsFixedLength::Two(ps) => ps.next(),
            ExhaustiveUniqueVecsFixedLength::GreaterThanTwo(xss) => xss.next().map(|p| p.1),
        }
    }
}

/// Generates [`Vec`]s of a given length with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $k$ is 0, the output length is 1.
///
/// If $k$ is nonzero and the input iterator is infinite, the output length is also infinite.
///
/// If $k$ is nonzero and the input iterator length is $n$, the output length is
/// $$
/// (n)_ k = \prod_ {i=0}^{k-1}(n - i) = frac{n!}{(n-k)!}.
/// $$
///
/// If $k$ is 0, the output consists of one empty [`Vec`].
///
/// If `xs` is empty, the output is also empty, unless $k$ is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_unique_vecs_fixed_length;
///
/// let xss = exhaustive_unique_vecs_fixed_length(4, 1..=6)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2, 3, 4],
///         &[1, 2, 3, 5],
///         &[1, 2, 4, 3],
///         &[1, 2, 4, 5],
///         &[1, 3, 2, 4],
///         &[1, 2, 5, 3],
///         &[1, 3, 4, 2],
///         &[1, 3, 4, 5],
///         &[1, 4, 2, 3],
///         &[1, 3, 2, 5],
///         &[1, 4, 3, 2],
///         &[1, 2, 5, 4],
///         &[2, 1, 3, 4],
///         &[1, 3, 5, 2],
///         &[2, 1, 4, 3],
///         &[2, 3, 4, 5],
///         &[2, 3, 1, 4],
///         &[1, 5, 2, 3],
///         &[2, 3, 4, 1],
///         &[1, 4, 2, 5]
///     ]
/// );
/// ```
pub fn exhaustive_unique_vecs_fixed_length<I: Iterator>(
    k: u64,
    xs: I,
) -> ExhaustiveUniqueVecsFixedLength<I>
where
    I::Item: Clone,
{
    match k {
        0 => ExhaustiveUniqueVecsFixedLength::Zero(false),
        1 => ExhaustiveUniqueVecsFixedLength::One(xs),
        2 => ExhaustiveUniqueVecsFixedLength::Two(exhaustive_unique_vecs_2(xs)),
        k => ExhaustiveUniqueVecsFixedLength::GreaterThanTwo(exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_ordered_unique_vecs_fixed_length(k, xs),
            ExhaustiveUniqueVecsGenerator::new(),
        )),
    }
}

/// Generates all [`Vec`]s of elements from an iterator, where the [`Vec`]s have no repetitions and
/// are ordered the same way as in the iterator.
#[derive(Clone)]
pub struct ExhaustiveUniqueVecs<I: Iterator>
where
    I::Item: Clone,
{
    xs: ExhaustiveDependentPairs<
        Vec<I::Item>,
        Vec<I::Item>,
        RulerSequence<usize>,
        ExhaustiveUniqueVecsGenerator<I::Item, I>,
        ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>,
        ExhaustiveVecPermutations<I::Item>,
    >,
}

impl<I: Iterator> Iterator for ExhaustiveUniqueVecs<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        self.xs.next().map(|p| p.1)
    }
}

/// Generates [`Vec`]s with elements from a single iterator, such that each [`Vec`] has no repeated
/// elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_ {k=0}^n \frac{n!}{k!}
/// $$
/// $$
/// = \\begin{cases}
///     1 & \text{if} \\quad n = 0, \\\\
///     2 & \text{if} \\quad n = 1, \\\\
///     \operatorname{round}(en!) & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// See <https://oeis.org/A000522>.
///
/// If `xs` is empty, the output consists of a single empty [`Vec`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_unique_vecs;
///
/// let xss = exhaustive_unique_vecs(1..=4).take(20).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[1],
///         &[2],
///         &[3],
///         &[1, 2],
///         &[1, 3],
///         &[2, 1],
///         &[1, 2, 3],
///         &[3, 1],
///         &[2, 3],
///         &[3, 2],
///         &[4],
///         &[1, 3, 2],
///         &[1, 4],
///         &[2, 1, 3],
///         &[3, 4],
///         &[2, 3, 1],
///         &[4, 1],
///         &[3, 1, 2],
///         &[2, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_unique_vecs<I: Iterator>(xs: I) -> ExhaustiveUniqueVecs<I>
where
    I::Item: Clone,
{
    ExhaustiveUniqueVecs {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_ordered_unique_vecs(xs),
            ExhaustiveUniqueVecsGenerator::new(),
        ),
    }
}

/// Generates [`Vec`]s with a mininum length, with elements from a single iterator, such that each
/// [`Vec`] has no repeated elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$ and the `min_length` is $\ell$, the output length is
/// $$
/// \sum_ {k=\ell}^n \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_unique_vecs_min_length;
///
/// let xss = exhaustive_unique_vecs_min_length(2, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[2, 1],
///         &[2, 3],
///         &[3, 1],
///         &[3, 2],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2],
///         &[1, 4],
///         &[2, 1, 3],
///         &[2, 4],
///         &[2, 3, 1],
///         &[4, 1],
///         &[3, 1, 2],
///         &[3, 4],
///         &[3, 2, 1],
///         &[4, 2],
///         &[1, 4, 2],
///         &[1, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_unique_vecs_min_length<I: Iterator>(
    min_length: u64,
    xs: I,
) -> ExhaustiveUniqueVecs<I>
where
    I::Item: Clone,
{
    ExhaustiveUniqueVecs {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_ordered_unique_vecs_min_length(min_length, xs),
            ExhaustiveUniqueVecsGenerator::new(),
        ),
    }
}

/// Generates [`Vec`]s, with lengths in a range $[a, b)$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $a \leq b$, the output is empty.
///
/// If $a = 0$ and $b = 1$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a < b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^{b - 1} \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_unique_vecs_length_range;
///
/// let xss = exhaustive_unique_vecs_length_range(2, 4, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[2, 1],
///         &[2, 3],
///         &[3, 1],
///         &[3, 2],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2],
///         &[1, 4],
///         &[2, 1, 3],
///         &[2, 4],
///         &[2, 3, 1],
///         &[4, 1],
///         &[3, 1, 2],
///         &[3, 4],
///         &[3, 2, 1],
///         &[4, 2],
///         &[1, 4, 2],
///         &[1, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_unique_vecs_length_range<I: Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveUniqueVecs<I>
where
    I::Item: Clone,
{
    ExhaustiveUniqueVecs {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_ordered_unique_vecs_length_range(a, b, xs),
            ExhaustiveUniqueVecsGenerator::new(),
        ),
    }
}

/// Generates [`Vec`]s, with lengths in a range $[a, b]$, with elements from a single iterator, such
/// that each [`Vec`] has no repeated elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If $a < b$, the output is empty.
///
/// If $a = b = 0$, the output consists of a single empty [`Vec`].
///
/// If the input iterator is infinite and $0 < a \leq b$, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is
/// $$
/// \sum_{i=a}^b \frac{n!}{k!}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_unique_vecs_length_inclusive_range;
///
/// let xss = exhaustive_unique_vecs_length_inclusive_range(2, 3, 1..=4)
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 2][..],
///         &[1, 3],
///         &[2, 1],
///         &[2, 3],
///         &[3, 1],
///         &[3, 2],
///         &[1, 2, 3],
///         &[1, 2, 4],
///         &[1, 3, 2],
///         &[1, 4],
///         &[2, 1, 3],
///         &[2, 4],
///         &[2, 3, 1],
///         &[4, 1],
///         &[3, 1, 2],
///         &[3, 4],
///         &[3, 2, 1],
///         &[4, 2],
///         &[1, 4, 2],
///         &[1, 3, 4]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_unique_vecs_length_inclusive_range<I: Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveUniqueVecs<I>
where
    I::Item: Clone,
{
    ExhaustiveUniqueVecs {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            exhaustive_ordered_unique_vecs_length_inclusive_range(a, b, xs),
            ExhaustiveUniqueVecsGenerator::new(),
        ),
    }
}

/// Generates all $k$-compositions of a number: all length-$k$ [`Vec`]s of positive [`usize`]s whose
/// sum is a given number.
#[derive(Clone, Debug)]
pub struct LexKCompositions {
    done: bool,
    first: bool,
    xs: Vec<usize>,
}

impl Iterator for LexKCompositions {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        } else if self.first {
            self.first = false;
            return Some(self.xs.clone());
        }
        let last_not_one_index = self.xs.iter().rposition(|&x| x != 1);
        if last_not_one_index.is_none() || last_not_one_index == Some(0) {
            self.done = true;
            return None;
        }
        let last_not_one_index = last_not_one_index.unwrap();
        self.xs[last_not_one_index - 1] += 1;
        let last_not_one = self.xs[last_not_one_index];
        let (last, init) = self.xs.split_last_mut().unwrap();
        *last = last_not_one - 1;
        for x in &mut init[last_not_one_index..] {
            *x = 1;
        }
        Some(self.xs.clone())
    }
}

/// Generates all $k$-compositions of a number: given $n$ and $k$, generates all length-$k$ [`Vec`]s
/// of positive [`usize`]s whose sum is $n$.
///
/// The [`Vec`]s are output in lexicographic order.
///
/// If $k = 0$ and $n \neq 0$, or if $n < k$, then the output is empty.
///
/// The output length is
/// $$
/// \binom{n-1}{k-1}.
/// $$
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::lex_k_compositions;
///
/// let xss = lex_k_compositions(5, 3).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[&[1, 1, 3], &[1, 2, 2], &[1, 3, 1], &[2, 1, 2], &[2, 2, 1], &[3, 1, 1]]
/// );
/// ```
pub fn lex_k_compositions(n: usize, k: usize) -> LexKCompositions {
    if k == 0 && n != 0 || n < k {
        return LexKCompositions {
            done: true,
            first: true,
            xs: Vec::new(),
        };
    }
    let mut xs = vec![1; k];
    if k != 0 {
        xs[k - 1] = n + 1 - k;
    }
    LexKCompositions {
        done: false,
        first: true,
        xs,
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct LexKCompositionsGenerator {
    k: usize,
}

impl ExhaustiveDependentPairsYsGenerator<usize, Vec<usize>, LexKCompositions>
    for LexKCompositionsGenerator
{
    #[inline]
    fn get_ys(&self, &n: &usize) -> LexKCompositions {
        lex_k_compositions(n, self.k)
    }
}

/// Generates $k$-compositions of $n$ for all $n$ in a given range: all length-$k$ [`Vec`]s of
/// positive [`usize`]s whose sum is in a given range.
#[derive(Clone, Debug)]
pub struct ExhaustiveCombinedKCompositions {
    xs: ExhaustiveDependentPairs<
        usize,
        Vec<usize>,
        RulerSequence<usize>,
        LexKCompositionsGenerator,
        PrimitiveIntIncreasingRange<usize>,
        LexKCompositions,
    >,
}

impl Iterator for ExhaustiveCombinedKCompositions {
    type Item = Vec<usize>;

    #[inline]
    fn next(&mut self) -> Option<Vec<usize>> {
        self.xs.next().map(|p| p.1)
    }
}

/// Given $n_\text{min}$, $n_\text{max}$, and $k$, generates all length-$k$ [`Vec`]s of positive
/// [`usize`]s whose sum is in the closed interval $[n_\text{min}, n_\text{max}]$.
///
/// The output length is
/// $$
/// \sum_{n=n_\text{min}}^{n_\text{max}} \binom{n-1}{k-1}.
/// $$
///
/// # Panics
/// Panics if $n_\text{min} > n_\text{max}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::vecs::exhaustive::exhaustive_combined_k_compositions;
///
/// let xss = exhaustive_combined_k_compositions(4, 6, 3).collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[1, 1, 2],
///         &[1, 1, 3],
///         &[1, 2, 1],
///         &[1, 1, 4],
///         &[2, 1, 1],
///         &[1, 2, 2],
///         &[1, 3, 1],
///         &[1, 2, 3],
///         &[2, 1, 2],
///         &[1, 3, 2],
///         &[2, 2, 1],
///         &[3, 1, 1],
///         &[1, 4, 1],
///         &[2, 1, 3],
///         &[2, 2, 2],
///         &[2, 3, 1],
///         &[3, 1, 2],
///         &[3, 2, 1],
///         &[4, 1, 1]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_combined_k_compositions(
    n_min: usize,
    n_max: usize,
    k: usize,
) -> ExhaustiveCombinedKCompositions {
    ExhaustiveCombinedKCompositions {
        xs: exhaustive_dependent_pairs(
            ruler_sequence(),
            primitive_int_increasing_inclusive_range(n_min, n_max),
            LexKCompositionsGenerator { k },
        ),
    }
}
