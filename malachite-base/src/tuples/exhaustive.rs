// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use crate::iterators::iterator_cache::IteratorCache;
use crate::num::arithmetic::traits::CheckedPow;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::logic::traits::SignificantBits;
use crate::vecs::exhaustive::{
    fixed_length_ordered_unique_indices_helper, next_bit_pattern, unique_indices, UniqueIndices,
};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::max;
use core::fmt::Debug;
use core::iter::{once, Once};
use core::marker::PhantomData;
use core::mem::take;

/// Generates the only unit: `()`.
///
/// The output length is 1.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::exhaustive_units;
///
/// assert_eq!(exhaustive_units().collect_vec(), &[()]);
/// ```
pub fn exhaustive_units() -> Once<()> {
    once(())
}

// hack for macro
pub_test! {
#[inline]
clone_helper<T: Clone>(x: &T, _i: usize) -> T {
    x.clone()
}}

/// Defines lexicographic tuple generators.
///
/// Malachite provides [`lex_pairs`] and [`lex_pairs_from_single`], but you can also define
/// `lex_triples`, `lex_quadruples`, and so on, and `lex_triples_from_single`,
/// `lex_quadruples_from_single`, and so on, in your program using the code below. The documentation
/// for [`lex_pairs`] and [`lex_pairs_from_single`] describes these other functions as well.
///
/// See usage examples [here](self#lex_pairs) and [here](self#lex_pairs_from_single).
///
/// ```
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_tuples;
///
/// fn clone_helper<T: Clone>(x: &T, _i: usize) -> T {
///     x.clone()
/// }
///
/// lex_tuples!(
///     (pub(crate)),
///     3,
///     LexTriples,
///     LexTriplesFromSingle,
///     lex_triples,
///     lex_triples_from_single,
///     (T, T, T),
///     [0, X, I, xs, x],
///     [1, Y, J, ys, y],
///     [2, Z, K, zs, z]
/// );
/// lex_tuples!(
///     (pub(crate)),
///     4,
///     LexQuadruples,
///     LexQuadruplesFromSingle,
///     lex_quadruples,
///     lex_quadruples_from_single,
///     (T, T, T, T),
///     [0, X, I, xs, x],
///     [1, Y, J, ys, y],
///     [2, Z, K, zs, z],
///     [3, W, L, ws, w]
/// );
/// lex_tuples!(
///     (pub(crate)),
///     5,
///     LexQuintuples,
///     LexQuintuplesFromSingle,
///     lex_quintuples,
///     lex_quintuples_from_single,
///     (T, T, T, T, T),
///     [0, X, I, xs, x],
///     [1, Y, J, ys, y],
///     [2, Z, K, zs, z],
///     [3, W, L, ws, w],
///     [4, V, M, vs, v]
/// );
/// lex_tuples!(
///     (pub(crate)),
///     6,
///     LexSextuples,
///     LexSextuplesFromSingle,
///     lex_sextuples,
///     lex_sextuples_from_single,
///     (T, T, T, T, T, T),
///     [0, X, I, xs, x],
///     [1, Y, J, ys, y],
///     [2, Z, K, zs, z],
///     [3, W, L, ws, w],
///     [4, V, M, vs, v],
///     [5, U, N, us, u]
/// );
/// lex_tuples!(
///     (pub(crate)),
///     7,
///     LexSeptuples,
///     LexSeptuplesFromSingle,
///     lex_septuples,
///     lex_septuples_from_single,
///     (T, T, T, T, T, T, T),
///     [0, X, I, xs, x],
///     [1, Y, J, ys, y],
///     [2, Z, K, zs, z],
///     [3, W, L, ws, w],
///     [4, V, M, vs, v],
///     [5, U, N, us, u],
///     [6, T, O, ts, t]
/// );
/// lex_tuples!(
///     (pub(crate)),
///     8,
///     LexOctuples,
///     LexOctuplesFromSingle,
///     lex_octuples,
///     lex_octuples_from_single,
///     (T, T, T, T, T, T, T, T),
///     [0, X, I, xs, x],
///     [1, Y, J, ys, y],
///     [2, Z, K, zs, z],
///     [3, W, L, ws, w],
///     [4, V, M, vs, v],
///     [5, U, N, us, u],
///     [6, T, O, ts, t],
///     [7, S, P, ss, s]
/// );
/// ```
#[macro_export]
macro_rules! lex_tuples {
    (
        ($($vis:tt)*),
        $k: expr,
        $exhaustive_struct: ident,
        $exhaustive_struct_from_single: ident,
        $exhaustive_fn: ident,
        $exhaustive_fn_from_single: ident,
        $single_out: tt,
        $([$i: expr, $t: ident, $it: ident, $xs: ident, $x:ident]),*
    ) => {
        /// This documentation applies not only to `LexPairs`, but also to `LexTriples`,
        /// `LexQuadruples`, and so on. See `lex_tuples` for more information.
        ///
        /// Generates all $n$-tuples with elements from $n$ iterators, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            first: bool,
            done: bool,
            $($xs: IteratorCache<$it>,)*
            counters: [usize; $k],
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> $exhaustive_struct<$($t, $it,)*> {
            fn increment_counters(&mut self) -> bool {
                for (i, counter) in self.counters.iter_mut().enumerate().rev() {
                    *counter += 1;
                    let no_carry = match i {
                        $(
                            $i => self.$xs.get(*counter).is_some(),
                        )*
                        _ => unreachable!(),
                    };
                    if no_carry {
                        return false;
                    } else if i == 0 {
                        return true;
                    }
                    *counter = 0;
                }
                false
            }
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $exhaustive_struct<$($t, $it,)*>
        {
            type Item = ($($t,)*);

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    None
                } else if self.first {
                    self.first = false;
                    $(
                        let $x;
                    )*
                    $(
                        if let Some(x) = self.$xs.get(0) {
                            $x = x.clone();
                        } else {
                            self.done = true;
                            return None;
                        }
                    )*
                    Some(($($x,)*))
                } else if self.increment_counters() {
                    self.done = true;
                    None
                } else {
                    Some(($(self.$xs.get(self.counters[$i]).unwrap().clone(),)*))
                }
            }
        }

        /// This documentation applies not only to `lex_pairs`, but also to `lex_triples`,
        /// `lex_quadruples`, and so on. See [`lex_tuples`] for more information.
        ///
        /// Generates all $n$-tuples with elements from $n$ iterators, in lexicographic order.
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
        /// See [here](self#lex_pairs).
        #[allow(dead_code)]
        $($vis)* const fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_struct {
                first: true,
                done: false,
                $($xs: IteratorCache::new($xs),)*
                counters: [$($i * 0,)*],
            }
        }

        /// This documentation applies not only to `LexPairsFromSingle`, but also to
        /// `LexTriplesFromSingle`, `LexQuadruplesFromSingle`, and so on. See [`lex_tuples`] for
        /// more information.
        ///
        /// Generates all $n$-tuples with elements a single iterator, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterator.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct_from_single<T: Clone, I: Iterator<Item = T>> {
            first: bool,
            done: bool,
            xs: IteratorCache<I>,
            counters: [usize; $k],
        }

        impl<T: Clone, I: Iterator<Item = T>> $exhaustive_struct_from_single<T, I> {
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

        impl<T: Clone, I: Iterator<Item = T>> Iterator for $exhaustive_struct_from_single<T, I> {
            type Item = $single_out;

            fn next(&mut self) -> Option<$single_out> {
                if self.done {
                    None
                } else if self.first {
                    self.first = false;
                    if let Some(x) = self.xs.get(0) {
                        Some(($(clone_helper(x, $i),)*))
                    } else {
                        self.done = true;
                        None
                    }
                } else if self.increment_counters() {
                    self.done = true;
                    None
                } else {
                    Some(($(self.xs.get(self.counters[$i]).unwrap().clone(),)*))
                }
            }
        }

        /// This documentation applies not only to `lex_pairs_from_single`, but also to
        /// `lex_triples_from_single`, `lex_quadruples_from_single`, and so on. See [`lex_tuples`]
        /// for more information.
        ///
        /// Generates all $n$-tuples with elements from a single iterator, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterator.
        ///
        /// `xs` must be finite.
        ///
        /// The output length is $k^n$, where $k$ is `xs.count()` and $n$ is `len`.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#lex_pairs_from_single).
        #[allow(dead_code)]
        $($vis)* const fn $exhaustive_fn_from_single<T: Clone, I: Iterator<Item = T>>(
            xs: I
        ) -> $exhaustive_struct_from_single<T, I> {
            $exhaustive_struct_from_single {
                first: true,
                done: false,
                xs: IteratorCache::new(xs),
                counters: [$($i * 0,)*],
            }
        }
    }
}
lex_tuples!(
    (pub),
    2,
    LexPairs,
    LexPairsFromSingle,
    lex_pairs,
    lex_pairs_from_single,
    (T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y]
);
lex_tuples!(
    (pub(crate)),
    4,
    LexQuadruples,
    LexQuadruplesFromSingle,
    lex_quadruples,
    lex_quadruples_from_single,
    (T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w]
);

