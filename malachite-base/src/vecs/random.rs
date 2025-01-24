// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::ExactFrom;
use crate::num::random::geometric::{
    geometric_random_unsigned_inclusive_range, geometric_random_unsigneds,
    GeometricRandomNaturalValues,
};
use crate::num::random::{
    random_unsigned_inclusive_range, random_unsigned_range, RandomUnsignedInclusiveRange,
    RandomUnsignedRange,
};
use crate::random::Seed;
use crate::sets::random::{
    random_b_tree_sets_fixed_length, random_b_tree_sets_from_length_iterator, RandomBTreeSets,
    RandomBTreeSetsFixedLength,
};
use crate::vecs::exhaustive::validate_oi_map;
use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{repeat, Repeat};

/// Generates random [`Vec`]s of a given length using elements from a single iterator.
///
/// This `struct` is created by [`random_vecs_fixed_length_from_single`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomFixedLengthVecsFromSingle<I: Iterator> {
    len: u64,
    xs: I,
}

impl<I: Iterator> Iterator for RandomFixedLengthVecsFromSingle<I> {
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some((&mut self.xs).take(usize::exact_from(self.len)).collect())
    }
}

/// Randomly generates [`Vec`]s of a given length using elements from a single iterator.
///
/// The probability of a particular length-$n$ [`Vec`] being generated is the product of the
/// probabilities of each of its elements.
///
/// If `len` is 0, the output consists of the empty list, repeated.
///
/// `xs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_fixed_length_from_single;
///
/// let xss = random_vecs_fixed_length_from_single(
///     2,
///     random_unsigned_inclusive_range::<u32>(EXAMPLE_SEED, 1, 100),
/// )
/// .take(10)
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[95, 24],
///         &[99, 71],
///         &[93, 53],
///         &[85, 34],
///         &[48, 2],
///         &[55, 11],
///         &[48, 18],
///         &[90, 93],
///         &[67, 93],
///         &[93, 95]
///     ]
/// );
/// ```
#[inline]
pub const fn random_vecs_fixed_length_from_single<I: Iterator>(
    len: u64,
    xs: I,
) -> RandomFixedLengthVecsFromSingle<I> {
    RandomFixedLengthVecsFromSingle { len, xs }
}

