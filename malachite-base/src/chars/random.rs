// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::random::{weighted_random_bools, WeightedRandomBools};
use crate::chars::char_is_graphic;
use crate::chars::crement::{char_to_contiguous_range, contiguous_range_to_char, decrement_char};
use crate::comparison::traits::Min;
use crate::num::random::{random_unsigned_inclusive_range, RandomUnsignedInclusiveRange};
use crate::random::Seed;
use crate::vecs::{random_values_from_vec, RandomValuesFromVec};

/// Uniformly generates random [`char`]s in a closed interval.
///
/// This `struct` is created by [`random_char_range`] and [`random_char_inclusive_range`]; see their
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomCharRange {
    chunks: RandomUnsignedInclusiveRange<u32>,
}

impl Iterator for RandomCharRange {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        contiguous_range_to_char(self.chunks.next().unwrap())
    }
}

/// Uniformly generates random [`char`]s in a closed interval, weighting graphic and non-graphic
/// [`char`]s separately.
///
/// This `struct` is created by [`graphic_weighted_random_char_range`] and
/// [`graphic_weighted_random_char_inclusive_range`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct WeightedGraphicRandomCharRange {
    xs: WeightedRandomBools,
    graphic: RandomValuesFromVec<char>,
    non_graphic: RandomValuesFromVec<char>,
}

impl Iterator for WeightedGraphicRandomCharRange {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.xs.next().unwrap() {
            self.graphic.next()
        } else {
            self.non_graphic.next()
        }
    }
}

/// Uniformly generates random [`char`]s.
///
/// $P(c) = \frac{1}{2^{20}+2^{16}-2^{11}}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_chars(EXAMPLE_SEED)
///         .take(10)
///         .collect::<String>()
///         .as_str(),
///     "\u{5f771}\u{87234}\u{bcd36}\u{9e195}\u{5da07}\u{36553}\u{45028}\u{1cdfd}\u{d8530}\u{c7f2e}"
/// )
/// ```
#[inline]
pub fn random_chars(seed: Seed) -> RandomCharRange {
    random_char_inclusive_range(seed, char::MIN, char::MAX)
}

/// Uniformly generates random ASCII [`char`]s.
///
/// $$
/// P(c) = \\begin{cases}
///     2^{-7} & \text{if} \\quad c < \\backslash\\text{u\\{0x80\\}} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_ascii_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_ascii_chars(EXAMPLE_SEED)
///         .take(20)
///         .collect::<String>()
///         .as_str(),
///     "q^\u{17}bF\\4T!/\u{1}q6\n/\u{11}Y\\wB"
/// )
/// ```
#[inline]
pub fn random_ascii_chars(seed: Seed) -> RandomCharRange {
    random_char_inclusive_range(seed, char::MIN, '\u{7f}')
}

/// Uniformly generates random [`char`]s in the half-open interval $[a, b)$.
///
/// $a$ must be less than $b$. This function cannot create a range that includes `char::MAX`; for
/// that, use [`random_char_inclusive_range`].
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}
///     {\mathrm{char\\_to\\_contiguous\\_range(b)}-\mathrm{char\\_to\\_contiguous\\_range(a)}} &
///         \text{if} \\quad a \leq x < b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_char_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_char_range(EXAMPLE_SEED, 'a', 'z')
///         .take(50)
///         .collect::<String>()
///         .as_str(),
///     "rlewrsgkdlbeouylrelopxqkoonftexoshqulgvonioatekqes"
/// )
/// ```
#[inline]
pub fn random_char_range(seed: Seed, a: char, mut b: char) -> RandomCharRange {
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    decrement_char(&mut b);
    random_char_inclusive_range(seed, a, b)
}

