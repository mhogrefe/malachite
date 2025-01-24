// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::random::{random_chars, RandomCharRange};
use crate::num::random::geometric::GeometricRandomNaturalValues;
use crate::random::Seed;
use crate::strings::StringsFromCharVecs;
use crate::vecs::random::{
    random_vecs, random_vecs_fixed_length_from_single, RandomFixedLengthVecsFromSingle, RandomVecs,
};

/// Randomly generates [`String`]s of a given length using [`char`]s from a single iterator.
///
/// The probability of a particular length-$n$ [`String`] being generated is the product of the
/// probabilities of each of its `char`s.
///
/// If `len` is 0, the output consists of the empty [`String`], repeated.
///
/// `cs` must be infinite.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::random::random_fixed_length_strings_using_chars;
///
/// let ss = random_fixed_length_strings_using_chars(
///     2,
///     random_char_inclusive_range(EXAMPLE_SEED, 'a', 'c'),
/// )
/// .take(10)
/// .collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &["ba", "bc", "bb", "ab", "ac", "ba", "bc", "ca", "ba", "cc"]
/// );
/// ```
#[inline]
pub const fn random_fixed_length_strings_using_chars<I: Iterator<Item = char>>(
    len: u64,
    cs: I,
) -> StringsFromCharVecs<RandomFixedLengthVecsFromSingle<I>> {
    StringsFromCharVecs {
        css: random_vecs_fixed_length_from_single(len, cs),
    }
}

/// Randomly generates [`String`]s of a given length.
///
/// The probability of a particular length-$n$ [`String`] being generated is $1112064^{-\ell}$,
/// where $\ell$ is `len`.
///
/// If `len` is 0, the output consists of the empty [`String`], repeated.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::random::random_fixed_length_strings;
///
/// let ss = random_fixed_length_strings(EXAMPLE_SEED, 2)
///     .take(10)
///     .collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &[
///         "\u{5f771}\u{87234}",
///         "\u{bcd36}\u{9e195}",
///         "\u{5da07}\u{36553}",
///         "\u{45028}\u{1cdfd}",
///         "\u{d8530}\u{c7f2e}",
///         "\u{ba4bc}\u{ff677}",
///         "\u{a12e2}\u{d775c}",
///         "\u{f827b}\u{bdf7a}",
///         "簅\u{15aca}",
///         "\u{4e5e2}\u{bb286}"
///     ]
/// );
/// ```
#[inline]
pub fn random_fixed_length_strings(
    seed: Seed,
    len: u64,
) -> StringsFromCharVecs<RandomFixedLengthVecsFromSingle<RandomCharRange>> {
    random_fixed_length_strings_using_chars(len, random_chars(seed))
}

/// Generates random [`String`]s using [`char`]s from an iterator.
///
/// The lengths of the [`String`]s are sampled from a geometric distribution with a specified mean
/// $m$, equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// $$
/// P((c_0, c_1, \ldots, c_{n-1})) = \frac{m^n}{(m+1)^{n+1}}\prod_{i=0}^{n-1}P(c_i).
/// $$
///
/// The iterators produced by `cs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::random::random_strings_using_chars;
///
/// let ss = random_strings_using_chars(
///     EXAMPLE_SEED,
///     &|seed| random_char_inclusive_range(seed, 'x', 'z'),
///     4,
///     1,
/// )
/// .take(10)
/// .collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &["", "yyyyzxxxzxzxzx", "zzzy", "xzzx", "y", "", "zyzxz", "zy", "zyyx", ""]
/// );
/// ```
#[inline]
pub fn random_strings_using_chars<I: Iterator<Item = char>>(
    seed: Seed,
    cs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StringsFromCharVecs<RandomVecs<char, GeometricRandomNaturalValues<u64>, I>> {
    StringsFromCharVecs {
        css: random_vecs(seed, cs_gen, mean_length_numerator, mean_length_denominator),
    }
}

/// Generates random [`String`]s.
///
/// The lengths of the [`String`]s are sampled from a geometric distribution with a specified mean
/// $m$, equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// $$
/// P((c_0, c_1, \ldots, c_{n-1})) = \frac{m^n}{1112064^n(m+1)^{n+1}}
/// $$
///
/// # Expected complexity per iteration
/// $T(n) = O(m)$
///
/// $M(n) = O(m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::random::random_strings;
///
/// let ss = random_strings(EXAMPLE_SEED, 4, 1).take(10).collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &[
///         "",
///         "\u{81355}\u{a331d}\u{b707b}\u{1354b}\u{b16ac}𣙘\u{67377}\u{4aaa4}\u{a6d6e}\u{45616}\
///         \u{7725f}\u{41e2d}\u{d6b59}\u{de165}",
///         "\u{c2d29}\u{695af}\u{98fd7}\u{10ca51}",
///         "\u{bec46}\u{c0bec}\u{cb677}\u{71318}",
///         "\u{755e1}",
///         "",
///         "𫮜\u{a2f84}柂\u{f5560}\u{6737b}",
///         "\u{8442e}\u{a6883}",
///         "\u{49cf2}\u{32d2b}\u{1e6e5}\u{1084bd}",
///         ""
///     ]
/// );
/// ```
#[inline]
pub fn random_strings(
    seed: Seed,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StringsFromCharVecs<RandomVecs<char, GeometricRandomNaturalValues<u64>, RandomCharRange>> {
    random_strings_using_chars(
        seed,
        &random_chars,
        mean_length_numerator,
        mean_length_denominator,
    )
}