/// Defines random fixed-length [`Vec`] generators.
///
/// Malachite provides [`random_vecs_length_2`] and [`random_vecs_fixed_length_2_inputs`], but you
/// can also define `random_vecs_length_3`, `random_vecs_length_4`, and so on, and
/// `random_vecs_fixed_length_3_inputs`, `random_vecs_fixed_length_4_inputs`, and so on, in your
/// program using the code below. The documentation for [`random_vecs_length_2`] and
/// [`random_vecs_fixed_length_2_inputs`] describes these other functions as well.
///
/// See usage examples [here](self#lex_vecs_length_2) and
/// [here](self#random_vecs_fixed_length_2_inputs).
///
/// ```
/// use malachite_base::random::Seed;
/// use malachite_base::random_vecs_fixed_length;
/// use malachite_base::vecs::exhaustive::validate_oi_map;
///
/// random_vecs_fixed_length!(
///     (pub(crate)),
///     RandomFixedLengthVecs3Inputs,
///     random_vecs_fixed_length_3_inputs,
///     random_vecs_length_3,
///     [0, I, xs, xs_gen],
///     [1, J, ys, ys_gen],
///     [2, K, zs, zs_gen]
/// );
/// random_vecs_fixed_length!(
///     (pub(crate)),
///     RandomFixedLengthVecs4Inputs,
///     random_vecs_fixed_length_4_inputs,
///     random_vecs_length_4,
///     [0, I, xs, xs_gen],
///     [1, J, ys, ys_gen],
///     [2, K, zs, zs_gen],
///     [3, L, ws, ws_gen]
/// );
/// random_vecs_fixed_length!(
///     (pub(crate)),
///     RandomFixedLengthVecs5Inputs,
///     random_vecs_fixed_length_5_inputs,
///     random_vecs_length_5,
///     [0, I, xs, xs_gen],
///     [1, J, ys, ys_gen],
///     [2, K, zs, zs_gen],
///     [3, L, ws, ws_gen],
///     [4, M, vs, vs_gen]
/// );
/// random_vecs_fixed_length!(
///     (pub(crate)),
///     RandomFixedLengthVecs6Inputs,
///     random_vecs_fixed_length_6_inputs,
///     random_vecs_length_6,
///     [0, I, xs, xs_gen],
///     [1, J, ys, ys_gen],
///     [2, K, zs, zs_gen],
///     [3, L, ws, ws_gen],
///     [4, M, vs, vs_gen],
///     [5, N, us, us_gen]
/// );
/// random_vecs_fixed_length!(
///     (pub(crate)),
///     RandomFixedLengthVecs7Inputs,
///     random_vecs_fixed_length_7_inputs,
///     random_vecs_length_7,
///     [0, I, xs, xs_gen],
///     [1, J, ys, ys_gen],
///     [2, K, zs, zs_gen],
///     [3, L, ws, ws_gen],
///     [4, M, vs, vs_gen],
///     [5, N, us, us_gen],
///     [6, O, ts, ts_gen]
/// );
/// random_vecs_fixed_length!(
///     (pub(crate)),
///     RandomFixedLengthVecs8Inputs,
///     random_vecs_fixed_length_8_inputs,
///     random_vecs_length_8,
///     [0, I, xs, xs_gen],
///     [1, J, ys, ys_gen],
///     [2, K, zs, zs_gen],
///     [3, L, ws, ws_gen],
///     [4, M, vs, vs_gen],
///     [5, N, us, us_gen],
///     [6, O, ts, ts_gen],
///     [7, P, ss, ss_gen]
/// );
/// ```
#[macro_export]
macro_rules! random_vecs_fixed_length {
    (
        ($($vis:tt)*),
        $random_struct: ident,
        $random_fn: ident,
        $random_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_gen: ident]),*
    ) => {
        /// This documentation applies not only to `RandomFixedLengthVecs2Inputs`, but also to
        /// `RandomFixedLengthVecs3Inputs`, `RandomFixedLengthVecs4Inputs`, and so on. See
        /// [`random_vecs_fixed_length`] for more information.
        ///
        /// Generates random [`Vec`]s of a given length using elements from $m$ iterators.
        ///
        /// The fixed length $n$ of the [`Vec`]s is greater than or equal to $m$.
        #[derive(Clone, Debug)]
        $($vis)* struct $random_struct<T, $($it: Iterator<Item = T>),*> {
            $($xs: $it,)*
            output_to_input_map: Vec<usize>,
        }

        impl<T, $($it: Iterator<Item = T>),*> Iterator
            for $random_struct<T, $($it),*>
        {
            type Item = Vec<T>;

            #[inline]
            fn next(&mut self) -> Option<Vec<T>> {
                let mut out = Vec::with_capacity(self.output_to_input_map.len());
                for &i in &self.output_to_input_map {
                    out.push(
                        match i {
                            $(
                                $i => self.$xs.next(),
                            )*
                            _ => unreachable!(),
                        }
                        .unwrap(),
                    );
                }
                Some(out)
            }
        }

        /// This documentation applies not only to `random_vecs_fixed_length_2_inputs`, but also to
        /// `random_vecs_fixed_length_3_inputs`, `random_vecs_fixed_length_4_inputs`, and so on. See
        /// [`random_vecs_fixed_length`] for more information.
        ///
        /// Generates random length-$n$ [`Vec`]s using elements from $m$ iterators, where $m \leq
        /// n$.
        ///
        /// The `output_to_input_map` parameter defines which iterators are mapped to which slot in
        /// the output [`Vec`]s. The length of the output [`Vec`]s, $n$, is specified by the length
        /// of `output_to_input_map`.
        ///
        /// The $i$th element of `output_to_input_map` is an index from 0 to $m-1$ which specifies
        /// which iterator the $i$th output slot is populated with. Together, the elements must
        /// include all indices from 0 to $m-1$, inclusive, possibly with repetitions.
        ///
        /// `xs` must be infinite.
        ///
        /// # Examples
        /// See [here](self#random_vecs_fixed_length_2_inputs).
        #[allow(dead_code)]
        $($vis)* fn $random_fn<T, $($it: Iterator<Item = T>),*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
            output_to_input_map: &[usize],
        ) -> $random_struct<T, $($it),*> {
            $(
                let _max_input_index = $i;
            )*
            validate_oi_map(_max_input_index, output_to_input_map.iter().cloned());
            $random_struct {
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
                output_to_input_map: output_to_input_map.to_vec(),
            }
        }

        /// This documentation applies not only to `random_vecs_length_2`, but also to
        /// `random_vecs_length_3`, `random_vecs_length_4`, and so on. See
        /// [`random_vecs_fixed_length`] for more information.
        ///
        /// Generates random length-$n$ [`Vec`]s with elements from $n$ iterators.
        ///
        /// The probability of a particular length-$n$ [`Vec`] being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs`, `ys`, `zs`, ... must be infinite.
        ///
        /// # Examples
        /// See [here](self#random_vecs_length_2).
        #[allow(dead_code)]
        #[inline]
        $($vis)* fn $random_1_to_1_fn<T, $($it: Iterator<Item = T>),*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
        ) -> $random_struct<T, $($it),*> {
            $random_fn(seed, $($xs_gen,)* &[$($i),*])
        }
    }
}

random_vecs_fixed_length!(
    (pub),
    RandomFixedLengthVecs2Inputs,
    random_vecs_fixed_length_2_inputs,
    random_vecs_length_2,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen]
);

/// Generates random [`Vec`]s using elements from an iterator and with lengths from another
/// iterator.
#[derive(Clone, Debug)]
pub struct RandomVecs<T, I: Iterator<Item = u64>, J: Iterator<Item = T>> {
    lengths: I,
    xs: J,
}

impl<T, I: Iterator<Item = u64>, J: Iterator<Item = T>> Iterator for RandomVecs<T, I, J> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        Some(
            (&mut self.xs)
                .take(usize::exact_from(self.lengths.next().unwrap()))
                .collect(),
        )
    }
}

