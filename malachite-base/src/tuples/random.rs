// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::random::Seed;
use std::cmp::Ordering::*;
use std::iter::{repeat, Repeat};

/// Generates random units; repeats `()`.
///
/// $P(()) = 1$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::tuples::random::random_units;
///
/// assert_eq!(random_units().take(10).collect_vec(), &[(); 10]);
/// ```
pub fn random_units() -> Repeat<()> {
    repeat(())
}

// hack for macro
#[doc(hidden)]
#[inline]
pub fn next_helper<I: Iterator>(x: &mut I, _i: usize) -> Option<I::Item> {
    x.next()
}

/// Defines random tuple generators.
///
/// Malachite provides [`random_pairs`] and [`random_pairs_from_single`], but you can also define
/// `random_triples`, `random_quadruples`, and so on, and `random_triples_from_single`,
/// `random_quadruples_from_single`, and so on, in your program using the code below. The
/// documentation for [`random_pairs`] and [`random_pairs_from_single`] describes these other
/// functions as well.
///
/// See usage examples [here](self#random_pairs) and [here](self#random_pairs_from_single).
///
/// ```
/// use malachite_base::random::Seed;
/// use malachite_base::random_tuples;
/// use malachite_base::tuples::random::next_helper;
///
/// random_tuples!(
///     (pub(crate)),
///     RandomTriples,
///     RandomTriplesFromSingle,
///     random_triples,
///     random_triples_from_single,
///     (I::Item, I::Item, I::Item),
///     [0, X, I, xs, xs_gen],
///     [1, Y, J, ys, ys_gen],
///     [2, Z, K, zs, zs_gen]
/// );
/// random_tuples!(
///     (pub(crate)),
///     RandomQuadruples,
///     RandomQuadruplesFromSingle,
///     random_quadruples,
///     random_quadruples_from_single,
///     (I::Item, I::Item, I::Item, I::Item),
///     [0, X, I, xs, xs_gen],
///     [1, Y, J, ys, ys_gen],
///     [2, Z, K, zs, zs_gen],
///     [3, W, L, ws, ws_gen]
/// );
/// random_tuples!(
///     (pub(crate)),
///     RandomQuintuples,
///     RandomQuintuplesFromSingle,
///     random_quintuples,
///     random_quintuples_from_single,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     [0, X, I, xs, xs_gen],
///     [1, Y, J, ys, ys_gen],
///     [2, Z, K, zs, zs_gen],
///     [3, W, L, ws, ws_gen],
///     [4, V, M, vs, vs_gen]
/// );
/// random_tuples!(
///     (pub(crate)),
///     RandomSextuples,
///     RandomSextuplesFromSingle,
///     random_sextuples,
///     random_sextuples_from_single,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     [0, X, I, xs, xs_gen],
///     [1, Y, J, ys, ys_gen],
///     [2, Z, K, zs, zs_gen],
///     [3, W, L, ws, ws_gen],
///     [4, V, M, vs, vs_gen],
///     [5, U, N, us, us_gen]
/// );
/// random_tuples!(
///     (pub(crate)),
///     RandomSeptuples,
///     RandomSeptuplesFromSingle,
///     random_septuples,
///     random_septuples_from_single,
///     (
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item,
///         I::Item
///     ),
///     [0, X, I, xs, xs_gen],
///     [1, Y, J, ys, ys_gen],
///     [2, Z, K, zs, zs_gen],
///     [3, W, L, ws, ws_gen],
///     [4, V, M, vs, vs_gen],
///     [5, U, N, us, us_gen],
///     [6, T, O, ts, ts_gen]
/// );
/// random_tuples!(
///     (pub(crate)),
///     RandomOctuples,
///     RandomOctuplesFromSingle,
///     random_octuples,
///     random_octuples_from_single,
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
///     [0, X, I, xs, xs_gen],
///     [1, Y, J, ys, ys_gen],
///     [2, Z, K, zs, zs_gen],
///     [3, W, L, ws, ws_gen],
///     [4, V, M, vs, vs_gen],
///     [5, U, N, us, us_gen],
///     [6, T, O, ts, ts_gen],
///     [7, S, P, ss, ss_gen]
/// );
/// ```
#[macro_export]
macro_rules! random_tuples {
    (
        ($($vis:tt)*),
        $random_struct: ident,
        $random_struct_from_single: ident,
        $random_fn: ident,
        $random_fn_from_single: ident,
        $single_out: tt,
        $([$i: expr, $t: ident, $it: ident, $xs: ident, $xs_gen:ident]),*
    ) => {
        /// This documentation applies not only to `RandomPairs`, but also to `RandomTriples`,
        /// `RandomQuadruples`, and so on. See [`random_tuples`] for more information.
        ///
        /// Generates random $n$-tuples using elements from $n$ iterators.
        #[derive(Clone, Debug)]
        #[allow(dead_code)]
        $($vis)* struct $random_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            $($xs: $it,)*
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $random_struct<$($t, $it,)*>
        {
            type Item = ($($t,)*);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some(($(self.$xs.next().unwrap()),*))
            }
        }

        /// This documentation applies not only to `random_pairs`, but also to `random_triples`,
        /// `random_quadruples`, and so on. See [`random_tuples`] for more information.
        ///
        /// Generates random $n$-tuples with elements from $n$ iterators.
        ///
        /// The probability of a particular $n$-tuple being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs`, `ys`, `zs`, ... must be infinite.
        ///
        /// # Examples
        /// See [here](self#random_pairs).
        #[allow(dead_code)]
        $($vis)* fn $random_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
        ) -> $random_struct<$($t, $it,)*> {
            $random_struct {
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
            }
        }

        /// This documentation applies not only to `RandomPairsFromSingle`, but also to
        /// `RandomTriplesFromSingle`, `RandomQuadruplesFromSingle`, and so on. See
        /// [`random_tuples`] for more information.
        ///
        /// Generates random $n$-tuples using elements from a single iterator.
        #[derive(Clone, Debug)]
        #[allow(dead_code)]
        $($vis)* struct $random_struct_from_single<I: Iterator> {
            xs: I
        }

        impl<I: Iterator> Iterator for $random_struct_from_single<I> {
            type Item = $single_out;

            #[inline]
            fn next(&mut self) -> Option<$single_out> {
                Some(($(next_helper(&mut self.xs, $i).unwrap(),)*))
            }
        }

        /// This documentation applies not only to `random_pairs_from_single`, but also to
        /// `random_triples_from_single`, `random_quadruples_from_single`, and so on. See
        /// [`random_tuples`] for more information.
        ///
        /// Generates random $n$-tuples using elements from a single iterator.
        ///
        /// The probability of a particular $n$-tuple being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs` must be infinite.
        ///
        /// # Examples
        /// See [here](self#random_pairs_from_single).
        #[allow(dead_code)]
        #[inline]
        $($vis)* const fn $random_fn_from_single<I: Iterator>(xs: I)
                -> $random_struct_from_single<I> {
            $random_struct_from_single { xs }
        }
    }
}

