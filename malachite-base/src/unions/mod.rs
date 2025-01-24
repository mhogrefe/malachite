// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use alloc::string::String;
use alloc::string::ToString;
use core::fmt::{self, Display, Formatter};
use core::str::FromStr;

/// This is the error type for the unions' [`FromStr`] implementations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnionFromStrError<E> {
    /// For when the union's variant can't be determined.
    Generic(String),
    /// For when the union's variant can be determined but the wrapped value can't be parsed.
    Specific(E),
}

/// Defines unions.
///
/// Malachite provides [`Union2`], but you can also define `Union3`, `Union4`, and so on, in your
/// program using the code below. The documentation for [`Union2`] and describes these other `enum`s
/// as well.
///
/// ```
/// use malachite_base::union_struct;
/// use malachite_base::unions::UnionFromStrError;
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
/// ```
#[macro_export]
macro_rules! union_struct {
    (
        ($($vis:tt)*),
        $name: ident,
        $single: ty,
        $([$t: ident, $cons: ident, $c: expr, $x: ident]),*
    ) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        /// This is a union, or sum type, of $n$ values. It is essentially a generic enum.
        $($vis)* enum $name<$($t),*> {
            $($cons($t)),*
        }

        impl<T> $single {
            /// Given a union whose variants all have the same type, unwraps it into a value of that
            /// type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](self#unwrap).
            #[allow(clippy::missing_const_for_fn)] // Can't be const because of destructor
            $($vis)* fn unwrap(self) -> T {
                match self {
                    $(
                        $name::$cons($x) => $x
                    ),*
                }
            }
        }

        impl<$($t: Display),*> Display for $name<$($t),*> {
            /// Converts a union to a [`String`].
            ///
            /// # Examples
            /// See [here](self#fmt).
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    $(
                        $name::$cons($x) => f.write_fmt(format_args!("{}({})", $c, $x))
                    ),*
                }
            }
        }

        impl<$($t: FromStr),*> FromStr for $name<$($t),*> {
            type Err = UnionFromStrError<$name<$($t::Err),*>>;

            /// Converts a string to a union.
            ///
            /// If the string does not represent a valid union, an error value is returned.
            ///
            /// # Examples
            /// See [here](self#from_str).
            #[inline]
            fn from_str(src: &str) -> Result<$name<$($t),*>, Self::Err> {
                if src.is_empty() {
                    return Err(UnionFromStrError::Generic(String::new()));
                }
                let (head, tail) = src.split_at(1);
                let tail = if let Some(tail) = tail.strip_prefix('(') {
                    tail
                } else {
                    return Err(UnionFromStrError::Generic(src.to_string()));
                };
                let tail = if let Some(tail) = tail.strip_suffix(')') {
                    tail
                } else {
                    return Err(UnionFromStrError::Generic(src.to_string()));
                };
                match head.chars().next().unwrap() {
                    $(
                        $c => $t::from_str(tail)
                                .map($name::$cons)
                                .map_err(|e| UnionFromStrError::Specific($name::$cons(e))),
                    )*
                    _ => Err(UnionFromStrError::Generic(src.to_string()))
                }
            }
        }
    }
}

union_struct!((pub), Union2, Union2<T, T>, [A, A, 'A', a], [B, B, 'B', b]);

/// Iterators that generate unions without repetition.
///
/// # lex_union2s
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::unions::exhaustive::lex_union2s;
/// use malachite_base::unions::Union2;
///
/// let u2s = lex_union2s(exhaustive_bools(), 0..4).collect_vec();
/// assert_eq!(
///     u2s.as_slice(),
///     &[
///         Union2::A(false),
///         Union2::A(true),
///         Union2::B(0),
///         Union2::B(1),
///         Union2::B(2),
///         Union2::B(3)
///     ]
/// );
/// ```
///
/// # exhaustive_union2s
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::unions::exhaustive::exhaustive_union2s;
/// use malachite_base::unions::Union2;
///
/// let u2s = exhaustive_union2s(exhaustive_bools(), 0..4).collect_vec();
/// assert_eq!(
///     u2s.as_slice(),
///     &[
///         Union2::A(false),
///         Union2::B(0),
///         Union2::A(true),
///         Union2::B(1),
///         Union2::B(2),
///         Union2::B(3)
///     ]
/// );
/// ```
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate unions randomly.
///
/// # random_union2s
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::unions::random::random_union2s;
/// use malachite_base::unions::Union2;
///
/// let us = random_union2s(
///     EXAMPLE_SEED,
///     &|seed| random_char_inclusive_range(seed, 'a', 'z'),
///     &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 10),
/// );
/// assert_eq!(
///     us.take(20).collect_vec().as_slice(),
///     &[
///         Union2::A('v'),
///         Union2::B(3),
///         Union2::A('c'),
///         Union2::A('q'),
///         Union2::A('i'),
///         Union2::A('e'),
///         Union2::A('p'),
///         Union2::A('g'),
///         Union2::A('s'),
///         Union2::B(7),
///         Union2::A('n'),
///         Union2::A('t'),
///         Union2::B(9),
///         Union2::A('m'),
///         Union2::A('z'),
///         Union2::B(7),
///         Union2::B(9),
///         Union2::A('o'),
///         Union2::A('m'),
///         Union2::B(3),
///     ],
/// );
/// ```
pub mod random;