/// Generates random [`Vec`]s using elements from an iterator and with lengths from another
/// iterator.
///
/// The probability of a particular [`Vec`] being generated is the product of the probabilities of
/// each of its elements, multiplied by the probability of its length being generated.
///
/// `lengths` and `xs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_from_length_iterator;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let xs = random_vecs_from_length_iterator(
///     EXAMPLE_SEED,
///     &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///     &random_primitive_ints::<u8>,
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11][..],
///         &[136, 200, 235, 134],
///         &[203, 223],
///         &[38, 235, 217, 177],
///         &[162, 32, 166, 234],
///         &[30, 218],
///         &[],
///         &[90, 106],
///         &[],
///         &[9, 216, 204, 151],
///         &[213, 97, 253, 78],
///         &[91, 39],
///         &[191, 175, 170, 232],
///         &[233, 2],
///         &[35, 22, 217, 198],
///         &[114, 17, 32, 173],
///         &[114, 65, 121, 222],
///         &[],
///         &[173, 25, 144, 148],
///         &[]
///     ]
/// );
/// ```
#[inline]
pub fn random_vecs_from_length_iterator<T, I: Iterator<Item = u64>, J: Iterator<Item = T>>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
) -> RandomVecs<T, I, J> {
    RandomVecs {
        lengths: lengths_gen(seed.fork("lengths")),
        xs: xs_gen(seed.fork("xs")),
    }
}

/// Generates random [`Vec`]s using elements from an iterator.
///
/// The lengths of the [`Vec`]s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = \frac{m^n}{(m+1)^{n+1}}\prod_{i=0}^{n-1}P(x_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs;
///
/// let xs = random_vecs(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32],
///         &[166, 234, 30, 218],
///         &[90, 106, 9, 216],
///         &[204],
///         &[],
///         &[151, 213, 97, 253, 78],
///         &[91, 39],
///         &[191, 175, 170, 232],
///         &[],
///         &[233, 2, 35, 22, 217, 198],
///         &[],
///         &[],
///         &[114, 17, 32, 173, 114, 65, 121, 222, 173, 25, 144],
///         &[148, 79, 115, 52, 73, 69, 137, 91],
///         &[],
///         &[153, 178, 112],
///         &[],
///         &[34, 95, 106, 167, 197],
///         &[130, 168, 122, 207, 172, 177, 86, 150, 221]
///     ]
/// );
/// ```
#[inline]
pub fn random_vecs<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomVecs<I::Item, GeometricRandomNaturalValues<u64>, I> {
    random_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        xs_gen,
    )
}

/// Generates random [`Vec`]s with a minimum length, using elements from an iterator.
///
/// The lengths of the [`Vec`]s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than
/// `min_length`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = \\begin{cases}
///     \frac{(m-a)^{n-a}}{(m+1-a)^{n+1-a}}\prod_{i=0}^{n-1}P(x_i) & \text{if} \\quad n\geq a, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $a$ is `min_length`.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, if their ratio is less
/// than or equal to `min_length`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_min_length;
///
/// let xs = random_vecs_min_length(EXAMPLE_SEED, 2, &random_primitive_ints::<u8>, 6, 1);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11][..],
///         &[136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32, 166, 234, 30, 218],
///         &[90, 106, 9, 216, 204, 151],
///         &[213, 97, 253, 78, 91, 39],
///         &[191, 175, 170],
///         &[232, 233],
///         &[2, 35, 22, 217, 198, 114, 17],
///         &[32, 173, 114, 65],
///         &[121, 222, 173, 25, 144, 148],
///         &[79, 115],
///         &[52, 73, 69, 137, 91, 153, 178, 112],
///         &[34, 95],
///         &[106, 167],
///         &[197, 130, 168, 122, 207, 172, 177, 86, 150, 221, 218, 101, 115],
///         &[74, 9, 123, 109, 52, 201, 159, 247, 250, 48],
///         &[133, 235],
///         &[196, 40, 97, 104, 68],
///         &[190, 216],
///         &[7, 216, 157, 43, 43, 112, 217],
///         &[24, 11, 103, 211, 84, 135, 55, 29, 206, 89, 65]
///     ]
/// );
/// ```
#[inline]
pub fn random_vecs_min_length<I: Iterator>(
    seed: Seed,
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomVecs<I::Item, GeometricRandomNaturalValues<u64>, I> {
    random_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        xs_gen,
    )
}

/// Generates random [`Vec`]s with lengths in $[a, b)$, using elements from an iterator.
///
/// The lengths of the [`Vec`]s are sampled from a uniform distribution on $[a, b)$. $a$ must be
/// less than $b$.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = \\begin{cases}
///     \frac{1}{b-a}\prod_{i=0}^{n-1}P(x_i) & \text{if} \\quad a \leq n < b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_length_range;
///
/// let xs = random_vecs_length_range(EXAMPLE_SEED, 2, 5, &random_primitive_ints::<u8>);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11, 136][..],
///         &[200, 235, 134, 203],
///         &[223, 38, 235],
///         &[217, 177, 162, 32],
///         &[166, 234, 30, 218],
///         &[90, 106, 9],
///         &[216, 204],
///         &[151, 213, 97],
///         &[253, 78],
///         &[91, 39, 191, 175],
///         &[170, 232, 233, 2],
///         &[35, 22, 217],
///         &[198, 114, 17, 32],
///         &[173, 114, 65],
///         &[121, 222, 173, 25],
///         &[144, 148, 79, 115],
///         &[52, 73, 69, 137],
///         &[91, 153],
///         &[178, 112, 34, 95],
///         &[106, 167]
///     ]
/// );
/// ```
#[inline]
pub fn random_vecs_length_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomVecs<I::Item, RandomUnsignedRange<u64>, I> {
    random_vecs_from_length_iterator(seed, &|seed_2| random_unsigned_range(seed_2, a, b), xs_gen)
}