random_tuples!(
    (pub),
    RandomPairs,
    RandomPairsFromSingle,
    random_pairs,
    random_pairs_from_single,
    (I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen]
);

/// Defines custom random tuple generators.
///
/// You can define custom tuple generators like `random_triples_xyx` in your program using the code
/// below.
///
/// See usage examples [here](self#random_triples_xyx).
///
/// ```
/// use malachite_base::random::Seed;
/// use malachite_base::random_custom_tuples;
///
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomTriplesXXY,
///     (X, X, Y),
///     random_triples_xxy,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1]],
///     [Y, J, ys, ys_gen, [y_2, y_2]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomTriplesXYX,
///     (X, Y, X),
///     random_triples_xyx,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
///     [Y, J, ys, ys_gen, [y_1, x_2]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomTriplesXYY,
///     (X, Y, Y),
///     random_triples_xyy,
///     [X, I, xs, xs_gen, [x_0, x_0]],
///     [Y, J, ys, ys_gen, [y_1, y_1], [y_2, y_2]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXXXY,
///     (X, X, X, Y),
///     random_quadruples_xxxy,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2]],
///     [Y, J, ys, ys_gen, [y_3, y_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXXYX,
///     (X, X, Y, X),
///     random_quadruples_xxyx,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_3, y_2]],
///     [Y, J, ys, ys_gen, [y_2, x_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXXYZ,
///     (X, X, Y, Z),
///     random_quadruples_xxyz,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1]],
///     [Y, J, ys, ys_gen, [y_2, y_2]],
///     [Z, K, zs, zs_gen, [z_3, z_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXYXZ,
///     (X, Y, X, Z),
///     random_quadruples_xyxz,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
///     [Y, J, ys, ys_gen, [y_1, x_2]],
///     [Z, K, zs, zs_gen, [z_3, z_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXYYX,
///     (X, Y, Y, X),
///     random_quadruples_xyyx,
///     [X, I, xs, xs_gen, [x_0, x_0], [x_3, y_1]],
///     [Y, J, ys, ys_gen, [y_1, y_2], [y_2, x_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXYYZ,
///     (X, Y, Y, Z),
///     random_quadruples_xyyz,
///     [X, I, xs, xs_gen, [x_0, x_0]],
///     [Y, J, ys, ys_gen, [y_1, y_1], [y_2, y_2]],
///     [Z, K, zs, zs_gen, [z_3, z_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuadruplesXYZZ,
///     (X, Y, Z, Z),
///     random_quadruples_xyzz,
///     [X, I, xs, xs_gen, [x_0, x_0]],
///     [Y, J, ys, ys_gen, [y_1, y_1]],
///     [Z, K, zs, zs_gen, [z_2, z_2], [z_3, z_3]]
/// );
/// random_custom_tuples!(
///     (pub(crate)),
///     RandomQuintuplesXYYYZ,
///     (X, Y, Y, Y, Z),
///     random_quintuples_xyyyz,
///     [X, I, xs, xs_gen, [x_0, x_0]],
///     [Y, J, ys, ys_gen, [y_1, y_1], [y_2, y_2], [y_3, y_3]],
///     [Z, K, zs, zs_gen, [z_4, z_4]]
/// );
/// ```
#[macro_export]
macro_rules! random_custom_tuples {
    (
        ($($vis:tt)*),
        $random_struct: ident,
        $out_t: ty,
        $random_fn: ident,
        $([$t: ident, $it: ident, $xs: ident, $xs_gen: ident, $([$x: ident, $x_ord: ident]),*]),*
    ) => {
        // Generates random $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        //
        // The mapping from iterators to tuple slots is indicated by the struct name; for example,
        // in `RandomTriplesXYX` there are two iterators, `X`, and `Y`; `X` generates the elements
        // in the first and third slots of the output triples, and `Y` generates the elements in the
        // second slots.
        #[derive(Clone, Debug)]
        $($vis)* struct $random_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            $($xs: $it,)*
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $random_struct<$($t, $it,)*>
        {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    $(
                        let $x = self.$xs.next().unwrap();
                    )*
                )*
                Some(($($($x_ord,)*)*))
            }
        }

        // Generates random $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        //
        // The mapping from iterators to tuple slots is indicated by the function name; for example,
        // `random_triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the elements in
        // the first and third slots of the output triples, and `ys` generates the elements in the
        // second slots.
        //
        // The probability of a particular $n$-tuple being generated is the product of the
        // probabilities of each of its elements.
        //
        // `xs`, `ys`, `zs`, ... must be infinite.
        //
        // # Examples
        // See [here](self#random_triples_xyx).
        $($vis)* fn $random_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
        ) -> $random_struct<$($t, $it,)*> {
            $random_struct {
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
            }
        }
    }
}