/// Defines custom lexicographic tuple generators.
///
/// You can define custom tuple generators like `lex_triples_xxy` in your program using the code
/// below.
///
/// See usage examples [here](self#lex_triples_xxy).
///
/// ```
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_custom_tuples;
///
/// fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
///     (a.unwrap(), b.unwrap(), c.unwrap())
/// }
///
/// lex_custom_tuples! {
///     (pub),
///     LexTriplesXXY,
///     (X, X, Y),
///     (None, None, None),
///     unwrap_triple,
///     lex_triples_xxy,
///     [X, I, xs, [0, x_0], [1, x_1]],
///     [Y, J, ys, [2, y_2]]
/// }
/// lex_custom_tuples!(
///     (pub),
///     LexTriplesXYX,
///     (X, Y, X),
///     (None, None, None),
///     unwrap_triple,
///     lex_triples_xyx,
///     [X, I, xs, [0, x_0], [2, x_2]],
///     [Y, J, ys, [1, y_1]]
/// );
/// lex_custom_tuples!(
///     (pub),
///     LexTriplesXYY,
///     (X, Y, Y),
///     (None, None, None),
///     unwrap_triple,
///     lex_triples_xyy,
///     [X, I, xs, [0, x_0]],
///     [Y, J, ys, [1, y_1], [2, y_2]]
/// );
/// ```
#[macro_export]
macro_rules! lex_custom_tuples {
    (
        ($($vis:tt)*),
        $exhaustive_struct: ident,
        $out_t: ty,
        $nones: expr,
        $unwrap_tuple: ident,
        $exhaustive_fn: ident,
        $([$t: ident, $it: ident, $xs: ident, $([$i: tt, $x: ident]),*]),*
    ) => {
        // Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$, in
        // lexicographic order.
        //
        // The order is lexicographic with respect to the order of the element iterators.
        //
        // The mapping from iterators to tuple slots is indicated by the struct name; for example,
        // in `LexTriplesXYX` there are two iterators, `X`, and `Y`; `X` generates the elements in
        // the first and third slots of the output triples, and `Y` generates the elements in the
        // second slots.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            first: bool,
            done: bool,
            $($xs: IteratorCache<$it>,)*
            counters: Vec<usize>,
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> $exhaustive_struct<$($t, $it,)*> {
            fn increment_counters(&mut self) -> bool {
                for (i, counter) in self.counters.iter_mut().enumerate().rev() {
                    *counter += 1;
                    let no_carry = match i {
                        $(
                            $($i)|* => self.$xs.get(*counter).is_some(),
                        )*
                        _ => unreachable!(),
                    };
                    if no_carry {
                        return false;
                    } else if i == 0 {
                        return true;
                    }
                    *counter = 0;
                }
                false
            }
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $exhaustive_struct<$($t, $it,)*>
        {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    None
                } else if self.first {
                    self.first = false;
                    $($(let $x;)*)*
                    $(
                        if let Some(x) = self.$xs.get(0) {
                            $($x = x.clone();)*
                        } else {
                            self.done = true;
                            return None;
                        }
                    )*
                    let mut out = $nones;
                    $($(out.$i = Some($x);)*)*
                    Some($unwrap_tuple(out))
                } else if self.increment_counters() {
                    self.done = true;
                    None
                } else {
                    let mut out = $nones;
                    $($(out.$i = self.$xs.get(self.counters[$i]).cloned();)*)*
                    Some($unwrap_tuple(out))
                }
            }
        }

        // Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$, in
        // lexicographic order.
        //
        // The order is lexicographic with respect to the order of the element iterators.
        //
        // The mapping from iterators to tuple slots is indicated by the function name; for example,
        // `lex_triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the elements in the
        // first and third slots of the output triples, and `ys` generates the elements in the
        // second slots.
        //
        // Let `xs` be the input iterator mapped to the first slot of the output tuples. All the
        // input iterators, except possibly `xs`, must be finite. If `xs` is finite, the output
        // length is the product of the lengths of all the input iterators. If `xs` is infinite, the
        // output is also infinite.
        //
        // If any of the input iterators is empty, the output is also empty.
        //
        // # Examples
        // See [here](self#lex_triples_xyx).
        $($vis)* fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_struct {
                first: true,
                done: false,
                $($xs: IteratorCache::new($xs),)*
                counters: vec![$($(($i * 0),)*)*],
            }
        }
    }
}

