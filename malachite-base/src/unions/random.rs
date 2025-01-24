// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::{random_unsigned_range, RandomUnsignedRange};
use crate::random::Seed;
use crate::unions::Union2;

/// Defines random union generators.
///
/// Malachite provides [`random_union2s`], but you can also define `random_union3s`,
/// `random_union4s`, and so on, in your program using the code below. The documentation for
/// [`random_union2s`] describes these other functions as well.
///
/// See usage examples [here](self#random_union2s).
///
/// ```
/// use malachite_base::num::random::{random_unsigned_range, RandomUnsignedRange};
/// use malachite_base::random::Seed;
/// use malachite_base::unions::UnionFromStrError;
/// use malachite_base::{random_unions, union_struct};
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
/// random_unions!(
///     (pub(crate)),
///     Union3,
///     RandomUnion3s,
///     random_union3s,
///     3,
///     [0, X, I, A, xs, xs_gen],
///     [1, Y, J, B, ys, ys_gen],
///     [2, Z, K, C, zs, zs_gen]
/// );
/// random_unions!(
///     (pub(crate)),
///     Union4,
///     RandomUnion4s,
///     random_union4s,
///     4,
///     [0, X, I, A, xs, xs_gen],
///     [1, Y, J, B, ys, ys_gen],
///     [2, Z, K, C, zs, zs_gen],
///     [3, W, L, D, ws, ws_gen]
/// );
/// random_unions!(
///     (pub(crate)),
///     Union5,
///     RandomUnion5s,
///     random_union5s,
///     5,
///     [0, X, I, A, xs, xs_gen],
///     [1, Y, J, B, ys, ys_gen],
///     [2, Z, K, C, zs, zs_gen],
///     [3, W, L, D, ws, ws_gen],
///     [4, V, M, E, vs, vs_gen]
/// );
/// random_unions!(
///     (pub(crate)),
///     Union6,
///     RandomUnion6s,
///     random_union6s,
///     6,
///     [0, X, I, A, xs, xs_gen],
///     [1, Y, J, B, ys, ys_gen],
///     [2, Z, K, C, zs, zs_gen],
///     [3, W, L, D, ws, ws_gen],
///     [4, V, M, E, vs, vs_gen],
///     [5, U, N, F, us, us_gen]
/// );
/// random_unions!(
///     (pub(crate)),
///     Union7,
///     RandomUnion7s,
///     random_union7s,
///     7,
///     [0, X, I, A, xs, xs_gen],
///     [1, Y, J, B, ys, ys_gen],
///     [2, Z, K, C, zs, zs_gen],
///     [3, W, L, D, ws, ws_gen],
///     [4, V, M, E, vs, vs_gen],
///     [5, U, N, F, us, us_gen],
///     [6, T, O, G, ts, ts_gen]
/// );
/// random_unions!(
///     (pub(crate)),
///     Union8,
///     RandomUnion8s,
///     random_union8s,
///     8,
///     [0, X, I, A, xs, xs_gen],
///     [1, Y, J, B, ys, ys_gen],
///     [2, Z, K, C, zs, zs_gen],
///     [3, W, L, D, ws, ws_gen],
///     [4, V, M, E, vs, vs_gen],
///     [5, U, N, F, us, us_gen],
///     [6, T, O, G, ts, ts_gen],
///     [7, S, P, H, ss, ss_gen]
/// );
/// ```
#[macro_export]
macro_rules! random_unions {
    (
        ($($vis:tt)*),
        $union: ident,
        $random_struct: ident,
        $random_fn: ident,
        $n: expr,
        $([$i: expr, $t: ident, $it: ident, $variant: ident, $xs: ident, $xs_gen: ident]),*
    ) => {
        /// This documentation applies not only to `RandomUnion2s`, but also to `RandomUnion3s`,
        /// `RandomUnion4s`, and so on. See [`random_unions`] for more information.
        ///
        /// Generates random $n$-unions with elements from $n$ iterators.
        #[derive(Clone, Debug)]
        $($vis)* struct $random_struct<$($t, $it: Iterator<Item=$t>),*> {
            indices: RandomUnsignedRange<usize>,
            $($xs: $it,)*
        }

        impl<$($t, $it: Iterator<Item=$t>),*> Iterator for $random_struct<$($t, $it),*> {
            type Item = $union<$($t),*>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.indices.next().unwrap() {
                    $($i => self.$xs.next().map($union::$variant),)*
                    _ => unreachable!(),
                }
            }
        }

        /// This documentation applies not only to `random_union2s`, but also to `random_union3s`,
        /// `random_union4s`, and so on. See [`random_unions`] for more information.
        ///
        /// Generates random $n$-unions with elements from $n$ iterators.
        ///
        /// The probability of a particular $n$-union being generated is the probability of its
        /// element divided by $n$.
        ///
        /// `xs`, `ys`, `zs`, ... must be infinite.
        ///
        /// # Examples
        /// See [here](self#random_union2s).
        $($vis)* fn $random_fn<$($t, $it: Iterator<Item=$t>),*>(
            seed: Seed, $($xs_gen: &dyn Fn(Seed) -> $it),*
        ) -> $random_struct<$($t, $it),*> {
            $random_struct {
                indices: random_unsigned_range(seed.fork("indices"), 0, $n),
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
            }
        }
    }
}
random_unions!(
    (pub),
    Union2,
    RandomUnion2s,
    random_union2s,
    2,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen]
);
