use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::random::geometric::{
    geometric_random_natural_signeds, geometric_random_negative_signeds,
    geometric_random_nonzero_signeds, geometric_random_positive_signeds, geometric_random_signeds,
    GeometricRandomNaturalValues, GeometricRandomNegativeSigneds, GeometricRandomNonzeroSigneds,
    GeometricRandomSigneds,
};
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::num::random::{random_primitive_ints, RandomPrimitiveInts};
use malachite_base::random::Seed;
use natural::random::{get_random_natural_with_bits, get_striped_random_natural_with_bits};

/// Generates random `Integer`s, given an iterator of random signed bit lengths.
///
/// The `Integer`s sign is taken from the sign of the bit length.
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

/// Generates random natural (non-negative) `Integer`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 0. Then an `Integer` is
/// chosen uniformly among all non-negative `Integer`s with that bit length. The resulting
/// distribution resembles a Pareto distribution. It has no mean or higher-order statistics (unless
/// $m < 1$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & n < 0 \\\\
///     \\frac{1}{m + 1} & n = 0 \\\\
///     \\frac{2}{m+1} \\left ( \\frac{m}{2(m+1)} \\right ) ^ {\\lfloor \\log_2 n \\rfloor + 1} &
///     \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or, if after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::random_natural_integers;
///
/// assert_eq!(
///     random_natural_integers(EXAMPLE_SEED, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "20431208470830262", "2777240", "114", "12184833305054", "1121025855008623490210",
///         "13478874522577592", "115311695", "7", "18", "54522366353"
///     ]
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

/// Generates random positive `Integer`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then an `Integer` is
/// chosen uniformly among all positive `Integer`s with that bit length. The resulting distribution
/// resembles a Pareto distribution. It has no mean or higher-order statistics (unless $m < 2$,
/// which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & n \leq 0 \\\\
///     \frac{1}{m} \left ( \frac{m-1}{2m} \right ) ^ {\lfloor \log_2 n \rfloor} & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// This would be a Pareto distribution, if it were not for the floor.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::random_positive_integers;
///
/// assert_eq!(
///     random_positive_integers(EXAMPLE_SEED, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "22", "4", "178", "55845661150", "93254818", "7577967529619388", "8", "11316951483471",
///         "11", "1005760138411689342464923704482"
///     ]
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

/// Generates random negative `Integer`s whose absolute values have a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then an `Integer` is
/// chosen uniformly among all positive `Integer`s with that bit length, and negated. The resulting
/// distribution resembles a negated Pareto distribution. It has no mean or higher-order statistics
/// (unless $m < 2$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & n \geq 0 \\\\
///     \frac{1}{m} \left ( \frac{m-1}{2m} \right ) ^ {\lfloor \log_2 (-n) \rfloor}
///     & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// This would be a negated Pareto distribution, if it were not for the floor.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::random_negative_integers;
///
/// assert_eq!(
///     random_negative_integers(EXAMPLE_SEED, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "-22", "-4", "-178", "-55845661150", "-93254818", "-7577967529619388", "-8",
///         "-11316951483471", "-11", "-1005760138411689342464923704482"
///     ]
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

/// Generates random nonzero `Integer`s whose absolute values have a specified mean bit length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// `geometric_random_nonzero_signeds`); $m$ must be greater than 1. Then an `Integer` is
/// chosen uniformly among all positive `Integer`s with that bit length, and its sign is set to the
/// sign of the signed bit length. The resulting distribution has no mean or higher-order statistics
/// (unless $m < 2$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & n = 0 \\\\
///     \\frac{1}{2m} \\left ( \\frac{m-1}{2m} \\right ) ^ {\\lfloor \\log_2 |n| \\rfloor}
///     & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::random_nonzero_integers;
///
/// assert_eq!(
///     random_nonzero_integers(EXAMPLE_SEED, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "6", "373973144", "46887963477285686350042496363292819122", "-93254818", "-126908",
///         "-4471675267836600", "1860142159", "-118004986915853475", "-98", "346513"
///     ]
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

