use iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use iterators::iterator_cache::IteratorCache;
use num::arithmetic::traits::CheckedPow;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::SignificantBits;
use std::cmp::max;
use std::fmt::Debug;
use std::iter::{once, Once};
use std::marker::PhantomData;

/// Generates the only unit: `()`.
///
/// The output length is 1.
///
/// # Worst-case complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::tuples::exhaustive::exhaustive_units;
///
/// assert_eq!(exhaustive_units().collect::<Vec<_>>(), &[()]);
/// ```
pub fn exhaustive_units() -> Once<()> {
    once(())
}

macro_rules! lex_custom_tuples {
    (
        $exhaustive_struct: ident,
        $out_t: ty,
        $nones: expr,
        $unwrap_tuple: ident,
        $exhaustive_fn: ident,
        $([$t: ident, $it: ident, $xs: ident, $([$i: tt, $x: ident]),*]),*
    ) => {
        /// Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$, in
        /// lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// The mapping from iterators to tuple slots is indicated by the struct name; for example,
        /// in `LexTriplesXYX` there are two iterators, `X`, and `Y`; `X` generates the elements in
        /// the first and third slots of the output triples, and `Y` generates the elements in the
        /// second slots.
        ///
        /// This struct is macro-generated.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
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
                    } else {
                        *counter = 0;
                    }
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
                    $(
                        $(
                            let $x;
                        )*
                    )*
                    $(
                        if let Some(x) = self.$xs.get(0) {
                            $(
                                $x = x.clone();
                            )*
                        } else {
                            self.done = true;
                            return None;
                        }
                    )*
                    let mut out = $nones;
                    $(
                        $(
                            out.$i = Some($x);
                        )*
                    )*
                    Some($unwrap_tuple(out))
                } else if self.increment_counters() {
                    self.done = true;
                    None
                } else {
                    let mut out = $nones;
                    $(
                        $(
                            out.$i = self.$xs.get(self.counters[$i]).cloned();
                        )*
                    )*
                    Some($unwrap_tuple(out))
                }
            }
        }

        /// Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$, in
        /// lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// The mapping from iterators to tuple slots is indicated by the function name; for
        /// example, `lex_triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the
        /// elements in the first and third slots of the output triples, and `ys` generates the
        /// elements in the second slots.
        ///
        /// Let `xs` be the input iterator mapped to the first slot of the output tuples. All the
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
        /// See the documentation of the `tuples::exhaustive` module.
        pub fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
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

#[allow(clippy::missing_const_for_fn)]
fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
    (a.unwrap(), b.unwrap(), c.unwrap())
}

lex_custom_tuples!(
    LexTriplesXXY,
    (X, X, Y),
    (None, None, None),
    unwrap_triple,
    lex_triples_xxy,
    [X, I, xs, [0, x_0], [1, x_1]],
    [Y, J, ys, [2, y_2]]
);
lex_custom_tuples!(
    LexTriplesXYX,
    (X, Y, X),
    (None, None, None),
    unwrap_triple,
    lex_triples_xyx,
    [X, I, xs, [0, x_0], [2, x_2]],
    [Y, J, ys, [1, y_1]]
);
lex_custom_tuples!(
    LexTriplesXYY,
    (X, Y, Y),
    (None, None, None),
    unwrap_triple,
    lex_triples_xyy,
    [X, I, xs, [0, x_0]],
    [Y, J, ys, [1, y_1], [2, y_2]]
);

// hack for macro
#[inline]
fn clone_helper<T: Clone>(x: &T, _i: usize) -> T {
    x.clone()
}

