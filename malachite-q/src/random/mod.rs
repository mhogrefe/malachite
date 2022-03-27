use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
use malachite_base::random::Seed;
use malachite_nz::natural::random::{
    random_naturals, random_positive_naturals, striped_random_naturals,
    striped_random_positive_naturals, RandomNaturals, StripedRandomNaturals,
};
use malachite_nz::natural::Natural;
use Rational;

/// Generates random non-negative `Rational`s, given an iterator of random `Natural` numerators and
/// denominators.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromSingle<I: Iterator<Item = Natural>> {
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomRationalsFromSingle<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_naturals(
            self.xs.next().unwrap(),
            self.xs.next().unwrap(),
        ))
    }
}

/// Generates random positive `Rational`s with a specified numerator and denominator mean bit
/// length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then the numerator
/// and denominator are chosen from all positive `Integer`s with that bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::random_positive_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     random_positive_rationals(EXAMPLE_SEED, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "11/2",
///         "89/27922830575",
///         "46627409/3788983764809694",
///         "8/11316951483471",
///         "11/1005760138411689342464923704482",
///         "948931/42716754",
///         "81013760999253680590984897748479904878392/23",
///         "1/97645164585502",
///         "1558028859598/29",
///         "200127331174844881647/4058622214797175252"
///     ]
/// )
/// ```
pub fn random_positive_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingle<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingle {
        xs: random_positive_naturals(seed, mean_bits_numerator, mean_bits_denominator),
    }
}

/// Generates random non-negative `Rational`s, given an iterator of random `Natural` numerators and
/// an iterator of random `Natural` denominators.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromDouble<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> {
    xs: I,
    ys: J,
}

impl<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> Iterator
    for RandomRationalsFromDouble<I, J>
{
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_naturals(
            self.xs.next().unwrap(),
            self.ys.next().unwrap(),
        ))
    }
}

/// Generates random non-negative `Rational`s with a specified numerator and denominator mean bit
/// length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::random_non_negative_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     random_non_negative_rationals(EXAMPLE_SEED, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "7301/34",
///         "4183103/1234731190583",
///         "54812347098686/6195807891591254727",
///         "812739/17841539017",
///         "665/908",
///         "677/1138982845180",
///         "166/22491855393807861245619791028129",
///         "270142/5",
///         "52040856788711439301087669967/15975369961878544862054",
///         "5718607/1953563256716085077"
///     ]
/// )
/// ```
pub fn random_non_negative_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDouble<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDouble {
        xs: random_naturals(
            seed.fork("numerator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: random_positive_naturals(
            seed.fork("denominator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates random negative `Rational`s, given an iterator of positive `Rational`s.
#[derive(Clone, Debug)]
pub struct NegativeRationals<I: Iterator<Item = Rational>> {
    xs: I,
}

impl<I: Iterator<Item = Rational>> Iterator for NegativeRationals<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        self.xs.next().map(|mut q| {
            q.sign = false;
            q
        })
    }
}

/// Generates random negative `Rational`s with a specified numerator and denominator mean bit
/// length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then the numerator
/// and denominator are chosen from all positive `Integer`s with that bit length. Finally, the
/// resulting `Rational` is reduced and negated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::random_negative_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     random_negative_rationals(EXAMPLE_SEED, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "-11/2",
///         "-89/27922830575",
///         "-46627409/3788983764809694",
///         "-8/11316951483471",
///         "-11/1005760138411689342464923704482",
///         "-948931/42716754",
///         "-81013760999253680590984897748479904878392/23",
///         "-1/97645164585502",
///         "-1558028859598/29",
///         "-200127331174844881647/4058622214797175252"
///     ]
/// )
/// ```
pub fn random_negative_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> NegativeRationals<RandomRationalsFromSingle<RandomNaturals<GeometricRandomNaturalValues<u64>>>>
{
    NegativeRationals {
        xs: random_positive_rationals(seed, mean_bits_numerator, mean_bits_denominator),
    }
}

/// Generates random non-negative `Rational`s, given an iterator of random `Natural` numerators and
/// an iterator of `bool` signs.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromSingleAndSign<I: Iterator<Item = Natural>> {
    bs: RandomBools,
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomRationalsFromSingleAndSign<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_sign_and_naturals(
            self.bs.next().unwrap(),
            self.xs.next().unwrap(),
            self.xs.next().unwrap(),
        ))
    }
}

/// Generates random nonzero `Rational`s with a specified numerator and denominator mean bit
/// length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::random_nonzero_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     random_nonzero_rationals(EXAMPLE_SEED, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "-80861953616/9687130509484985",
///         "-14557437513/313",
///         "100721397389/392237929981",
///         "713431423/1285",
///         "-3887883364/889",
///         "14185/969",
///         "12609/11359517108746272468338071",
///         "3443/4354945",
///         "1/29",
///         "5551/892095"
///     ]
/// )
/// ```
pub fn random_nonzero_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingleAndSign<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: random_positive_naturals(seed.fork("abs"), mean_bits_numerator, mean_bits_denominator),
    }
}

