use itertools::Itertools;
use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::iterators::iterator_to_bit_chunks;
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::striped::{get_striped_unsigned_vec, StripedBitSource};
use malachite_base::num::random::{random_primitive_ints, RandomPrimitiveInts};
use malachite_base::random::Seed;
use malachite_base::rounding_modes::RoundingMode;
use natural::arithmetic::mod_power_of_two::limbs_slice_mod_power_of_two_in_place;
use natural::logic::bit_access::limbs_slice_set_bit;
use natural::Natural;

/// Generates a random `Natural` with a given maximum bit length.
///
/// The `Natural` is chosen uniformly from $[0, 2^b)$; `Natural`s with bit lengths smaller than the
/// maximum may also be generated.
///
/// # Expected worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and `n` is `bits`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::random::get_random_natural_with_up_to_bits;
/// use malachite_base::num::random::random_primitive_ints;
///
/// assert_eq!(
///     get_random_natural_with_up_to_bits(&mut random_primitive_ints(EXAMPLE_SEED), 100)
///         .to_string(),
///     "976558340558744279591984426865"
/// );
/// ```
pub fn get_random_natural_with_up_to_bits(xs: &mut RandomPrimitiveInts<u64>, bits: u64) -> Natural {
    if bits == 0 {
        return Natural::ZERO;
    }
    #[cfg(feature = "32_bit_limbs")]
    let mut xs = iterator_to_bit_chunks(
        xs.take(usize::exact_from(
            bits.shr_round(u64::LOG_WIDTH, RoundingMode::Ceiling),
        )),
        u64::WIDTH,
        u32::WIDTH,
    )
    .collect_vec();
    #[cfg(not(feature = "32_bit_limbs"))]
    let mut xs = xs
        .take(usize::exact_from(
            bits.shr_round(u64::LOG_WIDTH, RoundingMode::Ceiling),
        ))
        .collect_vec();
    limbs_slice_mod_power_of_two_in_place(&mut xs, bits);
    Natural::from_owned_limbs_asc(xs)
}

/// Generates a random `Natural` with a given bit length.
///
/// The `Natural` is 0 if `bits` is 0, or else chosen uniformly from $[2^(b-1), 2^b)$.
///
/// # Expected worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and `n` is `bits`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::random::get_random_natural_with_bits;
/// use malachite_base::num::random::random_primitive_ints;
///
/// assert_eq!(
///     get_random_natural_with_bits(&mut random_primitive_ints(EXAMPLE_SEED), 100).to_string(),
///     "976558340558744279591984426865"
/// );
/// ```
pub fn get_random_natural_with_bits(xs: &mut RandomPrimitiveInts<u64>, bits: u64) -> Natural {
    if bits == 0 {
        return Natural::ZERO;
    }
    #[cfg(feature = "32_bit_limbs")]
    let mut xs = iterator_to_bit_chunks(
        xs.take(usize::exact_from(
            bits.shr_round(u64::LOG_WIDTH, RoundingMode::Ceiling),
        )),
        u64::WIDTH,
        u32::WIDTH,
    )
    .collect_vec();
    #[cfg(not(feature = "32_bit_limbs"))]
    let mut xs = xs
        .take(usize::exact_from(
            bits.shr_round(u64::LOG_WIDTH, RoundingMode::Ceiling),
        ))
        .collect_vec();
    limbs_slice_mod_power_of_two_in_place(&mut xs, bits);
    limbs_slice_set_bit(&mut xs, bits - 1);
    Natural::from_owned_limbs_asc(xs)
}

/// Generates a striped random `Natural` with a given maximum bit length.
///
/// `Natural`s with bit lengths smaller than the maximum may also be generated.
///
/// See `StripedBitSource`.
///
/// # Expected worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and `n` is `bits`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::random::striped::StripedBitSource;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::random::get_striped_random_natural_with_up_to_bits;
///
/// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 10, 1);
/// // 0x3fffff80000ffc007ffe03ff8
/// assert_eq!(
///     get_striped_random_natural_with_up_to_bits(&mut bit_source, 100).to_string(),
///     "316912612278197474676665499640"
/// );
/// ```
pub fn get_striped_random_natural_with_up_to_bits(xs: &mut StripedBitSource, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        Natural::from_owned_limbs_asc(get_striped_unsigned_vec(xs, bits))
    }
}

