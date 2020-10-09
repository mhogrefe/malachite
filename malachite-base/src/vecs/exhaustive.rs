use std::iter::{once, repeat, Once};
use std::marker::PhantomData;
use std::mem::swap;

use itertools::Itertools;

use iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use iterators::iterator_cache::IteratorCache;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::SignificantBits;

fn set_slice_to_none<T>(xs: &mut [Option<T>]) {
    for x in xs {
        *x = None;
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct LexExhaustiveFixedLengthVecsOutput {
    input_index: usize,
    counter: usize,
}

macro_rules! lex_exhaustive_fixed_length_vecs {
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
            outputs: Vec<LexExhaustiveFixedLengthVecsOutput>,
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

        /// Generates all length-$n$ `Vec`s of a given length with elements from $m$ iterators,
        /// where $m \leq n$, in lexicographic order.
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
        /// If `xs` is infinite: Let $j$ be the largest index of any output associated with `xs`,
        /// $X$ the set of outputs with indices higher than $j$, $P$ the product of the lengths of
        /// all the iterators associated with the outputs in $X$, including multiplicities, and
        /// $T^\prime$ and $M^\prime$ the time and additional memory complexities of `xs`.
        ///
        /// Finally, we have
        ///
        /// $$
        /// T(i, n) = O(n + T^\prime{i / P})
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + M^\prime{i / P})
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
            let oi_sorted_unique = output_to_input_map.iter()
                .cloned().unique().sorted().collect::<Vec<_>>();
            assert_eq!(oi_sorted_unique.len(), _max_input_index + 1);
            assert_eq!(*oi_sorted_unique.first().unwrap(), 0);
            assert_eq!(*oi_sorted_unique.last().unwrap(), _max_input_index);
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
                    .map(|&i| LexExhaustiveFixedLengthVecsOutput {
                        input_index: i,
                        counter: 0,
                    })
                    .collect(),
            }
        }

        /// Generates all length-$n$ `Vec`s of a given length with elements from $n$ iterators, in
        /// lexicographic order.
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
        /// T(i, n) = O(n + T^\prime{i / P})
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + M^\prime{i / P})
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

lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs2Inputs,
    lex_exhaustive_fixed_length_vecs_2_inputs,
    lex_exhaustive_length_2_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs]
);
lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs3Inputs,
    lex_exhaustive_fixed_length_vecs_3_inputs,
    lex_exhaustive_length_3_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs]
);
lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs4Inputs,
    lex_exhaustive_fixed_length_vecs_4_inputs,
    lex_exhaustive_length_4_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs]
);
lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs5Inputs,
    lex_exhaustive_fixed_length_vecs_5_inputs,
    lex_exhaustive_length_5_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs]
);
lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs6Inputs,
    lex_exhaustive_fixed_length_vecs_6_inputs,
    lex_exhaustive_length_6_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs],
    [5, N, us, us_outputs]
);
lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs7Inputs,
    lex_exhaustive_fixed_length_vecs_7_inputs,
    lex_exhaustive_length_7_vecs,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs],
    [3, L, ws, ws_outputs],
    [4, M, vs, vs_outputs],
    [5, N, us, us_outputs],
    [6, O, ts, ts_outputs]
);
lex_exhaustive_fixed_length_vecs!(
    LexExhaustiveFixedLengthVecs8Inputs,
    lex_exhaustive_fixed_length_vecs_8_inputs,
    lex_exhaustive_length_8_vecs,
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
pub struct LexExhaustiveFixedLengthVecsFromSingleG<I: Iterator>
where
    I::Item: Clone,
{
    first: bool,
    done: bool,
    xs: IteratorCache<I>,
    counters: Vec<usize>,
    phantom: PhantomData<*const I::Item>,
}

impl<I: Iterator> LexExhaustiveFixedLengthVecsFromSingleG<I>
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

impl<I: Iterator> Iterator for LexExhaustiveFixedLengthVecsFromSingleG<I>
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
            let mut next = Vec::with_capacity(self.counters.len());
            for &c in &self.counters {
                next.push(self.xs.get(c).unwrap().clone());
            }
            Some(next)
        }
    }
}

pub fn lex_exhaustive_fixed_length_vecs_from_single_g<I: Iterator>(
    len: usize,
    xs: I,
) -> LexExhaustiveFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    LexExhaustiveFixedLengthVecsFromSingleG {
        first: true,
        done: false,
        xs: IteratorCache::new(xs),
        counters: vec![0; len],
        phantom: PhantomData,
    }
}

#[derive(Clone, Debug)]
pub enum LexExhaustiveFixedLengthVecsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    Zero(Once<Vec<I::Item>>),
    One(I),
    GreaterThanOne(LexExhaustiveFixedLengthVecsFromSingleG<I>),
}