/// Generates random [`Vec`]s with lengths in $[a, b]$, using elements from an iterator.
///
/// The lengths of the [`Vec`]s are sampled from a uniform distribution on $[a, b]$. $a$ must be
/// less than or equal to $b$.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = \\begin{cases}
///     \frac{1}{b-a+1}\prod_{i=0}^{n-1}P(x_i) & \text{if} \\quad a \leq n \leq b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_vecs_length_inclusive_range;
///
/// let xs = random_vecs_length_inclusive_range(EXAMPLE_SEED, 2, 4, &random_primitive_ints::<u8>);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11, 136][..],
///         &[200, 235, 134, 203],
///         &[223, 38, 235],
///         &[217, 177, 162, 32],
///         &[166, 234, 30, 218],
///         &[90, 106, 9],
///         &[216, 204],
///         &[151, 213, 97],
///         &[253, 78],
///         &[91, 39, 191, 175],
///         &[170, 232, 233, 2],
///         &[35, 22, 217],
///         &[198, 114, 17, 32],
///         &[173, 114, 65],
///         &[121, 222, 173, 25],
///         &[144, 148, 79, 115],
///         &[52, 73, 69, 137],
///         &[91, 153],
///         &[178, 112, 34, 95],
///         &[106, 167]
///     ]
/// );
/// ```
#[inline]
pub fn random_vecs_length_inclusive_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomVecs<I::Item, RandomUnsignedInclusiveRange<u64>, I> {
    random_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        xs_gen,
    )
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct RandomOrderedUniqueVecsLength2<I: Iterator>
where
    I::Item: Ord,
{
    xs: I,
}

impl<I: Iterator> Iterator for RandomOrderedUniqueVecsLength2<I>
where
    I::Item: Ord,
{
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        let mut out = Vec::with_capacity(2);
        loop {
            let x = self.xs.next().unwrap();
            if out.is_empty() {
                out.push(x);
            } else {
                match x.cmp(&out[0]) {
                    Equal => {}
                    Greater => {
                        out.push(x);
                        break;
                    }
                    Less => {
                        out.insert(0, x);
                        break;
                    }
                }
            }
        }
        Some(out)
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct RandomOrderedUniqueVecsFixedLengthGreaterThan2<I: Iterator>
where
    I::Item: Ord,
{
    xs: RandomBTreeSetsFixedLength<I>,
}

impl<I: Iterator> Iterator for RandomOrderedUniqueVecsFixedLengthGreaterThan2<I>
where
    I::Item: Ord,
{
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some(self.xs.next().unwrap().into_iter().collect())
    }
}

/// Generates random [`Vec`]s of a fixed length, where the [`Vec`]s have no repeated elements, and
/// the elements are in ascending order.
///
/// This `struct` is created by [`random_ordered_unique_vecs_fixed_length`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub enum RandomOrderedUniqueVecsFixedLength<I: Iterator>
where
    I::Item: Ord,
{
    Zero,
    One(I),
    Two(RandomOrderedUniqueVecsLength2<I>),
    GreaterThan2(RandomOrderedUniqueVecsFixedLengthGreaterThan2<I>),
}

impl<I: Iterator> Iterator for RandomOrderedUniqueVecsFixedLength<I>
where
    I::Item: Ord,
{
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            RandomOrderedUniqueVecsFixedLength::Zero => Some(vec![]),
            RandomOrderedUniqueVecsFixedLength::One(ref mut xs) => xs.next().map(|x| vec![x]),
            RandomOrderedUniqueVecsFixedLength::Two(ref mut xs) => xs.next(),
            RandomOrderedUniqueVecsFixedLength::GreaterThan2(ref mut xs) => xs.next(),
        }
    }
}

/// Randomly generates [`Vec`]s of a given length, where the [`Vec`]s have no repeated elements, and
/// the elements are in ascending order.
///
/// The input iterator must generate at least `len` distinct elements; otherwise, this iterator will
/// hang.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// The above formula assumes that the [`Vec`] is valid, \emph{i.e.} its elements are strictly
/// increasing. The probability of an invalid [`Vec`] is zero.
///
/// If `len` is 0, the output consists of the empty list, repeated.
///
/// `xs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_ordered_unique_vecs_fixed_length;
///
/// let xss = random_ordered_unique_vecs_fixed_length(
///     2,
///     random_unsigned_inclusive_range::<u32>(EXAMPLE_SEED, 1, 100),
/// )
/// .take(10)
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[24, 95],
///         &[71, 99],
///         &[53, 93],
///         &[34, 85],
///         &[2, 48],
///         &[11, 55],
///         &[18, 48],
///         &[90, 93],
///         &[67, 93],
///         &[93, 95]
///     ]
/// );
/// ```
#[inline]
pub fn random_ordered_unique_vecs_fixed_length<I: Iterator>(
    len: u64,
    xs: I,
) -> RandomOrderedUniqueVecsFixedLength<I>
where
    I::Item: Ord,
{
    match len {
        0 => RandomOrderedUniqueVecsFixedLength::Zero,
        1 => RandomOrderedUniqueVecsFixedLength::One(xs),
        2 => RandomOrderedUniqueVecsFixedLength::Two(RandomOrderedUniqueVecsLength2 { xs }),
        len => RandomOrderedUniqueVecsFixedLength::GreaterThan2(
            RandomOrderedUniqueVecsFixedLengthGreaterThan2 {
                xs: random_b_tree_sets_fixed_length(len, xs),
            },
        ),
    }
}

