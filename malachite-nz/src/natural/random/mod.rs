use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{CeilingLogTwo, PowerOfTwo, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::iterators::iterator_to_bit_chunks;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::geometric::{
    geometric_random_positive_unsigneds, geometric_random_unsigned_inclusive_range,
    geometric_random_unsigneds, GeometricRandomNaturalValues,
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
/// P(n) = \\begin{cases}
///     0 & n = 0 \\\\
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

/// Uniformly generates random `Natural`s less than a positive limit.
#[derive(Clone, Debug)]
pub struct RandomNaturalsLessThan {
    bits: u64,
    limit: Natural,
    limbs: RandomPrimitiveInts<u64>,
}

impl Iterator for RandomNaturalsLessThan {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        loop {
            let x = get_random_natural_with_up_to_bits(&mut self.limbs, self.bits);
            if x < self.limit {
                return Some(x);
            }
        }
    }
}

/// Uniformly generates random `Natural`s less than a positive `limit`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{\\ell} & x < \\ell \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $\ell$ is `limit`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `limit.significant_bits()`.
///
/// # Panics
///
/// Panics if `limit` is 0.
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
/// use malachite_nz::natural::random::random_naturals_less_than;
///
/// assert_eq!(
///     random_naturals_less_than(EXAMPLE_SEED, Natural::from(10u32)).take(10)
///         .map(|x| Natural::to_string(&x)).collect_vec(),
///     &["1", "7", "5", "7", "9", "2", "8", "2", "4", "6"]
/// )
/// ```
pub fn random_naturals_less_than(seed: Seed, limit: Natural) -> RandomNaturalsLessThan {
    assert_ne!(limit, 0);
    RandomNaturalsLessThan {
        bits: limit.ceiling_log_two(),
        limit,
        limbs: random_primitive_ints(seed),
    }
}

/// Uniformly generates random `Natural`s in an interval.
#[derive(Clone, Debug)]
pub struct UniformRandomNaturalRange {
    xs: RandomNaturalsLessThan,
    a: Natural,
}

impl Iterator for UniformRandomNaturalRange {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        self.xs.next().map(|x| &self.a + x)
    }
}

/// Uniformly generates random `Natural`s in the half-open interval $[a, b)$.
///
/// `a` must be less than `b`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a} & a \leq x < b \\\\
///     0 & \\text{otherwise}
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
/// where $T$ is time, $M$ is additional memory, and $n$ = `b.significant_bits()`.
///
/// # Panics
///
/// Panics if $a \geq b$.
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
/// use malachite_nz::natural::random::uniform_random_natural_range;
///
/// assert_eq!(
///     uniform_random_natural_range(
///         EXAMPLE_SEED,
///         Natural::from(10u32),
///         Natural::from(100u32)
///     ).take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &["97", "17", "94", "37", "56", "32", "96", "11", "17", "39"]
/// )
/// ```
pub fn uniform_random_natural_range(
    seed: Seed,
    a: Natural,
    b: Natural,
) -> UniformRandomNaturalRange {
    assert!(a < b);
    UniformRandomNaturalRange {
        xs: random_naturals_less_than(seed, b - &a),
        a,
    }
}

/// Uniformly generates random `Natural`s in the closed interval $[a, b]$.
///
/// `a` must be less than or equal to `b`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a+1} & a \leq x \leq b \\\\
///     0 & \\text{otherwise}
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
/// where $T$ is time, $M$ is additional memory, and $n$ = `b.significant_bits()`.
///
/// # Panics
///
/// Panics if $a > b$.
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
/// use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
///
/// assert_eq!(
///     uniform_random_natural_inclusive_range(
///         EXAMPLE_SEED,
///         Natural::from(10u32),
///         Natural::from(99u32)
///     ).take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &["97", "17", "94", "37", "56", "32", "96", "11", "17", "39"]
/// )
/// ```
#[inline]
pub fn uniform_random_natural_inclusive_range(
    seed: Seed,
    a: Natural,
    b: Natural,
) -> UniformRandomNaturalRange {
    assert!(a <= b);
    uniform_random_natural_range(seed, a, b + Natural::ONE)
}

/// Generates random `Natural`s greater than or equal to a lower bound.
#[derive(Clone, Debug)]
pub struct RandomNaturalRangeToInfinity {
    min_bits: u64,
    bits: GeometricRandomNaturalValues<u64>,
    limbs: RandomPrimitiveInts<u64>,
    min_bit_xs: UniformRandomNaturalRange,
}

impl Iterator for RandomNaturalRangeToInfinity {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let bits = self.bits.next().unwrap();
        if bits == self.min_bits {
            self.min_bit_xs.next()
        } else {
            Some(get_random_natural_with_bits(&mut self.limbs, bits))
        }
    }
}

