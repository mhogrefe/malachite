use std::cmp::max;
use std::iter::{once, repeat, Once};
use std::marker::PhantomData;

use itertools::Itertools;

use iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use iterators::iterator_cache::IteratorCache;
use num::arithmetic::traits::CheckedPow;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::exhaustive::{
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
    PrimitiveIntIncreasingRange,
};
use num::iterators::{ruler_sequence, RulerSequence};
use num::logic::traits::SignificantBits;
use tuples::exhaustive::{
    exhaustive_dependent_pairs_stop_after_empty_ys, lex_dependent_pairs_stop_after_empty_ys,
    ExhaustiveDependentPairs, ExhaustiveDependentPairsYsGenerator, LexDependentPairs,
};

pub(crate) fn validate_oi_map<I: Iterator<Item = usize>>(max_input_index: usize, xs: I) {
    let oi_sorted_unique = xs.unique().sorted().collect::<Vec<_>>();
    assert_eq!(oi_sorted_unique.len(), max_input_index + 1);
    assert_eq!(*oi_sorted_unique.first().unwrap(), 0);
    assert_eq!(*oi_sorted_unique.last().unwrap(), max_input_index);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct LexFixedLengthVecsOutput {
    input_index: usize,
    counter: usize,
}

macro_rules! lex_fixed_length_vecs {
    (
        $exhaustive_struct: ident,
        $exhaustive_custom_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_outputs: ident]),*
    ) => {
        /// Generates all `Vec`s of a given length with elements from $m$ iterators, in
        /// lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// The fixed length $n$ of the `Vec`s is greater than or equal to $m$.
        ///
        /// This struct is macro-generated. The value of $m$ is in the struct's name. Remember that
        /// $m$ is the number of input iterators, not the length of the output `Vec`s!
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<T: Clone, $($it: Iterator<Item = T>,)*> {
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
                    } else {
                        output.counter = 0;
                    }
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

        /// Generates all length-$n$ `Vec`s with elements from $m$ iterators, where $m \leq n$, in
        /// lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// The `output_to_input_map` parameter defines which iterators are mapped to which slot in
        /// the output `Vec`s. The length of the output `Vec`s, $n$, is specified by the length of
        /// `output_to_input_map`.
        ///
        /// The $i$th element of `output_to_input_map` is an index from 0 to $m-1$ which specifies
        /// which iterator the $i$th output slot is populated with. Together, the elements must
        /// include all indices from 0 to $m-1$, inclusive, possibly with repetitions.
        ///
        /// This function is macro-generated. The value of $m$ is in the function's name. Remember
        /// that $m$ is the number of input iterators, not the length of the output `Vec`s!
        ///
        /// Let `xs` be the input iterator mapped to the first slot of the output `Vec`s. All the
        /// input iterators, except possibly `xs`, must be finite. If `xs` is finite, the output
        /// length is the product of the lengths of all the input iterators. If `xs` is infinite,
        /// the output is also infinite.
        ///
        /// If any of the input iterators is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If `xs` is finite:
        ///
        /// $T(i, n) = O(n)$
        ///
        /// $M(i, n) = O(n)$
        ///
        /// If `xs` is infinite: Let $j$ be the largest index of any output associated with `xs`,
        /// $X$ the set of outputs with indices higher than $j$, $P$ the product of the lengths of
        /// all the iterators associated with the outputs in $X$, including multiplicities, and
        /// $T^\prime$ and $M^\prime$ the time and additional memory complexities of `xs`.
        ///
        /// Finally, we have
        ///
        /// $$
        /// T(i, n) = O(n + T^\prime (i / P))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + M^\prime (i / P))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory and $n$ is the number of input iterators.
        ///
        /// # Examples
        ///
        /// See the documentation of the `vecs::exhaustive` module.
        pub fn $exhaustive_custom_fn<T: Clone, $($it: Iterator<Item = T>,)*>(
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

        /// Generates all length-$n$ `Vec`s with elements from $n$ iterators, in lexicographic
        /// order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// This function is macro-generated. The value of $n$ is in the function's name.
        ///
        /// All of `ys`, `zs`, ... (but not necessarily `xs`) must be finite. If `xs` is finite, the
        /// output length is the product of the lengths of all the input iterators. If `xs` is
        /// infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If `xs` is finite:
        ///
        /// $T(i, n) = O(n)$
        ///
        /// $M(i, n) = O(n)$
        ///
        /// If `xs`, is infinite:
        ///
        /// $$
        /// T(i, n) = O(n + T^\prime (i / P))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + M^\prime (i / P))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators,
        /// $T^\prime$ and $M^\prime$ are the time and additional memory complexities of `xs`, and
        /// $P$ is the product of the lengths of `ys`, `zs`, ... (excluding `xs`).
        ///
        /// # Examples
        ///
        /// See the documentation of the `vecs::exhaustive` module.
        #[inline]
        pub fn $exhaustive_1_to_1_fn<T: Clone, $($it: Iterator<Item = T>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<T, $($it,)*> {
            $exhaustive_custom_fn($($xs,)* &[$($i,)*])
        }
    }
}

lex_fixed_length_vecs!(
    LexFixedLengthVecs2Inputs,
    lex_fixed_length_vecs_2_inputs,
    lex_length_2_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs]
);
lex_fixed_length_vecs!(
    LexFixedLengthVecs3Inputs,
    lex_fixed_length_vecs_3_inputs,
    lex_length_3_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs]
);
lex_fixed_length_vecs!(
    LexFixedLengthVecs4Inputs,
    lex_fixed_length_vecs_4_inputs,
    lex_length_4_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs]
);
lex_fixed_length_vecs!(
    LexFixedLengthVecs5Inputs,
    lex_fixed_length_vecs_5_inputs,
    lex_length_5_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs]
);
lex_fixed_length_vecs!(
    LexFixedLengthVecs6Inputs,
    lex_fixed_length_vecs_6_inputs,
    lex_length_6_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs],
    [5, N, us, us_outputs]
);
lex_fixed_length_vecs!(
    LexFixedLengthVecs7Inputs,
    lex_fixed_length_vecs_7_inputs,
    lex_length_7_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs],
    [5, N, us, us_outputs],
    [6, O, ts, ts_outputs]
);
lex_fixed_length_vecs!(
    LexFixedLengthVecs8Inputs,
    lex_fixed_length_vecs_8_inputs,
    lex_length_8_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs],
    [5, N, us, us_outputs],
    [6, O, ts, ts_outputs],
    [7, P, ss, ss_outputs]
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
            } else {
                *counter = 0;
            }
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
                Some(repeat(x).cloned().take(self.counters.len()).collect())
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