/// Generates random [`Vec`]s with lengths from an iterator, where the [`Vec`]s have no repeated
/// elements, and the elements are in ascending order.
#[derive(Clone, Debug)]
pub struct RandomOrderedUniqueVecs<T: Ord, I: Iterator<Item = u64>, J: Iterator<Item = T>> {
    xs: RandomBTreeSets<T, I, J>,
}

impl<T: Ord, I: Iterator<Item = u64>, J: Iterator<Item = T>> Iterator
    for RandomOrderedUniqueVecs<T, I, J>
{
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Vec<T>> {
        Some(self.xs.next().unwrap().into_iter().collect())
    }
}

/// Generates random [`Vec`]s using elements from an iterator and with lengths from another
/// iterator, where the [`Vec`]s have no repeated elements, and the elements are in ascending order.
///
/// The input iterator must generate at least many distinct elements as any number generated by the
/// lengths iterator; otherwise, this iterator will hang.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P(n)\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// The above formula assumes that the [`Vec`] is valid, \emph{i.e.} its elements are strictly
/// increasing. The probability of an invalid [`Vec`] is zero.
///
/// `lengths` and `xs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_ordered_unique_vecs_from_length_iterator;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let xs = random_ordered_unique_vecs_from_length_iterator(
///     EXAMPLE_SEED,
///     &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///     &random_primitive_ints::<u8>,
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[11, 85][..],
///         &[134, 136, 200, 235],
///         &[203, 223],
///         &[38, 177, 217, 235],
///         &[32, 162, 166, 234],
///         &[30, 218],
///         &[],
///         &[90, 106],
///         &[],
///         &[9, 151, 204, 216],
///         &[78, 97, 213, 253],
///         &[39, 91],
///         &[170, 175, 191, 232],
///         &[2, 233],
///         &[22, 35, 198, 217],
///         &[17, 32, 114, 173],
///         &[65, 114, 121, 222],
///         &[],
///         &[25, 144, 148, 173],
///         &[]
///     ]
/// );
/// ```
#[inline]
pub fn random_ordered_unique_vecs_from_length_iterator<
    T: Ord,
    I: Iterator<Item = u64>,
    J: Iterator<Item = T>,
>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
) -> RandomOrderedUniqueVecs<T, I, J> {
    RandomOrderedUniqueVecs {
        xs: random_b_tree_sets_from_length_iterator(seed, lengths_gen, xs_gen),
    }
}

/// Generates random [`Vec`]s using elements from an iterator, where the [`Vec`]s have no repeated
/// elements, and the elements are in ascending order.
///
/// The lengths of the [`Vec`]s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if `mean_length_numerator /
/// mean_length_denominator` is significantly lower than 256, then it's ok to use
/// `random_unsigneds::<u8>`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P_g(n)\prod\_{i=0}^{n-1}P(x\_i),
/// $$
/// where $P_g(n)$ is the probability function described in [`geometric_random_unsigneds`].
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_ordered_unique_vecs;
///
/// let xs = random_ordered_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235],
///         &[30, 90, 218, 234],
///         &[9, 106, 204, 216],
///         &[151],
///         &[],
///         &[78, 91, 97, 213, 253],
///         &[39, 191],
///         &[170, 175, 232, 233],
///         &[],
///         &[2, 22, 35, 114, 198, 217],
///         &[],
///         &[],
///         &[17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222],
///         &[52, 69, 73, 91, 115, 137, 153, 178],
///         &[],
///         &[34, 95, 112],
///         &[],
///         &[106, 130, 167, 168, 197],
///         &[86, 101, 122, 150, 172, 177, 207, 218, 221]
///     ]
/// );
/// ```
#[inline]
pub fn random_ordered_unique_vecs<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomOrderedUniqueVecs<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Ord,
{
    random_ordered_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        xs_gen,
    )
}