/// Generates random pairs using elements from a single iterator, where the first element is less
/// than the second.
#[derive(Clone, Debug)]
pub struct RandomOrderedUniquePairs<I: Iterator>
where
    I::Item: Ord,
{
    xs: I,
}

impl<I: Iterator> Iterator for RandomOrderedUniquePairs<I>
where
    I::Item: Ord,
{
    type Item = (I::Item, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut out_0 = None;
        let out_1;
        loop {
            let x = self.xs.next().unwrap();
            if out_0.is_none() {
                out_0 = Some(x);
            } else {
                match x.cmp(out_0.as_ref().unwrap()) {
                    Equal => {}
                    Greater => {
                        out_1 = x;
                        break;
                    }
                    Less => {
                        out_1 = out_0.unwrap();
                        out_0 = Some(x);
                        break;
                    }
                }
            }
        }
        Some((out_0.unwrap(), out_1))
    }
}

/// Generates random pairs using elements from a single iterator, where the first element of each
/// pair is less than the second.
///
/// The input iterator must generate at least two distinct elements; otherwise, this iterator will
/// hang.
///
/// $$
/// P((x\_0, x\_1)) = 2P(x\_0)P(x\_1).
/// $$
///
/// The above formula assumes that the pair is valid, \emph{i.e.} its first element is less than its
/// second. The probability of an invalid pair is zero.
///
/// `xs` must be infinite.
#[inline]
pub const fn random_ordered_unique_pairs<I: Iterator>(xs: I) -> RandomOrderedUniquePairs<I>
where
    I::Item: Ord,
{
    RandomOrderedUniquePairs { xs }
}