fn lex_fixed_length_vecs_from_single_g<I: Iterator>(
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

/// Generates all `Vec`s of a given length with elements from a single iterator, in lexicographic
/// order.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// This `struct` is created by the `lex_fixed_length_vecs_from_single` function. See its
/// documentation for more.
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

/// Generates all `Vec`s of a given length with elements from a single iterator, in lexicographic
/// order.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// `xs` must be finite.
///
/// The output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty list.
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Complexity per iteration
///
/// $$
/// T(i, n) = O(n + T^\prime (i))
/// $$
///
/// $$
/// M(i, n) = O(n + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::exhaustive::lex_fixed_length_vecs_from_single;
///
/// let xss = lex_fixed_length_vecs_from_single(2, 0..4).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0], &[0, 1], &[0, 2], &[0, 3], &[1, 0], &[1, 1], &[1, 2], &[1, 3], &[2, 0],
///         &[2, 1], &[2, 2], &[2, 3], &[3, 0], &[3, 1], &[3, 2], &[3, 3]
///     ]
/// );
/// ```
pub fn lex_fixed_length_vecs_from_single<I: Iterator>(
    len: u64,
    xs: I,
) -> LexFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    match len {
        0 => LexFixedLengthVecsFromSingle::Zero(once(Vec::new())),
        1 => LexFixedLengthVecsFromSingle::One(xs),
        len => LexFixedLengthVecsFromSingle::GreaterThanOne(lex_fixed_length_vecs_from_single_g(
            len, xs,
        )),
    }
}