/// Defines exhaustive tuple generators that generate tuples from a single iterator.
///
/// Malachite provides [`exhaustive_pairs_from_single`] and [`exhaustive_pairs_1_input`], but you
/// can also define `exhaustive_triples_from_single`, `exhaustive_quadruples_from_single`, and so
/// on, and `exhaustive_triples_1_input`, `exhaustive_quadruples_1_input`, and so on, in your
/// program using the code below. The documentation for [`exhaustive_pairs_from_single`] and
/// [`exhaustive_pairs_1_input`] describes these other functions as well.
///
/// See usage examples [here](self#exhaustive_pairs_from_single) and
/// [here](self#exhaustive_pairs_1_input).
///
/// ```
/// use malachite_base::exhaustive_tuples_1_input;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::arithmetic::traits::CheckedPow;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use std::cmp::max;
/// use std::marker::PhantomData;
///
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveTriples1Input,
///     exhaustive_triples_1_input,
///     exhaustive_triples_from_single,
///     (I::Item, I::Item, I::Item),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z]
/// );
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveQuadruples1Input,
///     exhaustive_quadruples_1_input,
///     exhaustive_quadruples_from_single,
///     (I::Item, I::Item, I::Item, I::Item),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z],
///     [3, output_type_w]
/// );
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveQuintuples1Input,
///     exhaustive_quintuples_1_input,
///     exhaustive_quintuples_from_single,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z],
///     [3, output_type_w],
///     [4, output_type_v]
/// );
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveSextuples1Input,
///     exhaustive_sextuples_1_input,
///     exhaustive_sextuples_from_single,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z],
///     [3, output_type_w],
///     [4, output_type_v],
///     [5, output_type_u]
/// );
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveSeptuples1Input,
///     exhaustive_septuples_1_input,
///     exhaustive_septuples_from_single,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z],
///     [3, output_type_w],
///     [4, output_type_v],
///     [5, output_type_u],
///     [6, output_type_t]
/// );
/// exhaustive_tuples_1_input!(
///     (pub(crate)),
///     ExhaustiveOctuples1Input,
///     exhaustive_octuples_1_input,
///     exhaustive_octuples_from_single,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     [0, output_type_x],
///     [1, output_type_y],
///     [2, output_type_z],
///     [3, output_type_w],
///     [4, output_type_v],
///     [5, output_type_u],
///     [6, output_type_t],
///     [7, output_type_s]
/// );
/// ```
#[macro_export]
macro_rules! exhaustive_tuples_1_input {
    (
        ($($vis:tt)*),
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $exhaustive_fn_from_single: ident,
        $out_type: ty,
        $([$i: tt, $out_x: ident]),*
    ) => {
        /// This documentation applies not only to `ExhaustivePairs1Input`, but also to
        /// `ExhaustiveTriples1Input`, `ExhaustiveQuadruples1Input`, and so on. See
        /// [`exhaustive_tuples_1_input`] for more information.
        ///
        /// Generates all $n$-tuples of a given length with elements from a single iterator.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<I: Iterator>
        where
            I::Item: Clone,
        {
            i: u64,
            limit: Option<u64>,
            distributor: BitDistributor,
            xs: IteratorCache<I>,
            xs_done: bool,
            phantom: PhantomData<*const I::Item>,
        }

        impl<I: Iterator> Iterator for $exhaustive_struct<I>
        where
            I::Item: Clone,
        {
            type Item = $out_type;

            fn next(&mut self) -> Option<Self::Item> {
                if Some(self.i) == self.limit {
                    None
                } else {
                    if self.i == u64::MAX {
                        panic!("Too many iterations");
                    }
                    loop {
                        let mut all_are_valid = true;
                        $(
                            if all_are_valid &&
                                    self.xs.get(self.distributor.get_output($i)).is_none() {
                                all_are_valid = false;
                            }
                        )*
                        if all_are_valid {
                            break;
                        } else if !self.xs_done {
                            let xs_len = self.xs.known_len().unwrap();
                            $(
                                let _max_index = $i;
                            )*
                            let size = _max_index + 1;
                            self.limit = CheckedPow::checked_pow(
                                u64::exact_from(xs_len),
                                u64::exact_from(size)
                            );
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
                    let result = Some(
                        ($(self.xs.get(self.distributor.get_output($i)).unwrap().clone(),)*)
                    );
                    self.i += 1;
                    self.distributor.increment_counter();
                    result
                }
            }
        }

        /// This documentation applies not only to `exhaustive_pairs_1_input`, but also to
        /// `exhaustive_triples_1_input`, `exhaustive_quadruples_1_input`, and so on. See
        /// [`exhaustive_tuples_1_input`] for more information.
        ///
        /// Generates all length-$n$ tuples with elements from a single iterator.
        ///
        /// These functions differ from `exhaustive_[n-tuples]_from_single` in that different
        /// [`BitDistributorOutputType`]s may be specified for each output element.
        ///
        /// The $i$th parameter `output_types_[x_i]` is a [`BitDistributorOutputType`] that
        /// determines how quickly the $i$th output slot advances through the iterator; see the
        /// [`BitDistributor`] documentation for a description of the different types.
        ///
        /// If `xs` is finite, the output length is $k^n$, where $k$ is `xs.count()` and $n$ is the
        /// width of the tuples. If `xs` is infinite, the output is also infinite.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_pairs_1_input).
        #[allow(dead_code)]
        $($vis)* fn $exhaustive_fn<I: Iterator>(
            xs: I,
            $($out_x: BitDistributorOutputType,)*
        ) -> $exhaustive_struct<I>
        where
            I::Item: Clone,
        {
            $exhaustive_struct {
                i: 0,
                limit: None,
                distributor: BitDistributor::new(&[$($out_x,)*]),
                xs: IteratorCache::new(xs),
                xs_done: false,
                phantom: PhantomData,
            }
        }

        /// This documentation applies not only to `exhaustive_pairs_from_single`, but also to
        /// `exhaustive_triples_from_single`, `exhaustive_quadruples_from_single`, and so on. See
        /// [`exhaustive_tuples_1_input`] for more information.
        ///
        /// Generates all $n$-tuples with elements from a single iterator.
        ///
        /// If `xs` is finite, the output length is $k^n$, where $k$ is `xs.count()` and $n$ is the
        /// width of the tuples. If `xs` is infinite, the output is also infinite.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_pairs_from_single).
        #[allow(dead_code)]
        #[inline]
        $($vis)* fn $exhaustive_fn_from_single<I: Iterator>(xs: I) -> $exhaustive_struct<I>
        where
            I::Item: Clone,
        {
            $exhaustive_fn(xs, $(BitDistributorOutputType::normal(1 + $i * 0)),*)
        }
    }
}
exhaustive_tuples_1_input!(
    (pub),
    ExhaustivePairs1Input,
    exhaustive_pairs_1_input,
    exhaustive_pairs_from_single,
    (I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y]
);

/// Defines exhaustive tuple generators.
///
/// Malachite provides [`exhaustive_pairs`] and [`exhaustive_pairs_custom_output`], but you can also
/// define `exhaustive_triples`, `exhaustive_quadruples`, and so on, and
/// `exhaustive_triples_custom_output`, `exhaustive_quadruples_custom_output`, and so on, in your
/// program using the code below. The documentation for [`exhaustive_pairs`] and
/// [`exhaustive_pairs_custom_output`] describes these other functions as well.
///
/// See usage examples [here](self#exhaustive_pairs) and
/// [here](self#exhaustive_pairs_custom_output).
///
/// ```
/// use malachite_base::exhaustive_tuples;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use std::cmp::max;
///
/// exhaustive_tuples!(
///     (pub(crate)),
///     ExhaustiveTriples,
///     exhaustive_triples,
///     exhaustive_triples_custom_output,
///     [0, X, I, xs, xs_done, output_type_x],
///     [1, Y, J, ys, ys_done, output_type_y],
///     [2, Z, K, zs, zs_done, output_type_z]
/// );
/// exhaustive_tuples!(
///     (pub(crate)),
///     ExhaustiveQuadruples,
///     exhaustive_quadruples,
///     exhaustive_quadruples_custom_output,
///     [0, X, I, xs, xs_done, output_type_x],
///     [1, Y, J, ys, ys_done, output_type_y],
///     [2, Z, K, zs, zs_done, output_type_z],
///     [3, W, L, ws, ws_done, output_type_w]
/// );
/// exhaustive_tuples!(
///     (pub(crate)),
///     ExhaustiveQuintuples,
///     exhaustive_quintuples,
///     exhaustive_quintuples_custom_output,
///     [0, X, I, xs, xs_done, output_type_x],
///     [1, Y, J, ys, ys_done, output_type_y],
///     [2, Z, K, zs, zs_done, output_type_z],
///     [3, W, L, ws, ws_done, output_type_w],
///     [4, V, M, vs, vs_done, output_type_v]
/// );
/// exhaustive_tuples!(
///     (pub(crate)),
///     ExhaustiveSextuples,
///     exhaustive_sextuples,
///     exhaustive_sextuples_custom_output,
///     [0, X, I, xs, xs_done, output_type_x],
///     [1, Y, J, ys, ys_done, output_type_y],
///     [2, Z, K, zs, zs_done, output_type_z],
///     [3, W, L, ws, ws_done, output_type_w],
///     [4, V, M, vs, vs_done, output_type_v],
///     [5, U, N, us, us_done, output_type_u]
/// );
/// exhaustive_tuples!(
///     (pub(crate)),
///     ExhaustiveSeptuples,
///     exhaustive_septuples,
///     exhaustive_septuples_custom_output,
///     [0, X, I, xs, xs_done, output_type_x],
///     [1, Y, J, ys, ys_done, output_type_y],
///     [2, Z, K, zs, zs_done, output_type_z],
///     [3, W, L, ws, ws_done, output_type_w],
///     [4, V, M, vs, vs_done, output_type_v],
///     [5, U, N, us, us_done, output_type_u],
///     [6, T, O, ts, ts_done, output_type_t]
/// );
/// exhaustive_tuples!(
///     (pub(crate)),
///     ExhaustiveOctuples,
///     exhaustive_octuples,
///     exhaustive_octuples_custom_output,
///     [0, X, I, xs, xs_done, output_type_x],
///     [1, Y, J, ys, ys_done, output_type_y],
///     [2, Z, K, zs, zs_done, output_type_z],
///     [3, W, L, ws, ws_done, output_type_w],
///     [4, V, M, vs, vs_done, output_type_v],
///     [5, U, N, us, us_done, output_type_u],
///     [6, T, O, ts, ts_done, output_type_t],
///     [7, S, P, ss, ss_done, output_type_s]
/// );
/// ```
#[macro_export]
macro_rules! exhaustive_tuples {
    (
        ($($vis:tt)*),
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $exhaustive_fn_custom_output: ident,
        $([$i: tt, $t: ident, $it: ident, $xs: ident, $xs_done: ident, $out_x: ident]),*
    ) => {
        /// This documentation applies not only to `ExhaustivePairs`, but also to
        /// `ExhaustiveTriples`, `ExhaustiveQuadruples`, and so on. See [`exhaustive_tuples`] for
        /// more information.
        ///
        /// Generates all $n$-tuples with elements from $n$ iterators.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            i: u64,
            limit: Option<u64>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
            )*
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> $exhaustive_struct<$($t, $it,)*> {
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
                    if let Some(new_product) = product.checked_mul(u64::exact_from(xs_len)) {
                        product = new_product;
                    } else {
                        return;
                    }
                )*
                self.limit = Some(product);
            }
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $exhaustive_struct<$($t, $it,)*>
        {
            type Item = ($($t,)*);

            fn next(&mut self) -> Option<Self::Item> {
                if Some(self.i) == self.limit {
                    None
                } else {
                    if self.i == u64::MAX {
                        panic!("Too many iterations");
                    }
                    loop {
                        $(
                            if self.$xs.get(self.distributor.get_output($i)).is_none() {
                                if !self.$xs_done {
                                    let xs_len = self.$xs.known_len().unwrap();
                                    self.try_getting_limit();
                                    if Some(self.i) == self.limit {
                                        return None;
                                    }
                                    self.$xs_done = true;
                                    self.distributor.set_max_bits(
                                        &[$i],
                                        max(
                                            1,
                                            usize::wrapping_from((xs_len - 1).significant_bits())
                                        ),
                                    );
                                } else {
                                    self.distributor.increment_counter();
                                }
                                continue;
                            }
                        )*
                        break;
                    }
                    let result = Some(
                        ($(self.$xs.get(self.distributor.get_output($i)).unwrap().clone(),)*)
                    );
                    self.i += 1;
                    self.distributor.increment_counter();
                    result
                }
            }
        }

        /// This documentation applies not only to `exhaustive_pairs_custom_output`, but also to
        /// `exhaustive_triples_custom_output`, `exhaustive_quadruples_custom_output`, and so on.
        /// See [`exhaustive_tuples`] for more information.
        ///
        /// Generates all $n$-tuples with elements from $n$ iterators, possibly with different
        /// output growth rates.
        ///
        /// The $i$th `output_type_[x_i]` parameter is a [`BitDistributorOutputType`] that
        /// determines how quickly the $i$th output slot advances through its iterator; see the
        /// [`BitDistributor`] documentation for a description of the different types.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_pairs_custom_output).
        #[allow(dead_code)]
        $($vis)* fn $exhaustive_fn_custom_output<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
            $($out_x: BitDistributorOutputType,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_struct {
                i: 0,
                limit: None,
                distributor: BitDistributor::new(&[$($out_x,)*]),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                )*
            }
        }

        /// This documentation applies not only to `exhaustive_pairs`, but also to
        /// `exhaustive_triples`, `exhaustive_quadruples`, and so on. See [`exhaustive_tuples`] for
        /// more information.
        ///
        /// Generates all $n$-tuples with elements from $n$ iterators.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_pairs).
        #[allow(dead_code)]
        #[inline]
        $($vis)* fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_fn_custom_output(
                $($xs,)*
                $(BitDistributorOutputType::normal(1 + 0 * $i),)*
            )
        }
    }
}

exhaustive_tuples!(
    (pub),
    ExhaustivePairs,
    exhaustive_pairs,
    exhaustive_pairs_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y]
);

#[cfg(feature = "test_build")]
exhaustive_tuples!(
    (pub),
    ExhaustiveTriples,
    exhaustive_triples,
    exhaustive_triples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z]
);

#[cfg(not(feature = "test_build"))]
exhaustive_tuples!(
    (pub(crate)),
    ExhaustiveTriples,
    exhaustive_triples,
    exhaustive_triples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z]
);

#[cfg(feature = "test_build")]
exhaustive_tuples!(
    (pub),
    ExhaustiveQuadruples,
    exhaustive_quadruples,
    exhaustive_quadruples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z],
    [3, W, L, ws, ws_done, output_type_w]
);