macro_rules! lex_tuples {
    (
        $exhaustive_struct: ident,
        $exhaustive_struct_from_single: ident,
        $exhaustive_fn: ident,
        $exhaustive_fn_from_single: ident,
        $single_out: tt,
        $([$i: expr, $t: ident, $it: ident, $xs: ident, $x:ident]),*
    ) => {
        /// Generates all $n$-tuples with elements from $n$ iterators, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators.
        ///
        /// This struct is macro-generated.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
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
                            $i => self.$xs.get(*counter).is_some(),
                        )*
                        _ => unreachable!(),
                    };
                    if no_carry {
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
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements in each
        /// tuple, $T^\prime$ and $M^\prime$ are the time and additional memory complexities of
        /// `xs`, and $P$ is the product of the lengths of `ys`, `zs`, ... (excluding `xs`).
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        pub fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_struct {
                first: true,
                done: false,
                $($xs: IteratorCache::new($xs),)*
                counters: vec![$($i * 0,)*],
            }
        }

        /// Generates all $n$-tuples with elements a single iterator, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterator.
        ///
        /// This struct is macro-generated.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct_from_single<T: Clone, I: Iterator<Item = T>> {
            first: bool,
            done: bool,
            xs: IteratorCache<I>,
            counters: Vec<usize>,
        }

        impl<T: Clone, I: Iterator<Item = T>> $exhaustive_struct_from_single<T, I> {
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

        /// Generates all $n$-tuples with elements from a single iterator, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterator.
        ///
        /// `xs` must be finite.
        ///
        /// The output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$ is `len`.
        ///
        /// If `xs` is empty, the output is also empty.
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
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements in each
        /// tuple, and $T^\prime$ and $M^\prime$ are the time and additional memory functions of
        /// `xs`.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        pub fn $exhaustive_fn_from_single<T: Clone, I: Iterator<Item = T>>(
            xs: I
        ) -> $exhaustive_struct_from_single<T, I> {
            $exhaustive_struct_from_single {
                first: true,
                done: false,
                xs: IteratorCache::new(xs),
                counters: vec![$($i * 0,)*],
            }
        }
    }
}
lex_tuples!(
    LexPairs,
    LexPairsFromSingle,
    lex_pairs,
    lex_pairs_from_single,
    (T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y]
);
lex_tuples!(
    LexTriples,
    LexTriplesFromSingle,
    lex_triples,
    lex_triples_from_single,
    (T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z]
);
lex_tuples!(
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
lex_tuples!(
    LexQuintuples,
    LexQuintuplesFromSingle,
    lex_quintuples,
    lex_quintuples_from_single,
    (T, T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w],
    [4, V, M, vs, v]
);
lex_tuples!(
    LexSextuples,
    LexSextuplesFromSingle,
    lex_sextuples,
    lex_sextuples_from_single,
    (T, T, T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w],
    [4, V, M, vs, v],
    [5, U, N, us, u]
);
lex_tuples!(
    LexSeptuples,
    LexSeptuplesFromSingle,
    lex_septuples,
    lex_septuples_from_single,
    (T, T, T, T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w],
    [4, V, M, vs, v],
    [5, U, N, us, u],
    [6, T, O, ts, t]
);
lex_tuples!(
    LexOctuples,
    LexOctuplesFromSingle,
    lex_octuples,
    lex_octuples_from_single,
    (T, T, T, T, T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w],
    [4, V, M, vs, v],
    [5, U, N, us, u],
    [6, T, O, ts, t],
    [7, S, P, ss, s]
);

macro_rules! exhaustive_tuples_1_input {
    (
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $exhaustive_fn_from_single: ident,
        $out_type: ty,
        $([$i: tt, $out_x: ident]),*
    ) => {
        /// Generates all $n$-tuples of a given length with elements from a single iterator.
        ///
        /// This struct is macro-generated.
        #[allow(clippy::type_complexity)]
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<I: Iterator>
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
            #[allow(clippy::type_complexity)]
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

        /// Generates all length-$n$ tuples with elements from a single iterator.
        ///
        /// These functions differ from `exhaustive_[n-tuples]_from_single` in that different
        /// `BitDistributorOutputType`s may be specified for each output element.
        ///
        /// The $i$th parameter `output_types_[x_i]` is a `BitDistributorOutputType` that determines
        /// how quickly the $i$th output slot advances through the iterator; see the
        /// `BitDistributor` documentation for a description of the different types.
        ///
        /// If `xs` is finite, the output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$
        /// is the width of the tuples. If `xs` is infinite, the output is also infinite.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If all of `xs` is finite:
        ///
        /// $T(i, n) = O((\ell/2)^n \sum_{j=0}^{k-1}T_j(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))$
        ///
        /// If `xs` is infinite:
        ///
        /// Let $s$ be the sum of the weights of the normal output types, and let $t$ be the number
        /// of tiny outputs. Then define a weight function $W$ for each of the $k$ infinite outputs.
        /// - If the $j$th output has a normal output type with weight $w$, $W_j(i)=i^{w/s}$.
        /// - If the $j$th output has a tiny output type, $W_j(i)=\sqrt[t]{\log i}$.
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
        /// where $T$ is time, $M$ is additional memory, $n$ is the width of the tuples, and
        /// $T^\prime$ and $M^\prime$ are the time and additional memory functions of `xs`.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        pub fn $exhaustive_fn<I: Iterator>(
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

        /// Generates all $n$-tuples with elements from a single iterator.
        ///
        /// If `xs` is finite, the output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$
        /// is the width of the tuples. If `xs` is infinite, the output is also infinite.
        ///
        /// If `xs` is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If `xs` is finite:
        ///
        /// $T(i, n) = O((\ell/2)^n T^\prime(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + M^\prime(\sqrt[n]{i}))$
        ///
        /// If `xs` is infinite:
        ///
        /// $T(i, n) = O(n + T^\prime(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + M^\prime(\sqrt[n]{i}))$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the width of the tuples, and
        /// $T^\prime$ and $M^\prime$ are the time and additional memory functions of `xs`.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        #[inline]
        pub fn $exhaustive_fn_from_single<I: Iterator>(xs: I) -> $exhaustive_struct<I>
        where
            I::Item: Clone,
        {
            $exhaustive_fn(xs, $(BitDistributorOutputType::normal(1 + $i * 0)),*)
        }
    }
}
exhaustive_tuples_1_input!(
    ExhaustivePairs1Input,
    exhaustive_pairs_1_input,
    exhaustive_pairs_from_single,
    (I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y]
);
exhaustive_tuples_1_input!(
    ExhaustiveTriples1Input,
    exhaustive_triples_1_input,
    exhaustive_triples_from_single,
    (I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z]
);
exhaustive_tuples_1_input!(
    ExhaustiveQuadruples1Input,
    exhaustive_quadruples_1_input,
    exhaustive_quadruples_from_single,
    (I::Item, I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w]
);
exhaustive_tuples_1_input!(
    ExhaustiveQuintuples1Input,
    exhaustive_quintuples_1_input,
    exhaustive_quintuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v]
);
exhaustive_tuples_1_input!(
    ExhaustiveSextuples1Input,
    exhaustive_sextuples_1_input,
    exhaustive_sextuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v],
    [5, output_type_u]
);
exhaustive_tuples_1_input!(
    ExhaustiveSeptuples1Input,
    exhaustive_septuples_1_input,
    exhaustive_septuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v],
    [5, output_type_u],
    [6, output_type_t]
);
exhaustive_tuples_1_input!(
    ExhaustiveOctuples1Input,
    exhaustive_octuples_1_input,
    exhaustive_octuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v],
    [5, output_type_u],
    [6, output_type_t],
    [7, output_type_s]
);

