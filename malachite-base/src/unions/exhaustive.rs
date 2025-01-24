// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::unions::Union2;

/// Defines exhaustive union generators.
///
/// Malachite provides [`lex_union2s`] and [`exhaustive_union2s`], but you can also define
/// `lex_union3s`, `lex_union4s`, and so on, and `exhaustive_union3s`, `exhaustive_union4s`, and so
/// on, in your program using the code below. The documentation for [`lex_union2s`] and
/// [`exhaustive_union2s`] describes these other functions as well.
///
/// See usage examples [here](self#lex_union2s) and [here](self#exhaustive_union2s).
///
/// ```
/// use malachite_base::unions::UnionFromStrError;
/// use malachite_base::{exhaustive_unions, union_struct};
/// use std::fmt::{self, Display, Formatter};
/// use std::str::FromStr;
///
/// union_struct!(
///     (pub(crate)),
///     Union3,
///     Union3<T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c]
/// );
/// union_struct!(
///     (pub(crate)),
///     Union4,
///     Union4<T, T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c],
///     [D, D, 'D', d]
/// );
/// union_struct!(
///     (pub(crate)),
///     Union5,
///     Union5<T, T, T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c],
///     [D, D, 'D', d],
///     [E, E, 'E', e]
/// );
/// union_struct!(
///     (pub(crate)),
///     Union6,
///     Union6<T, T, T, T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c],
///     [D, D, 'D', d],
///     [E, E, 'E', e],
///     [F, F, 'F', f]
/// );
/// union_struct!(
///     (pub(crate)),
///     Union7,
///     Union7<T, T, T, T, T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c],
///     [D, D, 'D', d],
///     [E, E, 'E', e],
///     [F, F, 'F', f],
///     [G, G, 'G', g]
/// );
/// union_struct!(
///     (pub(crate)),
///     Union8,
///     Union8<T, T, T, T, T, T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c],
///     [D, D, 'D', d],
///     [E, E, 'E', e],
///     [F, F, 'F', f],
///     [G, G, 'G', g],
///     [H, H, 'H', h]
/// );
///
/// exhaustive_unions!(
///     (pub(crate)),
///     Union3,
///     LexUnion3s,
///     ExhaustiveUnion3s,
///     lex_union3s,
///     exhaustive_union3s,
///     3,
///     [0, X, I, A, xs, xs_done],
///     [1, Y, J, B, ys, ys_done],
///     [2, Z, K, C, zs, zs_done]
/// );
/// exhaustive_unions!(
///     (pub(crate)),
///     Union4,
///     LexUnion4s,
///     ExhaustiveUnion4s,
///     lex_union4s,
///     exhaustive_union4s,
///     4,
///     [0, X, I, A, xs, xs_done],
///     [1, Y, J, B, ys, ys_done],
///     [2, Z, K, C, zs, zs_done],
///     [3, W, L, D, ws, ws_done]
/// );
/// exhaustive_unions!(
///     (pub(crate)),
///     Union5,
///     LexUnion5s,
///     ExhaustiveUnion5s,
///     lex_union5s,
///     exhaustive_union5s,
///     5,
///     [0, X, I, A, xs, xs_done],
///     [1, Y, J, B, ys, ys_done],
///     [2, Z, K, C, zs, zs_done],
///     [3, W, L, D, ws, ws_done],
///     [4, V, M, E, vs, vs_done]
/// );
/// exhaustive_unions!(
///     (pub(crate)),
///     Union6,
///     LexUnion6s,
///     ExhaustiveUnion6s,
///     lex_union6s,
///     exhaustive_union6s,
///     6,
///     [0, X, I, A, xs, xs_done],
///     [1, Y, J, B, ys, ys_done],
///     [2, Z, K, C, zs, zs_done],
///     [3, W, L, D, ws, ws_done],
///     [4, V, M, E, vs, vs_done],
///     [5, U, N, F, us, us_done]
/// );
/// exhaustive_unions!(
///     (pub(crate)),
///     Union7,
///     LexUnion7s,
///     ExhaustiveUnion7s,
///     lex_union7s,
///     exhaustive_union7s,
///     7,
///     [0, X, I, A, xs, xs_done],
///     [1, Y, J, B, ys, ys_done],
///     [2, Z, K, C, zs, zs_done],
///     [3, W, L, D, ws, ws_done],
///     [4, V, M, E, vs, vs_done],
///     [5, U, N, F, us, us_done],
///     [6, T, O, G, ts, ts_done]
/// );
/// exhaustive_unions!(
///     (pub(crate)),
///     Union8,
///     LexUnion8s,
///     ExhaustiveUnion8s,
///     lex_union8s,
///     exhaustive_union8s,
///     8,
///     [0, X, I, A, xs, xs_done],
///     [1, Y, J, B, ys, ys_done],
///     [2, Z, K, C, zs, zs_done],
///     [3, W, L, D, ws, ws_done],
///     [4, V, M, E, vs, vs_done],
///     [5, U, N, F, us, us_done],
///     [6, T, O, G, ts, ts_done],
///     [7, S, P, H, ss, ss_done]
/// );
/// ```
#[macro_export]
macro_rules! exhaustive_unions {
    (
        ($($vis:tt)*),
        $union: ident,
        $lex_struct: ident,
        $exhaustive_struct: ident,
        $lex_fn: ident,
        $exhaustive_fn: ident,
        $n: expr,
        $([$i: expr, $t: ident, $it: ident, $variant: ident, $xs: ident, $xs_done:ident]),*
    ) => {
        /// This documentation applies not only to `LexUnion2s`, but also to `LexUnion3s`,
        /// `LexUnion4s`, and so on. See [`exhaustive_unions`] for more information.
        ///
        /// Generates all $n$-unions with elements from $n$ iterators, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators. All of
        /// the first variant's elements are generated first, followed by the second variant's
        /// elements, and so on.
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        $($vis)* struct $lex_struct<$($t, $it: Iterator<Item=$t>),*> {
            i: u64,
            $($xs: $it,)*
        }

        impl<$($t, $it: Iterator<Item=$t>),*> Iterator for $lex_struct<$($t, $it),*> {
            type Item = $union<$($t),*>;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.i {
                        $(
                            $i => {
                                let next = self.$xs.next().map($union::$variant);
                                if next.is_some() {
                                    return next;
                                }
                            },
                        )*
                        _ => return None,
                    }
                    self.i += 1;
                }
            }
        }

        /// This documentation applies not only to `lex_union2s`, but also to `lex_union3s`,
        /// `lex_union4s`, and so on. See [`exhaustive_unions`] for more information.
        ///
        /// Generates all $n$-unions with elements from $n$ iterators, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators. All of
        /// the first variant's elements are generated first, followed by the second variant's
        /// elements, and so on. This means that all of the iterators, except possibly the last one,
        /// must be finite. For functions that support multiple infinite element iterators, try
        /// `exhaustive_union[n]s`.
        ///
        /// If the last iterator is finite, the output length is the sum of the lengths of all the
        /// input iterators. If the last iterator is infinite, the output is also infinite.
        ///
        /// If all of the input iterators are empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#lex_union2s).
        #[allow(dead_code)]
        #[inline]
        $($vis)* const fn $lex_fn<$($t, $it: Iterator<Item=$t>),*>($($xs: $it),*) ->
                $lex_struct<$($t, $it),*> {
            $lex_struct {
                i: 0,
                $($xs,)*
            }
        }

        /// This documentation applies not only to `ExhaustiveUnion2s`, but also to
        /// `ExhaustiveUnion3s`, `ExhaustiveUnion4s`, and so on. See [`exhaustive_unions`] for more
        /// information.
        ///
        /// Generates all $n$-unions with elements from $n$ iterators.
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        $($vis)* struct $exhaustive_struct<$($t, $it: Iterator<Item=$t>),*> {
            done: bool,
            i: u64,
            $(
                $xs: $it,
                $xs_done: bool,
            )*
        }

        impl<$($t, $it: Iterator<Item=$t>),*> Iterator for $exhaustive_struct<$($t, $it),*> {
            type Item = $union<$($t),*>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    None
                } else {
                    let original_i = self.i;
                    loop {
                        let mut next = None;
                        match self.i {
                            $(
                                $i => if !self.$xs_done {
                                    next = self.$xs.next().map($union::$variant);
                                    self.$xs_done = next.is_none();
                                },
                            )*
                            _ => unreachable!(),
                        }
                        self.i += 1;
                        if self.i == $n {
                            self.i = 0;
                        }
                        if next.is_some() {
                            return next;
                        }
                        if self.i == original_i {
                            self.done = true;
                            return None;
                        }
                    }
                }
            }
        }

        /// This documentation applies not only to `exhaustive_union2s`, but also to
        /// `exhaustive_union3s`, `exhaustive_union4s`, and so on. See [`exhaustive_unions`] for
        /// more information.
        ///
        /// Generates all $n$-unions with elements from $n$ iterators.
        ///
        /// The input iterators are advanced in a round-robin fashion. First an element from the
        /// first variant's iterator is selected, followed by an element from the second variant's
        /// iterator, and so on until an element has been selected from each iterator. Then another
        /// element from the first iterator is selected, etc. Iterators that have been exhausted are
        /// skipped. [`exhaustive_union2s`] behaves just like
        /// [`Itertools::interleave`]([`itertools::Itertools::interleave`]).
        ///
        /// If all input iterators are finite, the output length is the sum of the lengths of the
        /// iterators. If any iterator is infinite, the output is also infinite.
        ///
        /// If all of the input iterators are empty, the output is also empty.
        ///
        /// # Examples
        /// See [here](self#exhaustive_union2s).
        #[allow(dead_code)]
        #[inline]
        $($vis)* const fn $exhaustive_fn<$($t, $it: Iterator<Item=$t>),*>($($xs: $it),*) ->
                $exhaustive_struct<$($t, $it),*> {
            $exhaustive_struct {
                done: false,
                i: 0,
                $(
                    $xs,
                    $xs_done: false,
                )*
            }
        }
    }
}

exhaustive_unions!(
    (pub),
    Union2,
    LexUnion2s,
    ExhaustiveUnion2s,
    lex_union2s,
    exhaustive_union2s,
    2,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done]
);