/// Defines custom exhaustive tuple generators.
///
/// You can define custom tuple generators like `exhaustive_triples_xyx` or
/// `exhaustive_triples_xyx_custom_output` in your program using the code below.
///
/// See usage examples [here](self#exhaustive_triples_xyx) and
/// [here](self#exhaustive_triples_xyx_custom_output).
///
/// ```
/// use malachite_base::custom_tuples;
/// use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
/// use malachite_base::num::logic::traits::SignificantBits;
/// use std::cmp::max;
///
/// #[allow(clippy::missing_const_for_fn)]
/// fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
///     (a.unwrap(), b.unwrap(), c.unwrap())
/// }
///
/// #[allow(clippy::missing_const_for_fn)]
/// fn unwrap_quadruple<X, Y, Z, W>(
///     (a, b, c, d): (Option<X>, Option<Y>, Option<Z>, Option<W>),
/// ) -> (X, Y, Z, W) {
///     (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap())
/// }
///
/// #[allow(clippy::missing_const_for_fn)]
/// fn unwrap_quintuple<X, Y, Z, W, V>(
///     (a, b, c, d, e): (Option<X>, Option<Y>, Option<Z>, Option<W>, Option<V>),
/// ) -> (X, Y, Z, W, V) {
///     (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap(), e.unwrap())
/// }
///
/// custom_tuples!(
///     (pub),
///     ExhaustiveTriplesXXY,
///     (X, X, Y),
///     (None, None, None),
///     unwrap_triple,
///     exhaustive_triples_xxy,
///     exhaustive_triples_xxy_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1]],
///     [Y, J, ys, ys_done, [2, output_type_ys_2]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveTriplesXYX,
///     (X, Y, X),
///     (None, None, None),
///     unwrap_triple,
///     exhaustive_triples_xyx,
///     exhaustive_triples_xyx_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [2, output_type_ys_1]],
///     [Y, J, ys, ys_done, [1, output_type_xs_2]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveTriplesXYY,
///     (X, Y, Y),
///     (None, None, None),
///     unwrap_triple,
///     exhaustive_triples_xyy,
///     exhaustive_triples_xyy_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0]],
///     [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXXXY,
///     (X, X, X, Y),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xxxy,
///     exhaustive_quadruples_xxxy_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1], [2, output_type_xs_2]],
///     [Y, J, ys, ys_done, [3, output_type_ys_3]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXXYX,
///     (X, X, Y, X),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xxyx,
///     exhaustive_quadruples_xxyx_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1], [3, output_type_xs_3]],
///     [Y, J, ys, ys_done, [2, output_type_ys_2]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXXYZ,
///     (X, X, Y, Z),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xxyz,
///     exhaustive_quadruples_xxyz_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1]],
///     [Y, J, ys, ys_done, [2, output_type_ys_2]],
///     [Z, K, zs, zs_done, [3, output_type_zs_3]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXYXZ,
///     (X, Y, X, Z),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xyxz,
///     exhaustive_quadruples_xyxz_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [2, output_type_xs_2]],
///     [Y, J, ys, ys_done, [1, output_type_ys_1]],
///     [Z, K, zs, zs_done, [3, output_type_zs_3]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXYYX,
///     (X, Y, Y, X),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xyyx,
///     exhaustive_quadruples_xyyx_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0], [3, output_type_xs_3]],
///     [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXYYZ,
///     (X, Y, Y, Z),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xyyz,
///     exhaustive_quadruples_xyyz_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0]],
///     [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2]],
///     [Z, K, zs, zs_done, [3, output_type_zs_3]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuadruplesXYZZ,
///     (X, Y, Z, Z),
///     (None, None, None, None),
///     unwrap_quadruple,
///     exhaustive_quadruples_xyzz,
///     exhaustive_quadruples_xyzz_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0]],
///     [Y, J, ys, ys_done, [1, output_type_ys_1]],
///     [Z, K, zs, zs_done, [2, output_type_zs_2], [3, output_type_zs_3]]
/// );
/// custom_tuples!(
///     (pub),
///     ExhaustiveQuintuplesXYYYZ,
///     (X, Y, Y, Y, Z),
///     (None, None, None, None, None),
///     unwrap_quintuple,
///     exhaustive_quintuples_xyyyz,
///     exhaustive_quintuples_xyyyz_custom_output,
///     [X, I, xs, xs_done, [0, output_type_xs_0]],
///     [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2], [3, output_type_ys_3]],
///     [Z, K, zs, zs_done, [4, output_type_zs_4]]
/// );
/// ```
#[macro_export]
macro_rules! custom_tuples {
    (
        ($($vis:tt)*),
        $exhaustive_struct: ident,
        $out_t: ty,
        $nones: expr,
        $unwrap_tuple: ident,
        $exhaustive_fn: ident,
        $exhaustive_custom_fn: ident,
        $([$t: ident, $it: ident, $xs: ident, $xs_done: ident, $([$i: tt, $out_x: ident]),*]),*
    ) => {
        // Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        //
        // The mapping from iterators to tuple slots is indicated by the struct name; for example,
        // in `TriplesXYX` there are two iterators, `X`, and `Y`; `X` generates the elements in the
        // first and third slots of the output triples, and `Y` generates the elements in the second
        // slots.
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            i: u64,
            limit: Option<u64>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
            )*
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> $exhaustive_struct<$($t, $it,)*> {
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
                    $(
                        let _x = $i;
                        if let Some(new_product) = product.checked_mul(xs_len) {
                            product = new_product;
                        } else {
                            return;
                        }
                    )*
                )*
                self.limit = Some(product);
            }
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator
            for $exhaustive_struct<$($t, $it,)*>
        {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                if Some(self.i) == self.limit {
                    None
                } else {
                    if self.i == u64::MAX {
                        panic!("Too many iterations");
                    }
                    loop {
                        $(
                            $(
                                if self.$xs.get(self.distributor.get_output($i)).is_none() {
                                    if !self.$xs_done {
                                        let xs_len = self.$xs.known_len().unwrap();
                                        self.try_getting_limit();
                                        if Some(self.i) == self.limit {
                                            return None;
                                        }
                                        self.$xs_done = true;
                                        self.distributor.set_max_bits(
                                            &[$i],
                                            max(
                                                1,
                                                usize::wrapping_from(
                                                    (xs_len - 1).significant_bits()
                                                )
                                            ),
                                        );
                                    } else {
                                        self.distributor.increment_counter();
                                    }
                                    continue;
                                }
                            )*
                        )*
                        break;
                    }
                    let mut out = $nones;
                    $(
                        $(
                            let x = self.$xs.get(self.distributor.get_output($i)).unwrap();
                            out.$i = Some(x.clone());
                        )*
                    )*
                    self.i += 1;
                    self.distributor.increment_counter();
                    Some($unwrap_tuple(out))
                }
            }
        }

        // Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$, possibly
        // with different output growth rates.
        //
        // The mapping from iterators to tuple slots is indicated by the function name; for example,
        // `exhaustive_triples_xyx_custom_output` takes two iterators, `xs`, and `ys`; `xs`
        // generates the elements in the first and third slots of the output triples, and `ys`
        // generates the elements in the second slots.
        //
        // Let $i$ be the index of the input iterators and $j$ be the index of the output slots. So
        // for `exhaustive_triples_xyx_custom_output`, $i=0$ corresponds to $j=0$ and $j=2$, and
        // $i=1$ corresponds to $j=1$.
        //
        // The $j$th `output_type_[i_j]` parameter is a
        // [`BitDistributorOutputType`](crate::iterators::bit_distributor::BitDistributorOutputType)
        // that determines how quickly the $j$th output slot advances through its iterator; see the
        // [`BitDistributor`](crate::iterators::bit_distributor::BitDistributor) documentation for a
        // description of the different types.
        //
        // If all of `xs`, `ys`, `zs`, ... are finite, then the output length may be obtained by
        // raising the length of each input iterator to power of the number of outputs it maps to,
        // and taking the product of the resulting values.
        //
        // If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        //
        // If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        //
        // # Examples
        // See [here](self#exhaustive_triples_xyx_custom_output).
        #[allow(dead_code)]
        $($vis)* fn $exhaustive_custom_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
            $($($out_x: BitDistributorOutputType,)*)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_struct {
                i: 0,
                limit: None,
                distributor: BitDistributor::new(&[$($($out_x,)*)*]),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                )*
            }
        }

        // Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        //
        // The mapping from iterators to tuple slots is indicated by the function name; for example,
        // `exhaustive_triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the elements
        // in the first and third slots of the output triples, and `ys` generates the elements in
        // the second slots.
        //
        // If all of `xs`, `ys`, `zs`, ... are finite, then the output length may be obtained by
        // raising the length of each input iterator to power of the number of outputs it maps to,
        // and taking the product of the resulting values.
        //
        // If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        //
        // If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        //
        // # Examples
        // See [here](self#exhaustive_triples_xyx).
        #[allow(dead_code)]
        #[inline]
        $($vis)* fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_custom_fn(
                $($xs,)*
                $($(BitDistributorOutputType::normal(1 + 0 * $i),)*)*
            )
        }
    }
}

#[cfg(feature = "test_build")]
#[allow(clippy::missing_const_for_fn)]
fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
    (a.unwrap(), b.unwrap(), c.unwrap())
}

#[cfg(feature = "test_build")]
custom_tuples!(
    (pub),
    ExhaustiveTriplesXYY,
    (X, Y, Y),
    (None, None, None),
    unwrap_triple,
    exhaustive_triples_xyy,
    exhaustive_triples_xyy_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0]],
    [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2]]
);