/// Generates random `Integer`s whose absolute values have a specified mean bit length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// `geometric_random_nonzero_signeds`); $m$ must be greater than 0. Then an `Integer` is
/// chosen uniformly among all `Integer`s with that bit length, and its sign is set to the sign of
/// the signed bit length. The resulting distribution has no mean or higher-order statistics (unless
/// $m < 1$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     \\frac{1}{2m+1} & n = 0 \\\\
///     \\frac{2}{2m+1} \\left ( \\frac{m}{2(m+1)} \\right ) ^ {\\lfloor \\log_2 |n| \\rfloor + 1}
///     & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or, if after being reduced
/// to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::random_integers;
///
/// assert_eq!(
///     random_integers(EXAMPLE_SEED, 32, 1).take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "89270", "69403499476962893258904", "62", "-1848070042786", "-64671510460", "-696", "0",
///         "-79", "70819", "7330"
///     ]
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

/// Generates striped random `Integer`s, given an iterator of random signed bit lengths.
///
/// The `Integer`s sign is taken from the sign of the bit length.
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

/// Generates striped random natural (non-negative) `Integer`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 0. A striped bit
/// sequence (see `StripedBitSource`) with the given stripe parameter is generated and truncated at
/// the bit length. The highest bit is forced to be 1, and the `Integer` is generated from the
/// sequence.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::striped_random_natural_integers;
///
/// assert_eq!(
///     striped_random_natural_integers(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "18014656207519744", "2228160", "64", "17592184995840", "1179440951012584587264",
///         "9007749010526207", "67108864", "5", "24", "34359738879"
///     ]
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

/// Generates striped random positive `Integer`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. A striped bit
/// sequence (see `StripedBitSource`) with the given stripe parameter is generated and truncated at
/// the bit length. The highest bit is forced to be 1, and the `Integer` is generated from the
/// sequence.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or if `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::striped_random_positive_integers;
///
/// assert_eq!(
///     striped_random_positive_integers(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "16", "4", "128", "34391195648", "75493376", "9007199120523391", "8", "8796094070783",
///         "8", "950737950171027935941967741439"
///     ]
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

/// Generates striped random negative `Integer`s whose absolute values have a specified mean bit
/// length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. A striped bit
/// sequence (see `StripedBitSource`) with the given stripe parameter is generated and truncated at
/// the bit length. The highest bit is forced to be 1, and the `Integer` is generated from the
/// sequence and negated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or if `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::striped_random_negative_integers;
///
/// assert_eq!(
///     striped_random_negative_integers(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "-16", "-4", "-128", "-34391195648", "-75493376", "-9007199120523391", "-8",
///         "-8796094070783", "-8", "-950737950171027935941967741439"
///     ]
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

/// Generates striped random nonzero `Integer`s whose absolute values have a specified mean bit
/// length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// `geometric_random_nonzero_signeds`); $m$ must be greater than 1. A striped bit sequence (see
/// `StripedBitSource`) with the given stripe parameter is generated and truncated at the bit
/// length. The highest bit is forced to be 1, an `Integer` is generated from the sequence, and its
/// sign is set to the sign of the signed bit length. The resulting distribution has no mean or
/// higher-order statistics (unless $m < 2$, which is not typical).
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or if `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::striped_random_nonzero_integers;
///
/// assert_eq!(
///     striped_random_nonzero_integers(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "4", "268435456", "84405977732342160290572740160760316144", "-133169152", "-131064",
///         "-2251834173421823", "1577058304", "-126100789566374399", "-76", "270335"
///     ]
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

/// Generates striped random `Integer`s whose absolute values have a specified mean bit length.
///
/// The actual signed bit length is chosen from a distribution that produces values whose mean
/// absolute values are $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator` (see
/// `geometric_random_nonzero_signeds`); $m$ must be greater than 0. A striped bit sequence (see
/// `StripedBitSource`) with the given stripe parameter is generated and truncated at the bit
/// length. The highest bit is forced to be 1, an `Integer` is generated from the sequence, and its
/// sign is set to the sign of the signed bit length. The resulting distribution has no mean or
/// higher-order statistics (unless $m < 1$, which is not typical).
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_bits_numerator` or
/// `mean_bits_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer::random::striped_random_integers;
///
/// assert_eq!(
///     striped_random_integers(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Integer::to_string(&x)).collect_vec(),
///     &[
///         "65536", "75521006248971741167616", "32", "-2199023255520", "-68719468544", "-527", "0",
///         "-112", "131071", "4152"
///     ]
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