/// Generates random [`Vec`]s with a minimum length, using elements from an iterator, where the
/// [`Vec`]s have no repeated elements, and the elements are in ascending order.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if `mean_length_numerator /
/// mean_length_denominator` is significantly lower than 256, then it's ok to use
/// `random_unsigneds::<u8>`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P_g(n)\prod\_{i=0}^{n-1}P(x\_i),
/// $$
/// where $P_g(n)$ is the probability function described in
/// [`geometric_random_unsigned_inclusive_range`], with $a$ equal to `min_length` and `b` to
/// `u64::MAX`.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, if their ratio is less
/// than or equal to `min_length`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_ordered_unique_vecs_min_length;
///
/// let xs =
///     random_ordered_unique_vecs_min_length(EXAMPLE_SEED, 2, &random_primitive_ints::<u8>, 6, 1);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[11, 85][..],
///         &[30, 32, 38, 90, 134, 136, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235],
///         &[9, 106, 151, 204, 213, 216],
///         &[39, 78, 91, 97, 191, 253],
///         &[170, 175, 232],
///         &[2, 233],
///         &[17, 22, 32, 35, 114, 198, 217],
///         &[65, 114, 121, 173],
///         &[25, 79, 144, 148, 173, 222],
///         &[52, 115],
///         &[34, 69, 73, 91, 112, 137, 153, 178],
///         &[95, 106],
///         &[167, 197],
///         &[74, 86, 101, 115, 122, 130, 150, 168, 172, 177, 207, 218, 221],
///         &[9, 48, 52, 109, 123, 133, 159, 201, 247, 250],
///         &[196, 235],
///         &[40, 68, 97, 104, 190],
///         &[7, 216],
///         &[11, 24, 43, 112, 157, 216, 217],
///         &[29, 51, 55, 65, 84, 89, 103, 135, 191, 206, 211]
///     ]
/// );
/// ```
#[inline]
pub fn random_ordered_unique_vecs_min_length<I: Iterator>(
    seed: Seed,
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomOrderedUniqueVecs<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Ord,
{
    random_ordered_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        xs_gen,
    )
}

/// Generates random [`Vec`]s with lengths in $[a, b)$, using elements from an iterator, where the
/// [`Vec`]s have no repeated elements, and the elements are in ascending order.
///
/// The lengths of the [`Vec`]s are sampled from a uniform distribution on $[a, b)$. $a$ must be
/// less than $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}, a, b) = \frac{n!}{b - a}\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_ordered_unique_vecs_length_range;
///
/// let xs =
///     random_ordered_unique_vecs_length_range(EXAMPLE_SEED, 2, 5, &random_primitive_ints::<u8>);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[11, 85, 136][..],
///         &[134, 200, 203, 235],
///         &[38, 223, 235],
///         &[32, 162, 177, 217],
///         &[30, 166, 218, 234],
///         &[9, 90, 106],
///         &[204, 216],
///         &[97, 151, 213],
///         &[78, 253],
///         &[39, 91, 175, 191],
///         &[2, 170, 232, 233],
///         &[22, 35, 217],
///         &[17, 32, 114, 198],
///         &[65, 114, 173],
///         &[25, 121, 173, 222],
///         &[79, 115, 144, 148],
///         &[52, 69, 73, 137],
///         &[91, 153],
///         &[34, 95, 112, 178],
///         &[106, 167]
///     ]
/// );
/// ```
#[inline]
pub fn random_ordered_unique_vecs_length_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomOrderedUniqueVecs<I::Item, RandomUnsignedRange<u64>, I>
where
    I::Item: Ord,
{
    random_ordered_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        xs_gen,
    )
}

/// Generates random [`Vec`]s with lengths in $[a, b]$, using elements from an iterator, where the
/// [`Vec`]s have no repeated elements, and the elements are in ascending order.
///
/// The lengths of the [`Vec`]s are sampled from a uniform distribution on $[a, b]$. $a$ must be
/// less than or equal to $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}, a, b) = \frac{n!}{b - a + 1}\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_ordered_unique_vecs_length_inclusive_range;
///
/// let xs = random_ordered_unique_vecs_length_inclusive_range(
///     EXAMPLE_SEED,
///     2,
///     4,
///     &random_primitive_ints::<u8>,
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[11, 85, 136][..],
///         &[134, 200, 203, 235],
///         &[38, 223, 235],
///         &[32, 162, 177, 217],
///         &[30, 166, 218, 234],
///         &[9, 90, 106],
///         &[204, 216],
///         &[97, 151, 213],
///         &[78, 253],
///         &[39, 91, 175, 191],
///         &[2, 170, 232, 233],
///         &[22, 35, 217],
///         &[17, 32, 114, 198],
///         &[65, 114, 173],
///         &[25, 121, 173, 222],
///         &[79, 115, 144, 148],
///         &[52, 69, 73, 137],
///         &[91, 153],
///         &[34, 95, 112, 178],
///         &[106, 167]
///     ]
/// );
/// ```
#[inline]
pub fn random_ordered_unique_vecs_length_inclusive_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomOrderedUniqueVecs<I::Item, RandomUnsignedInclusiveRange<u64>, I>
where
    I::Item: Ord,
{
    random_ordered_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        xs_gen,
    )
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct RandomUniqueVecsLength2<I: Iterator>
where
    I::Item: Eq,
{
    xs: I,
}

impl<I: Iterator> Iterator for RandomUniqueVecsLength2<I>
where
    I::Item: Eq,
{
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        let mut out = Vec::with_capacity(2);
        loop {
            let x = self.xs.next().unwrap();
            if out.is_empty() {
                out.push(x);
            } else if x != out[0] {
                out.push(x);
                return Some(out);
            }
        }
    }
}

/// Generates random [`Vec`]s with lengths from an iterator, where the [`Vec`]s have no repeated
/// elements.
#[derive(Clone, Debug)]
pub struct RandomUniqueVecs<T: Eq + Hash, I: Iterator<Item = u64>, J: Iterator<Item = T>> {
    lengths: I,
    xs: J,
}