/// A trait used by dependent-pairs structs.
///
/// Given a reference to an `x`, produces an iterator of `ys`.
///
/// See [`LexDependentPairs`] and [`ExhaustiveDependentPairs`].
pub trait ExhaustiveDependentPairsYsGenerator<X: Clone, Y, J: Iterator<Item = Y>> {
    fn get_ys(&self, x: &X) -> J;
}

/// Generates pairs $(x, y)$, where the possible values of $y$ depend on the value of $x$. All $y$
/// values are output before proceeding to the next $x$.
///
/// This `struct` is created by; [`lex_dependent_pairs`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct LexDependentPairs<
    X: Clone,
    Y,
    S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
    I: Iterator<Item = X>,
    J: Iterator<Item = Y>,
> {
    done: bool,
    stop_after_empty_ys: bool,
    xs: I,
    ys: Option<J>,
    x: Option<X>,
    ys_generator: S,
}

impl<
        X: Clone,
        Y,
        S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
        I: Iterator<Item = X>,
        J: Iterator<Item = Y>,
    > LexDependentPairs<X, Y, S, I, J>
{
    fn advance_xs(&mut self) -> bool {
        if let Some(next_x) = self.xs.next() {
            self.x = Some(next_x);
            self.ys = Some(self.ys_generator.get_ys(self.x.as_ref().unwrap()));
            false
        } else {
            true
        }
    }
}

impl<
        X: Clone,
        Y,
        S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
        I: Iterator<Item = X>,
        J: Iterator<Item = Y>,
    > Iterator for LexDependentPairs<X, Y, S, I, J>
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<(X, Y)> {
        if self.done {
            None
        } else {
            let mut new_ys = false;
            if self.x.is_none() {
                if self.advance_xs() {
                    self.done = true;
                    return None;
                }
                new_ys = true;
            }
            loop {
                if let Some(y) = self.ys.as_mut().unwrap().next() {
                    return Some((self.x.as_ref().unwrap().clone(), y));
                } else if self.stop_after_empty_ys && new_ys || self.advance_xs() {
                    self.done = true;
                    return None;
                }
                new_ys = true;
            }
        }
    }
}

/// Generates pairs $(x, y)$, where the possible values of $y$ depend on the value of $x$. All $y$
/// values are output before proceeding to the next $x$.
///
/// This function takes an iterator `xs` that produces $x$ values, along with a `ys_generator` that
/// creates an iterator of $y$ values when given a reference to an $x$ value. The resulting iterator
/// first generates all pairs generated by the first $x$ value, then all pairs generated by the
/// second $x$ value, and so on.
///
/// It's called `lex_dependent_pairs` because if the `xs` iterator produces elements in some order,
/// and each `ys` iterator produces elements in some order (uniform across all `ys`), then the
/// resulting pairs are output in lexicographic order with respect to the $x$ and $y$ orders.
///
/// Each `ys` iterator produced by `ys_generator` must be finite; if some `ys` is infinite, then no
/// further `xs` value will be used. For a similar function that works with infinite `ys`, see
/// [`exhaustive_dependent_pairs`].
///
/// If, after a certain point, all the generated `ys` are empty, the output iterator will hang
/// trying to find another $(x, y)$ to output. To get around this, try
/// [`lex_dependent_pairs_stop_after_empty_ys`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::{
///     lex_dependent_pairs, ExhaustiveDependentPairsYsGenerator,
/// };
/// use maplit::hashmap;
/// use std::collections::HashMap;
/// use std::hash::Hash;
/// use std::iter::Cloned;
/// use std::slice::Iter;
///
/// #[derive(Clone, Debug)]
/// struct DPGeneratorFromMap<X: Clone + Eq + Hash, Y: 'static + Clone> {
///     map: HashMap<X, &'static [Y]>,
/// }
///
/// impl<X: Clone + Eq + Hash, Y: 'static + Clone>
///     ExhaustiveDependentPairsYsGenerator<X, Y, Cloned<Iter<'static, Y>>>
///     for DPGeneratorFromMap<X, Y>
/// {
///     #[inline]
///     fn get_ys(&self, x: &X) -> Cloned<Iter<'static, Y>> {
///         self.map[x].iter().cloned()
///     }
/// }
///
/// let xs = ["a", "b", "c", "b", "a"].iter().cloned();
/// let xss = lex_dependent_pairs(
///     xs,
///     DPGeneratorFromMap {
///         map: hashmap! {
///             "a" => &[2, 3, 4][..],
///             "b" => &[20][..],
///             "c" => &[30, 40][..]
///         },
///     },
/// )
/// .take(20)
/// .collect_vec();
/// assert_eq!(
///     xss.as_slice(),
///     &[
///         ("a", 2),
///         ("a", 3),
///         ("a", 4),
///         ("b", 20),
///         ("c", 30),
///         ("c", 40),
///         ("b", 20),
///         ("a", 2),
///         ("a", 3),
///         ("a", 4)
///     ]
/// );
///
/// let xs = [1, 2, 3, 2, 3, 2, 2].iter().cloned();
/// let xss = lex_dependent_pairs(
///     xs,
///     DPGeneratorFromMap {
///         map: hashmap! {
///             1 => &[100, 101, 102][..],
///             2 => &[][..],
///             3 => &[300, 301, 302][..]
///         },
///     },
/// )
/// .take(20)
/// .collect_vec();
/// assert_eq!(
///     xss.as_slice(),
///     &[(1, 100), (1, 101), (1, 102), (3, 300), (3, 301), (3, 302), (3, 300), (3, 301), (3, 302),]
/// );
/// ```
#[inline]
pub const fn lex_dependent_pairs<
    X: Clone,
    Y,
    S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
    I: Iterator<Item = X>,
    J: Iterator<Item = Y>,
>(
    xs: I,
    ys_generator: S,
) -> LexDependentPairs<X, Y, S, I, J> {
    LexDependentPairs {
        done: false,
        stop_after_empty_ys: false,
        xs,
        ys: None,
        x: None,
        ys_generator,
    }
}

/// Generates pairs $(x, y)$, where the possible values of $y$ depend on the value of $x$. $x$
/// values with no corresponding $y$ values are treated specially.
///
/// See [`lex_dependent_pairs`] for context.
///
/// If the output iterator encounters an $x$ value whose corresponding `ys` iterator is empty, the
/// output iterator stops iterating altogether. This prevents the iterator from getting stuck if all
/// `ys` iterators after a certain point are empty.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::{
///     lex_dependent_pairs_stop_after_empty_ys, ExhaustiveDependentPairsYsGenerator,
/// };
/// use maplit::hashmap;
/// use std::collections::HashMap;
/// use std::hash::Hash;
/// use std::iter::Cloned;
/// use std::slice::Iter;
///
/// #[derive(Clone, Debug)]
/// struct DPGeneratorFromMap<X: Clone + Eq + Hash, Y: 'static + Clone> {
///     map: HashMap<X, &'static [Y]>,
/// }
///
/// impl<X: Clone + Eq + Hash, Y: 'static + Clone>
///     ExhaustiveDependentPairsYsGenerator<X, Y, Cloned<Iter<'static, Y>>>
///     for DPGeneratorFromMap<X, Y>
/// {
///     #[inline]
///     fn get_ys(&self, x: &X) -> Cloned<Iter<'static, Y>> {
///         self.map[x].iter().cloned()
///     }
/// }
///
/// let xs = [1, 2, 3, 2, 3, 2, 2].iter().cloned();
/// let xss = lex_dependent_pairs_stop_after_empty_ys(
///     xs,
///     DPGeneratorFromMap {
///         map: hashmap! {
///             1 => &[100, 101, 102][..],
///             2 => &[][..],
///             3 => &[300, 301, 302][..]
///         },
///     },
/// )
/// .take(20)
/// .collect_vec();
/// // Stops after seeing 2, which maps to an empty iterator
/// assert_eq!(xss.as_slice(), &[(1, 100), (1, 101), (1, 102)]);
/// ```
#[inline]
pub const fn lex_dependent_pairs_stop_after_empty_ys<
    X: Clone,
    Y,
    S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
    I: Iterator<Item = X>,
    J: Iterator<Item = Y>,
>(
    xs: I,
    ys_generator: S,
) -> LexDependentPairs<X, Y, S, I, J> {
    LexDependentPairs {
        done: false,
        stop_after_empty_ys: true,
        xs,
        ys: None,
        x: None,
        ys_generator,
    }
}

/// Generates pairs $(x, y)$, where the possible values of $y$ depend on the value of $x$.
///
/// This `struct` is created by [`exhaustive_dependent_pairs`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveDependentPairs<
    X: Clone,
    Y,
    G: Iterator<Item = usize>,
    S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
    I: Iterator<Item = X>,
    J: Iterator<Item = Y>,
> {
    done: bool,
    xs_done: bool,
    stop_after_empty_ys: bool,
    index_generator: G,
    xs: I,
    xs_yss: Vec<(X, J, bool)>,
    ys_generator: S,
}

