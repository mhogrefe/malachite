// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::random::{
    get_random_natural_less_than, get_random_natural_with_bits,
    get_striped_random_natural_with_bits, random_naturals_less_than,
    striped_random_natural_inclusive_range, RandomNaturalsLessThan,
    StripedRandomNaturalInclusiveRange,
};
use crate::natural::Natural;
use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::num::arithmetic::traits::{PowerOf2, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::geometric::{
    geometric_random_natural_signeds, geometric_random_negative_signeds,
    geometric_random_nonzero_signeds, geometric_random_positive_signeds,
    geometric_random_signed_inclusive_range, geometric_random_signeds,
    get_geometric_random_signed_from_inclusive_range, GeometricRandomNaturalValues,
    GeometricRandomNegativeSigneds, GeometricRandomNonzeroSigneds, GeometricRandomSignedRange,
    GeometricRandomSigneds,
};
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::num::random::{
    random_primitive_ints, RandomPrimitiveInts, VariableRangeGenerator,
};
use malachite_base::random::Seed;

/// Generates random [`Integer`]s, given an iterator of random signed bit lengths.
///
/// The [`Integer`]'s signs are taken from the signs of the bit lengths.
#[derive(Clone, Debug)]
pub struct RandomIntegers<I: Iterator<Item = i64>> {
    bits: I,
    limbs: RandomPrimitiveInts<u64>,
}

impl<I: Iterator<Item = i64>> Iterator for RandomIntegers<I> {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let bits = self.bits.next().unwrap();
        Some(Integer::from_sign_and_abs(
            bits >= 0,
            get_random_natural_with_bits(&mut self.limbs, bits.unsigned_abs()),
        ))
    }
}