macro_rules! exhaustive_fixed_length_vecs {
    (
        $exhaustive_struct: ident,
        $exhaustive_custom_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_done: ident, $outputs: ident]),*
    ) => {
        /// Generates all `Vec`s of a given length with elements from $m$ iterators.
        ///
        /// The fixed length $n$ of the `Vec`s is greater than or equal to $m$.
        ///
        /// This struct is macro-generated. The value of $m$ is in the struct's name. Remember that
        /// $m$ is the number of input iterators, not the length of the output `Vec`s!
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<T: Clone, $($it: Iterator<Item=T>,)*> {
            i: u64,
            len: u64,
            limit: Option<u64>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
                $outputs: Vec<usize>,
            )*
            output_to_input_map: Vec<usize>,
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

        /// Generates all `Vec`s of a given length with elements from $m$ iterators, where
        /// $m \leq n$.
        ///
        /// The `output_types` parameter defines which iterators are mapped to which slot in the
        /// output `Vec`s, and how quickly each output slot advances through its iterator. The
        /// length of the output `Vec`s, $n$, is specified by the length of `output_types`.
        ///
        /// The $i$th element of `output_types` is a pair of `BitDistributorOutputType` and `usize`.
        /// The `BitDistributorOutputType` determines how quickly the $i$th output slot advances
        /// through its iterator; see the `BitDistributor` documentation for a description of the
        /// different types. The `usize` is an index from 0 to $m-1$ which specifies which iterator
        /// the $i$th output slot is populated with. Together, the `usize`s must include all indices
        /// from 0 to $m-1$, inclusive, possibly with repetitions.
        ///
        /// This function is macro-generated. The value of $m$ is in the function's name. Remember
        /// that $m$ is the number of input iterators, not the length of the output `Vec`s!
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite:
        ///
        /// $T(i, n) = O((\ell/2)^n \sum_{j=0}^{k-1}T_j(\sqrt\[n\]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt\[n\]{i}))$
        ///
        /// If some of `xs`, `ys`, `zs`, ... are infinite, but all the infinite ones are associated
        /// with tiny outputs: Let $k$ be the number of outputs associated with the infinite
        /// iterators, and $T_0, T_1, \ldots T_{k-1}$ and $M_0, M_1, \ldots M_{k-1}$ be the time and
        /// additional memory complexities of the outputs' input iterators.
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{k-1}T_j(\sqrt\[n\]{i}))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt\[n\]{i}))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory and $n$ is `len`.
        ///
        /// If some of `xs`, `ys`, `zs`, ... are infinite and associated with normal output types:
        /// Let $k$ be the number of outputs associated with the infinite iterators, and
        /// $T_0, T_1, \ldots T_{k-1}$ and $M_0, M_1, \ldots M_{k-1}$ be the time and additional
        /// memory functions of the outputs' input iterators.
        ///
        /// Let $s$ be the sum of the weights of the normal output types associated with infinite
        /// iterators, and let $t$ be the number of tiny outputs associated with infinite iterators.
        /// Then define a weight function $W$ for each of the $k$ infinite outputs.
        /// - If the $j$th output has a normal output type with weight $w$, $W_j(i)=i^{w/s}$.
        /// - If the $j$th output has a tiny output type, $W_j(i)=\sqrt\[t\]{\log i}$.
        ///
        /// Finally, we have
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{k-1}T_j(W_j(i)))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(W_j(i)))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory and $n$ is `len`.
        ///
        /// # Panics
        ///
        /// Panics if the `usize`s in `output_types` do not include all indices from 0 to $m-1$,
        /// inclusive, possibly with repetitions. In particular, the length of `output_types` must
        /// be at least $m$.
        ///
        /// # Examples
        ///
        /// See the documentation of the `vecs::exhaustive` module.
        pub fn $exhaustive_custom_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
            output_types: &[(BitDistributorOutputType, usize)],
        ) -> $exhaustive_struct<T, $($it,)*> {
            $(
                let _max_input_index = $i;
            )*
            let output_to_input_map: Vec<usize> = output_types.iter().map(|(_, i)| *i).collect();
            validate_oi_map(_max_input_index, output_to_input_map.iter().cloned());
            $exhaustive_struct {
                i: 0,
                len: u64::exact_from(output_types.len()),
                limit: None,
                distributor: BitDistributor::new(output_types.iter().map(|(ot, _)| *ot)
                    .collect::<Vec<_>>().as_slice()),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                    $outputs: output_types.iter().enumerate()
                        .filter_map(|(o, (_, i))| if *i == $i { Some(o) } else { None }).collect(),
                )*
                output_to_input_map
            }
        }

        /// Generates all length-$n$ `Vec`s with elements from $n$ iterators.
        ///
        /// This function is macro-generated. The value of $n$ is in the function's name.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite:
        ///
        /// $T(i, n) = O((\ell/2)^n \sum_{j=0}^{n-1}T_j(\sqrt\[n\]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{n-1}M_j(\sqrt\[n\]{i}))$
        ///
        /// If $k$ of `xs`, `ys`, `zs`, ... are infinite:
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{n-1}T_j(\sqrt\[n\]{i}))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{n-1}M_j(\sqrt\[n\]{i}))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_0, T_1, \ldots T_{n-1}$ and $M_0, M_1, \ldots M_{n-1}$ are the time and additional
        /// memory functions of the infinite input iterators.
        ///
        /// # Examples
        ///
        /// See the documentation of the `vecs::exhaustive` module.
        #[inline]
        pub fn $exhaustive_1_to_1_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
        ) -> $exhaustive_struct<T, $($it,)*> {
            $exhaustive_custom_fn(
                $($xs,)*
                &[$((BitDistributorOutputType::normal(1), $i),)*]
            )
        }
    }
}

exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs2Inputs,
    exhaustive_fixed_length_vecs_2_inputs,
    exhaustive_length_2_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs3Inputs,
    exhaustive_fixed_length_vecs_3_inputs,
    exhaustive_length_3_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs4Inputs,
    exhaustive_fixed_length_vecs_4_inputs,
    exhaustive_length_4_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs],
    [3, L, ws, ws_done, ws_outputs]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs5Inputs,
    exhaustive_fixed_length_vecs_5_inputs,
    exhaustive_length_5_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs],
    [3, L, ws, ws_done, ws_outputs],
    [4, M, vs, vs_done, vs_outputs]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs6Inputs,
    exhaustive_fixed_length_vecs_6_inputs,
    exhaustive_length_6_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs],
    [3, L, ws, ws_done, ws_outputs],
    [4, M, vs, vs_done, vs_outputs],
    [5, N, us, us_done, us_outputs]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs7,
    exhaustive_fixed_length_vecs_7_inputs,
    exhaustive_length_7_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs],
    [3, L, ws, ws_done, ws_outputs],
    [4, M, vs, vs_done, vs_outputs],
    [5, N, us, us_done, us_outputs],
    [6, O, ts, ts_done, ts_outputs]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs8Inputs,
    exhaustive_fixed_length_vecs_8_inputs,
    exhaustive_length_8_vecs,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs],
    [3, L, ws, ws_done, ws_outputs],
    [4, M, vs, vs_done, vs_outputs],
    [5, N, us, us_done, us_outputs],
    [6, O, ts, ts_done, ts_outputs],
    [7, P, ss, ss_done, ss_outputs]
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
            if self.i == u64::MAX {
                panic!("Too many iterations");
            }
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