impl<I: Iterator> Iterator for LexExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            LexExhaustiveFixedLengthVecsFromSingle::Zero(ref mut xs) => xs.next(),
            LexExhaustiveFixedLengthVecsFromSingle::One(ref mut xs) => xs.next().map(|x| vec![x]),
            LexExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all `Vec`s of a given length with elements from a single iterator, in lexicographic
/// order.
///
/// The order is lexicographic with respect to the order of the element iterators.
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
/// $T(i, n) = O(n)$
///
/// $M(i, n) = O(n)$
///
/// If `xs` is infinite:
///
/// $$
/// T(i, n) = O(n + T^\prime{i})
/// $$
///
/// $$
/// M(i, n) = O(n + M^\prime{i})
/// $$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::exhaustive::lex_exhaustive_fixed_length_vecs_from_single;
///
/// let xss = lex_exhaustive_fixed_length_vecs_from_single(2, 0..4).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0], &[0, 1], &[0, 2], &[0, 3], &[1, 0], &[1, 1], &[1, 2], &[1, 3], &[2, 0],
///         &[2, 1], &[2, 2], &[2, 3], &[3, 0], &[3, 1], &[3, 2], &[3, 3]
///     ]
/// );
/// ```
pub fn lex_exhaustive_fixed_length_vecs_from_single<I: Iterator>(
    len: usize,
    xs: I,
) -> LexExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    match len {
        0 => LexExhaustiveFixedLengthVecsFromSingle::Zero(once(Vec::new())),
        1 => LexExhaustiveFixedLengthVecsFromSingle::One(xs),
        len => LexExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(
            lex_exhaustive_fixed_length_vecs_from_single_g(len, xs),
        ),
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
            done: bool,
            next: Vec<Option<T>>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
                $outputs: Vec<usize>,
            )*
            oi_map: Vec<usize>,
        }

        impl<T: Clone, $($it: Iterator<Item=T>,)*> Iterator for $exhaustive_struct<T, $($it,)*> {
            type Item = Vec<T>;

            fn next(&mut self) -> Option<Vec<T>> {
                if self.done {
                    None
                } else {
                    loop {
                        let mut some_are_valid = false;
                        let mut all_are_valid = true;
                        $(
                            let mut no_x = false;
                            for &output_index in &self.$outputs {
                                if let Some(x) = self.$xs.get(
                                    self.distributor.get_output(output_index)
                                ) {
                                    self.next[output_index] = Some(x.clone());
                                    some_are_valid = true;
                                } else {
                                    no_x = true;
                                    all_are_valid = false;
                                    if some_are_valid {
                                        break;
                                    }
                                }
                            }
                            if no_x {
                                if !self.$xs_done {
                                    let xs_len = self.$xs.known_len().unwrap();
                                    if xs_len == 0 {
                                        self.done = true;
                                        return None;
                                    }
                                    self.$xs_done = true;
                                    self.distributor.set_max_bits(
                                        &self.$outputs,
                                        usize::wrapping_from(xs_len.significant_bits())
                                    );
                                    set_slice_to_none(&mut self.next);
                                    continue;
                                } else if some_are_valid {
                                    set_slice_to_none(&mut self.next);
                                    self.distributor.increment_counter();
                                    continue;
                                }
                            }
                        )*
                        if !some_are_valid {
                            self.done = true;
                            return None;
                        } else if all_are_valid {
                            break;
                        } else {
                            set_slice_to_none(&mut self.next);
                            self.distributor.increment_counter();
                        }
                    }
                    let mut out = vec![None; self.next.len()];
                    swap(&mut self.next, &mut out);
                    self.distributor.increment_counter();
                    Some(out.into_iter().map(Option::unwrap).collect())
                }
            }
        }

        /// Generates all length-$n$ `Vec`s of a given length with elements from $m$ iterators,
        /// where $m \leq n$.
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
        /// $T(i, n) = O(n)$
        ///
        /// $M(i, n) = O(n)$
        ///
        /// If some of `xs`, `ys`, `zs`, ... are infinite, but all the infinite ones are associated
        /// with tiny outputs: Let $k$ be the number of outputs associated with the infinite
        /// iterators, and $T_0, T_1, \ldots T_{k-1}$ and $M_0, M_1, \ldots M_{k-1}$ be the time and
        /// additional memory complexities of the outputs' input iterators.
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{k-1}T_j(\sqrt[n]{i}))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))
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
        /// - If the $j$th output has a tiny output type, $W_j(i)=\sqrt[t]{\log i}$.
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
            let oi_map: Vec<usize> = output_types.iter().map(|(_, i)| *i).collect();
            let oi_sorted_unique = oi_map.iter().cloned().unique().sorted().collect::<Vec<_>>();
            assert_eq!(oi_sorted_unique.len(), _max_input_index + 1);
            assert_eq!(*oi_sorted_unique.first().unwrap(), 0);
            assert_eq!(*oi_sorted_unique.last().unwrap(), _max_input_index);
            $exhaustive_struct {
                done: false,
                next: vec![None; output_types.len()],
                distributor: BitDistributor::new(output_types.iter().map(|(ot, _)| *ot)
                    .collect::<Vec<_>>().as_slice()),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                    $outputs: output_types.iter().enumerate()
                        .filter_map(|(o, (_, i))| if *i == $i { Some(o) } else { None }).collect(),
                )*
                oi_map
            }
        }

        /// Generates all length-$n$ `Vec`s of a given length with elements from $n$ iterators.
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
        /// $T(i, n) = O(n)$
        ///
        /// $M(i, n) = O(n)$
        ///
        /// If $k$ of `xs`, `ys`, `zs`, ... are infinite:
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{k-1}T_j(\sqrt[n]{i}))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_0, T_1, \ldots T_{k-1}$ and $M_0, M_1, \ldots M_{k-1}$ are the time and additional
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
    ExhaustiveFixedLengthVecs1Input,
    exhaustive_fixed_length_vecs_1_input, //TODO test
    _dont_use_this,
    [0, I, xs, xs_done, xs_outputs]
);
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
pub struct ExhaustiveFixedLengthVecsFromSingleG<I: Iterator>
where
    I::Item: Clone,
{
    done: bool,
    next: Vec<Option<I::Item>>,
    distributor: BitDistributor,
    xs: IteratorCache<I>,
    xs_done: bool,
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if self.done {
            None
        } else {
            loop {
                let mut some_are_valid = false;
                let mut all_are_valid = true;
                for (i, n) in self.next.iter_mut().enumerate() {
                    if let Some(x) = self.xs.get(self.distributor.get_output(i)) {
                        *n = Some(x.clone());
                        some_are_valid = true;
                    } else {
                        all_are_valid = false;
                        if some_are_valid {
                            break;
                        }
                    }
                }
                if all_are_valid {
                    break;
                } else if !self.xs_done {
                    let xs_len = self.xs.known_len().unwrap();
                    if xs_len == 0 {
                        self.done = true;
                        return None;
                    }
                    self.xs_done = true;
                    self.distributor
                        .set_max_bits(&[0], usize::wrapping_from(xs_len.significant_bits()));
                    set_slice_to_none(&mut self.next);
                } else if some_are_valid {
                    set_slice_to_none(&mut self.next);
                    self.distributor.increment_counter();
                } else {
                    self.done = true;
                    return None;
                }
            }
            let mut out = vec![None; self.next.len()];
            swap(&mut self.next, &mut out);
            self.distributor.increment_counter();
            Some(out.into_iter().map(Option::unwrap).collect())
        }
    }
}