macro_rules! exhaustive_tuples {
    (
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $exhaustive_fn_custom_output: ident,
        $([$i: tt, $t: ident, $it: ident, $xs: ident, $xs_done: ident, $out_x: ident]),*
    ) => {
        /// Generates all $n$-tuples with elements from $n$ iterators.
        ///
        /// This struct is macro-generated.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
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

        /// Generates all $n$-tuples with elements from $n$ iterators, possibly with different
        /// output growth rates.
        ///
        /// The $i$th `output_type_[x_i]` parameter is a `BitDistributorOutputType` that determines
        /// how quickly the $i$th output slot advances through its iterator; see the
        /// `BitDistributor` documentation for a description of the different types.
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
        /// $T(i, n) = O((\ell/2)^n \sum_{j=0}^{k-1}T_j(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))$
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
        /// where $T$ is time, $M$ is additional memory, and $n$ is the width of the tuples.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        pub fn $exhaustive_fn_custom_output<$($t: Clone, $it: Iterator<Item = $t>,)*>(
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

        /// Generates all $n$-tuples with elements from $n$ iterators.
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
        /// $T(i, n) = O((\ell/2)^n \sum_{j=0}^{n-1}T_j(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{n-1}M_j(\sqrt[n]{i}))$
        ///
        /// If $k$ of `xs`, `ys`, `zs`, ... are infinite:
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{n-1}T_j(\sqrt[n]{i}))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{n-1}M_j(\sqrt[n]{i}))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the width of the tuples, and
        /// $T_0, T_1, \ldots T_{n-1}$ and $M_0, M_1, \ldots M_{n-1}$ are the time and additional
        /// memory functions of the infinite input iterators.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        #[inline]
        pub fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
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
    ExhaustivePairs,
    exhaustive_pairs,
    exhaustive_pairs_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y]
);
exhaustive_tuples!(
    ExhaustiveTriples,
    exhaustive_triples,
    exhaustive_triples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z]
);
exhaustive_tuples!(
    ExhaustiveQuadruples,
    exhaustive_quadruples,
    exhaustive_quadruples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z],
    [3, W, L, ws, ws_done, output_type_w]
);
exhaustive_tuples!(
    ExhaustiveQuintuples,
    exhaustive_quintuples,
    exhaustive_quintuples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z],
    [3, W, L, ws, ws_done, output_type_w],
    [4, V, M, vs, vs_done, output_type_v]
);
exhaustive_tuples!(
    ExhaustiveSextuples,
    exhaustive_sextuples,
    exhaustive_sextuples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z],
    [3, W, L, ws, ws_done, output_type_w],
    [4, V, M, vs, vs_done, output_type_v],
    [5, U, N, us, us_done, output_type_u]
);
exhaustive_tuples!(
    ExhaustiveSeptuples,
    exhaustive_septuples,
    exhaustive_septuples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z],
    [3, W, L, ws, ws_done, output_type_w],
    [4, V, M, vs, vs_done, output_type_v],
    [5, U, N, us, us_done, output_type_u],
    [6, T, O, ts, ts_done, output_type_t]
);
exhaustive_tuples!(
    ExhaustiveOctuples,
    exhaustive_octuples,
    exhaustive_octuples_custom_output,
    [0, X, I, xs, xs_done, output_type_x],
    [1, Y, J, ys, ys_done, output_type_y],
    [2, Z, K, zs, zs_done, output_type_z],
    [3, W, L, ws, ws_done, output_type_w],
    [4, V, M, vs, vs_done, output_type_v],
    [5, U, N, us, us_done, output_type_u],
    [6, T, O, ts, ts_done, output_type_t],
    [7, S, P, ss, ss_done, output_type_s]
);