impl<
        X: Clone,
        Y,
        G: Iterator<Item = usize>,
        S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
        I: Iterator<Item = X>,
        J: Iterator<Item = Y>,
    > Iterator for ExhaustiveDependentPairs<X, Y, G, S, I, J>
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<(X, Y)> {
        if self.done {
            None
        } else {
            let original_i = self.index_generator.next().unwrap();
            loop {
                let mut i = original_i;
                let mut xs_yss_len = self.xs_yss.len();
                if self.xs_done {
                    i %= xs_yss_len;
                } else if i >= xs_yss_len {
                    for x in (&mut self.xs).take(i - xs_yss_len + 1) {
                        let ys = self.ys_generator.get_ys(&x);
                        self.xs_yss.push((x, ys, true));
                    }
                    xs_yss_len = self.xs_yss.len();
                    if xs_yss_len == 0 {
                        self.done = true;
                        return None;
                    } else if i >= xs_yss_len {
                        self.xs_done = true;
                        i %= xs_yss_len;
                    }
                }
                let t = &mut self.xs_yss[i];
                if let Some(y) = t.1.next() {
                    // t has been used
                    t.2 = false;
                    return Some((t.0.clone(), y));
                } else if self.stop_after_empty_ys && t.2 {
                    self.done = true;
                    return None;
                }
                self.xs_yss.remove(i);
                if self.xs_done && self.xs_yss.is_empty() {
                    self.done = true;
                    return None;
                }
            }
        }
    }
}

/// Generates pairs $(x, y)$, where the possible values of $y$ depend on the value of $x$.
///
/// This function takes an iterator `xs` that produces $x$ values, along with a `ys_generator` that
/// creates an iterator of $y$ values when given a reference to an $x$ value. The resulting iterator
/// does not use all of an $x$'s $y$s immediately. Instead, it uses an `index_generator` (an
/// iterator of `usize`s) to determine which $x$ value's iterator should be advanced. This
/// arrangement allows for an $x$ to map to infinitely many `ys`.
///
/// `index_generator` must generate every natural number infinitely many times. Good generators can
/// be created using [`ruler_sequence`](crate::num::iterators::ruler_sequence) or
/// [`bit_distributor_sequence`](crate::num::iterators::bit_distributor_sequence). The slower the
/// sequence's growth rate, the more this iterator will prefer to use initial $x$ values before
/// exploring later ones.
///
/// If you want all of an $x$ value's $y$s to be used before moving on to the next $x$, use
/// [`lex_dependent_pairs`] instead.
///
/// If, after a certain point, all the generated `ys` are empty, the output iterator will hang
/// trying to find another $(x, y)$ to output. To get around this, try
/// [`exhaustive_dependent_pairs_stop_after_empty_ys`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
/// use malachite_base::num::iterators::ruler_sequence;
/// use malachite_base::tuples::exhaustive::{
///     exhaustive_dependent_pairs, ExhaustiveDependentPairsYsGenerator,
/// };
/// use maplit::hashmap;
/// use std::collections::HashMap;
/// use std::hash::Hash;
/// use std::iter::Cloned;
/// use std::slice::Iter;
///
/// #[derive(Clone, Debug)]
/// pub struct MultiplesGeneratorHelper {
///     u: u64,
///     step: u64,
/// }
///
/// impl Iterator for MultiplesGeneratorHelper {
///     type Item = u64;
///
///     fn next(&mut self) -> Option<u64> {
///         let next = self.u;
///         self.u += self.step;
///         Some(next)
///     }
/// }
///
/// #[derive(Clone, Debug)]
/// struct MultiplesGenerator {}
///
/// impl ExhaustiveDependentPairsYsGenerator<u64, u64, MultiplesGeneratorHelper>
///     for MultiplesGenerator
/// {
///     #[inline]
///     fn get_ys(&self, x: &u64) -> MultiplesGeneratorHelper {
///         MultiplesGeneratorHelper { u: *x, step: *x }
///     }
/// }
///
/// #[derive(Clone, Debug)]
/// struct DPGeneratorFromMap<X: Clone + Eq + Hash, Y: 'static + Clone> {
///     map: HashMap<X, &'static [Y]>,
/// }
///
/// impl<X: Clone + Eq + Hash, Y: 'static + Clone>
///     ExhaustiveDependentPairsYsGenerator<X, Y, Cloned<Iter<'static, Y>>>
///     for DPGeneratorFromMap<X, Y>
/// {
///     #[inline]
///     fn get_ys(&self, x: &X) -> Cloned<Iter<'static, Y>> {
///         self.map[x].iter().cloned()
///     }
/// }
///
/// // All (x, y) where x is a positive natural and y is a positive multiple of x. It would be
/// // easier to do
/// //
/// // exhaustive_pairs_from_single(exhaustive_positive_primitive_ints::<u64>())
/// //      .map(|(x, y)| (x, x * y))
/// //
/// // in this case.
/// let xs = exhaustive_positive_primitive_ints::<u64>();
/// let xss = exhaustive_dependent_pairs(ruler_sequence(), xs.clone(), MultiplesGenerator {})
///     .take(50)
///     .collect_vec();
/// assert_eq!(
///     xss.as_slice(),
///     &[
///         (1, 1),
///         (2, 2),
///         (1, 2),
///         (3, 3),
///         (1, 3),
///         (2, 4),
///         (1, 4),
///         (4, 4),
///         (1, 5),
///         (2, 6),
///         (1, 6),
///         (3, 6),
///         (1, 7),
///         (2, 8),
///         (1, 8),
///         (5, 5),
///         (1, 9),
///         (2, 10),
///         (1, 10),
///         (3, 9),
///         (1, 11),
///         (2, 12),
///         (1, 12),
///         (4, 8),
///         (1, 13),
///         (2, 14),
///         (1, 14),
///         (3, 12),
///         (1, 15),
///         (2, 16),
///         (1, 16),
///         (6, 6),
///         (1, 17),
///         (2, 18),
///         (1, 18),
///         (3, 15),
///         (1, 19),
///         (2, 20),
///         (1, 20),
///         (4, 12),
///         (1, 21),
///         (2, 22),
///         (1, 22),
///         (3, 18),
///         (1, 23),
///         (2, 24),
///         (1, 24),
///         (5, 10),
///         (1, 25),
///         (2, 26)
///     ]
/// );
///
/// let xs = [1, 2, 3, 2, 3, 2, 2].iter().cloned();
/// let xss = exhaustive_dependent_pairs(
///     ruler_sequence(),
///     xs,
///     DPGeneratorFromMap {
///         map: hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
///     },
/// )
/// .take(20)
/// .collect_vec();
/// assert_eq!(
///     xss.as_slice(),
///     &[(1, 100), (3, 300), (1, 101), (3, 300), (1, 102), (3, 301), (3, 302), (3, 301), (3, 302)]
/// );
/// ```
#[inline]
pub const fn exhaustive_dependent_pairs<
    X: Clone,
    Y,
    G: Iterator<Item = usize>,
    S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
    I: Iterator<Item = X>,
    J: Iterator<Item = Y>,
>(
    index_generator: G,
    xs: I,
    ys_generator: S,
) -> ExhaustiveDependentPairs<X, Y, G, S, I, J> {
    ExhaustiveDependentPairs {
        done: false,
        xs_done: false,
        stop_after_empty_ys: false,
        index_generator,
        xs,
        xs_yss: Vec::new(),
        ys_generator,
    }
}

/// Generates pairs $(x, y)$, where the possible values of $y$ depend on the value of $x$. $x$
/// values with no corresponding $y$ values are treated specially.
///
/// See [`exhaustive_dependent_pairs`] for context.
///
/// If the output iterator encounters an $x$ value whose corresponding `ys` iterator is empty, the
/// output iterator stops iterating altogether. This prevents the iterator from getting stuck if all
/// `ys` iterators after a certain point are empty.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::iterators::ruler_sequence;
/// use malachite_base::tuples::exhaustive::{
///     exhaustive_dependent_pairs_stop_after_empty_ys, ExhaustiveDependentPairsYsGenerator,
/// };
/// use maplit::hashmap;
/// use std::collections::HashMap;
/// use std::hash::Hash;
/// use std::iter::Cloned;
/// use std::slice::Iter;
///
/// #[derive(Clone, Debug)]
/// pub struct MultiplesGeneratorHelper {
///     u: u64,
///     step: u64,
/// }
///
/// impl Iterator for MultiplesGeneratorHelper {
///     type Item = u64;
///
///     fn next(&mut self) -> Option<u64> {
///         let next = self.u;
///         self.u += self.step;
///         Some(next)
///     }
/// }
///
/// #[derive(Clone, Debug)]
/// struct DPGeneratorFromMap<X: Clone + Eq + Hash, Y: 'static + Clone> {
///     map: HashMap<X, &'static [Y]>,
/// }
///
/// impl<X: Clone + Eq + Hash, Y: 'static + Clone>
///     ExhaustiveDependentPairsYsGenerator<X, Y, Cloned<Iter<'static, Y>>>
///     for DPGeneratorFromMap<X, Y>
/// {
///     #[inline]
///     fn get_ys(&self, x: &X) -> Cloned<Iter<'static, Y>> {
///         self.map[x].iter().cloned()
///     }
/// }
///
/// let xs = [1, 2, 3, 2, 3, 2, 2].iter().cloned();
/// let xss = exhaustive_dependent_pairs_stop_after_empty_ys(
///     ruler_sequence(),
///     xs,
///     DPGeneratorFromMap {
///         map: hashmap! {
///             1 => &[100, 101, 102][..],
///             2 => &[][..],
///             3 => &[300, 301, 302][..]
///         },
///     },
/// )
/// .take(20)
/// .collect_vec();
/// assert_eq!(xss.as_slice(), &[(1, 100)]);
/// ```
#[inline]
pub const fn exhaustive_dependent_pairs_stop_after_empty_ys<
    X: Clone,
    Y,
    G: Iterator<Item = usize>,
    S: ExhaustiveDependentPairsYsGenerator<X, Y, J>,
    I: Iterator<Item = X>,
    J: Iterator<Item = Y>,