fn exhaustive_fixed_length_vecs_1_input_g<I: Iterator>(
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

/// Generates all `Vec`s of a given length with elements from a single iterator.
///
/// This `struct` is created by the `exhaustive_fixed_length_vecs_from_single` function. See its
/// documentation for more.
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

/// Generates all length-$n$ `Vec`s with elements from a single iterator.
///
/// This function differs from `exhaustive_fixed_length_vecs_from_single` in that different
/// `BitDistributorOutputType`s may be specified for each output element.
///
/// The $i$th element of `output_types` is a `BitDistributorOutputType` that determines how quickly
/// the $i$th output slot advances through the iterator; see the `BitDistributor` documentation for
/// a description of the different types. The length of the output `Vec`s, $n$, is specified by the
/// length of `output_types`.
///
/// If `xs` is finite, the output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$ is `len`.
/// If `xs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty list.
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Complexity per iteration
///
/// If all of `xs` is finite:
///
/// $T(i, n) = O((\ell/2)^n \sum_{j=0}^{k-1}T_j(\sqrt\[n\]{i}))$
///
/// $M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt\[n\]{i}))$
///
/// If `xs` is infinite:
///
/// Let $s$ be the sum of the weights of the normal output types, and let $t$ be the number of tiny
/// outputs. Then define a weight function $W$ for each of the $k$ infinite outputs.
/// - If the $j$th output has a normal output type with weight $w$, $W_j(i)=i^{w/s}$.
/// - If the $j$th output has a tiny output type, $W_j(i)=\sqrt\[t\]{\log i}$.
///
/// Finally, we have
///
/// $$
/// T(i, n) = O(n + \sum_{j=0}^{n-1}T^\prime(W_j(i)))
/// $$
///
/// $$
/// M(i, n) = O(n + \sum_{j=0}^{n-1}M^\prime(W_j(i)))
/// $$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `xs`.
///
/// # Examples
///
/// ```
/// use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
/// use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
/// use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_1_input;
///
/// // We are generating length-3 `Vec`s of chars using one input iterator, which produces all ASCII
/// // chars. The third element has a tiny output type, so it will grow more slowly than the other
/// // two elements (though it doesn't look that way from the first few `Vec`s).
/// let xss = exhaustive_fixed_length_vecs_1_input(
///     exhaustive_ascii_chars(),
///     &[
///         BitDistributorOutputType::normal(1),
///         BitDistributorOutputType::normal(1),
///         BitDistributorOutputType::tiny(),
///     ],
/// );
/// let xss_prefix = xss.take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss_prefix.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &['a', 'a', 'a'], &['a', 'a', 'b'], &['a', 'a', 'c'], &['a', 'a', 'd'],
///         &['a', 'b', 'a'], &['a', 'b', 'b'], &['a', 'b', 'c'], &['a', 'b', 'd'],
///         &['a', 'a', 'e'], &['a', 'a', 'f'], &['a', 'a', 'g'], &['a', 'a', 'h'],
///         &['a', 'b', 'e'], &['a', 'b', 'f'], &['a', 'b', 'g'], &['a', 'b', 'h'],
///         &['b', 'a', 'a'], &['b', 'a', 'b'], &['b', 'a', 'c'], &['b', 'a', 'd']
///     ]
/// );
/// ```
pub fn exhaustive_fixed_length_vecs_1_input<I: Iterator>(
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
            exhaustive_fixed_length_vecs_1_input_g(xs, output_types),
        ),
    }
}

/// Generates all `Vec`s of a given length with elements from a single iterator.
///
/// If `xs` is finite, the output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$ is `len`.
/// If `xs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty list.
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Complexity per iteration
///
/// If `xs` is finite:
///
/// $T(i, n) = O((\ell/2)^n T^\prime(\sqrt\[n\]{i}))$
///
/// $M(i, n) = O(n + M^\prime(\sqrt\[n\]{i}))$
///
/// If `xs` is infinite:
///
/// $T(i, n) = O(n + T^\prime(\sqrt\[n\]{i}))$
///
/// $M(i, n) = O(n + M^\prime(\sqrt\[n\]{i}))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_from_single;
///
/// let xss = exhaustive_fixed_length_vecs_from_single(2, 0..4).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0], &[0, 1], &[1, 0], &[1, 1], &[0, 2], &[0, 3], &[1, 2], &[1, 3], &[2, 0],
///         &[2, 1], &[3, 0], &[3, 1], &[2, 2], &[2, 3], &[3, 2], &[3, 3]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_fixed_length_vecs_from_single<I: Iterator>(
    len: u64,
    xs: I,
) -> ExhaustiveFixedLengthVecs1Input<I>
where
    I::Item: Clone,
{
    exhaustive_fixed_length_vecs_1_input(
        xs,
        &vec![BitDistributorOutputType::normal(1); usize::exact_from(len)],
    )
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct LexVecsGenerator<Y: Clone, J: Clone + Iterator<Item = Y>> {
    ys: J,
}

impl<Y: Clone, J: Clone + Iterator<Item = Y>>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<Y>, LexFixedLengthVecsFromSingle<J>>
    for LexVecsGenerator<Y, J>
{
    #[inline]
    fn get_ys(&self, &x: &u64) -> LexFixedLengthVecsFromSingle<J> {
        lex_fixed_length_vecs_from_single(x, self.ys.clone())
    }
}

#[inline]
fn shortlex_vecs_from_element_iterator_helper<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
) -> LexDependentPairs<u64, Vec<T>, LexVecsGenerator<T, J>, I, LexFixedLengthVecsFromSingle<J>> {
    lex_dependent_pairs_stop_after_empty_ys(xs, LexVecsGenerator { ys })
}