/// Generates random natural (non-negative) [`Integer`]s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 0. Then an [`Integer`]
/// is chosen uniformly among all non-negative [`Integer`]s with that bit length. The resulting
/// distribution resembles a Pareto distribution. It has no mean or higher-order statistics (unless
/// $m < 1$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & \text{if} \\quad n < 0, \\\\
///     \\frac{1}{m + 1} & \text{if} \\quad n = 0, \\\\
///     \\frac{2}{m+1} \\left ( \\frac{m}{2(m+1)} \\right ) ^ {\\lfloor \\log_2 n \\rfloor + 1} &
///     \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or, if after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_natural_integers;
///
/// assert_eq!(
///     prefix_to_string(random_natural_integers(EXAMPLE_SEED, 32, 1), 10),
///     "[20431208470830262, 2777240, 114, 12184833305054, 1121025855008623490210, \
///     13478874522577592, 115311695, 7, 18, 54522366353, ...]"
/// )
/// ```
pub fn random_natural_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegers<GeometricRandomNaturalValues<i64>> {
    RandomIntegers {
        bits: geometric_random_natural_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates random positive [`Integer`]s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then an [`Integer`]
/// is chosen uniformly among all positive [`Integer`]s with that bit length. The resulting
/// distribution resembles a Pareto distribution. It has no mean or higher-order statistics (unless
/// $m < 2$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & \text{if} \\quad n \leq 0, \\\\
///     \frac{1}{m} \left ( \frac{m-1}{2m} \right ) ^ {\lfloor \log_2 n \rfloor} &
///         \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_positive_integers;
///
/// assert_eq!(
///     prefix_to_string(random_positive_integers(EXAMPLE_SEED, 32, 1), 10),
///     "[22, 4, 178, 55845661150, 93254818, 7577967529619388, 8, 11316951483471, 11, \
///     1005760138411689342464923704482, ...]"
/// )
/// ```
pub fn random_positive_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegers<GeometricRandomNaturalValues<i64>> {
    RandomIntegers {
        bits: geometric_random_positive_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates random negative [`Integer`]s whose absolute values have a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then an [`Integer`]
/// is chosen uniformly among all positive [`Integer`]s with that bit length, and negated. The
/// resulting distribution resembles a negated Pareto distribution. It has no mean or higher-order
/// statistics (unless $m < 2$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & \text{if} \\quad n \geq 0 ,\\\\
///     \frac{1}{m} \left ( \frac{m-1}{2m} \right ) ^ {\lfloor \log_2 (-n) \rfloor}
///     & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_negative_integers;
///
/// assert_eq!(
///     prefix_to_string(random_negative_integers(EXAMPLE_SEED, 32, 1), 10),
///     "[-22, -4, -178, -55845661150, -93254818, -7577967529619388, -8, -11316951483471, -11, \
///     -1005760138411689342464923704482, ...]"
/// )
/// ```
pub fn random_negative_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegers<GeometricRandomNegativeSigneds<i64>> {
    RandomIntegers {
        bits: geometric_random_negative_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates random nonzero [`Integer`]s whose absolute values have a specified mean bit length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// [`geometric_random_nonzero_signeds`]); $m$ must be greater than 1. Then an [`Integer`] is chosen
/// uniformly among all positive [`Integer`]s with that bit length, and its sign is set to the sign
/// of the signed bit length. The resulting distribution has no mean or higher-order statistics
/// (unless $m < 2$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & \text{if} \\quad n = 0, \\\\
///     \\frac{1}{2m} \\left ( \\frac{m-1}{2m} \\right ) ^ {\\lfloor \\log_2 |n| \\rfloor}
///     & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_nonzero_integers;
///
/// assert_eq!(
///     prefix_to_string(random_nonzero_integers(EXAMPLE_SEED, 32, 1), 10),
///     "[6, 373973144, 46887963477285686350042496363292819122, -93254818, -126908, \
///     -4471675267836600, 1860142159, -118004986915853475, -98, 346513, ...]"
/// )
/// ```
pub fn random_nonzero_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegers<GeometricRandomNonzeroSigneds<i64>> {
    RandomIntegers {
        bits: geometric_random_nonzero_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates random [`Integer`]s whose absolute values have a specified mean bit length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// [`geometric_random_signeds`]); $m$ must be greater than 0. Then an [`Integer`] is chosen
/// uniformly among all [`Integer`]s with that bit length, and its sign is set to the sign of the
/// signed bit length. The resulting distribution has no mean or higher-order statistics (unless $m
/// < 1$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     \\frac{1}{2m+1} & \text{if} \\quad n = 0, \\\\
///     \\frac{2}{2m+1} \\left ( \\frac{m}{2(m+1)} \\right ) ^ {\\lfloor \\log_2 |n| \\rfloor + 1}
///     & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or, if after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_integers;
///
/// assert_eq!(
///     prefix_to_string(random_integers(EXAMPLE_SEED, 32, 1), 10),
///     "[89270, 69403499476962893258904, 62, -1848070042786, -64671510460, -696, 0, -79, 70819, \
///     7330, ...]"
/// )
/// ```
pub fn random_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegers<GeometricRandomSigneds<i64>> {
    RandomIntegers {
        bits: geometric_random_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates striped random [`Integer`]s, given an iterator of random signed bit lengths.
///
/// The [`Integer`]s signs are taken from the signs of the bit lengths.
#[derive(Clone, Debug)]
pub struct StripedRandomIntegers<I: Iterator<Item = i64>> {
    bits: I,
    bit_source: StripedBitSource,
}

impl<I: Iterator<Item = i64>> Iterator for StripedRandomIntegers<I> {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let bits = self.bits.next().unwrap();
        Some(Integer::from_sign_and_abs(
            bits >= 0,
            get_striped_random_natural_with_bits(&mut self.bit_source, bits.unsigned_abs()),
        ))
    }
}

/// Generates striped random natural (non-negative) [`Integer`]s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Integer`] is generated from the sequence.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or, if
/// after being reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_natural_integers;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_integers(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[18014656207519744, 2228160, 64, 17592184995840, 1179440951012584587264, \
///     9007749010526207, 67108864, 5, 24, 34359738879, ...]"
/// )
/// ```
pub fn striped_random_natural_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegers<GeometricRandomNaturalValues<i64>> {
    StripedRandomIntegers {
        bits: geometric_random_natural_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates striped random positive [`Integer`]s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Integer`] is generated from the sequence.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_positive_integers;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_positive_integers(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[16, 4, 128, 34391195648, 75493376, 9007199120523391, 8, 8796094070783, 8, \
///     950737950171027935941967741439, ...]"
/// )
/// ```
pub fn striped_random_positive_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegers<GeometricRandomNaturalValues<i64>> {
    StripedRandomIntegers {
        bits: geometric_random_positive_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates striped random negative [`Integer`]s whose absolute values have a specified mean bit
/// length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Integer`] is generated from the sequence and negated.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_negative_integers;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_negative_integers(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[-16, -4, -128, -34391195648, -75493376, -9007199120523391, -8, -8796094070783, -8, \
///     -950737950171027935941967741439, ...]"
/// )
/// ```
pub fn striped_random_negative_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegers<GeometricRandomNegativeSigneds<i64>> {
    StripedRandomIntegers {
        bits: geometric_random_negative_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates striped random nonzero [`Integer`]s whose absolute values have a specified mean bit
/// length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// [`geometric_random_nonzero_signeds`]); $m$ must be greater than 1. A striped bit sequence with
/// the given stripe parameter is generated and truncated at the bit length. The highest bit is
/// forced to be 1, an [`Integer`] is generated from the sequence, and its sign is set to the sign
/// of the signed bit length. The resulting distribution has no mean or higher-order statistics
/// (unless $m < 2$, which is not typical).
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_nonzero_integers;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_nonzero_integers(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[4, 268435456, 84405977732342160290572740160760316144, -133169152, -131064, \
///     -2251834173421823, 1577058304, -126100789566374399, -76, 270335, ...]"
/// )
/// ```
pub fn striped_random_nonzero_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegers<GeometricRandomNonzeroSigneds<i64>> {
    StripedRandomIntegers {
        bits: geometric_random_nonzero_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates striped random [`Integer`]s whose absolute values have a specified mean bit length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// [`geometric_random_signeds`]); $m$ must be greater than 0. A striped bit sequence with the given
/// stripe parameter is generated and truncated at the bit length. The highest bit is forced to be
/// 1, an [`Integer`] is generated from the sequence, and its sign is set to the sign of the signed
/// bit length. The resulting distribution has no mean or higher-order statistics (unless $m < 1$,
/// which is not typical).
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or, if
/// after being reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_integers;
///
/// assert_eq!(
///     prefix_to_string(striped_random_integers(EXAMPLE_SEED, 16, 1, 32, 1), 10),
///     "[65536, 75521006248971741167616, 32, -2199023255520, -68719468544, -527, 0, -112, \
///     131071, 4152, ...]"
/// )
/// ```
pub fn striped_random_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegers<GeometricRandomSigneds<i64>> {
    StripedRandomIntegers {
        bits: geometric_random_signeds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Uniformly generates random [`Integer`]s in an interval.
#[derive(Clone, Debug)]
pub struct UniformRandomIntegerRange {
    xs: RandomNaturalsLessThan,
    a: Integer,
}

impl Iterator for UniformRandomIntegerRange {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        self.xs.next().map(|x| &self.a + Integer::from(x))
    }
}

/// Uniformly generates random [`Integer`]s in the half-open interval $[a, b)$.
///
/// $a$ must be less than $b$.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a} & \text{if} \\quad a \leq x < b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
/// b.significant_bits())`.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::uniform_random_integer_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         uniform_random_integer_range(EXAMPLE_SEED, Integer::from(-10), Integer::from(100)),
///         10
///     ),
///     "[77, 83, -3, 95, 94, 97, 74, 17, 36, 83, ...]"
/// )
/// ```
pub fn uniform_random_integer_range(
    seed: Seed,
    a: Integer,
    b: Integer,
) -> UniformRandomIntegerRange {
    assert!(a < b);
    UniformRandomIntegerRange {
        xs: random_naturals_less_than(seed, Natural::exact_from(b - &a)),
        a,
    }
}

/// Uniformly generates random [`Integer`]s in the closed interval $[a, b]$.
///
/// $a$ must be less than or equal to $b$.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a+1} & \text{if} \\quad a \leq x \leq b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
/// b.significant_bits())`.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         uniform_random_integer_inclusive_range(
///             EXAMPLE_SEED,
///             Integer::from(-10),
///             Integer::from(100)
///         ),
///         10
///     ),
///     "[77, 83, -3, 95, 94, 97, 74, 17, 36, 83, ...]"
/// )
/// ```
#[inline]
pub fn uniform_random_integer_inclusive_range(
    seed: Seed,
    a: Integer,
    b: Integer,
) -> UniformRandomIntegerRange {
    assert!(a <= b);
    uniform_random_integer_range(seed, a, b + Integer::ONE)
}

/// Generates a random [`Integer`] in the half-open interval $[a, b)$.
///
/// The [`Integer`] is chosen uniformly from the interval.
///
/// $$
/// P(n) = \\begin{cases}
///     1/(b-a) & \text{if} a\leq n<b, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $\ell$ is `limit`.
///
/// # Expected complexity
/// $T(n) = O(n)$
///
/// $M(m) = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_bits_numerator +
/// mean_bits_denominator`, and $m$ is `max(a.significant_bits(), b.significant_bits())`.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::get_uniform_random_integer_in_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     get_uniform_random_integer_in_range(
///         &mut random_primitive_ints(EXAMPLE_SEED),
///         Integer::from(500u32),
///         Integer::from(1000u32)
///     ),
///     869
/// );
/// ```
pub fn get_uniform_random_integer_in_range(
    limbs: &mut RandomPrimitiveInts<u64>,
    a: Integer,
    b: Integer,
) -> Integer {
    assert!(a < b);
    Integer::from(get_random_natural_less_than(
        limbs,
        &Natural::exact_from(b - &a),
    )) + a
}

/// Generates a random [`Integer`] in the closed interval $[a, b]$.
///
/// The [`Integer`] is chosen uniformly from the interval.
///
/// $$
/// P(n) = \\begin{cases}
///     1/(b-a+1) & \text{if} a\leq n\leq b, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $\ell$ is `limit`.
///
/// # Expected complexity
/// $T(n) = O(n)$
///
/// $M(m) = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_bits_numerator +
/// mean_bits_denominator`, and $m$ is `max(a.significant_bits(), b.significant_bits())`.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::get_uniform_random_integer_in_inclusive_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     get_uniform_random_integer_in_inclusive_range(
///         &mut random_primitive_ints(EXAMPLE_SEED),
///         Integer::from(500u32),
///         Integer::from(1001u32)
///     ),
///     869
/// );
/// ```
pub fn get_uniform_random_integer_in_inclusive_range(
    limbs: &mut RandomPrimitiveInts<u64>,
    a: Integer,
    b: Integer,
) -> Integer {
    assert!(a <= b);
    get_uniform_random_integer_in_range(limbs, a, b + Integer::ONE)
}

fn signed_significant_bits(a: &Integer) -> (u64, i64) {
    let unsigned_bits = a.significant_bits();
    let bits = if *a >= 0 {
        i64::exact_from(unsigned_bits)
    } else {
        -i64::exact_from(unsigned_bits)
    };
    (unsigned_bits, bits)
}

fn signed_min_bit_range(
    seed: Seed,
    a: Integer,
    unsigned_min_bits: u64,
) -> UniformRandomIntegerRange {
    if a >= 0 {
        uniform_random_integer_range(seed, a, Integer::power_of_2(unsigned_min_bits))
    } else {
        uniform_random_integer_inclusive_range(seed, a, -Integer::power_of_2(unsigned_min_bits - 1))
    }
}

fn signed_max_bit_range(
    seed: Seed,
    a: Integer,
    unsigned_max_bits: u64,
) -> UniformRandomIntegerRange {
    if a > 0 {
        uniform_random_integer_inclusive_range(seed, Integer::power_of_2(unsigned_max_bits - 1), a)
    } else {
        // also handles a == 0
        uniform_random_integer_inclusive_range(
            seed,
            -Integer::power_of_2(unsigned_max_bits) + Integer::ONE,
            a,
        )
    }
}

fn get_random_integer_in_signed_min_bit_range(
    limbs: &mut RandomPrimitiveInts<u64>,
    a: Integer,
    unsigned_min_bits: u64,
) -> Integer {
    if a >= 0 {
        get_uniform_random_integer_in_range(limbs, a, Integer::power_of_2(unsigned_min_bits))
    } else {
        get_uniform_random_integer_in_inclusive_range(
            limbs,
            a,
            -Integer::power_of_2(unsigned_min_bits - 1),
        )
    }
}

fn get_random_integer_in_signed_max_bit_range(
    limbs: &mut RandomPrimitiveInts<u64>,
    a: Integer,
    unsigned_max_bits: u64,
) -> Integer {
    if a > 0 {
        get_uniform_random_integer_in_inclusive_range(
            limbs,
            Integer::power_of_2(unsigned_max_bits - 1),
            a,
        )
    } else {
        // also handles a == 0
        get_uniform_random_integer_in_inclusive_range(
            limbs,
            -Integer::power_of_2(unsigned_max_bits) + Integer::ONE,
            a,
        )
    }
}

/// Generates random [`Integer`]s greater than or equal to a lower bound, or less than or equal to
/// an upper bound.
#[derive(Clone, Debug)]
pub struct RandomIntegerRangeToInfinity {
    boundary_bits: i64,
    bits: GeometricRandomSignedRange<i64>,
    limbs: RandomPrimitiveInts<u64>,
    boundary_bit_xs: UniformRandomIntegerRange,
}

impl Iterator for RandomIntegerRangeToInfinity {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let bits = self.bits.next().unwrap();
        if bits == self.boundary_bits {
            self.boundary_bit_xs.next()
        } else {
            Some(Integer::from_sign_and_abs(
                bits >= 0,
                get_random_natural_with_bits(&mut self.limbs, bits.unsigned_abs()),
            ))
        }
    }
}

/// Generates random [`Integer`]s greater than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the absolute values of the generated values is specified. $m$ is
/// equal to `mean_bits_numerator / mean_bits_denominator`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a > 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_integer_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_range_to_infinity(EXAMPLE_SEED, Integer::from(-1000), 10, 1),
///         10
///     ),
///     "[15542, 2, 1714, 27863518, -162, 956, 8, 14648399, -419, -98, ...]"
/// )
/// ```
pub fn random_integer_range_to_infinity(
    seed: Seed,
    a: Integer,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerRangeToInfinity {
    let (unsigned_min_bits, min_bits) = signed_significant_bits(&a);
    RandomIntegerRangeToInfinity {
        boundary_bits: min_bits,
        bits: geometric_random_signed_inclusive_range(
            seed.fork("bits"),
            min_bits,
            i64::MAX,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
        boundary_bit_xs: signed_min_bit_range(seed.fork("min_bit_xs"), a, unsigned_min_bits),
    }
}

/// Generates a random [`Integer`] greater than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the absolute values of the generated value is specified. $m$ is equal
/// to `mean_bits_numerator / mean_bits_denominator`.
///
/// # Expected complexity
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a > 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::num::random::variable_range_generator;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::get_random_integer_from_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     get_random_integer_from_range_to_infinity(
///         &mut random_primitive_ints(EXAMPLE_SEED.fork("limbs")),
///         &mut variable_range_generator(EXAMPLE_SEED.fork("rg")),
///         Integer::from(-1000),
///         20,
///         1
///     ),
///     -2
/// )
/// ```
pub fn get_random_integer_from_range_to_infinity(
    limbs: &mut RandomPrimitiveInts<u64>,
    range_generator: &mut VariableRangeGenerator,
    a: Integer,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Integer {
    let (unsigned_min_bits, min_bits) = signed_significant_bits(&a);
    let bits = get_geometric_random_signed_from_inclusive_range(
        range_generator,
        min_bits,
        i64::MAX,
        mean_bits_numerator,
        mean_bits_denominator,
    );
    if bits == min_bits {
        get_random_integer_in_signed_min_bit_range(limbs, a, unsigned_min_bits)
    } else {
        Integer::from_sign_and_abs(
            bits >= 0,
            get_random_natural_with_bits(limbs, bits.unsigned_abs()),
        )
    }
}

/// Generates random [`Integer`]s less than or equal to an upper bound $a$.
///
/// The mean bit length $m$ of the absolute values of the generated values is specified. $m$ is
/// equal to `mean_bits_numerator / mean_bits_denominator`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a < 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_integer_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_range_to_negative_infinity(EXAMPLE_SEED, Integer::from(1000), 10, 1),
///         10
///     ),
///     "[6, 2, -1714, -235958584061012446, -455842, 514, -12, -14936760, 335, 99, ...]"
/// )
/// ```
pub fn random_integer_range_to_negative_infinity(
    seed: Seed,
    a: Integer,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerRangeToInfinity {
    let (unsigned_max_bits, max_bits) = signed_significant_bits(&a);
    RandomIntegerRangeToInfinity {
        boundary_bits: max_bits,
        bits: geometric_random_signed_inclusive_range(
            seed.fork("bits"),
            i64::MIN,
            max_bits,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
        boundary_bit_xs: signed_max_bit_range(seed.fork("max_bit_xs"), a, unsigned_max_bits),
    }
}

/// Generates a random [`Integer`] less than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the absolute values of the generated value is specified. $m$ is equal
/// to `mean_bits_numerator / mean_bits_denominator`.
///
/// # Expected complexity
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a < 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::num::random::variable_range_generator;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::get_random_integer_from_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     get_random_integer_from_range_to_negative_infinity(
///         &mut random_primitive_ints(EXAMPLE_SEED.fork("limbs")),
///         &mut variable_range_generator(EXAMPLE_SEED.fork("rg")),
///         Integer::from(-1000),
///         20,
///         1
///     ),
///     -3254
/// )
/// ```
pub fn get_random_integer_from_range_to_negative_infinity(
    limbs: &mut RandomPrimitiveInts<u64>,
    range_generator: &mut VariableRangeGenerator,
    a: Integer,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Integer {
    let (unsigned_max_bits, max_bits) = signed_significant_bits(&a);
    let bits = get_geometric_random_signed_from_inclusive_range(
        range_generator,
        i64::MIN,
        max_bits,
        mean_bits_numerator,
        mean_bits_denominator,
    );
    if bits == max_bits {
        get_random_integer_in_signed_max_bit_range(limbs, a, unsigned_max_bits)
    } else {
        Integer::from_sign_and_abs(
            bits >= 0,
            get_random_natural_with_bits(limbs, bits.unsigned_abs()),
        )
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct RandomIntegerRangeMultipleOrders {
    min_bits: i64,
    max_bits: i64,
    bits: GeometricRandomSignedRange<i64>,
    limbs: RandomPrimitiveInts<u64>,
    min_bit_xs: UniformRandomIntegerRange,
    max_bit_xs: UniformRandomIntegerRange,
}

impl Iterator for RandomIntegerRangeMultipleOrders {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let bits = self.bits.next().unwrap();
        if bits == self.min_bits {
            self.min_bit_xs.next()
        } else if bits == self.max_bits {
            self.max_bit_xs.next()
        } else {
            Some(Integer::from_sign_and_abs(
                bits >= 0,
                get_random_natural_with_bits(&mut self.limbs, bits.unsigned_abs()),
            ))
        }
    }
}

/// Generates random [`Integer`]s in an interval.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum RandomIntegerRange {
    SingleOrder(UniformRandomIntegerRange),
    MultipleOrders(RandomIntegerRangeMultipleOrders),
}

impl Iterator for RandomIntegerRange {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            RandomIntegerRange::SingleOrder(xs) => xs.next(),
            RandomIntegerRange::MultipleOrders(xs) => xs.next(),
        }
    }
}

/// Generates random [`Integer`]s in the half-open interval $[a, b)$.
///
/// In general, the [`Integer`]s are not generated uniformly; for that, use
/// [`uniform_random_integer_range`]. Instead, [`Natural`]s with smaller bit lengths are generated
/// more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_bits_numerator / mean_bits_denominator`. It is not actually the mean bit length, though it
/// approaches the mean bit length of the values minus $a$ as $\log (b/a)$ approaches infinity. $m$
/// cannot be 0, and must be greater than the bit length of the smallest integer in the range, but
/// it may be arbitrarily large. The smaller it is, the more quickly the probabilities decrease as
/// bit length increases. The larger it is, the more closely the distribution approaches a uniform
/// distribution over the bit lengths.
///
/// Once a bit length is selected, the [`Integer`] is chosen uniformly from all [`Integer`]s with
/// that bit length that are in $[a, b)$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(m) = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_bits_numerator +
/// mean_bits_denominator`, and $m$ is `max(a.significant_bits(), b.significant_bits())`.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their
/// ratio is less than or equal to the bit length of the smallest integer in the range, or if they
/// are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_integer_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_range(
///             EXAMPLE_SEED,
///             Integer::from(-1000),
///             Integer::from(1000000000),
///             20,
///             1
///         ),
///         10
///     ),
///     "[1, 1728664, 434, -30, 5282, 515436476, 2353848, -15, 19, 418, ...]"
/// )
/// ```
#[inline]
pub fn random_integer_range(
    seed: Seed,
    a: Integer,
    b: Integer,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerRange {
    assert!(a < b);
    random_integer_inclusive_range(
        seed,
        a,
        b - Integer::ONE,
        mean_bits_numerator,
        mean_bits_denominator,
    )
}

/// Generates random [`Integer`]s in the closed interval $[a, b]$.
///
/// In general, the [`Integer`]s are not generated uniformly; for that, use
/// [`uniform_random_integer_inclusive_range`]. Instead, [`Integer`]s with smaller bit lengths are
/// generated more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_bits_numerator / mean_bits_denominator`. It is not actually the mean bit length, though it
/// approaches the mean bit length of the values minus $a$ as $\log (b/a)$ approaches infinity. $m$
/// cannot be 0, and must be greater than the bit length of the smallest integer in the range, but
/// it may be arbitrarily large. The smaller it is, the more quickly the probabilities decrease as
/// bit length increases. The larger it is, the more closely the distribution approaches a uniform
/// distribution over the bit lengths.
///
/// Once a bit length is selected, the [`Integer`] is chosen uniformly from all [`Integer`]s with
/// that bit length that are in $[a, b]$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(m) = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_bits_numerator +
/// mean_bits_denominator`, and $m$ is `max(a.significant_bits(), b.significant_bits())`.
///
/// # Panics
/// Panics if $a > b$, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their ratio
/// is less than or equal to the bit length of the smallest integer in the range, or if they are too
/// large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::random_integer_inclusive_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         random_integer_inclusive_range(
///             EXAMPLE_SEED,
///             Integer::from(-1000),
///             Integer::from(999999999),
///             20,
///             1
///         ),
///         10
///     ),
///     "[1, 1728664, 434, -30, 5282, 515436476, 2353848, -15, 19, 418, ...]"
/// )
/// ```
pub fn random_integer_inclusive_range(
    seed: Seed,
    a: Integer,
    b: Integer,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomIntegerRange {
    assert!(a <= b);
    let (unsigned_min_bits, min_bits) = signed_significant_bits(&a);
    let (unsigned_max_bits, max_bits) = signed_significant_bits(&b);
    if min_bits == max_bits {
        RandomIntegerRange::SingleOrder(uniform_random_integer_inclusive_range(seed, a, b))
    } else {
        RandomIntegerRange::MultipleOrders(RandomIntegerRangeMultipleOrders {
            min_bits,
            max_bits,
            bits: geometric_random_signed_inclusive_range(
                seed.fork("bits"),
                min_bits,
                max_bits,
                mean_bits_numerator,
                mean_bits_denominator,
            ),
            limbs: random_primitive_ints(seed.fork("limbs")),
            min_bit_xs: signed_min_bit_range(seed.fork("min_bit_xs"), a, unsigned_min_bits),
            max_bit_xs: signed_max_bit_range(seed.fork("max_bit_xs"), b, unsigned_max_bits),
        })
    }
}

/// Generates random striped [`Integer`]s from a range.
#[derive(Clone, Debug)]
pub enum StripedRandomIntegerInclusiveRange {
    NonNegative(StripedRandomNaturalInclusiveRange),
    Negative(StripedRandomNaturalInclusiveRange),
    Both(
        RandomBools,
        Box<StripedRandomNaturalInclusiveRange>,
        Box<StripedRandomNaturalInclusiveRange>,
    ),
}

impl Iterator for StripedRandomIntegerInclusiveRange {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            StripedRandomIntegerInclusiveRange::NonNegative(xs) => xs.next().map(Integer::from),
            StripedRandomIntegerInclusiveRange::Negative(xs) => {
                xs.next().map(|x| Integer::from_sign_and_abs(false, x))
            }
            StripedRandomIntegerInclusiveRange::Both(bs, xs_nn, xs_n) => {
                if bs.next().unwrap() {
                    xs_nn.next().map(Integer::from)
                } else {
                    xs_n.next().map(|x| Integer::from_sign_and_abs(false, x))
                }
            }
        }
    }
}

/// Generates random striped [`Integer`]s in the range $[a, b)$.
///
/// Because the [`Integer`]s are constrained to be within a certain range, the actual mean run
/// length will usually not be $m$. Nonetheless, setting a higher $m$ will result in a higher mean
/// run length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
/// b.significant_bits())`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <=
/// mean_stripe_denominator`, or if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
/// use malachite_nz::integer::random::striped_random_integer_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_range(EXAMPLE_SEED, Integer::from(-4), Integer::from(7), 4, 1)
///             .map(|x| x.to_binary_string()),
///         10
///     ),
///     "[-100, -100, 110, 11, -100, 0, 110, 11, 0, 110, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_integer_range(
    seed: Seed,
    a: Integer,
    b: Integer,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomIntegerInclusiveRange {
    assert!(a < b);
    striped_random_integer_inclusive_range(
        seed,
        a,
        b - Integer::ONE,
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped [`Integer`]s in the range $[a, b]$.
///
/// Because the [`Integer`]s are constrained to be within a certain range, the actual mean run
/// length will usually not be $m$. Nonetheless, setting a higher $m$ will result in a higher mean
/// run length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
/// b.significant_bits())`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <=
/// mean_stripe_denominator`, or if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
/// use malachite_nz::integer::random::striped_random_integer_inclusive_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_inclusive_range(
///             EXAMPLE_SEED,
///             Integer::from(-4),
///             Integer::from(6),
///             4,
///             1
///         )
///         .map(|x| x.to_binary_string()),
///         10
///     ),
///     "[-100, -100, 110, 11, -100, 0, 110, 11, 0, 110, ...]"
/// );
/// ```
pub fn striped_random_integer_inclusive_range(
    seed: Seed,
    a: Integer,
    b: Integer,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomIntegerInclusiveRange {
    assert!(a <= b);
    if a >= 0u32 {
        StripedRandomIntegerInclusiveRange::NonNegative(striped_random_natural_inclusive_range(
            seed,
            a.unsigned_abs(),
            b.unsigned_abs(),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ))
    } else if b < 0u32 {
        StripedRandomIntegerInclusiveRange::Negative(striped_random_natural_inclusive_range(
            seed,
            b.unsigned_abs(),
            a.unsigned_abs(),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ))
    } else {
        StripedRandomIntegerInclusiveRange::Both(
            random_bools(seed.fork("sign")),
            Box::new(striped_random_natural_inclusive_range(
                seed.fork("non-negative"),
                Natural::ZERO,
                b.unsigned_abs(),
                mean_stripe_numerator,
                mean_stripe_denominator,
            )),
            Box::new(striped_random_natural_inclusive_range(
                seed.fork("negative"),
                Natural::ONE,
                a.unsigned_abs(),
                mean_stripe_numerator,
                mean_stripe_denominator,
            )),
        )
    }
}

fn striped_signed_min_bit_range(
    seed: Seed,
    a: Integer,
    unsigned_min_bits: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomIntegerInclusiveRange {
    if a >= 0 {
        striped_random_integer_range(
            seed.fork("min_bit_xs"),
            a,
            Integer::power_of_2(unsigned_min_bits),
            mean_stripe_numerator,
            mean_stripe_denominator,
        )
    } else {
        striped_random_integer_inclusive_range(
            seed.fork("min_bit_xs"),
            a,
            -Integer::power_of_2(unsigned_min_bits - 1),
            mean_stripe_numerator,
            mean_stripe_denominator,
        )
    }
}

fn striped_signed_max_bit_range(
    seed: Seed,
    a: Integer,
    unsigned_max_bits: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomIntegerInclusiveRange {
    if a > 0 {
        striped_random_integer_inclusive_range(
            seed.fork("max_bit_xs"),
            Integer::power_of_2(unsigned_max_bits - 1),
            a,
            mean_stripe_numerator,
            mean_stripe_denominator,
        )
    } else {
        // also handles a == 0
        striped_random_integer_inclusive_range(
            seed.fork("max_bit_xs"),
            -Integer::power_of_2(unsigned_max_bits) + Integer::ONE,
            a,
            mean_stripe_numerator,
            mean_stripe_denominator,
        )
    }
}

/// Generates striped random [`Integer`]s greater than or equal to a lower bound, or less than or
/// equal to an upper bound.
#[derive(Clone, Debug)]
pub struct StripedRandomIntegerRangeToInfinity {
    boundary_bits: i64,
    bits: GeometricRandomSignedRange<i64>,
    bit_source: StripedBitSource,
    boundary_bit_xs: StripedRandomIntegerInclusiveRange,
}

impl Iterator for StripedRandomIntegerRangeToInfinity {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let bits = self.bits.next().unwrap();
        if bits == self.boundary_bits {
            self.boundary_bit_xs.next()
        } else {
            Some(Integer::from_sign_and_abs(
                bits >= 0,
                get_striped_random_natural_with_bits(&mut self.bit_source, bits.unsigned_abs()),
            ))
        }
    }
}

/// Generates striped random [`Integer`]s greater than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the [`Integer`]s is specified; it must be greater than the bit length
/// of $a$. $m$ is equal to `mean_bits_numerator / mean_bits_denominator`.
///
/// The actual bit length is chosen from a geometric distribution with lower bound $a$ and mean $m$.
/// The resulting distribution has no mean or higher-order statistics (unless $a < m < a + 1$, which
/// is not typical).
///
/// Because the [`Integer`]s are constrained to be within a certain range, the actual mean run
/// length will usually not be $\mu$. Nonetheless, setting a higher $\mu$ will result in a higher
/// mean run length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a >
/// 0$ and their ratio is less than or equal to the bit length of $a$, or if they are too large and
/// manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_integer_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_range_to_infinity(
///             EXAMPLE_SEED,
///             Integer::from(-1000),
///             20,
///             1,
///             10,
///             1
///         ),
///         10
///     ),
///     "[8192, 2, 1024, 33554400, -128, 1023, 8, 14745599, -256, -67, ...]"
/// )
/// ```
pub fn striped_random_integer_range_to_infinity(
    seed: Seed,
    a: Integer,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegerRangeToInfinity {
    let (unsigned_min_bits, min_bits) = signed_significant_bits(&a);
    StripedRandomIntegerRangeToInfinity {
        boundary_bits: min_bits,
        bits: geometric_random_signed_inclusive_range(
            seed.fork("bits"),
            min_bits,
            i64::MAX,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        boundary_bit_xs: striped_signed_min_bit_range(
            seed.fork("min_bit_xs"),
            a,
            unsigned_min_bits,
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates striped random [`Integer`]s less than or equal to an upper bound $a$.
///
/// The mean bit length $m$ of the [`Integer`]s is specified; it must be greater than the bit length
/// of $a$. $m$ is equal to `mean_bits_numerator / mean_bits_denominator`.
///
/// The actual bit length is chosen from a geometric distribution with lower bound $a$ and mean $m$.
/// The resulting distribution has no mean or higher-order statistics (unless $a < m < a + 1$, which
/// is not typical).
///
/// Because the [`Integer`]s are constrained to be within a certain range, the actual mean run
/// length will usually not be $\mu$. Nonetheless, setting a higher $\mu$ will result in a higher
/// mean run length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n + m)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $b <
/// 0$ and their ratio is less than or equal to the bit length of $b$, or if they are too large and
/// manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::random::striped_random_integer_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_integer_range_to_negative_infinity(
///             EXAMPLE_SEED,
///             Integer::from(1000),
///             20,
///             1,
///             10,
///             1
///         ),
///         10
///     ),
///     "[4, 2, -1024, -144115188075919360, -516096, 992, -15, -16776704, 511, 64, ...]"
/// )
/// ```
pub fn striped_random_integer_range_to_negative_infinity(
    seed: Seed,
    a: Integer,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomIntegerRangeToInfinity {
    let (unsigned_max_bits, max_bits) = signed_significant_bits(&a);
    StripedRandomIntegerRangeToInfinity {
        boundary_bits: max_bits,
        bits: geometric_random_signed_inclusive_range(
            seed.fork("bits"),
            i64::MIN,
            max_bits,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        boundary_bit_xs: striped_signed_max_bit_range(
            seed.fork("max_bit_xs"),
            a,
            unsigned_max_bits,
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}