/// Generates random `Natural`s greater than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the `Natural`s is specified; it must be greater than the bit length
/// of $a$. $m$ is equal to `mean_bits_numerator / mean_bits_denominator`.
///
/// The actual bit length is chosen from a geometric distribution with lower bound $a$ and mean $m$.
/// Then a `Natural` is chosen uniformly among all `Natural`s with that bit length that are greater
/// than or equal to $a$. The resulting distribution has no mean or higher-order statistics (unless
/// $a < m < a + 1$, which is not typical).
///
/// $$
/// P(n) = \\begin{cases}
///     0 & n < a \\\\
///     \frac{1}{m+1} & a = n = 0 \\\\
///     \frac{m}{(m+1)^2}\left ( \frac{m}{2(m+1)} \right )^\nu & 0 = a < n \\\\
///     \frac{1}{(2^{\nu + 1}-a)(m-\alpha)} & 0 < a \\ \text{and} \\ \alpha = \nu \\\\
///     \frac{\left ( 1 - \frac{1}{\alpha-m}\right )^{\nu-\alpha}}{2^\nu(m-\alpha)} &
///         0 < a \\ \text{and} \\ \alpha < \nu \\\\
/// \\end{cases}
/// $$
/// where $\alpha = \lfloor \log_2 a \rfloor$ and $\nu = \lfloor \log_2 n \rfloor$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`, and $m$ = `mean_bits_numerator` / `mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their ratio is less than
/// or equal to `a`, or if they are too large and manipulating them leads to arithmetic overflow.
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
/// use malachite_nz::natural::random::random_natural_range_to_infinity;
///
/// assert_eq!(
///     random_natural_range_to_infinity(EXAMPLE_SEED, Natural::from(1000u32), 20, 1)
///         .take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "3254", "4248", "163506", "1189717027294", "5282", "12220", "60088", "1016911",
///         "5772451", "5473099750562"
///     ]
/// )
/// ```
pub fn random_natural_range_to_infinity(
    seed: Seed,
    a: Natural,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalRangeToInfinity {
    let min_bits = a.significant_bits();
    RandomNaturalRangeToInfinity {
        min_bits,
        bits: geometric_random_unsigned_inclusive_range(
            seed.fork("bits"),
            min_bits,
            u64::MAX,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        limbs: random_primitive_ints(seed.fork("limbs")),
        min_bit_xs: uniform_random_natural_range(
            seed.fork("min_bit_xs"),
            a,
            Natural::power_of_two(min_bits),
        ),
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct RandomNaturalRangeMultipleOrders {
    min_bits: u64,
    max_bits: u64,
    bits: GeometricRandomNaturalValues<u64>,
    limbs: RandomPrimitiveInts<u64>,
    min_bit_xs: UniformRandomNaturalRange,
    max_bit_xs: UniformRandomNaturalRange,
}

impl Iterator for RandomNaturalRangeMultipleOrders {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let bits = self.bits.next().unwrap();
        if bits == self.min_bits {
            self.min_bit_xs.next()
        } else if bits == self.max_bits {
            self.max_bit_xs.next()
        } else {
            Some(get_random_natural_with_bits(&mut self.limbs, bits))
        }
    }
}

/// Generates random `Natural`s in an interval.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum RandomNaturalRange {
    SingleOrder(UniformRandomNaturalRange),
    MultipleOrders(RandomNaturalRangeMultipleOrders),
}

impl Iterator for RandomNaturalRange {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        match self {
            RandomNaturalRange::SingleOrder(xs) => xs.next(),
            RandomNaturalRange::MultipleOrders(xs) => xs.next(),
        }
    }
}

/// Generates random `Natural`s in the half-open interval $[a, b)$.
///
/// In general, the `Natural`s are not generated uniformly; for that, use
/// `uniform_random_natural_range`. Instead, `Natural`s with smaller bit lengths are generated more
/// frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_bits_numerator` / `mean_bits_denominator`. It is not actually the mean bit length, though
/// it approaches the mean bit length as $\log (b/a)$ approaches infinity. $m$ must be greater than
/// $a$, but it may be arbitrarily large. The smaller it is, the more quickly the probabilities
/// decrease as bit length increases. The larger it is, the more closely the distribution approaches
/// a uniform distribution over the bit lengths.
///
/// Once a bit length is selected, the `Natural` is chosen uniformly from all `Natural`s with that
/// bit length that are in $[a, b)$.
///
/// To obtain the probability mass function, adjust $b$ and see `natural_random_inclusive_range`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`, and $m$ = `b.significant_bits()`.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their
/// ratio is less than or equal to `a`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
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
/// use malachite_nz::natural::random::random_natural_range;
///
/// assert_eq!(
///     random_natural_range(
///         EXAMPLE_SEED,
///         Natural::from(1000u32),
///         Natural::from(1000000000u32),
///         20,
///         1
///     ).take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "3254", "4248", "163506", "600542", "5282", "12220", "60088", "1016911", "5772451",
///         "2792610"
///     ]
/// )
/// ```
#[inline]
pub fn random_natural_range(
    seed: Seed,
    a: Natural,
    b: Natural,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalRange {
    assert!(a < b);
    random_natural_inclusive_range(
        seed,
        a,
        b - Natural::ONE,
        mean_bits_numerator,
        mean_bits_denominator,
    )
}

/// Generates random `Natural`s in the closed interval $[a, b]$.
///
/// In general, the `Natural`s are not generated uniformly; for that, use
/// `uniform_random_natural_inclusive_range`. Instead, `Natural`s with smaller bit lengths are
/// generated more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_bits_numerator` / `mean_bits_denominator`. It is not actually the mean bit length, though
/// it approaches the mean bit length as $\log (b/a)$ approaches infinity. $m$ must be greater than
/// $a$, but it may be arbitrarily large. The smaller it is, the more quickly the probabilities
/// decrease as bit length increases. The larger it is, the more closely the distribution approaches
/// a uniform distribution over the bit lengths.
///
/// Once a bit length is selected, the `Natural` is chosen uniformly from all `Natural`s with that
/// bit length that are in $[a, b]$.
///
/// $$
/// P(n) = \\begin{cases}
///     0 & n < a \\ \text{or} \\ n > b  \\\\
///     1 & a = n = b = 0 \\\\
///     \frac{1}{b-a} & 0 < a \\ \text{and} \\ \alpha = \beta \\\\
///     \left \[ (m+1)\left ( 1-\left (\frac{m}{m+1}\right )^{\beta+2} \right ) \right \]^{-1} &
///         0 = a = n < b \\\\
///     (2^{\alpha+1}-n)\left \[ (m-\alpha)\left ( 1+\frac{1}{\alpha-m}\right )^{\alpha-\nu}-
///         (m-\alpha-1)\left ( 1+\frac{1}{\alpha-m}\right )^{\beta-\nu}\right \]^{-1} &
///         0 < a \\ \text{and} \\ \alpha = \nu < \beta \\\\
///     \frac{m^{\nu+1}(m+1)^{\beta-\nu}}{((m+1)^{\beta+2}-m^{\beta+2})(2^\beta-n+1)} &
///         0 = a < n \\ \text{and} \\ \nu = \beta \\\\
///     (n-2^\beta+1)\left \[ (m-\alpha)\left ( 1+\frac{1}{\alpha-m}\right )^{\alpha-\nu}-
///         (m-\alpha-1)\left ( 1+\frac{1}{\alpha-m}\right )^{\beta-\nu}\right \]^{-1} &
///         0 < a \\ \text{and} \\ \alpha < \nu = \beta \\\\
///     \frac{m(m+1)^\beta \left ( \frac{m}{2(m+1)}\right )^\nu}{(m+1)^{\beta+2}-m^{\beta+2}} &
///         0 = a < n \\ \text{and} \\ \nu < \beta \\\\
///     2^{-\nu}\left \[ (m-\alpha)\left ( 1+\frac{1}{\alpha-m}\right )^{\alpha-\nu}-(m-\alpha-1)
///         \left ( 1+\frac{1}{\alpha-m}\right )^{\beta-\nu}\right \]^{-1} &
///         0 < a \\ \text{and} \\ \alpha < \nu < \beta
/// \\end{cases}
/// $$
/// where $\alpha = \lfloor \log_2 a \rfloor$, $\beta = \lfloor \log_2 b \rfloor$, and
/// $\nu = \lfloor \log_2 n \rfloor$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E\[T\] = O(n)$
///
/// $E\[M\] = O(m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ = `mean_bits_numerator` +
/// `mean_bits_denominator`, and $m$ = `b.significant_bits()`.
///
/// # Panics
/// Panics if $a > b$, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their ratio
/// is less than or equal to `a`, or, if they are too large and manipulating them leads to
/// arithmetic overflow.
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
/// use malachite_nz::natural::random::random_natural_inclusive_range;
///
/// assert_eq!(
///     random_natural_inclusive_range(
///         EXAMPLE_SEED,
///         Natural::from(1000u32),
///         Natural::from(1000000000u32),
///         20,
///         1
///     ).take(10).map(|x| Natural::to_string(&x)).collect_vec(),
///     &[
///         "3254", "4248", "163506", "600542", "5282", "12220", "60088", "1016911", "5772451",
///         "2792610"
///     ]
/// )
/// ```
pub fn random_natural_inclusive_range(
    seed: Seed,
    a: Natural,
    b: Natural,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalRange {
    assert!(a <= b);
    let min_bits = a.significant_bits();
    let max_bits = b.significant_bits();
    if min_bits == max_bits {
        RandomNaturalRange::SingleOrder(uniform_random_natural_inclusive_range(seed, a, b))
    } else {
        RandomNaturalRange::MultipleOrders(RandomNaturalRangeMultipleOrders {
            min_bits,
            max_bits,
            bits: geometric_random_unsigned_inclusive_range(
                seed.fork("bits"),
                min_bits,
                max_bits,
                mean_bits_numerator,
                mean_bits_denominator,
            ),
            limbs: random_primitive_ints(seed.fork("limbs")),
            min_bit_xs: uniform_random_natural_range(
                seed.fork("min_bit_xs"),
                a,
                Natural::power_of_two(min_bits),
            ),
            max_bit_xs: uniform_random_natural_inclusive_range(
                seed.fork("max_bit_xs"),
                Natural::power_of_two(max_bits - 1),
                b,
            ),
        })
    }
}