/// Uniformly generates random [`char`]s in the closed interval $[a, b]$.
///
/// $a$ must be less than or equal to $b$.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}
///         {\mathrm{char\\_to\\_contiguous\\_range(b)}-\mathrm{char\\_to\\_contiguous\\_range(a)}
///         +1} &
///         \text{if} \\quad a \leq x < b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_char_inclusive_range(EXAMPLE_SEED, 'a', 'z')
///         .take(50)
///         .collect::<String>()
///         .as_str(),
///     "rlewrsgkdlbeouylrelopxqkoonftexoshqulgvonioatekqes"
/// )
/// ```
#[inline]
pub fn random_char_inclusive_range(seed: Seed, a: char, b: char) -> RandomCharRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    RandomCharRange {
        chunks: random_unsigned_inclusive_range(
            seed,
            char_to_contiguous_range(a),
            char_to_contiguous_range(b),
        ),
    }
}

/// Generates random [`char`]s, weighting graphic and non-graphic [`char`]s separately.
///
/// See [`char_is_graphic`] for the definition of a graphic [`char`].
///
/// Let $n_p$ be `p_numerator` and $d_p$ be `p_denominator`, and let $p = p_n/p_d$.
///
/// The set of graphic [`char`]s is selected with probability $p$, and the set of non-graphic
/// [`char`]s with probability $1-p$. Then, a [`char`] is selected uniformly from the appropriate
/// set. There are 142,523 graphic [`char`]s out of 1,112,064, so we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{p}{142523} & \text{if} \\quad x \\ \\text{is} \\ \\text{graphic} \\\\
///     \frac{1-p}{969541} & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $p = 142523/1112064$, which is roughly $1/8$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `p_denominator` is zero or `p_denominator > p_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_chars(EXAMPLE_SEED, 10, 11)
///         .take(20)
///         .collect::<String>()
///         .as_str(),
///     "𘋈𰒏礝깶ꅊ杽쭡𬝱╱𣑺⢍𰰬瀋\u{8daa8}𱽊ՠ𦧴\u{3805c}\u{d7447}𪂆"
/// )
/// ```
#[inline]
pub fn graphic_weighted_random_chars(
    seed: Seed,
    p_numerator: u64,
    p_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    graphic_weighted_random_char_inclusive_range(
        seed,
        char::MIN,
        char::MAX,
        p_numerator,
        p_denominator,
    )
}

/// Generates random ASCII [`char`]s, weighting graphic and non-graphic [`char`]s separately.
///
/// See [`char_is_graphic`] for the definition of a graphic [`char`].
///
/// Let $n_p$ be `p_numerator` and $d_p$ be `p_denominator`, and let $p = p_n/p_d$.
///
/// The set of graphic ASCII [`char`]s is selected with probability $p$, and the set of non-graphic
/// ASCII [`char`]s with probability $1-p$. Then, a [`char`] is selected uniformly from the
/// appropriate set. There are 95 graphic ASCII [`char`]s out of 128, so we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{p}{95} & \text{if} \\quad
///     x < \\backslash\\text{u\\{0x80\\}} \\ \\text{and} \\ x \\ \\text{is graphic} \\\\
///     \frac{1-p}{33} & \text{if} \\quad
///     x < \\backslash\\text{u\\{0x80\\}} \\ \\text{and} \\ x \\ \\text{is not graphic} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $p = 95/128$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `p_denominator` is zero or `p_denominator > p_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_ascii_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_ascii_chars(EXAMPLE_SEED, 10, 11)
///         .take(40)
///         .collect::<String>()
///         .as_str(),
///     "x14N(bcXr$g)7\u{1b}/E+\u{8}\rf\u{2}\u{11}Y\u{11}Poo.$V2R.$V=6\u{13}\t\u{11}"
/// )
/// ```
#[inline]
pub fn graphic_weighted_random_ascii_chars(
    seed: Seed,
    p_numerator: u64,
    p_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    graphic_weighted_random_char_inclusive_range(
        seed,
        char::MIN,
        '\u{7f}',
        p_numerator,
        p_denominator,
    )
}