/// Generates all `Vec`s with elements from a specified iterator and with lengths from another
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

/// Generates all `Vec`s with elements from a specified iterator and with lengths from another
/// iterator.
///
/// The length-generating iterator is `xs`, and the element-generating iterator is `ys`.
///
/// If the provided lengths are $\ell_0, \ell_1, \ell_2, \ldots$, then first all `Vec`s with length
/// $\ell_0$ will be generated, in lexicographic order; then all `Vec`s with length $\ell_2$, and so
/// on. If the lengths iterator has repetitions, then the generated `Vec`s will be repeated too.
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
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::nevers::nevers;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_from_length_iterator;
///
/// let xss = shortlex_vecs_from_length_iterator([2, 1, 2].iter().cloned(), exhaustive_bools())
///     .collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[false, false][..], &[false, true], &[true, false], &[true, true], &[false], &[true],
///         &[false, false], &[false, true], &[true, false], &[true, true]
///     ]
/// );
///
/// let xss = shortlex_vecs_from_length_iterator([0, 0, 1, 0].iter().cloned(), nevers())
///     .collect::<Vec<_>>();
/// // Stops after first empty ys
/// assert_eq!(xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(), &[&[], &[]]);
/// ```
#[inline]
pub fn shortlex_vecs_from_length_iterator<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
) -> ShortlexVecs<T, I, J> {
    ShortlexVecs(shortlex_vecs_from_element_iterator_helper(xs, ys))
}

/// Generates `Vec`s with elements from a specified iterator, in shortlex order.
///
/// Shortlex order means that the `Vec`s are output from shortest to longest, and `Vec`s of the same
/// length are output in lexicographic order with respect to the ordering of the `Vec` elements
/// specified by the input iterator.
///
/// `xs` must be finite; if it's infinite, only `Vec`s of length 0 and 1 are ever produced.
///
/// If `xs` is empty, the output length is 1; otherwise, the output is infinite.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs;
///
/// let bss = shortlex_vecs(exhaustive_bools()).take(20).collect::<Vec<_>>();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[][..], &[false], &[true], &[false, false], &[false, true], &[true, false],
///         &[true, true], &[false, false, false], &[false, false, true], &[false, true, false],
///         &[false, true, true], &[true, false, false], &[true, false, true], &[true, true, false],
///         &[true, true, true], &[false, false, false, false], &[false, false, false, true],
///         &[false, false, true, false], &[false, false, true, true], &[false, true, false, false]
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

/// Generates all `Vec`s with a minimum length and with elements from a specified iterator, in
/// shortlex order.
///
/// Shortlex order means that the `Vec`s are output from shortest to longest, and `Vec`s of the same
/// length are output in lexicographic order with respect to the ordering of the `Vec` elements
/// specified by the input iterator.
///
/// `xs` must be finite; if it's infinite, only `Vec`s of length `min_length` (or 0 and 1, if
/// `min_length` is 0) are ever produced.
///
/// If `xs` is empty and `min_length` is 0, the output length is 1; if `xs` is empty and
/// `min_length` is greater than 0, the output is empty; otherwise, the output is infinite.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_min_length;
///
/// let bss = shortlex_vecs_min_length(2, exhaustive_bools()).take(20).collect::<Vec<_>>();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[false, false][..], &[false, true], &[true, false], &[true, true],
///         &[false, false, false], &[false, false, true], &[false, true, false],
///         &[false, true, true], &[true, false, false], &[true, false, true], &[true, true, false],
///         &[true, true, true], &[false, false, false, false], &[false, false, false, true],
///         &[false, false, true, false], &[false, false, true, true], &[false, true, false, false],
///         &[false, true, false, true], &[false, true, true, false], &[false, true, true, true]
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