/// Defines random ordered unique tuple generators.
///
/// Malachite provides [`random_ordered_unique_pairs`], but you can also define
/// `random_ordered_unique_triples`, `random_ordered_unique_quadruples`, and so on, in your program
/// using the code below.
///
/// See usage examples [here](self#random_ordered_unique_quadruples).
///
/// ```
/// use malachite_base::random_ordered_unique_tuples;
/// use malachite_base::sets::random::{
///     random_b_tree_sets_fixed_length, RandomBTreeSetsFixedLength,
/// };
///
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueTriples,
///     3,
///     (I::Item, I::Item, I::Item),
///     random_ordered_unique_triples,
///     [0, 1, 2]
/// );
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     random_ordered_unique_quadruples,
///     [0, 1, 2, 3]
/// );
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueQuintuples,
///     5,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     random_ordered_unique_quintuples,
///     [0, 1, 2, 3, 4]
/// );
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueSextuples,
///     6,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     random_ordered_unique_sextuples,
///     [0, 1, 2, 3, 4, 5]
/// );
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueSeptuples,
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
///     random_ordered_unique_septuples,
///     [0, 1, 2, 3, 4, 5, 6]
/// );
/// random_ordered_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueOctuples,
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
///     random_ordered_unique_octuples,
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// );
/// ```
#[macro_export]
macro_rules! random_ordered_unique_tuples {
    (
        ($($vis:tt)*),
        $struct: ident,
        $k: expr,
        $out_t: ty,
        $fn: ident,
        [$($i: expr),*]
    ) => {
        // Generates random $n$-tuples using elements from a single iterator, where the tuples have
        // no repeated elements, and the elements are in ascending order.
        #[derive(Clone, Debug)]
        $($vis)* struct $struct<I: Iterator> where I::Item: Ord {
            xs: RandomBTreeSetsFixedLength<I>,
        }

        impl<I: Iterator> Iterator for $struct<I> where I::Item: Ord {
            type Item = $out_t;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let mut elements = self.xs.next().unwrap().into_iter();
                Some(($(((elements.next().unwrap(), $i).0)),*))
            }
        }

        // Generates random $n$-tuples using elements from a single iterator, where the tuples have
        // no repeated elements, and the elements are in ascending order.
        //
        // The input iterator must generate at least `len` distinct elements; otherwise, this
        // iterator will hang.
        //
        // $$
        // P((x\_i)\_{i=0}^{n-1}) = n!\prod\_{i=0}^{n-1}P(x\_i).
        // $$
        //
        // The above formula assumes that the tuple is valid, \emph{i.e.} its elements are strictly
        // increasing. The probability of an invalid tuple is zero.
        //
        // `xs` must be infinite.
        //
        // # Examples
        // See [here](self#random_ordered_unique_quadruples).
        #[inline]
        $($vis)* fn $fn<I: Iterator>(xs: I) -> $struct<I>
        where
            I::Item: Ord,
        {
            $struct {
                xs: random_b_tree_sets_fixed_length($k, xs),
            }
        }
    }
}