/// Generates a striped random `Natural` with a given bit length.
///
/// See `StripedBitSource`.
///
/// # Expected worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and `n` is `bits`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::random::striped::StripedBitSource;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::random::get_striped_random_natural_with_bits;
///
/// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 10, 1);
/// // 0xbfffff80000ffc007ffe03ff8
/// assert_eq!(
///     get_striped_random_natural_with_bits(&mut bit_source, 100).to_string(),
///     "950737912392312175425017102328"
/// );
/// ```
pub fn get_striped_random_natural_with_bits(xs: &mut StripedBitSource, bits: u64) -> Natural {
    if bits == 0 {
        Natural::ZERO
    } else {
        let mut xs = get_striped_unsigned_vec(xs, bits);
        limbs_slice_set_bit(&mut xs, bits - 1);
        Natural::from_owned_limbs_asc(xs)
    }
}

/// Generates random `Natural`s, given an iterator of random bit lengths.
#[derive(Clone, Debug)]
pub struct RandomNaturals<I: Iterator<Item = u64>> {
    bits: I,
    limbs: RandomPrimitiveInts<u64>,
}

impl<I: Iterator<Item = u64>> Iterator for RandomNaturals<I> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(get_random_natural_with_bits(
            &mut self.limbs,
            self.bits.next().unwrap(),
        ))
    }
}

/// Generates random `Natural`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 0. Then a `Natural` is
/// chosen uniformly among all `Natural`s with that bit length. The resulting distribution resembles
/// a Pareto distribution. It has no mean or higher-order statistics (unless $m < 1$, which is not
/// typical).
///
/// $$
/// P(n) = \\begin{cases}
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
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::random_naturals;
///
/// assert_eq!(
///     random_naturals(EXAMPLE_SEED, 32, 1).take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "20431208470830262", "2777240", "114", "12184833305054", "1121025855008623490210",
///         "13478874522577592", "115311695", "7", "18", "54522366353"
///     ]
/// )
/// ```
pub fn random_naturals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturals<GeometricRandomNaturalValues<u64>> {
    RandomNaturals {
        bits: geometric_random_unsigneds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates random positive `Natural`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then a `Natural` is
/// chosen uniformly among all `Natural`s with that bit length. The resulting distribution resembles
/// a Pareto distribution. It has no mean or higher-order statistics (unless $m < 2$, which is not
/// typical).
///
/// $$
/// P(n) = \frac{1}{m} \left ( \frac{m-1}{2m} \right ) ^ {\lfloor \log_2 n \rfloor}
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
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::random_positive_naturals;
///
/// assert_eq!(
///     random_positive_naturals(EXAMPLE_SEED, 32, 1)
///         .take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "22", "4", "178", "55845661150", "93254818", "7577967529619388", "8", "11316951483471",
///         "11", "1005760138411689342464923704482"
///     ]
/// )
/// ```
pub fn random_positive_naturals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturals<GeometricRandomNaturalValues<u64>> {
    RandomNaturals {
        bits: geometric_random_positive_unsigneds(
            seed.fork("bits"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
    }
}

/// Generates striped random `Natural`s, given an iterator of random bit lengths.
#[derive(Clone, Debug)]
pub struct StripedRandomNaturals<I: Iterator<Item = u64>> {
    bits: I,
    bit_source: StripedBitSource,
}

impl<I: Iterator<Item = u64>> Iterator for StripedRandomNaturals<I> {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        Some(get_striped_random_natural_with_bits(
            &mut self.bit_source,
            self.bits.next().unwrap(),
        ))
    }
}

/// Generates striped random `Natural`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 0. A striped bit
/// sequence (see `StripedBitSource`) with the given stripe parameter is generated and truncated at
/// the bit length. The highest bit is forced to be 1, and the `Natural` is generated from the
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
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::striped_random_naturals;
///
/// assert_eq!(
///     striped_random_naturals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "18014656207519744", "2228160", "64", "17592184995840", "1179440951012584587264",
///         "9007749010526207", "67108864", "5", "24", "34359738879"
///     ]
/// )
/// ```
pub fn striped_random_naturals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomNaturals<GeometricRandomNaturalValues<u64>> {
    StripedRandomNaturals {
        bits: geometric_random_unsigneds(
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

/// Generates striped random positive `Natural`s with a specified mean bit length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. A striped bit
/// sequence (see `StripedBitSource`) with the given stripe parameter is generated and truncated at
/// the bit length. The highest bit is forced to be 1, and the `Natural` is generated from the
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
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::random::striped_random_positive_naturals;
///
/// assert_eq!(
///     striped_random_positive_naturals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "16", "4", "128", "34391195648", "75493376", "9007199120523391", "8", "8796094070783",
///         "8", "950737950171027935941967741439"
///     ]
/// )
/// ```
pub fn striped_random_positive_naturals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomNaturals<GeometricRandomNaturalValues<u64>> {
    StripedRandomNaturals {
        bits: geometric_random_positive_unsigneds(
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