/// Generates all `Vec`s with lengths in $[a, b)$ and with elements from a specified iterator, in
/// shortlex order.
///
/// Shortlex order means that the `Vec`s are output from shortest to longest, and `Vec`s of the same
/// length are output in lexicographic order with respect to the ordering of the `Vec` elements
/// specified by the input iterator.
///
/// `xs` must be finite; if it's infinite and $a < b$, only `Vec`s of length `a` (or 0 and 1, if `a`
/// is 0) are ever produced.
///
/// The output length is
/// $$
/// \sum_{k=a}^{b-1} n^k,
/// $$
/// where $k$ is `xs.count()`.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_length_range;
///
/// let bss = shortlex_vecs_length_range(2, 4, exhaustive_bools()).collect::<Vec<_>>();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[false, false][..], &[false, true], &[true, false], &[true, true],
///         &[false, false, false], &[false, false, true], &[false, true, false],
///         &[false, true, true], &[true, false, false], &[true, false, true], &[true, true, false],
///         &[true, true, true]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_vecs_length_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    shortlex_vecs_from_length_iterator(primitive_int_increasing_range(a, b), xs)
}

/// Generates all `Vec`s with lengths in $[a, b]$ and with elements from a specified iterator, in
/// shortlex order.
///
/// Shortlex order means that the `Vec`s are output from shortest to longest, and `Vec`s of the same
/// length are output in lexicographic order with respect to the ordering of the `Vec` elements
/// specified by the input iterator.
///
/// `xs` must be finite; if it's infinite, only `Vec`s of length `a` (or 0 and 1, if `a` is 0) are
/// ever produced.
///
/// The output length is
/// $$
/// \sum_{k=a}^b n^k,
/// $$
/// where $k$ is `xs.count()`.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::vecs::exhaustive::shortlex_vecs_length_inclusive_range;
///
/// let bss = shortlex_vecs_length_inclusive_range(2, 3, exhaustive_bools()).collect::<Vec<_>>();
/// assert_eq!(
///     bss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[false, false][..], &[false, true], &[true, false], &[true, true],
///         &[false, false, false], &[false, false, true], &[false, true, false],
///         &[false, true, true], &[true, false, false], &[true, false, true], &[true, true, false],
///         &[true, true, true]
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_vecs_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ShortlexVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    shortlex_vecs_from_length_iterator(primitive_int_increasing_inclusive_range(a, b), xs)
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExhaustiveVecsGenerator<Y: Clone, J: Clone + Iterator<Item = Y>> {
    ys: J,
}

impl<Y: Clone, J: Clone + Iterator<Item = Y>>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<Y>, ExhaustiveFixedLengthVecs1Input<J>>
    for ExhaustiveVecsGenerator<Y, J>
{
    #[inline]
    fn get_ys(&self, &x: &u64) -> ExhaustiveFixedLengthVecs1Input<J> {
        exhaustive_fixed_length_vecs_1_input(
            self.ys.clone(),
            &vec![BitDistributorOutputType::normal(1); usize::exact_from(x)],
        )
    }
}

#[allow(clippy::type_complexity)]
#[inline]
fn exhaustive_vecs_from_element_iterator_helper<
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

/// Generates all `Vec`s with elements from a specified iterator and with lengths from another
/// iterator.
#[allow(clippy::type_complexity)]
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

/// Generates all `Vec`s with elements from a specified iterator and with lengths from another
/// iterator.
///
/// The length-generating iterator is `xs`, and the element-generating iterator is `ys`.
///
/// If the lengths iterator has repetitions, then the generated `Vec`s will be repeated too.
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
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::nevers::nevers;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_from_length_iterator;
///
/// let xss = exhaustive_vecs_from_length_iterator([2, 1, 2].iter().cloned(), exhaustive_bools())
///     .collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[false, false][..], &[false], &[false, true], &[false, false], &[true, false], &[true],
///         &[true, true], &[false, true], &[true, false], &[true, true]
///     ]
/// );
///
/// let xss = exhaustive_vecs_from_length_iterator([0, 0, 1, 0].iter().cloned(), nevers())
///     .collect::<Vec<_>>();
/// // Stops at some point after first empty ys
/// assert_eq!(xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(), &[&[], &[]]);
/// ```
#[inline]
pub fn exhaustive_vecs_from_length_iterator<
    T: Clone,
    I: Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    lengths: I,
    xs: J,
) -> ExhaustiveVecs<T, I, J> {
    ExhaustiveVecs(exhaustive_vecs_from_element_iterator_helper(lengths, xs))
}