>(
    index_generator: G,
    xs: I,
    ys_generator: S,
) -> ExhaustiveDependentPairs<X, Y, G, S, I, J> {
    ExhaustiveDependentPairs {
        done: false,
        xs_done: false,
        stop_after_empty_ys: true,
        index_generator,
        xs,
        xs_yss: Vec::new(),
        ys_generator,
    }
}

/// Defines lexicographic ordered unique tuple generators.
///
/// Malachite provides [`lex_ordered_unique_pairs`], but you can also define
/// `lex_ordered_unique_triples`, `lex_ordered_unique_quadruples`, and so on, in your program using
/// the code below. The documentation for [`lex_ordered_unique_pairs`] describes these other
/// functions as well.
///
/// See usage examples [here](self#lex_ordered_unique_quadruples).
///
/// ```
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_ordered_unique_tuples;
/// use malachite_base::vecs::exhaustive::fixed_length_ordered_unique_indices_helper;
/// use std::marker::PhantomData;
///
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueTriples,
///     3,
///     (I::Item, I::Item, I::Item),
///     lex_ordered_unique_triples,
///     [0, 1, 2]
/// );
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     lex_ordered_unique_quadruples,
///     [0, 1, 2, 3]
/// );
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueQuintuples,
///     5,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     lex_ordered_unique_quintuples,
///     [0, 1, 2, 3, 4]
/// );
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueSextuples,
///     6,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     lex_ordered_unique_sextuples,
///     [0, 1, 2, 3, 4, 5]
/// );
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueSeptuples,
///     7,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     lex_ordered_unique_septuples,
///     [0, 1, 2, 3, 4, 5, 6]
/// );
/// lex_ordered_unique_tuples!(
///     (pub(crate)),
///     LexOrderedUniqueOctuples,
///     8,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     lex_ordered_unique_octuples,
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// );
/// ```
#[macro_export]
macro_rules! lex_ordered_unique_tuples {
    (
        ($($vis:tt)*),
        $struct: ident,
        $k: expr,
        $out_t: ty,
        $fn: ident,
        [$($i: expr),*]
    ) => {
        /// This documentation applies not only to `LexOrderedUniquePairs`, but also to
        /// `LexOrderedUniqueTriples`, `LexOrderedUniqueQuadruples`, and so on. See
        /// [`lex_ordered_unique_tuples`] for more information.
        ///
        /// Generates all $k$-tuples of elements from an iterator, where the tuples have no
        /// repetitions and are ordered the same way as in the iterator.
        ///
        /// The tuples are generated in lexicographic order with respect to the order of the element
        /// iterator.
        #[derive(Clone, Debug)]
        $($vis)* struct $struct<I: Iterator> where I::Item: Clone {
            first: bool,
            done: bool,
            xs: IteratorCache<I>,
            indices: [usize; $k],
            phantom: PhantomData<*const I::Item>,
        }

        impl<I: Iterator> Iterator for $struct<I> where I::Item: Clone {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    return None;
                }
                if self.first {
                    self.first = false;
                    self.xs.get($k);
                    if let Some(n) = self.xs.known_len() {
                        if n < $k {
                            self.done = true;
                            return None;
                        }
                    }
                } else {
                    if let Some(n) = self.xs.known_len() {
                        if fixed_length_ordered_unique_indices_helper(n, $k, &mut self.indices) {
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
                Some(($(self.xs.assert_get(self.indices[$i]).clone(),)*))
            }
        }

        /// This documentation applies not only to `lex_ordered_unique_pairs`, but also to
        /// `lex_ordered_unique_triples`, `lex_ordered_unique_quadruples`, and so on. See
        /// [`lex_ordered_unique_tuples`] for more information.
        ///
        /// Generates $k$-tuples of elements from a single iterator, such that each tuple has no
        /// repeated elements, and the elements in each [`Vec`] are ordered the same way as they are
        /// in the source iterator.
        ///
        /// The source iterator should not repeat any elements, but this is not enforced.
        ///
        /// The order is lexicographic with respect to the order of the element iterator.
        ///
        /// If the input iterator is infinite, the output length is also infinite.
        ///
        /// If the input iterator length is $n$, the output length is $\binom{n}{k}$.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#lex_ordered_unique_quadruples).
        $($vis)* const fn $fn<I: Iterator>(xs: I) -> $struct<I> where I::Item: Clone {
            $struct {
                first: true,
                done: false,
                xs: IteratorCache::new(xs),
                indices: [$($i),*],
                phantom: PhantomData,
            }
        }
    }
}

lex_ordered_unique_tuples!(
    (pub),
    LexOrderedUniquePairs,
    2,
    (I::Item, I::Item),
    lex_ordered_unique_pairs,
    [0, 1]
);

/// Defines exhaustive ordered unique tuple generators.
///
/// Malachite provides [`exhaustive_ordered_unique_pairs`], but you can also define
/// `exhaustive_ordered_unique_triples`, `exhaustive_ordered_unique_quadruples`, and so on, in your
/// program using the code below. The documentation for [`exhaustive_ordered_unique_pairs`]
/// describes these other functions as well.
///
/// See usage examples [here](self#exhaustive_ordered_unique_quadruples).
///
/// ```
/// use malachite_base::exhaustive_ordered_unique_tuples;
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::vecs::exhaustive::next_bit_pattern;
///
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueTriples,
///     3,
///     (I::Item, I::Item, I::Item),
///     exhaustive_ordered_unique_triples,
///     [0, 1, 2]
/// );
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     exhaustive_ordered_unique_quadruples,
///     [0, 1, 2, 3]
/// );
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueQuintuples,
///     5,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     exhaustive_ordered_unique_quintuples,
///     [0, 1, 2, 3, 4]
/// );
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueSextuples,
///     6,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     exhaustive_ordered_unique_sextuples,
///     [0, 1, 2, 3, 4, 5]
/// );
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueSeptuples,
///     7,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     exhaustive_ordered_unique_septuples,
///     [0, 1, 2, 3, 4, 5, 6]
/// );
/// exhaustive_ordered_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveOrderedUniqueOctuples,
///     8,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     exhaustive_ordered_unique_octuples,
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// );
/// ```
#[macro_export]
macro_rules! exhaustive_ordered_unique_tuples {
    (
        ($($vis:tt)*),
        $struct: ident,
        $k: expr,
        $out_t: ty,
        $fn: ident,
        [$($i: expr),*]
    ) => {
        /// This documentation applies not only to `ExhaustiveOrderedUniquePairs`, but also to
        /// `ExhaustiveOrderedUniqueTriples`, `ExhaustiveOrderedUniqueQuadruples`, and so on. See
        /// [`exhaustive_ordered_unique_tuples`] for more information.
        ///
        /// Generates all $k$-tuples of elements from an iterator, where the tuples have no
        /// repetitions and are ordered the same way as in the iterator.
        #[derive(Clone)]
        pub struct $struct<I: Iterator>
        where
            I::Item: Clone,
        {
            done: bool,
            first: bool,
            xs: IteratorCache<I>,
            pattern: Vec<bool>,
        }

        impl<I: Iterator> Iterator for $struct<I>
        where
            I::Item: Clone,
        {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    return None;
                } else if self.first {
                    self.first = false;
                } else {
                    let mut c = $k;
                    next_bit_pattern(&mut self.pattern, &mut c, $k, $k);
                }
                if !self.pattern.is_empty() && self.xs.get(self.pattern.len() - 1).is_none() {
                    self.done = true;
                    return None;
                }
                let mut results = self.pattern.iter().enumerate().filter_map(|(i, &b)| {
                    if b {
                        Some(self.xs.assert_get(i).clone())
                    } else {
                        None
                    }
                });
                Some(($(((results.next().unwrap(), $i).0)),*))
            }
        }

        /// This documentation applies not only to `exhaustive_ordered_unique_pairs`, but also to
        /// `exhaustive_ordered_unique_triples`, `exhaustive_ordered_unique_quadruples`, and so on.
        /// See [`exhaustive_ordered_unique_tuples`] for more information.
        ///
        /// Generates $k$-tuples of elements from a single iterator, such that each tuple has no
        /// repeated elements, and the elements in each [`Vec`] are ordered the same way as they are
        /// in the source iterator.
        ///
        /// The source iterator should not repeat any elements, but this is not enforced.
        ///
        /// If the input iterator is infinite, the output length is also infinite.
        ///
        /// If the input iterator length is $n$, the output length is $\binom{n}{k}$.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_ordered_unique_quadruples).
        pub fn $fn<I: Iterator>(xs: I) -> $struct<I>
        where
            I::Item: Clone,
        {
            $struct {
                done: false,
                first: true,
                xs: IteratorCache::new(xs),
                pattern: vec![true; $k],
            }
        }
    }
}
exhaustive_ordered_unique_tuples!(
    (pub),
    ExhaustiveOrderedUniquePairs,
    2,
    (I::Item, I::Item),
    exhaustive_ordered_unique_pairs,
    [0, 1]
);