impl<T: Eq + Hash, I: Iterator<Item = u64>, J: Iterator<Item = T>> Iterator
    for RandomUniqueVecs<T, I, J>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        // We avoid cloning the T values. We first move them into a HashMap, then into a
        // Vec<Option<T>>, then into the output Vec<T>.
        let len = usize::exact_from(self.lengths.next().unwrap());
        let mut xs_to_indices = HashMap::with_capacity(len);
        let mut i = 0;
        while i < len {
            xs_to_indices
                .entry(self.xs.next().unwrap())
                .or_insert_with(|| {
                    i += 1;
                    i - 1
                });
        }
        let mut out = Vec::with_capacity(len);
        out.resize_with(len, || None);
        for (x, i) in xs_to_indices {
            out[i] = Some(x);
        }
        Some(out.into_iter().map(Option::unwrap).collect())
    }
}

/// Generates random [`Vec`]s using elements from an iterator and with lengths from another
/// iterator, where the [`Vec`]s have no repeated elements.
///
/// The input iterator must generate at least many distinct elements as any number generated by the
/// lengths iterator; otherwise, this iterator will hang.
///
/// `lengths` and `xs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_unique_vecs_from_length_iterator;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let xs = random_unique_vecs_from_length_iterator(
///     EXAMPLE_SEED,
///     &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///     &random_primitive_ints::<u8>,
/// );
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11][..],
///         &[136, 200, 235, 134],
///         &[203, 223],
///         &[38, 235, 217, 177],
///         &[162, 32, 166, 234],
///         &[30, 218],
///         &[],
///         &[90, 106],
///         &[],
///         &[9, 216, 204, 151],
///         &[213, 97, 253, 78],
///         &[91, 39],
///         &[191, 175, 170, 232],
///         &[233, 2],
///         &[35, 22, 217, 198],
///         &[114, 17, 32, 173],
///         &[114, 65, 121, 222],
///         &[],
///         &[173, 25, 144, 148],
///         &[]
///     ]
/// );
/// ```
#[inline]
pub fn random_unique_vecs_from_length_iterator<
    T: Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = T>,
>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
) -> RandomUniqueVecs<T, I, J> {
    RandomUniqueVecs {
        lengths: lengths_gen(seed.fork("lengths")),
        xs: xs_gen(seed.fork("xs")),
    }
}

/// Generates random [`Vec`]s of a fixed length, where the [`Vec`]s have no repeated elements.
///
/// This `enum` is created by [`random_unique_vecs_fixed_length`]; see its documentation for more.
#[derive(Clone, Debug)]
pub enum RandomUniqueVecsFixedLength<I: Iterator>
where
    I::Item: Eq + Hash,
{
    Zero,
    One(I),
    Two(RandomUniqueVecsLength2<I>),
    GreaterThan2(RandomUniqueVecs<I::Item, Repeat<u64>, I>),
}

impl<I: Iterator> Iterator for RandomUniqueVecsFixedLength<I>
where
    I::Item: Eq + Hash,
{
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            RandomUniqueVecsFixedLength::Zero => Some(vec![]),
            RandomUniqueVecsFixedLength::One(ref mut xs) => xs.next().map(|x| vec![x]),
            RandomUniqueVecsFixedLength::Two(ref mut xs) => xs.next(),
            RandomUniqueVecsFixedLength::GreaterThan2(ref mut xs) => xs.next(),
        }
    }
}

/// Randomly generates [`Vec`]s of a given length, where the [`Vec`]s have no repeated elements.
///
/// The input iterator must generate at least `len` distinct elements; otherwise, this iterator will
/// hang.
///
/// If `len` is 0, the output consists of the empty list, repeated.
///
/// `xs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_unique_vecs_fixed_length;
///
/// let xss = random_unique_vecs_fixed_length(
///     2,
///     random_unsigned_inclusive_range::<u32>(EXAMPLE_SEED, 1, 100),
/// )
/// .take(10)
/// .collect_vec();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[95, 24],
///         &[99, 71],
///         &[93, 53],
///         &[85, 34],
///         &[48, 2],
///         &[55, 11],
///         &[48, 18],
///         &[90, 93],
///         &[67, 93],
///         &[93, 95]
///     ]
/// );
/// ```
#[inline]
pub fn random_unique_vecs_fixed_length<I: Iterator>(
    len: u64,
    xs: I,
) -> RandomUniqueVecsFixedLength<I>
where
    I::Item: Eq + Hash,
{
    match len {
        0 => RandomUniqueVecsFixedLength::Zero,
        1 => RandomUniqueVecsFixedLength::One(xs),
        2 => RandomUniqueVecsFixedLength::Two(RandomUniqueVecsLength2 { xs }),
        len => RandomUniqueVecsFixedLength::GreaterThan2(RandomUniqueVecs {
            lengths: repeat(len),
            xs,
        }),
    }
}

/// Generates random [`Vec`]s using elements from an iterator, where the [`Vec`]s have no repeated
/// elements.
///
/// The lengths of the [`Vec`]s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if `mean_length_numerator /
/// mean_length_denominator` is significantly lower than 256, then it's ok to use
/// `random_unsigneds::<u8>`.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_unique_vecs;
///
/// let xs = random_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[][..],
///         &[85, 11, 136, 200, 235, 134, 203, 223, 38, 217, 177, 162, 32, 166],
///         &[234, 30, 218, 90],
///         &[106, 9, 216, 204],
///         &[151],
///         &[],
///         &[213, 97, 253, 78, 91],
///         &[39, 191],
///         &[175, 170, 232, 233],
///         &[],
///         &[2, 35, 22, 217, 198, 114],
///         &[],
///         &[],
///         &[17, 32, 173, 114, 65, 121, 222, 25, 144, 148, 79],
///         &[115, 52, 73, 69, 137, 91, 153, 178],
///         &[],
///         &[112, 34, 95],
///         &[],
///         &[106, 167, 197, 130, 168],
///         &[122, 207, 172, 177, 86, 150, 221, 218, 101]
///     ]
/// );
/// ```
#[inline]
pub fn random_unique_vecs<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomUniqueVecs<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        xs_gen,
    )
}