macro_rules! custom_tuples {
    (
        $exhaustive_struct: ident,
        $out_t: ty,
        $nones: expr,
        $unwrap_tuple: ident,
        $exhaustive_fn: ident,
        $exhaustive_custom_fn: ident,
        $([$t: ident, $it: ident, $xs: ident, $xs_done: ident, $([$i: tt, $out_x: ident]),*]),*
    ) => {
        /// Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        ///
        /// The mapping from iterators to tuple slots is indicated by the struct name; for example,
        /// in `TriplesXYX` there are two iterators, `X`, and `Y`; `X` generates the elements in the
        /// first and third slots of the output triples, and `Y` generates the elements in the
        /// second slots.
        ///
        /// This struct is macro-generated.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
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

        /// Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$, possibly
        /// with different output growth rates.
        ///
        /// The mapping from iterators to tuple slots is indicated by the function name; for
        /// example, `triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the elements
        /// in the first and third slots of the output triples, and `ys` generates the elements in
        /// the second slots.
        ///
        /// Let $i$ be the index of the input iterators and $j$ be the index of the output slots. So
        /// for `triples_xyx`, $i=0$ corresponds to $j=0$ and $j=2$, and $i=1$ corresponds to $j=1$.
        ///
        /// The $j$th `output_type_[i_j]` parameter is a `BitDistributorOutputType` that determines
        /// how quickly the $j$th output slot advances through its iterator; see the
        /// `BitDistributor` documentation for a description of the different types.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, then the output length may be obtained by
        /// raising the length of each input iterator to power of the number of outputs it maps to,
        /// and taking the product of the resulting values.
        ///
        /// If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite:
        ///
        /// $T(i, n) = O(P \sum_{j=0}^{n-1}T_j(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{n-1}M_j(\sqrt[n]{i}))$
        ///
        /// If $k$ of `xs`, `ys`, `zs`, ... are infinite:
        ///
        /// $T(i, n) = O(P(n + \sum_{j=0}^{k-1}T_j(\sqrt[n]{i})))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the width of the tuples, $T_j$ and
        /// $M_j$ are the time and additional memory functions of the input iterator corresponding
        /// to the $j$th infinite output, $k$ is the number of outputs corresponding to infinite
        /// input iterators, $m$ is the number of outputs corresponding to finite input iterators,
        /// $\ell_j$ is the length of the input iterator corresponding to the $j$th finite output,
        /// and $P = \product{j=0}^{m-1}\frac{\ell_j}{2}$.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        pub fn $exhaustive_custom_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
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

        /// Generates all $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        ///
        /// The mapping from iterators to tuple slots is indicated by the function name; for
        /// example, `triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the elements
        /// in the first and third slots of the output triples, and `ys` generates the elements in
        /// the second slots.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, then the output length may be obtained by
        /// raising the length of each input iterator to power of the number of outputs it maps to,
        /// and taking the product of the resulting values.
        ///
        /// If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite:
        ///
        /// $T(i, n) = O(P \sum_{j=0}^{n-1}T_j(\sqrt[n]{i}))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{n-1}M_j(\sqrt[n]{i}))$
        ///
        /// If $k$ of `xs`, `ys`, `zs`, ... are infinite:
        ///
        /// $T(i, n) = O(P(n + \sum_{j=0}^{k-1}T_j(\sqrt[n]{i})))$
        ///
        /// $M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the width of the tuples, $T_j$ and
        /// $M_j$ are the time and additional memory functions of the input iterator corresponding
        /// to the $j$th infinite output, $k$ is the number of outputs corresponding to infinite
        /// input iterators, $m$ is the number of outputs corresponding to finite input iterators,
        /// $\ell_j$ is the length of the input iterator corresponding to the $j$th finite output,
        /// and $P = \product{j=0}^{m-1}\frac{\ell_j}{2}$.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::exhaustive` module.
        #[inline]
        pub fn $exhaustive_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            $($xs: $it,)*
        ) -> $exhaustive_struct<$($t, $it,)*> {
            $exhaustive_custom_fn(
                $($xs,)*
                $($(BitDistributorOutputType::normal(1 + 0 * $i),)*)*
            )
        }
    }
}

custom_tuples!(
    ExhaustiveTriplesXXY,
    (X, X, Y),
    (None, None, None),
    unwrap_triple,
    exhaustive_triples_xxy,
    exhaustive_triples_xxy_custom_output,
    [
        X,
        I,
        xs,
        xs_done,
        [0, output_type_xs_0],
        [1, output_type_xs_1]
    ],
    [Y, J, ys, ys_done, [2, output_type_ys_2]]
);
custom_tuples!(
    ExhaustiveTriplesXYX,
    (X, Y, X),
    (None, None, None),
    unwrap_triple,
    exhaustive_triples_xyx,
    exhaustive_triples_xyx_custom_output,
    [
        X,
        I,
        xs,
        xs_done,
        [0, output_type_xs_0],
        [2, output_type_ys_1]
    ],
    [Y, J, ys, ys_done, [1, output_type_xs_2]]
);
custom_tuples!(
    ExhaustiveTriplesXYY,
    (X, Y, Y),
    (None, None, None),
    unwrap_triple,
    exhaustive_triples_xyy,
    exhaustive_triples_xyy_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0]],
    [
        Y,
        J,
        ys,
        ys_done,
        [1, output_type_ys_1],
        [2, output_type_ys_2]
    ]
);