/// Generates random [`char`]s in the half-open interval $[a, b)$, weighting graphic and non-graphic
/// [`char`]s separately.
///
/// See [`char_is_graphic`] for the definition of a graphic [`char`].
///
/// Let $n_p$ be `p_numerator` and $d_p$ be `p_denominator`, and let $p = p_n/p_d$.
///
/// The set of graphic [`char`]s in the specified range is selected with probability $p$, and the
/// set of non-graphic [`char`]s in the range with probability $1-p$. Then, a [`char`] is selected
/// uniformly from the appropriate set.
///
/// $a$ must be less than $b$. Furthermore, $[a, b)$ must contain both graphic and non-graphic
/// [`char`]s. This function cannot create a range that includes `char::MAX`; for that, use
/// [`graphic_weighted_random_char_inclusive_range`].
///
/// Let $g$ be the number of graphic [`char`]s in $[a, b)$. Then we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{p}{g} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is graphic} \\\\
///     \frac{1-p}{b-a-g} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is not graphic} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $p = g/(b-a)$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `p_denominator` is zero or `p_denominator > p_denominator`, if $a \geq b$, if $[a, b)$
/// contains no graphic [`char`]s, or if $[a, b)$ contains only graphic [`char`]s.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_char_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_char_range(EXAMPLE_SEED, '\u{100}', '\u{400}', 10, 11)
///         .take(30)
///         .collect::<String>()
///         .as_str(),
///     "ǘɂŜȢΙƘƣʅΰǟ˳ˊȇ\u{31b}ʰɥΈ\u{324}\u{35a}Ϟ\u{367}\u{337}ƃ\u{342}ʌμƢϳϪǰ"
/// )
/// ```
#[inline]
pub fn graphic_weighted_random_char_range(
    seed: Seed,
    a: char,
    mut b: char,
    p_numerator: u64,
    p_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    decrement_char(&mut b);
    graphic_weighted_random_char_inclusive_range(seed, a, b, p_numerator, p_denominator)
}

/// Generates random [`char`]s in the closed interval $[a, b]$, weighting graphic and non-graphic
/// [`char`]s separately.
///
/// See [`char_is_graphic`] for the definition of a graphic [`char`].
///
/// Let $n_p$ be `p_numerator` and $d_p$ be `p_denominator`, and let $p = p_n/p_d$.
///
/// The set of graphic [`char`]s in the specified range is selected with probability $p$, and the
/// set of non-graphic [`char`]s in the range with probability $1-p$. Then, a [`char`] is selected
/// uniformly from the appropriate set.
///
/// $a$ must be less than $b$. Furthermore, $[a, b]$ must contain both graphic and non-graphic
/// [`char`]s. This function cannot create a range that includes `char::MAX`; for that, use
/// [`graphic_weighted_random_char_inclusive_range`].
///
/// Let $g$ be the number of graphic [`char`]s in $[a, b]$. Then we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{p}{g} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is graphic} \\\\
///     \frac{1-p}{b-a-g+1} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is not graphic} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $p = g/(b-a+1)$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `p_denominator` is zero or `p_denominator > p_denominator`, if $a > b$, if $[a, b]$
/// contains no graphic [`char`]s, or if $[a, b]$ contains only graphic [`char`]s.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_char_inclusive_range(EXAMPLE_SEED, '\u{100}', '\u{3ff}', 10, 11)
///         .take(30)
///         .collect::<String>()
///         .as_str(),
///     "ǘɂŜȢΙƘƣʅΰǟ˳ˊȇ\u{31b}ʰɥΈ\u{324}\u{35a}Ϟ\u{367}\u{337}ƃ\u{342}ʌμƢϳϪǰ"
/// )
/// ```
pub fn graphic_weighted_random_char_inclusive_range(
    seed: Seed,
    a: char,
    b: char,
    p_numerator: u64,
    p_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    let (graphic_chars, non_graphic_chars): (Vec<_>, Vec<_>) =
        (a..=b).partition(|&c| char_is_graphic(c));
    assert!(
        !graphic_chars.is_empty(),
        "The range {a:?}..={b:?} contains no graphic chars"
    );
    assert!(
        !non_graphic_chars.is_empty(),
        "The range {a:?}..={b:?} only contains graphic chars"
    );
    WeightedGraphicRandomCharRange {
        xs: weighted_random_bools(seed.fork("xs"), p_numerator, p_denominator),
        graphic: random_values_from_vec(seed.fork("graphic"), graphic_chars),
        non_graphic: random_values_from_vec(seed.fork("non_graphic"), non_graphic_chars),
    }
}