/// Generates random non-negative `Rational`s, given an iterator of random `Natural` numerators, an
/// iterator of random `Natural` denominators, and an iterator of `bool` signs.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromDoubleAndSign<
    I: Iterator<Item = Natural>,
    J: Iterator<Item = Natural>,
> {
    pub bs: RandomBools,
    pub xs: I,
    pub ys: J,
}

impl<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> Iterator
    for RandomRationalsFromDoubleAndSign<I, J>
{
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_sign_and_naturals(
            self.bs.next().unwrap(),
            self.xs.next().unwrap(),
            self.ys.next().unwrap(),
        ))
    }
}

/// Generates random `Rational`s with a specified numerator and denominator mean bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::random_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     random_rationals(EXAMPLE_SEED, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "-7301/34",
///         "-4183103/1234731190583",
///         "54812347098686/6195807891591254727",
///         "812739/17841539017",
///         "-665/908",
///         "677/1138982845180",
///         "166/22491855393807861245619791028129",
///         "270142/5",
///         "52040856788711439301087669967/15975369961878544862054",
///         "5718607/1953563256716085077"
///     ]
/// )
/// ```
pub fn random_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDoubleAndSign<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDoubleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: random_naturals(
            seed.fork("numerator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: random_positive_naturals(
            seed.fork("denominator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random positive `Rational`s with a specified mean numerator and denominator
/// bit length.
///
/// The actual numerator and denominator bit lengths are chosen from a geometric distribution with
/// mean $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than
/// 1. A striped bit sequence (see `StripedBitSource`) with the given stripe parameter is generated
/// and truncated at the bit lengths to produce the numerators and denominators. The highest bits
/// are forced to be 1. Finally, the `Rational` is reduced.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::striped_random_positive_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     striped_random_positive_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "4",
///         "1/268681216",
///         "75493376/9007199120523391",
///         "8/8796094070783",
///         "8/950737950171027935941967741439",
///         "1040391/33554432",
///         "2813000899879757964630563421437095845888",
///         "1/79164837199872",
///         "2199023255551/16",
///         "220784470296873664512/4611685966886694919"
///     ]
/// )
/// ```
pub fn striped_random_positive_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingle<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingle {
        xs: striped_random_positive_naturals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random non-positive `Rational`s with a specified mean numerator and
/// denominator bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::striped_random_non_negative_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     striped_random_non_negative_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "8192/127",
///         "16776704/4396972769407",
///         "8796093005951/648518346332962816",
///         "87381/2863267840",
///         "1024/2043",
///         "51/58408828928",
///         "85/13521606402434254795714066382848",
///         "270335/7",
///         "59421159664630116152453890047/9444741445172838006656",
///         "6291455/1154891846623166464"
///     ]
/// )
/// ```
pub fn striped_random_non_negative_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDouble<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDouble {
        xs: striped_random_naturals(
            seed.fork("numerator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: striped_random_positive_naturals(
            seed.fork("denominator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random negative `Rational`s with a specified mean numerator and denominator
/// bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::striped_random_negative_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     striped_random_negative_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "-4",
///         "-1/268681216",
///         "-75493376/9007199120523391",
///         "-8/8796094070783",
///         "-8/950737950171027935941967741439",
///         "-1040391/33554432",
///         "-2813000899879757964630563421437095845888",
///         "-1/79164837199872",
///         "-2199023255551/16",
///         "-220784470296873664512/4611685966886694919"
///     ]
/// )
/// ```
pub fn striped_random_negative_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> NegativeRationals<
    RandomRationalsFromSingle<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>>,
> {
    NegativeRationals {
        xs: striped_random_positive_rationals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random nonzero `Rational`s with a specified mean numerator and denominator
/// bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::striped_random_nonzero_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     striped_random_nonzero_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "-68720000000/18006083452797439",
///         "-2545165805/29",
///         "549754781664/1236950581247",
///         "1065353727/2047",
///         "-2147745791/513",
///         "16128/575",
///         "8192/17000482516899619632318463",
///         "18431/16778240",
///         "1/31",
///         "4096/526335"
///     ]
/// )
/// ```
pub fn striped_random_nonzero_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingleAndSign<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: striped_random_positive_naturals(
            seed.fork("abs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random `Rational`s with a specified mean numerator and denominator bit
/// length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// TODO
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
/// use malachite_q::random::striped_random_rationals;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     striped_random_rationals(EXAMPLE_SEED, 16, 1, 32, 1)
///         .take(10)
///         .map(|x| Rational::to_string(&x))
///         .collect_vec(),
///     &[
///         "-8192/127",
///         "-16776704/4396972769407",
///         "8796093005951/648518346332962816",
///         "87381/2863267840",
///         "-1024/2043",
///         "51/58408828928",
///         "85/13521606402434254795714066382848",
///         "270335/7",
///         "59421159664630116152453890047/9444741445172838006656",
///         "6291455/1154891846623166464"
///     ]
/// )
/// ```
pub fn striped_random_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDoubleAndSign<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDoubleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: striped_random_naturals(
            seed.fork("numerator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: striped_random_positive_naturals(
            seed.fork("denominator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}
