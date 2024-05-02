// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::exhaustive::{exhaustive_chars, ExhaustiveChars};
use crate::num::exhaustive::PrimitiveIntIncreasingRange;
use crate::strings::{strings_from_char_vecs, StringsFromCharVecs};
use crate::vecs::exhaustive::{
    exhaustive_vecs, exhaustive_vecs_fixed_length_from_single, lex_vecs_fixed_length_from_single,
    shortlex_vecs, ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, LexFixedLengthVecsFromSingle,
    ShortlexVecs,
};

/// Generates all [`String`]s of a given length with [`char`]s from a single iterator, in
/// lexicographic order.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// `cs` must be finite.
///
/// The output length is $\ell^n$, where $\ell$ is `cs.count()` and $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty [`String`].
///
/// If `cs` is empty, the output is also empty, unless `len` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::lex_fixed_length_strings_using_chars;
///
/// let ss = lex_fixed_length_strings_using_chars(2, ['c', 'a', 't'].iter().cloned()).collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &["cc", "ca", "ct", "ac", "aa", "at", "tc", "ta", "tt"]
/// );
/// ```
#[inline]
pub fn lex_fixed_length_strings_using_chars<I: Iterator<Item = char>>(
    len: u64,
    cs: I,
) -> StringsFromCharVecs<LexFixedLengthVecsFromSingle<I>> {
    strings_from_char_vecs(lex_vecs_fixed_length_from_single(len, cs))
}

/// Generates all [`String`]s of a given length in lexicographic order.
///
/// The order is lexicographic with respect to the order of [`exhaustive_chars`], which is not the
/// default lexicographic order for [`char`]s. (For example, the first characters are not control
/// characters, but lowercase Latin letters.) If you want the default [`char`] order, use
/// `lex_fixed_length_strings_using_chars(len, chars_increasing())`.
///
/// The output length is $1112064^n$, where $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty [`String`].
///
/// # Complexity per iteration
/// $T(i, n) = O(n)$
///
/// $M(i, n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::lex_fixed_length_strings;
///
/// let ss = lex_fixed_length_strings(2).take(20).collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &[
///         "aa", "ab", "ac", "ad", "ae", "af", "ag", "ah", "ai", "aj", "ak", "al", "am", "an",
///         "ao", "ap", "aq", "ar", "as", "at"
///     ]
/// );
/// ```
#[inline]
pub fn lex_fixed_length_strings(
    len: u64,
) -> StringsFromCharVecs<LexFixedLengthVecsFromSingle<ExhaustiveChars>> {
    lex_fixed_length_strings_using_chars(len, exhaustive_chars())
}

/// Generates all `String`s of a given length with [`char`]s from a single iterator.
///
/// If `cs` is finite, the output length is $\ell^n$, where $\ell$ is `cs.count()` and $n$ is `len`.
/// If `cs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty [`String`].
///
/// If `cs` is empty, the output is also empty, unless `len` is 0.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings_using_chars;
///
/// let ss = exhaustive_fixed_length_strings_using_chars(2, ['c', 'a', 't'].iter().cloned())
///     .collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &["cc", "ca", "ac", "aa", "ct", "at", "tc", "ta", "tt"]
/// );
/// ```
#[inline]
pub fn exhaustive_fixed_length_strings_using_chars<I: Iterator<Item = char>>(
    len: u64,
    cs: I,
) -> StringsFromCharVecs<ExhaustiveFixedLengthVecs1Input<I>> {
    strings_from_char_vecs(exhaustive_vecs_fixed_length_from_single(len, cs))
}

/// Generates all [`String`]s of a given length.
///
/// The output length is $1112064^n$, where $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty [`String`].
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings;
///
/// let ss = exhaustive_fixed_length_strings(2).take(20).collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &[
///         "aa", "ab", "ba", "bb", "ac", "ad", "bc", "bd", "ca", "cb", "da", "db", "cc", "cd",
///         "dc", "dd", "ae", "af", "be", "bf"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_fixed_length_strings(
    len: u64,
) -> StringsFromCharVecs<ExhaustiveFixedLengthVecs1Input<ExhaustiveChars>> {
    exhaustive_fixed_length_strings_using_chars(len, exhaustive_chars())
}

