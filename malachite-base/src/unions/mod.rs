use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// This is the error type for the unions' `FromStr` implementations.
///
/// The `Generic` variant is for when the union's variant can't be determined, and the `Specific`
/// variant is for when the union's variant can be determined but the wrapped value can't be parsed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnionFromStrError<E> {
    Generic(String),
    Specific(E),
}

macro_rules! union {
    ($name: ident, $single: ty, $([$t: ident, $cons: ident, $c: expr, $x: ident]),*) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        /// This is a union, or sum type, of $n$ values. It is essentially a generic enum.
        ///
        /// This enum is macro-generated. The value of $n$, the union's arity, is in the enum's
        /// name.
        pub enum $name<$($t),*> {
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
            /// See the documentation of the `unions` module.
            #[allow(clippy::missing_const_for_fn)] // Can't be const because of destructor
            pub fn unwrap(self) -> T {
                match self {
                    $(
                        $name::$cons($x) => $x
                    ),*
                }
            }
        }

        impl<$($t: Display),*> Display for $name<$($t),*> {
            /// Converts a union to a `String`.
            ///
            /// # Worst-case complexity
            /// The time and additional memory complexities are the maxima of the time and memory
            /// complexities of the `Display::fmt` implementations of the variant types.
            ///
            /// # Examples
            /// See the documentation of the `unions` module.
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

            /// Converts a `String` to a union.
            ///
            /// If the `&str` does not represent a valid union, an error value is returned.
            ///
            /// # Worst-case complexity
            /// The time and additional memory complexities are the maxima of the time and memory
            /// complexities of the `FromStr::from_str` implementations of the variant types, or
            /// $O(n)$, whichever is greater.
            ///
            /// # Examples
            /// See the documentation of the `unions` module.
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

union!(Union2, Union2<T, T>, [A, A, 'A', a], [B, B, 'B', b]);
union!(Union3, Union3<T, T, T>, [A, A, 'A', a], [B, B, 'B', b], [C, C, 'C', c]);
union!(Union4, Union4<T, T, T, T>, [A, A, 'A', a], [B, B, 'B', b], [C, C, 'C', c], [D, D, 'D', d]);
union!(
    Union5,
    Union5<T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e]
);
union!(
    Union6,
    Union6<T, T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e],
    [F, F, 'F', f]
);
union!(
    Union7,
    Union7<T, T, T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e],
    [F, F, 'F', f],
    [G, G, 'G', g]
);
union!(
    Union8,
    Union8<T, T, T, T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e],
    [F, F, 'F', f],
    [G, G, 'G', g],
    [H, H, 'H', h]
);

/// This module contains iterators that generate unions without repetition.
///
/// Here are usage examples of the macro-generated functions:
///
/// # lex_union\[n\]s
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::unions::exhaustive::lex_union2s;
/// use malachite_base::unions::Union2;
///
/// let u2s = lex_union2s(exhaustive_bools(), 0..4).collect_vec();
/// assert_eq!(
///     u2s.as_slice(),
///     &[Union2::A(false), Union2::A(true), Union2::B(0), Union2::B(1), Union2::B(2), Union2::B(3)]
/// );
/// ```
///
/// # exhaustive_union\[n\]s
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::bools::exhaustive::exhaustive_bools;
/// use malachite_base::unions::exhaustive::exhaustive_union2s;
/// use malachite_base::unions::Union2;
///
/// let u2s = exhaustive_union2s(exhaustive_bools(), 0..4).collect_vec();
/// assert_eq!(
///     u2s.as_slice(),
///     &[Union2::A(false), Union2::B(0), Union2::A(true), Union2::B(1), Union2::B(2), Union2::B(3)]
/// );
/// ```
pub mod exhaustive;
/// This module contains iterators that generate unions randomly.
pub mod random;