/// Defines lexicographic unique tuple generators.
///
/// Malachite provides [`lex_unique_pairs`], but you can also define `lex_unique_triples`,
/// `lex_unique_quadruples`, and so on, in your program using the code below. The documentation for
/// [`lex_unique_pairs`] describes these other functions as well.
///
/// See usage examples [here](self#lex_unique_pairs).
///
/// ```
/// use malachite_base::iterators::iterator_cache::IteratorCache;
/// use malachite_base::lex_unique_tuples;
/// use malachite_base::vecs::exhaustive::{unique_indices, UniqueIndices};
///
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueTriples,
///     3,
///     (I::Item, I::Item, I::Item),
///     lex_unique_triples,
///     [0, 1, 2]
/// );
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     lex_unique_quadruples,
///     [0, 1, 2, 3]
/// );
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueQuintuples,
///     5,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     lex_unique_quintuples,
///     [0, 1, 2, 3, 4]
/// );
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueSextuples,
///     6,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     lex_unique_sextuples,
///     [0, 1, 2, 3, 4, 5]
/// );
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueSeptuples,
///     7,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     lex_unique_septuples,
///     [0, 1, 2, 3, 4, 5, 6]
/// );
/// lex_unique_tuples!(
///     (pub(crate)),
///     LexUniqueOctuples,
///     8,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     lex_unique_octuples,
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// );
/// ```
#[macro_export]
macro_rules! lex_unique_tuples {
    (
        ($($vis:tt)*),
        $struct: ident,
        $k: expr,
        $out_t: ty,
        $fn: ident,
        [$($i: expr),*]
    ) => {
        /// This documentation applies not only to `LexUniquePairs`, but also to `LexUniqueTriples`,
        /// `LexUniqueQuadruples`, and so on. See [`lex_unique_tuples`] for more information.
        ///
        /// Generates all $k$-tuples of elements from an iterator, where the tuples have no
        /// repetitions.
        ///
        /// The tuples are generated in lexicographic order with respect to the order of the element
        /// iterator.
        #[derive(Clone)]
        $($vis)* struct $struct<I: Iterator> where I::Item: Clone {
            first: bool,
            xs: IteratorCache<I>,
            indices: UniqueIndices,
        }

        impl<I: Iterator> Iterator for $struct<I> where I::Item: Clone {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.first {
                    let nonempty = !self.indices.used.is_empty();
                    if nonempty && self.xs.get(self.indices.get_n() - 1).is_none() {
                        self.indices.done = true;
                    }
                    self.first = false;
                }
                if self.xs.get(self.indices.get_n()).is_some() {
                    self.indices.increment_n();
                }
                self.indices.next().map(|indices| {
                    let mut results = indices.into_iter().map(|i| self.xs.assert_get(i).clone());
                    ($(((results.next().unwrap(), $i).0)),*)
                })
            }
        }

        /// This documentation applies not only to `lex_unique_pairs`, but also to
        /// `lex_unique_triples`, `lex_unique_quadruples`, and so on. See [`lex_unique_tuples`] for
        /// more information.
        ///
        /// Generates $k$-tuples of elements from a single iterator, such that each tuple has no
        /// repeated elements.
        ///
        /// The source iterator should not repeat any elements, but this is not enforced.
        ///
        /// The order is lexicographic with respect to the order of the element iterator.
        ///
        /// If the input iterator is infinite, the output length is also infinite.
        ///
        /// If the input iterator length is $n$, the output length is $\frac{n!}{k!}$.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#lex_unique_quadruples).
        #[inline]
        $($vis)* fn $fn<I: Iterator>(xs: I) -> $struct<I> where I::Item: Clone {
            $struct {
                first: true,
                xs: IteratorCache::new(xs),
                // Initial n is k, but will grow to reach actual n (or forever, if n is infinite)
                indices: unique_indices($k, $k),
            }
        }
    }
}

lex_unique_tuples!(
    (pub),
    LexUniquePairs,
    2,
    (I::Item, I::Item),
    lex_unique_pairs,
    [0, 1]
);

/// Generates all pairs of elements from an iterator, where the pairs have no repetitions.
///
/// This `struct` is created by [`exhaustive_unique_pairs`]; see its documentation for more.
#[derive(Clone)]
pub struct ExhaustiveUniquePairs<I: Iterator>
where
    I::Item: Clone,
{
    next: Option<(I::Item, I::Item)>,
    ps: ExhaustiveOrderedUniquePairs<I>,
}

impl<I: Iterator> Iterator for ExhaustiveUniquePairs<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        if self.next.is_some() {
            take(&mut self.next)
        } else if let Some(p) = self.ps.next() {
            self.next = Some((p.1.clone(), p.0.clone()));
            Some(p)
        } else {
            None
        }
    }
}

/// Generates pairs of elements from a single iterator, such that each pair has no repeated
/// elements.
///
/// The source iterator should not repeat any elements, but this is not enforced.
///
/// If the input iterator is infinite, the output length is also infinite.
///
/// If the input iterator length is $n$, the output length is $\tfrac{1}{2}{n!}$.
///
/// If `xs` is empty, the output is also empty.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::exhaustive::exhaustive_unique_pairs;
///
/// let xss = exhaustive_unique_pairs(1..=6).take(20).collect_vec();
/// assert_eq!(
///     xss.into_iter().collect_vec().as_slice(),
///     &[
///         (1, 2),
///         (2, 1),
///         (1, 3),
///         (3, 1),
///         (2, 3),
///         (3, 2),
///         (1, 4),
///         (4, 1),
///         (2, 4),
///         (4, 2),
///         (3, 4),
///         (4, 3),
///         (1, 5),
///         (5, 1),
///         (2, 5),
///         (5, 2),
///         (3, 5),
///         (5, 3),
///         (4, 5),
///         (5, 4)
///     ]
/// );
/// ```
pub fn exhaustive_unique_pairs<I: Iterator>(xs: I) -> ExhaustiveUniquePairs<I>
where
    I::Item: Clone,
{
    ExhaustiveUniquePairs {
        next: None,
        ps: exhaustive_ordered_unique_pairs(xs),
    }
}

/// Defines lexicographic unique tuple generators.
///
/// Malachite provides [`exhaustive_unique_pairs`], but you can also define
/// `exhaustive_unique_triples`, `lex_unique_quadruples`, and so on, in your program using the code
/// below.
///
/// See usage examples [here](self#lex_unique_quadruples).
///
/// ```
/// use malachite_base::exhaustive_unique_tuples;
/// use malachite_base::num::iterators::{ruler_sequence, RulerSequence};
/// use malachite_base::tuples::exhaustive::{
///     exhaustive_dependent_pairs, ExhaustiveDependentPairs,
/// };
/// use malachite_base::vecs::exhaustive::{
///     exhaustive_ordered_unique_vecs_fixed_length, ExhaustiveOrderedUniqueCollections,
///     ExhaustiveUniqueVecsGenerator,
/// };
/// use malachite_base::vecs::ExhaustiveVecPermutations;
///
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueTriples,
///     3,
///     (I::Item, I::Item, I::Item),
///     exhaustive_unique_triples,
///     [0, 1, 2]
/// );
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     exhaustive_unique_quadruples,
///     [0, 1, 2, 3]
/// );
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueQuintuples,
///     5,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     exhaustive_unique_quintuples,
///     [0, 1, 2, 3, 4]
/// );
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueSextuples,
///     6,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     exhaustive_unique_sextuples,
///     [0, 1, 2, 3, 4, 5]
/// );
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueSeptuples,
///     7,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     exhaustive_unique_septuples,
///     [0, 1, 2, 3, 4, 5, 6]
/// );
/// exhaustive_unique_tuples!(
///     (pub(crate)),
///     ExhaustiveUniqueOctuples,
///     8,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     exhaustive_unique_octuples,
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// );
/// ```
#[macro_export]
macro_rules! exhaustive_unique_tuples {
    (
        ($($vis:tt)*),
        $struct: ident,
        $k: expr,
        $out_t: ty,
        $fn: ident,
        [$($i: expr),*]
    ) => {
        // Generates all $k$-tuples of elements from an iterator, where the tuples have no
        // repetitions.
        #[derive(Clone)]
        $($vis)* struct $struct<I: Iterator> where I::Item: Clone {
            xss: ExhaustiveDependentPairs<
                Vec<I::Item>,
                Vec<I::Item>,
                RulerSequence<usize>,
                ExhaustiveUniqueVecsGenerator<I::Item, I>,
                ExhaustiveOrderedUniqueCollections<I, Vec<I::Item>>,
                ExhaustiveVecPermutations<I::Item>,
            >
        }

        impl<I: Iterator> Iterator for $struct<I> where I::Item: Clone {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                self.xss.next().map(|mut p| {
                    let mut drain = p.1.drain(..);
                    ($(((drain.next().unwrap(), $i).0)),*)
                })
            }
        }

        // Generates $k$-tuples of elements from a single iterator, such that each tuple has no
        // repeated elements.
        //
        // The source iterator should not repeat any elements, but this is not enforced.
        //
        // If the input iterator is infinite, the output length is also infinite.
        //
        // If the input iterator length is $n$, the output length is $\frac{n!}{k!}$.
        //
        // If `xs` is empty, the output is also empty.
        //
        // # Examples
        // See [here](self#exhaustive_unique_quadruples).
        $($vis)* fn $fn<I: Iterator>(xs: I) -> $struct<I> where I::Item: Clone {
            $struct {
                xss: exhaustive_dependent_pairs(
                    ruler_sequence(),
                    exhaustive_ordered_unique_vecs_fixed_length($k, xs),
                    ExhaustiveUniqueVecsGenerator::new(),
                )
            }
        }
    }
}
