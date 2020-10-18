use std::iter::{once, Once};

use iterators::iterator_cache::IteratorCache;

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
        $none_t: expr,
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
                    let mut out = $none_t;
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
                    let mut out = $none_t;
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
        /// This function is macro-generated.
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
        /// This function is macro-generated.
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
                        $(
                            let $x = x.clone();
                        )*
                        Some(($($x,)*))
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
        /// T(i, n) = O(n + T^\prime{i})
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + M^\prime{i})
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