/// Generates random pairs using elements from a single iterator, where the first element is not
/// equal to the second.
#[derive(Clone, Debug)]
pub struct RandomUniquePairs<I: Iterator>
where
    I::Item: Eq,
{
    xs: I,
}

impl<I: Iterator> Iterator for RandomUniquePairs<I>
where
    I::Item: Eq,
{
    type Item = (I::Item, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut out_0 = None;
        let out_1;
        loop {
            let x = self.xs.next().unwrap();
            if let Some(out_0) = out_0.as_ref() {
                if x != *out_0 {
                    out_1 = x;
                    break;
                }
            } else {
                out_0 = Some(x);
            }
        }
        Some((out_0.unwrap(), out_1))
    }
}

/// Generates random pairs using elements from a single iterator, where the two elements of each
/// pair are unequal.
///
/// The input iterator must generate at least two distinct elements; otherwise, this iterator will
/// hang.
///
/// `xs` must be infinite.
#[inline]
pub const fn random_unique_pairs<I: Iterator>(xs: I) -> RandomUniquePairs<I>
where
    I::Item: Eq,
{
    RandomUniquePairs { xs }
}

/// Defines random unique tuple generators.
///
/// Malachite provides [`random_unique_pairs`], but you can also define `random_unique_triples`,
/// `random_unique_quadruples`, and so on, in your program using the code below.
///
/// See usage examples [here](self#random_unique_quadruples).
///
/// ```
/// use malachite_base::random_unique_tuples;
/// use std::collections::HashMap;
/// use std::hash::Hash;
///
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueTriples,
///     3,
///     (I::Item, I::Item, I::Item),
///     random_unique_triples,
///     [0, 1, 2]
/// );
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueQuadruples,
///     4,
///     (I::Item, I::Item, I::Item, I::Item),
///     random_unique_quadruples,
///     [0, 1, 2, 3]
/// );
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueQuintuples,
///     5,
///     (I::Item, I::Item, I::Item, I::Item, I::Item),
///     random_unique_quintuples,
///     [0, 1, 2, 3, 4]
/// );
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueSextuples,
///     6,
///     (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
///     random_unique_sextuples,
///     [0, 1, 2, 3, 4, 5]
/// );
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueSeptuples,
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
///     random_unique_septuples,
///     [0, 1, 2, 3, 4, 5, 6]
/// );
/// random_unique_tuples!(
///     (pub(crate)),
///     RandomOrderedUniqueOctuples,
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
///     random_unique_octuples,
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// );
/// ```
#[macro_export]
macro_rules! random_unique_tuples {
    (
        ($($vis:tt)*),
        $struct: ident,
        $k: expr,
        $out_t: ty,
        $fn: ident,
        [$($i: tt),*]
    ) => {
        #[derive(Clone, Debug)]
        $($vis)* struct $struct<I: Iterator> where I::Item: Eq + Hash {
            xs: I,
        }

        impl<I: Iterator> Iterator for $struct<I> where I::Item: Eq + Hash {
            type Item = $out_t;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let mut xs_to_indices = HashMap::with_capacity($k);
                let mut i = 0;
                while i < $k {
                    xs_to_indices
                        .entry(self.xs.next().unwrap())
                        .or_insert_with(|| {
                            i += 1;
                            i - 1
                        });
                }
                let mut out = ($((None, $i).0),*);
                for (x, i) in xs_to_indices {
                    match i {
                        $($i => {out.$i = Some(x)},)*
                        _ => {}
                    }
                }
                Some(($(out.$i.unwrap()),*))
            }
        }

        #[inline]
        $($vis)* fn $fn<I: Iterator>(xs: I) -> $struct<I> where I::Item: Eq + Hash,
        {
            $struct { xs }
        }
    }
}