/// Generates all `Vec`s with elements from a specified iterator.
///
/// If `xs` is empty, the output length is 1; otherwise, the output is infinite.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs;
///
/// let xss = exhaustive_vecs(exhaustive_unsigneds::<u32>()).take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[][..], &[0], &[1], &[0, 0, 0], &[2], &[0, 0], &[3], &[0, 0, 0, 0], &[4], &[0, 1],
///         &[5], &[0, 0, 1], &[6], &[1, 0], &[7], &[0, 0, 0, 0, 0], &[8], &[1, 1], &[9], &[0, 1, 0]
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

/// Generates all `Vec`s with a minimum length and with elements from a specified iterator.
///
/// If `xs` is empty and `min_length` is 0, the output length is 1; if `xs` is empty and
/// `min_length` is greater than 0, the output is empty; otherwise, the output is infinite.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_min_length;
///
/// let xss = exhaustive_vecs_min_length(2, exhaustive_unsigneds::<u32>())
///         .take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0][..], &[0, 0, 0], &[0, 1], &[0, 0, 0, 0], &[1, 0], &[0, 0, 1], &[1, 1],
///         &[0, 0, 0, 0, 0], &[0, 2], &[0, 1, 0], &[0, 3], &[0, 0, 0, 1], &[1, 2], &[0, 1, 1],
///         &[1, 3], &[0, 0, 0, 0, 0, 0], &[2, 0], &[1, 0, 0], &[2, 1], &[0, 0, 1, 0]
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

/// Generates all `Vec`s with lengths in $[a, b)$ and with elements from a specified iterator.
///
/// - If $a = b$, the output length is 0.
/// - If $a = 0$ and $b = 1$, the output length is 1.
/// - If $a < b$, $b > 1$, and `xs` is infinite, the output length is infinite.
/// - If `xs` is finite, the output length is
///   $$
///   \sum_{k=a}^{b-1} n^k,
///   $$
///   where $k$ is `xs.count()`.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_length_range;
///
/// let xss = exhaustive_vecs_length_range(2, 4, exhaustive_unsigneds::<u32>())
///         .take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0][..], &[0, 0, 0], &[0, 1], &[1, 0], &[1, 1], &[0, 0, 1], &[0, 2], &[0, 1, 0],
///         &[0, 3], &[0, 1, 1], &[1, 2], &[1, 3], &[2, 0], &[1, 0, 0], &[2, 1], &[3, 0], &[3, 1],
///         &[1, 0, 1], &[2, 2], &[2, 3]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs_length_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    exhaustive_vecs_from_length_iterator(primitive_int_increasing_range(a, b), xs)
}

/// Generates all `Vec`s with lengths in $[a, b]$ and with elements from a specified iterator.
///
/// - If $a = b = 0$, the output length is 1.
/// - If $a < b$, $b > 0$, and `xs` is infinite, the output length is infinite.
/// - If `xs` is finite, the output length is
///   $$
///   \sum_{k=a}^b n^k,
///   $$
///   where $k$ is `xs.count()`.
///
/// The lengths of the output `Vec`s grow logarithmically.
///
/// # Complexity per iteration
///
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_vecs_length_inclusive_range;
///
/// let xss = exhaustive_vecs_length_inclusive_range(2, 4, exhaustive_unsigneds::<u32>())
///         .take(20).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0][..], &[0, 0, 0], &[0, 1], &[0, 0, 0, 0], &[1, 0], &[0, 0, 1], &[1, 1], &[0, 2],
///         &[0, 3], &[0, 1, 0], &[1, 2], &[0, 0, 0, 1], &[1, 3], &[0, 1, 1], &[2, 0], &[1, 0, 0],
///         &[2, 1], &[1, 0, 1], &[3, 0], &[0, 0, 1, 0]
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_vecs_length_inclusive_range<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
) -> ExhaustiveVecs<I::Item, PrimitiveIntIncreasingRange<u64>, I>
where
    I::Item: Clone,
{
    exhaustive_vecs_from_length_iterator(primitive_int_increasing_inclusive_range(a, b), xs)
}