/// Generates random [`Vec`]s with a minimum length, using elements from an iterator, where the
/// [`Vec`]s have no repeated elements.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if `mean_length_numerator /
/// mean_length_denominator` is significantly lower than 256, then it's ok to use
/// `random_unsigneds::<u8>`.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, if their ratio is less
/// than or equal to `min_length`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_unique_vecs_min_length;
///
/// let xs = random_unique_vecs_min_length(EXAMPLE_SEED, 2, &random_primitive_ints::<u8>, 6, 1);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11][..],
///         &[136, 200, 235, 134, 203, 223, 38, 217, 177, 162, 32, 166, 234, 30, 218, 90],
///         &[106, 9, 216, 204, 151, 213],
///         &[97, 253, 78, 91, 39, 191],
///         &[175, 170, 232],
///         &[233, 2],
///         &[35, 22, 217, 198, 114, 17, 32],
///         &[173, 114, 65, 121],
///         &[222, 173, 25, 144, 148, 79],
///         &[115, 52],
///         &[73, 69, 137, 91, 153, 178, 112, 34],
///         &[95, 106],
///         &[167, 197],
///         &[130, 168, 122, 207, 172, 177, 86, 150, 221, 218, 101, 115, 74],
///         &[9, 123, 109, 52, 201, 159, 247, 250, 48, 133],
///         &[235, 196],
///         &[40, 97, 104, 68, 190],
///         &[216, 7],
///         &[216, 157, 43, 112, 217, 24, 11],
///         &[103, 211, 84, 135, 55, 29, 206, 89, 65, 191, 51]
///     ]
/// );
/// ```
#[inline]
pub fn random_unique_vecs_min_length<I: Iterator>(
    seed: Seed,
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomUniqueVecs<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        xs_gen,
    )
}

/// Generates random [`Vec`]s with lengths in $[a, b)$, using elements from an iterator, where the
/// [`Vec`]s have no repeated elements.
///
/// The lengths of the [`Vec`]s are sampled from a uniform distribution on $[a, b)$. $a$ must be
/// less than $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_unique_vecs_length_range;
///
/// let xs = random_unique_vecs_length_range(EXAMPLE_SEED, 2, 5, &random_primitive_ints::<u8>);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11, 136][..],
///         &[200, 235, 134, 203],
///         &[223, 38, 235],
///         &[217, 177, 162, 32],
///         &[166, 234, 30, 218],
///         &[90, 106, 9],
///         &[216, 204],
///         &[151, 213, 97],
///         &[253, 78],
///         &[91, 39, 191, 175],
///         &[170, 232, 233, 2],
///         &[35, 22, 217],
///         &[198, 114, 17, 32],
///         &[173, 114, 65],
///         &[121, 222, 173, 25],
///         &[144, 148, 79, 115],
///         &[52, 73, 69, 137],
///         &[91, 153],
///         &[178, 112, 34, 95],
///         &[106, 167]
///     ]
/// );
/// ```
#[inline]
pub fn random_unique_vecs_length_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomUniqueVecs<I::Item, RandomUnsignedRange<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        xs_gen,
    )
}

/// Generates random [`Vec`]s with lengths in $[a, b]$, using elements from an iterator, where the
/// [`Vec`]s have no repeated elements.
///
/// The lengths of the [`Vec`]s are sampled from a uniform distribution on $[a, b]$. $a$ must be
/// less than or equal to $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_unique_vecs_length_inclusive_range;
///
/// let xs =
///     random_unique_vecs_length_inclusive_range(EXAMPLE_SEED, 2, 4, &random_primitive_ints::<u8>);
/// let values = xs.take(20).collect_vec();
/// assert_eq!(
///     values.iter().map(Vec::as_slice).collect_vec().as_slice(),
///     &[
///         &[85, 11, 136][..],
///         &[200, 235, 134, 203],
///         &[223, 38, 235],
///         &[217, 177, 162, 32],
///         &[166, 234, 30, 218],
///         &[90, 106, 9],
///         &[216, 204],
///         &[151, 213, 97],
///         &[253, 78],
///         &[91, 39, 191, 175],
///         &[170, 232, 233, 2],
///         &[35, 22, 217],
///         &[198, 114, 17, 32],
///         &[173, 114, 65],
///         &[121, 222, 173, 25],
///         &[144, 148, 79, 115],
///         &[52, 73, 69, 137],
///         &[91, 153],
///         &[178, 112, 34, 95],
///         &[106, 167]
///     ]
/// );
/// ```
#[inline]
pub fn random_unique_vecs_length_inclusive_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomUniqueVecs<I::Item, RandomUnsignedInclusiveRange<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_unique_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        xs_gen,
    )
}