fn exhaustive_fixed_length_vecs_from_single_g<I: Iterator>(
    len: usize,
    xs: I,
) -> ExhaustiveFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    ExhaustiveFixedLengthVecsFromSingleG {
        done: false,
        next: vec![None; len],
        distributor: BitDistributor::new(&vec![BitDistributorOutputType::normal(1); len]),
        xs: IteratorCache::new(xs),
        xs_done: false,
    }
}

/// Generates all `Vec`s of a given length with elements from a single iterator.
///
/// This `struct` is created by the `exhaustive_fixed_length_vecs_from_single` method. See its
/// documentation for more.
#[derive(Clone, Debug)]
pub enum ExhaustiveFixedLengthVecsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    Zero(Once<Vec<I::Item>>),
    One(I),
    GreaterThanOne(ExhaustiveFixedLengthVecsFromSingleG<I>),
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            ExhaustiveFixedLengthVecsFromSingle::Zero(ref mut xs) => xs.next(),
            ExhaustiveFixedLengthVecsFromSingle::One(ref mut xs) => xs.next().map(|x| vec![x]),
            ExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(ref mut xs) => xs.next(),
        }
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
/// $T(i, n) = O(n)$
///
/// $M(i, n) = O(n)$
///
/// If `xs` is infinite:
///
/// $T(i, n) = O(n + T^\prime(\sqrt[n]{i}))$
///
/// $M(i, n) = O(n + M^\prime(\sqrt[n]{i}))$
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
pub fn exhaustive_fixed_length_vecs_from_single<I: Iterator>(
    len: usize,
    xs: I,
) -> ExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    match len {
        0 => ExhaustiveFixedLengthVecsFromSingle::Zero(once(Vec::new())),
        1 => ExhaustiveFixedLengthVecsFromSingle::One(xs),
        len => ExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(
            exhaustive_fixed_length_vecs_from_single_g(len, xs),
        ),
    }
}