/// Generates [`String`]s with [`char`]s from a specified iterator, in shortlex order.
///
/// Shortlex order means that the [`String`]s are output from shortest to longest, and [`String`]s
/// of the same length are output in lexicographic order with respect to the ordering of the
/// [`char`]s specified by the input iterator.
///
/// `cs` must be finite; if it's infinite, only [`String`]s of length 0 and 1 are ever produced.
///
/// If `cs` is empty, the output length is 1; otherwise, the output is infinite.
///
/// The lengths of the output [`String`]s grow logarithmically.
///
/// # Complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::shortlex_strings_using_chars;
///
/// let ss = shortlex_strings_using_chars('x'..='z')
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     ss.iter().map(String::as_str).collect_vec().as_slice(),
///     &[
///         "", "x", "y", "z", "xx", "xy", "xz", "yx", "yy", "yz", "zx", "zy", "zz", "xxx", "xxy",
///         "xxz", "xyx", "xyy", "xyz", "xzx"
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_strings_using_chars<I: Clone + Iterator<Item = char>>(
    cs: I,
) -> StringsFromCharVecs<ShortlexVecs<char, PrimitiveIntIncreasingRange<u64>, I>> {
    strings_from_char_vecs(shortlex_vecs(cs))
}

/// Generates [`String`]s in shortlex order.
///
/// Shortlex order means that the [`String`]s are output from shortest to longest, and [`String`]s
/// of the same length are output in lexicographic order with respect to the order of
/// [`exhaustive_chars`], which is not the default lexicographic order for [`char`]s. (For example,
/// the first characters are not control characters, but lowercase Latin letters.) If you want the
/// default [`char`] order, use `shortlex_strings_using_chars(chars_increasing())`.
///
/// The output is infinite.
///
/// The lengths of the output [`String`]s grow logarithmically.
///
/// # Complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::shortlex_strings;
///
/// let ss = shortlex_strings().take(20).collect_vec();
/// assert_eq!(
///     ss.iter().map(String::as_str).collect_vec().as_slice(),
///     &[
///         "", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
///         "q", "r", "s"
///     ]
/// );
/// ```
#[inline]
pub fn shortlex_strings(
) -> StringsFromCharVecs<ShortlexVecs<char, PrimitiveIntIncreasingRange<u64>, ExhaustiveChars>> {
    shortlex_strings_using_chars(exhaustive_chars())
}

/// Generates all [`String`]s with [`char`]s from a specified iterator.
///
/// If `cs` is empty, the output length is 1; otherwise, the output is infinite.
///
/// The lengths of the output [`String`]s grow logarithmically.
///
/// # Complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::exhaustive_strings_using_chars;
///
/// let ss = exhaustive_strings_using_chars('x'..='z')
///     .take(20)
///     .collect_vec();
/// assert_eq!(
///     ss.iter().map(String::as_str).collect_vec().as_slice(),
///     &[
///         "", "x", "y", "xxx", "z", "xx", "xy", "xxxxx", "yx", "xxy", "yy", "xxxx", "xz", "xyx",
///         "yz", "xxxxxx", "zx", "xyy", "zy", "xxxy"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_strings_using_chars<I: Clone + Iterator<Item = char>>(
    cs: I,
) -> StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, I>> {
    strings_from_char_vecs(exhaustive_vecs(cs))
}

/// Generates all [`String`]s.
///
/// The lengths of the output [`String`]s grow logarithmically.
///
/// # Complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::exhaustive::exhaustive_strings;
///
/// let ss = exhaustive_strings().take(20).collect_vec();
/// assert_eq!(
///     ss.iter().map(String::as_str).collect_vec().as_slice(),
///     &[
///         "", "a", "b", "aaa", "c", "aa", "d", "aaaa", "e", "ab", "f", "aab", "g", "ba", "h",
///         "aaaaa", "i", "bb", "j", "aba"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_strings(
) -> StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, ExhaustiveChars>> {
    exhaustive_strings_using_chars(exhaustive_chars())
}
